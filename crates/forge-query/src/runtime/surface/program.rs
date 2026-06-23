use super::super::ForgeQueryRuntimeError;
use super::live::ForgeQueryPatchBatch;
use super::mutation::ForgeQueryWriteReceipt;
use crate::program::{ForgeQueryOperationOutput, ForgeQueryProgramOperationIdentity};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ForgeQueryProgramInstallationIdentity {
    value: String,
}

impl ForgeQueryProgramInstallationIdentity {
    pub(in crate::runtime) fn from_program_id(program_id: impl Into<String>) -> Self {
        Self {
            value: program_id.into(),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ForgeQueryProgramRunIdentity {
    value: String,
}

impl ForgeQueryProgramRunIdentity {
    pub(in crate::runtime) fn from_run_id(run_id: impl Into<String>) -> Self {
        Self {
            value: run_id.into(),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryInstalledProgram {
    pub(in crate::runtime) program_identity: ForgeQueryProgramInstallationIdentity,
}

impl ForgeQueryInstalledProgram {
    pub fn program_id(&self) -> &str {
        self.program_identity.as_str()
    }

    pub fn operation(
        &self,
        operation_id: impl Into<String>,
    ) -> Result<ForgeQueryInstalledOperation, ForgeQueryRuntimeError> {
        Ok(ForgeQueryInstalledOperation {
            program_identity: self.program_identity.clone(),
            operation_identity: ForgeQueryProgramOperationIdentity::from_operation_id(operation_id),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryInstalledOperation {
    pub(in crate::runtime) program_identity: ForgeQueryProgramInstallationIdentity,
    pub(in crate::runtime) operation_identity: ForgeQueryProgramOperationIdentity,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryRunReceipt {
    pub(in crate::runtime) run_identity: ForgeQueryProgramRunIdentity,
    pub(in crate::runtime) operation: ForgeQueryInstalledOperation,
    pub(in crate::runtime) outputs: Vec<ForgeQueryOperationOutput>,
    pub(in crate::runtime) write_receipts: Vec<ForgeQueryWriteReceipt>,
    pub(in crate::runtime) patch_batches: Vec<ForgeQueryPatchBatch>,
}

impl ForgeQueryRunReceipt {
    pub fn run_id(&self) -> &str {
        self.run_identity.as_str()
    }

    pub fn outputs(&self) -> &[ForgeQueryOperationOutput] {
        &self.outputs
    }

    pub fn write_receipts(&self) -> &[ForgeQueryWriteReceipt] {
        &self.write_receipts
    }

    pub fn patch_batches(&self) -> &[ForgeQueryPatchBatch] {
        &self.patch_batches
    }
}
