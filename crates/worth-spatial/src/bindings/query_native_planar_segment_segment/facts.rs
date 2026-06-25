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
use crate::bindings::query_native_planar_segment_segment::authoring::CertifiedSegmentSegment2DEntry;
use crate::bindings::query_native_planar_segment_segment::domain::CertifiedSegmentSegment2DQueryDomain;
use crate::planar_contracts::predicate_authority::{
    PlanarPredicateFactReceipt, PlanarPredicateInputBasis,
};
use crate::planar_contracts::segment_segment_2d::{
    CertifiedSegmentSegment2DDenial, CertifiedSegmentSegment2DMutationEvidence,
    CertifiedSegmentSegment2DPerformanceCounters, CertifiedSegmentSegment2DReceipt,
};

#[derive(Clone, Debug, PartialEq)]
pub enum CertifiedSegmentSegment2DFactError {
    OutcomeNotBound {
        kind: ForgeQueryOrdinaryPostureKind,
        reason: String,
        next_step: ForgeQueryOrdinaryNextStep,
    },
    PredicateFact {
        source: PlanarPredicateAuthorityFactError,
    },
    SegmentBasis {
        denial: CertifiedSegmentSegment2DDenial,
    },
}

impl CertifiedSegmentSegment2DFactError {
    fn outcome_not_bound(posture: &ForgeQueryOrdinaryPosture) -> Self {
        Self::OutcomeNotBound {
            kind: posture.kind(),
            reason: posture.reason().to_string(),
            next_step: posture.next_step(),
        }
    }
}

pub fn certified_segment_segment_2d_facts<SC, PC>(
    entry: &CertifiedSegmentSegment2DEntry,
    segment_handle: &ForgeQueryAdmittedConfiguredDomainHandle<
        CertifiedSegmentSegment2DQueryDomain,
        SC,
    >,
    predicate_handle: &ForgeQueryAdmittedConfiguredDomainHandle<
        PlanarPredicateAuthorityQueryDomain,
        PC,
    >,
) -> Result<CertifiedSegmentSegment2DReceipt, CertifiedSegmentSegment2DFactError>
where
    SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
    PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
{
    certified_segment_segment_2d_facts_with_predicate_resolver(entry, segment_handle, |basis| {
        let predicate_entry =
            planar_predicate_authority_entry(PlanarPredicateAuthorityCase::orient2d(basis));
        planar_predicate_authority_facts(&predicate_entry, predicate_handle)
            .map_err(|source| CertifiedSegmentSegment2DFactError::PredicateFact { source })
    })
}

pub fn certified_segment_segment_2d_facts_with_predicate_resolver<SC, F>(
    entry: &CertifiedSegmentSegment2DEntry,
    segment_handle: &ForgeQueryAdmittedConfiguredDomainHandle<
        CertifiedSegmentSegment2DQueryDomain,
        SC,
    >,
    mut predicate_resolver: F,
) -> Result<CertifiedSegmentSegment2DReceipt, CertifiedSegmentSegment2DFactError>
where
    SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
    F: FnMut(
        PlanarPredicateInputBasis,
    ) -> Result<PlanarPredicateFactReceipt, CertifiedSegmentSegment2DFactError>,
{
    match segment_handle.orchestrate_declaration_entry_outcome(entry.clone()) {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => {
            let basis = entry.case().basis().clone();
            let points = basis.expected_orientation_points();
            let receipts = points
                .iter()
                .map(|projected_points| {
                    let predicate_basis = PlanarPredicateInputBasis::from_projected_orient2d_points(
                        basis.frame_identity(),
                        basis.topology_basis_identity(),
                        basis.movement_rotation_posture_identity(),
                        basis.tolerance_policy_identity(),
                        *projected_points,
                    );
                    predicate_resolver(predicate_basis)
                })
                .collect::<Result<Vec<_>, _>>()?;
            let certified_basis = basis
                .with_orientation_receipts([&receipts[0], &receipts[1], &receipts[2], &receipts[3]])
                .map_err(|denial| CertifiedSegmentSegment2DFactError::SegmentBasis { denial })?;
            let envelope_digest = format!("{:?}", envelope.envelope_digest());
            let declaration_digest = envelope.declaration_digest().to_string();
            let basis_part_count = CertifiedSegmentSegment2DReceipt::digest_parts(
                &certified_basis,
                &declaration_digest,
                &envelope_digest,
            )
            .len();
            let fact_digest = CertifiedSegmentSegment2DReceipt::fact_digest_for(
                &certified_basis,
                &declaration_digest,
                &envelope_digest,
            );
            let mutation_evidence = CertifiedSegmentSegment2DMutationEvidence::from_segment_fact(
                &certified_basis,
                &declaration_digest,
                &envelope_digest,
                &fact_digest,
            );
            Ok(CertifiedSegmentSegment2DReceipt::new(
                certified_basis,
                declaration_digest,
                envelope_digest,
                fact_digest,
                mutation_evidence,
                receipts,
                CertifiedSegmentSegment2DPerformanceCounters::certified(basis_part_count),
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
            CertifiedSegmentSegment2DFactError::outcome_not_bound(&posture),
        ),
    }
}
