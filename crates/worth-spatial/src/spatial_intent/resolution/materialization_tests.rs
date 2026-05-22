use forge_foundational::facade::{
    AdmissionReadinessProfile, CertificationPostureProfile, CompatibilityPostureProfile,
    DiagnosticRichnessProfile, FoundationalBoundaryEvidenceFreshnessPosture,
    FoundationalDiagnosticOutcomeKind, FoundationalProfileNarrowingKind,
    FoundationalProfileNarrowingRecord, FoundationalProfileSet, FoundationalProfileSetInput,
    RetentionDeliveryProfile, SupportPostureProfile,
};

use super::{
    materialize_spatial_direction_witness_support_report,
    materialize_spatial_point_witness_support_report, SpatialWitnessMaterializationDenial,
    SpatialWitnessMaterializationProfilePlan,
};
use crate::facade::{
    resolve_spatial_direction_witness, resolve_spatial_point_witness_with_catalog,
    SpatialCarrierKind, SpatialCatalogParameterAdmission, SpatialCatalogResolvedPointWitness,
    SpatialCatalogTrimmedAdmissionPosture, SpatialCatalogWitnessResolutionClass,
    SpatialDirectionWitnessRef, SpatialPointWitnessRef,
};
use crate::test_support::SpatialFixtureWitnessCatalog;
use worth_geom::{ParameterDomain, ParameterSpacePoint};

#[test]
fn direct_world_point_materializes_as_decision_not_failure() {
    let requested = SpatialPointWitnessRef::world_point([1.0, 2.0, 3.0]);
    let outcome = crate::facade::resolve_spatial_point_witness(requested.clone());
    let materialized = materialize_spatial_point_witness_support_report(
        requested,
        outcome,
        standard_profile_plan(),
    )
    .expect("materialized point support");

    assert_eq!(materialized.support_report().decision_rows().count(), 1);
    assert_eq!(materialized.support_report().failure_rows().count(), 0);
    assert_eq!(
        materialized.provenance().freshness_posture(),
        FoundationalBoundaryEvidenceFreshnessPosture::FreshRetained
    );
}

#[test]
fn ambiguous_point_denial_materializes_as_decision_row() {
    let requested = SpatialPointWitnessRef::ambiguous_curve_point("curve-1");
    let outcome = crate::facade::resolve_spatial_point_witness(requested.clone());
    let materialized = materialize_spatial_point_witness_support_report(
        requested,
        outcome,
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
        FoundationalDiagnosticOutcomeKind::Denied
    );
    assert_eq!(materialized.support_report().failure_rows().count(), 0);
}

#[test]
fn fallback_direction_materializes_support_and_reduced_freshness() {
    let requested = SpatialDirectionWitnessRef::frame_perpendicular_axis(
        crate::facade::SpatialFrameRef::workplane("wp-1", [0.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
        crate::facade::SpatialAxis::W,
    );
    let outcome = resolve_spatial_direction_witness(requested.clone());
    let materialized = materialize_spatial_direction_witness_support_report(
        requested,
        outcome,
        standard_profile_plan(),
    )
    .expect("materialized fallback direction");

    assert_eq!(materialized.support_report().support_rows().count(), 1);
    assert_eq!(
        materialized.provenance().freshness_posture(),
        FoundationalBoundaryEvidenceFreshnessPosture::ReducedRetained
    );
}

#[test]
fn parameter_admission_posture_survives_into_provenance_support_context() {
    let requested_parameter = ParameterSpacePoint::try_new([0.5, 0.25]).unwrap();
    let admission = SpatialCatalogParameterAdmission::new(
        requested_parameter,
        ParameterDomain::plane().admit(requested_parameter).unwrap(),
        ParameterDomain::plane()
            .canonicalize(requested_parameter)
            .unwrap(),
    )
    .with_trimmed_posture(SpatialCatalogTrimmedAdmissionPosture::PolygonalRegion);
    let catalog = SpatialFixtureWitnessCatalog::new().with_parameter_space_point(
        SpatialCarrierKind::Surface,
        "surface-1",
        requested_parameter,
        Ok(
            SpatialCatalogResolvedPointWitness::with_parameter_admission(
                [8.0, 9.0, 10.0],
                SpatialCatalogWitnessResolutionClass::CarrierDerived,
                admission,
            ),
        ),
    );
    let requested =
        SpatialPointWitnessRef::surface_parameter_point("surface-1", requested_parameter);
    let outcome = resolve_spatial_point_witness_with_catalog(requested.clone(), &catalog);
    let materialized = materialize_spatial_point_witness_support_report(
        requested,
        outcome,
        standard_profile_plan(),
    )
    .expect("materialized parameter-space point");

    let support_context: Vec<_> = materialized
        .provenance()
        .support_context_attachments()
        .iter()
        .map(|attachment| format!("{attachment:?}"))
        .collect();
    assert!(support_context
        .iter()
        .any(|value| value.contains("parameter.requested")));
    assert!(support_context
        .iter()
        .any(|value| value.contains("parameter.domain_admitted")));
    assert!(support_context
        .iter()
        .any(|value| value.contains("parameter.canonicalized")));
    assert!(support_context
        .iter()
        .any(|value| value.contains("parameter.trimmed_polygonal")));
}

#[test]
fn profile_strengthening_is_denied() {
    let stronger = profile(
        DiagnosticRichnessProfile::Forensic,
        SupportPostureProfile::SupportReady,
    );
    let weaker = profile(
        DiagnosticRichnessProfile::Standard,
        SupportPostureProfile::SupportReady,
    );
    let requested = SpatialPointWitnessRef::world_point([1.0, 2.0, 3.0]);
    let outcome = crate::facade::resolve_spatial_point_witness(requested.clone());
    let denial = materialize_spatial_point_witness_support_report(
        requested,
        outcome,
        SpatialWitnessMaterializationProfilePlan::new(weaker, stronger, stronger, None, None),
    )
    .expect_err("illegal strengthening should deny");

    assert!(matches!(
        denial,
        SpatialWitnessMaterializationDenial::ProfileProgression(_)
    ));
}

fn standard_profile_plan() -> SpatialWitnessMaterializationProfilePlan {
    let requested = profile(
        DiagnosticRichnessProfile::Forensic,
        SupportPostureProfile::SupportReady,
    );
    let admitted = profile(
        DiagnosticRichnessProfile::Standard,
        SupportPostureProfile::SupportReady,
    );
    let materialized = admitted;
    SpatialWitnessMaterializationProfilePlan::new(
        requested,
        admitted,
        materialized,
        Some(FoundationalProfileNarrowingRecord::new(
            FoundationalProfileNarrowingKind::RichnessReduced,
            "witness support reports default to standard richness",
        )),
        None,
    )
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
    .unwrap()
}
