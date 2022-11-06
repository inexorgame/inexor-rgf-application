extern crate test;

use std::process::Termination;
use test::Bencher;

use crate::builder::ComponentBuilder;
use crate::model::ComponentTypeId;
use crate::tests::utils::application::init_application;
use crate::tests::utils::r_string;

#[bench]
fn creation_benchmark(bencher: &mut Bencher) -> impl Termination {
    let application = init_application();
    let component_manager = application.get_component_manager();
    let namespace = r_string();
    let component_name = r_string();
    let property_name = r_string();
    let ty = ComponentTypeId::new_from_type(namespace.clone(), component_name.clone());
    let component = ComponentBuilder::new(ty.clone()).string_property(&property_name).build();
    bencher.iter(move || {
        let _ = component_manager.register(component.clone());
        component_manager.delete(&ty);
    })
}
