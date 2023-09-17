use crate::ReactiveInstance;
use crate::TypedReactivePropertyCreator;
use crate::TypedReactivePropertyImpl;
use inexor_rgf_graph::DataType;
use inexor_rgf_graph::Mutability;

impl<IdType, ReactiveInstanceType> TypedReactivePropertyCreator<IdType, ReactiveInstanceType>
    for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, String>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    type Target = String;

    fn create(&self) {
        if !self.reactive_instance.has_property(&self.property_name) {
            self.reactive_instance
                .add_property(&self.property_name, Mutability::Mutable, DataType::String.default_value());
        }
    }
}
