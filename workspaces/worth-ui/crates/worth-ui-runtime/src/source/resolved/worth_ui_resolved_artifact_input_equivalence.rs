use crate::source::{WorthUiResolvedArtifactInput, WorthUiResolvedArtifactInputNode};

pub(crate) struct WorthUiResolvedArtifactInputEquivalentShape;

impl WorthUiResolvedArtifactInputEquivalentShape {
    pub(crate) fn packages_are_equivalent(
        left: &WorthUiResolvedArtifactInput,
        right: &WorthUiResolvedArtifactInput,
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
    left: &WorthUiResolvedArtifactInputNode,
    right: &WorthUiResolvedArtifactInputNode,
) -> bool {
    match (left, right) {
        (
            WorthUiResolvedArtifactInputNode::Import(left),
            WorthUiResolvedArtifactInputNode::Import(right),
        ) => left.target() == right.target(),
        (
            WorthUiResolvedArtifactInputNode::Component(left),
            WorthUiResolvedArtifactInputNode::Component(right),
        ) => {
            left.component().id() == right.component().id() && left.structure() == right.structure()
        }
        (
            WorthUiResolvedArtifactInputNode::Surface(left),
            WorthUiResolvedArtifactInputNode::Surface(right),
        ) => left.surface().id() == right.surface().id() && left.structure() == right.structure(),
        (
            WorthUiResolvedArtifactInputNode::Binding(left),
            WorthUiResolvedArtifactInputNode::Binding(right),
        ) => {
            left.view_binding().id() == right.view_binding().id()
                && left.structure() == right.structure()
        }
        (
            WorthUiResolvedArtifactInputNode::Token(left),
            WorthUiResolvedArtifactInputNode::Token(right),
        ) => left.theme_token().id() == right.theme_token().id(),
        _ => false,
    }
}
