use super::super::{
    SupportCertificationBatchScope, SupportCertificationBatchScopeKind,
    SupportCertificationEvidenceBundle, SupportCertificationHandoffReport,
    SupportDomainCertificationBatchPlan, SupportDomainCertificationBundle,
    SupportDomainCertificationCounterSnapshot, SupportDomainCertificationRow,
    SupportDomainCertificationScenario, SupportGenericCertificationReport,
    SupportTrustAllocationScope, SupportTrustDensityClass, SupportTrustPathClass,
    SupportTrustProvenance, SupportTrustStrength,
};
use super::certification_bundle::{
    certified_first_ship_support_trust_for, first_ship_certification_bundle,
    generic_support_certification_report, generic_support_certification_report_for,
};
use super::operational_basis::basis_for;
use crate::subscription_support::{
    SubscriptionResumeClassification, SubscriptionSupportFamilyKind,
    SubscriptionSupportOperationalVerdict, SubscriptionSupportRole,
};

pub(super) fn first_ship_domain_batch_scope() -> SupportCertificationBatchScope {
    SupportCertificationBatchScope::new(
        SupportCertificationBatchScopeKind::DomainScenarioLocal,
        SupportTrustDensityClass::DomainScenarioLocal,
        SupportTrustPathClass::DomainCertificationPath,
        SupportTrustAllocationScope::DomainCertification,
        5,
        5,
        4,
        1,
    )
    .unwrap()
}

pub(super) fn first_ship_domain_batch_plan() -> SupportDomainCertificationBatchPlan {
    SupportDomainCertificationBatchPlan::new(5, 5, first_ship_domain_batch_scope(), 5).unwrap()
}

pub(super) fn first_ship_domain_rows(
    generic: &SupportGenericCertificationReport,
) -> Vec<SupportDomainCertificationRow> {
    let materialized = generic_support_certification_report_for(
        "generic:subscription-support-trust:materialized-narrowing",
        certified_first_ship_support_trust_for(
            basis_for(
                "materialized-narrowing-support",
                SubscriptionSupportFamilyKind::MaterializedNarrowingSupport,
                SubscriptionSupportRole::NarrowingMaterialization,
                "artifact:first-ship:materialized",
            ),
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
            SubscriptionResumeClassification::Exact,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
            "row:materialized-narrowing-exact",
        ),
    );
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
    vec![
        SupportDomainCertificationRow::certified_from_generic_report(
            SupportDomainCertificationScenario::GeometryCadSessionContinuation,
            generic,
        )
        .unwrap(),
        SupportDomainCertificationRow::certified_from_generic_report(
            SupportDomainCertificationScenario::WebDataRestartReplication,
            &materialized,
        )
        .unwrap(),
        SupportDomainCertificationRow::certified_from_generic_report(
            SupportDomainCertificationScenario::AiBranchWorkspaceDegradedContinuation,
            &degraded,
        )
        .unwrap(),
        SupportDomainCertificationRow::explicit_advanced_family_debt(
            SupportDomainCertificationScenario::ChipSimulationLongHistoryRebuild,
            generic,
        )
        .unwrap(),
        SupportDomainCertificationRow::explicit_advanced_family_debt(
            SupportDomainCertificationScenario::OfflineCollaborativeCapsuleOmission,
            generic,
        )
        .unwrap(),
    ]
}

pub(super) fn phase7_suite_artifacts() -> (
    SupportCertificationEvidenceBundle,
    SupportGenericCertificationReport,
    SupportDomainCertificationBundle,
    SupportCertificationHandoffReport,
) {
    let evidence_bundle = first_ship_certification_bundle();
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
    (evidence_bundle, generic, domain, handoff)
}
