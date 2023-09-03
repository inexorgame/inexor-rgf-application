use std::ops::Deref;
use std::sync::Arc;

use async_trait::async_trait;
use log::debug;
use log::trace;
use log::warn;
use serde_json::json;

use crate::api::ComponentManager;
use crate::api::EntityTypeManager;
use crate::api::Lifecycle;
use crate::api::SystemEventManager;
use crate::di::component;
use crate::di::Component;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Wrc;
use crate::error::types::entity::EntityTypeCreationError;
use crate::error::types::entity::EntityTypeRegistrationError;
use crate::model::ComponentTypeId;
use crate::model::ComponentTypeIds;
use crate::model::EntityType;
use crate::model::EntityTypeAddComponentError;
use crate::model::EntityTypeAddExtensionError;
use crate::model::EntityTypeAddPropertyError;
use crate::model::EntityTypeId;
use crate::model::EntityTypeIds;
use crate::model::EntityTypeMergeError;
use crate::model::EntityTypeRemoveComponentError;
use crate::model::EntityTypeRemoveExtensionError;
use crate::model::EntityTypeRemovePropertyError;
use crate::model::EntityTypes;
use crate::model::EntityTypeUpdateExtensionError;
use crate::model::EntityTypeUpdatePropertyError;
use crate::model::Extension;
use crate::model::ExtensionContainer;
use crate::model::Extensions;
use crate::model::ExtensionTypeId;
use crate::model::NamespacedTypeComponentTypeIdContainer;
use crate::model::NamespacedTypeContainer;
use crate::model::NamespacedTypeExtensionContainer;
use crate::model::NamespacedTypePropertyTypeContainer;
use crate::model::Namespaces;
use crate::model::PropertyType;
use crate::model::PropertyTypeContainer;
use crate::model::PropertyTypes;
use crate::model::TypeDefinitionGetter;
use crate::model_runtime::EXTENSION_DIVERGENT;
use crate::plugins::EntityTypeProvider;
use crate::plugins::SystemEvent;

#[wrapper]
pub struct EntityTypesStorage(EntityTypes);

#[provides]
fn create_entity_types_storage() -> EntityTypesStorage {
    EntityTypesStorage(EntityTypes::new())
}

#[component]
pub struct EntityTypeManagerImpl {
    event_manager: Wrc<dyn SystemEventManager>,

    component_manager: Wrc<dyn ComponentManager>,

    entity_types: EntityTypesStorage,
}

impl EntityTypeManagerImpl {}

#[async_trait]
#[provides]
impl EntityTypeManager for EntityTypeManagerImpl {
    fn register(&self, entity_type: EntityType) -> Result<EntityType, EntityTypeRegistrationError> {
        let ty = entity_type.ty.clone();
        if self.entity_types.contains_key(&ty) {
            return Err(EntityTypeRegistrationError::EntityTypeAlreadyExists(ty));
        }

        // Apply components
        let mut divergent = Vec::new();
        for component_ty in entity_type.components.iter() {
            let mut is_divergent = false;
            match self.component_manager.get(&component_ty) {
                Some(component) => {
                    // TODO: what if multiple components have the same property? (like c__http__http__*__result and c__logical__action__*__result)
                    for (property_name, property_type) in component.properties {
                        // Own property wins
                        if !entity_type.has_own_property(&property_name) {
                            entity_type.properties.push(property_type.clone());
                        } else {
                            // Check for divergent data type
                            if let Some(entity_type_property_type) = entity_type.get_own_property(&property_type.name) {
                                if property_type.data_type != entity_type_property_type.data_type {
                                    is_divergent = true;
                                    warn!(
                                        "{}__{} has divergent data type {} to {}__{} which has data type {}",
                                        &entity_type.ty,
                                        &entity_type_property_type.name,
                                        &entity_type_property_type.data_type,
                                        component_ty.deref(),
                                        &property_type.name,
                                        &property_type.data_type
                                    );
                                }
                            }
                            // TODO: merge description (if no own description)
                            // TODO: merge extensions (for each: if own does not have the extension, add it)
                        }
                    }
                }
                None => {
                    is_divergent = true;
                    warn!(
                        "Entity type {} not fully initialized: No component named {}",
                        entity_type.type_definition().to_string(),
                        component_ty.type_definition().to_string()
                    )
                }
            }
            if is_divergent {
                divergent.push(component_ty.to_string());
            }
        }
        divergent.sort();
        let _ = entity_type.add_extension(Extension::new(EXTENSION_DIVERGENT.clone(), String::new(), json!(divergent)));
        // entity_type
        //     .extensions
        //     .push(Extension::new(EXTENSION_DIVERGENT.clone(), String::new(), json!(divergent)));
        self.entity_types.push(entity_type.clone());
        // self.entity_types.0.write().unwrap().push(entity_type.clone());
        debug!("Registered entity type {}", entity_type.type_definition().to_string());
        self.event_manager.emit_event(SystemEvent::EntityTypeCreated(entity_type.ty.clone()));
        Ok(entity_type)
    }

    fn get_all(&self) -> EntityTypes {
        self.entity_types.clone()
    }

    fn get_type_ids(&self) -> EntityTypeIds {
        self.entity_types.type_ids()
    }

    fn get_namespaces(&self) -> Namespaces {
        self.entity_types.namespaces()
    }

    fn get_by_namespace(&self, namespace: &str) -> EntityTypes {
        self.entity_types.get_by_namespace(namespace)
    }

    fn get_types_by_namespace(&self, namespace: &str) -> EntityTypeIds {
        self.entity_types.get_types_by_namespace(namespace)
    }

    fn get_by_having_component(&self, component_ty: &ComponentTypeId) -> EntityTypes {
        self.entity_types.get_by_having_component(component_ty)
    }

    fn has(&self, ty: &EntityTypeId) -> bool {
        self.entity_types.contains_key(ty)
    }

    fn has_by_type(&self, namespace: &str, type_name: &str) -> bool {
        self.has(&EntityTypeId::new_from_type(namespace, type_name))
    }

    fn get(&self, ty: &EntityTypeId) -> Option<EntityType> {
        self.entity_types.get(ty)
            .map(|entity_type| entity_type.value().clone())
    }

    fn get_by_type(&self, namespace: &str, type_name: &str) -> Option<EntityType> {
        self.get(&EntityTypeId::new_from_type(namespace, type_name))
    }

    fn find_by_type_name(&self, search: &str) -> EntityTypes {
        self.entity_types.find_by_type_name(search)
    }

    fn count(&self) -> usize {
        self.entity_types.len()
    }

    fn count_by_namespace(&self, namespace: &str) -> usize {
        self.entity_types.count_by_namespace(namespace)
    }

    fn create(
        &self,
        ty: &EntityTypeId,
        description: &str,
        components: ComponentTypeIds,
        properties: PropertyTypes,
        extensions: Extensions,
    ) -> Result<EntityType, EntityTypeCreationError> {
        let entity_type = EntityType::builder()
            .ty(ty)
            .description(description)
            .components(components)
            .properties(properties)
            .extensions(extensions)
            .build();
        self.register(entity_type)
        // self.register(EntityType::new(ty.clone(), description, components.to_vec(), properties.to_vec(), extensions.to_vec()))
            .map_err(EntityTypeCreationError::RegistrationError)
    }

    fn merge(&self, entity_type_to_merge: EntityType) -> Result<EntityType, EntityTypeMergeError> {
        self.entity_types.merge(entity_type_to_merge)
        // let ty = entity_type_to_merge.ty;
        // if !self.has(&ty) {
        //     return Err(EntityTypeMergeError::EntityTypeDoesNotExists(ty));
        // }
        // for component_ty in entity_type_to_merge.components {
        //     let _ = self.add_component(&ty, &component_ty);
        // }
        // let mut guard = self.entity_types.0.write().unwrap();
        // let Some(entity_type) = guard.iter_mut().find(|e| e.ty == ty) else {
        //     return Err(EntityTypeMergeError::EntityTypeDoesNotExists(ty));
        // };
        // entity_type.description = entity_type_to_merge.description.clone();
        // entity_type.merge_properties(entity_type_to_merge.properties);
        // entity_type.merge_extensions(entity_type_to_merge.extensions);
        // Ok(entity_type.clone())
    }

    fn add_component(&self, entity_ty: &EntityTypeId, component_ty: &ComponentTypeId) -> Result<(), EntityTypeAddComponentError> {
        if !self.component_manager.has(component_ty) {
            return Err(EntityTypeAddComponentError::ComponentDoesNotExist(component_ty.clone()));
        }
        self.entity_types.add_component(entity_ty, component_ty)
        // let mut guard = self.entity_types.0.write().unwrap();
        // for entity_type in guard.iter_mut() {
        //     if &entity_type.ty == ty {
        //         if entity_type.is_a(component_ty) {
        //             return Err(EntityTypeComponentError::ComponentAlreadyAssigned);
        //         }
        //         match self.component_manager.get(component_ty) {
        //             Some(component) => {
        //                 entity_type.components.push(component_ty.clone());
        //                 entity_type.merge_properties(component.properties);
        //             }
        //             None => {
        //                 return Err(EntityTypeComponentError::ComponentDoesNotExist);
        //             }
        //         }
        //         self.event_manager
        //             .emit_event(SystemEvent::EntityTypeComponentAdded(ty.clone(), component_ty.clone()));
        //     }
        // }
        // Ok(())
    }

    fn remove_component(&self, entity_ty: &EntityTypeId, component_ty: &ComponentTypeId) -> Result<ComponentTypeId, EntityTypeRemoveComponentError> {
        self.entity_types.remove_component(entity_ty, component_ty)
        // let mut guard = self.entity_types.0.write().unwrap();
        // for entity_type in guard.iter_mut() {
        //     if &entity_type.ty == ty {
        //         entity_type.components.retain(|c| c != component_ty);
        //         // TODO: what if multiple components have the same property?
        //         if let Some(component) = self.component_manager.get(component_ty) {
        //             let properties_to_remove: Vec<String> = component.properties.iter().map(|property| property.name.clone()).collect();
        //             entity_type.properties.retain(|property| !properties_to_remove.contains(&property.name));
        //         }
        //         self.event_manager
        //             .emit_event(SystemEvent::EntityTypeComponentRemoved(ty.clone(), component_ty.clone()));
        //     }
        // }
    }

    fn add_property(&self, entity_ty: &EntityTypeId, property_type: PropertyType) -> Result<PropertyType, EntityTypeAddPropertyError> {
        self.entity_types.add_property(entity_ty, property_type)
        // let mut guard = self.entity_types.0.write().unwrap();
        // for entity_type in guard.iter_mut() {
        //     if &entity_type.ty == ty {
        //         if entity_type.has_own_property(property.name.clone()) {
        //             return Err(EntityTypePropertyError::PropertyAlreadyExists);
        //         }
        //         entity_type.properties.push(property.clone());
        //         self.event_manager
        //             .emit_event(SystemEvent::EntityTypePropertyAdded(ty.clone(), property.name.clone()));
        //     }
        // }
        // Ok(())
    }

    fn update_property(&self, entity_ty: &EntityTypeId, property_name: &str, property_type: PropertyType) -> Result<PropertyType, EntityTypeUpdatePropertyError> {
        self.entity_types.update_property(entity_ty, property_name, property_type)

    }

    fn remove_property(&self, entity_ty: &EntityTypeId, property_name: &str) -> Result<PropertyType, EntityTypeRemovePropertyError> {
        self.entity_types.remove_property(entity_ty, property_name)
        // let mut guard = self.entity_types.0.write().unwrap();
        // for entity_type in guard.iter_mut() {
        //     if &entity_type.ty == ty {
        //         entity_type.properties.retain(|property| property.name != property_name);
        //         self.event_manager
        //             .emit_event(SystemEvent::EntityTypePropertyRemoved(ty.clone(), property_name.to_string()));
        //     }
        // }
    }

    fn add_extension(&self, entity_ty: &EntityTypeId, extension: Extension) -> Result<ExtensionTypeId, EntityTypeAddExtensionError> {
        self.entity_types.add_extension(entity_ty, extension)
        // let extension_ty = extension.ty.clone();
        // let mut guard = self.entity_types.0.write().unwrap();
        // for entity_type in guard.iter_mut() {
        //     if &entity_type.ty == ty {
        //         if entity_type.has_own_extension(&extension_ty) {
        //             return Err(EntityTypeExtensionError::ExtensionAlreadyExists(extension_ty));
        //         }
        //         entity_type.extensions.push(extension.clone());
        //         self.event_manager
        //             .emit_event(SystemEvent::EntityTypeExtensionAdded(ty.clone(), extension_ty.clone()));
        //     }
        // }
        // Ok(())
    }

    fn update_extension(&self, entity_ty: &EntityTypeId, extension_ty: &ExtensionTypeId, extension: Extension) -> Result<Extension, EntityTypeUpdateExtensionError> {
        self.entity_types.update_extension(entity_ty, extension_ty, extension)
    }

    fn remove_extension(&self, entity_ty: &EntityTypeId, extension_ty: &ExtensionTypeId) -> Result<Extension, EntityTypeRemoveExtensionError> {
        self.entity_types.remove_extension(entity_ty, extension_ty)
        // let mut guard = self.entity_types.0.write().unwrap();
        // for entity_type in guard.iter_mut() {
        //     if &entity_type.ty == ty {
        //         entity_type.extensions.retain(|extension| &extension.ty != extension_ty);
        //         self.event_manager
        //             .emit_event(SystemEvent::EntityTypeExtensionRemoved(ty.clone(), extension_ty.clone()));
        //     }
        // }
    }

    // TODO: parameter "cascade": relation types, flow types and entity instances (and their dependencies) depends on a entity type
    // TODO: first delete the entity instance of this type, then delete the entity type itself.
    fn delete(&self, entity_ty: &EntityTypeId) -> Option<EntityType> {
        self.entity_types
            .remove(entity_ty)
            .map(|(entity_ty, entity_type)| {
                self.event_manager.emit_event(SystemEvent::EntityTypeDeleted(entity_ty.clone()));
                entity_type
            })
    }

    fn validate(&self, ty: &EntityTypeId) -> bool {
        if let Some(entity_type) = self.get(ty) {
            return entity_type.components.iter().all(|component| self.component_manager.has(&component));
        }
        false
    }

    fn add_provider(&self, entity_type_provider: Arc<dyn EntityTypeProvider>) {
        for entity_type in entity_type_provider.get_entity_types() {
            trace!("Registering entity type: {}", entity_type.ty);
            if self.register(entity_type.clone()).is_err() {
                trace!("Merging entity type: {}", entity_type.ty);
                let _ = self.merge(entity_type);
            }
        }
    }
}

#[async_trait]
impl Lifecycle for EntityTypeManagerImpl {
    async fn shutdown(&self) {
        self.entity_types.clear()
    }
}

#[cfg(test)]
mod test {
    extern crate test;

    use crate::get_runtime;
    use crate::model::Component;
    use crate::model::ComponentTypeId;
    use crate::model::ComponentTypeIdContainer;
    use crate::model::EntityType;
    use crate::model::EntityTypeId;
    use crate::model::NamespacedTypeGetter;
    use crate::model::PropertyType;
    use crate::model::PropertyTypeContainer;
    use crate::test_utils::r_string;

    use std::process::Termination;
    use test::Bencher;
    use default_test::DefaultTest;
    use crate::model::ComponentTypeIds;

    #[test]
    fn test_register_entity_type() {
        let runtime = get_runtime();
        let entity_type_manager = runtime.get_entity_type_manager();

        let namespace = r_string();
        let type_name = r_string();
        let description = r_string();

        let component_ty = ComponentTypeId::new_from_type(&namespace, &r_string());
        let entity_type = EntityType::new_from_type(&namespace, &type_name, &description, vec![component_ty], vec![PropertyType::string("x")], vec![]);
        let result = entity_type_manager.register(entity_type.clone());
        assert!(result.is_ok());
        assert!(entity_type_manager.has_by_type(&namespace, &type_name));
        assert!(entity_type_manager.has(&entity_type.ty));

        assert_eq!(type_name, entity_type_manager.get_by_type(&namespace, &type_name).unwrap().type_name());
        assert_eq!(type_name, entity_type_manager.get(&entity_type.ty).unwrap().type_name());
    }

    #[test]
    fn test_create_and_delete_entity_type() {
        let runtime = get_runtime();
        let entity_type_manager = runtime.get_entity_type_manager();

        let entity_type = entity_type_manager.register(EntityType::default_test()).expect("Failed to register the entity type!");
        let ty = entity_type.ty.clone();

        assert!(entity_type_manager.has(&ty), "The entity type should be registered!");
        entity_type_manager.delete(&ty).expect("Failed to delete the entity type!");
        assert!(!entity_type_manager.has(&ty), "The entity type shouldn't be registered anymore!");
        assert!(entity_type_manager.get(&ty).is_none(), "The entity type shouldn't be registered anymore!");
    }

    #[test]
    fn test_get_entity_types() {
        let runtime = get_runtime();
        let entity_type_manager = runtime.get_entity_type_manager();

        let entity_type = entity_type_manager.register(EntityType::default_test()).expect("Failed to register the entity type!");
        assert!(entity_type_manager.has(&entity_type.ty), "The entity type should be registered!");
        let entity_types = entity_type_manager.get_all();
        assert_eq!(1, entity_types.len(), "There should be exactly one entity type!");
        for entity_type in entity_types.iter() {
            assert!(entity_type_manager.has(&entity_type.ty), "It should be possible to check if the returned entity types are registered!");
            let _ = entity_type_manager.get(&entity_type.ty).expect("It should be possible to get the returned entity types by type id!");
        }
    }

    #[test]
    fn test_register_entity_type_has_component() {
        let runtime = get_runtime();
        let component_manager = runtime.get_component_manager();
        let entity_type_manager = runtime.get_entity_type_manager();

        let component = component_manager.register(Component::default_test()).expect("Failed to register component!");

        let entity_ty = EntityTypeId::default_test();
        let entity_type = EntityType::builder_from_ty(&entity_ty)
            .component(&component.ty)
            .build();

        let _entity_type = entity_type_manager.register(entity_type).expect("Failed to register entity type!");
        let entity_type = entity_type_manager.get(&entity_ty).expect("It should be possible to get the entity type by type id!");
        assert!(entity_type.is_a(&component.ty), "The entity type should contain the component!");
        assert!(entity_type.components.contains(&component.ty), "The entity type should contain the component!");
    }

    #[test]
    fn test_register_entity_type_has_property() {
        let runtime = get_runtime();
        let entity_type_manager = runtime.get_entity_type_manager();

        let property_type = PropertyType::default_test();

        let entity_ty = EntityTypeId::default_test();
        let entity_type = EntityType::builder_from_ty(&entity_ty)
            .components(ComponentTypeIds::default_test())
            .property(property_type.clone())
            .build();

        let _entity_type = entity_type_manager.register(entity_type).expect("Failed to register entity type!");
        let entity_type = entity_type_manager.get(&entity_ty).expect("It should be possible to get the entity type by type id!");
        assert!(entity_type.has_own_property(&property_type.name));
        assert!(entity_type.properties.contains_key(&property_type.name));

        // // let property_name = String::from("x");
        // // let property_type = PropertyType::string(&property_name);
        //
        // // let entity_type_name = r_string();
        // // let namespace = r_string();
        //
        // let entity_ty = EntityTypeId::new_from_type(&namespace, &entity_type_name);
        // let entity_type = EntityType::new(&entity_ty, String::new(), vec![], vec![property_type], vec![]);
        // assert!(entity_type_manager.register(entity_type).is_ok());
        // assert!(entity_type_manager.get(&entity_ty).unwrap().has_own_property(property_name.as_str()));
    }

    #[bench]
    fn creation_benchmark(bencher: &mut Bencher) -> impl Termination {
        let runtime = get_runtime();
        let entity_type_manager = runtime.get_entity_type_manager();
        let entity_type = EntityType::default_test();
        let ty = entity_type.ty.clone();
        bencher.iter(move || {
            let _ = entity_type_manager.register(entity_type.clone());
            let _ = entity_type_manager.delete(&ty);
        })
    }

}
