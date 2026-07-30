use worth_store_io_scheduler::QueueExecutionOutcome;

use crate::physical_runtime::PhysicalWorkSettlementEvidence;

use super::{
    CanonicalCandidateFrameWrite, CanonicalRecordMutationFailure, CanonicalRecordMutationKind,
    CanonicalRecordMutationSettlement, PreparedCanonicalRecordMutation,
};

#[allow(
    clippy::large_enum_variant,
    reason = "settled physical-effect proof stays inline so canonical mutation completion adds no post-effect heap allocation"
)]
pub(in crate::physical_runtime) enum CanonicalRecordMutationCompletion {
    CandidateFrame(CanonicalCandidateFrameWrite),
    PublicationEffect(CanonicalRecordMutationSettlement),
}

impl CanonicalRecordMutationCompletion {
    pub(in crate::physical_runtime::record_serving) const fn settlement(
        &self,
    ) -> CanonicalRecordMutationSettlement {
        match self {
            Self::CandidateFrame(completed) => completed.physical.settlement(),
            Self::PublicationEffect(settlement) => *settlement,
        }
    }
}

impl PreparedCanonicalRecordMutation {
    pub(in crate::physical_runtime) fn execute(
        self,
    ) -> Result<CanonicalRecordMutationCompletion, CanonicalRecordMutationFailure> {
        let identity = self.identity;
        let outcome = self
            .execution
            .execute_physical_work(self.command)
            .map_err(|failure| CanonicalRecordMutationFailure::pre_effect(identity, failure))?;
        let settled = outcome.into_settled();
        let settlement = CanonicalRecordMutationSettlement::from_settled(&settled);
        classify(
            self.expected,
            self.target,
            settlement,
            settled.into_evidence(),
        )
    }
}

fn classify(
    expected: CanonicalRecordMutationKind,
    target: crate::physical_runtime::PhysicalWorkRecoveryTarget,
    settlement: CanonicalRecordMutationSettlement,
    evidence: PhysicalWorkSettlementEvidence,
) -> Result<CanonicalRecordMutationCompletion, CanonicalRecordMutationFailure> {
    match (expected, evidence) {
        (
            CanonicalRecordMutationKind::NewArtifact,
            PhysicalWorkSettlementEvidence::NewArtifact {
                physical,
                scheduler: QueueExecutionOutcome::Executed(_),
            },
        ) => Ok(CanonicalRecordMutationCompletion::CandidateFrame(
            CanonicalCandidateFrameWrite {
                physical: super::super::residency::candidate_frame_residency::
                    CandidateFramePhysicalWrite::completed(physical.into_write(), settlement),
            },
        )),
        (
            CanonicalRecordMutationKind::PublicationEffect,
            PhysicalWorkSettlementEvidence::PublicationEffect {
                physical: _,
                scheduler: QueueExecutionOutcome::Executed(_),
            },
        ) => Ok(CanonicalRecordMutationCompletion::PublicationEffect(
            settlement,
        )),
        (_, PhysicalWorkSettlementEvidence::NoEffect(evidence)) => {
            Err(CanonicalRecordMutationFailure::backend(
                settlement,
                target,
                evidence.failure(),
            ))
        }
        (_, PhysicalWorkSettlementEvidence::TerminalFailure(failure)) => {
            Err(CanonicalRecordMutationFailure::terminal(
                settlement, failure,
            ))
        }
        _ => Err(CanonicalRecordMutationFailure::settlement_mismatch(
            settlement,
        )),
    }
}
