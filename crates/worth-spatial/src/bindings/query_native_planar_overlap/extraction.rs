use forge_query::facade::ForgeQueryDomainOperatingContext;

use crate::bindings::query_native_planar_overlap::facts::CoplanarOverlapContractFactError;
use crate::bindings::query_native_planar_predicate::PlanarPredicateAuthorityQueryDomain;
use crate::bindings::query_native_planar_segment_segment::{
    CertifiedProjectedSegment2D, CertifiedSegmentSegment2D, CertifiedSegmentSegment2DContracts,
    CertifiedSegmentSegment2DQueryDomain, SegmentContactPolicy,
};
use crate::planar_contracts::coplanar_overlap_contract::{
    AmbiguousContactRow, ContainmentRelationRow, CoplanarOverlapContractBasis,
    CoplanarOverlapDenial, CoplanarOverlapDenialBasisLocus, CoplanarOverlapDenialKind,
    CoplanarOverlapPerformanceCounters, OverlapIslandRow, PolicyRequiredExitRow, SharedIntervalRow,
};
use crate::planar_contracts::polygon_winding_2d::CertifiedLoopWindingSummary;
use crate::planar_contracts::segment_segment_2d::{
    CertifiedSegmentSegment2DClassification, CertifiedSegmentSegment2DReceipt,
};

pub(crate) struct ExtractedOverlapRows {
    pub(crate) shared_intervals: Vec<SharedIntervalRow>,
    pub(crate) overlap_islands: Vec<OverlapIslandRow>,
    pub(crate) containment_relations: Vec<ContainmentRelationRow>,
    pub(crate) ambiguous_contacts: Vec<AmbiguousContactRow>,
    pub(crate) policy_required_exits: Vec<PolicyRequiredExitRow>,
    pub(crate) counters: CoplanarOverlapPerformanceCounters,
}

pub(crate) fn extract_overlap_rows<SC, PC>(
    basis: &CoplanarOverlapContractBasis,
    segment_contracts: &CertifiedSegmentSegment2DContracts<SC, PC>,
) -> Result<ExtractedOverlapRows, CoplanarOverlapContractFactError>
where
    SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
    PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
{
    let first_edges = face_edges(
        basis.first_face().face_identity(),
        basis.first_face().signed_area_receipt().basis().loops(),
    );
    let second_edges = face_edges(
        basis.second_face().face_identity(),
        basis.second_face().signed_area_receipt().basis().loops(),
    );
    let candidate_pair_breadth = first_edges.len() * second_edges.len();
    let mut shared_intervals = Vec::new();
    let mut ambiguous_contacts = Vec::new();
    let policy_required_exits = area_policy_exits(basis);
    let containment_relations = containment_rows(basis);
    if !policy_required_exits.is_empty() {
        return Ok(policy_required_rows(
            candidate_pair_breadth,
            containment_relations,
            policy_required_exits,
        ));
    }

    let mut certified_count = 0;
    for first in &first_edges {
        for second in &second_edges {
            let receipt =
                certify_edge_pair(first, second, basis.pair_identity(), segment_contracts)?;
            certified_count += 1;
            record_contact_row(
                first,
                second,
                &receipt,
                &mut shared_intervals,
                &mut ambiguous_contacts,
            );
        }
    }
    shared_intervals
        .sort_by(|left, right| left.segment_fact_digest().cmp(right.segment_fact_digest()));
    ambiguous_contacts
        .sort_by(|left, right| left.segment_fact_digest().cmp(right.segment_fact_digest()));
    let overlap_islands = shared_intervals
        .iter()
        .map(|row| OverlapIslandRow::new(row.island_identity().to_string(), 1))
        .collect::<Vec<_>>();
    let counters = CoplanarOverlapPerformanceCounters::certified(
        candidate_pair_breadth,
        certified_count,
        overlap_islands.len(),
        shared_intervals.len(),
        containment_relations.len(),
        0,
        0,
    );
    Ok(ExtractedOverlapRows {
        shared_intervals,
        overlap_islands,
        containment_relations,
        ambiguous_contacts,
        policy_required_exits: Vec::new(),
        counters,
    })
}

fn policy_required_rows(
    candidate_pair_breadth: usize,
    containment_relations: Vec<ContainmentRelationRow>,
    policy_required_exits: Vec<PolicyRequiredExitRow>,
) -> ExtractedOverlapRows {
    let counters = CoplanarOverlapPerformanceCounters::certified(
        candidate_pair_breadth,
        0,
        0,
        0,
        containment_relations.len(),
        policy_required_exits.len(),
        0,
    );
    ExtractedOverlapRows {
        shared_intervals: Vec::new(),
        overlap_islands: Vec::new(),
        containment_relations,
        ambiguous_contacts: Vec::new(),
        policy_required_exits,
        counters,
    }
}

#[derive(Clone)]
struct EdgeSnapshot {
    identity: String,
    start: crate::planar_contracts::projection_2d::ProjectPointToCertifiedPlane2DReceipt,
    end: crate::planar_contracts::projection_2d::ProjectPointToCertifiedPlane2DReceipt,
}

fn face_edges(face_identity: &str, loops: &[CertifiedLoopWindingSummary]) -> Vec<EdgeSnapshot> {
    loops
        .iter()
        .flat_map(|loop_summary| {
            let vertices = loop_summary.canonical_vertices();
            (0..vertices.len()).map(move |index| {
                let vertex = vertices[index];
                let next = vertices[(index + 1) % vertices.len()];
                EdgeSnapshot {
                    identity: format!(
                        "{}:{}:edge:{}",
                        face_identity,
                        loop_summary.loop_identity(),
                        index
                    ),
                    start: vertex.receipt.clone(),
                    end: next.receipt.clone(),
                }
            })
        })
        .collect()
}

fn record_contact_row(
    first: &EdgeSnapshot,
    second: &EdgeSnapshot,
    receipt: &CertifiedSegmentSegment2DReceipt,
    shared_intervals: &mut Vec<SharedIntervalRow>,
    ambiguous_contacts: &mut Vec<AmbiguousContactRow>,
) {
    let mut segment_identities = [first.identity.clone(), second.identity.clone()];
    segment_identities.sort();
    match receipt.classification() {
        CertifiedSegmentSegment2DClassification::CollinearOverlap
        | CertifiedSegmentSegment2DClassification::Identical
        | CertifiedSegmentSegment2DClassification::ReverseIdentical => {
            shared_intervals.push(SharedIntervalRow::new(
                format!("island:{}", receipt.fact_digest()),
                segment_identities[0].clone(),
                segment_identities[1].clone(),
                receipt.classification(),
                receipt.fact_digest().to_string(),
            ));
        }
        CertifiedSegmentSegment2DClassification::EndpointTouch
        | CertifiedSegmentSegment2DClassification::ProperCrossing => {
            ambiguous_contacts.push(AmbiguousContactRow::new(
                format!("contact:{}", receipt.fact_digest()),
                segment_identities[0].clone(),
                segment_identities[1].clone(),
                receipt.classification(),
                receipt.fact_digest().to_string(),
            ));
        }
        _ => {}
    }
}

fn certify_edge_pair<SC, PC>(
    first: &EdgeSnapshot,
    second: &EdgeSnapshot,
    topology_basis: &str,
    segment_contracts: &CertifiedSegmentSegment2DContracts<SC, PC>,
) -> Result<CertifiedSegmentSegment2DReceipt, CoplanarOverlapContractFactError>
where
    SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
    PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
{
    let (first, second) = if first.identity <= second.identity {
        (first, second)
    } else {
        (second, first)
    };
    let first_segment = CertifiedProjectedSegment2D::from_projected_endpoints(
        first.identity.clone(),
        first.start.clone(),
        first.end.clone(),
    )
    .map_err(|denial| CoplanarOverlapContractFactError::OverlapBasis {
        denial: segment_denial(denial.reason()),
    })?;
    let second_segment = CertifiedProjectedSegment2D::from_projected_endpoints(
        second.identity.clone(),
        second.start.clone(),
        second.end.clone(),
    )
    .map_err(|denial| CoplanarOverlapContractFactError::OverlapBasis {
        denial: segment_denial(denial.reason()),
    })?;
    CertifiedSegmentSegment2D::classify(first_segment, second_segment)
        .within_topology_basis(topology_basis)
        .with_policy(SegmentContactPolicy::CertifyContactsDenyImprintRequired)
        .compile(segment_contracts)
        .map_err(|denial| CoplanarOverlapContractFactError::OverlapBasis {
            denial: segment_denial(denial.reason()),
        })?
        .certify()
        .map_err(|error| CoplanarOverlapContractFactError::SegmentContact { error })
}

fn segment_denial(reason: &str) -> CoplanarOverlapDenial {
    CoplanarOverlapDenial::new(
        CoplanarOverlapDenialKind::AmbiguousContactRequiresPolicy,
        CoplanarOverlapDenialBasisLocus::SegmentContact,
        reason,
    )
}

fn area_policy_exits(basis: &CoplanarOverlapContractBasis) -> Vec<PolicyRequiredExitRow> {
    if !basis.area_policy_required() {
        return Vec::new();
    }
    let mut area_fact_digests = [
        basis
            .first_face()
            .signed_area_receipt()
            .fact_digest()
            .to_string(),
        basis
            .second_face()
            .signed_area_receipt()
            .fact_digest()
            .to_string(),
    ];
    area_fact_digests.sort();
    vec![PolicyRequiredExitRow::new(
        basis.pair_identity().to_string(),
        "signed-area-policy-required-before-overlap-imprint".to_string(),
        format!("{}:{}", area_fact_digests[0], area_fact_digests[1]),
    )]
}

fn containment_rows(basis: &CoplanarOverlapContractBasis) -> Vec<ContainmentRelationRow> {
    let mut rows = [
        (
            basis.first_face().face_identity(),
            basis.first_face().signed_area_receipt(),
        ),
        (
            basis.second_face().face_identity(),
            basis.second_face().signed_area_receipt(),
        ),
    ]
    .into_iter()
    .flat_map(|(face, receipt)| {
        receipt
            .basis()
            .loops()
            .iter()
            .filter(move |loop_summary| {
                loop_summary.loop_identity() != receipt.basis().primary_loop_identity()
            })
            .filter_map(move |loop_summary| {
                receipt
                    .basis()
                    .winding_receipt()
                    .containment_for(loop_summary.loop_identity())
                    .map(|containment| {
                        ContainmentRelationRow::new(
                            face.to_string(),
                            loop_summary.loop_identity().to_string(),
                            containment.as_str().to_string(),
                            receipt.fact_digest().to_string(),
                        )
                    })
            })
    })
    .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.winding_fact_digest()
            .cmp(right.winding_fact_digest())
            .then_with(|| left.face_identity().cmp(right.face_identity()))
            .then_with(|| left.loop_identity().cmp(right.loop_identity()))
            .then_with(|| left.containment().cmp(right.containment()))
    });
    rows
}
