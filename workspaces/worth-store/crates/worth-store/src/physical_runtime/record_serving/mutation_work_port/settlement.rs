use worth_store_io_scheduler::QueueExecutionOutcome;

use crate::physical_runtime::PhysicalWorkSettlementEvidence;

use super::{
    CanonicalRecordMutationFailure, CanonicalRecordMutationSettlement,
    PreparedCanonicalRecordMutation,
};

pub(in crate::physical_runtime) struct CanonicalRecordMutationCompletion {
    physical: super::super::residency::candidate_frame_residency::CandidateFramePhysicalWrite,
}

impl CanonicalRecordMutationCompletion {
    pub(in crate::physical_runtime::record_serving) fn into_physical(
        self,
    ) -> super::super::residency::candidate_frame_residency::CandidateFramePhysicalWrite {
        self.physical
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
        classify(self.target, settlement, settled.into_evidence())
    }
}

fn classify(
    target: crate::physical_runtime::PhysicalWorkRecoveryTarget,
    settlement: CanonicalRecordMutationSettlement,
    evidence: PhysicalWorkSettlementEvidence,
) -> Result<CanonicalRecordMutationCompletion, CanonicalRecordMutationFailure> {
    match evidence {
        PhysicalWorkSettlementEvidence::NewArtifact {
            physical,
            coordinate,
            scheduler: QueueExecutionOutcome::Executed(_),
        } => Ok(CanonicalRecordMutationCompletion {
            physical: super::super::residency::candidate_frame_residency::
                CandidateFramePhysicalWrite::completed(physical, coordinate, settlement),
        }),
        PhysicalWorkSettlementEvidence::NoEffect(evidence) => Err(
            CanonicalRecordMutationFailure::backend(settlement, target, evidence.failure()),
        ),
        PhysicalWorkSettlementEvidence::TerminalFailure(failure) => {
            Err(CanonicalRecordMutationFailure::terminal(settlement, failure))
        }
        _ => Err(CanonicalRecordMutationFailure::settlement_mismatch(
            settlement,
        )),
    }
}
