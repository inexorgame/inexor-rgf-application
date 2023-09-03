use thiserror::Error;
use crate::{AddEntityInstanceError, UpdateEntityInstanceError, UpdateRelationInstanceError};
use crate::AddExtensionError;
use crate::AddRelationInstanceError;
use crate::AddVariableError;
use crate::FlowTypeId;
use crate::RemoveEntityInstanceError;
use crate::RemoveExtensionError;
use crate::RemoveRelationInstanceError;
use crate::RemoveVariableError;
use crate::UpdateExtensionError;
use crate::UpdateVariableError;

#[derive(Debug, Error)]
#[error("The flow type {0} does not exist")]
pub struct FlowTypeDoesNotExistError(pub FlowTypeId);

#[derive(Debug, Error)]
pub enum FlowTypeAddEntityInstanceError {
    #[error("The flow type {0} does not exist")]
    FlowTypeDoesNotExist(FlowTypeId),
    #[error("Failed to add entity instance {0}")]
    AddEntityInstanceError(AddEntityInstanceError),
}

#[derive(Debug, Error)]
pub enum FlowTypeUpdateEntityInstanceError {
    #[error("The flow type {0} does not exist")]
    FlowTypeDoesNotExist(FlowTypeId),
    #[error("Failed to update entity instance {0}")]
    UpdateEntityInstanceError(UpdateEntityInstanceError),
}

#[derive(Debug, Error)]
pub enum FlowTypeRemoveEntityInstanceError {
    #[error("The flow type {0} does not exist")]
    FlowTypeDoesNotExist(FlowTypeId),
    #[error("Failed to remove entity instance {0}")]
    RemoveEntityInstanceError(RemoveEntityInstanceError),
}

#[derive(Debug, Error)]
pub enum FlowTypeAddRelationInstanceError {
    #[error("The flow type {0} does not exist")]
    FlowTypeDoesNotExist(FlowTypeId),
    #[error("Failed to add relation instance {0}")]
    AddRelationInstanceError(AddRelationInstanceError),
}

#[derive(Debug, Error)]
pub enum FlowTypeUpdateRelationInstanceError {
    #[error("The flow type {0} does not exist")]
    FlowTypeDoesNotExist(FlowTypeId),
    #[error("Failed to update relation instance {0}")]
    UpdateRelationInstanceError(UpdateRelationInstanceError),
}

#[derive(Debug, Error)]
pub enum FlowTypeRemoveRelationInstanceError {
    #[error("The flow type {0} does not exist")]
    FlowTypeDoesNotExist(FlowTypeId),
    #[error("Failed to remove relation instance {0}")]
    RemoveRelationInstanceError(RemoveRelationInstanceError),
}

#[derive(Debug, Error)]
pub enum FlowTypeAddVariableError {
    #[error("The flow type {0} does not exist")]
    FlowTypeDoesNotExist(FlowTypeId),
    #[error("Failed to add variable {0}")]
    AddVariableError(AddVariableError),
}

#[derive(Debug, Error)]
pub enum FlowTypeUpdateVariableError {
    #[error("The flow type {0} does not exist")]
    FlowTypeDoesNotExist(FlowTypeId),
    #[error("Failed to update variable {0}")]
    UpdateVariableError(UpdateVariableError),
}

#[derive(Debug, Error)]
pub enum FlowTypeRemoveVariableError {
    #[error("The flow type {0} does not exist")]
    FlowTypeDoesNotExist(FlowTypeId),
    #[error("Failed to remove variable {0}")]
    RemoveVariableError(RemoveVariableError),
}

#[derive(Debug, Error)]
pub enum FlowTypeMergeVariablesError {
    #[error("The flow type {0} does not exist")]
    FlowTypeDoesNotExist(FlowTypeId),
}

#[derive(Debug, Error)]
pub enum FlowTypeAddExtensionError {
    #[error("The flow type {0} does not exist")]
    FlowTypeDoesNotExist(FlowTypeId),
    #[error("Failed to add extension {0}")]
    AddExtensionError(AddExtensionError),
}

#[derive(Debug, Error)]
pub enum FlowTypeUpdateExtensionError {
    #[error("The flow type {0} does not exist")]
    FlowTypeDoesNotExist(FlowTypeId),
    #[error("Failed to update extension {0}")]
    UpdateExtensionError(UpdateExtensionError),
}

#[derive(Debug, Error)]
pub enum FlowTypeRemoveExtensionError {
    #[error("The flow type {0} does not exist")]
    FlowTypeDoesNotExist(FlowTypeId),
    #[error("Failed to remove extension {0}")]
    RemoveExtensionError(RemoveExtensionError),
}

#[derive(Debug, Error)]
pub enum FlowTypeMergeExtensionsError {
    #[error("The flow type {0} does not exist")]
    FlowTypeDoesNotExist(FlowTypeId),
}
