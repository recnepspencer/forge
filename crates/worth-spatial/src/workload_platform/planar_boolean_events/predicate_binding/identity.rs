use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::planar_contracts::predicate_consumption::PredicateCertificateConsumptionReceipt;

use super::bound_pair::PlanarBooleanPredicateBoundPair;
use super::counters::PlanarBooleanEventPredicateBindingCounters;
use crate::workload_platform::planar_boolean_events::PlanarBooleanSegmentPairEnumerationReceipt;

pub(crate) fn predicate_binding_identity(
    reduced_pair_identity: &str,
    pair_enumeration: &PlanarBooleanSegmentPairEnumerationReceipt,
    predicate_consumption: &PredicateCertificateConsumptionReceipt,
    counters: PlanarBooleanEventPredicateBindingCounters,
    bound_pairs: &[PlanarBooleanPredicateBoundPair],
) -> String {
    let mut parts = vec![
        "planar-boolean-event-predicate-binding".to_string(),
        format!("reduced-pair:{reduced_pair_identity}"),
        format!(
            "pair-enumeration:{}",
            pair_enumeration.segment_pair_enumeration_identity()
        ),
        format!(
            "canonical-segment-set:{}",
            pair_enumeration.canonical_segment_set_identity()
        ),
        format!(
            "predicate-consumption:{}",
            predicate_consumption.fact_digest()
        ),
        format!(
            "required-segment-contracts:{}",
            counters.required_segment_contracts()
        ),
        format!(
            "supplied-segment-contracts:{}",
            counters.supplied_segment_contracts()
        ),
        format!("bound-segment-pairs:{}", counters.bound_segment_pairs()),
        format!(
            "required-predicate-rows:{}",
            counters.required_predicate_rows()
        ),
        format!(
            "certified-predicate-rows:{}",
            counters.certified_predicate_rows()
        ),
    ];
    parts.extend(
        bound_pairs
            .iter()
            .map(|pair| format!("bound-pair:{}", pair.bound_pair_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) struct BoundPairIdentityBasis<'a> {
    pub(crate) reduced_pair_identity: &'a str,
    pub(crate) segment_pair_identity: &'a str,
    pub(crate) left_segment_identity: &'a str,
    pub(crate) right_segment_identity: &'a str,
    pub(crate) left_carrier_identity: &'a str,
    pub(crate) right_carrier_identity: &'a str,
    pub(crate) segment_contract_fact_digest: &'a str,
    pub(crate) predicate_consumption_fact_digest: &'a str,
    pub(crate) local_frame_identity: &'a str,
    pub(crate) precision_basis_identity: &'a str,
}

pub(crate) fn bound_pair_identity(basis: BoundPairIdentityBasis<'_>) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-predicate-bound-pair".to_string(),
            format!("reduced-pair:{}", basis.reduced_pair_identity),
            format!("segment-pair:{}", basis.segment_pair_identity),
            format!("left:{}", basis.left_segment_identity),
            format!("right:{}", basis.right_segment_identity),
            format!("left-carrier:{}", basis.left_carrier_identity),
            format!("right-carrier:{}", basis.right_carrier_identity),
            format!("segment-contract:{}", basis.segment_contract_fact_digest),
            format!(
                "predicate-consumption:{}",
                basis.predicate_consumption_fact_digest
            ),
            format!("local-frame:{}", basis.local_frame_identity),
            format!("precision-basis:{}", basis.precision_basis_identity),
        ],
    )
}
