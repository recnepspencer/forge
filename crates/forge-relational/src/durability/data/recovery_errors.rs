use serde::{Deserialize, Serialize};

use crate::errors::data::{ErrorContext, ErrorOperation, RelationalSubsystem, SuggestedFix};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecoveryFailureClass {
    SchemaMismatch,
    ProfileMismatch,
    RuntimeNameMismatch,
    CorruptCheckpoint,
    CorruptSegment,
    MissingParentChain,
    ReplayFailure,
    DurableIoFailure,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurabilityError {
    pub class: RecoveryFailureClass,
    pub detail: String,
    pub context: ErrorContext,
}

impl DurabilityError {
    pub fn new(class: RecoveryFailureClass, detail: impl Into<String>) -> Self {
        let operation = match class {
            RecoveryFailureClass::DurableIoFailure => ErrorOperation::ReadDurableStore,
            RecoveryFailureClass::CorruptCheckpoint
            | RecoveryFailureClass::CorruptSegment
            | RecoveryFailureClass::MissingParentChain
            | RecoveryFailureClass::ReplayFailure
            | RecoveryFailureClass::SchemaMismatch
            | RecoveryFailureClass::ProfileMismatch
            | RecoveryFailureClass::RuntimeNameMismatch => ErrorOperation::Recover,
        };
        Self {
            class,
            detail: detail.into(),
            context: ErrorContext::new(RelationalSubsystem::Durability, operation)
                .with_fix(SuggestedFix::RepairDurableStore),
        }
    }
}
