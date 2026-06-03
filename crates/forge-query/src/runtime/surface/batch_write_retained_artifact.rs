use super::{
    ForgeQueryBatchWriteReceipt, ForgeQueryDerivedArtifactBinding,
    ForgeQueryDerivedMaterializationTarget,
};
use crate::runtime::{
    ForgeQueryBatchWriteReceiptInspection, ForgeQueryInspection, ForgeQueryRuntimeError,
    ForgeQueryWorkspace,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryBatchWriteRetainedArtifact {
    receipt: ForgeQueryBatchWriteReceipt,
    inspection: ForgeQueryBatchWriteReceiptInspection,
    retained_artifact: ForgeQueryDerivedArtifactBinding,
}

impl ForgeQueryBatchWriteRetainedArtifact {
    pub(in crate::runtime) fn build(
        workspace: &mut ForgeQueryWorkspace,
        receipt: &ForgeQueryBatchWriteReceipt,
        artifact_name: impl Into<String>,
        targets: impl IntoIterator<Item = ForgeQueryDerivedMaterializationTarget>,
    ) -> Result<Self, ForgeQueryRuntimeError> {
        let inspection = match workspace.inspect(receipt)? {
            ForgeQueryInspection::BatchWriteReceipt(inspection) => inspection,
            other => {
                return Err(ForgeQueryRuntimeError::Workspace(
                    crate::memory_workspace::ForgeQueryWorkspaceError::new(format!(
                        "expected batch-write receipt inspection, got `{other:?}`",
                    )),
                ));
            }
        };
        let retained_artifact =
            workspace.materialize_derived_artifact_binding(artifact_name, targets)?;
        Ok(Self {
            receipt: receipt.clone(),
            inspection,
            retained_artifact,
        })
    }

    pub fn receipt(&self) -> &ForgeQueryBatchWriteReceipt {
        &self.receipt
    }

    pub fn inspection(&self) -> &ForgeQueryBatchWriteReceiptInspection {
        &self.inspection
    }

    pub fn retained_artifact(&self) -> &ForgeQueryDerivedArtifactBinding {
        &self.retained_artifact
    }
}
