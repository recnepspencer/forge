use crate::source::{WorthUiIdentitySeededArtifactInput, WorthUiIdentitySeededArtifactInputNode};

pub(crate) struct WorthUiIdentitySeededArtifactInputEquivalentShape;

impl WorthUiIdentitySeededArtifactInputEquivalentShape {
    pub(crate) fn packages_are_equivalent(
        left: &WorthUiIdentitySeededArtifactInput,
        right: &WorthUiIdentitySeededArtifactInput,
    ) -> bool {
        left.module_ids() == right.module_ids()
            && left.module_ids().iter().all(|module_id| {
                left.module(module_id)
                    .zip(right.module(module_id))
                    .is_some_and(|(left_module, right_module)| {
                        left_module.nodes().len() == right_module.nodes().len()
                            && left_module
                                .nodes()
                                .iter()
                                .zip(right_module.nodes().iter())
                                .all(|(left_node, right_node)| {
                                    nodes_are_equivalent(left_node, right_node)
                                })
                    })
            })
    }
}

fn nodes_are_equivalent(
    left: &WorthUiIdentitySeededArtifactInputNode,
    right: &WorthUiIdentitySeededArtifactInputNode,
) -> bool {
    match (left, right) {
        (
            WorthUiIdentitySeededArtifactInputNode::Import(left),
            WorthUiIdentitySeededArtifactInputNode::Import(right),
        ) => left == right,
        (
            WorthUiIdentitySeededArtifactInputNode::Page(left),
            WorthUiIdentitySeededArtifactInputNode::Page(right),
        ) => left == right,
        (
            WorthUiIdentitySeededArtifactInputNode::Component(left),
            WorthUiIdentitySeededArtifactInputNode::Component(right),
        ) => left == right,
        (
            WorthUiIdentitySeededArtifactInputNode::Surface(left),
            WorthUiIdentitySeededArtifactInputNode::Surface(right),
        ) => left == right,
        (
            WorthUiIdentitySeededArtifactInputNode::Binding(left),
            WorthUiIdentitySeededArtifactInputNode::Binding(right),
        ) => left == right,
        (
            WorthUiIdentitySeededArtifactInputNode::Token(left),
            WorthUiIdentitySeededArtifactInputNode::Token(right),
        ) => left == right,
        _ => false,
    }
}
