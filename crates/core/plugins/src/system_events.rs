use uuid::Uuid;
use crate::model::RelationInstanceId;

use crate::model::ComponentTypeId;
use crate::model::EntityTypeId;
use crate::model::ExtensionTypeId;
use crate::model::FlowTypeId;
use crate::model::RelationTypeId;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum SystemEventTypes {
    ComponentCreated,
    ComponentPropertyAdded,
    ComponentPropertyRenamed,
    ComponentPropertyUpdated,
    ComponentPropertyRemoved,
    ComponentExtensionAdded,
    ComponentExtensionRenamed,
    ComponentExtensionUpdated,
    ComponentExtensionRemoved,
    ComponentDeleted,
    EntityTypeCreated,
    EntityTypeComponentAdded,
    EntityTypeComponentRenamed,
    EntityTypeComponentUpdated,
    EntityTypeComponentRemoved,
    EntityTypePropertyAdded,
    EntityTypePropertyRenamed,
    EntityTypePropertyUpdated,
    EntityTypePropertyRemoved,
    EntityTypeExtensionAdded,
    EntityTypeExtensionRenamed,
    EntityTypeExtensionUpdated,
    EntityTypeExtensionRemoved,
    EntityTypeDeleted,
    RelationTypeCreated,
    RelationTypeComponentAdded,
    RelationTypeComponentRenamed,
    RelationTypeComponentUpdated,
    RelationTypeComponentRemoved,
    RelationTypePropertyAdded,
    RelationTypePropertyRenamed,
    RelationTypePropertyUpdated,
    RelationTypePropertyRemoved,
    RelationTypeExtensionAdded,
    RelationTypeExtensionRenamed,
    RelationTypeExtensionUpdated,
    RelationTypeExtensionRemoved,
    RelationTypeDeleted,
    FlowTypeCreated,
    FlowTypeUpdated,
    FlowTypeDeleted,

    /// The type system has changed
    TypeSystemChanged,

    EntityInstanceCreated,
    EntityInstanceDeleted,
    RelationInstanceCreated,
    RelationInstanceDeleted,
    FlowInstanceCreated,
    FlowInstanceDeleted,
}

pub enum SystemEvent {
    ComponentCreated(ComponentTypeId),
    ComponentPropertyAdded(ComponentTypeId, String),
    ComponentPropertyRenamed(ComponentTypeId, String, String),
    ComponentPropertyUpdated(ComponentTypeId, String),
    ComponentPropertyRemoved(ComponentTypeId, String),
    ComponentExtensionAdded(ComponentTypeId, ExtensionTypeId),
    ComponentExtensionRenamed(ComponentTypeId, ExtensionTypeId, ExtensionTypeId),
    ComponentExtensionUpdated(ComponentTypeId, ExtensionTypeId),
    ComponentExtensionRemoved(ComponentTypeId, ExtensionTypeId),
    ComponentDeleted(ComponentTypeId),
    EntityTypeCreated(EntityTypeId),
    EntityTypeComponentAdded(EntityTypeId, ComponentTypeId),
    EntityTypeComponentRenamed(EntityTypeId, ComponentTypeId, ComponentTypeId),
    EntityTypeComponentUpdated(EntityTypeId, ComponentTypeId),
    EntityTypeComponentRemoved(EntityTypeId, ComponentTypeId),
    EntityTypePropertyAdded(EntityTypeId, String),
    EntityTypePropertyRenamed(EntityTypeId, String, String),
    EntityTypePropertyUpdated(EntityTypeId, String),
    EntityTypePropertyRemoved(EntityTypeId, String),
    EntityTypeExtensionAdded(EntityTypeId, ExtensionTypeId),
    EntityTypeExtensionRenamed(EntityTypeId, ExtensionTypeId, ExtensionTypeId),
    EntityTypeExtensionUpdated(EntityTypeId, ExtensionTypeId),
    EntityTypeExtensionRemoved(EntityTypeId, ExtensionTypeId),
    EntityTypeDeleted(EntityTypeId),
    RelationTypeCreated(RelationTypeId),
    RelationTypeComponentAdded(RelationTypeId, ComponentTypeId),
    RelationTypeComponentRenamed(EntityTypeId, ComponentTypeId, ComponentTypeId),
    RelationTypeComponentUpdated(EntityTypeId, ComponentTypeId),
    RelationTypeComponentRemoved(RelationTypeId, ComponentTypeId),
    RelationTypePropertyAdded(RelationTypeId, String),
    RelationTypePropertyRenamed(EntityTypeId, String, String),
    RelationTypePropertyUpdated(EntityTypeId, String),
    RelationTypePropertyRemoved(RelationTypeId, String),
    RelationTypeExtensionAdded(RelationTypeId, ExtensionTypeId),
    RelationTypeExtensionRenamed(EntityTypeId, ExtensionTypeId, ExtensionTypeId),
    RelationTypeExtensionUpdated(EntityTypeId, ExtensionTypeId),
    RelationTypeExtensionRemoved(RelationTypeId, ExtensionTypeId),
    RelationTypeDeleted(RelationTypeId),
    FlowTypeCreated(FlowTypeId),
    // TODO: Replace FlowTypeUpdated with more concrete events
    FlowTypeUpdated(FlowTypeId),
    FlowTypeDeleted(FlowTypeId),
    TypeSystemChanged,
    EntityInstanceCreated(Uuid),
    EntityInstanceDeleted(Uuid),
    RelationInstanceCreated(RelationInstanceId),
    RelationInstanceDeleted(RelationInstanceId),
    FlowInstanceCreated(Uuid),
    FlowInstanceDeleted(Uuid),
}

impl From<&SystemEvent> for SystemEventTypes {
    fn from(event: &SystemEvent) -> Self {
        match event {
            SystemEvent::ComponentCreated(_) => SystemEventTypes::ComponentCreated,
            SystemEvent::ComponentPropertyAdded(_, _) => SystemEventTypes::ComponentPropertyAdded,
            SystemEvent::ComponentPropertyRenamed(_, _, _) => SystemEventTypes::ComponentPropertyRenamed,
            SystemEvent::ComponentPropertyUpdated(_, _) => SystemEventTypes::ComponentPropertyUpdated,
            SystemEvent::ComponentPropertyRemoved(_, _) => SystemEventTypes::ComponentPropertyRemoved,
            SystemEvent::ComponentExtensionAdded(_, _) => SystemEventTypes::ComponentExtensionAdded,
            SystemEvent::ComponentExtensionRenamed(_, _, _) => SystemEventTypes::ComponentExtensionRenamed,
            SystemEvent::ComponentExtensionUpdated(_, _) => SystemEventTypes::ComponentExtensionUpdated,
            SystemEvent::ComponentExtensionRemoved(_, _) => SystemEventTypes::ComponentExtensionRemoved,
            SystemEvent::ComponentDeleted(_) => SystemEventTypes::ComponentDeleted,
            SystemEvent::EntityTypeCreated(_) => SystemEventTypes::EntityTypeCreated,
            SystemEvent::EntityTypeComponentAdded(_, _) => SystemEventTypes::EntityTypeComponentAdded,
            SystemEvent::EntityTypeComponentRenamed(_, _, _) => SystemEventTypes::EntityTypeComponentRenamed,
            SystemEvent::EntityTypeComponentUpdated(_, _) => SystemEventTypes::EntityTypeComponentUpdated,
            SystemEvent::EntityTypeComponentRemoved(_, _) => SystemEventTypes::EntityTypeComponentRemoved,
            SystemEvent::EntityTypePropertyAdded(_, _) => SystemEventTypes::EntityTypePropertyAdded,
            SystemEvent::EntityTypePropertyRenamed(_, _, _) => SystemEventTypes::EntityTypePropertyRenamed,
            SystemEvent::EntityTypePropertyUpdated(_, _) => SystemEventTypes::EntityTypePropertyUpdated,
            SystemEvent::EntityTypePropertyRemoved(_, _) => SystemEventTypes::EntityTypePropertyRemoved,
            SystemEvent::EntityTypeExtensionAdded(_, _) => SystemEventTypes::EntityTypeExtensionAdded,
            SystemEvent::EntityTypeExtensionRenamed(_, _, _) => SystemEventTypes::EntityTypeExtensionRenamed,
            SystemEvent::EntityTypeExtensionUpdated(_, _) => SystemEventTypes::EntityTypeExtensionUpdated,
            SystemEvent::EntityTypeExtensionRemoved(_, _) => SystemEventTypes::EntityTypeExtensionRemoved,
            SystemEvent::EntityTypeDeleted(_) => SystemEventTypes::EntityTypeDeleted,
            SystemEvent::RelationTypeCreated(_) => SystemEventTypes::RelationTypeCreated,
            SystemEvent::RelationTypeComponentAdded(_, _) => SystemEventTypes::RelationTypeComponentAdded,
            SystemEvent::RelationTypeComponentRenamed(_, _, _) => SystemEventTypes::RelationTypeComponentRenamed,
            SystemEvent::RelationTypeComponentUpdated(_, _) => SystemEventTypes::RelationTypeComponentUpdated,
            SystemEvent::RelationTypeComponentRemoved(_, _) => SystemEventTypes::RelationTypeComponentRemoved,
            SystemEvent::RelationTypePropertyAdded(_, _) => SystemEventTypes::RelationTypePropertyAdded,
            SystemEvent::RelationTypePropertyRenamed(_, _, _) => SystemEventTypes::RelationTypePropertyRenamed,
            SystemEvent::RelationTypePropertyUpdated(_, _) => SystemEventTypes::RelationTypePropertyUpdated,
            SystemEvent::RelationTypePropertyRemoved(_, _) => SystemEventTypes::RelationTypePropertyRemoved,
            SystemEvent::RelationTypeExtensionAdded(_, _) => SystemEventTypes::RelationTypeExtensionAdded,
            SystemEvent::RelationTypeExtensionRenamed(_, _, _) => SystemEventTypes::RelationTypeExtensionRenamed,
            SystemEvent::RelationTypeExtensionUpdated(_, _) => SystemEventTypes::RelationTypeExtensionUpdated,
            SystemEvent::RelationTypeExtensionRemoved(_, _) => SystemEventTypes::RelationTypeExtensionRemoved,
            SystemEvent::RelationTypeDeleted(_) => SystemEventTypes::RelationTypeDeleted,
            SystemEvent::FlowTypeCreated(_) => SystemEventTypes::FlowTypeCreated,
            SystemEvent::FlowTypeUpdated(_) => SystemEventTypes::FlowTypeUpdated,
            SystemEvent::FlowTypeDeleted(_) => SystemEventTypes::FlowTypeDeleted,
            SystemEvent::TypeSystemChanged => SystemEventTypes::TypeSystemChanged,
            SystemEvent::EntityInstanceCreated(_) => SystemEventTypes::EntityInstanceCreated,
            SystemEvent::EntityInstanceDeleted(_) => SystemEventTypes::EntityInstanceDeleted,
            SystemEvent::RelationInstanceCreated(_) => SystemEventTypes::RelationInstanceCreated,
            SystemEvent::RelationInstanceDeleted(_) => SystemEventTypes::RelationInstanceDeleted,
            SystemEvent::FlowInstanceCreated(_) => SystemEventTypes::FlowInstanceCreated,
            SystemEvent::FlowInstanceDeleted(_) => SystemEventTypes::FlowInstanceDeleted,
        }
    }
}
