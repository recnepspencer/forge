use crate::source::{WorthUiArtifact, WorthUiArtifactNode};

pub(super) fn is_query_binding_topology_only_difference(
    active: &WorthUiArtifact,
    candidate: &WorthUiArtifact,
) -> bool {
    if active.module_ids() != candidate.module_ids() {
        return false;
    }
    active.module_ids().iter().all(|module_id| {
        let Some(active_module) = active.module(module_id) else {
            return false;
        };
        let Some(candidate_module) = candidate.module(module_id) else {
            return false;
        };
        let active_non_query = active_module
            .nodes()
            .iter()
            .filter(|node| !matches!(node, WorthUiArtifactNode::Binding(_)));
        let candidate_non_query = candidate_module
            .nodes()
            .iter()
            .filter(|node| !matches!(node, WorthUiArtifactNode::Binding(_)));
        same_non_query_meaning(active_non_query, candidate_non_query)
    })
}

fn same_non_query_meaning<'artifact>(
    mut active: impl Iterator<Item = &'artifact WorthUiArtifactNode>,
    mut candidate: impl Iterator<Item = &'artifact WorthUiArtifactNode>,
) -> bool {
    loop {
        match (active.next(), candidate.next()) {
            (Some(left), Some(right))
                if left.has_same_semantic_meaning_ignoring_location(right) => {}
            (None, None) => return true,
            _ => return false,
        }
    }
}
