use std::ops::Deref;
use std::sync::Arc;
use std::sync::RwLock;
use std::time;

use async_trait::async_trait;
use serde_json::json;
use tokio::task;

use crate::api::EntityTypeManager;
use crate::api::Lifecycle;
use crate::api::ReactiveEntityInstanceManager;
use crate::api::ShutdownManager;
use crate::api::UUID_SHUTDOWN;
use crate::api::UUID_SHUTDOWN_TRIGGER;
use crate::builder::EntityTypeBuilder;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::core_model::COMPONENT_ACTION;
use crate::core_model::COMPONENT_LABELED;
use crate::core_model::ENTITY_TYPE_SHUTDOWN;
use crate::core_model::PROPERTY_LABEL;
use crate::core_model::PROPERTY_RESULT;
use crate::core_model::PROPERTY_SHUTDOWN;
use crate::core_model::PROPERTY_TRIGGER;
use crate::di::*;
use crate::model::DataType;
use crate::model::ReactivePropertyContainer;

#[wrapper]
pub struct ShutdownStateContainer(Arc<RwLock<bool>>);

#[provides]
fn create_shutdown_state() -> ShutdownStateContainer {
    ShutdownStateContainer(Arc::new(RwLock::new(false)))
}

#[component]
pub struct ShutdownManagerImpl {
    entity_type_manager: Wrc<dyn EntityTypeManager>,
    reactive_entity_instance_manager: Wrc<dyn ReactiveEntityInstanceManager>,

    shutdown_state: ShutdownStateContainer,
}

#[async_trait]
#[provides]
impl ShutdownManager for ShutdownManagerImpl {
    fn do_shutdown(&self) {
        let mut guard = self.shutdown_state.0.write().unwrap();
        *guard = true;
    }

    fn is_shutdown(&self) -> bool {
        *self.shutdown_state.0.read().unwrap().deref()
    }
}

impl Lifecycle for ShutdownManagerImpl {
    fn init(&self) {
        let entity_type = EntityTypeBuilder::new(&ENTITY_TYPE_SHUTDOWN.clone())
            .property(PROPERTY_SHUTDOWN, DataType::Bool)
            .property(PROPERTY_TRIGGER, DataType::Bool)
            .component(&COMPONENT_LABELED.clone())
            .component(&COMPONENT_ACTION.clone())
            .build();
        let _ = self.entity_type_manager.register(entity_type);
        let shutdown_handler = ReactiveEntityInstanceBuilder::new(&ENTITY_TYPE_SHUTDOWN.clone())
            .id(UUID_SHUTDOWN)
            .property(PROPERTY_LABEL, json!("/org/inexor/system/shutdown"))
            .property(PROPERTY_SHUTDOWN, json!(false))
            .property(PROPERTY_TRIGGER, json!(false))
            .property(PROPERTY_RESULT, json!(false))
            .component(&COMPONENT_LABELED.clone())
            .component(&COMPONENT_ACTION.clone())
            .build();
        let _ = self.reactive_entity_instance_manager.register_reactive_instance(shutdown_handler.clone());
        let shutdown_state = self.shutdown_state.0.clone();
        shutdown_handler.observe_with_handle(
            PROPERTY_TRIGGER,
            move |v| {
                if v.is_boolean() && v.as_bool().unwrap() {
                    let mut guard = shutdown_state.write().unwrap();
                    *guard = true;
                }
            },
            UUID_SHUTDOWN_TRIGGER.as_u128(),
        );
        let shutdown_state = self.shutdown_state.0.clone();
        shutdown_handler.observe_with_handle(
            PROPERTY_SHUTDOWN,
            move |v| {
                if v.is_boolean() && v.as_bool().unwrap() {
                    let mut guard = shutdown_state.write().unwrap();
                    *guard = true;
                }
                if v.is_number() {
                    let shutdown_in_seconds = time::Duration::from_secs(v.as_u64().unwrap());
                    let shutdown_state_deferred = shutdown_state.clone();
                    task::spawn(async move {
                        tokio::time::sleep(shutdown_in_seconds).await;
                        let mut guard = shutdown_state_deferred.write().unwrap();
                        *guard = true;
                    });
                }
            },
            UUID_SHUTDOWN.as_u128(),
        );
    }

    fn shutdown(&self) {
        // Disconnect reactive streams of the shutdown handler
        if let Some(shutdown_handler) = self.reactive_entity_instance_manager.get(UUID_SHUTDOWN) {
            shutdown_handler.remove_observer(PROPERTY_TRIGGER, UUID_SHUTDOWN_TRIGGER.as_u128());
            shutdown_handler.remove_observer(PROPERTY_SHUTDOWN, UUID_SHUTDOWN.as_u128());
        }
    }
}