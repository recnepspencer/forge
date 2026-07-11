#[allow(dead_code)]
#[path = "../../../support/scheduling/access_policy_support/access_policy_support.rs"]
mod s6_access_policy_support;
#[path = "../evidence_materialization_support/mod.rs"]
mod s6_evidence_materialization_support;

use forge_store_certification::{
    adopt_materialized_s6_certification_evidence_for_closeout,
    materialize_s6_certification_evidence,
    reject_materialized_s6_certification_as_runtime_authority,
    S6CertificationMaterializationDenial, S6CertificationRuntimeAuthorityDenial,
    S6CounterStrengthFamily, S6FoundationalAuthorityBoundary, S6MaterializedCounterStrength,
    S6PostAdmissionViolationCause, S6PostAdmissionViolationFamily,
    S6ReadinessResidualDebtEvidenceKind,
};
use forge_store_io_scheduler::BackgroundDebtKind;
use forge_store_physical_backend::AccessPolicyViolationKind;

#[test]
fn materialized_s6_evidence_binds_store_witnesses_to_foundational_and_proof_surfaces() {
    let sources = s6_evidence_materialization_support::sources();

    let bundle = materialize_s6_certification_evidence(sources)
        .expect("executed S.6 evidence should materialize");

    assert!(bundle.is_courtroom_evidence_over_executed_store_law());
    assert_eq!(
        bundle.profiles().authority_boundary(),
        S6FoundationalAuthorityBoundary::CertificationEvidenceOnly
    );
    assert!(bundle.performance().has_required_counter_contracts());
    assert!(bundle.proof().is_checked_from_executed_store_law());
    assert_ne!(bundle.canonical().execution_identity_tag(), 0);
    assert_eq!(bundle.canonical().lane_binding_mask().count_ones(), 10);
    assert_eq!(
        bundle
            .proof()
            .projection()
            .payload()
            .execution_identity_tag(),
        bundle.canonical().execution_identity_tag()
    );
    assert_eq!(
        bundle.proof().projection().payload().lane_binding_mask(),
        bundle.canonical().lane_binding_mask()
    );
    assert!(bundle
        .counter_strengths()
        .iter()
        .any(|row| row.family() == S6CounterStrengthFamily::QualificationMatrix));
    assert!(bundle.proof().projection().payload().checked_execution());
    assert_eq!(bundle.canonical().access_policy_rows(), 2);
    assert_eq!(bundle.canonical().post_admission_violation_rows(), 2);
    assert_eq!(
        bundle.proof().projection().payload().access_policy_rows(),
        2
    );
    assert_eq!(
        bundle
            .proof()
            .projection()
            .payload()
            .post_admission_violation_rows(),
        2
    );
}

#[test]
fn materialization_marks_certification_only_strength_when_lower_strength_is_not_exposed() {
    let bundle =
        materialize_s6_certification_evidence(s6_evidence_materialization_support::sources())
            .expect("executed S.6 evidence should materialize");

    s6_evidence_materialization_support::assert_fixture_counter_strength_matrix(&bundle);
    s6_evidence_materialization_support::assert_performance_receipts_are_exact_for_fixture(&bundle);
}

#[test]
fn independently_built_sources_materialize_equivalent_replay_and_proof_surfaces() {
    let first =
        materialize_s6_certification_evidence(s6_evidence_materialization_support::sources())
            .expect("first executed S.6 evidence should materialize");
    let second =
        materialize_s6_certification_evidence(s6_evidence_materialization_support::sources())
            .expect("second executed S.6 evidence should materialize");

    assert_eq!(first.canonical(), second.canonical());
    assert_eq!(first.proof(), second.proof());
    assert_eq!(first.performance(), second.performance());
    assert_eq!(first.profiles(), second.profiles());
    assert_eq!(first.counter_strengths(), second.counter_strengths());
    assert_eq!(
        first
            .harness_closeout()
            .harness_evidence()
            .executed_replay_coverage_rows(),
        second
            .harness_closeout()
            .harness_evidence()
            .executed_replay_coverage_rows()
    );
    assert_eq!(
        first.post_admission_violations(),
        second.post_admission_violations()
    );
}

#[test]
fn materialized_s6_evidence_is_adopted_by_closeout_without_runtime_authority() {
    let bundle =
        materialize_s6_certification_evidence(s6_evidence_materialization_support::sources())
            .expect("executed S.6 evidence should materialize");

    let receipt = adopt_materialized_s6_certification_evidence_for_closeout(&bundle)
        .expect("readiness closeout should adopt materialized evidence");

    assert_eq!(receipt.profile_count(), 6);
    assert!(receipt.profile_boundary_certification_only());
    assert_eq!(receipt.performance_receipt_count(), 4);
    s6_evidence_materialization_support::assert_readiness_fixture_counter_strength_matrix(&receipt);
    assert_ne!(receipt.canonical_execution_identity_tag(), 0);
    assert_eq!(
        receipt.canonical_execution_identity_tag(),
        receipt.proof_execution_identity_tag()
    );
    assert_eq!(receipt.canonical_lane_binding_mask().count_ones(), 10);
    assert_eq!(
        receipt.canonical_lane_binding_mask(),
        receipt.proof_lane_binding_mask()
    );
    assert_eq!(receipt.canonical_access_policy_rows(), 2);
    assert_eq!(receipt.canonical_post_admission_violation_rows(), 2);
    assert_eq!(receipt.proof().post_admission_violation_rows(), 2);
    assert!(receipt.proof().checked_execution());
    assert_eq!(
        receipt.canonical_access_policy_rows(),
        receipt.proof().access_policy_rows()
    );
    s6_evidence_materialization_support::assert_readiness_residual_debt_matrix(
        &receipt,
        &[
            (
                S6ReadinessResidualDebtEvidenceKind::UnsupportedBackendProfile,
                1,
            ),
            (S6ReadinessResidualDebtEvidenceKind::UnavailableEvidence, 2),
            (
                S6ReadinessResidualDebtEvidenceKind::DegradedBackendPosture,
                2,
            ),
            (S6ReadinessResidualDebtEvidenceKind::DeniedClaim, 6),
            (S6ReadinessResidualDebtEvidenceKind::StaleEvidence, 2),
            (S6ReadinessResidualDebtEvidenceKind::RebindRequired, 1),
            (
                S6ReadinessResidualDebtEvidenceKind::ResidualQualificationDebt,
                6,
            ),
        ],
    );
    assert!(receipt
        .proof_topology()
        .is_checked_for_closeout(receipt.proof()));
}

#[test]
fn certification_bundle_is_rejected_as_runtime_authority() {
    let bundle =
        materialize_s6_certification_evidence(s6_evidence_materialization_support::sources())
            .expect("executed S.6 evidence should materialize");

    let denial = reject_materialized_s6_certification_as_runtime_authority(&bundle);

    assert_eq!(
        denial,
        S6CertificationRuntimeAuthorityDenial::CertificationEvidenceCannotStrengthenBackendCapability
    );
}

#[test]
fn materialization_requires_access_policy_evidence() {
    let sources = s6_evidence_materialization_support::sources_without_access_policy_rows();

    let denial = materialize_s6_certification_evidence(sources)
        .expect_err("access policy evidence is mandatory");

    assert!(matches!(
        denial,
        S6CertificationMaterializationDenial::MissingAccessPolicyEvidence
    ));
}

#[test]
fn materialization_derives_post_admission_violations_from_store_outcomes() {
    let sources = s6_evidence_materialization_support::sources_without_post_admission_violations();

    let bundle = materialize_s6_certification_evidence(sources)
        .expect("background violation should still materialize as post-admission evidence");

    assert_eq!(bundle.canonical().post_admission_violation_rows(), 1);
    assert_eq!(
        bundle
            .proof()
            .projection()
            .payload()
            .post_admission_violation_rows(),
        1
    );
    assert_eq!(bundle.post_admission_violations().len(), 1);
    assert_eq!(
        bundle.post_admission_violations()[0].family(),
        S6PostAdmissionViolationFamily::BackgroundPacing
    );
    assert_eq!(
        bundle.post_admission_violations()[0].cause(),
        S6PostAdmissionViolationCause::BackgroundPacing(BackgroundDebtKind::RepairPressure)
    );
    assert_eq!(
        bundle.post_admission_violations()[0].observed_violations(),
        1
    );
    assert_eq!(
        bundle.post_admission_violations()[0].counter_strength(),
        S6MaterializedCounterStrength::Derived
    );
}

#[test]
fn materialization_requires_flush_durability_rows() {
    let sources = s6_evidence_materialization_support::sources_without_flush_rows();

    let denial = materialize_s6_certification_evidence(sources)
        .expect_err("flush durability evidence is mandatory");

    assert!(matches!(
        denial,
        S6CertificationMaterializationDenial::MissingFlushDurabilityEvidence
    ));
}

#[test]
fn materialization_exposes_causal_post_admission_violation_rows() {
    let bundle =
        materialize_s6_certification_evidence(s6_evidence_materialization_support::sources())
            .expect("executed S.6 evidence should materialize");

    assert_eq!(bundle.post_admission_violations().len(), 2);
    s6_evidence_materialization_support::assert_violation_row(
        &bundle,
        S6PostAdmissionViolationFamily::BackgroundPacing,
        S6PostAdmissionViolationCause::BackgroundPacing(BackgroundDebtKind::RepairPressure),
        1,
        S6MaterializedCounterStrength::Derived,
    );
    s6_evidence_materialization_support::assert_violation_row(
        &bundle,
        S6PostAdmissionViolationFamily::AccessPolicy,
        S6PostAdmissionViolationCause::AccessPolicy(AccessPolicyViolationKind::MmapLazyFault),
        1,
        S6MaterializedCounterStrength::Exact,
    );
}

#[test]
fn materialization_rejects_near_miss_store_execution_bindings() {
    s6_evidence_materialization_support::assert_source_denial(
        s6_evidence_materialization_support::sources_with_backend_profile_mismatch(),
        S6CertificationMaterializationDenial::StoreEvidenceBackendBindingMismatch,
    );
    s6_evidence_materialization_support::assert_source_denial(
        s6_evidence_materialization_support::sources_with_backend_evidence_class_mismatch(),
        S6CertificationMaterializationDenial::StoreEvidenceBackendBindingMismatch,
    );
    s6_evidence_materialization_support::assert_source_denial(
        s6_evidence_materialization_support::sources_with_access_policy_backend_mismatch(),
        S6CertificationMaterializationDenial::StoreEvidenceBackendBindingMismatch,
    );
    let empty_qualification =
        s6_evidence_materialization_support::sources_with_empty_qualification_matrix()
            .expect("empty qualification is structurally source-bindable");
    assert_eq!(
        materialize_s6_certification_evidence(empty_qualification)
            .expect_err("empty qualification must not materialize"),
        S6CertificationMaterializationDenial::EmptyQualificationMatrix
    );
}
