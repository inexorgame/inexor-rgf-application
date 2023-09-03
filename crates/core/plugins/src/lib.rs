#![feature(register_tool)]
#![feature(concat_idents)]
#![register_tool(tarpaulin)]

pub use command_manager::*;
pub use component_manager::*;
pub use component_import_export_manager::*;
pub use component_provider::*;
pub use config_manager::*;
pub use embedded_asset_provider::*;
pub use entity_behaviour_registry::*;
pub use entity_component_behaviour_registry::*;
pub use entity_instance_manager::*;
pub use entity_type_manager::*;
pub use entity_type_import_export_manager::*;
pub use entity_type_provider::*;
pub use flow_instance_manager::*;
pub use flow_instance_provider::*;
pub use flow_type_manager::*;
pub use flow_type_import_export_manager::*;
pub use flow_type_provider::*;
pub use graphql_query_service::*;
pub use http_body::HttpBody;
pub use plugin::Plugin;
pub use plugin::PluginActivationError;
pub use plugin::PluginDeactivationError;
pub use plugin::PluginLoadingError;
pub use plugin::PluginUnloadingError;
pub use plugin::PLUGIN_NAME_PREFIX;
pub use plugin_context::*;
pub use plugin_declaration::*;
pub use plugin_dependency::*;
pub use plugin_state::PluginDeployError;
pub use plugin_state::PluginDisableError;
pub use plugin_state::PluginRefreshingState;
pub use plugin_state::PluginResolveState;
pub use plugin_state::PluginStartError;
pub use plugin_state::PluginStartingState;
pub use plugin_state::PluginState;
pub use plugin_state::PluginStopError;
pub use plugin_state::PluginStoppingState;
pub use plugin_state::PluginUninstallError;
pub use plugin_state::PluginUninstallingState;
pub use relation_behaviour_registry::*;
pub use relation_component_behaviour_registry::*;
pub use relation_instance_manager::*;
pub use relation_type_manager::*;
pub use relation_type_import_export_manager::*;
pub use relation_type_provider::*;
pub use system_event_manager::*;
pub use system_events::*;
pub use web_resource_provider::*;

use inexor_rgf_behaviour as behaviour;
use inexor_rgf_core_config as config;
use inexor_rgf_core_model as model;
use inexor_rgf_reactive as reactive;
use inexor_rgf_model_command as model_command;

pub mod command_manager;
pub mod component_manager;
pub mod component_import_export_manager;
pub mod component_provider;
pub mod config_manager;
pub mod embedded_asset_provider;
pub mod entity_behaviour_registry;
pub mod entity_component_behaviour_registry;
pub mod entity_instance_manager;
pub mod entity_type_manager;
pub mod entity_type_import_export_manager;
pub mod entity_type_provider;
pub mod flow_instance_manager;
pub mod flow_instance_provider;
pub mod flow_type_manager;
pub mod flow_type_import_export_manager;
pub mod flow_type_provider;
pub mod graphql_query_service;
pub mod http_body;
pub mod plugin;
pub mod plugin_context;
pub mod plugin_declaration;
pub mod plugin_dependency;
pub mod plugin_state;
pub mod relation_behaviour_registry;
pub mod relation_component_behaviour_registry;
pub mod relation_instance_manager;
pub mod relation_type_manager;
pub mod relation_type_import_export_manager;
pub mod relation_type_provider;
pub mod system_event_manager;
pub mod system_events;
pub mod web_resource_provider;

pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");
pub static PLUGIN_API_VERSION: &str = env!("CARGO_PKG_VERSION");

#[macro_export]
macro_rules! export_plugin {
    ($register:expr, $get_dependencies:expr, $name:expr, $description:expr, $version:expr) => {
        #[doc(hidden)]
        #[no_mangle]
        pub static plugin_declaration: $crate::PluginDeclaration = $crate::PluginDeclaration {
            rustc_version: $crate::RUSTC_VERSION,
            plugin_api_version: $crate::PLUGIN_API_VERSION,
            name: $name,
            description: $description,
            version: $version,
            register: $register,
            get_dependencies: $get_dependencies,
        };
    };
}

#[cfg(test)]
#[tarpaulin::ignore]
pub mod tests;
