use crate::source::{
    WorthUiLegallyStructuredArtifactInput, WorthUiLegallyStructuredArtifactInputNode,
};

pub(crate) struct WorthUiLegallyStructuredArtifactInputEquivalentShape;

impl WorthUiLegallyStructuredArtifactInputEquivalentShape {
    pub(crate) fn packages_are_equivalent(
        left: &WorthUiLegallyStructuredArtifactInput,
        right: &WorthUiLegallyStructuredArtifactInput,
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
    left: &WorthUiLegallyStructuredArtifactInputNode,
    right: &WorthUiLegallyStructuredArtifactInputNode,
) -> bool {
    match (left, right) {
        (
            WorthUiLegallyStructuredArtifactInputNode::Import(left),
            WorthUiLegallyStructuredArtifactInputNode::Import(right),
        ) => left.target() == right.target(),
        (
            WorthUiLegallyStructuredArtifactInputNode::Page(left),
            WorthUiLegallyStructuredArtifactInputNode::Page(right),
        ) => {
            left.name_text() == right.name_text()
                && left.template_parameters() == right.template_parameters()
                && left.structure() == right.structure()
        }
        (
            WorthUiLegallyStructuredArtifactInputNode::Component(left),
            WorthUiLegallyStructuredArtifactInputNode::Component(right),
        ) => {
            left.component().id() == right.component().id() && left.structure() == right.structure()
        }
        (
            WorthUiLegallyStructuredArtifactInputNode::Surface(left),
            WorthUiLegallyStructuredArtifactInputNode::Surface(right),
        ) => left.surface().id() == right.surface().id() && left.structure() == right.structure(),
        (
            WorthUiLegallyStructuredArtifactInputNode::Binding(left),
            WorthUiLegallyStructuredArtifactInputNode::Binding(right),
        ) => {
            left.view_binding().id() == right.view_binding().id()
                && left.structure() == right.structure()
        }
        (
            WorthUiLegallyStructuredArtifactInputNode::Token(left),
            WorthUiLegallyStructuredArtifactInputNode::Token(right),
        ) => left.theme_token().id() == right.theme_token().id(),
        _ => false,
    }
}
