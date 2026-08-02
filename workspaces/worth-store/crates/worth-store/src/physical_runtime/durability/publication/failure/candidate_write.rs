use crate::physical_runtime::record_serving::{
    RootCandidateWriteFailureKind, RootCandidateWriteFailurePosture,
};
use crate::physical_runtime::CandidateFrameContractViolation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalRootCandidateWriteFailureCause {
    Contract {
        violation: CandidateFrameContractViolation,
        posture: PhysicalRootCandidateWriteFailurePosture,
    },
    Effect {
        fate: crate::physical_runtime::PhysicalWorkEffectFate,
    },
    Residency {
        denial: crate::physical_runtime::RecordAppendDenial,
        posture: PhysicalRootCandidateWriteFailurePosture,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalRootCandidateWriteFailurePosture {
    ProvenNoEffect,
    UnsettledBeforeEffect,
    EffectPossible,
}

pub(super) fn project_candidate_write_failure(
    cause: RootCandidateWriteFailureKind,
) -> PhysicalRootCandidateWriteFailureCause {
    match cause {
        RootCandidateWriteFailureKind::Contract { violation, posture } => {
            PhysicalRootCandidateWriteFailureCause::Contract {
                violation,
                posture: project_candidate_write_posture(posture),
            }
        }
        RootCandidateWriteFailureKind::Effect { fate } => {
            PhysicalRootCandidateWriteFailureCause::Effect { fate }
        }
        RootCandidateWriteFailureKind::Residency { denial, posture } => {
            PhysicalRootCandidateWriteFailureCause::Residency {
                denial,
                posture: project_candidate_write_posture(posture),
            }
        }
    }
}

const fn project_candidate_write_posture(
    posture: RootCandidateWriteFailurePosture,
) -> PhysicalRootCandidateWriteFailurePosture {
    match posture {
        RootCandidateWriteFailurePosture::ProvenNoEffect => {
            PhysicalRootCandidateWriteFailurePosture::ProvenNoEffect
        }
        RootCandidateWriteFailurePosture::UnsettledBeforeEffect => {
            PhysicalRootCandidateWriteFailurePosture::UnsettledBeforeEffect
        }
        RootCandidateWriteFailurePosture::EffectPossible => {
            PhysicalRootCandidateWriteFailurePosture::EffectPossible
        }
    }
}
