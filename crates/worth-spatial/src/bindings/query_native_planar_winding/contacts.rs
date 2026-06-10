use forge_query::facade::ForgeQueryDomainOperatingContext;

use crate::bindings::query_native_planar_predicate::PlanarPredicateAuthorityQueryDomain;
use crate::bindings::query_native_planar_segment_segment::{
    CertifiedProjectedSegment2D, CertifiedSegmentSegment2D, CertifiedSegmentSegment2DContracts,
    CertifiedSegmentSegment2DQueryDomain, SegmentContactPolicy,
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
) -> Result<Vec<CertifiedSegmentSegment2DReceipt>, CertifiedPolygonWinding2DFactError>
where
    SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
    PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
{
    let mut receipts = Vec::new();
    for loop_summary in basis.loop_summaries() {
        receipts.extend(certify_self_contacts(
            loop_summary,
            basis,
            segment_contracts,
        )?);
    }
    let loops = basis.loop_summaries();
    for candidate in loops.iter().skip(1) {
        receipts.extend(certify_cross_loop_contacts(
            &loops[0],
            candidate,
            basis,
            segment_contracts,
        )?);
    }
    Ok(receipts)
}

fn certify_self_contacts<SC, PC>(
    loop_summary: &CertifiedLoopWindingSummary,
    basis: &CertifiedPolygonWinding2DBasis,
    segment_contracts: &CertifiedSegmentSegment2DContracts<SC, PC>,
) -> Result<Vec<CertifiedSegmentSegment2DReceipt>, CertifiedPolygonWinding2DFactError>
where
    SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
    PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
{
    let mut receipts = Vec::new();
    let edge_count = loop_summary.canonical_vertices().len();
    for first in 0..edge_count {
        for second in first + 1..edge_count {
            if edges_are_adjacent(first, second, edge_count) {
                continue;
            }
            receipts.push(certify_edge_pair(
                loop_summary,
                first,
                loop_summary,
                second,
                basis,
                segment_contracts,
            )?);
        }
    }
    Ok(receipts)
}

fn certify_cross_loop_contacts<SC, PC>(
    primary: &CertifiedLoopWindingSummary,
    candidate: &CertifiedLoopWindingSummary,
    basis: &CertifiedPolygonWinding2DBasis,
    segment_contracts: &CertifiedSegmentSegment2DContracts<SC, PC>,
) -> Result<Vec<CertifiedSegmentSegment2DReceipt>, CertifiedPolygonWinding2DFactError>
where
    SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
    PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
{
    let mut receipts = Vec::new();
    for first in 0..primary.canonical_vertices().len() {
        for second in 0..candidate.canonical_vertices().len() {
            receipts.push(certify_edge_pair(
                primary,
                first,
                candidate,
                second,
                basis,
                segment_contracts,
            )?);
        }
    }
    Ok(receipts)
}

fn certify_edge_pair<SC, PC>(
    first_loop: &CertifiedLoopWindingSummary,
    first_edge: usize,
    second_loop: &CertifiedLoopWindingSummary,
    second_edge: usize,
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
        stable_edge_identity(first_loop.loop_identity(), &first_vertices, first_edge),
        first_vertices[first_edge].receipt.clone(),
        first_vertices[(first_edge + 1) % first_vertices.len()]
            .receipt
            .clone(),
    )
    .map_err(|denial| map_segment_denial(denial.reason()))?;
    let second = CertifiedProjectedSegment2D::from_projected_endpoints(
        stable_edge_identity(second_loop.loop_identity(), &second_vertices, second_edge),
        second_vertices[second_edge].receipt.clone(),
        second_vertices[(second_edge + 1) % second_vertices.len()]
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

fn edges_are_adjacent(first: usize, second: usize, edge_count: usize) -> bool {
    first == second || first + 1 == second || (first == 0 && second + 1 == edge_count)
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
