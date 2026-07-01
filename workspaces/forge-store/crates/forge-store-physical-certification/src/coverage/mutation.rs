use crate::{
    FaultDeliveryAttempt, FaultDeliveryDenial, ShortcutRejectionObservationKind,
    SimulationReplayBundle,
};

use super::{CoverageGapDenial, MutationValidationPosture, Roadmap2HarnessSequence};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalMutationCoverageEvidence {
    sequence: Roadmap2HarnessSequence,
    plan_identity: [u8; 32],
    posture: MutationValidationPosture,
    denial: FaultDeliveryDenial,
}

impl PhysicalMutationCoverageEvidence {
    pub fn from_replay_private_mutation_denial(
        sequence: Roadmap2HarnessSequence,
        replay: &SimulationReplayBundle,
        attempt: FaultDeliveryAttempt,
    ) -> Result<Self, CoverageGapDenial> {
        if !replay
            .trace()
            .shortcut_rejections()
            .iter()
            .any(|observation| {
                observation.kind() == ShortcutRejectionObservationKind::PrivateMutationDenied
            })
        {
            return Err(CoverageGapDenial::MissingMutationResult);
        }
        match attempt.admit() {
            Err(FaultDeliveryDenial::PrivateMutationDenied) => Ok(Self {
                sequence,
                plan_identity: *replay.plan().identity().digest_bytes(),
                posture: MutationValidationPosture::ExpectedFailureObserved,
                denial: FaultDeliveryDenial::PrivateMutationDenied,
            }),
            _ => Err(CoverageGapDenial::MissingMutationResult),
        }
    }

    pub const fn sequence(&self) -> Roadmap2HarnessSequence {
        self.sequence
    }

    pub const fn plan_identity(&self) -> &[u8; 32] {
        &self.plan_identity
    }

    pub const fn posture(&self) -> MutationValidationPosture {
        self.posture
    }

    pub const fn denial(&self) -> &FaultDeliveryDenial {
        &self.denial
    }
}
