use std::collections::{BTreeMap, BTreeSet, VecDeque};

use super::counters::PlanarBooleanOverlapIslandComponentCounters;
use super::denial::PlanarBooleanOverlapIslandComponentDenial;
use super::identity::{component_set_identity, partition_identity};
use super::product::{
    PlanarBooleanAreaOverlapComponentSet, PlanarBooleanBoundaryContactComponentSet,
    PlanarBooleanOverlapIslandCandidateSet, PlanarBooleanOverlapIslandPartition,
    PlanarBooleanOverlapIslandSet,
};
use super::rows::{
    PlanarBooleanAreaOverlapComponentRow, PlanarBooleanBoundaryContactComponentRow,
    PlanarBooleanOverlapIslandCandidateKind::{AreaOverlap, BoundaryContact},
    PlanarBooleanOverlapIslandRow,
};
use super::validation::mixed_partition;

pub(super) fn build_island_partition(
    island_candidates: &PlanarBooleanOverlapIslandCandidateSet,
) -> Result<PlanarBooleanOverlapIslandPartition, PlanarBooleanOverlapIslandComponentDenial> {
    let mut counters = island_candidates.counters();
    let mut islands = Vec::new();
    let mut boundary_contact_components = Vec::new();
    let mut area_overlap_components = Vec::new();
    let candidates_by_island =
        island_candidates
            .rows()
            .iter()
            .fold(BTreeMap::<&str, Vec<_>>::new(), |mut map, row| {
                map.entry(row.island_identity()).or_default().push(row);
                map
            });

    for (island_identity, candidates) in candidates_by_island {
        let neighborhood_identity = candidates[0].neighborhood_identity().to_string();
        let mut boundary_component_ids = Vec::new();
        let mut area_component_ids = Vec::new();
        let mut propagated_names = candidates
            .iter()
            .flat_map(|candidate| {
                candidate
                    .propagated_persistent_name_identities()
                    .iter()
                    .cloned()
            })
            .collect::<Vec<_>>();
        propagated_names.sort();
        propagated_names.dedup();

        let connected_groups = connected_candidate_groups(&candidates);

        for group in connected_groups {
            let has_boundary_contact = group
                .iter()
                .any(|candidate| candidate.kind() == BoundaryContact);
            let has_area_overlap = group
                .iter()
                .any(|candidate| candidate.kind() == AreaOverlap);
            if has_boundary_contact && has_area_overlap {
                return Err(mixed_partition(island_identity, &mut counters));
            }

            let is_area_component = group[0].kind() == AreaOverlap;
            let component_identity = format!(
                "overlap-component:{}:{}:{}",
                if is_area_component {
                    "area"
                } else {
                    "boundary"
                },
                island_identity,
                component_group_key(&group)
            );
            let cell_identities = group
                .iter()
                .map(|candidate| candidate.cell_identity().to_string())
                .collect::<Vec<_>>();
            let boundary_component_identities = group
                .iter()
                .flat_map(|candidate| candidate.boundary_component_identities().iter().cloned())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            let boundary_segment_identities = group
                .iter()
                .flat_map(|candidate| candidate.boundary_segment_identities().iter().cloned())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            let source_loop_identities = group
                .iter()
                .flat_map(|candidate| candidate.source_loop_identities().iter().cloned())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();

            if is_area_component {
                area_component_ids.push(component_identity.clone());
                area_overlap_components.push(PlanarBooleanAreaOverlapComponentRow::new(
                    component_identity,
                    island_identity.to_string(),
                    neighborhood_identity.clone(),
                    cell_identities,
                    boundary_component_identities,
                    boundary_segment_identities,
                    source_loop_identities,
                ));
                counters.admitted_area_overlap_component();
            } else {
                boundary_component_ids.push(component_identity.clone());
                boundary_contact_components.push(PlanarBooleanBoundaryContactComponentRow::new(
                    component_identity,
                    island_identity.to_string(),
                    neighborhood_identity.clone(),
                    cell_identities,
                    boundary_component_identities,
                    boundary_segment_identities,
                    source_loop_identities,
                ));
                counters.admitted_boundary_contact_component();
            }
        }

        islands.push(PlanarBooleanOverlapIslandRow::new(
            island_identity.to_string(),
            neighborhood_identity,
            candidates
                .iter()
                .map(|candidate| candidate.candidate_identity().to_string())
                .collect(),
            candidates
                .iter()
                .map(|candidate| candidate.cell_identity().to_string())
                .collect(),
            boundary_component_ids,
            area_component_ids,
            propagated_names,
        ));
        counters.admitted_island();
    }

    let overlap_islands = PlanarBooleanOverlapIslandSet::new(
        format!(
            "overlap-island-set:{}:{}",
            island_candidates.request_identity(),
            islands.len()
        ),
        island_candidates.request_identity().to_string(),
        island_candidates.arrangement_graph_identity().to_string(),
        island_candidates.cell_set_identity().to_string(),
        island_candidates.ordering_basis_identity().to_string(),
        islands,
    );
    let boundary_contact_components = PlanarBooleanBoundaryContactComponentSet::new(
        component_set_identity(
            island_candidates.request_identity(),
            "boundary-contact",
            boundary_contact_components.len(),
        ),
        island_candidates.request_identity().to_string(),
        island_candidates.arrangement_graph_identity().to_string(),
        island_candidates.cell_set_identity().to_string(),
        island_candidates.ordering_basis_identity().to_string(),
        boundary_contact_components,
    );
    let area_overlap_components = PlanarBooleanAreaOverlapComponentSet::new(
        component_set_identity(
            island_candidates.request_identity(),
            "area-overlap",
            area_overlap_components.len(),
        ),
        island_candidates.request_identity().to_string(),
        island_candidates.arrangement_graph_identity().to_string(),
        island_candidates.cell_set_identity().to_string(),
        island_candidates.ordering_basis_identity().to_string(),
        area_overlap_components,
    );

    Ok(PlanarBooleanOverlapIslandPartition::new(
        partition_identity(
            island_candidates.request_identity(),
            overlap_islands.rows().len(),
        ),
        island_candidates.request_identity().to_string(),
        island_candidates.arrangement_graph_identity().to_string(),
        island_candidates.cell_set_identity().to_string(),
        island_candidates.ordering_basis_identity().to_string(),
        overlap_islands,
        boundary_contact_components,
        area_overlap_components,
        counters,
    ))
}

fn connected_candidate_groups<'a>(
    candidates: &[&'a super::rows::PlanarBooleanOverlapIslandCandidateRow],
) -> Vec<Vec<&'a super::rows::PlanarBooleanOverlapIslandCandidateRow>> {
    let mut visited = vec![false; candidates.len()];
    let mut groups = Vec::new();

    for start in 0..candidates.len() {
        if visited[start] {
            continue;
        }
        visited[start] = true;
        let mut queue = VecDeque::from([start]);
        let mut group = Vec::new();

        while let Some(index) = queue.pop_front() {
            let candidate = candidates[index];
            group.push(candidate);

            for neighbor in 0..candidates.len() {
                if visited[neighbor] || !shares_partition_basis(candidate, candidates[neighbor]) {
                    continue;
                }
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }

        group.sort_by(|left, right| left.candidate_identity().cmp(right.candidate_identity()));
        groups.push(group);
    }

    groups.sort_by_key(|group| component_group_key(group));
    groups
}

fn shares_partition_basis(
    left: &super::rows::PlanarBooleanOverlapIslandCandidateRow,
    right: &super::rows::PlanarBooleanOverlapIslandCandidateRow,
) -> bool {
    left.candidate_identity() == right.candidate_identity()
        || shares_any_identity(
            left.boundary_component_identities(),
            right.boundary_component_identities(),
        )
        || shares_any_identity(
            left.boundary_segment_identities(),
            right.boundary_segment_identities(),
        )
}

fn shares_any_identity(left: &[String], right: &[String]) -> bool {
    left.iter().any(|identity| {
        right
            .iter()
            .any(|candidate_identity| candidate_identity == identity)
    })
}

fn component_group_key(group: &[&super::rows::PlanarBooleanOverlapIslandCandidateRow]) -> String {
    group
        .iter()
        .map(|candidate| candidate.candidate_identity())
        .collect::<Vec<_>>()
        .join("+")
}
