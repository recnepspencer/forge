use forge_foundational::facade::{
    AdmissionReadinessProfile, CertificationPostureProfile, CompatibilityPostureProfile,
    DiagnosticRichnessProfile, FoundationalBoundaryEvidenceFreshnessPosture,
    FoundationalBoundaryEvidenceSupportContextAttachment, FoundationalDiagnosticEvidencePosture,
    FoundationalProfileNarrowingKind, FoundationalProfileNarrowingRecord, FoundationalProfileSet,
    FoundationalProfileSetInput, RetentionDeliveryProfile, SupportPostureProfile,
};

use super::{
    materialize_lowered_spatial_intent_support_report,
    LoweredSpatialIntentMaterializationProfilePlan,
};
use crate::facade::{
    admit_spatial_move, lower_admitted_move_intent, SpatialAnchorRef, SpatialMoveSpec,
};

#[test]
fn accepted_lowered_move_materializes_as_decision_and_support() {
    let admitted = admit_spatial_move(
        SpatialMoveSpec::shape_origin()
            .from(SpatialAnchorRef::shape_origin())
            .to([1.0, 2.0, 3.0]),
    )
    .expect("admitted move");
    let lowered =
        lower_admitted_move_intent(crate::facade::SpatialPlacementSpec::world(), &admitted)
            .expect("lowered move");
    let materialized = materialize_lowered_spatial_intent_support_report(
        crate::facade::LoweredSpatialIntentFamily::Move,
        Ok(lowered.payload().clone()),
        standard_profile_plan(),
    )
    .expect("materialized support");

    assert_eq!(materialized.support_report().decision_rows().count(), 1);
    assert_eq!(materialized.support_report().support_rows().count(), 1);
    let support_row = materialized
        .support_report()
        .support_rows()
        .next()
        .expect("support row");
    assert_eq!(
        support_row.evidence_posture(),
        &forge_foundational::facade::FoundationalDiagnosticSupportEvidencePosture::Present(
            FoundationalDiagnosticEvidencePosture::RetainedDirect,
        )
    );
    let labels: Vec<_> = support_row
        .semantic_labels()
        .labels()
        .iter()
        .map(|label| label.as_str())
        .collect();
    assert!(labels.contains(&"worth.spatial.lowering.move"));
    assert!(labels.contains(&"worth.spatial.lowering.payload.move"));
    assert!(labels.contains(&"shape_origin_point"));
    assert_eq!(
        materialized.provenance().freshness_posture(),
        FoundationalBoundaryEvidenceFreshnessPosture::FreshRetained
    );
    let support_codes: Vec<_> = materialized
        .provenance()
        .support_context_attachments()
        .iter()
        .filter_map(|attachment| match attachment {
            FoundationalBoundaryEvidenceSupportContextAttachment::DiagnosticCode(code) => {
                Some(code.as_str())
            }
            _ => None,
        })
        .collect();
    assert!(support_codes.contains(&"direct"));
    assert!(support_codes.contains(&"point_witness"));
    assert!(support_codes.contains(&"worth.spatial.lowering.payload.move"));
}

#[test]
fn denied_lowering_materializes_as_denial_without_failure_row() {
    let materialized = materialize_lowered_spatial_intent_support_report(
        crate::facade::LoweredSpatialIntentFamily::PointsToward,
        Err(crate::facade::SpatialLoweringDenial::Coincident),
        standard_profile_plan(),
    )
    .expect("materialized denial");

    let row = materialized
        .support_report()
        .decision_rows()
        .next()
        .expect("decision row");
    assert_eq!(
        row.outcome_kind(),
        forge_foundational::facade::FoundationalDiagnosticOutcomeKind::Denied
    );
    let labels: Vec<_> = row
        .semantic_labels()
        .labels()
        .iter()
        .map(|label| label.as_str())
        .collect();
    assert!(labels.contains(&"worth.spatial.lowering.denial.coincident"));
    assert_eq!(materialized.support_report().failure_rows().count(), 0);
    let support_codes: Vec<_> = materialized
        .provenance()
        .support_context_attachments()
        .iter()
        .filter_map(|attachment| match attachment {
            FoundationalBoundaryEvidenceSupportContextAttachment::DiagnosticCode(code) => {
                Some(code.as_str())
            }
            _ => None,
        })
        .collect();
    assert!(support_codes.contains(&"worth.spatial.lowering.denial.coincident"));
}

fn standard_profile_plan() -> LoweredSpatialIntentMaterializationProfilePlan {
    let requested = profile(
        DiagnosticRichnessProfile::Forensic,
        SupportPostureProfile::SupportReady,
    );
    let admitted = profile(
        DiagnosticRichnessProfile::Standard,
        SupportPostureProfile::SupportReady,
    );
    LoweredSpatialIntentMaterializationProfilePlan {
        requested,
        admitted,
        materialized: admitted,
        requested_to_admitted_narrowing: Some(FoundationalProfileNarrowingRecord::new(
            FoundationalProfileNarrowingKind::RichnessReduced,
            "lowering reports default to standard richness",
        )),
        admitted_to_materialized_narrowing: None,
    }
}

fn profile(
    richness: DiagnosticRichnessProfile,
    support: SupportPostureProfile,
) -> FoundationalProfileSet {
    FoundationalProfileSet::new(FoundationalProfileSetInput {
        diagnostic_richness: richness,
        support_posture: support,
        compatibility_posture: CompatibilityPostureProfile::NativeOnly,
        admission_readiness: AdmissionReadinessProfile::Admitted,
        retention_delivery: RetentionDeliveryProfile::Retained,
        certification_posture: CertificationPostureProfile::Uncertified,
    })
    .expect("profile")
}
