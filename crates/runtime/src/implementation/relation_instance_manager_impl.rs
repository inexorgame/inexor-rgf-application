use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use async_trait::async_trait;
use indradb::EdgeKey;
use log::error;
use serde_json::Value;
use uuid::Uuid;

use crate::api::EntityInstanceManager;
use crate::api::RelationEdgeManager;
use crate::api::RelationInstanceCreationError;
use crate::api::RelationInstanceImportError;
use crate::api::RelationInstanceManager;
use crate::di::*;
use crate::model::NamespacedTypeGetter;
use crate::model::RelationInstance;

#[component]
pub struct RelationInstanceManagerImpl {
    relation_edge_manager: Wrc<dyn RelationEdgeManager>,

    entity_instance_manager: Wrc<dyn EntityInstanceManager>,
}

#[async_trait]
#[provides]
impl RelationInstanceManager for RelationInstanceManagerImpl {
    fn has(&self, edge_key: &EdgeKey) -> bool {
        self.relation_edge_manager.has(edge_key)
    }

    fn get(&self, edge_key: &EdgeKey) -> Option<RelationInstance> {
        self.relation_edge_manager
            .get_properties(edge_key)
            .and_then(|properties| RelationInstance::try_from(properties).ok())
    }

    fn get_by_outbound_entity(&self, outbound_entity_id: Uuid) -> Vec<RelationInstance> {
        self.relation_edge_manager
            .get_by_outbound_entity(outbound_entity_id)
            .iter()
            .map(|edge| edge.key.clone())
            .filter_map(|edge_key| self.get(&edge_key))
            .collect()
    }

    fn get_by_inbound_entity(&self, inbound_entity_id: Uuid) -> Vec<RelationInstance> {
        self.relation_edge_manager
            .get_by_outbound_entity(inbound_entity_id)
            .iter()
            .map(|edge| edge.key.clone())
            .filter_map(|edge_key| self.get(&edge_key))
            .collect()
    }

    fn create(&self, edge_key: &EdgeKey, properties: HashMap<String, Value>) -> Result<EdgeKey, RelationInstanceCreationError> {
        if self.relation_edge_manager.has(edge_key) {
            // Edge already exists!
            return Err(RelationInstanceCreationError::EdgeAlreadyExists(edge_key.clone()));
        }
        if !self.entity_instance_manager.has(edge_key.outbound_id) {
            // Outbound entity does not exist!
            return Err(RelationInstanceCreationError::MissingOutboundEntityInstance(edge_key.outbound_id));
        }
        if !self.entity_instance_manager.has(edge_key.inbound_id) {
            // Inbound entity does not exist!
            return Err(RelationInstanceCreationError::MissingInboundEntityInstance(edge_key.inbound_id));
        }
        self.relation_edge_manager
            .create(edge_key, properties)
            .map_err(RelationInstanceCreationError::RelationEdgeCreationError)
    }

    fn create_from_instance(&self, relation_instance: RelationInstance) -> Result<EdgeKey, RelationInstanceCreationError> {
        self.create(&relation_instance.get_key(), relation_instance.properties)
    }

    fn create_from_instance_if_not_exist(&self, relation_instance: RelationInstance) -> Result<EdgeKey, RelationInstanceCreationError> {
        let edge_key = relation_instance.get_key();
        if self.relation_edge_manager.has(&edge_key) {
            Ok(edge_key)
        } else {
            self.create_from_instance(relation_instance)
        }
    }

    fn commit(&self, relation_instance: RelationInstance) {
        self.relation_edge_manager.commit(&relation_instance.get_key(), relation_instance.properties);
    }

    fn delete(&self, edge_key: &EdgeKey) -> bool {
        self.relation_edge_manager.delete(edge_key)
    }

    fn import(&self, path: &str) -> Result<RelationInstance, RelationInstanceImportError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let relation_instance: RelationInstance = serde_json::from_reader(reader)?;
        let edge_key = relation_instance.get_key();
        if self.has(&edge_key) {
            return Err(RelationInstanceImportError::RelationAlreadyExists(edge_key));
        }
        self.relation_edge_manager
            .create(&edge_key, relation_instance.properties.clone())
            .map(|_| relation_instance)
            .map_err(RelationInstanceImportError::RelationEdgeCreation)
    }

    fn export(&self, edge_key: &EdgeKey, path: &str) {
        if let Some(relation_instance) = self.get(edge_key) {
            let r_file = File::create(path);
            match r_file {
                Ok(file) => {
                    let result = serde_json::to_writer_pretty(&file, &relation_instance);
                    if result.is_err() {
                        // TODO: implement Display trait for RelationInstance
                        error!(
                            "Failed to export relation instance {} {} {} to {}: {}",
                            relation_instance.outbound_id,
                            relation_instance.type_name(),
                            relation_instance.inbound_id,
                            path,
                            result.err().unwrap()
                        );
                    }
                }
                Err(error) => {
                    // TODO: implement Display trait for RelationInstance
                    error!(
                        "Failed to export relation instance {} {} {} to {}: {}",
                        relation_instance.outbound_id,
                        relation_instance.type_name(),
                        relation_instance.inbound_id,
                        path,
                        error.to_string()
                    );
                }
            }
        }
    }
}