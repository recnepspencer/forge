use worth_store_io_scheduler::QueueExecutionOutcome;

use crate::physical_runtime::PhysicalWorkSettlementEvidence;

use super::{
    CanonicalCandidateFrameWrite, CanonicalRecordMutationFailure, CanonicalRecordMutationKind,
    PreparedCanonicalRecordMutation,
};

pub(in crate::physical_runtime) enum CanonicalRecordMutationCompletion {
    CandidateFrame(CanonicalCandidateFrameWrite),
    PublicationEffect(crate::physical_runtime::PhysicalWorkIdentity),
}

impl CanonicalRecordMutationCompletion {
    pub(in crate::physical_runtime::record_serving) const fn identity(
        &self,
    ) -> crate::physical_runtime::PhysicalWorkIdentity {
        match self {
            Self::CandidateFrame(completed) => completed.identity,
            Self::PublicationEffect(identity) => *identity,
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
        classify(
            self.expected,
            self.identity,
            self.target,
            outcome.into_settled().into_evidence(),
        )
    }
}

fn classify(
    expected: CanonicalRecordMutationKind,
    identity: crate::physical_runtime::PhysicalWorkIdentity,
    target: crate::physical_runtime::PhysicalWorkRecoveryTarget,
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
                    CandidateFramePhysicalWrite::completed(physical.into_write()),
                identity,
            },
        )),
        (
            CanonicalRecordMutationKind::ExactWrite,
            PhysicalWorkSettlementEvidence::Publication {
                physical,
                scheduler: QueueExecutionOutcome::Executed(_),
            },
        ) => Ok(CanonicalRecordMutationCompletion::CandidateFrame(
            CanonicalCandidateFrameWrite {
                physical: super::super::residency::candidate_frame_residency::
                    CandidateFramePhysicalWrite::completed(physical),
                identity,
            },
        )),
        (
            CanonicalRecordMutationKind::PublicationEffect,
            PhysicalWorkSettlementEvidence::PublicationEffect {
                physical: _,
                scheduler: QueueExecutionOutcome::Executed(_),
            },
        ) => Ok(CanonicalRecordMutationCompletion::PublicationEffect(identity)),
        (_, PhysicalWorkSettlementEvidence::NoEffect(evidence)) => {
            Err(CanonicalRecordMutationFailure::backend(
                identity,
                target,
                evidence.failure(),
            ))
        }
        (_, PhysicalWorkSettlementEvidence::TerminalFailure(failure)) => {
            Err(CanonicalRecordMutationFailure::terminal(failure))
        }
        _ => Err(CanonicalRecordMutationFailure::settlement_mismatch(identity)),
    }
}
