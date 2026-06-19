use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanFragmentContinuationRow;

use super::counters::PlanarBooleanClosedWalkCandidateCounters;
use super::identity::{
    closed_walk_candidate_identity, closed_walk_candidate_set_identity,
    fragment_consumption_proof_identity,
};
use super::input::PlanarBooleanClosedWalkCandidateSetInput;
use super::product::{
    PlanarBooleanClosedWalkCandidateAssembly, PlanarBooleanClosedWalkCandidateSet,
};
use super::proof::{
    PlanarBooleanFragmentConsumptionProof, PlanarBooleanFragmentConsumptionProofRow,
};
use super::row::{PlanarBooleanClosedWalkCandidate, PlanarBooleanClosedWalkCandidateContinuation};

pub(crate) fn assemble_closed_walk_candidates(
    input: PlanarBooleanClosedWalkCandidateSetInput<'_>,
) -> PlanarBooleanClosedWalkCandidateAssembly {
    let request_identity = input.continuation_index().request_identity().to_string();
    let continuation_index_identity = input
        .continuation_index()
        .continuation_index_identity()
        .to_string();
    let rows = input.continuation_index().rows();
    let mut counters = PlanarBooleanClosedWalkCandidateCounters::default();
    let mut grouped_rows = BTreeMap::<String, Vec<&PlanarBooleanFragmentContinuationRow>>::new();
    for row in rows {
        counters.consumed_continuation_row();
        grouped_rows
            .entry(row.source_loop_identity().to_string())
            .or_default()
            .push(row);
    }

    let mut candidates = Vec::new();
    let mut proof_rows = Vec::new();
    for (source_loop_identity, rows) in grouped_rows {
        for component in connected_components(rows) {
            let candidate = build_candidate(
                &request_identity,
                &continuation_index_identity,
                source_loop_identity.clone(),
                component,
            );
            counters.assembled_walk_candidate();
            proof_rows.push(PlanarBooleanFragmentConsumptionProofRow::new(
                candidate.closed_walk_candidate_identity().to_string(),
                candidate.fragment_identities().to_vec(),
                candidate.split_vertex_identities().to_vec(),
                candidate
                    .continuations()
                    .iter()
                    .map(|continuation| continuation.continuation_identity().to_string())
                    .collect(),
            ));
            counters.emitted_fragment_consumption_row();
            candidates.push(candidate);
        }
    }

    candidates.sort_by(|left, right| {
        left.source_loop_identity()
            .cmp(right.source_loop_identity())
            .then_with(|| {
                left.closed_walk_candidate_identity()
                    .cmp(right.closed_walk_candidate_identity())
            })
    });
    proof_rows.sort_by(|left, right| {
        left.closed_walk_candidate_identity()
            .cmp(right.closed_walk_candidate_identity())
    });

    let candidate_set = PlanarBooleanClosedWalkCandidateSet::new(
        closed_walk_candidate_set_identity(
            &request_identity,
            &continuation_index_identity,
            &candidates,
        ),
        request_identity.clone(),
        continuation_index_identity.clone(),
        candidates,
        counters,
    );
    let fragment_consumption_proof = PlanarBooleanFragmentConsumptionProof::new(
        fragment_consumption_proof_identity(
            &request_identity,
            &continuation_index_identity,
            &proof_rows,
        ),
        request_identity,
        continuation_index_identity,
        proof_rows,
    );
    PlanarBooleanClosedWalkCandidateAssembly::new(candidate_set, fragment_consumption_proof)
}

fn connected_components(
    rows: Vec<&PlanarBooleanFragmentContinuationRow>,
) -> Vec<Vec<&PlanarBooleanFragmentContinuationRow>> {
    let mut indices_by_fragment = BTreeMap::<&str, Vec<usize>>::new();
    let mut indices_by_vertex = BTreeMap::<&str, Vec<usize>>::new();
    for (index, row) in rows.iter().enumerate() {
        indices_by_fragment
            .entry(row.fragment_identity())
            .or_default()
            .push(index);
        indices_by_vertex
            .entry(row.split_vertex_identity())
            .or_default()
            .push(index);
    }

    let mut visited = BTreeSet::new();
    let mut components = Vec::new();
    for start in 0..rows.len() {
        if !visited.insert(start) {
            continue;
        }
        let mut queue = VecDeque::from([start]);
        let mut component_indices = Vec::new();
        while let Some(index) = queue.pop_front() {
            component_indices.push(index);
            let row = rows[index];
            for neighbor in indices_by_fragment
                .get(row.fragment_identity())
                .into_iter()
                .flat_map(|neighbors| neighbors.iter().copied())
                .chain(
                    indices_by_vertex
                        .get(row.split_vertex_identity())
                        .into_iter()
                        .flat_map(|neighbors| neighbors.iter().copied()),
                )
            {
                if visited.insert(neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }
        let mut component = component_indices
            .into_iter()
            .map(|index| rows[index])
            .collect::<Vec<_>>();
        component.sort_by_key(|row| row.continuation_identity().to_string());
        components.push(component);
    }
    components
}

fn build_candidate(
    request_identity: &str,
    continuation_index_identity: &str,
    source_loop_identity: String,
    rows: Vec<&PlanarBooleanFragmentContinuationRow>,
) -> PlanarBooleanClosedWalkCandidate {
    let source_face_identities = unique_strings(rows.iter().map(|row| row.source_face_identity()));
    let source_loop_carrier_identities =
        unique_strings(rows.iter().map(|row| row.source_loop_carrier_identity()));
    let source_senses = unique_values(rows.iter().map(|row| row.source_sense()));
    let fragment_identities = unique_strings(rows.iter().map(|row| row.fragment_identity()));
    let split_vertex_identities =
        unique_strings(rows.iter().map(|row| row.split_vertex_identity()));
    let local_frame_identities = unique_strings(rows.iter().map(|row| row.local_frame_identity()));
    let precision_basis_identities =
        unique_strings(rows.iter().map(|row| row.precision_basis_identity()));
    let continuations = rows
        .iter()
        .map(|row| PlanarBooleanClosedWalkCandidateContinuation::from_continuation_row(row))
        .collect::<Vec<_>>();
    let continuation_identities = continuations
        .iter()
        .map(|continuation| continuation.continuation_identity().to_string())
        .collect::<Vec<_>>();
    let closed_walk_candidate_identity = closed_walk_candidate_identity(
        request_identity,
        continuation_index_identity,
        &source_loop_identity,
        &fragment_identities,
        &split_vertex_identities,
        &continuation_identities,
    );
    PlanarBooleanClosedWalkCandidate::new(
        closed_walk_candidate_identity,
        source_loop_identity,
        source_face_identities,
        source_loop_carrier_identities,
        source_senses,
        fragment_identities,
        split_vertex_identities,
        continuations,
        local_frame_identities,
        precision_basis_identities,
    )
}

fn unique_strings<'a>(values: impl Iterator<Item = &'a str>) -> Vec<String> {
    values
        .map(ToString::to_string)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn unique_values<T: Copy + Ord>(values: impl Iterator<Item = T>) -> Vec<T> {
    values.collect::<BTreeSet<_>>().into_iter().collect()
}
