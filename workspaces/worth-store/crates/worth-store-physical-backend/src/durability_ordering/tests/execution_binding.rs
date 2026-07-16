use super::super::execution::{
    StoreDurabilityExecutionObservation, StoreDurabilityExecutionSession,
};
use crate::{
    BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet, BackendMediaAssumptionSet,
    StoreDurabilityAdmission, StoreDurabilityDenialKind, StoreDurabilityFileSyncKind,
    StoreDurabilityPublicationKind, StoreDurabilityRequirement, WalDurabilityBarrier,
    WalDurabilityBarrierSet,
};

use super::super::test_support::{execution_proof, witness, RequestAssertingDurabilityBackend};

#[test]
fn execution_request_carries_admitted_scope_profile_and_requirement() {
    let witness = witness(
        BackendCapabilityEvidenceBasis::certified_backend_profile(),
        BackendCapabilitySupportSet::buffered_durable_only(),
        BackendMediaAssumptionSet::platform_file_defaults(),
    );
    let requirement = StoreDurabilityRequirement::wal_ordering_barrier(
        WalDurabilityBarrierSet::of(WalDurabilityBarrier::WalFileFsync),
    );
    let accepted = StoreDurabilityAdmission::admit(requirement, &witness)
        .unwrap()
        .submit_write("wal-scope-17")
        .backend_accepted();
    let mut backend = RequestAssertingDurabilityBackend {
        expected_scope: "wal-scope-17",
        expected_requirement: requirement,
        expected_publication: StoreDurabilityPublicationKind::WalFrame,
        observation: StoreDurabilityExecutionObservation::new(
            requirement.required_barriers(),
            StoreDurabilityFileSyncKind::Fdatasync,
        )
        .with_ordering_barrier_completed()
        .with_persisted_artifact(std::path::PathBuf::from("request-artifact"), 0, 1),
    };

    let proof = StoreDurabilityExecutionSession::for_owned_backend(&mut backend)
        .execute(&accepted)
        .unwrap();
    let ordered = accepted
        .reach_durability_boundary(proof)
        .unwrap()
        .ordering_barrier_durable()
        .unwrap();

    assert_eq!(ordered.counters().ordering_barriers_completed(), 1);
}

#[test]
fn execution_proof_cannot_complete_a_different_accepted_scope() {
    let witness = witness(
        BackendCapabilityEvidenceBasis::certified_backend_profile(),
        BackendCapabilitySupportSet::buffered_durable_only(),
        BackendMediaAssumptionSet::platform_file_defaults(),
    );
    let requirement = StoreDurabilityRequirement::wal_ordering_barrier(
        WalDurabilityBarrierSet::of(WalDurabilityBarrier::WalFileFsync),
    );
    let first = StoreDurabilityAdmission::admit(requirement, &witness)
        .unwrap()
        .submit_write("wal-a")
        .backend_accepted();
    let second = StoreDurabilityAdmission::admit(requirement, &witness)
        .unwrap()
        .submit_write("wal-b")
        .backend_accepted();
    let proof = execution_proof(
        &first,
        StoreDurabilityFileSyncKind::Fdatasync,
        false,
        false,
        true,
    );

    let denial = second.reach_durability_boundary(proof).unwrap_err();

    assert_eq!(
        denial.kind(),
        StoreDurabilityDenialKind::ExecutionBindingMismatch
    );
}
