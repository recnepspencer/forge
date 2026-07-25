use crate::source::{WorthUiArtifactNode, WorthUiDurableStateEligibility};

pub(super) fn node_identity_seed(
    node: &WorthUiArtifactNode,
) -> &crate::source::WorthUiArtifactIdentitySeed {
    match node {
        WorthUiArtifactNode::Import(node) => node.identity_seed(),
        WorthUiArtifactNode::Component(node) => node.identity_seed(),
        WorthUiArtifactNode::Surface(node) => node.identity_seed(),
        WorthUiArtifactNode::Binding(node) => node.identity_seed(),
        WorthUiArtifactNode::Token(node) => node.identity_seed(),
    }
}

pub(super) fn node_durable_state_eligibility(
    node: &WorthUiArtifactNode,
) -> &WorthUiDurableStateEligibility {
    match node {
        WorthUiArtifactNode::Import(node) => node.durable_state_eligibility(),
        WorthUiArtifactNode::Component(node) => node.durable_state_eligibility(),
        WorthUiArtifactNode::Surface(node) => node.durable_state_eligibility(),
        WorthUiArtifactNode::Binding(node) => node.durable_state_eligibility(),
        WorthUiArtifactNode::Token(node) => node.durable_state_eligibility(),
    }
}

pub(super) fn durable_state_is_eligible(eligibility: &WorthUiDurableStateEligibility) -> bool {
    matches!(eligibility, WorthUiDurableStateEligibility::Eligible { .. })
}
