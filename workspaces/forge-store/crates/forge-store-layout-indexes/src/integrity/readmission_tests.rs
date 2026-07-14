pub(super) use super::readmission_test_support::import_witness;
use super::readmission_test_support::{
    authoritative_quarantine_record, current_authority, current_security_scope,
    current_security_scope_with, offline_witness, quarantine_witness, record_backed_witness,
    record_backed_witness_for_scope,
};
use super::tests::{
    admitted_family, admitted_family_for_store, family, offline_admission, other_family,
};
use crate::integrity::{
    import_readmission, layout_corruption, offline_readmission, quarantine_readmission,
    CorruptionDenial, ImportReadmissionView, LayoutCorruptionView, LayoutReadmissionSource,
    OfflineReadmissionView, QuarantineReadmissionView,
};
use forge_store_security::{StoreKeyScope, StoreTenantScope};

mod case_coverage;

#[test]
fn offline_readmission_resumes_foreground_authority_with_family_bound_store_witness() {
    let required = layout_corruption()
        .require_offline_readmission(admitted_family(), &offline_admission("offline-success"))
        .into_offline_readmission_requirement()
        .expect("offline classification must issue offline readmission requirement");
    let outcome =
        offline_readmission().admit(required, offline_witness(family(), "offline-success"));
    let counters = outcome.counters();
    assert_eq!(counters.evidence_witnesses_inspected(), 1);
    assert_eq!(counters.identity_bindings_checked(), 1);
    assert_eq!(counters.replay_frontiers_checked(), 1);
    assert_eq!(counters.foreground_witnesses_issued(), 1);

    assert!(matches!(
        outcome.view(),
        OfflineReadmissionView::Readmitted(witness)
            if witness.family() == family()
                && witness.source() == LayoutReadmissionSource::OfflineRecoveryEvidence
    ));
}

#[test]
fn quarantine_readmission_resumes_foreground_authority_with_family_bound_store_witness() {
    let quarantine_record = authoritative_quarantine_record("quarantine-success");
    let classified = layout_corruption().assess_physical_quarantine(
        admitted_family_for_store("store.new.strategy"),
        quarantine_record.clone(),
    );
    assert_eq!(classified.counters().quarantine_records_inspected(), 1);
    let required = layout_corruption()
        .require_record_backed_recovery_readmission(
            classified,
            &current_authority("store.new.strategy", "quarantine-success"),
            current_security_scope("store.new.strategy", "quarantine-success").witnesses(),
        )
        .expect("record-backed quarantine should derive readmission requirement")
        .into_quarantine_readmission_requirement()
        .expect("record-backed classification must issue quarantine requirement");
    let outcome = quarantine_readmission().admit(
        required,
        record_backed_witness(family(), &quarantine_record, "quarantine-success"),
    );
    let counters = outcome.counters();
    assert_eq!(counters.evidence_witnesses_inspected(), 1);
    assert_eq!(counters.identity_bindings_checked(), 1);
    assert_eq!(counters.foreground_witnesses_issued(), 1);

    assert!(matches!(
        outcome.view(),
        QuarantineReadmissionView::Readmitted(witness)
            if witness.family() == family()
                && witness.source() == LayoutReadmissionSource::QuarantineRecovery
    ));
}

#[test]
fn quarantine_readmission_rejects_witness_for_different_family_or_artifact_identity() {
    let quarantine_record = authoritative_quarantine_record("quarantine-required-a");
    let required = layout_corruption()
        .require_record_backed_recovery_readmission(
            layout_corruption()
                .assess_physical_quarantine(admitted_family(), quarantine_record.clone()),
            &current_authority("store.new.strategy", "quarantine-required-a"),
            current_security_scope("store.new.strategy", "quarantine-required-a").witnesses(),
        )
        .expect("record-backed quarantine should derive readmission requirement")
        .into_quarantine_readmission_requirement()
        .expect("record-backed classification must issue quarantine requirement");

    let wrong_family_required = layout_corruption()
        .require_record_backed_recovery_readmission(
            layout_corruption()
                .assess_physical_quarantine(admitted_family(), quarantine_record.clone()),
            &current_authority("store.new.strategy", "quarantine-required-a"),
            current_security_scope("store.new.strategy", "quarantine-required-a").witnesses(),
        )
        .unwrap()
        .into_quarantine_readmission_requirement()
        .unwrap();
    let wrong_family = quarantine_readmission().admit(
        wrong_family_required,
        record_backed_witness(other_family(), &quarantine_record, "quarantine-required-a"),
    );

    let wrong_identity = quarantine_readmission().admit(
        required,
        quarantine_witness(family(), "quarantine-required-b"),
    );

    assert!(matches!(
        wrong_family.view(),
        QuarantineReadmissionView::Denied(denied)
            if matches!(denied.denial(), CorruptionDenial::FamilyBoundReadmissionWitnessRequired {
                family: actual_family,
                source: LayoutReadmissionSource::QuarantineRecovery,
            } if *actual_family == family())
    ));
    assert!(matches!(
        wrong_identity.view(),
        QuarantineReadmissionView::Denied(denied)
            if matches!(denied.denial(), CorruptionDenial::FamilyBoundReadmissionWitnessRequired {
                family: actual_family,
                source: LayoutReadmissionSource::QuarantineRecovery,
            } if *actual_family == family())
    ));
}

#[test]
fn quarantine_readmission_rejects_cross_tenant_and_cross_key_scope_substitution() {
    let record = authoritative_quarantine_record("quarantine-security-scope");
    let authority = current_authority("store.new.strategy", "quarantine-security-scope");

    for mismatched_security in [
        current_security_scope_with(
            "store.new.strategy",
            "quarantine-security-scope",
            StoreKeyScope::TenantEnvelope,
            StoreTenantScope::StoreInternal,
        ),
        current_security_scope_with(
            "store.new.strategy",
            "quarantine-security-scope",
            StoreKeyScope::StoreManagedRoot,
            StoreTenantScope::MultiTenantPhysicalBoundary,
        ),
    ] {
        let denied = layout_corruption().require_record_backed_recovery_readmission(
            layout_corruption().assess_physical_quarantine(admitted_family(), record.clone()),
            &authority,
            mismatched_security.witnesses(),
        );
        assert!(matches!(
            denied,
            Err(CorruptionDenial::SecurityScopeReadmissionMismatch { .. })
        ));
    }

    let required = layout_corruption()
        .require_record_backed_recovery_readmission(
            layout_corruption().assess_physical_quarantine(admitted_family(), record.clone()),
            &authority,
            current_security_scope("store.new.strategy", "quarantine-security-scope").witnesses(),
        )
        .unwrap()
        .into_quarantine_readmission_requirement()
        .unwrap();
    let foreign_scope_witness = record_backed_witness_for_scope(
        family(),
        &record,
        "store.new.strategy",
        "quarantine-security-scope",
        StoreKeyScope::TenantEnvelope,
        StoreTenantScope::StoreInternal,
    );

    assert!(matches!(
        quarantine_readmission()
            .admit(required, foreign_scope_witness)
            .view(),
        QuarantineReadmissionView::Denied(denied)
            if matches!(denied.denial(), CorruptionDenial::FamilyBoundReadmissionWitnessRequired {
                source: LayoutReadmissionSource::QuarantineRecovery,
                ..
            })
    ));
}

#[test]
fn offline_readmission_rejects_witness_for_different_family_or_artifact_identity() {
    let required = layout_corruption()
        .require_offline_readmission(admitted_family(), &offline_admission("offline-required-a"))
        .into_offline_readmission_requirement()
        .unwrap();

    let wrong_family_required = layout_corruption()
        .require_offline_readmission(admitted_family(), &offline_admission("offline-required-a"))
        .into_offline_readmission_requirement()
        .unwrap();
    let wrong_family = offline_readmission().admit(
        wrong_family_required,
        offline_witness(other_family(), "offline-required-a"),
    );

    let wrong_identity =
        offline_readmission().admit(required, offline_witness(family(), "offline-required-b"));

    assert!(matches!(
        wrong_family.view(),
        OfflineReadmissionView::Denied(denied)
            if matches!(denied.denial(), CorruptionDenial::FamilyBoundReadmissionWitnessRequired {
                family: actual_family,
                source: LayoutReadmissionSource::OfflineRecoveryEvidence,
            } if *actual_family == family())
    ));
    assert!(matches!(
        wrong_identity.view(),
        OfflineReadmissionView::Denied(denied)
            if matches!(denied.denial(), CorruptionDenial::FamilyBoundReadmissionWitnessRequired {
                family: actual_family,
                source: LayoutReadmissionSource::OfflineRecoveryEvidence,
            } if *actual_family == family())
    ));
}

#[test]
fn terminal_import_readmission_resumes_foreground_authority_with_family_bound_store_witness() {
    let required = layout_corruption()
        .require_import_readmission(
            admitted_family(),
            import_witness(family(), "terminal-import-success"),
        )
        .into_import_readmission_requirement()
        .unwrap();
    let outcome = import_readmission().admit(
        required,
        import_witness(family(), "terminal-import-success"),
    );
    let counters = outcome.counters();
    assert_eq!(counters.evidence_witnesses_inspected(), 1);
    assert_eq!(counters.identity_bindings_checked(), 1);
    assert_eq!(counters.foreground_witnesses_issued(), 1);

    assert!(matches!(
        outcome.view(),
        ImportReadmissionView::Readmitted(witness)
            if witness.family() == family()
                && witness.source() == LayoutReadmissionSource::TerminalImport
    ));
}

#[test]
fn terminal_import_does_not_accept_offline_recovery_witness_as_readmission_authority() {
    let required = layout_corruption()
        .require_import_readmission(
            admitted_family(),
            import_witness(family(), "terminal-import"),
        )
        .into_import_readmission_requirement()
        .unwrap();
    let outcome = import_readmission().admit(
        required,
        offline_witness(family(), "offline-terminal-mismatch"),
    );

    assert!(matches!(
        outcome.view(),
        ImportReadmissionView::Denied(denied)
            if matches!(denied.denial(), CorruptionDenial::FamilyBoundReadmissionWitnessRequired {
                family: actual_family,
                source: LayoutReadmissionSource::TerminalImport,
            } if *actual_family == family())
    ));
}

#[test]
fn terminal_import_keeps_receipt_identity_in_required_outcome() {
    let witness = import_witness(family(), "terminal-import-identity");
    let expected_identity = witness.identity().clone();
    let required = layout_corruption().require_import_readmission(admitted_family(), witness);

    assert!(matches!(
        required.view(),
        LayoutCorruptionView::ImportReadmissionRequired(requirement)
            if requirement.family() == family()
                && requirement.identity() == &expected_identity
    ));
}

#[test]
fn terminal_import_readmission_rejects_witness_for_different_family_or_artifact_identity() {
    let required = layout_corruption()
        .require_import_readmission(
            admitted_family(),
            import_witness(family(), "terminal-required-a"),
        )
        .into_import_readmission_requirement()
        .unwrap();

    let wrong_family_required = layout_corruption()
        .require_import_readmission(
            admitted_family(),
            import_witness(family(), "terminal-required-a"),
        )
        .into_import_readmission_requirement()
        .unwrap();
    let wrong_family = import_readmission().admit(
        wrong_family_required,
        import_witness(other_family(), "terminal-required-a"),
    );

    let wrong_identity =
        import_readmission().admit(required, import_witness(family(), "terminal-required-b"));

    assert!(matches!(
        wrong_family.view(),
        ImportReadmissionView::Denied(denied)
            if matches!(denied.denial(), CorruptionDenial::FamilyBoundReadmissionWitnessRequired {
                family: actual_family,
                source: LayoutReadmissionSource::TerminalImport,
            } if *actual_family == family())
    ));
    assert!(matches!(
        wrong_identity.view(),
        ImportReadmissionView::Denied(denied)
            if matches!(denied.denial(), CorruptionDenial::FamilyBoundReadmissionWitnessRequired {
                family: actual_family,
                source: LayoutReadmissionSource::TerminalImport,
            } if *actual_family == family())
    ));
}
