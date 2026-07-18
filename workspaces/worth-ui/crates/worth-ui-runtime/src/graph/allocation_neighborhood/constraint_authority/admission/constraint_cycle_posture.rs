use crate::declaration::stable_text_digest;
use crate::evidence::{
    UiConstraintCycleParticipationPosture, UiConstraintPropagationDenial,
    UiConstraintPropagationDenialReason, UiConstraintPropagationEdge,
    UiConstraintPropagationEdgeFamily,
};

pub(super) fn admit_cycle_postures(
    edges: Vec<UiConstraintPropagationEdge>,
    admitted_cycle_families: &[UiConstraintPropagationEdgeFamily],
    neighborhood_identity_digest: u64,
    contract_identity_digest: u64,
) -> Result<Vec<UiConstraintPropagationEdge>, UiConstraintPropagationDenial> {
    let node_digests = unique_node_digests(&edges);
    let adjacency = adjacency_lists(&edges, &node_digests);
    let reachability = reachable_matrix(&adjacency);

    let mut classified = Vec::with_capacity(edges.len());
    for edge in edges {
        let posture = classify_cycle_posture(&edge, &node_digests, &reachability);
        if matches!(
            posture,
            UiConstraintCycleParticipationPosture::AdmittedFixedPoint
        ) && !admitted_cycle_families.contains(&edge.family())
        {
            return Err(UiConstraintPropagationDenial::new(
                UiConstraintPropagationDenialReason::UnsupportedCycleConvergence,
                neighborhood_identity_digest,
                contract_identity_digest,
                Some(edge.family()),
                edge.identity_digest()
                    ^ stable_text_digest("worth-ui.constraint-unsupported-cycle").rotate_left(7),
            ));
        }
        classified.push(edge.with_cycle_participation_posture(posture));
    }

    Ok(classified)
}

fn unique_node_digests(edges: &[UiConstraintPropagationEdge]) -> Vec<u64> {
    let mut node_digests = edges
        .iter()
        .flat_map(|edge| {
            [
                edge.source_member_identity_digest(),
                edge.target_member_identity_digest(),
            ]
        })
        .collect::<Vec<_>>();
    node_digests.sort_unstable();
    node_digests.dedup();
    node_digests
}

fn adjacency_lists(edges: &[UiConstraintPropagationEdge], node_digests: &[u64]) -> Vec<Vec<usize>> {
    let mut adjacency = vec![Vec::new(); node_digests.len()];
    for edge in edges {
        if edge.source_member_identity_digest() == edge.target_member_identity_digest() {
            continue;
        }
        let source_index = node_index(node_digests, edge.source_member_identity_digest());
        let target_index = node_index(node_digests, edge.target_member_identity_digest());
        adjacency[source_index].push(target_index);
    }
    adjacency
}

fn reachable_matrix(adjacency: &[Vec<usize>]) -> Vec<Vec<bool>> {
    let mut reachable = vec![vec![false; adjacency.len()]; adjacency.len()];
    for (start, reachable_from_start) in reachable.iter_mut().enumerate() {
        let mut stack = vec![start];
        while let Some(current) = stack.pop() {
            for &neighbor in &adjacency[current] {
                if !reachable_from_start[neighbor] {
                    reachable_from_start[neighbor] = true;
                    stack.push(neighbor);
                }
            }
        }
    }
    reachable
}

fn classify_cycle_posture(
    edge: &UiConstraintPropagationEdge,
    node_digests: &[u64],
    reachable: &[Vec<bool>],
) -> UiConstraintCycleParticipationPosture {
    if edge.source_member_identity_digest() == edge.target_member_identity_digest() {
        return UiConstraintCycleParticipationPosture::Acyclic;
    }

    let source_index = node_index(node_digests, edge.source_member_identity_digest());
    let target_index = node_index(node_digests, edge.target_member_identity_digest());
    if reachable[target_index][source_index] {
        UiConstraintCycleParticipationPosture::AdmittedFixedPoint
    } else {
        UiConstraintCycleParticipationPosture::Acyclic
    }
}

fn node_index(node_digests: &[u64], node_digest: u64) -> usize {
    node_digests
        .binary_search(&node_digest)
        .expect("constraint cycle classifier must resolve emitted node digests")
}
