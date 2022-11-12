use crate::model::DataType;

#[derive(Debug)]
pub enum BehaviourTransitionError {
    InvalidTransition,
    BehaviourInvalid(BehaviourInvalid),
    BehaviourInitializationFailed(BehaviourInitializationFailed),
    BehaviourConnectFailed(BehaviourConnectFailed),
    BehaviourDisconnectFailed(BehaviourDisconnectFailed),
}

#[derive(Debug)]
pub enum BehaviourCreationError {
    /// Creating the behaviour failed because connecting the behaviour failed.
    BehaviourTransitionError(BehaviourTransitionError),
}

#[derive(Debug)]
pub struct BehaviourConnectFailed {}

#[derive(Debug)]
pub struct BehaviourDisconnectFailed {}

#[derive(Debug)]
pub struct BehaviourInitializationFailed {}

#[derive(Debug)]
pub struct BehaviourShutdownFailed {}

#[derive(Debug)]
pub enum BehaviourInvalid {
    /// The behaviour is invalid because one or multiple properties are invalid.
    BehaviourPropertyInvalid(BehaviourPropertyInvalid),
}

#[derive(Debug)]
pub enum BehaviourPropertyInvalid {
    /// The property with the given name is missing.
    PropertyMissing(String),

    /// The property with the given name has a data type which is not the expected data type.
    InvalidDataType(String, DataType, DataType),
}
