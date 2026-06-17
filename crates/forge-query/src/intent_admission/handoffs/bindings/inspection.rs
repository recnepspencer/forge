use super::handoff_execution_binding_identity;
use crate::intent_admission::{
    ForgeQueryDerivedInspectionExecutionHandoff, ForgeQueryDerivedMaterializationExecutionHandoff,
    ForgeQueryIntentAdmissionCoveredEntrypoint, ForgeQueryIntentAdmissionExecutionSeam,
    ForgeQueryIntentAdmissionFamily,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryDerivedMaterializationExecutionBinding {
    handoff: ForgeQueryDerivedMaterializationExecutionHandoff,
    binding_digest: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryDerivedInspectionExecutionBinding {
    handoff: ForgeQueryDerivedInspectionExecutionHandoff,
    binding_digest: String,
}

impl ForgeQueryDerivedMaterializationExecutionBinding {
    pub(crate) fn from_handoff(handoff: ForgeQueryDerivedMaterializationExecutionHandoff) -> Self {
        let binding_digest = handoff_execution_binding_identity(
            "derived-materialization-execution",
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

    pub fn view_name(&self) -> &str {
        self.handoff.view_name()
    }

    pub fn handoff(&self) -> &ForgeQueryDerivedMaterializationExecutionHandoff {
        &self.handoff
    }

    pub fn binding_digest(&self) -> &str {
        &self.binding_digest
    }
}

impl ForgeQueryDerivedInspectionExecutionBinding {
    pub(crate) fn from_handoff(handoff: ForgeQueryDerivedInspectionExecutionHandoff) -> Self {
        let binding_digest = handoff_execution_binding_identity(
            "derived-inspection-execution",
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

    pub fn view_name(&self) -> &str {
        self.handoff.view_name()
    }

    pub fn handoff(&self) -> &ForgeQueryDerivedInspectionExecutionHandoff {
        &self.handoff
    }

    pub fn binding_digest(&self) -> &str {
        &self.binding_digest
    }
}
