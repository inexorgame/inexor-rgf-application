use serde_json::Value;
use std::sync::Arc;

use inexor_rgf_graph::Mutability;
use inexor_rgf_rt_api::ReactiveRelationComponentAddError;
use inexor_rgf_rt_api::ReactiveRelationComponentRemoveError;
use inexor_rgf_rt_api::ReactiveRelationCreationError;
use inexor_rgf_rt_api::ReactiveRelationPropertyAddError;
use inexor_rgf_rt_api::ReactiveRelationPropertyRemoveError;
use uuid::Uuid;

use crate::api::ComponentManager;
use crate::api::ReactiveRelationManager;
use crate::api::RelationTypeManager;
use crate::behaviour_api::BehaviourTypeId;
use crate::model::ComponentTypeId;
use crate::model::RelationInstance;
use crate::model::RelationInstanceId;
use crate::model::RelationTypeId;
use crate::plugins::RelationInstanceManager;
use crate::reactive::ReactiveRelation;

pub struct RelationInstanceManagerImpl {
    component_manager: Arc<dyn ComponentManager>,
    relation_type_manager: Arc<dyn RelationTypeManager>,
    reactive_relation_manager: Arc<dyn ReactiveRelationManager>,
}

impl RelationInstanceManagerImpl {
    pub fn new(
        component_manager: Arc<dyn ComponentManager>,
        relation_type_manager: Arc<dyn RelationTypeManager>,
        reactive_relation_manager: Arc<dyn ReactiveRelationManager>,
    ) -> Self {
        Self {
            component_manager,
            relation_type_manager,
            reactive_relation_manager,
        }
    }
}
impl RelationInstanceManager for RelationInstanceManagerImpl {
    fn has(&self, edge_key: &RelationInstanceId) -> bool {
        self.reactive_relation_manager.has(edge_key)
    }

    fn get(&self, edge_key: &RelationInstanceId) -> Option<ReactiveRelation> {
        self.reactive_relation_manager.get(edge_key)
    }

    fn get_by_outbound_entity(&self, outbound_entity_id: Uuid) -> Vec<ReactiveRelation> {
        self.reactive_relation_manager.get_by_outbound_entity(outbound_entity_id)
    }

    fn get_by_inbound_entity(&self, inbound_entity_id: Uuid) -> Vec<ReactiveRelation> {
        self.reactive_relation_manager.get_by_inbound_entity(inbound_entity_id)
    }

    fn get_all(&self) -> Vec<ReactiveRelation> {
        self.reactive_relation_manager.get_all()
    }

    fn get_by_type(&self, ty: &RelationTypeId) -> Vec<ReactiveRelation> {
        self.reactive_relation_manager.get_by_type(ty)
    }

    fn get_by_namespace(&self, namespace: &str) -> Vec<ReactiveRelation> {
        self.reactive_relation_manager.get_by_namespace(namespace)
    }

    fn get_keys(&self) -> Vec<RelationInstanceId> {
        self.reactive_relation_manager.get_keys()
    }

    fn count(&self) -> usize {
        self.reactive_relation_manager.count()
    }

    fn count_by_type(&self, ty: &RelationTypeId) -> usize {
        self.reactive_relation_manager.count_by_type(ty)
    }

    fn count_by_component(&self, component: &ComponentTypeId) -> usize {
        self.reactive_relation_manager.count_by_component(component)
    }

    fn count_by_behaviour(&self, behaviour_ty: &BehaviourTypeId) -> usize {
        self.reactive_relation_manager.count_by_behaviour(behaviour_ty)
    }

    fn create(&self, relation_instance: RelationInstance) -> Result<ReactiveRelation, ReactiveRelationCreationError> {
        let relation_ty = relation_instance.relation_type_id();
        let relation_type = self.relation_type_manager.get(&relation_ty);
        // let relation_type = self.relation_type_manager.get_starts_with(&relation_instance.ty);
        match relation_type {
            Some(relation_type) => {
                let id = relation_instance.id();
                if self.reactive_relation_manager.has(&id) {
                    if let Some(reactive_relation_instance) = self.reactive_relation_manager.get(&id) {
                        return Ok(reactive_relation_instance);
                    }
                }
                let relation_instance = relation_instance;
                // Add properties from relation type if not existing
                for property in relation_type.properties.iter() {
                    if !relation_instance.properties.contains_key(property.key()) {
                        relation_instance.properties.insert(property.key().clone(), property.data_type.default_value());
                    }
                }
                // Add properties from components if not existing
                for component_ty in relation_type.components.iter() {
                    if let Some(component) = self.component_manager.get(&component_ty) {
                        for property in component.properties.iter() {
                            if !relation_instance.properties.contains_key(property.key()) {
                                relation_instance.properties.insert(property.key().clone(), property.data_type.default_value());
                            }
                        }
                    }
                }
                self.reactive_relation_manager.create_reactive_instance(relation_instance)
            }
            None => Err(ReactiveRelationCreationError::UnknownRelationType(relation_ty.clone())),
        }
    }

    fn add_component(&self, id: &RelationInstanceId, component: &ComponentTypeId) -> Result<(), ReactiveRelationComponentAddError> {
        self.reactive_relation_manager.add_component(id, component)
    }

    fn remove_component(&self, id: &RelationInstanceId, component: &ComponentTypeId) -> Result<(), ReactiveRelationComponentRemoveError> {
        self.reactive_relation_manager.remove_component(id, component)
    }

    fn add_property(&self, id: &RelationInstanceId, property_name: &str, mutability: Mutability, value: Value) -> Result<(), ReactiveRelationPropertyAddError> {
        self.reactive_relation_manager.add_property(id, property_name, mutability, value)
    }

    fn remove_property(&self, id: &RelationInstanceId, property_name: &str) -> Result<(), ReactiveRelationPropertyRemoveError> {
        self.reactive_relation_manager.remove_property(id, property_name)
    }

    fn delete(&self, id: &RelationInstanceId) -> bool {
        self.reactive_relation_manager.delete(id)
    }
}
