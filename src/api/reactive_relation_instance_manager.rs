use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use async_trait::async_trait;
use indradb::{EdgeKey, ValidationError};
use serde_json::Value;
use uuid::Uuid;

use crate::api::RelationInstanceCreationError;
use crate::model::{ReactiveRelationInstance, RelationInstance};

#[derive(Debug)]
pub enum ReactiveRelationInstanceCreationError {
    InvalidEdgeKey,
    MissingOutboundEntityInstance(Uuid),
    MissingInboundEntityInstance(Uuid),
    MissingInstance,
    RelationInstanceCreationError(RelationInstanceCreationError),
    ValidationError(ValidationError),
}

impl fmt::Display for ReactiveRelationInstanceCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.clone() {
            // ReactiveRelationInstanceCreationError::UuidTaken(id) => write!(f, "The UUID {} has been already taken!", id),
            ReactiveRelationInstanceCreationError::InvalidEdgeKey => {
                write!(f, "The edge key is invalid")
            }
            ReactiveRelationInstanceCreationError::MissingOutboundEntityInstance(id) => {
                write!(f, "The outbound entity instance {} cannot be found", id)
            }
            ReactiveRelationInstanceCreationError::MissingInboundEntityInstance(id) => {
                write!(f, "The inbound entity instance {} cannot be found", id)
            }
            ReactiveRelationInstanceCreationError::MissingInstance => {
                write!(f, "The created instance cannot be found")
            }
            ReactiveRelationInstanceCreationError::RelationInstanceCreationError(error) => write!(
                f,
                "Failed to create reactive relation instance: {}",
                error.to_string()
            ),
            ReactiveRelationInstanceCreationError::ValidationError(error) => {
                write!(f, "Validation Error: {}", error.to_string())
            }
        }
    }
}

#[derive(Debug)]
pub struct ReactiveRelationInstanceImportError;

#[async_trait]
pub trait ReactiveRelationInstanceManager: Send + Sync {
    /// Returns true, if an relation of the given type exists which starts at the given outbound entity and
    /// ends at the given inbound entity.
    fn has(&self, edge_key: EdgeKey) -> bool;

    /// Returns the ReactiveRelationInstance with the given type_name, which starts at the given
    /// outbound entity and ends at the given inbound entity.
    fn get(&self, edge_key: EdgeKey) -> Option<Arc<ReactiveRelationInstance>>;

    // TODO: Rename to: "get_all"
    fn get_relation_instances(&self) -> Vec<Arc<ReactiveRelationInstance>>;

    fn get_by_outbound_entity(
        &self,
        outbound_entity_id: Uuid,
    ) -> Vec<Arc<ReactiveRelationInstance>>;

    fn get_by_inbound_entity(&self, inbound_entity_id: Uuid) -> Vec<Arc<ReactiveRelationInstance>>;

    fn create(
        &self,
        edge_key: EdgeKey,
        properties: HashMap<String, Value>,
    ) -> Result<Arc<ReactiveRelationInstance>, ReactiveRelationInstanceCreationError>;

    fn create_reactive_instance(
        &self,
        relation_instance: RelationInstance,
    ) -> Result<Arc<ReactiveRelationInstance>, ReactiveRelationInstanceCreationError>;

    fn register_reactive_instance(&self, reactive_relation_instance: Arc<ReactiveRelationInstance>);

    // TODO: fn commit(&self, relation_instance: RelationInstance);
    // TODO: return result
    fn commit(&self, edge_key: EdgeKey);

    fn delete(&self, edge_key: EdgeKey) -> bool;

    fn unregister_reactive_instance(&self, edge_key: EdgeKey);

    fn import(
        &self,
        path: String,
    ) -> Result<Arc<ReactiveRelationInstance>, ReactiveRelationInstanceImportError>;

    // TODO: return result
    fn export(&self, edge_key: EdgeKey, path: String);
}