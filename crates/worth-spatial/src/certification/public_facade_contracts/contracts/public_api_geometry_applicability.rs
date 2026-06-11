use worth_spatial::facade::support::{
    geometry_applicability_matrix, GeometryApplicabilityStatus, GeometryPublicSurface,
    GeometryRuntimeConcern,
};

#[test]
fn spatial_public_geometry_applicability_matrix_covers_every_surface_and_runtime_concern() {
    let matrix = geometry_applicability_matrix();
    let surfaces = GeometryPublicSurface::all();
    let concerns = GeometryRuntimeConcern::all();

    assert_eq!(matrix.rows().len(), surfaces.len() * concerns.len());
    assert!(!matrix.matrix_digest().is_empty());

    for surface in surfaces {
        for concern in concerns {
            let row = matrix.row(surface, concern).expect(
                "every admitted geometry surface must classify every major runtime concern",
            );
            assert_eq!(row.surface(), surface);
            assert_eq!(row.concern(), concern);
            assert!(!row.rationale().is_empty());
            assert!(!row.row_digest().is_empty());
        }
    }
}

#[test]
fn spatial_public_geometry_applicability_matrix_keeps_spec_examples_honest() {
    let matrix = geometry_applicability_matrix();

    assert_eq!(
        matrix
            .row(
                GeometryPublicSurface::GeometryTargetIdentity,
                GeometryRuntimeConcern::GroupedNeighborhoodWorkflow,
            )
            .expect("target identity grouped row should exist")
            .status(),
        GeometryApplicabilityStatus::NotApplicable
    );
    assert_eq!(
        matrix
            .row(
                GeometryPublicSurface::GeometryTargetIdentity,
                GeometryRuntimeConcern::MutationEvidence,
            )
            .expect("target identity mutation row should exist")
            .status(),
        GeometryApplicabilityStatus::NotApplicable
    );
    assert_eq!(
        matrix
            .row(
                GeometryPublicSurface::GeometryTargetIdentity,
                GeometryRuntimeConcern::HistoricalInspection,
            )
            .expect("target identity historical row should exist")
            .status(),
        GeometryApplicabilityStatus::RequiredNow
    );

    assert_eq!(
        matrix
            .row(
                GeometryPublicSurface::PrimitiveRebinding,
                GeometryRuntimeConcern::GroupedNeighborhoodWorkflow,
            )
            .expect("rebinding grouped row should exist")
            .status(),
        GeometryApplicabilityStatus::RequiredNow
    );
    assert_eq!(
        matrix
            .row(
                GeometryPublicSurface::PrimitiveRebinding,
                GeometryRuntimeConcern::ReplayParity,
            )
            .expect("rebinding replay row should exist")
            .status(),
        GeometryApplicabilityStatus::RequiredNow
    );
    assert_eq!(
        matrix
            .row(
                GeometryPublicSurface::PrimitiveRebinding,
                GeometryRuntimeConcern::SignalContinuation,
            )
            .expect("rebinding signal row should exist")
            .status(),
        GeometryApplicabilityStatus::RequiredNow
    );

    assert_eq!(
        matrix
            .row(
                GeometryPublicSurface::GeometryRecoveryAction,
                GeometryRuntimeConcern::MutationEvidence,
            )
            .expect("recovery mutation row should exist")
            .status(),
        GeometryApplicabilityStatus::RequiredNow
    );
    assert_eq!(
        matrix
            .row(
                GeometryPublicSurface::GeometryRecoveryAction,
                GeometryRuntimeConcern::ReplayParity,
            )
            .expect("recovery replay row should exist")
            .status(),
        GeometryApplicabilityStatus::DeniedForThisRuntime
    );
}
