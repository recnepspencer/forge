use crate::source::{WorthUiBoundArtifactInput, WorthUiBoundArtifactInputNode};

pub(crate) struct WorthUiBoundArtifactInputEquivalentShape;

impl WorthUiBoundArtifactInputEquivalentShape {
    pub(crate) fn packages_are_equivalent(
        left: &WorthUiBoundArtifactInput,
        right: &WorthUiBoundArtifactInput,
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
    left: &WorthUiBoundArtifactInputNode,
    right: &WorthUiBoundArtifactInputNode,
) -> bool {
    match (left, right) {
        (
            WorthUiBoundArtifactInputNode::Import(left),
            WorthUiBoundArtifactInputNode::Import(right),
        ) => left.target() == right.target(),
        (WorthUiBoundArtifactInputNode::Page(left), WorthUiBoundArtifactInputNode::Page(right)) => {
            left.name_text() == right.name_text()
                && left.template_parameters() == right.template_parameters()
                && left.structure() == right.structure()
        }
        (
            WorthUiBoundArtifactInputNode::Component(left),
            WorthUiBoundArtifactInputNode::Component(right),
        ) => {
            left.component().id() == right.component().id() && left.structure() == right.structure()
        }
        (
            WorthUiBoundArtifactInputNode::Surface(left),
            WorthUiBoundArtifactInputNode::Surface(right),
        ) => {
            left.surface().id() == right.surface().id()
                && left.structure() == right.structure()
                && left.semantics() == right.semantics()
        }
        (
            WorthUiBoundArtifactInputNode::Binding(left),
            WorthUiBoundArtifactInputNode::Binding(right),
        ) => {
            left.view_binding_reference().view_binding().id()
                == right.view_binding_reference().view_binding().id()
                && left.structure() == right.structure()
                && left.view_binding_reference() == right.view_binding_reference()
        }
        (
            WorthUiBoundArtifactInputNode::Token(left),
            WorthUiBoundArtifactInputNode::Token(right),
        ) => {
            left.theme_token().id() == right.theme_token().id()
                && left.semantics() == right.semantics()
        }
        _ => false,
    }
}
