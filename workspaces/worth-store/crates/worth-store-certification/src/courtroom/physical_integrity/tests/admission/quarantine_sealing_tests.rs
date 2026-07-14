use worth_foundational::boundary_evidence_api::lower_lane::receipts::FoundationalBoundaryEvidenceReceiptKind;
use worth_store_physical_integrity::{
    AuthorityDamageBoundary, DamageClassification, ExecutedQuarantineFinding,
    PhysicalBoundaryLocalization, PhysicalQuarantineAuthority, QuarantineHandoffPosture,
    QuarantineLifecyclePosture, QuarantineLocalityBoundary, QuarantineSealDenialKind,
    QuarantineSealRequest, RebuildabilityPrerequisite,
};

use crate::courtroom::blobs::chunk_integrity_without_blob_lifecycle_tests::inspect_unknown_chunk_denial;
use crate::courtroom::harness::test_support::physical_container_integrity_test_support::{
    inspect_page_denial, inspect_page_report, page_payload_with_record,
};
use crate::courtroom::layout::derived_index_damage_tests::{
    inspect_damaged_derived_index_with_authority, inspect_intact_derived_index_with_authority,
    inspect_with_damaged_authority, inspect_without_authority_basis,
};

#[test]
fn equivalent_executed_findings_produce_same_sealed_quarantine_record() {
    let first = seal_ambiguous_page_damage("equivalent-damage");
    let second = seal_ambiguous_page_damage("equivalent-damage");

    assert_eq!(first, second);
    assert_eq!(
        first.lifecycle_posture(),
        QuarantineLifecyclePosture::Sealed
    );
    assert_eq!(
        first.receipt().foundational_basis().receipt_kind(),
        FoundationalBoundaryEvidenceReceiptKind::Execution
    );
    assert!(!first.proves_recovery());
    assert!(!first.proves_repair());
}

#[test]
fn ambiguous_physical_evidence_quarantines_broader_honest_boundary() {
    let record = seal_ambiguous_page_damage("ambiguous-locality");

    match record.locality().boundary() {
        QuarantineLocalityBoundary::BroaderPhysicalBoundary(_, boundary) => {
            assert!(matches!(
                boundary,
                PhysicalBoundaryLocalization::AmbiguousBoundary
                    | PhysicalBoundaryLocalization::SlotDirectory
            ));
        }
        QuarantineLocalityBoundary::ExactPhysicalScope(_) => {
            panic!("ambiguous damage must not invent exact locality")
        }
    }
    assert!(matches!(
        record.damage_classification(),
        DamageClassification::QuarantinedPhysicalDamage(damage)
            if damage.ambiguous_boundary().is_some()
    ));
}

#[test]
fn lifecycle_postures_are_representable_without_physical_integrity_performing_later_transitions() {
    let later_postures = [
        QuarantineLifecyclePosture::SupersededByRecovery,
        QuarantineLifecyclePosture::ReleasedAfterRepair,
        QuarantineLifecyclePosture::RetainedForAudit,
        QuarantineLifecyclePosture::InvalidatedByRootChange,
    ];

    for posture in later_postures {
        assert!(!posture.is_physical_integrity_mintable());
    }

    let finding = executed_ambiguous_page_finding("later-owner-required");
    let denial = PhysicalQuarantineAuthority::seal(
        QuarantineSealRequest::from_executed_finding(finding)
            .with_initial_posture(QuarantineLifecyclePosture::ReleasedAfterRepair),
    )
    .unwrap_err();
    assert_eq!(
        denial.kind(),
        QuarantineSealDenialKind::LaterLifecycleOwnerRequired
    );
}

#[test]
fn intact_physical_boundary_classification_is_distinct_from_quarantined_damage() {
    let payload = page_payload_with_record(b"intact-boundary");
    let report = inspect_page_report(&payload);
    let finding = ExecutedQuarantineFinding::intact_page(&report);
    let record = PhysicalQuarantineAuthority::seal(
        QuarantineSealRequest::from_executed_finding(finding)
            .with_handoff_posture(QuarantineHandoffPosture::AuditRetentionOwnerRequired),
    )
    .unwrap();

    assert!(matches!(
        record.damage_classification(),
        DamageClassification::IntactPhysicalBoundary(_)
    ));
    assert!(!matches!(
        record.damage_classification(),
        DamageClassification::QuarantinedPhysicalDamage(_)
    ));
}

#[test]
fn derived_damage_classes_survive_quarantine_sealing_without_substitution() {
    let rebuildable = seal_finding(ExecutedQuarantineFinding::from_index_page_report(
        &inspect_damaged_derived_index_with_authority(7),
    ));
    match rebuildable.damage_classification() {
        DamageClassification::RebuildableDerivedDamage(damage) => {
            assert_eq!(
                rebuildable.locality().boundary(),
                QuarantineLocalityBoundary::ExactPhysicalScope(damage.damaged_scope())
            );
        }
        other => panic!("expected rebuildable derived damage, got {other:?}"),
    }

    let unrecoverable = seal_finding(
        ExecutedQuarantineFinding::from_index_page_denial(&inspect_with_damaged_authority())
            .unwrap(),
    );
    match unrecoverable.damage_classification() {
        DamageClassification::UnrecoverableAuthorityDamage(damage) => {
            assert_eq!(damage.boundary(), AuthorityDamageBoundary::AllocationMap);
        }
        other => panic!("expected unrecoverable authority damage, got {other:?}"),
    }

    let indeterminate = seal_finding(
        ExecutedQuarantineFinding::from_index_page_denial(&inspect_without_authority_basis())
            .unwrap(),
    );
    match indeterminate.damage_classification() {
        DamageClassification::IndeterminatePhysicalDamage(damage) => {
            assert_eq!(
                indeterminate.locality().boundary(),
                QuarantineLocalityBoundary::ExactPhysicalScope(damage.scope())
            );
            assert_eq!(
                damage.missing_prerequisite(),
                RebuildabilityPrerequisite::CurrentAuthorityBasis
            );
        }
        other => panic!("expected indeterminate physical damage, got {other:?}"),
    }
}

#[test]
fn intact_derived_boundary_remains_distinct_when_quarantined_for_handoff() {
    let finding = ExecutedQuarantineFinding::from_index_page_report(
        &inspect_intact_derived_index_with_authority(),
    );
    let record = seal_finding(finding);

    assert!(matches!(
        record.damage_classification(),
        DamageClassification::IntactPhysicalBoundary(_)
    ));
}

#[test]
fn unknown_chunk_damage_preserves_ambiguous_quarantine_boundary() {
    let finding =
        ExecutedQuarantineFinding::from_chunk_denial(&inspect_unknown_chunk_denial()).unwrap();
    let record = seal_finding(finding);

    assert!(matches!(
        record.locality().boundary(),
        QuarantineLocalityBoundary::BroaderPhysicalBoundary(
            _,
            PhysicalBoundaryLocalization::AmbiguousBoundary
        )
    ));
    assert!(matches!(
        record.damage_classification(),
        DamageClassification::QuarantinedPhysicalDamage(damage)
            if damage.ambiguous_boundary().is_some()
    ));
}

#[test]
fn handoff_posture_stays_distinct_from_repair_or_recovery_claims() {
    let finding = executed_ambiguous_page_finding("handoff");
    let record = PhysicalQuarantineAuthority::seal(
        QuarantineSealRequest::from_executed_finding(finding)
            .with_handoff_posture(QuarantineHandoffPosture::RepairOwnerRequired),
    )
    .unwrap();

    assert_eq!(
        record.handoff_posture(),
        QuarantineHandoffPosture::RepairOwnerRequired
    );
    assert!(!record.proves_repair());
    assert!(!record.proves_recovery());
}

fn seal_ambiguous_page_damage(label: &str) -> worth_store_physical_integrity::QuarantineRecord {
    seal_finding(executed_ambiguous_page_finding(label))
}

fn seal_finding(
    finding: ExecutedQuarantineFinding,
) -> worth_store_physical_integrity::QuarantineRecord {
    PhysicalQuarantineAuthority::seal(QuarantineSealRequest::from_executed_finding(finding))
        .unwrap()
}

fn executed_ambiguous_page_finding(label: &str) -> ExecutedQuarantineFinding {
    let mut payload = page_payload_with_record(label.as_bytes());
    payload[0] = 9;
    let denial = inspect_page_denial(&payload);
    ExecutedQuarantineFinding::from_container_denial(&denial).unwrap()
}
