use std::collections::BTreeMap;

use crate::runtime::{
    WorthUiIdentityMatchCounters, WorthUiIdentityMatchDenial, WorthUiIdentityMatchEdge,
    WorthUiIdentityMatchGraph, WorthUiMovedNodeIdentity, WorthUiRepeatedTemplateIdentity,
};

use super::types::IndexedIdentityNode;

pub(super) fn build_match_graph(
    active_index: BTreeMap<String, IndexedIdentityNode>,
    candidate_index: BTreeMap<String, IndexedIdentityNode>,
    mut counters: WorthUiIdentityMatchCounters,
) -> Result<WorthUiIdentityMatchGraph, WorthUiIdentityMatchDenial> {
    let mut matches = Vec::new();
    let mut repeated_template_identities = Vec::new();
    let mut moved_node_identities = Vec::new();

    for (identity_basis, active) in &active_index {
        let Some(candidate) = candidate_index.get(identity_basis) else {
            counters.record_unmatched_active();
            continue;
        };
        classify_cross_side_identity_kind_alignment(
            identity_basis,
            active,
            candidate,
            &mut counters,
        )?;
        counters.record_match_emitted();
        matches.push(WorthUiIdentityMatchEdge::new(
            active.node.handle().clone(),
            candidate.node.handle().clone(),
            identity_basis.to_owned(),
        ));
        if active.node.handle() != candidate.node.handle() {
            moved_node_identities.push(WorthUiMovedNodeIdentity::new(
                active.node.handle().clone(),
                candidate.node.handle().clone(),
                identity_basis.to_owned(),
            ));
        }
        if let Some(repeated) = WorthUiRepeatedTemplateIdentity::from_identity_basis(identity_basis)
        {
            repeated_template_identities.push(repeated);
        }
    }

    for identity_basis in candidate_index.keys() {
        if !active_index.contains_key(identity_basis) {
            counters.record_unmatched_candidate();
        }
    }

    Ok(WorthUiIdentityMatchGraph::new(
        active_index
            .into_values()
            .map(|indexed| indexed.node)
            .collect(),
        candidate_index
            .into_values()
            .map(|indexed| indexed.node)
            .collect(),
        matches,
        repeated_template_identities,
        moved_node_identities,
        counters,
    ))
}

/// Named classifier: active/candidate nodes sharing a seed must share identity kind.
fn classify_cross_side_identity_kind_alignment(
    identity_basis: &str,
    active: &IndexedIdentityNode,
    candidate: &IndexedIdentityNode,
    counters: &mut WorthUiIdentityMatchCounters,
) -> Result<(), WorthUiIdentityMatchDenial> {
    if active.node.kind() == candidate.node.kind() {
        return Ok(());
    }

    counters.record_identity_kind_mismatch();
    Err(WorthUiIdentityMatchDenial::IdentityKindMismatch {
        identity_basis: identity_basis.to_owned(),
        active_kind: active.node.kind(),
        candidate_kind: candidate.node.kind(),
        active_node_summary: active.node.node_summary(),
        candidate_node_summary: candidate.node.node_summary(),
        counters: *counters,
    })
}
