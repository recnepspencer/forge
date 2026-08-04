use super::super::{
    admit_support_trust_request, check_support_trust_drift, check_support_trust_equivalence,
    classify_operational_support_trust, translate_support_trust_inputs, RawSupportTrustRequest,
    SupportImportAdmissionReceipt, SupportTrustBatchCardinality, SupportTrustClass,
    SupportTrustDriftScanPlan, SupportTrustEquivalenceContract, SupportTrustEquivalenceEvidence,
    SupportTrustEquivalenceLane, SupportTrustEvidenceBudget, SupportTrustFailureKind,
    SupportTrustProvenance, SupportTrustReceiptStatus, SupportTrustRequestedUse,
    SupportTrustStrength,
};
use super::equivalence_evidence::exact_equivalence_contract;
use super::operational_basis::{basis, epochs, phase2_performance_plan, raw_phase2_request};
use super::receipt_evidence::phase2_receipts;
use crate::subscription_support::{
    SubscriptionResumeClassification, SubscriptionSupportArtifactId, SubscriptionSupportFamilyId,
    SubscriptionSupportOperationalVerdict, SubscriptionSupportRole,
};

#[test]
fn phase2_transformed_exact_waits_for_phase3_equivalence() {
    let admitted = admit_support_trust_request(
        raw_phase2_request(
            SupportTrustStrength::Exact,
            SupportTrustProvenance::Replicated,
        ),
        phase2_receipts(
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        ),
    )
    .unwrap();
    let translated = translate_support_trust_inputs(admitted).unwrap();
    let drift_checked = check_support_trust_drift(
        translated,
        SupportTrustDriftScanPlan::foreground_support_identity(),
    )
    .unwrap();

    let error =
        check_support_trust_equivalence(drift_checked, SupportTrustEquivalenceEvidence::none())
            .expect_err("transformed exact trust requires Phase 3 equivalence proof");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustEquivalenceMissing
    );
}

#[test]
fn phase3_replicated_exact_classifies_with_full_equivalence_contract() {
    let admitted = admit_support_trust_request(
        raw_phase2_request(
            SupportTrustStrength::Exact,
            SupportTrustProvenance::Replicated,
        ),
        phase2_receipts(
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        ),
    )
    .unwrap();
    let translated = translate_support_trust_inputs(admitted).unwrap();
    let drift_checked = check_support_trust_drift(
        translated,
        SupportTrustDriftScanPlan::foreground_support_identity(),
    )
    .unwrap();
    let equivalence = SupportTrustEquivalenceEvidence::none()
        .with_contract(exact_equivalence_contract(
            SupportTrustEquivalenceLane::Replication,
        ))
        .unwrap();
    let equivalence_checked = check_support_trust_equivalence(drift_checked, equivalence).unwrap();
    let classified = classify_operational_support_trust(equivalence_checked).unwrap();

    assert_eq!(
        classified.report().trust_strength(),
        SupportTrustStrength::Exact
    );
    assert_eq!(
        classified.report().trust_class(),
        SupportTrustClass::ReplicatedSupportTrusted
    );
    assert_eq!(classified.cost_surface().equivalence_checks_performed(), 2);
}

#[test]
fn phase3_migrated_exact_classifies_only_with_migration_equivalence() {
    let admitted = admit_support_trust_request(
        raw_phase2_request(
            SupportTrustStrength::Exact,
            SupportTrustProvenance::Migrated,
        ),
        phase2_receipts(
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        ),
    )
    .unwrap();
    let translated = translate_support_trust_inputs(admitted).unwrap();
    let drift_checked = check_support_trust_drift(
        translated,
        SupportTrustDriftScanPlan::foreground_support_identity(),
    )
    .unwrap();
    let equivalence = SupportTrustEquivalenceEvidence::none()
        .with_contract(exact_equivalence_contract(
            SupportTrustEquivalenceLane::Migration,
        ))
        .unwrap();
    let equivalence_checked = check_support_trust_equivalence(drift_checked, equivalence).unwrap();
    let classified = classify_operational_support_trust(equivalence_checked).unwrap();

    assert_eq!(
        classified.report().trust_strength(),
        SupportTrustStrength::Exact
    );
    assert_eq!(
        classified.report().trust_class(),
        SupportTrustClass::MigratedSupportTrusted
    );
}

#[test]
fn phase3_imported_exact_requires_admission_and_import_equivalence() {
    let bundle = phase2_receipts(
        SubscriptionResumeClassification::Exact,
        SubscriptionSupportOperationalVerdict::ExactResumePreserved,
    )
    .with_import_admission(
        SupportImportAdmissionReceipt::new(
            SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            "import:admission",
            SupportTrustReceiptStatus::Proven,
        )
        .unwrap(),
    );
    let request = RawSupportTrustRequest::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
        SupportTrustStrength::Exact,
        SupportTrustProvenance::Imported,
        SupportTrustRequestedUse::StoreLocalResume,
        SupportTrustBatchCardinality::SingleSupportArtifact,
        epochs(),
        phase2_performance_plan(),
        SupportTrustEvidenceBudget::new(4096, 9, 1).unwrap(),
    );
    let admitted = admit_support_trust_request(request, bundle).unwrap();
    let translated = translate_support_trust_inputs(admitted).unwrap();
    let drift_checked = check_support_trust_drift(
        translated,
        SupportTrustDriftScanPlan::foreground_support_identity(),
    )
    .unwrap();
    let equivalence = SupportTrustEquivalenceEvidence::none()
        .with_contract(exact_equivalence_contract(
            SupportTrustEquivalenceLane::Import,
        ))
        .unwrap();
    let equivalence_checked = check_support_trust_equivalence(drift_checked, equivalence).unwrap();
    let classified = classify_operational_support_trust(equivalence_checked).unwrap();

    assert_eq!(
        classified.report().trust_strength(),
        SupportTrustStrength::Exact
    );
    assert_eq!(
        classified.report().provenance(),
        SupportTrustProvenance::Imported
    );
}

#[test]
fn phase3_equivalence_contract_rejects_role_and_portability_drift() {
    let admitted = admit_support_trust_request(
        raw_phase2_request(
            SupportTrustStrength::Exact,
            SupportTrustProvenance::Replicated,
        ),
        phase2_receipts(
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        ),
    )
    .unwrap();
    let translated = translate_support_trust_inputs(admitted).unwrap();
    let drift_checked = check_support_trust_drift(
        translated,
        SupportTrustDriftScanPlan::foreground_support_identity(),
    )
    .unwrap();
    let bad_contract = SupportTrustEquivalenceContract::new(
        SupportTrustEquivalenceLane::Replication,
        basis(),
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportRole::DegradedContinuation,
        SubscriptionSupportArtifactId("artifact:trust:phase-1".into()),
        "basis:trust",
        "cursor:trust",
        "checkpoint:trust",
        "compatibility:trust",
        "portability:wrong",
        SubscriptionResumeClassification::Exact,
        SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        "equivalence:exact",
    )
    .unwrap();
    let equivalence = SupportTrustEquivalenceEvidence::none()
        .with_contract(bad_contract)
        .unwrap();

    let error = check_support_trust_equivalence(drift_checked, equivalence)
        .expect_err("role drift must reject before digest-only equivalence can pass");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustRoleMismatch
    );
}

#[test]
fn phase3_native_exact_rejects_irrelevant_equivalence_evidence() {
    let admitted = admit_support_trust_request(
        raw_phase2_request(
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
        ),
        phase2_receipts(
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        ),
    )
    .unwrap();
    let translated = translate_support_trust_inputs(admitted).unwrap();
    let drift_checked = check_support_trust_drift(
        translated,
        SupportTrustDriftScanPlan::foreground_support_identity(),
    )
    .unwrap();
    let equivalence = SupportTrustEquivalenceEvidence::none()
        .with_contract(exact_equivalence_contract(
            SupportTrustEquivalenceLane::Replication,
        ))
        .unwrap();

    let error = check_support_trust_equivalence(drift_checked, equivalence)
        .expect_err("native exact trust must not accept loose transformed evidence");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustEquivalenceMissing
    );
}
