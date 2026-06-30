use crate::workload_platform::evidence_ledger::WorkloadEvidenceStage;
use crate::workload_platform::evidence_lookup_family_catalog::EvidenceLookupProjectionFactFamily;
use crate::workload_platform::evidence_lookup_inventory::EvidenceLookupQuerySurface;

use super::super::{
    current_evidence_lookup_query_surface_matrix, EvidenceLookupQuerySurfaceTouchpoint,
};

#[test]
fn every_query_touchpoint_has_exact_surface_category() {
    let matrix = current_evidence_lookup_query_surface_matrix().expect("matrix closes");

    for row in matrix.rows() {
        if row.query_support_required() {
            assert_ne!(row.query_surface(), EvidenceLookupQuerySurface::NotQuery);
            assert!(row.query_surface_type_name().is_some());
            assert!(row.proof_digest().is_some());
        } else {
            assert_eq!(row.query_surface(), EvidenceLookupQuerySurface::NotQuery);
            assert!(row.query_surface_type_name().is_none());
            assert!(row.proof_digest().is_none());
        }
        assert!(!row.claims_lookup_execution_authority());
        assert!(!row.claims_query_descriptor_authority());
    }
}

#[test]
fn projection_consuming_lookup_uses_typed_fact_receipts() {
    let matrix = current_evidence_lookup_query_surface_matrix().expect("matrix closes");
    let touchpoints = [
        EvidenceLookupQuerySurfaceTouchpoint::FamilyCatalogQueryPosture,
        EvidenceLookupQuerySurfaceTouchpoint::InputAdmissionQuerySupport,
        EvidenceLookupQuerySurfaceTouchpoint::PlanSelectionQueryPosture,
        EvidenceLookupQuerySurfaceTouchpoint::IndexProductQuerySupport,
        EvidenceLookupQuerySurfaceTouchpoint::ExecutionReceiptQuerySupport,
        EvidenceLookupQuerySurfaceTouchpoint::DiagnosticWitnessContract,
        EvidenceLookupQuerySurfaceTouchpoint::PublicCloseoutProof,
    ];

    for touchpoint in touchpoints {
        let row = matrix
            .require_family_stage_touchpoint_row(
                "spatial-touch.boolean.projection-consumption-evidence.v1",
                WorkloadEvidenceStage::BooleanOperandAProjectionConsumption,
                touchpoint,
            )
            .expect("projection row exists for every touchpoint");

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
}

#[test]
fn every_query_matrix_touchpoint_is_present_for_projection_lookup() {
    let matrix = current_evidence_lookup_query_surface_matrix().expect("matrix closes");

    for touchpoint in [
        EvidenceLookupQuerySurfaceTouchpoint::FamilyCatalogQueryPosture,
        EvidenceLookupQuerySurfaceTouchpoint::InputAdmissionQuerySupport,
        EvidenceLookupQuerySurfaceTouchpoint::PlanSelectionQueryPosture,
        EvidenceLookupQuerySurfaceTouchpoint::IndexProductQuerySupport,
        EvidenceLookupQuerySurfaceTouchpoint::ExecutionReceiptQuerySupport,
        EvidenceLookupQuerySurfaceTouchpoint::DiagnosticWitnessContract,
        EvidenceLookupQuerySurfaceTouchpoint::PublicCloseoutProof,
    ] {
        matrix
            .require_family_stage_touchpoint_row(
                "spatial-touch.boolean.projection-consumption-evidence.v1",
                WorkloadEvidenceStage::BooleanOperandAProjectionConsumption,
                touchpoint,
            )
            .expect("phase 9 requires matrix coverage for each lookup touchpoint");
    }
}

#[test]
fn lower_runtime_boundary_envelopes_are_not_synthesized() {
    let matrix = current_evidence_lookup_query_surface_matrix().expect("matrix closes");

    assert_eq!(matrix.counters().lower_runtime_boundary_row_count(), 0);
    assert!(matrix.rows().iter().all(|row| {
        row.query_surface() != EvidenceLookupQuerySurface::LowerRuntimeBoundaryEnvelope
            || row.query_surface_type_name()
                == Some(
                    "forge_query::facade::runtime::ForgeQueryLowerRuntimeBoundaryEnvelopeSource",
                )
    }));
}
