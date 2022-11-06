use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use async_trait::async_trait;
use log::error;
use serde_json::Value;
use uuid::Uuid;

use crate::api::EntityInstanceCreationError;
use crate::api::EntityInstanceImportError;
use crate::api::EntityInstanceManager;
use crate::api::EntityVertexCreationError;
use crate::api::EntityVertexManager;
use crate::di::component;
use crate::di::provides;
use crate::di::Component;
use crate::di::Wrc;
use crate::model::EntityInstance;
use crate::model::EntityTypeId;

#[component]
pub struct EntityInstanceManagerImpl {
    entity_vertex_manager: Wrc<dyn EntityVertexManager>,
}

#[async_trait]
#[provides]
impl EntityInstanceManager for EntityInstanceManagerImpl {
    fn has(&self, id: Uuid) -> bool {
        self.entity_vertex_manager.has(id)
    }

    fn get(&self, id: Uuid) -> Option<EntityInstance> {
        self.entity_vertex_manager
            .get_properties(id)
            .and_then(|properties| EntityInstance::try_from(properties).ok())
    }

    fn create(&self, ty: &EntityTypeId, properties: HashMap<String, Value, RandomState>) -> Result<Uuid, EntityInstanceCreationError> {
        let result = self.entity_vertex_manager.create(ty, properties);
        if result.is_err() {
            return Err(EntityInstanceCreationError::EntityVertexCreationError(result.err().unwrap()));
        }
        Ok(result.unwrap())
    }

    fn create_with_id(&self, ty: &EntityTypeId, id: Uuid, properties: HashMap<String, Value, RandomState>) -> Result<Uuid, EntityInstanceCreationError> {
        let result = self.entity_vertex_manager.create_with_id(ty, id, properties);
        if result.is_err() {
            return Err(EntityInstanceCreationError::EntityVertexCreationError(result.err().unwrap()));
        }
        Ok(result.unwrap())
    }

    fn create_from_instance(&self, entity_instance: EntityInstance) -> Result<Uuid, EntityInstanceCreationError> {
        self.entity_vertex_manager
            .create_with_id(&entity_instance.ty, entity_instance.id, entity_instance.properties)
            .map_err(EntityInstanceCreationError::EntityVertexCreationError)
    }

    fn create_from_instance_if_not_exist(&self, entity_instance: EntityInstance) -> Result<Uuid, EntityInstanceCreationError> {
        if self.entity_vertex_manager.has(entity_instance.id) {
            Ok(entity_instance.id)
        } else {
            self.create_from_instance(entity_instance)
        }
    }

    fn commit(&self, entity_instance: EntityInstance) {
        self.entity_vertex_manager.commit(entity_instance.id, entity_instance.properties);
    }

    fn delete(&self, id: Uuid) {
        self.entity_vertex_manager.delete(id);
    }

    fn import(&self, path: &str) -> Result<Uuid, EntityInstanceImportError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let entity_instance: EntityInstance = serde_json::from_reader(reader)?;
        if self.has(entity_instance.id) {
            return Err(EntityInstanceImportError::EntityAlreadyExists(entity_instance.id));
        }
        self.entity_vertex_manager
            .create_with_id(&entity_instance.ty, entity_instance.id, entity_instance.properties)
            .map_err(EntityVertexCreationError::into)
    }

    fn export(&self, id: Uuid, path: &str) {
        if let Some(entity_instance) = self.get(id) {
            match File::create(path) {
                Ok(file) => {
                    if let Err(error) = serde_json::to_writer_pretty(&file, &entity_instance) {
                        error!("Failed to export entity instance {} to {}: {}", id, path, error);
                    }
                }
                Err(error) => error!("Failed to export entity instance {} to {}: {}", id, path, error.to_string()),
            }
        }
    }
}
