use inexor_rgf_type_system_api::ComponentCreationError;
use std::sync::Arc;

use inexor_rgf_graph::Component;
use inexor_rgf_graph::ComponentAddExtensionError;
use inexor_rgf_graph::ComponentAddPropertyError;
use inexor_rgf_graph::ComponentRemoveExtensionError;
use inexor_rgf_graph::ComponentRemovePropertyError;
use inexor_rgf_graph::ComponentTypeId;
use inexor_rgf_graph::ComponentUpdateExtensionError;
use inexor_rgf_graph::ComponentUpdatePropertyError;
use inexor_rgf_graph::Components;
use inexor_rgf_graph::Extension;
use inexor_rgf_graph::ExtensionTypeId;
use inexor_rgf_graph::PropertyType;

pub struct ComponentManagerDelegate {
    component_manager: Arc<dyn inexor_rgf_type_system_api::ComponentManager + Send + Sync>,
}

impl ComponentManagerDelegate {
    pub fn new(component_manager: Arc<dyn inexor_rgf_type_system_api::ComponentManager + Send + Sync>) -> Self {
        Self { component_manager }
    }
}
impl inexor_rgf_plugin_api::ComponentManager for ComponentManagerDelegate {
    fn get_all(&self) -> Components {
        self.component_manager.get_all()
    }

    fn get_by_namespace(&self, namespace: &str) -> Components {
        self.component_manager.get_by_namespace(namespace)
    }

    fn has(&self, ty: &ComponentTypeId) -> bool {
        self.component_manager.has(ty)
    }

    fn has_by_type(&self, namespace: &str, type_name: &str) -> bool {
        self.component_manager.has_by_type(namespace, type_name)
    }

    fn get(&self, ty: &ComponentTypeId) -> Option<Component> {
        self.component_manager.get(ty)
    }

    fn get_by_type(&self, namespace: &str, type_name: &str) -> Option<Component> {
        self.component_manager.get_by_type(namespace, type_name)
    }

    fn find_by_type_name(&self, search: &str) -> Components {
        self.component_manager.find_by_type_name(search)
    }

    fn count(&self) -> usize {
        self.component_manager.count()
    }

    fn count_by_namespace(&self, namespace: &str) -> usize {
        self.component_manager.count_by_namespace(namespace)
    }

    fn create(
        &self,
        ty: &ComponentTypeId,
        description: &str,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) -> Result<Component, ComponentCreationError> {
        self.component_manager.create_component(ty, description, properties, extensions)
    }

    fn replace(&self, ty: &ComponentTypeId, component: Component) {
        self.component_manager.replace(ty, component)
    }

    fn add_property(&self, ty: &ComponentTypeId, property: PropertyType) -> Result<PropertyType, ComponentAddPropertyError> {
        self.component_manager.add_property(ty, property)
    }

    fn update_property(&self, ty: &ComponentTypeId, property_name: &str, property: PropertyType) -> Result<PropertyType, ComponentUpdatePropertyError> {
        self.component_manager.update_property(ty, property_name, property)
    }

    fn remove_property(&self, ty: &ComponentTypeId, property_name: &str) -> Result<PropertyType, ComponentRemovePropertyError> {
        self.component_manager.remove_property(ty, property_name)
    }

    fn add_extension(&self, ty: &ComponentTypeId, extension: Extension) -> Result<ExtensionTypeId, ComponentAddExtensionError> {
        self.component_manager.add_extension(ty, extension)
    }

    fn update_extension(
        &self,
        component_ty: &ComponentTypeId,
        extension_ty: &ExtensionTypeId,
        extension: Extension,
    ) -> Result<Extension, ComponentUpdateExtensionError> {
        self.component_manager.update_extension(component_ty, extension_ty, extension)
    }

    fn remove_extension(&self, component_ty: &ComponentTypeId, extension_ty: &ExtensionTypeId) -> Result<Extension, ComponentRemoveExtensionError> {
        self.component_manager.remove_extension(component_ty, extension_ty)
    }

    fn delete(&self, ty: &ComponentTypeId) -> bool {
        self.component_manager.delete(ty)
    }
}
