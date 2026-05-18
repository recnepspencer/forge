use crate::identity::hash_parts;
use crate::intent_admission::ForgeQueryUnifiedInspectionExecutionHandoff;

use super::{
    ForgeQueryIntentAdmissionCoveredEntrypoint, ForgeQueryIntentAdmissionExecutionSeam,
    ForgeQueryIntentAdmissionFamily,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryUnifiedInspectionExecutionBinding {
    handoff: ForgeQueryUnifiedInspectionExecutionHandoff,
    binding_digest: String,
}

impl ForgeQueryUnifiedInspectionExecutionBinding {
    pub(crate) fn from_handoff(handoff: ForgeQueryUnifiedInspectionExecutionHandoff) -> Self {
        let binding_digest = hash_parts(&[
            "forge_query_unified_inspection_execution_binding_v1".to_string(),
            format!("handoff:{}", handoff.handoff_digest()),
        ]);
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

    pub fn seed(&self) -> &crate::intent_admission::ForgeQueryGenericInspectionIntentSeed {
        self.handoff.seed()
    }

    pub fn handoff(&self) -> &ForgeQueryUnifiedInspectionExecutionHandoff {
        &self.handoff
    }

    pub fn binding_digest(&self) -> &str {
        &self.binding_digest
    }
}
