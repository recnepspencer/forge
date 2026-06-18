use std::collections::{BTreeMap, BTreeSet};

use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanOverlapChainBoundaryRole, PlanarBooleanSplitEdgeFragment,
    PlanarBooleanSplitEdgeFragmentEndpointKind, PlanarBooleanSplitEdgeFragmentEndpointRef,
    PlanarBooleanSplitVertexIdentityRow,
};

use super::counters::PlanarBooleanFragmentContinuationCounters;
use super::denial::{
    PlanarBooleanFragmentContinuationDenial, PlanarBooleanFragmentContinuationDenialKind as Kind,
};
use super::identity::{
    continuation_identity, continuation_index_identity, continuation_neighborhood_identity,
    ordering_basis_from_identities,
};
use super::input::PlanarBooleanFragmentContinuationIndexInput;
use super::ordering::canonicalize_continuation_rows;
use super::product::PlanarBooleanFragmentContinuationIndex;
use super::row::{
    PlanarBooleanFragmentContinuationEndpointRole, PlanarBooleanFragmentContinuationRow,
};
use super::validation::validate_fragment_continuation_input;

pub(crate) fn build_fragment_continuation_index(
    input: PlanarBooleanFragmentContinuationIndexInput<'_>,
) -> Result<PlanarBooleanFragmentContinuationIndex, PlanarBooleanFragmentContinuationDenial> {
    let mut counters = PlanarBooleanFragmentContinuationCounters::default();
    validate_fragment_continuation_input(&input, &mut counters)?;
    let vertex_index = index_split_vertices(&input);
    let overlap_roles = collect_overlap_boundary_roles(&input);
    let rows =
        collect_fragment_continuation_rows(&input, &vertex_index, &overlap_roles, &mut counters)?;
    let mut canonical_rows = rows;
    let ordered_continuation_identities = canonicalize_continuation_rows(&mut canonical_rows);
    let continuation_index_identity = continuation_index_identity(
        input.request().request_identity(),
        input.source_provenance().bundle_identity(),
        &canonical_rows,
    );
    let ordering_basis = ordering_basis_from_identities(
        input.request().request_identity(),
        &continuation_index_identity,
        ordered_continuation_identities,
    );
    Ok(PlanarBooleanFragmentContinuationIndex::new(
        continuation_index_identity,
        input.request().request_identity().to_string(),
        input.source_provenance().bundle_identity().to_string(),
        input
            .split_vertices()
            .split_vertex_identity_set_identity()
            .to_string(),
        input.split_fragments().fragment_set_identity().to_string(),
        input.overlap_chains().chain_set_identity().to_string(),
        canonical_rows,
        ordering_basis,
        counters,
    ))
}

fn index_split_vertices<'a>(
    input: &'a PlanarBooleanFragmentContinuationIndexInput<'_>,
) -> BTreeMap<&'a str, &'a PlanarBooleanSplitVertexIdentityRow> {
    input
        .split_vertices()
        .vertices()
        .fold(BTreeMap::new(), |mut index, row| {
            index.insert(row.split_vertex_identity(), row);
            index
        })
}

fn collect_overlap_boundary_roles(
    input: &PlanarBooleanFragmentContinuationIndexInput<'_>,
) -> BTreeMap<String, Vec<PlanarBooleanOverlapChainBoundaryRole>> {
    let mut roles = BTreeMap::<String, Vec<PlanarBooleanOverlapChainBoundaryRole>>::new();
    for row in input.source_provenance().overlap_chain_lineage_map().rows() {
        for (fragment_identity, boundary_role) in row
            .fragment_identities()
            .iter()
            .zip(row.boundary_roles().iter().copied())
        {
            roles
                .entry(fragment_identity.clone())
                .or_default()
                .push(boundary_role);
        }
    }
    for fragment_roles in roles.values_mut() {
        fragment_roles.sort_by_key(|role| overlap_boundary_role_rank(*role));
    }
    roles
}

fn collect_fragment_continuation_rows(
    input: &PlanarBooleanFragmentContinuationIndexInput<'_>,
    vertex_index: &BTreeMap<&str, &PlanarBooleanSplitVertexIdentityRow>,
    overlap_roles: &BTreeMap<String, Vec<PlanarBooleanOverlapChainBoundaryRole>>,
    counters: &mut PlanarBooleanFragmentContinuationCounters,
) -> Result<Vec<PlanarBooleanFragmentContinuationRow>, PlanarBooleanFragmentContinuationDenial> {
    let membership_map = input.source_provenance().fragment_membership_map();
    let mut rows = Vec::new();
    let mut seen_slots = BTreeSet::new();
    for fragment in input.split_fragments().fragments() {
        let membership = membership_map
            .membership_for_fragment_identity(fragment.fragment_identity())
            .ok_or_else(|| {
                counters.rejected_dangling_reference();
                PlanarBooleanFragmentContinuationDenial::new(
                    Kind::MissingFragmentMembership,
                    fragment.fragment_identity(),
                    *counters,
                    "fragment continuation indexing requires source-loop membership for every split fragment",
                )
            })?;
        for source_sense in fragment.source_senses() {
            if let Some(row) = build_fragment_endpoint_continuation(
                input,
                fragment,
                membership,
                vertex_index,
                overlap_roles,
                *source_sense,
                fragment.start_endpoint(),
                PlanarBooleanFragmentContinuationEndpointRole::Start,
                counters,
            )? {
                register_continuation_row(row, &mut rows, &mut seen_slots, counters)?;
            }
            if let Some(row) = build_fragment_endpoint_continuation(
                input,
                fragment,
                membership,
                vertex_index,
                overlap_roles,
                *source_sense,
                fragment.end_endpoint(),
                PlanarBooleanFragmentContinuationEndpointRole::End,
                counters,
            )? {
                register_continuation_row(row, &mut rows, &mut seen_slots, counters)?;
            }
        }
    }
    Ok(rows)
}

#[allow(clippy::too_many_arguments)]
fn build_fragment_endpoint_continuation(
    input: &PlanarBooleanFragmentContinuationIndexInput<'_>,
    fragment: &PlanarBooleanSplitEdgeFragment,
    membership: &crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanFragmentMembershipRow,
    vertex_index: &BTreeMap<&str, &PlanarBooleanSplitVertexIdentityRow>,
    overlap_roles: &BTreeMap<String, Vec<PlanarBooleanOverlapChainBoundaryRole>>,
    source_sense: crate::workload_platform::planar_boolean_events::PlanarBooleanSourceIntervalSense,
    endpoint: &PlanarBooleanSplitEdgeFragmentEndpointRef,
    endpoint_role: PlanarBooleanFragmentContinuationEndpointRole,
    counters: &mut PlanarBooleanFragmentContinuationCounters,
) -> Result<Option<PlanarBooleanFragmentContinuationRow>, PlanarBooleanFragmentContinuationDenial> {
    if endpoint.endpoint_kind() != PlanarBooleanSplitEdgeFragmentEndpointKind::SplitVertex {
        return Ok(None);
    }
    let vertex = vertex_index
        .get(endpoint.endpoint_identity())
        .copied()
        .ok_or_else(|| {
            counters.rejected_dangling_reference();
            PlanarBooleanFragmentContinuationDenial::new(
                Kind::MissingSplitVertexBinding,
                endpoint.endpoint_identity(),
                *counters,
                "fragment continuation indexing requires every split-vertex fragment endpoint to bind a split vertex row",
            )
        })?;
    if vertex.source_edge_identity() != fragment.source_edge_identity()
        || vertex.carrier_identity() != fragment.carrier_identity()
        || vertex.normalized_parameter_bits() != endpoint.parameter_bits()
        || vertex.local_frame_identity() != fragment.local_frame_identity()
        || vertex.precision_basis_identity() != fragment.precision_basis_identity()
    {
        counters.rejected_foreign_lineage();
        return Err(PlanarBooleanFragmentContinuationDenial::new(
            Kind::MissingSplitVertexBinding,
            endpoint.endpoint_identity(),
            *counters,
            "fragment continuation indexing requires split-vertex authority to match the fragment endpoint exactly",
        ));
    }
    let neighborhood_identity = continuation_neighborhood_identity(
        input.request().request_identity(),
        vertex.split_vertex_identity(),
        membership.source_loop_identity(),
        source_sense,
    );
    let mut event_group_identities = fragment.event_group_identities().to_vec();
    event_group_identities.sort();
    let boundary_roles = overlap_roles
        .get(fragment.fragment_identity())
        .map(|roles| {
            let mut canonical_roles = roles.clone();
            canonical_roles.sort_by_key(|role| overlap_boundary_role_rank(*role));
            canonical_roles
        })
        .unwrap_or_default();
    let continuation_identity = continuation_identity(
        input.request().request_identity(),
        vertex.split_vertex_identity(),
        fragment.fragment_identity(),
        membership.source_loop_identity(),
        membership.source_face_identity(),
        fragment.source_edge_identity(),
        fragment.carrier_identity(),
        membership.source_loop_carrier_identity(),
        endpoint_role,
        source_sense,
        endpoint.parameter_bits(),
        fragment.parameter_range_bits(),
        fragment.local_frame_identity(),
        fragment.precision_basis_identity(),
        &event_group_identities,
        &boundary_roles,
    );
    Ok(Some(PlanarBooleanFragmentContinuationRow::new(
        continuation_identity,
        neighborhood_identity,
        vertex.split_vertex_identity().to_string(),
        fragment.fragment_identity().to_string(),
        membership.source_loop_identity().to_string(),
        membership.source_face_identity().to_string(),
        fragment.source_edge_identity().to_string(),
        fragment.carrier_identity().to_string(),
        membership.source_loop_carrier_identity().to_string(),
        endpoint_role,
        source_sense,
        endpoint.parameter_bits(),
        fragment.parameter_range_bits(),
        fragment.local_frame_identity().to_string(),
        fragment.precision_basis_identity().to_string(),
        event_group_identities,
        boundary_roles,
    )))
}

fn register_continuation_row(
    row: PlanarBooleanFragmentContinuationRow,
    rows: &mut Vec<PlanarBooleanFragmentContinuationRow>,
    seen_slots: &mut BTreeSet<String>,
    counters: &mut PlanarBooleanFragmentContinuationCounters,
) -> Result<(), PlanarBooleanFragmentContinuationDenial> {
    if !seen_slots.insert(row.continuation_identity().to_string()) {
        counters.rejected_duplicate_slot();
        return Err(PlanarBooleanFragmentContinuationDenial::new(
            Kind::DuplicateContinuationSlot,
            row.continuation_identity(),
            *counters,
            "fragment continuation indexing requires unique continuation slots before policy admission",
        ));
    }
    counters.indexed_fragment_continuation();
    rows.push(row);
    Ok(())
}

fn overlap_boundary_role_rank(role: PlanarBooleanOverlapChainBoundaryRole) -> u8 {
    match role {
        PlanarBooleanOverlapChainBoundaryRole::FullOverlapSpan => 0,
        PlanarBooleanOverlapChainBoundaryRole::OverlapStartBoundary => 1,
        PlanarBooleanOverlapChainBoundaryRole::OverlapInteriorFragment => 2,
        PlanarBooleanOverlapChainBoundaryRole::OverlapEndBoundary => 3,
    }
}
