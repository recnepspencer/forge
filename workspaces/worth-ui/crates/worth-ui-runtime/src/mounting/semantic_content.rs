use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiMountedSemanticContentInput {
    by_graph_node: BTreeMap<crate::graph::UiGraphNodeIdentity, UiMountedSemanticTextContent>,
    projection_input_capacity: usize,
    projection_inputs: BTreeMap<
        worth_ui_query_binding::UiProjectionInputSlot,
        worth_ui_query_binding::UiProjectionInputFactReference,
    >,
    schema_transitions: Vec<crate::runtime::rebind::UiProjectionSchemaTransition>,
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
            projection_input_capacity: 0,
            projection_inputs: BTreeMap::new(),
            schema_transitions: Vec::new(),
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
            && self.projection_inputs.is_empty()
            && self.projection_input_capacity == 0
    }

    pub(crate) fn set_projection_input_capacity(&mut self, capacity: usize) {
        self.projection_input_capacity = capacity;
    }

    pub(crate) fn insert_projection_input(
        &mut self,
        input: worth_ui_query_binding::UiProjectionInputFactReference,
    ) -> Result<(), ()> {
        let slot = input.revision().slot();
        if slot.index() >= self.projection_input_capacity {
            return Err(());
        }
        match self.projection_inputs.get(&slot) {
            Some(existing) if existing != &input => Err(()),
            Some(_) => Ok(()),
            None => {
                self.projection_inputs.insert(slot, input);
                Ok(())
            }
        }
    }

    pub(crate) const fn projection_input_capacity(&self) -> usize {
        self.projection_input_capacity
    }

    pub(crate) fn projection_inputs(
        &self,
    ) -> impl ExactSizeIterator<
        Item = (
            worth_ui_query_binding::UiProjectionInputSlot,
            &worth_ui_query_binding::UiProjectionInputFactReference,
        ),
    > {
        self.projection_inputs
            .iter()
            .map(|(slot, input)| (*slot, input))
    }

    pub(crate) fn record_schema_transition(
        &mut self,
        transition: crate::runtime::rebind::UiProjectionSchemaTransition,
    ) {
        self.schema_transitions.push(transition);
    }

    pub(crate) fn schema_transitions(
        &self,
    ) -> &[crate::runtime::rebind::UiProjectionSchemaTransition] {
        &self.schema_transitions
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
