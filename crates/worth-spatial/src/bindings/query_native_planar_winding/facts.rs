use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
    ForgeQueryOrdinaryNextStep, ForgeQueryOrdinaryOutcome, ForgeQueryOrdinaryPosture,
    ForgeQueryOrdinaryPostureKind,
};

use crate::bindings::query_native_planar_predicate::{
    planar_predicate_authority_entry, planar_predicate_authority_facts,
    PlanarPredicateAuthorityCase, PlanarPredicateAuthorityFactError,
    PlanarPredicateAuthorityQueryDomain,
};
use crate::bindings::query_native_planar_segment_segment::{
    CertifiedSegmentSegment2DContracts, CertifiedSegmentSegment2DFactError,
    CertifiedSegmentSegment2DQueryDomain,
};
use crate::bindings::query_native_planar_winding::authoring::CertifiedPolygonWinding2DEntry;
use crate::bindings::query_native_planar_winding::contacts::certify_segment_contacts;
use crate::bindings::query_native_planar_winding::domain::CertifiedPolygonWinding2DQueryDomain;
use crate::planar_contracts::polygon_winding_2d::{
    point_strictly_inside_loop, CertifiedLoopContainment, CertifiedPolygonWinding2DDenial,
    CertifiedPolygonWinding2DDenialKind, CertifiedPolygonWinding2DPerformanceCounters,
    CertifiedPolygonWinding2DReceipt,
};
use crate::planar_contracts::predicate_authority::PlanarPredicateInputBasis;
use crate::planar_contracts::segment_segment_2d::{
    CertifiedSegmentSegment2DClassification, CertifiedSegmentSegment2DReceipt,
};

#[derive(Clone, Debug, PartialEq)]
pub enum CertifiedPolygonWinding2DFactError {
    OutcomeNotBound {
        kind: ForgeQueryOrdinaryPostureKind,
        reason: String,
        next_step: ForgeQueryOrdinaryNextStep,
    },
    PredicateFact {
        source: PlanarPredicateAuthorityFactError,
    },
    SegmentFact {
        source: CertifiedSegmentSegment2DFactError,
    },
    WindingBasis {
        denial: CertifiedPolygonWinding2DDenial,
    },
}

impl CertifiedPolygonWinding2DFactError {
    fn outcome_not_bound(posture: &ForgeQueryOrdinaryPosture) -> Self {
        Self::OutcomeNotBound {
            kind: posture.kind(),
            reason: posture.reason().to_string(),
            next_step: posture.next_step(),
        }
    }
}

pub fn certified_polygon_winding_2d_facts<WC, SC, PC>(
    entry: &CertifiedPolygonWinding2DEntry,
    winding_handle: &ForgeQueryAdmittedConfiguredDomainHandle<
        CertifiedPolygonWinding2DQueryDomain,
        WC,
    >,
    segment_contracts: &CertifiedSegmentSegment2DContracts<SC, PC>,
    predicate_handle: &ForgeQueryAdmittedConfiguredDomainHandle<
        PlanarPredicateAuthorityQueryDomain,
        PC,
    >,
) -> Result<CertifiedPolygonWinding2DReceipt, CertifiedPolygonWinding2DFactError>
where
    WC: ForgeQueryDomainOperatingContext<CertifiedPolygonWinding2DQueryDomain>,
    SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
    PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
{
    match winding_handle.orchestrate_declaration_entry_outcome(entry.clone()) {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => {
            let basis = entry.case().basis().clone();
            let segments = certify_segment_contacts(&basis, segment_contracts)?;
            deny_bad_contacts(&segments)?;
            let predicates = certify_winding_predicates(&basis, predicate_handle)?;
            let containments = containment_rows(&basis, &segments)?;
            let certified_basis = basis
                .with_certification_evidence(predicates, segments, containments)
                .map_err(|denial| CertifiedPolygonWinding2DFactError::WindingBasis { denial })?;
            let envelope_digest = format!("{:?}", envelope.envelope_digest());
            let declaration_digest = envelope.declaration_digest().to_string();
            let basis_part_count = CertifiedPolygonWinding2DReceipt::digest_parts(
                &certified_basis,
                &declaration_digest,
                &envelope_digest,
            )
            .len();
            let fact_digest = CertifiedPolygonWinding2DReceipt::fact_digest_for(
                &certified_basis,
                &declaration_digest,
                &envelope_digest,
            );
            Ok(CertifiedPolygonWinding2DReceipt::new(
                certified_basis.clone(),
                declaration_digest,
                envelope_digest,
                fact_digest,
                CertifiedPolygonWinding2DPerformanceCounters::certified(
                    certified_basis.loop_edges_walked(),
                    certified_basis.vertices().len(),
                    certified_basis.segment_contact_fact_digests().len(),
                    certified_basis.winding_predicate_fact_digests().len(),
                    certified_basis.winding_tie_breaks_used(),
                    basis_part_count,
                ),
            ))
        }
        ForgeQueryOrdinaryOutcome::Ambiguous(posture)
        | ForgeQueryOrdinaryOutcome::AspectConflict(posture)
        | ForgeQueryOrdinaryOutcome::AuthorityMismatch(posture)
        | ForgeQueryOrdinaryOutcome::BasisMismatch(posture)
        | ForgeQueryOrdinaryOutcome::Deferred(posture)
        | ForgeQueryOrdinaryOutcome::Denied(posture)
        | ForgeQueryOrdinaryOutcome::ExplicitNarrowingRequired(posture)
        | ForgeQueryOrdinaryOutcome::Failed(posture)
        | ForgeQueryOrdinaryOutcome::MissingRequiredAspect(posture)
        | ForgeQueryOrdinaryOutcome::RebindRequired(posture)
        | ForgeQueryOrdinaryOutcome::Refused(posture)
        | ForgeQueryOrdinaryOutcome::Stale(posture)
        | ForgeQueryOrdinaryOutcome::Unavailable(posture)
        | ForgeQueryOrdinaryOutcome::Unsupported(posture)
        | ForgeQueryOrdinaryOutcome::WrongHandle(posture)
        | ForgeQueryOrdinaryOutcome::WrongWorld(posture) => Err(
            CertifiedPolygonWinding2DFactError::outcome_not_bound(&posture),
        ),
    }
}

fn certify_winding_predicates<PC>(
    basis: &crate::planar_contracts::polygon_winding_2d::CertifiedPolygonWinding2DBasis,
    predicate_handle: &ForgeQueryAdmittedConfiguredDomainHandle<
        PlanarPredicateAuthorityQueryDomain,
        PC,
    >,
) -> Result<
    Vec<crate::planar_contracts::predicate_authority::PlanarPredicateFactReceipt>,
    CertifiedPolygonWinding2DFactError,
>
where
    PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
{
    let mut receipts = Vec::new();
    for loop_summary in basis.loop_summaries() {
        let vertices = loop_summary.canonical_vertices();
        for index in 1..vertices.len() - 1 {
            let predicate_basis = PlanarPredicateInputBasis::from_projected_orient2d_points(
                basis.frame_identity(),
                loop_summary.topology_loop_identity(),
                basis.movement_rotation_posture_identity(),
                basis.tolerance_policy_identity(),
                [
                    vertices[0].point_2d,
                    vertices[index].point_2d,
                    vertices[index + 1].point_2d,
                ],
            );
            let predicate_entry = planar_predicate_authority_entry(
                PlanarPredicateAuthorityCase::orient2d(predicate_basis),
            );
            receipts.push(
                planar_predicate_authority_facts(&predicate_entry, predicate_handle).map_err(
                    |source| CertifiedPolygonWinding2DFactError::PredicateFact { source },
                )?,
            );
        }
    }
    Ok(receipts)
}

fn deny_bad_contacts(
    receipts: &[CertifiedSegmentSegment2DReceipt],
) -> Result<(), CertifiedPolygonWinding2DFactError> {
    for receipt in receipts {
        match receipt.classification() {
            CertifiedSegmentSegment2DClassification::Disjoint
            | CertifiedSegmentSegment2DClassification::CollinearDisjoint => {}
            _ => {
                return Err(CertifiedPolygonWinding2DFactError::WindingBasis {
                    denial: CertifiedPolygonWinding2DDenial::new(
                        CertifiedPolygonWinding2DDenialKind::SelfIntersectionOrAmbiguousTouch,
                        "loop winding denies self-intersection and ambiguous loop-edge touch",
                    ),
                });
            }
        }
    }
    Ok(())
}

fn containment_rows(
    basis: &crate::planar_contracts::polygon_winding_2d::CertifiedPolygonWinding2DBasis,
    _segments: &[CertifiedSegmentSegment2DReceipt],
) -> Result<Vec<(String, CertifiedLoopContainment)>, CertifiedPolygonWinding2DFactError> {
    let primary_points = basis.loop_summaries()[0]
        .canonical_vertices()
        .iter()
        .map(|vertex| vertex.point_2d)
        .collect::<Vec<_>>();
    let mut rows = Vec::new();
    for candidate in basis.loop_summaries().iter().skip(1) {
        let inside = candidate
            .vertices()
            .iter()
            .all(|vertex| point_strictly_inside_loop(vertex.point_2d, &primary_points));
        rows.push((
            candidate.loop_identity().to_string(),
            if inside {
                CertifiedLoopContainment::ContainedHole
            } else {
                CertifiedLoopContainment::Outside
            },
        ));
    }
    Ok(rows)
}
