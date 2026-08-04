use super::super::{
    SubscriptionSupportAccuracyCertificationRowKind,
    SubscriptionSupportAccuracyCertificationRunner, SubscriptionSupportAccuracyCertificationSuite,
    SubscriptionSupportAccuracyPersistencePosture, SupportCertificationBatchScope,
    SupportCertificationBatchScopeKind, SupportCertificationCounterSnapshot,
    SupportCertificationEvidenceBundle, SupportCertificationHandoffReport,
    SupportDomainCertificationBatchPlan, SupportDomainCertificationBundle,
    SupportDomainCertificationCounterSnapshot, SupportGenericCertificationCounterSnapshot,
    SupportGenericCertificationReport, SupportTrustAllocationScope, SupportTrustDensityClass,
    SupportTrustFailureKind, SupportTrustPathClass,
    SUBSCRIPTION_SUPPORT_ACCURACY_CERTIFICATION_SUITE_NAME,
};
use super::accuracy_lane_evidence::phase7_lane_evidence;
use super::certification_bundle::{
    certified_first_ship_support_trust, first_ship_certification_bundle,
    generic_support_certification_report, generic_support_certification_report_for,
};
use super::certification_coverage::first_ship_certification_matrix;
use super::domain_handoff::{
    first_ship_domain_batch_plan, first_ship_domain_rows, phase7_suite_artifacts,
};

#[test]
fn phase7_named_subscription_support_accuracy_suite_emits_required_outputs() {
    let (evidence_bundle, generic, domain, handoff) = phase7_suite_artifacts();
    let lane_evidence = phase7_lane_evidence();
    let suite =
        SubscriptionSupportAccuracyCertificationSuite::from_phase_artifacts_and_lane_evidence(
            &evidence_bundle,
            &generic,
            &domain,
            &handoff,
            &lane_evidence,
        )
        .unwrap();

    assert_eq!(
        suite.suite_name(),
        SUBSCRIPTION_SUPPORT_ACCURACY_CERTIFICATION_SUITE_NAME
    );
    assert_eq!(
        suite.rows().len(),
        SubscriptionSupportAccuracyCertificationRowKind::required().len()
    );
    assert_eq!(
        suite.counter_snapshot().required_row_count(),
        SubscriptionSupportAccuracyCertificationRowKind::required().len() as u64
    );
    assert_eq!(
        suite.required_outputs().artifact_digest(),
        evidence_bundle.artifact_digest()
    );
    assert_eq!(
        suite.required_outputs().subscription_support_digest(),
        evidence_bundle.subscription_support_digest()
    );
    assert_eq!(
        suite.required_outputs().diagnostics_digest(),
        evidence_bundle.diagnostics_digest()
    );
    assert_eq!(
        suite.required_outputs().counter_snapshot_digest(),
        evidence_bundle.counter_snapshot_digest()
    );
    assert_eq!(
        suite.required_outputs().certification_summary_digest(),
        evidence_bundle.certification_summary_digest()
    );
    assert!(!suite.suite_digest().is_empty());
}

#[test]
fn phase7_production_runner_emits_performance_access_and_persistence_closeout() {
    let (evidence_bundle, generic, domain, handoff) = phase7_suite_artifacts();
    let lane_evidence = phase7_lane_evidence();
    let run = SubscriptionSupportAccuracyCertificationRunner::production()
        .certify(
            &evidence_bundle,
            &generic,
            &domain,
            &handoff,
            &lane_evidence,
        )
        .expect("production runner must emit the named suite with closeout proof");

    assert_eq!(
        run.suite().rows().len(),
        SubscriptionSupportAccuracyCertificationRowKind::required().len()
    );
    assert_eq!(run.performance_closeout().certification_row_count(), 4);
    assert_eq!(
        run.performance_closeout().certification_index_probe_count(),
        4
    );
    assert_eq!(
        run.performance_closeout()
            .certification_receipt_reuse_count(),
        3
    );
    assert_eq!(
        run.performance_closeout().certification_allocation_count(),
        1
    );
    assert_eq!(run.performance_closeout().generic_row_count(), 1);
    assert_eq!(run.performance_closeout().generic_index_probe_count(), 1);
    assert_eq!(run.performance_closeout().generic_receipt_reuse_count(), 1);
    assert_eq!(run.performance_closeout().generic_allocation_count(), 1);
    assert_eq!(run.performance_closeout().domain_scenario_row_count(), 5);
    assert_eq!(run.performance_closeout().domain_index_probe_count(), 5);
    assert_eq!(run.performance_closeout().domain_receipt_reuse_count(), 4);
    assert_eq!(run.performance_closeout().domain_allocation_count(), 1);
    assert_eq!(run.performance_closeout().global_scan_debt_count(), 0);
    assert_eq!(
        run.access_closeout().certified_semantic_domain_row_count(),
        3
    );
    assert_eq!(
        run.access_closeout().explicit_advanced_family_debt_count(),
        2
    );
    assert!(run.access_closeout().handoff_semantic_trust_closed());
    assert!(run.access_closeout().roadmap2_physical_debt_explicit());
    assert!(run.access_closeout().milestone15_extension_debt_explicit());
    assert_eq!(
        run.persistence_posture(),
        SubscriptionSupportAccuracyPersistencePosture::InMemoryCertificationOnly
    );
    assert!(!run.run_digest().is_empty());
}

#[test]
fn phase7_runner_rejects_handoff_not_bound_to_phase_artifacts() {
    let (evidence_bundle, generic, domain, _) = phase7_suite_artifacts();
    let mismatched_generic = generic_support_certification_report_for(
        "generic:subscription-support-trust:mismatched",
        certified_first_ship_support_trust(),
    );
    let mismatched_handoff =
        SupportCertificationHandoffReport::from_generic_and_domain_certification(
            &mismatched_generic,
            &domain,
        )
        .unwrap();
    let error = SubscriptionSupportAccuracyCertificationRunner::production()
        .certify(
            &evidence_bundle,
            &generic,
            &domain,
            &mismatched_handoff,
            &phase7_lane_evidence(),
        )
        .expect_err("runner must reject a handoff digest from a different generic artifact");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustCoverageMissing
    );
}

#[test]
fn phase7_runner_rejects_certification_counter_regime_drift() {
    let drifted_scope = SupportCertificationBatchScope::new(
        SupportCertificationBatchScopeKind::CertificationScopeLocal,
        SupportTrustDensityClass::CertificationScopeLocal,
        SupportTrustPathClass::BatchCertificationPath,
        SupportTrustAllocationScope::BatchCertification,
        4,
        5,
        3,
        1,
    )
    .unwrap();
    let evidence_bundle = SupportCertificationEvidenceBundle::new(
        "run:13.3:first-ship:wrong-index-probes",
        first_ship_certification_matrix(),
        drifted_scope,
        SupportCertificationCounterSnapshot::new(4, 4, 3, 5, 1, 0, 0),
    )
    .unwrap();
    let generic = generic_support_certification_report();
    let domain = SupportDomainCertificationBundle::new(
        first_ship_domain_rows(&generic),
        first_ship_domain_batch_plan(),
        SupportDomainCertificationCounterSnapshot::new(5, 3, 2, 5, 4, 1, 2),
    )
    .unwrap();
    let handoff =
        SupportCertificationHandoffReport::from_generic_and_domain_certification(&generic, &domain)
            .unwrap();
    let error = SubscriptionSupportAccuracyCertificationRunner::production()
        .certify(
            &evidence_bundle,
            &generic,
            &domain,
            &handoff,
            &phase7_lane_evidence(),
        )
        .expect_err("runner closeout must reject a valid bundle whose counter regime drifted");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustCoverageMissing
    );
}

#[test]
fn phase7_runner_rejects_generic_performance_without_physical_debt_counter() {
    let evidence_bundle = first_ship_certification_bundle();
    let certified = certified_first_ship_support_trust();
    let generic = SupportGenericCertificationReport::from_certified_support_trust(
        "generic:subscription-support-trust:missing-physical-debt-counter",
        certified.report().clone(),
        certified.coverage_witness(),
        SupportGenericCertificationCounterSnapshot::new(1, 1, 1, 1, 1, 0).unwrap(),
    )
    .unwrap();
    let domain = SupportDomainCertificationBundle::new(
        first_ship_domain_rows(&generic),
        first_ship_domain_batch_plan(),
        SupportDomainCertificationCounterSnapshot::new(5, 3, 2, 5, 4, 1, 2),
    )
    .unwrap();
    let handoff =
        SupportCertificationHandoffReport::from_generic_and_domain_certification(&generic, &domain)
            .unwrap();
    let error = SubscriptionSupportAccuracyCertificationRunner::production()
        .certify(
            &evidence_bundle,
            &generic,
            &domain,
            &handoff,
            &phase7_lane_evidence(),
        )
        .expect_err(
            "runner closeout must reject generic performance counters without physical debt",
        );

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustCoverageMissing
    );
}

#[test]
fn phase7_runner_rejects_domain_counter_regime_drift() {
    let evidence_bundle = first_ship_certification_bundle();
    let generic = generic_support_certification_report();
    let drifted_scope = SupportCertificationBatchScope::new(
        SupportCertificationBatchScopeKind::DomainScenarioLocal,
        SupportTrustDensityClass::DomainScenarioLocal,
        SupportTrustPathClass::DomainCertificationPath,
        SupportTrustAllocationScope::DomainCertification,
        5,
        6,
        4,
        1,
    )
    .unwrap();
    let drifted_domain_plan =
        SupportDomainCertificationBatchPlan::new(5, 5, drifted_scope, 5).unwrap();
    let domain = SupportDomainCertificationBundle::new(
        first_ship_domain_rows(&generic),
        drifted_domain_plan,
        SupportDomainCertificationCounterSnapshot::new(5, 3, 2, 6, 4, 1, 2),
    )
    .unwrap();
    let handoff =
        SupportCertificationHandoffReport::from_generic_and_domain_certification(&generic, &domain)
            .unwrap();
    let error = SubscriptionSupportAccuracyCertificationRunner::production()
        .certify(
            &evidence_bundle,
            &generic,
            &domain,
            &handoff,
            &phase7_lane_evidence(),
        )
        .expect_err(
            "runner closeout must reject a valid domain bundle whose exact counter regime drifted",
        );

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustCoverageMissing
    );
}
