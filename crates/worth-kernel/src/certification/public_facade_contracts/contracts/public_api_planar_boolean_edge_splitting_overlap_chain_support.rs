use std::collections::{BTreeMap, BTreeSet};

use super::edge_splitting_split_vertex_identity_support::build_interval_subdivision_schedule_for_metaboss;
use super::metaboss_support::MetabossEventExtractionSubject;
use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanIntervalSubdivisionNormalizedScheduleSet, PlanarBooleanOverlapEdgeChainSet,
    PlanarBooleanSplitEdgeFragmentSet,
};
use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanIntervalEventKind, PlanarBooleanSourceIntervalSense,
};

pub(crate) fn assert_overlap_edge_chains_match_metaboss(subject: &MetabossEventExtractionSubject) {
    let interval_normalized = build_interval_subdivision_schedule_for_metaboss(subject);
    let split_vertices = interval_normalized
        .mint_split_vertex_identities()
        .expect("metaboss split vertices should mint before overlap chains");
    let fragments = interval_normalized
        .build_split_edge_fragments(&split_vertices)
        .expect("metaboss split fragments should build before overlap chains");
    let chains = interval_normalized
        .build_overlap_edge_chains(&fragments)
        .expect("metaboss overlap edge chains should build from interval authority and fragments");

    assert_eq!(
        chains.interval_subdivision_schedule_set_identity(),
        interval_normalized.schedule_set_identity()
    );
    assert_eq!(
        chains.split_edge_fragment_set_identity(),
        fragments.fragment_set_identity()
    );
    assert!(chains.certifies_prepared_overlap_chains());
    assert!(!chains.emits_topology_truth());
    assert_overlap_chain_counters_reconcile(&interval_normalized, &fragments, &chains);
    assert_overlap_chain_rows_preserve_public_authority(&chains);
}

fn assert_overlap_chain_counters_reconcile(
    interval_normalized: &PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
    fragments: &PlanarBooleanSplitEdgeFragmentSet,
    chains: &PlanarBooleanOverlapEdgeChainSet,
) {
    let expected_chain_count = interval_normalized
        .schedules()
        .iter()
        .flat_map(|schedule| schedule.interval_subdivisions())
        .map(|subdivision| subdivision.interval_event_identity().to_string())
        .collect::<BTreeSet<_>>()
        .len();
    let expected_subdivision_count = interval_normalized
        .schedules()
        .iter()
        .map(|schedule| schedule.interval_subdivisions().len())
        .sum::<usize>();
    let expected_fragment_rows = fragments.fragments().count();
    assert_eq!(chains.counters().chains_emitted(), expected_chain_count);
    assert_eq!(
        chains.counters().interval_subdivisions_inspected(),
        expected_subdivision_count
    );
    assert_eq!(
        chains.counters().fragment_rows_inspected(),
        expected_fragment_rows
    );
    assert_eq!(chains.counters().topology_products_emitted(), 0);
    assert_eq!(chains.counters().foreign_fragment_sets_rejected(), 0);
    assert_eq!(chains.counters().missing_fragment_references_rejected(), 0);
    assert_eq!(
        chains.counters().missing_subdivision_references_rejected(),
        0
    );
    assert_eq!(
        chains.counters().mismatched_fragment_authority_rejected(),
        0
    );
}

fn assert_overlap_chain_rows_preserve_public_authority(chains: &PlanarBooleanOverlapEdgeChainSet) {
    let mut kind_counts = BTreeMap::<PlanarBooleanIntervalEventKind, usize>::new();
    let mut saw_reversed = false;
    for chain in chains.chains() {
        *kind_counts.entry(chain.interval_event_kind()).or_default() += 1;
        assert!(!chain.chain_identity().is_empty());
        assert!(!chain.interval_event_identity().is_empty());
        assert!(!chain.source_interval_identities().is_empty());
        assert!(!chain.normalized_interval_identities().is_empty());
        assert!(!chain.event_group_identities().is_empty());
        assert!(!chain.members().is_empty());
        for member in chain.members() {
            assert!(!member.member_identity().is_empty());
            assert!(!member.fragment_identity().is_empty());
            assert!(!member.interval_subdivision_identity().is_empty());
            assert!(!member.source_edge_identity().is_empty());
            assert!(!member.carrier_identity().is_empty());
            assert!(member.fragment_parameter_range()[0] < member.fragment_parameter_range()[1]);
            assert!(!member.source_interval_identity().is_empty());
            assert!(!member.normalized_interval_identity().is_empty());
            assert!(!member.local_frame_identity().is_empty());
            assert!(!member.precision_basis_identity().is_empty());
            assert!(!member.event_group_identities().is_empty());
            if member.source_sense() == PlanarBooleanSourceIntervalSense::Reversed {
                saw_reversed = true;
            }
        }
    }
    assert!(kind_counts.contains_key(&PlanarBooleanIntervalEventKind::PartialOverlap));
    assert!(kind_counts.contains_key(&PlanarBooleanIntervalEventKind::IdenticalAntiParallel));
    assert!(saw_reversed);
}
