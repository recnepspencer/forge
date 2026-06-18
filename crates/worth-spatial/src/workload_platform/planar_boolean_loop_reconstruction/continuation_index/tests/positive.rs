use std::collections::BTreeMap;

use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanOverlapChainBoundaryRole, PlanarBooleanSplitEdgeFragmentEndpointKind,
};
use crate::workload_platform::planar_boolean_events::PlanarBooleanSourceIntervalSense;
use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::{
    prepared_loop_continuation_subject, LoopFixtureEntryOrder,
};

use super::super::{
    PlanarBooleanFragmentContinuationEndpointRole, PlanarBooleanFragmentContinuationIndex,
    PlanarBooleanFragmentContinuationIndexInput,
};

#[derive(Clone, Debug, Eq, PartialEq)]
struct ExpectedContinuationSemanticRow {
    split_vertex_identity: String,
    fragment_identity: String,
    source_loop_identity: String,
    source_face_identity: String,
    source_edge_identity: String,
    carrier_identity: String,
    source_loop_carrier_identity: String,
    fragment_endpoint_role: PlanarBooleanFragmentContinuationEndpointRole,
    source_sense: PlanarBooleanSourceIntervalSense,
    endpoint_parameter_bits: u64,
    fragment_parameter_range_bits: [u64; 2],
    local_frame_identity: String,
    precision_basis_identity: String,
    event_group_identities: Vec<String>,
    boundary_roles: Vec<PlanarBooleanOverlapChainBoundaryRole>,
}

#[test]
fn fragment_continuation_index_preserves_exact_neighborhood_membership_and_named_order() {
    let prepared = prepared_loop_continuation_subject(LoopFixtureEntryOrder::Canonical);
    let index = admit_index(&prepared);
    let expected_rows = expected_semantic_rows(&prepared);

    assert_eq!(index.rows().len(), expected_rows.len());
    assert_eq!(
        index.counters().fragment_continuations_indexed(),
        expected_rows.len()
    );

    let ordered_actual_rows = index
        .ordered_rows_with_basis()
        .map(|(row, key)| {
            assert_eq!(key.split_vertex_identity(), row.split_vertex_identity());
            assert_eq!(key.source_sense(), row.source_sense());
            assert_eq!(key.endpoint_parameter_bits(), row.endpoint_parameter_bits());
            assert_eq!(
                key.fragment_parameter_range_bits(),
                row.fragment_parameter_range_bits()
            );
            assert_eq!(key.fragment_endpoint_role(), row.fragment_endpoint_role());
            assert_eq!(key.source_loop_identity(), row.source_loop_identity());
            assert_eq!(key.fragment_identity(), row.fragment_identity());
            assert_eq!(key.source_edge_identity(), row.source_edge_identity());
            assert_eq!(key.carrier_identity(), row.carrier_identity());
            assert_eq!(key.continuation_identity(), row.continuation_identity());
            semantic_row(row)
        })
        .collect::<Vec<_>>();
    let mut expected_order = expected_rows.clone();
    sort_expected_rows(&mut expected_order);
    assert_eq!(ordered_actual_rows, expected_order);

    for ((split_vertex_identity, source_loop_identity, source_sense), expected_neighborhood_rows) in
        expected_rows_by_neighborhood(&expected_rows)
    {
        let neighborhood = index
            .neighborhood(
                split_vertex_identity.as_str(),
                source_loop_identity.as_str(),
                source_sense,
            )
            .expect("every expected continuation neighborhood should exist");
        assert!(!neighborhood.is_empty());
        assert_eq!(
            neighborhood.split_vertex_identity(),
            split_vertex_identity.as_str()
        );
        assert_eq!(
            neighborhood.source_loop_identity(),
            source_loop_identity.as_str()
        );
        assert_eq!(neighborhood.source_sense(), source_sense);

        let actual_rows = neighborhood.rows().map(semantic_row).collect::<Vec<_>>();
        assert_eq!(actual_rows, expected_neighborhood_rows);
        assert!(neighborhood
            .rows()
            .all(|row| row.neighborhood_identity() == neighborhood.neighborhood_identity()));
    }
}

#[test]
fn fragment_continuation_index_identity_and_ordering_are_replay_stable() {
    let canonical = prepared_loop_continuation_subject(LoopFixtureEntryOrder::Canonical);
    let replayed = prepared_loop_continuation_subject(LoopFixtureEntryOrder::Replayed);

    let canonical_index = admit_index(&canonical);
    let replayed_index = admit_index(&replayed);

    assert_eq!(
        canonical_index.continuation_index_identity(),
        replayed_index.continuation_index_identity()
    );
    assert_eq!(
        canonical_index.ordering_basis().basis_identity(),
        replayed_index.ordering_basis().basis_identity()
    );
    assert_eq!(
        canonical_index
            .ordering_basis()
            .ordered_continuation_identities(),
        replayed_index
            .ordering_basis()
            .ordered_continuation_identities()
    );
    assert_eq!(canonical_index.counters(), replayed_index.counters());
    assert_eq!(
        canonical_index
            .ordered_rows_with_basis()
            .map(|(row, _)| semantic_row(row))
            .collect::<Vec<_>>(),
        replayed_index
            .ordered_rows_with_basis()
            .map(|(row, _)| semantic_row(row))
            .collect::<Vec<_>>()
    );
}

fn admit_index(
    prepared: &crate::workload_platform::planar_boolean_loop_reconstruction::test_support::PreparedLoopContinuationIndexSubject,
) -> PlanarBooleanFragmentContinuationIndex {
    PlanarBooleanFragmentContinuationIndex::admit(
        PlanarBooleanFragmentContinuationIndexInput::from_request_and_provenance(
            &prepared.request,
            &prepared.source_provenance,
            &prepared.subject.vertices,
            &prepared.subject.fragments,
            &prepared.subject.overlap_chains,
        ),
    )
    .expect("continuation index should admit")
}

fn expected_semantic_rows(
    prepared: &crate::workload_platform::planar_boolean_loop_reconstruction::test_support::PreparedLoopContinuationIndexSubject,
) -> Vec<ExpectedContinuationSemanticRow> {
    let split_vertices = prepared
        .subject
        .vertices
        .vertices()
        .map(|vertex| (vertex.split_vertex_identity().to_string(), vertex))
        .collect::<BTreeMap<_, _>>();
    let mut overlap_roles = BTreeMap::<String, Vec<PlanarBooleanOverlapChainBoundaryRole>>::new();
    for row in prepared
        .source_provenance
        .overlap_chain_lineage_map()
        .rows()
    {
        for (fragment_identity, boundary_role) in row
            .fragment_identities()
            .iter()
            .zip(row.boundary_roles().iter().copied())
        {
            overlap_roles
                .entry(fragment_identity.clone())
                .or_default()
                .push(boundary_role);
        }
    }

    let mut rows = Vec::new();
    for fragment in prepared.subject.fragments.fragments() {
        let membership = prepared
            .source_provenance
            .fragment_membership_map()
            .membership_for_fragment_identity(fragment.fragment_identity())
            .expect("prepared subject should keep fragment membership aligned");
        for source_sense in fragment.source_senses() {
            collect_expected_endpoint_row(
                &mut rows,
                &split_vertices,
                &overlap_roles,
                fragment,
                membership,
                *source_sense,
                fragment.start_endpoint(),
                PlanarBooleanFragmentContinuationEndpointRole::Start,
            );
            collect_expected_endpoint_row(
                &mut rows,
                &split_vertices,
                &overlap_roles,
                fragment,
                membership,
                *source_sense,
                fragment.end_endpoint(),
                PlanarBooleanFragmentContinuationEndpointRole::End,
            );
        }
    }
    rows
}

#[allow(clippy::too_many_arguments)]
fn collect_expected_endpoint_row(
    rows: &mut Vec<ExpectedContinuationSemanticRow>,
    split_vertices: &BTreeMap<String, &crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanSplitVertexIdentityRow>,
    overlap_roles: &BTreeMap<String, Vec<PlanarBooleanOverlapChainBoundaryRole>>,
    fragment: &crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanSplitEdgeFragment,
    membership: &crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanFragmentMembershipRow,
    source_sense: PlanarBooleanSourceIntervalSense,
    endpoint: &crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanSplitEdgeFragmentEndpointRef,
    endpoint_role: PlanarBooleanFragmentContinuationEndpointRole,
) {
    if endpoint.endpoint_kind() != PlanarBooleanSplitEdgeFragmentEndpointKind::SplitVertex {
        return;
    }
    let vertex = split_vertices
        .get(endpoint.endpoint_identity())
        .expect("split endpoint should bind a real split vertex");
    assert_eq!(
        vertex.source_edge_identity(),
        fragment.source_edge_identity()
    );
    assert_eq!(vertex.carrier_identity(), fragment.carrier_identity());
    assert_eq!(
        vertex.normalized_parameter_bits(),
        endpoint.parameter_bits()
    );

    let mut event_group_identities = fragment.event_group_identities().to_vec();
    event_group_identities.sort();
    let mut boundary_roles = overlap_roles
        .get(fragment.fragment_identity())
        .cloned()
        .unwrap_or_default();
    boundary_roles.sort_by_key(|role| boundary_role_rank(*role));

    rows.push(ExpectedContinuationSemanticRow {
        split_vertex_identity: vertex.split_vertex_identity().to_string(),
        fragment_identity: fragment.fragment_identity().to_string(),
        source_loop_identity: membership.source_loop_identity().to_string(),
        source_face_identity: membership.source_face_identity().to_string(),
        source_edge_identity: fragment.source_edge_identity().to_string(),
        carrier_identity: fragment.carrier_identity().to_string(),
        source_loop_carrier_identity: membership.source_loop_carrier_identity().to_string(),
        fragment_endpoint_role: endpoint_role,
        source_sense,
        endpoint_parameter_bits: endpoint.parameter_bits(),
        fragment_parameter_range_bits: fragment.parameter_range_bits(),
        local_frame_identity: fragment.local_frame_identity().to_string(),
        precision_basis_identity: fragment.precision_basis_identity().to_string(),
        event_group_identities,
        boundary_roles,
    });
}

fn expected_rows_by_neighborhood(
    expected_rows: &[ExpectedContinuationSemanticRow],
) -> BTreeMap<
    (String, String, PlanarBooleanSourceIntervalSense),
    Vec<ExpectedContinuationSemanticRow>,
> {
    let mut neighborhoods = BTreeMap::new();
    for row in expected_rows {
        neighborhoods
            .entry((
                row.split_vertex_identity.clone(),
                row.source_loop_identity.clone(),
                row.source_sense,
            ))
            .or_insert_with(Vec::new)
            .push(row.clone());
    }
    for rows in neighborhoods.values_mut() {
        sort_expected_rows(rows);
    }
    neighborhoods
}

fn semantic_row(
    row: &crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanFragmentContinuationRow,
) -> ExpectedContinuationSemanticRow {
    let mut event_group_identities = row.event_group_identities().to_vec();
    event_group_identities.sort();
    let mut boundary_roles = row.boundary_roles().to_vec();
    boundary_roles.sort_by_key(|role| boundary_role_rank(*role));
    ExpectedContinuationSemanticRow {
        split_vertex_identity: row.split_vertex_identity().to_string(),
        fragment_identity: row.fragment_identity().to_string(),
        source_loop_identity: row.source_loop_identity().to_string(),
        source_face_identity: row.source_face_identity().to_string(),
        source_edge_identity: row.source_edge_identity().to_string(),
        carrier_identity: row.carrier_identity().to_string(),
        source_loop_carrier_identity: row.source_loop_carrier_identity().to_string(),
        fragment_endpoint_role: row.fragment_endpoint_role(),
        source_sense: row.source_sense(),
        endpoint_parameter_bits: row.endpoint_parameter_bits(),
        fragment_parameter_range_bits: row.fragment_parameter_range_bits(),
        local_frame_identity: row.local_frame_identity().to_string(),
        precision_basis_identity: row.precision_basis_identity().to_string(),
        event_group_identities,
        boundary_roles,
    }
}

fn sort_expected_rows(rows: &mut [ExpectedContinuationSemanticRow]) {
    rows.sort_by(|left, right| {
        left.split_vertex_identity
            .cmp(&right.split_vertex_identity)
            .then_with(|| left.source_sense.cmp(&right.source_sense))
            .then_with(|| {
                left.endpoint_parameter_bits
                    .cmp(&right.endpoint_parameter_bits)
            })
            .then_with(|| {
                left.fragment_parameter_range_bits[0].cmp(&right.fragment_parameter_range_bits[0])
            })
            .then_with(|| {
                left.fragment_parameter_range_bits[1].cmp(&right.fragment_parameter_range_bits[1])
            })
            .then_with(|| {
                left.fragment_endpoint_role
                    .cmp(&right.fragment_endpoint_role)
            })
            .then_with(|| left.source_loop_identity.cmp(&right.source_loop_identity))
            .then_with(|| left.fragment_identity.cmp(&right.fragment_identity))
            .then_with(|| left.source_edge_identity.cmp(&right.source_edge_identity))
            .then_with(|| left.carrier_identity.cmp(&right.carrier_identity))
    });
}

fn boundary_role_rank(role: PlanarBooleanOverlapChainBoundaryRole) -> u8 {
    match role {
        PlanarBooleanOverlapChainBoundaryRole::FullOverlapSpan => 0,
        PlanarBooleanOverlapChainBoundaryRole::OverlapStartBoundary => 1,
        PlanarBooleanOverlapChainBoundaryRole::OverlapInteriorFragment => 2,
        PlanarBooleanOverlapChainBoundaryRole::OverlapEndBoundary => 3,
    }
}
