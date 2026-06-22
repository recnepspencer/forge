use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
    ForgeQueryOrdinaryNextStep, ForgeQueryOrdinaryOutcome, ForgeQueryOrdinaryPosture,
    ForgeQueryOrdinaryPostureKind,
};

use crate::bindings::query_native_planar_overlap::authoring::CoplanarOverlapContractEntry;
use crate::bindings::query_native_planar_overlap::domain::CoplanarOverlapContractQueryDomain;
use crate::bindings::query_native_planar_overlap::extraction::extract_overlap_rows;
use crate::bindings::query_native_planar_predicate::PlanarPredicateAuthorityQueryDomain;
use crate::bindings::query_native_planar_segment_segment::CertifiedSegmentSegment2DContracts;
use crate::bindings::query_native_planar_segment_segment::{
    CertifiedSegmentSegment2DFactError, CertifiedSegmentSegment2DQueryDomain,
};
use crate::planar_contracts::coplanar_overlap_contract::{
    CoplanarOverlapContractReceipt, CoplanarOverlapDenial, CoplanarOverlapPerformanceCounters,
};

#[derive(Clone, Debug, PartialEq)]
pub enum CoplanarOverlapContractFactError {
    OutcomeNotBound {
        kind: ForgeQueryOrdinaryPostureKind,
        reason: String,
        next_step: ForgeQueryOrdinaryNextStep,
    },
    OverlapBasis {
        denial: CoplanarOverlapDenial,
    },
    SegmentContact {
        error: CertifiedSegmentSegment2DFactError,
    },
}

impl CoplanarOverlapContractFactError {
    fn outcome_not_bound(posture: &ForgeQueryOrdinaryPosture) -> Self {
        Self::OutcomeNotBound {
            kind: posture.kind(),
            reason: posture.reason().to_string(),
            next_step: posture.next_step(),
        }
    }
}

pub fn coplanar_overlap_contract_facts<OC, SC, PC>(
    entry: &CoplanarOverlapContractEntry,
    overlap_handle: &ForgeQueryAdmittedConfiguredDomainHandle<
        CoplanarOverlapContractQueryDomain,
        OC,
    >,
    segment_contracts: &CertifiedSegmentSegment2DContracts<SC, PC>,
) -> Result<CoplanarOverlapContractReceipt, CoplanarOverlapContractFactError>
where
    OC: ForgeQueryDomainOperatingContext<CoplanarOverlapContractQueryDomain>,
    SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
    PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
{
    match overlap_handle.orchestrate_declaration_entry_outcome(entry.clone()) {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => {
            let rows = extract_overlap_rows(entry.case().basis(), segment_contracts)?;
            let shared_intervals = rows.shared_intervals.clone();
            let overlap_islands = rows.overlap_islands.clone();
            let containment_relations = rows.containment_relations.clone();
            let ambiguous_contacts = rows.ambiguous_contacts.clone();
            let policy_required_exits = rows.policy_required_exits.clone();
            let declaration_digest = envelope.declaration_digest().to_string();
            let envelope_digest = format!("{:?}", envelope.envelope_digest());
            let certified_basis = entry.case().basis().clone().with_rows(
                rows.shared_intervals,
                rows.overlap_islands,
                rows.containment_relations,
                rows.ambiguous_contacts,
                rows.policy_required_exits,
                rows.counters,
            );
            let basis_part_count = CoplanarOverlapContractReceipt::digest_parts(
                &certified_basis,
                &declaration_digest,
                &envelope_digest,
            )
            .len();
            let counters = CoplanarOverlapPerformanceCounters::certified(
                certified_basis.counters().candidate_pair_breadth(),
                certified_basis.counters().segment_contacts_certified(),
                certified_basis.counters().overlap_islands(),
                certified_basis.counters().shared_intervals(),
                certified_basis.counters().containment_relations(),
                certified_basis.counters().policy_required_exits(),
                basis_part_count,
            );
            let certified_basis = certified_basis.with_rows(
                shared_intervals,
                overlap_islands,
                containment_relations,
                ambiguous_contacts,
                policy_required_exits,
                counters,
            );
            let fact_digest = CoplanarOverlapContractReceipt::fact_digest_for(
                &certified_basis,
                &declaration_digest,
                &envelope_digest,
            );
            Ok(CoplanarOverlapContractReceipt::new(
                certified_basis,
                declaration_digest,
                envelope_digest,
                fact_digest,
                counters,
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
            CoplanarOverlapContractFactError::outcome_not_bound(&posture),
        ),
    }
}
