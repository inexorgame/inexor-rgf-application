#[macro_export]
macro_rules! entity_behaviour {
    (
        /// The ident of the behaviour.
        $behaviour: ident,
        /// The ident of the factory to create instances of the behaviour.
        $factory: ident,
        /// The ident of the finite state machine of the behaviour.
        $fsm: ident,
        /// The ident of the transitions of the finite state machine.
        $transitions: ident,
        /// The ident of the property validator of the behaviour.
        $validator: ty
        $(,
            // Function name.
            $fn_name: ident,
            // Function.
            $fn_ident: ident
        )*
        $(,)?
    ) => {
        pub struct $behaviour {
            pub reactive_instance: inexor_rgf_reactive::ReactiveEntity,
            pub fsm: $fsm,
        }

        impl $behaviour {
            pub fn new(reactive_instance: inexor_rgf_reactive::ReactiveEntity, ty: inexor_rgf_behaviour_api::BehaviourTypeId, $($fn_name: $fn_ident)*) -> Result<std::sync::Arc<$behaviour>, $crate::BehaviourCreationError> {
                let transitions = <$transitions>::new(reactive_instance.clone(), ty.clone() $(, $fn_name)*);
                let validator = <$validator>::new(reactive_instance.clone());
                let fsm = <$fsm>::new(reactive_instance.clone(), ty, validator, transitions);
                let mut behaviour = $behaviour { reactive_instance, fsm };
                behaviour
                    .fsm
                    .transition($crate::BehaviourState::Connected)
                    .map_err($crate::BehaviourCreationError::BehaviourTransitionError)?;
                Ok(std::sync::Arc::new(behaviour))
            }
        }

        impl inexor_rgf_behaviour_api::BehaviourFsm<uuid::Uuid, inexor_rgf_reactive::ReactiveEntity> for $behaviour {
            fn ty(&self) -> &inexor_rgf_behaviour_api::BehaviourTypeId {
                &self.fsm.ty
            }

            fn get_state(&self) -> inexor_rgf_behaviour_api::BehaviourState {
                self.fsm.get_state()
            }

            fn set_state(&self, state: inexor_rgf_behaviour_api::BehaviourState) {
                self.fsm.set_state(state);
            }

            fn get_validator(&self) -> &dyn inexor_rgf_behaviour_api::BehaviourValidator<uuid::Uuid, inexor_rgf_reactive::ReactiveEntity> {
                &self.fsm.validator
            }

            fn get_transitions(&self) -> &dyn inexor_rgf_behaviour_api::BehaviourTransitions<uuid::Uuid, inexor_rgf_reactive::ReactiveEntity> {
                &self.fsm.transitions
            }
        }

        impl inexor_rgf_reactive_api::ReactiveInstanceContainer<uuid::Uuid, inexor_rgf_reactive::ReactiveEntity> for $behaviour {
            fn get_reactive_instance(&self) -> &inexor_rgf_reactive::ReactiveEntity {
                &self.reactive_instance
            }

            fn get(&self, property_name: &str) -> Option<serde_json::Value> {
                // inexor_rgf_graph::PropertyInstanceGetter::get(self, property_name)
                self.reactive_instance.get(property_name)
            }

            fn set(&self, property_name: &str, value: serde_json::Value) {
                // inexor_rgf_graph::PropertyInstanceSetter::get(self, property_name, value)
                self.reactive_instance.set(property_name, value);
            }
        }

        impl Drop for $behaviour {
            fn drop(&mut self) {
                log::trace!("Drop entity behaviour {}", &self.fsm.ty);
            }
        }

        inexor_rgf_behaviour_api::behaviour_factory!($factory, $behaviour, uuid::Uuid, inexor_rgf_reactive::ReactiveEntity $(, $fn_name, $fn_ident)*);

        inexor_rgf_behaviour_api::behaviour_fsm!($fsm, $validator, $transitions, uuid::Uuid, inexor_rgf_reactive::ReactiveEntity);

        $crate::entity_behaviour_transitions!($transitions $(, $fn_name, $fn_ident)*);
    };
}
