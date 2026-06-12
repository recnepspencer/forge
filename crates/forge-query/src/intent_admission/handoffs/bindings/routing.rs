use crate::runtime::ForgeQueryExistingTruthProbeRequest;

use super::handoff_execution_binding_identity;
use crate::intent_admission::{
    ForgeQueryExistingTruthProbeExecutionHandoff, ForgeQueryIntentAdmissionCoveredEntrypoint,
    ForgeQueryIntentAdmissionExecutionSeam, ForgeQueryIntentAdmissionFamily,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryExistingTruthProbeExecutionBinding {
    handoff: ForgeQueryExistingTruthProbeExecutionHandoff,
    binding_digest: String,
}

impl ForgeQueryExistingTruthProbeExecutionBinding {
    pub(crate) fn from_handoff(handoff: ForgeQueryExistingTruthProbeExecutionHandoff) -> Self {
        let binding_digest = handoff_execution_binding_identity(
            "existing-truth-probe-execution",
            handoff.handoff_digest(),
        );
        Self {
            handoff,
            binding_digest,
        }
    }

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        self.handoff.family()
    }

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
        self.handoff.entrypoint()
    }

    pub fn execution_seam(&self) -> ForgeQueryIntentAdmissionExecutionSeam {
        self.handoff.execution_seam()
    }

    pub fn request(&self) -> &ForgeQueryExistingTruthProbeRequest {
        self.handoff.request()
    }

    pub fn handoff(&self) -> &ForgeQueryExistingTruthProbeExecutionHandoff {
        &self.handoff
    }

    pub fn binding_digest(&self) -> &str {
        &self.binding_digest
    }
}
