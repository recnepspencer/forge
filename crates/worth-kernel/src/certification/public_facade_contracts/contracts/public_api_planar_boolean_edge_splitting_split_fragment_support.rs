use super::edge_splitting_split_vertex_identity_support::build_interval_subdivision_schedule_for_metaboss;
use super::metaboss_support::MetabossEventExtractionSubject;
use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanSplitEdgeFragmentEndpointKind, PlanarBooleanSplitEdgeFragmentSet,
};

pub(crate) fn assert_split_edge_fragments_match_metaboss(subject: &MetabossEventExtractionSubject) {
    let interval_normalized = build_interval_subdivision_schedule_for_metaboss(subject);
    let split_vertices = interval_normalized
        .mint_split_vertex_identities()
        .expect("metaboss split vertices should mint before fragments");
    let fragments = interval_normalized
        .build_split_edge_fragments(&split_vertices)
        .expect("metaboss split edge fragments should build from split vertices");

    assert_eq!(
        fragments.interval_subdivision_schedule_set_identity(),
        interval_normalized.schedule_set_identity()
    );
    assert_eq!(
        fragments.split_vertex_identity_set_identity(),
        split_vertices.split_vertex_identity_set_identity()
    );
    assert!(fragments.certifies_domain_coverage());
    assert_split_fragment_counters_reconcile(&fragments);
    assert_split_fragment_rows_preserve_public_authority(&fragments);
}

fn assert_split_fragment_counters_reconcile(fragments: &PlanarBooleanSplitEdgeFragmentSet) {
    let counters = fragments.counters();
    assert_eq!(counters.schedules_inspected(), fragments.schedules().len());
    assert_eq!(counters.source_edges_covered(), fragments.schedules().len());
    assert_eq!(counters.fragments_emitted(), fragments.fragments().count());
    assert!(counters.original_endpoint_boundaries_synthesized() >= fragments.schedules().len() * 2);
    assert_eq!(counters.collapsed_fragments_rejected(), 0);
    assert_eq!(counters.coverage_gaps_rejected(), 0);
    assert_eq!(counters.foreign_schedule_rows_rejected(), 0);
}

fn assert_split_fragment_rows_preserve_public_authority(
    fragments: &PlanarBooleanSplitEdgeFragmentSet,
) {
    assert!(fragments.fragments().any(|fragment| {
        !fragment.interval_subdivision_identities().is_empty()
            && !fragment.normalized_interval_identities().is_empty()
    }));
    for schedule in fragments.schedules() {
        let rows = schedule.fragments();
        assert_eq!(
            rows.first()
                .expect("each source edge emits fragments")
                .start_endpoint()
                .endpoint_kind(),
            PlanarBooleanSplitEdgeFragmentEndpointKind::OriginalSourceStart
        );
        assert_eq!(
            rows.last()
                .expect("each source edge emits fragments")
                .end_endpoint()
                .endpoint_kind(),
            PlanarBooleanSplitEdgeFragmentEndpointKind::OriginalSourceEnd
        );
        for fragment in rows {
            assert_eq!(
                fragment.source_edge_identity(),
                schedule.source_edge_identity()
            );
            assert_eq!(fragment.carrier_identity(), schedule.carrier_identity());
            assert!(fragment.parameter_range()[0] < fragment.parameter_range()[1]);
            assert!(!fragment.fragment_identity().is_empty());
            assert!(!fragment.local_frame_identity().is_empty());
            assert!(!fragment.precision_basis_identity().is_empty());
            assert!(!fragment.source_senses().is_empty());
        }
    }
}
