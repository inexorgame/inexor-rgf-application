use std::sync::Arc;

use libloading::Library;
use log::debug;

use crate::plugins::{ComponentProvider, EntityBehaviourProvider, EntityTypeProvider, FlowProvider, Plugin, PluginError, RelationBehaviourProvider, RelationTypeProvider};

/// A proxy object which wraps a [`Plugin`] and makes sure it can't outlive
/// the library it came from.
pub struct PluginProxy {
    pub(crate) plugin: Box<Arc<dyn Plugin>>,
    pub(crate) lib: Arc<Library>,
}

impl Plugin for PluginProxy {
    fn init(&self) -> Result<(), PluginError> {
        debug!("Proxy: init");
        self.plugin.init()
    }

    fn post_init(&self) -> Result<(), PluginError> {
        debug!("Proxy: post_init");
        self.plugin.post_init()
    }

    fn pre_shutdown(&self) -> Result<(), PluginError> {
        debug!("Proxy: pre_shutdown");
        self.plugin.pre_shutdown()
    }

    fn shutdown(&self) -> Result<(), PluginError> {
        debug!("Proxy: shutdown");
        self.plugin.shutdown()
    }

    fn get_component_provider(&self) -> Result<Arc<dyn ComponentProvider>, PluginError> {
        self.plugin.get_component_provider()
    }

    fn get_entity_type_provider(&self) -> Result<Arc<dyn EntityTypeProvider>, PluginError> {
        self.plugin.get_entity_type_provider()
    }

    fn get_relation_type_provider(&self) -> Result<Arc<dyn RelationTypeProvider>, PluginError> {
        self.plugin.get_relation_type_provider()
    }

    fn get_entity_behaviour_provider(&self) -> Result<Arc<dyn EntityBehaviourProvider>, PluginError> {
        self.plugin.get_entity_behaviour_provider()
    }

    fn get_relation_behaviour_provider(&self) -> Result<Arc<dyn RelationBehaviourProvider>, PluginError> {
        self.plugin.get_relation_behaviour_provider()
    }

    fn get_flow_provider(&self) -> Result<Arc<dyn FlowProvider>, PluginError> {
        self.plugin.get_flow_provider()
    }
}