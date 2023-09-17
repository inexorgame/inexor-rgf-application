use crate::ReactiveInstance;
use crate::TypedReactivePropertyImpl;
use serde_json::json;

impl<IdType, ReactiveInstanceType> FnOnce<(bool,)> for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, bool>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    type Output = ();

    extern "rust-call" fn call_once(mut self, args: (bool,)) -> Self::Output {
        self.call_mut(args)
    }
}

impl<IdType, ReactiveInstanceType> FnMut<(bool,)> for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, bool>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    extern "rust-call" fn call_mut(&mut self, args: (bool,)) -> Self::Output {
        self.call(args)
    }
}

impl<IdType, ReactiveInstanceType> Fn<(bool,)> for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, bool>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    extern "rust-call" fn call(&self, args: (bool,)) -> Self::Output {
        self.reactive_instance.set(&self.property_name, json!(args.0));
    }
}

impl<IdType, ReactiveInstanceType> TypedReactivePropertyImpl<IdType, ReactiveInstanceType, bool>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    pub fn op<F>(&self, f: F)
    where
        F: Fn(bool) -> bool,
    {
        if let Some(v) = self.reactive_instance.as_bool(&self.property_name) {
            self.reactive_instance.set(&self.property_name, json!(f(v)));
        }
    }
}
