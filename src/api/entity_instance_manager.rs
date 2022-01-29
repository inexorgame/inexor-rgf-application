use std::collections::HashMap;
use std::fmt;

use async_trait::async_trait;
use serde_json::Value;
use uuid::Uuid;

use crate::api::EntityVertexCreationError;
use crate::model::EntityInstance;

#[derive(Debug)]
pub enum EntityInstanceCreationError {
    EntityVertexCreationError(EntityVertexCreationError),
}

impl fmt::Display for EntityInstanceCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            EntityInstanceCreationError::EntityVertexCreationError(error) => {
                write!(f, "Failed to create entity instance: {}", error)
            }
        }
    }
}

#[derive(Debug)]
pub enum EntityInstanceImportError {
    Io(std::io::Error),
    Deserialization(serde_json::Error),
    EntityAlreadyExists(Uuid),
    VertexCreation(EntityVertexCreationError),
}

impl From<std::io::Error> for EntityInstanceImportError {
    fn from(e: std::io::Error) -> Self {
        EntityInstanceImportError::Io(e)
    }
}

impl From<serde_json::Error> for EntityInstanceImportError {
    fn from(e: serde_json::Error) -> Self {
        EntityInstanceImportError::Deserialization(e)
    }
}

impl From<EntityVertexCreationError> for EntityInstanceImportError {
    fn from(e: EntityVertexCreationError) -> Self {
        EntityInstanceImportError::VertexCreation(e)
    }
}

#[async_trait]
pub trait EntityInstanceManager: Send + Sync {
    /// Returns true, if an entity instance exists with the given UUID.
    fn has(&self, id: Uuid) -> bool;

    /// Returns the entity instance with the given UUID or None.
    fn get(&self, id: Uuid) -> Option<EntityInstance>;

    // TODO: return Result<EntityInstance, EntityInstanceCreationError>
    fn create(&self, type_name: String, properties: HashMap<String, Value>) -> Result<Uuid, EntityInstanceCreationError>;

    // TODO: return Result<EntityInstance, EntityInstanceCreationError>
    fn create_with_id(&self, type_name: String, id: Uuid, properties: HashMap<String, Value>) -> Result<Uuid, EntityInstanceCreationError>;

    fn create_from_instance(&self, entity_instance: EntityInstance) -> Result<Uuid, EntityInstanceCreationError>;

    // TODO: return result
    fn commit(&self, entity_instance: EntityInstance);

    // TODO: return result
    fn delete(&self, id: Uuid);

    fn import(&self, path: String) -> Result<Uuid, EntityInstanceImportError>;

    // TODO: return result
    fn export(&self, id: Uuid, path: String);
}
