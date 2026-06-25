use worth_ui::facade::{
    WorthUiCompositionNodeKind, WorthUiMountedCompositionTreeReceipt,
    WorthUiMountedControlNodeReceipt, WorthUiMountedInteractionNodeReceipt,
    WorthUiMountedNodeReceipt,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExpectedMountedChildKind {
    Control,
    Interaction,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExpectedMountedChild {
    kind: ExpectedMountedChildKind,
    subject_id: &'static str,
    sizing: &'static str,
}

pub fn expected_control(subject_id: &'static str, sizing: &'static str) -> ExpectedMountedChild {
    ExpectedMountedChild {
        kind: ExpectedMountedChildKind::Control,
        subject_id,
        sizing,
    }
}

pub fn expected_interaction(
    subject_id: &'static str,
    sizing: &'static str,
) -> ExpectedMountedChild {
    ExpectedMountedChild {
        kind: ExpectedMountedChildKind::Interaction,
        subject_id,
        sizing,
    }
}

pub fn assert_container_children(
    tree: &WorthUiMountedCompositionTreeReceipt,
    parent_id: &str,
    expected: &[ExpectedMountedChild],
) {
    let children = tree.ordered_children(parent_id);
    assert_eq!(
        children.len(),
        expected.len(),
        "container '{parent_id}' mounted unexpected child count"
    );
    for (order, (child, expected_child)) in children.iter().zip(expected).enumerate() {
        assert_eq!(
            child.composition_node().kind(),
            expected_child.node_kind(),
            "container '{parent_id}' child {order} has wrong node kind"
        );
        assert_eq!(
            child.composition_node().authority_identity(),
            expected_child.subject_id,
            "container '{parent_id}' child {order} has wrong subject identity"
        );
        assert_eq!(
            child.order(),
            order as u32,
            "container '{parent_id}' child {order} has wrong composition order"
        );
        assert_mounted_child_matches(parent_id, order, child.mounted_node(), expected_child);
    }
}

pub fn assert_container_excludes_subject(
    tree: &WorthUiMountedCompositionTreeReceipt,
    parent_id: &str,
    subject_id: &str,
) {
    assert!(
        tree.ordered_children(parent_id)
            .iter()
            .all(|child| child.composition_node().authority_identity() != subject_id),
        "container '{parent_id}' unexpectedly contains subject '{subject_id}'"
    );
}

pub fn mounted_control_child<'a>(
    tree: &'a WorthUiMountedCompositionTreeReceipt,
    control_id: &str,
) -> Option<&'a WorthUiMountedControlNodeReceipt> {
    fn visit<'a>(
        tree: &'a WorthUiMountedCompositionTreeReceipt,
        parent_id: &str,
        control_id: &str,
    ) -> Option<&'a WorthUiMountedControlNodeReceipt> {
        for child in tree.ordered_children(parent_id) {
            if let WorthUiMountedNodeReceipt::Control(node) = child.mounted_node() {
                if node.control_id() == control_id {
                    return Some(node);
                }
            }
            if let Some(node) = visit(tree, child.node_id(), control_id) {
                return Some(node);
            }
        }
        None
    }
    visit(tree, tree.root().root_id().as_str(), control_id)
}

pub fn mounted_interaction_child<'a>(
    tree: &'a WorthUiMountedCompositionTreeReceipt,
    interaction_id: &str,
) -> Option<&'a WorthUiMountedInteractionNodeReceipt> {
    fn visit<'a>(
        tree: &'a WorthUiMountedCompositionTreeReceipt,
        parent_id: &str,
        interaction_id: &str,
    ) -> Option<&'a WorthUiMountedInteractionNodeReceipt> {
        for child in tree.ordered_children(parent_id) {
            if let WorthUiMountedNodeReceipt::Interaction(node) = child.mounted_node() {
                if node.interaction().interaction_id() == interaction_id {
                    return Some(node);
                }
            }
            if let Some(node) = visit(tree, child.node_id(), interaction_id) {
                return Some(node);
            }
        }
        None
    }
    visit(tree, tree.root().root_id().as_str(), interaction_id)
}

impl ExpectedMountedChild {
    fn node_kind(self) -> WorthUiCompositionNodeKind {
        match self.kind {
            ExpectedMountedChildKind::Control => WorthUiCompositionNodeKind::Control,
            ExpectedMountedChildKind::Interaction => WorthUiCompositionNodeKind::Interaction,
        }
    }
}

fn assert_mounted_child_matches(
    parent_id: &str,
    order: usize,
    node: &WorthUiMountedNodeReceipt,
    expected_child: &ExpectedMountedChild,
) {
    match (expected_child.kind, node) {
        (ExpectedMountedChildKind::Control, WorthUiMountedNodeReceipt::Control(control)) => {
            assert_eq!(control.control_id(), expected_child.subject_id);
            assert_eq!(control.composition_child_binding().parent_id(), parent_id);
            assert_eq!(control.composition_child_binding().order(), order as u32);
            assert_eq!(
                control.composition_child_binding().sizing_token(),
                expected_child.sizing
            );
        }
        (
            ExpectedMountedChildKind::Interaction,
            WorthUiMountedNodeReceipt::Interaction(interaction),
        ) => {
            assert_eq!(
                interaction.interaction().interaction_id(),
                expected_child.subject_id
            );
            assert_eq!(
                interaction.composition_child_binding().parent_id(),
                parent_id
            );
            assert_eq!(
                interaction.composition_child_binding().order(),
                order as u32
            );
            assert_eq!(
                interaction.composition_child_binding().sizing_token(),
                expected_child.sizing
            );
        }
        _ => panic!("container '{parent_id}' child {order} mounted wrong receipt kind"),
    }
}
