use inexor_rgf_graph::ComponentOrEntityTypeId;
use inexor_rgf_graph::ComponentTypeId;
use inexor_rgf_graph::ComponentTypeIds;
use inexor_rgf_graph::Extension;
use inexor_rgf_graph::ExtensionTypeId;
use inexor_rgf_graph::Extensions;
use inexor_rgf_graph::PropertyType;
use inexor_rgf_graph::PropertyTypes;
use inexor_rgf_graph::RelationType;
use inexor_rgf_graph::RelationTypeAddComponentError;
use inexor_rgf_graph::RelationTypeAddExtensionError;
use inexor_rgf_graph::RelationTypeAddPropertyError;
use inexor_rgf_graph::RelationTypeId;
use inexor_rgf_graph::RelationTypeRemoveComponentError;
use inexor_rgf_graph::RelationTypeRemoveExtensionError;
use inexor_rgf_graph::RelationTypeRemovePropertyError;
use inexor_rgf_graph::RelationTypeUpdateExtensionError;
use inexor_rgf_graph::RelationTypeUpdatePropertyError;
use inexor_rgf_graph::RelationTypes;
use inexor_rgf_type_system_api::RelationTypeCreationError;

#[derive(Debug)]
pub enum RelationTypeManagerError {
    InitializationError,
}

pub trait RelationTypeManager: Send + Sync {
    /// Returns all relation types.
    fn get_all(&self) -> RelationTypes;

    /// Returns all relation types of the given namespace.
    fn get_by_namespace(&self, namespace: &str) -> RelationTypes;

    /// Returns true, if a relation type with the given name exists.
    fn has(&self, ty: &RelationTypeId) -> bool;

    /// Returns true, if a relation type with the given fully qualified name exists.
    fn has_by_type(&self, namespace: &str, type_name: &str) -> bool;

    /// Returns the relation type with the given name.
    fn get(&self, ty: &RelationTypeId) -> Option<RelationType>;

    /// Returns the relation type with the given fully qualified name.
    fn get_by_type(&self, namespace: &str, type_name: &str) -> Option<RelationType>;

    /// Returns all relation types whose names matches the given search string.
    fn find_by_type_name(&self, search: &str) -> RelationTypes;

    /// Returns the count of relation types.
    fn count(&self) -> usize;

    /// Returns the count of relation types of the given namespace.
    fn count_by_namespace(&self, namespace: &str) -> usize;

    /// Creates a new relation type.
    #[allow(clippy::too_many_arguments)]
    fn create(
        &self,
        outbound_type: &ComponentOrEntityTypeId,
        ty: &RelationTypeId,
        inbound_type: &ComponentOrEntityTypeId,
        description: &str,
        components: ComponentTypeIds,
        properties: PropertyTypes,
        extensions: Extensions,
    ) -> Result<RelationType, RelationTypeCreationError>;

    /// Adds the component with the given type to the given relation type.
    fn add_component(&self, ty: &RelationTypeId, component: &ComponentTypeId) -> Result<(), RelationTypeAddComponentError>;

    /// Remove the component with the given type from the given relation type.
    fn remove_component(&self, ty: &RelationTypeId, component: &ComponentTypeId) -> Result<ComponentTypeId, RelationTypeRemoveComponentError>;

    /// Adds a property to the given relation type.
    fn add_property(&self, ty: &RelationTypeId, property: PropertyType) -> Result<PropertyType, RelationTypeAddPropertyError>;

    /// Updates the property with the given property_name.
    /// It's possible to rename the property by using another name in the new property than the provided property_name.
    fn update_property(
        &self,
        relation_ty: &RelationTypeId,
        property_name: &str,
        property_type: PropertyType,
    ) -> Result<PropertyType, RelationTypeUpdatePropertyError>;

    /// Removes the property with the given property_name from the given relation type.
    fn remove_property(&self, ty: &RelationTypeId, property_name: &str) -> Result<PropertyType, RelationTypeRemovePropertyError>;

    /// Adds an extension to the given relation type.
    fn add_extension(&self, ty: &RelationTypeId, extension: Extension) -> Result<ExtensionTypeId, RelationTypeAddExtensionError>;

    fn update_extension(
        &self,
        relation_ty: &RelationTypeId,
        extension_ty: &ExtensionTypeId,
        extension: Extension,
    ) -> Result<Extension, RelationTypeUpdateExtensionError>;

    /// Removes the extension with the given type from the given relation type.
    fn remove_extension(&self, relation_ty: &RelationTypeId, extension_ty: &ExtensionTypeId) -> Result<Extension, RelationTypeRemoveExtensionError>;

    /// Deletes the given relation type.
    fn delete(&self, ty: &RelationTypeId) -> Option<RelationType>;

    /// Validates the relation type with the given name.
    /// Tests that all components, the outbound and inbound entity type exists.
    fn validate(&self, ty: &RelationTypeId) -> bool;
}
