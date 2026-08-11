use std::collections::BTreeMap;

use crate::memory_workspace::WorthQueryCommitIdentity;

use super::operations::WorthQueryAuthorityRequirement;
use super::values::WorthQueryProgramValue;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryProgramTrace {
    program_id: String,
    operation_id: String,
    bound_inputs: Vec<String>,
    authority_requirements: Vec<WorthQueryAuthorityRequirement>,
    generated_declarations: Vec<String>,
    write_receipts: Vec<WorthQueryCommitIdentity>,
    patch_artifacts: Vec<String>,
    replay_or_parity_metadata: Vec<String>,
}

impl WorthQueryProgramTrace {
    pub(crate) fn new(
        program_id: impl Into<String>,
        operation_id: impl Into<String>,
        bound_inputs: &BTreeMap<String, WorthQueryProgramValue>,
        authority_requirements: Vec<WorthQueryAuthorityRequirement>,
    ) -> Self {
        Self {
            program_id: program_id.into(),
            operation_id: operation_id.into(),
            bound_inputs: bound_inputs.keys().cloned().collect(),
            authority_requirements,
            generated_declarations: Vec::new(),
            write_receipts: Vec::new(),
            patch_artifacts: Vec::new(),
            replay_or_parity_metadata: Vec::new(),
        }
    }

    pub(crate) fn record_declaration(&mut self, declaration: impl Into<String>) {
        self.generated_declarations.push(declaration.into());
    }

    pub(crate) fn record_write_receipt(&mut self, receipt: WorthQueryCommitIdentity) {
        self.write_receipts.push(receipt);
    }

    pub(crate) fn record_patch_artifact(&mut self, artifact: impl Into<String>) {
        self.patch_artifacts.push(artifact.into());
    }

    pub(crate) fn record_replay_or_parity(&mut self, metadata: impl Into<String>) {
        self.replay_or_parity_metadata.push(metadata.into());
    }

    pub fn program_id(&self) -> &str {
        &self.program_id
    }

    pub fn operation_id(&self) -> &str {
        &self.operation_id
    }

    pub fn generated_declarations(&self) -> &[String] {
        &self.generated_declarations
    }

    pub fn write_receipts(&self) -> &[WorthQueryCommitIdentity] {
        &self.write_receipts
    }

    pub fn patch_artifacts(&self) -> &[String] {
        &self.patch_artifacts
    }
}
