use std::collections::{BTreeMap, BTreeSet};

use super::{OwnerPlanNode, OwnerPlanNodeIdentity, OwnerPlanPrerequisite};

pub(super) fn first_irreversible_node_in_execution_order(
    nodes: &[OwnerPlanNode],
    edges: &[OwnerPlanPrerequisite],
) -> Option<OwnerPlanNodeIdentity> {
    let mut indegree = nodes
        .iter()
        .map(|node| (node.identity(), 0_usize))
        .collect::<BTreeMap<_, _>>();
    let mut outgoing = BTreeMap::<OwnerPlanNodeIdentity, Vec<OwnerPlanNodeIdentity>>::new();
    for edge in edges {
        *indegree.get_mut(&edge.dependent())? += 1;
        outgoing
            .entry(edge.prerequisite())
            .or_default()
            .push(edge.dependent());
    }
    let mut ready = indegree
        .iter()
        .filter_map(|(identity, degree)| (*degree == 0).then_some(*identity))
        .collect::<BTreeSet<_>>();
    while let Some(identity) = ready.pop_first() {
        let node = nodes.iter().find(|node| node.identity() == identity)?;
        if node.irreversible() {
            return Some(identity);
        }
        for dependent in outgoing.get(&identity).into_iter().flatten() {
            let degree = indegree.get_mut(dependent)?;
            *degree = degree.checked_sub(1)?;
            if *degree == 0 {
                ready.insert(*dependent);
            }
        }
    }
    None
}
