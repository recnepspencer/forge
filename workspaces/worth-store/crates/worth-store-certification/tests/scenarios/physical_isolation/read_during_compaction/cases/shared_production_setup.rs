use worth_store_physical_integrity::CompactionSourceIntegrityClearance;
use worth_store_physical_isolation::{
    CompactionSourceIntegrityEvidence, CurrentGenerationPhysicalReference, CurrentPhysicalRoot,
    PhysicalReadStabilityAuthority, StablePhysicalReadExecution, StablePhysicalReadReceipt,
};

use super::plan_admission::{admit_plan, protected_set};
use super::source_precedence_fixture;

pub(super) fn stable_source_evidence(
    authority: &PhysicalReadStabilityAuthority,
    root: CurrentPhysicalRoot,
    reference: CurrentGenerationPhysicalReference,
) -> CompactionSourceIntegrityEvidence {
    let evidence =
        source_precedence_fixture::intact_wal_integrity_evidence_for_owner(reference.owner());
    let clearance = CompactionSourceIntegrityClearance::from_integrity_evidence(&evidence).unwrap();
    CompactionSourceIntegrityEvidence::from_stable_read_receipt_and_integrity_clearance(
        execute_read(authority, root, reference),
        clearance,
    )
    .unwrap()
}

pub(super) fn execute_read(
    authority: &PhysicalReadStabilityAuthority,
    root: CurrentPhysicalRoot,
    reference: CurrentGenerationPhysicalReference,
) -> StablePhysicalReadReceipt {
    StablePhysicalReadExecution::from_execution_ready_handle(
        admit_plan(authority, root, protected_set([reference], 4), 8, 4)
            .into_execution_ready_handle(),
    )
    .complete()
}
