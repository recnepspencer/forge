use super::super::ForgeQueryRuntimeError;
use super::live::ForgeQueryPatchBatch;
use super::mutation::ForgeQueryWriteReceipt;
use crate::program::ForgeQueryOperationOutput;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryInstalledProgram {
    pub(in crate::runtime) program_id: String,
}

impl ForgeQueryInstalledProgram {
    pub fn program_id(&self) -> &str {
        &self.program_id
    }

    pub fn operation(
        &self,
        operation_id: impl Into<String>,
    ) -> Result<ForgeQueryInstalledOperation, ForgeQueryRuntimeError> {
        Ok(ForgeQueryInstalledOperation {
            program_id: self.program_id.clone(),
            operation_id: operation_id.into(),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryInstalledOperation {
    pub(in crate::runtime) program_id: String,
    pub(in crate::runtime) operation_id: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryRunReceipt {
    pub(in crate::runtime) run_id: String,
    pub(in crate::runtime) operation: ForgeQueryInstalledOperation,
    pub(in crate::runtime) outputs: Vec<ForgeQueryOperationOutput>,
    pub(in crate::runtime) write_receipts: Vec<ForgeQueryWriteReceipt>,
    pub(in crate::runtime) patch_batches: Vec<ForgeQueryPatchBatch>,
}

impl ForgeQueryRunReceipt {
    pub fn run_id(&self) -> &str {
        &self.run_id
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
