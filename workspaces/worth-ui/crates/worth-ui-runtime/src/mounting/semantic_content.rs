use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiMountedSemanticContentInput {
    by_graph_node: BTreeMap<crate::graph::UiGraphNodeIdentity, UiMountedSemanticTextContent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum UiMountedSemanticTextContent {
    Scalar(UiMountedScalarSemanticTextContent),
    Collection(UiMountedCollectionSemanticTextContent),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiMountedScalarSemanticTextContent {
    value: UiMountedSemanticTextValueDirective,
    posture: Arc<str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiMountedCollectionSemanticTextContent {
    value: UiMountedCollectionTextDirective,
    posture: Arc<str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum UiMountedSemanticTextValueDirective {
    Replace(Arc<str>),
    Preserve,
    Clear,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum UiMountedCollectionTextDirective {
    Replace(Box<[UiMountedCollectionTextRow]>),
    Patch(Box<[UiMountedCollectionTextChange]>),
    Preserve,
    Clear,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiMountedCollectionTextRow {
    identity: Arc<str>,
    selected_values: Box<[Arc<str>]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum UiMountedCollectionTextChange {
    Insert {
        row: UiMountedCollectionTextRow,
        at: usize,
    },
    Remove {
        identity: Arc<str>,
        from: usize,
    },
    Move {
        identity: Arc<str>,
        from: usize,
        to: usize,
    },
    Update(UiMountedCollectionTextRow),
    WindowShift,
}

impl UiMountedSemanticContentInput {
    pub(crate) fn empty() -> Self {
        Self {
            by_graph_node: BTreeMap::new(),
        }
    }

    pub(crate) fn insert_scalar(
        &mut self,
        graph_node: crate::graph::UiGraphNodeIdentity,
        value: UiMountedSemanticTextValueDirective,
        posture: Arc<str>,
    ) -> Result<(), ()> {
        self.insert(
            graph_node,
            UiMountedSemanticTextContent::Scalar(UiMountedScalarSemanticTextContent {
                value,
                posture,
            }),
        )
    }

    pub(crate) fn insert_collection(
        &mut self,
        graph_node: crate::graph::UiGraphNodeIdentity,
        value: UiMountedCollectionTextDirective,
        posture: Arc<str>,
    ) -> Result<(), ()> {
        self.insert(
            graph_node,
            UiMountedSemanticTextContent::Collection(UiMountedCollectionSemanticTextContent {
                value,
                posture,
            }),
        )
    }

    fn insert(
        &mut self,
        graph_node: crate::graph::UiGraphNodeIdentity,
        content: UiMountedSemanticTextContent,
    ) -> Result<(), ()> {
        if self.by_graph_node.insert(graph_node, content).is_some() {
            return Err(());
        }
        Ok(())
    }

    pub(crate) fn get(
        &self,
        graph_node: crate::graph::UiGraphNodeIdentity,
    ) -> Option<&UiMountedSemanticTextContent> {
        self.by_graph_node.get(&graph_node)
    }

    pub(crate) fn graph_nodes(
        &self,
    ) -> impl ExactSizeIterator<Item = crate::graph::UiGraphNodeIdentity> + '_ {
        self.by_graph_node.keys().copied()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.by_graph_node.is_empty()
    }
}

impl UiMountedScalarSemanticTextContent {
    pub(crate) const fn value(&self) -> &UiMountedSemanticTextValueDirective {
        &self.value
    }

    pub(crate) fn posture(&self) -> &Arc<str> {
        &self.posture
    }
}

impl UiMountedCollectionSemanticTextContent {
    pub(crate) const fn value(&self) -> &UiMountedCollectionTextDirective {
        &self.value
    }

    pub(crate) fn posture(&self) -> &Arc<str> {
        &self.posture
    }
}

impl UiMountedCollectionTextRow {
    pub(crate) fn new(identity: Arc<str>, selected_values: impl Into<Box<[Arc<str>]>>) -> Self {
        Self {
            identity,
            selected_values: selected_values.into(),
        }
    }

    pub(crate) fn identity(&self) -> &Arc<str> {
        &self.identity
    }

    pub(crate) fn selected_values(&self) -> &[Arc<str>] {
        &self.selected_values
    }
}
