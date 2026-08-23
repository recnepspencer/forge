use worth_store_contracts::DurableArtifactFamilyId;
use worth_store_layout_indexes::integrity::{
    layout_readmission, RecoveryLayoutReadmissionAdmissionDenial,
    RecoveryLayoutReadmissionOutcomeView,
};
use worth_store_test_support::harness::layout::{
    audit_retained_layout_quarantine_record, authoritative_layout_quarantine_record,
    layout_integrity_authority, unresolved_layout_authority_record,
};

use super::{
    map_quarantine_readmission_outcome, map_quarantine_record, QuarantineReadmissionDenial,
    QuarantineReadmissionModel, QuarantineReadmissionState,
};

#[test]
fn real_quarantine_record_is_observation_not_repair_authority() {
    let record = authoritative_layout_quarantine_record("formal-quarantine-record");
    let observation = map_quarantine_record(&record);

    assert!(!observation.receipt_digest().is_empty());
    assert!(!observation.proves_repair());
    assert!(!record.proves_recovery());
    assert_eq!(
        observation.states().collect::<Vec<_>>(),
        vec![
            QuarantineReadmissionState::Proposed,
            QuarantineReadmissionState::Sealed,
        ]
    );
}

#[test]
fn real_layout_readmission_outcomes_map_to_readmitted_and_denied_states() {
    let fixture = layout_integrity_authority("formal-quarantine-admitted");
    let record = authoritative_layout_quarantine_record("formal-quarantine-admitted");
    let admitted = layout_readmission().admit_quarantine(
        DurableArtifactFamilyId::PhysicalRootManifest,
        &record,
        fixture.current_authority(),
        fixture.security_scope().witnesses(),
    );
    assert!(matches!(
        admitted.view(),
        RecoveryLayoutReadmissionOutcomeView::Readmitted(_)
    ));
    assert_eq!(
        map_quarantine_readmission_outcome(admitted.view())
            .states()
            .collect::<Vec<_>>(),
        vec![
            QuarantineReadmissionState::RecoveryVerificationPending,
            QuarantineReadmissionState::Readmitted,
        ]
    );

    let denied_record = unresolved_layout_authority_record("formal-quarantine-denied");
    let denied = layout_readmission().admit_quarantine(
        DurableArtifactFamilyId::PhysicalRootManifest,
        &denied_record,
        fixture.current_authority(),
        fixture.security_scope().witnesses(),
    );
    assert!(matches!(
        denied.view(),
        RecoveryLayoutReadmissionOutcomeView::Denied(
            RecoveryLayoutReadmissionAdmissionDenial::UnexpectedReadmissionClass { .. }
        )
    ));
    assert_eq!(
        map_quarantine_readmission_outcome(denied.view())
            .states()
            .collect::<Vec<_>>(),
        vec![
            QuarantineReadmissionState::RecoveryVerificationPending,
            QuarantineReadmissionState::Denied,
        ]
    );
}

#[test]
fn audit_retention_handoff_maps_to_retained_for_audit() {
    let fixture = layout_integrity_authority("formal-quarantine-audit-retention");
    let record = audit_retained_layout_quarantine_record("formal-quarantine-audit-retention");
    let retained = layout_readmission().admit_quarantine(
        DurableArtifactFamilyId::PhysicalRootManifest,
        &record,
        fixture.current_authority(),
        fixture.security_scope().witnesses(),
    );

    assert!(matches!(
        retained.view(),
        RecoveryLayoutReadmissionOutcomeView::Denied(
            RecoveryLayoutReadmissionAdmissionDenial::NoForegroundAuthority
        )
    ));
    assert_eq!(
        map_quarantine_readmission_outcome(retained.view())
            .states()
            .collect::<Vec<_>>(),
        vec![
            QuarantineReadmissionState::RecoveryVerificationPending,
            QuarantineReadmissionState::RetainedForAudit,
        ]
    );
}

#[test]
fn readmission_requires_exact_scope_verification_and_current_authority() {
    let scope = "segment:7/page:2/generation:4";
    let mut model = QuarantineReadmissionModel::sealed(scope);
    model.begin_verification();

    assert_eq!(
        model.readmit(scope, false, true),
        Err(QuarantineReadmissionDenial::VerificationFrontierIncomplete)
    );
    assert_eq!(
        model.readmit("segment:7/page:9/generation:4", true, true),
        Err(QuarantineReadmissionDenial::ScopeMismatch)
    );
    assert_eq!(
        model.readmit(scope, true, false),
        Err(QuarantineReadmissionDenial::CurrentAuthorityRequired)
    );
    model.readmit(scope, true, true).unwrap();
    assert_eq!(model.state(), QuarantineReadmissionState::Readmitted);
}

#[test]
fn observation_and_operator_intent_are_not_repair_authority() {
    assert_eq!(
        QuarantineReadmissionModel::reject_offline_observation(),
        QuarantineReadmissionDenial::ObservationIsNotRepairAuthority
    );
    assert_eq!(
        QuarantineReadmissionModel::reject_operator_repair(),
        QuarantineReadmissionDenial::OperatorIntentIsNotRepairAuthority
    );
}
