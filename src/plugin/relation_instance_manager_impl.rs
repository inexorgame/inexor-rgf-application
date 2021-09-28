use crate::api::{ReactiveRelationInstanceManager, RelationTypeManager};
use crate::model::{ReactiveRelationInstance, RelationInstance};
use crate::plugins::relation_instance_manager::{
    RelationInstanceCreationError, RelationInstanceManager,
};
use indradb::EdgeKey;
use std::sync::Arc;
use uuid::Uuid;

pub struct RelationInstanceManagerImpl {
    relation_type_manager: Arc<dyn RelationTypeManager>,
    reactive_relation_instance_manager: Arc<dyn ReactiveRelationInstanceManager>,
}

impl RelationInstanceManagerImpl {
    pub fn new(
        relation_type_manager: Arc<dyn RelationTypeManager>,
        reactive_relation_instance_manager: Arc<dyn ReactiveRelationInstanceManager>,
    ) -> Self {
        Self {
            relation_type_manager,
            reactive_relation_instance_manager,
        }
    }
}
impl RelationInstanceManager for RelationInstanceManagerImpl {
    fn has(&self, edge_key: EdgeKey) -> bool {
        self.reactive_relation_instance_manager.has(edge_key)
    }

    fn get(&self, edge_key: EdgeKey) -> Option<Arc<ReactiveRelationInstance>> {
        self.reactive_relation_instance_manager.get(edge_key)
    }

    fn get_by_outbound_entity(
        &self,
        outbound_entity_id: Uuid,
    ) -> Vec<Arc<ReactiveRelationInstance>> {
        self.reactive_relation_instance_manager
            .get_by_outbound_entity(outbound_entity_id)
    }

    fn get_by_inbound_entity(&self, inbound_entity_id: Uuid) -> Vec<Arc<ReactiveRelationInstance>> {
        self.reactive_relation_instance_manager
            .get_by_inbound_entity(inbound_entity_id)
    }

    fn create(
        &self,
        relation_instance: RelationInstance,
    ) -> Result<Arc<ReactiveRelationInstance>, RelationInstanceCreationError> {
        let relation_type = self
            .relation_type_manager
            .get_starts_with(relation_instance.type_name.clone());
        match relation_type {
            Some(relation_type) => {
                let mut relation_instance = relation_instance.clone();
                for property in relation_type.properties.iter() {
                    if !relation_instance.properties.contains_key(&property.name) {
                        relation_instance
                            .properties
                            .insert(property.name.clone(), property.data_type.default_value());
                    }
                }
                let reactive_relation_instance = self
                    .reactive_relation_instance_manager
                    .create_reactive_instance(relation_instance);
                match reactive_relation_instance {
                    Ok(reactive_relation_instance) => Ok(reactive_relation_instance),
                    Err(_) => {
                        return Err(RelationInstanceCreationError::Failed);
                    }
                }
            }
            None => {
                return Err(RelationInstanceCreationError::Failed);
            }
        }
    }

    fn delete(&self, edge_key: EdgeKey) -> bool {
        self.reactive_relation_instance_manager.delete(edge_key)
    }
}