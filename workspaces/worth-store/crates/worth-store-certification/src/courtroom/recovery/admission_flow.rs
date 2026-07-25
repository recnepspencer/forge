use super::support::{
    admit_entry, damaged_integrity_model_input, intact_integrity_model_input, physical_authority,
    recovery_blocking_quarantine_binding, recovery_blocking_wal_damage_map,
    recovery_memory_allocation,
};
use crate::courtroom::harness::test_support::recovery_memory_allocation_test_support::operation_allocation;
use worth_store_buffer_pool::{OperationAllocationScope, PhysicalResidencyDenial};
use worth_store_recovery_physics::{
    IntegrityHandoffPayload, RecoveryEntryAdmission, RecoveryEntryAdmissionDecision,
    RecoveryMemoryAllocation, RecoveryMemoryAllocationDenial,
};

#[test]
fn intact_integrity_model_input_and_recovery_envelope_produce_stable_entry_identity() {
    let first = admit_entry(intact_integrity_model_input("entry-stability"));
    let second = admit_entry(intact_integrity_model_input("entry-stability"));

    assert_eq!(first.entry_identity(), second.entry_identity());
    assert_eq!(first.recovery_basis(), second.recovery_basis());
    assert_eq!(
        first.recovery_basis().integrity_handoff_identity(),
        second.recovery_basis().integrity_handoff_identity()
    );
    assert_eq!(first.counters().vetted_record_count(), 5);
    assert_eq!(first.counters().memory_envelope_admissions(), 1);
    assert_eq!(first.counters().replay_plans_started(), 0);
    assert_eq!(first.counters().source_precedence_choices(), 0);
    assert!(!first.claims_replay_plan());
    assert!(!first.source_precedence_chosen());
}

#[test]
fn recovery_entry_rejects_wrong_scope_or_over_budget_allocation_before_admission() {
    let maintenance = operation_allocation(OperationAllocationScope::Maintenance, 128)
        .expect("bounded maintenance allocation should admit");
    let denial = RecoveryMemoryAllocation::from_allocation_grant(maintenance).unwrap_err();

    assert_eq!(
        denial,
        RecoveryMemoryAllocationDenial::WrongAllocationScope {
            actual: OperationAllocationScope::Maintenance,
        }
    );

    let over_budget = operation_allocation(OperationAllocationScope::Recovery, 513).unwrap_err();
    assert_eq!(
        over_budget,
        PhysicalResidencyDenial::OperationBudgetExceeded
    );
}

#[test]
fn recovery_blocking_damage_blocks_before_replay_or_source_precedence() {
    let model_input = damaged_integrity_model_input();
    let decision = RecoveryEntryAdmission::admit(
        model_input,
        recovery_memory_allocation(),
        physical_authority(),
    );

    let RecoveryEntryAdmissionDecision::Blocked(blocked) = decision else {
        panic!("recovery-blocking S.3 damage must block entry");
    };

    assert_eq!(blocked.blocker_count(), 1);
    assert!(!blocked.replay_planning_started());
    assert!(!blocked.source_precedence_chosen());
}

#[test]
fn quarantine_summary_preserves_its_damage_case_across_mixed_recovery_blockers() {
    let intact = intact_integrity_model_input("damage-case-binding");
    let (quarantine_record, quarantine_receipt, quarantine_damage) =
        recovery_blocking_quarantine_binding();
    let damage_map = recovery_blocking_wal_damage_map()
        .with_unresolved_authority_damage(
            worth_store_recovery_physics::RecoveryBlockedByIntegrityDamage::unresolved_authority_damage(
                &super::support::unresolved_authority_record(),
            )
            .unwrap(),
        )
        .unwrap()
        .with_recovery_blocking_quarantine(
            &quarantine_record,
            quarantine_receipt,
            &quarantine_damage,
        )
        .unwrap();

    let payload = IntegrityHandoffPayload::declare()
        .root_manifest(intact.payload().root_manifest().clone())
        .segment_manifest(intact.payload().segment_manifest().clone())
        .page_frame(intact.payload().page_frames()[0].clone())
        .wal_frame(intact.payload().wal_frames()[0].clone())
        .checkpoint_record(intact.payload().checkpoint_records()[0].clone())
        .damage_map(damage_map)
        .inspection_envelope(intact.payload().inspection_envelope().clone())
        .seal()
        .unwrap();
    let model_input = super::support::admit_recovery_handoff_payload(payload);

    let RecoveryEntryAdmissionDecision::Blocked(blocked) = RecoveryEntryAdmission::admit(
        model_input,
        recovery_memory_allocation(),
        physical_authority(),
    ) else {
        panic!("mixed blockers should still block before replay planning");
    };

    let handoff = blocked
        .corruption_readmission_handoffs()
        .first()
        .expect("quarantine summary should emit a readmission handoff");
    assert_eq!(
        handoff.primary_damage_case(),
        worth_store_contracts::CorruptionHandoffDamageCase::ChecksumMismatch
    );
    assert_eq!(
        handoff.repair_capability(),
        worth_store_recovery_physics::RecoveryCorruptionRepairCapability::ClassifyGenerationPosture
    );
}
