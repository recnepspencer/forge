use super::super::{
    DegradedSupportTrustWitness, ExactSupportTrustWitness, OperationalSupportTrustClassified,
    OperationalSupportTrustReport, RebuildDerivedSupportTrustWitness, RejectedSupportTrustWitness,
    SubscriptionSupportCertificationCoveragePlan, SupportCertificationCoverageMatrix,
    SupportCertificationEpoch, SupportCertificationLaneDigestSet, SupportCertificationRow,
    SupportCertificationRowEvidence, SupportCertificationRowRequirement,
    SupportExactTrustTranslation, SupportOperationalLedgerEpoch, SupportTrustClass,
    SupportTrustClassificationCostSurface, SupportTrustFreshnessWitness, SupportTrustProvenance,
    SupportTrustStrength,
};
use super::operational_basis::{basis_for, epochs};
use crate::subscription_support::{
    SubscriptionResumeClassification, SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind,
    SubscriptionSupportOperationalVerdict, SubscriptionSupportRole,
};

pub(super) fn certification_lanes() -> SupportCertificationLaneDigestSet {
    SupportCertificationLaneDigestSet::new(
        "lane:control:exact",
        "lane:hostile:stale",
        "lane:replay:retained",
    )
    .unwrap()
}

pub(super) fn exact_certification_requirement(row_id: &str) -> SupportCertificationRowRequirement {
    SupportCertificationRowRequirement::new(
        row_id,
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        SupportTrustClass::ExactSupportTrusted,
        SupportTrustStrength::Exact,
        SupportTrustProvenance::NativePublished,
        SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        SubscriptionResumeClassification::Exact,
        None,
    )
    .unwrap()
}

pub(super) fn certification_requirement_for(
    row_id: &str,
    family_id: &str,
    family_kind: SubscriptionSupportFamilyKind,
    support_role: SubscriptionSupportRole,
    trust_class: SupportTrustClass,
    trust_strength: SupportTrustStrength,
    provenance: SupportTrustProvenance,
    verdict: SubscriptionSupportOperationalVerdict,
    classification: SubscriptionResumeClassification,
) -> SupportCertificationRowRequirement {
    SupportCertificationRowRequirement::new(
        row_id,
        SubscriptionSupportFamilyId::new(family_id).unwrap(),
        family_kind,
        support_role,
        trust_class,
        trust_strength,
        provenance,
        verdict,
        classification,
        None,
    )
    .unwrap()
}

pub(super) fn exact_certification_plan(
    row_id: &str,
) -> SubscriptionSupportCertificationCoveragePlan {
    SubscriptionSupportCertificationCoveragePlan::new(
        SupportOperationalLedgerEpoch::new(7).unwrap(),
        SupportCertificationEpoch::new(11).unwrap(),
        vec![exact_certification_requirement(row_id)],
    )
    .unwrap()
}

pub(super) fn exact_certification_row(
    row_id: &str,
    classified: &OperationalSupportTrustClassified,
) -> SupportCertificationRow {
    let evidence = SupportCertificationRowEvidence::from_operational_report(
        row_id,
        classified.report(),
        SubscriptionResumeClassification::Exact,
        SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        SupportOperationalLedgerEpoch::new(7).unwrap(),
        SupportCertificationEpoch::new(11).unwrap(),
        certification_lanes(),
        "artifact:digest:exact",
        "subscription-support:digest:exact",
        "diagnostics:digest:exact",
        None,
        Vec::new(),
    )
    .unwrap();
    SupportCertificationRow::new(evidence).unwrap()
}

pub(super) fn report_for_certification_row(
    family_id: &str,
    family_kind: SubscriptionSupportFamilyKind,
    support_role: SubscriptionSupportRole,
    artifact_id: &str,
    trust_strength: SupportTrustStrength,
) -> OperationalSupportTrustReport {
    let basis = basis_for(family_id, family_kind, support_role, artifact_id);
    let freshness = SupportTrustFreshnessWitness::new(epochs());
    match trust_strength {
        SupportTrustStrength::Exact => {
            let translation = SupportExactTrustTranslation::new(
                basis,
                SubscriptionResumeClassification::Exact,
                SubscriptionSupportOperationalVerdict::ExactResumePreserved,
            )
            .unwrap();
            let witness = ExactSupportTrustWitness::from_exact_translation(
                translation,
                SupportTrustProvenance::NativePublished,
                freshness,
            )
            .unwrap();
            OperationalSupportTrustReport::from_exact_witness(witness)
        }
        SupportTrustStrength::Degraded => {
            let witness = DegradedSupportTrustWitness::new(basis, freshness);
            OperationalSupportTrustReport::from_degraded_witness(
                witness,
                SupportTrustProvenance::NativePublished,
                SupportTrustClassificationCostSurface::phase1_zero(),
            )
        }
        SupportTrustStrength::RebuildOnly => {
            let witness = RebuildDerivedSupportTrustWitness::new(basis, freshness);
            OperationalSupportTrustReport::from_rebuild_witness(
                witness,
                SupportTrustProvenance::Rebuilt,
                SupportTrustClassificationCostSurface::phase1_zero(),
            )
        }
        SupportTrustStrength::Rejected | SupportTrustStrength::Unsupported => {
            let witness = RejectedSupportTrustWitness::new(basis, freshness);
            OperationalSupportTrustReport::from_rejected_witness(
                witness,
                SupportTrustProvenance::Omitted,
                SupportTrustClassificationCostSurface::phase1_zero(),
            )
        }
    }
}

pub(super) fn certification_row_from_report(
    row_id: &str,
    report: &OperationalSupportTrustReport,
    classification: SubscriptionResumeClassification,
    verdict: SubscriptionSupportOperationalVerdict,
) -> SupportCertificationRow {
    let evidence = SupportCertificationRowEvidence::from_operational_report(
        row_id,
        report,
        classification,
        verdict,
        SupportOperationalLedgerEpoch::new(7).unwrap(),
        SupportCertificationEpoch::new(11).unwrap(),
        certification_lanes(),
        format!("artifact:digest:{row_id}"),
        format!("subscription-support:digest:{row_id}"),
        format!("diagnostics:digest:{row_id}"),
        None,
        Vec::new(),
    )
    .unwrap();
    SupportCertificationRow::new(evidence).unwrap()
}

pub(super) fn first_ship_certification_matrix() -> SupportCertificationCoverageMatrix {
    first_ship_certification_matrix_for_basis_artifact("artifact:trust:phase-1")
}

pub(super) fn first_ship_certification_matrix_for_basis_artifact(
    basis_bound_artifact_id: &str,
) -> SupportCertificationCoverageMatrix {
    first_ship_certification_matrix_for_basis_artifact_and_materialized_family(
        basis_bound_artifact_id,
        "materialized-narrowing-support",
    )
}

pub(super) fn first_ship_certification_matrix_for_basis_artifact_and_materialized_family(
    basis_bound_artifact_id: &str,
    materialized_family_id: &str,
) -> SupportCertificationCoverageMatrix {
    let requirements = vec![
        certification_requirement_for(
            "row:basis-bound-exact",
            "basis-bound-continuation-support",
            SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
            SubscriptionSupportRole::ExactContinuation,
            SupportTrustClass::ExactSupportTrusted,
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
            SubscriptionResumeClassification::Exact,
        ),
        certification_requirement_for(
            "row:materialized-narrowing-exact",
            materialized_family_id,
            SubscriptionSupportFamilyKind::MaterializedNarrowingSupport,
            SubscriptionSupportRole::NarrowingMaterialization,
            SupportTrustClass::ExactSupportTrusted,
            SupportTrustStrength::Exact,
            SupportTrustProvenance::NativePublished,
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
            SubscriptionResumeClassification::Exact,
        ),
        certification_requirement_for(
            "row:degraded-continuation",
            "degraded-continuation-support",
            SubscriptionSupportFamilyKind::DegradedContinuationSupport,
            SubscriptionSupportRole::DegradedContinuation,
            SupportTrustClass::DegradedSupportTrusted,
            SupportTrustStrength::Degraded,
            SupportTrustProvenance::NativePublished,
            SubscriptionSupportOperationalVerdict::DegradedResumePreserved,
            SubscriptionResumeClassification::Degraded,
        ),
        certification_requirement_for(
            "row:extension-defined-rejected",
            "extension-defined-support",
            SubscriptionSupportFamilyKind::ExtensionDefinedSupport,
            SubscriptionSupportRole::ExactContinuation,
            SupportTrustClass::StaleSupportRejected,
            SupportTrustStrength::Rejected,
            SupportTrustProvenance::Omitted,
            SubscriptionSupportOperationalVerdict::RejectedByPolicy,
            SubscriptionResumeClassification::NotResumable,
        ),
    ];
    let plan = SubscriptionSupportCertificationCoveragePlan::new(
        SupportOperationalLedgerEpoch::new(7).unwrap(),
        SupportCertificationEpoch::new(11).unwrap(),
        requirements,
    )
    .unwrap();
    let basis_bound = report_for_certification_row(
        "basis-bound-continuation-support",
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        basis_bound_artifact_id,
        SupportTrustStrength::Exact,
    );
    let materialized = report_for_certification_row(
        materialized_family_id,
        SubscriptionSupportFamilyKind::MaterializedNarrowingSupport,
        SubscriptionSupportRole::NarrowingMaterialization,
        "artifact:first-ship:materialized",
        SupportTrustStrength::Exact,
    );
    let degraded = report_for_certification_row(
        "degraded-continuation-support",
        SubscriptionSupportFamilyKind::DegradedContinuationSupport,
        SubscriptionSupportRole::DegradedContinuation,
        "artifact:first-ship:degraded",
        SupportTrustStrength::Degraded,
    );
    let extension = report_for_certification_row(
        "extension-defined-support",
        SubscriptionSupportFamilyKind::ExtensionDefinedSupport,
        SubscriptionSupportRole::ExactContinuation,
        "artifact:first-ship:extension",
        SupportTrustStrength::Rejected,
    );
    SupportCertificationCoverageMatrix::from_rows(
        &plan,
        vec![
            certification_row_from_report(
                "row:basis-bound-exact",
                &basis_bound,
                SubscriptionResumeClassification::Exact,
                SubscriptionSupportOperationalVerdict::ExactResumePreserved,
            ),
            certification_row_from_report(
                "row:materialized-narrowing-exact",
                &materialized,
                SubscriptionResumeClassification::Exact,
                SubscriptionSupportOperationalVerdict::ExactResumePreserved,
            ),
            certification_row_from_report(
                "row:degraded-continuation",
                &degraded,
                SubscriptionResumeClassification::Degraded,
                SubscriptionSupportOperationalVerdict::DegradedResumePreserved,
            ),
            certification_row_from_report(
                "row:extension-defined-rejected",
                &extension,
                SubscriptionResumeClassification::NotResumable,
                SubscriptionSupportOperationalVerdict::RejectedByPolicy,
            ),
        ],
    )
    .unwrap()
}
