use super::super::WorthQueryRuntimeError;
use super::live::WorthQueryPatchBatch;
use super::mutation::WorthQueryWriteReceipt;
use crate::program::{WorthQueryOperationOutput, WorthQueryProgramOperationIdentity};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthQueryProgramInstallationIdentity {
    value: String,
}

impl WorthQueryProgramInstallationIdentity {
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
pub struct WorthQueryProgramRunIdentity {
    value: String,
}

impl WorthQueryProgramRunIdentity {
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
pub struct WorthQueryInstalledProgram {
    pub(in crate::runtime) program_identity: WorthQueryProgramInstallationIdentity,
}

impl WorthQueryInstalledProgram {
    pub fn program_id(&self) -> &str {
        self.program_identity.as_str()
    }

    pub fn operation(
        &self,
        operation_id: impl Into<String>,
    ) -> Result<WorthQueryInstalledOperation, WorthQueryRuntimeError> {
        Ok(WorthQueryInstalledOperation {
            program_identity: self.program_identity.clone(),
            operation_identity: WorthQueryProgramOperationIdentity::from_operation_id(operation_id),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryInstalledOperation {
    pub(in crate::runtime) program_identity: WorthQueryProgramInstallationIdentity,
    pub(in crate::runtime) operation_identity: WorthQueryProgramOperationIdentity,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryRunReceipt {
    pub(in crate::runtime) run_identity: WorthQueryProgramRunIdentity,
    pub(in crate::runtime) operation: WorthQueryInstalledOperation,
    pub(in crate::runtime) outputs: Vec<WorthQueryOperationOutput>,
    pub(in crate::runtime) write_receipts: Vec<WorthQueryWriteReceipt>,
    pub(in crate::runtime) patch_batches: Vec<WorthQueryPatchBatch>,
}

impl WorthQueryRunReceipt {
    pub fn run_id(&self) -> &str {
        self.run_identity.as_str()
    }

    pub fn outputs(&self) -> &[WorthQueryOperationOutput] {
        &self.outputs
    }

    pub fn write_receipts(&self) -> &[WorthQueryWriteReceipt] {
        &self.write_receipts
    }

    pub fn patch_batches(&self) -> &[WorthQueryPatchBatch] {
        &self.patch_batches
    }
}
