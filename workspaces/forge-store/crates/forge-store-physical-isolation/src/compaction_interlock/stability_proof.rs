use super::{CompactionReadInterlockDenial, CompactionRewritePublication};
use crate::CurrentPhysicalRoot;
use forge_store_recovery_physics::{
    CompactionCutoverRecoveryPosture, CompactionGenerationVisibility,
};

#[derive(Debug, Clone)]
pub struct CompactionCutoverStabilityProof {
    publication: CompactionRewritePublication,
    recovery_posture: CompactionCutoverRecoveryPosture,
}

impl CompactionCutoverStabilityProof {
    pub const fn cutover_state(&self) -> super::CompactionCutoverState {
        super::CompactionCutoverState::RecoveryVisibilityAdmitted
    }

    pub const fn cutover_transition(&self) -> super::CompactionCutoverTransition {
        super::CompactionCutoverTransitionKind::AdmitRecoveryVisibility.transition()
    }

    pub fn admit(
        publication: CompactionRewritePublication,
        recovery_posture: CompactionCutoverRecoveryPosture,
    ) -> Result<Self, CompactionReadInterlockDenial> {
        match recovery_posture.visibility() {
            CompactionGenerationVisibility::VisibleAfterAdmittedCutover { .. } => Ok(Self {
                publication,
                recovery_posture,
            }),
            CompactionGenerationVisibility::ResidueRejected(rejection) => Err(
                CompactionReadInterlockDenial::BackendResidueCandidateSelection(rejection.reason()),
            ),
        }
    }

    pub const fn pre_cutover_root(&self) -> CurrentPhysicalRoot {
        self.publication.publication().old_root()
    }

    pub const fn post_cutover_root(&self) -> CurrentPhysicalRoot {
        self.publication.publication().new_root()
    }

    pub const fn publication(&self) -> &CompactionRewritePublication {
        &self.publication
    }

    pub const fn recovery_posture(&self) -> &CompactionCutoverRecoveryPosture {
        &self.recovery_posture
    }
}
