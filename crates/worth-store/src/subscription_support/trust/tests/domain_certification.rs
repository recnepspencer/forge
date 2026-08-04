use super::super::{
    SupportCertificationHandoffReport, SupportDomainCertificationBundle,
    SupportDomainCertificationCounterSnapshot, SupportDomainCertificationDebtOwner,
    SupportDomainCertificationDebtReason, SupportDomainCertificationRow,
    SupportDomainCertificationRowStatus, SupportDomainCertificationScenario,
    SupportRoadmapPhysicalReadinessPosture, SupportTrustFailureKind, SupportTrustProvenance,
    SupportTrustStrength, SupportTrustUseBoundary,
};
use super::certification_bundle::{
    certified_first_ship_support_trust_for, generic_support_certification_report,
    generic_support_certification_report_for,
};
use super::domain_handoff::{first_ship_domain_batch_plan, first_ship_domain_rows};
use super::operational_basis::basis_for;
use crate::subscription_support::{
    SubscriptionResumeClassification, SubscriptionSupportFamilyKind,
    SubscriptionSupportOperationalVerdict, SubscriptionSupportRole,
};

#[test]
fn phase6_generic_certification_consumes_certified_support_trust() {
    let generic = generic_support_certification_report();

    assert_eq!(
        generic.certified_report().use_boundary(),
        SupportTrustUseBoundary::CertifiedPlatform
    );
    assert_eq!(
        generic.counter_snapshot().certified_support_report_count(),
        1
    );
    assert!(!generic.generic_certification_digest().is_empty());
    assert_eq!(generic.coverage_summary().row_count(), 4);
}

#[test]
fn phase6_domain_certification_emits_scenarios_and_explicit_physical_debt() {
    let generic = generic_support_certification_report();
    let bundle = SupportDomainCertificationBundle::new(
        first_ship_domain_rows(&generic),
        first_ship_domain_batch_plan(),
        SupportDomainCertificationCounterSnapshot::new(5, 3, 2, 5, 4, 1, 2),
    )
    .unwrap();
    let handoff =
        SupportCertificationHandoffReport::from_generic_and_domain_certification(&generic, &bundle)
            .unwrap();

    assert_eq!(bundle.rows().len(), 5);
    assert_eq!(bundle.counter_snapshot().physical_readiness_debt_count(), 2);
    assert_eq!(
        bundle
            .rows()
            .iter()
            .filter(|row| row.row_status()
                == SupportDomainCertificationRowStatus::CertifiedSemanticSupport)
            .count(),
        3
    );
    let chip_debt = bundle
        .rows()
        .iter()
        .find(|row| {
            row.scenario() == SupportDomainCertificationScenario::ChipSimulationLongHistoryRebuild
        })
        .expect("chip simulation scenario row must be present");
    assert_eq!(
        chip_debt.debt_reason(),
        Some(SupportDomainCertificationDebtReason::RebuildEquivalenceLaneDeferred)
    );
    assert_eq!(
        chip_debt.required_future_milestone(),
        Some(SupportDomainCertificationDebtOwner::Roadmap2PhysicalDatabaseFoundation)
    );
    let offline_debt = bundle
        .rows()
        .iter()
        .find(|row| {
            row.scenario()
                == SupportDomainCertificationScenario::OfflineCollaborativeCapsuleOmission
        })
        .expect("offline capsule scenario row must be present");
    assert_eq!(
        offline_debt.debt_reason(),
        Some(SupportDomainCertificationDebtReason::OmittedSupportImportLaneDeferred)
    );
    assert_eq!(
        offline_debt.required_future_milestone(),
        Some(SupportDomainCertificationDebtOwner::Milestone15ExtensionSupportRegistration)
    );
    assert!(handoff.semantic_support_trust_closed());
    assert_eq!(
        handoff.roadmap_physical_readiness_posture(),
        SupportRoadmapPhysicalReadinessPosture::PhysicalDatabaseReadinessDeferredToRoadmap2
    );
    assert!(!handoff.handoff_digest().is_empty());
}

#[test]
fn phase6_domain_plan_rejects_counter_width_drift() {
    let generic = generic_support_certification_report();
    let error = SupportDomainCertificationBundle::new(
        first_ship_domain_rows(&generic),
        first_ship_domain_batch_plan(),
        SupportDomainCertificationCounterSnapshot::new(5, 3, 1, 5, 4, 1, 1),
    )
    .expect_err("explicit debt row count must match domain scenario rows");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustCoverageMissing
    );
}

#[test]
fn phase6_domain_certified_row_rejects_scenario_family_drift() {
    let generic = generic_support_certification_report();
    let error = SupportDomainCertificationRow::certified_from_generic_report(
        SupportDomainCertificationScenario::WebDataRestartReplication,
        &generic,
    )
    .expect_err("basis-bound exact support cannot certify materialized narrowing scenario");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustRoleMismatch
    );
}

#[test]
fn phase6_degraded_domain_certification_cannot_satisfy_exact_scenario() {
    let degraded = generic_support_certification_report_for(
        "generic:subscription-support-trust:degraded-continuation",
        certified_first_ship_support_trust_for(
            basis_for(
                "degraded-continuation-support",
                SubscriptionSupportFamilyKind::DegradedContinuationSupport,
                SubscriptionSupportRole::DegradedContinuation,
                "artifact:first-ship:degraded",
            ),
            SupportTrustStrength::Degraded,
            SupportTrustProvenance::NativePublished,
            SubscriptionResumeClassification::Degraded,
            SubscriptionSupportOperationalVerdict::DegradedResumePreserved,
            "row:degraded-continuation",
        ),
    );

    let error = SupportDomainCertificationRow::certified_from_generic_report(
        SupportDomainCertificationScenario::GeometryCadSessionContinuation,
        &degraded,
    )
    .expect_err("degraded support cannot certify an exact continuation scenario");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustRoleMismatch
    );
}

#[test]
fn phase6_domain_rows_reject_first_ship_scenarios_as_advanced_family_debt() {
    let generic = generic_support_certification_report();
    let error = SupportDomainCertificationRow::explicit_advanced_family_debt(
        SupportDomainCertificationScenario::WebDataRestartReplication,
        &generic,
    )
    .expect_err("web/data first-ship scenario must be certified, not debt");

    assert_eq!(
        error.kind(),
        SupportTrustFailureKind::SupportTrustForbiddenExactOverclaim
    );
}

#[test]
fn phase6_handoff_keeps_physical_readiness_debt_explicit() {
    let generic = generic_support_certification_report();
    let bundle = SupportDomainCertificationBundle::new(
        first_ship_domain_rows(&generic),
        first_ship_domain_batch_plan(),
        SupportDomainCertificationCounterSnapshot::new(5, 3, 2, 5, 4, 1, 2),
    )
    .unwrap();
    let honest =
        SupportCertificationHandoffReport::from_generic_and_domain_certification(&generic, &bundle)
            .unwrap();

    assert_eq!(
        honest.roadmap_physical_readiness_posture(),
        SupportRoadmapPhysicalReadinessPosture::PhysicalDatabaseReadinessDeferredToRoadmap2
    );
}
