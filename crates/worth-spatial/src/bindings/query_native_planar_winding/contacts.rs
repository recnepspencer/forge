use forge_query::facade::ForgeQueryDomainOperatingContext;

use crate::bindings::query_native_planar_predicate::PlanarPredicateAuthorityQueryDomain;
use crate::bindings::query_native_planar_segment_segment::{
    CertifiedProjectedSegment2D, CertifiedSegmentSegment2D, CertifiedSegmentSegment2DContracts,
    CertifiedSegmentSegment2DQueryDomain, SegmentContactPolicy,
};
use crate::bindings::query_native_planar_winding::candidate_index::{
    WindingSegmentContactCandidateIndex, WindingSegmentContactCandidateIndexCounters,
    WindingSegmentContactCandidateRow,
};
use crate::bindings::query_native_planar_winding::facts::CertifiedPolygonWinding2DFactError;
use crate::planar_contracts::polygon_winding_2d::{
    CertifiedLoopWindingSummary, CertifiedPolygonWinding2DBasis, CertifiedPolygonWinding2DDenial,
    CertifiedPolygonWinding2DDenialKind, ProjectedLoopVertexSnapshot,
};
use crate::planar_contracts::segment_segment_2d::CertifiedSegmentSegment2DReceipt;

pub(crate) fn certify_segment_contacts<SC, PC>(
    basis: &CertifiedPolygonWinding2DBasis,
    segment_contracts: &CertifiedSegmentSegment2DContracts<SC, PC>,
) -> Result<
    (
        Vec<CertifiedSegmentSegment2DReceipt>,
        WindingSegmentContactCandidateIndexCounters,
    ),
    CertifiedPolygonWinding2DFactError,
>
where
    SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
    PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
{
    let candidate_index = WindingSegmentContactCandidateIndex::from_basis(basis);
    let mut receipts = Vec::new();
    let loops = basis.loop_summaries();
    for row in candidate_index.rows() {
        receipts.push(certify_edge_pair(
            row,
            &loops[row.first_loop_index()],
            &loops[row.second_loop_index()],
            basis,
            segment_contracts,
        )?);
    }
    Ok((receipts, candidate_index.counters()))
}

fn certify_edge_pair<SC, PC>(
    row: &WindingSegmentContactCandidateRow,
    first_loop: &CertifiedLoopWindingSummary,
    second_loop: &CertifiedLoopWindingSummary,
    basis: &CertifiedPolygonWinding2DBasis,
    segment_contracts: &CertifiedSegmentSegment2DContracts<SC, PC>,
) -> Result<CertifiedSegmentSegment2DReceipt, CertifiedPolygonWinding2DFactError>
where
    SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
    PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
{
    let first_vertices = first_loop.canonical_vertices();
    let second_vertices = second_loop.canonical_vertices();
    let first = CertifiedProjectedSegment2D::from_projected_endpoints(
        stable_edge_identity(row.first_loop_identity(), &first_vertices, row.first_edge()),
        first_vertices[row.first_edge()].receipt.clone(),
        first_vertices[(row.first_edge() + 1) % first_vertices.len()]
            .receipt
            .clone(),
    )
    .map_err(|denial| map_segment_denial(denial.reason()))?;
    let second = CertifiedProjectedSegment2D::from_projected_endpoints(
        stable_edge_identity(
            row.second_loop_identity(),
            &second_vertices,
            row.second_edge(),
        ),
        second_vertices[row.second_edge()].receipt.clone(),
        second_vertices[(row.second_edge() + 1) % second_vertices.len()]
            .receipt
            .clone(),
    )
    .map_err(|denial| map_segment_denial(denial.reason()))?;
    CertifiedSegmentSegment2D::classify(first, second)
        .within_topology_basis(basis.planar_neighborhood_identity())
        .with_policy(SegmentContactPolicy::CertifyContactsDenyImprintRequired)
        .compile(segment_contracts)
        .map_err(|denial| map_segment_denial(denial.reason()))?
        .certify()
        .map_err(|source| CertifiedPolygonWinding2DFactError::SegmentFact { source })
}

fn stable_edge_identity(
    loop_identity: &str,
    vertices: &[&ProjectedLoopVertexSnapshot],
    edge: usize,
) -> String {
    let left = vertices[edge].projection_fact_digest.as_str();
    let right = vertices[(edge + 1) % vertices.len()]
        .projection_fact_digest
        .as_str();
    if left <= right {
        format!("{loop_identity}:edge:{left}:{right}")
    } else {
        format!("{loop_identity}:edge:{right}:{left}")
    }
}

fn map_segment_denial(reason: &'static str) -> CertifiedPolygonWinding2DFactError {
    CertifiedPolygonWinding2DFactError::WindingBasis {
        denial: CertifiedPolygonWinding2DDenial::new(
            CertifiedPolygonWinding2DDenialKind::SegmentContactCertificationBasis,
            reason,
        ),
    }
}
