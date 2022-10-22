use indradb::Identifier;
use serde_json::json;

use crate::tests::utils::r_string;
use crate::tests::utils::r_string_1000;
use crate::ComponentType;
use crate::DataType;
use crate::EntityTypeType;
use crate::Extension;
use crate::ExtensionContainer;
use crate::NamespacedTypeGetter;
use crate::PropertyType;
use crate::RelationType;
use crate::RelationTypeType;
use crate::TypeContainer;
use crate::TypeDefinitionGetter;

#[test]
fn create_relation_type_test() {
    let type_name = r_string();
    let outbound_type_name = r_string();
    let inbound_type_name = r_string();

    let namespace = r_string();
    let description = r_string();

    let component_name = r_string();
    let behaviour_name = r_string();
    let property_name = r_string();
    let extension_name = r_string();
    let extension_value = json!("JSON");
    let mut component_names = Vec::new();
    let component_ty = ComponentType::new_from_type(&namespace, &component_name);
    component_names.push(component_ty.clone());
    let mut behaviour_names = Vec::new();
    behaviour_names.push(behaviour_name.clone());
    let mut property_types = Vec::new();
    let property_type = PropertyType::new(property_name.clone(), DataType::String);
    property_types.push(property_type.clone());
    let mut extensions = Vec::new();
    let extension = Extension {
        name: extension_name.clone(),
        extension: extension_value.clone(),
    };
    extensions.push(extension.clone());
    let ty = RelationTypeType::new_from_type(&namespace, &type_name);
    let outbound_type = EntityTypeType::new_from_type(&namespace, &outbound_type_name);
    let inbound_type = EntityTypeType::new_from_type(&namespace, &inbound_type_name);
    let relation_type = RelationType::new(
        outbound_type.clone(),
        ty,
        inbound_type.clone(),
        description.clone(),
        component_names,
        property_types,
        extensions,
    );

    assert_eq!(namespace, relation_type.namespace());
    assert_eq!(type_name, relation_type.type_name());
    assert_eq!(format!("r__{}__{}", &namespace, &type_name), relation_type.type_definition().to_string());
    assert_eq!(
        Identifier::new(relation_type.type_definition().to_string().as_str()).unwrap(),
        (&relation_type.type_definition()).into()
    );
    assert_eq!(outbound_type, relation_type.outbound_type);
    assert_eq!(inbound_type, relation_type.inbound_type);
    assert_eq!(description, relation_type.description);
    assert_eq!(component_ty, *relation_type.components.first().unwrap());
    assert!(relation_type.is_a(&component_ty));
    assert_eq!(property_name, *relation_type.properties.first().unwrap().name);
    assert!(relation_type.has_own_property(property_name.clone()));
    assert!(!relation_type.has_own_property(r_string()));
    assert_eq!(property_type.data_type, relation_type.get_own_property(property_name).unwrap().data_type);
    assert_eq!(extension_name.clone(), relation_type.extensions.get(0).unwrap().name);
    assert_eq!(extension_value, relation_type.extensions.get(0).unwrap().extension);
    assert!(relation_type.has_own_extension(&extension_name));
    assert!(!relation_type.has_own_extension(r_string()));
    assert_eq!(extension.extension, relation_type.get_own_extension(&extension_name).unwrap().extension);
}

#[test]
fn long_relation_type_test() {
    let namespace = r_string_1000();
    let outbound_type_name = r_string_1000();
    let type_name = r_string_1000();
    let inbound_type_name = r_string_1000();
    let description = r_string();
    let ty = RelationTypeType::new_from_type(&namespace, &type_name);
    let outbound_type = EntityTypeType::new_from_type(&namespace, &outbound_type_name);
    let inbound_type = EntityTypeType::new_from_type(&namespace, &inbound_type_name);
    let rt = RelationType::new(outbound_type, ty, inbound_type, &description, Vec::new(), Vec::new(), Vec::new());
    let identifier: Identifier = rt.type_id();
    assert!(identifier.as_str().len() < 255);
}
