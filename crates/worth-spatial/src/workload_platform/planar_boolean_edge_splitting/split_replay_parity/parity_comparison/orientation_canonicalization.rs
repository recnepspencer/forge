use super::super::parity_receipt::denial::{
    PlanarBooleanEdgeSplitReplayParityDenial, PlanarBooleanEdgeSplitReplayParityDenialKind as Kind,
};
use super::input::PlanarBooleanEdgeSplitReplayParityInput;
use crate::workload_platform::planar_boolean_events::PlanarBooleanSourceIntervalSense;

pub(super) fn validate_reversed_source_sense_canonicalization(
    input: &PlanarBooleanEdgeSplitReplayParityInput<'_>,
) -> Result<String, PlanarBooleanEdgeSplitReplayParityDenial> {
    let original_reversed_fragments =
        reversed_fragment_count(input.original_fragments().fragments());
    let replayed_reversed_fragments =
        reversed_fragment_count(input.replayed_fragments().fragments());
    let original_reversed_overlap_members =
        reversed_overlap_member_count(input.original_overlap_chains());
    let replayed_reversed_overlap_members =
        reversed_overlap_member_count(input.replayed_overlap_chains());

    if original_reversed_fragments != replayed_reversed_fragments {
        return Err(PlanarBooleanEdgeSplitReplayParityDenial::new(
            Kind::OrientationCanonicalizationMismatch,
            "reversed-fragment-count",
            original_reversed_fragments.to_string(),
            replayed_reversed_fragments.to_string(),
            "reversed source-sense fragment rows must remain canonical across replay",
        ));
    }
    if original_reversed_overlap_members != replayed_reversed_overlap_members {
        return Err(PlanarBooleanEdgeSplitReplayParityDenial::new(
            Kind::OrientationCanonicalizationMismatch,
            "reversed-overlap-member-count",
            original_reversed_overlap_members.to_string(),
            replayed_reversed_overlap_members.to_string(),
            "reversed source-sense overlap members must remain canonical across replay",
        ));
    }
    if original_reversed_fragments == 0 && original_reversed_overlap_members == 0 {
        return Err(PlanarBooleanEdgeSplitReplayParityDenial::new(
            Kind::OrientationCanonicalizationMismatch,
            "reversed-source-sense-coverage",
            "at least one reversed source-sense row",
            "none",
            "edge-split replay parity must cover a reversed source-sense split product",
        ));
    }
    Ok(format!(
        "reversed-source-sense:{original_reversed_fragments}:{original_reversed_overlap_members}"
    ))
}

fn reversed_fragment_count<'a>(
    fragments: impl Iterator<Item = &'a crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanSplitEdgeFragment>,
) -> usize {
    fragments
        .filter(|fragment| {
            fragment
                .source_senses()
                .contains(&PlanarBooleanSourceIntervalSense::Reversed)
        })
        .count()
}

fn reversed_overlap_member_count(
    chains: &crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanOverlapEdgeChainSet,
) -> usize {
    chains
        .chains()
        .iter()
        .flat_map(|chain| chain.members())
        .filter(|member| member.source_sense() == PlanarBooleanSourceIntervalSense::Reversed)
        .count()
}
