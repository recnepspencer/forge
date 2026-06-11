use std::collections::HashSet;

use super::basis::{CertifiedLoopWindingSummary, ProjectedLoopVertexSnapshot};
use super::{
    CertifiedPolygonWinding2DBasis, CertifiedPolygonWinding2DDenial,
    CertifiedPolygonWinding2DDenialKind,
};

pub(crate) fn validate_polygon_winding_basis(
    basis: &CertifiedPolygonWinding2DBasis,
) -> Result<(), CertifiedPolygonWinding2DDenial> {
    if basis.primary_loop_identity().is_empty() {
        return Err(denial(
            CertifiedPolygonWinding2DDenialKind::MissingPrimaryLoopIdentity,
            "certified polygon winding requires a primary loop identity",
        ));
    }
    if basis.planar_neighborhood_identity().is_empty() {
        return Err(denial(
            CertifiedPolygonWinding2DDenialKind::MissingPlanarNeighborhood,
            "certified polygon winding requires a planar neighborhood identity",
        ));
    }
    if basis.winding_policy_identity().is_empty() {
        return Err(denial(
            CertifiedPolygonWinding2DDenialKind::MissingWindingPolicy,
            "certified polygon winding requires an explicit winding policy",
        ));
    }
    for loop_summary in basis.loop_summaries() {
        validate_loop_summary(loop_summary)?;
    }
    validate_shared_projection_basis(basis)?;
    Ok(())
}

fn validate_loop_summary(
    loop_summary: &CertifiedLoopWindingSummary,
) -> Result<(), CertifiedPolygonWinding2DDenial> {
    if loop_summary.loop_identity().is_empty()
        || loop_summary.topology_loop_identity().is_empty()
        || loop_summary.loop_membership_fact_digest().is_empty()
        || loop_summary
            .topology_to_spatial_contract_digest()
            .is_empty()
    {
        return Err(denial(
            CertifiedPolygonWinding2DDenialKind::MissingTopologyLoopBasis,
            "certified winding requires explicit topology loop basis rows",
        ));
    }
    if loop_summary.vertices().len() < 3 {
        return Err(denial(
            CertifiedPolygonWinding2DDenialKind::TooFewVertices,
            "certified winding requires at least three projected vertices per loop",
        ));
    }
    let mut seen = HashSet::new();
    for vertex in loop_summary.vertices() {
        if vertex.projection_fact_digest.is_empty() {
            return Err(denial(
                CertifiedPolygonWinding2DDenialKind::MissingProjectedVertexReceipt,
                "certified winding requires projected vertex receipts",
            ));
        }
        let key = (vertex.point_2d[0].to_bits(), vertex.point_2d[1].to_bits());
        if !seen.insert(key) {
            return Err(denial(
                CertifiedPolygonWinding2DDenialKind::DuplicateVertex,
                "duplicate projected loop vertices are ambiguous before loop cleanup",
            ));
        }
    }
    Ok(())
}

fn validate_shared_projection_basis(
    basis: &CertifiedPolygonWinding2DBasis,
) -> Result<(), CertifiedPolygonWinding2DDenial> {
    let first = basis.first_vertex().ok_or_else(|| {
        denial(
            CertifiedPolygonWinding2DDenialKind::MissingProjectedVertexReceipt,
            "certified winding requires projected vertex receipts",
        )
    })?;
    for vertex in basis.vertices().iter().skip(1) {
        validate_vertex_basis(first, vertex)?;
    }
    Ok(())
}

fn validate_vertex_basis(
    first: &ProjectedLoopVertexSnapshot,
    vertex: &ProjectedLoopVertexSnapshot,
) -> Result<(), CertifiedPolygonWinding2DDenial> {
    if vertex.movement_rotation_posture_identity != first.movement_rotation_posture_identity {
        return Err(denial(
            CertifiedPolygonWinding2DDenialKind::MovementRotationMismatch,
            "all projected loop vertices must share movement and rotation posture",
        ));
    }
    if vertex.local_frame_fact_digest != first.local_frame_fact_digest
        || vertex.local_frame_declaration_digest != first.local_frame_declaration_digest
        || vertex.local_frame_envelope_digest != first.local_frame_envelope_digest
        || vertex.frame_identity != first.frame_identity
        || vertex.transform_chain_digest != first.transform_chain_digest
    {
        return Err(denial(
            CertifiedPolygonWinding2DDenialKind::FrameBasisMismatch,
            "all projected loop vertices must share one certified local-frame basis",
        ));
    }
    if vertex.tolerance_policy_identity != first.tolerance_policy_identity {
        return Err(denial(
            CertifiedPolygonWinding2DDenialKind::TolerancePolicyMismatch,
            "all projected loop vertices must share tolerance policy",
        ));
    }
    Ok(())
}

fn denial(
    kind: CertifiedPolygonWinding2DDenialKind,
    reason: &'static str,
) -> CertifiedPolygonWinding2DDenial {
    CertifiedPolygonWinding2DDenial::new(kind, reason)
}
