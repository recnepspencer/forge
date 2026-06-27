use worth_spatial::facade::evidence_lookup_family_catalog::EvidenceLookupProjectionFactFamily;
use worth_spatial::facade::evidence_lookup_inventory::EvidenceLookupQuerySurface;
use worth_spatial::facade::evidence_lookup_query_surface_matrix::{
    current_evidence_lookup_query_surface_matrix, EvidenceLookupQuerySurfaceTouchpoint,
};
use worth_spatial::facade::workload_vocabulary::WorkloadEvidenceStage;

#[test]
fn spatial_public_api_exports_read_only_query_surface_matrix() {
    let matrix = current_evidence_lookup_query_surface_matrix().expect("matrix closes");

    assert!(!matrix.rows().is_empty());
    assert_eq!(matrix.counters().row_count(), matrix.rows().len());
    assert!(!matrix.matrix_digest().is_empty());
    assert!(!matrix.claims_lookup_execution_authority());
    assert!(!matrix.claims_query_descriptor_authority());
}

#[test]
fn spatial_public_api_exposes_projection_consumption_surface_exactly() {
    let matrix = current_evidence_lookup_query_surface_matrix().expect("matrix closes");
    let row = matrix
        .require_family_stage_touchpoint_row(
            "spatial-touch.boolean.projection-consumption-evidence.v1",
            WorkloadEvidenceStage::BooleanOperandAProjectionConsumption,
            EvidenceLookupQuerySurfaceTouchpoint::PublicCloseoutProof,
        )
        .expect("projection closeout row exists");

    assert_eq!(
        row.query_surface(),
        EvidenceLookupQuerySurface::ProjectionConsumption
    );
    assert_eq!(
        row.query_surface_type_name(),
        Some("forge_query::facade::ProjectionConsumptionReceipt")
    );
    assert_eq!(
        row.projection_fact_family(),
        Some(EvidenceLookupProjectionFactFamily::SpatialTouchOperandProjection)
    );
    assert!(row.proof_digest().is_some());
}
