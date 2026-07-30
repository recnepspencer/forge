#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiProjectionSchemaTransitionKind {
    Stopped,
    Recovered,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiProjectionPredecessorValuePolicy {
    Preserve,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiProjectionSchemaRequirement {
    Scalar(worth_ui_query_binding::UiScalarSchemaRequirement),
    Collection(worth_ui_query_binding::UiCollectionSchemaRequirement),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiProjectionSchemaTransition {
    kind: UiProjectionSchemaTransitionKind,
    predecessor_policy: UiProjectionPredecessorValuePolicy,
    component_identity: Box<str>,
    declaration_identity: Box<str>,
    view_identity: worth_ui_query_binding::WorthUiQueryViewIdentity,
    graph_node: crate::graph::UiGraphNodeIdentity,
    predecessor: UiProjectionSchemaRequirement,
    candidate: UiProjectionSchemaRequirement,
    installed: UiProjectionSchemaRequirement,
}

pub(crate) struct UiProjectionSchemaTransitionInput {
    pub(crate) kind: UiProjectionSchemaTransitionKind,
    pub(crate) component_identity: Box<str>,
    pub(crate) declaration_identity: Box<str>,
    pub(crate) view_identity: worth_ui_query_binding::WorthUiQueryViewIdentity,
    pub(crate) graph_node: crate::graph::UiGraphNodeIdentity,
    pub(crate) predecessor: UiProjectionSchemaRequirement,
    pub(crate) candidate: UiProjectionSchemaRequirement,
    pub(crate) installed: UiProjectionSchemaRequirement,
}

impl UiProjectionSchemaTransition {
    pub(crate) fn new(input: UiProjectionSchemaTransitionInput) -> Self {
        Self {
            kind: input.kind,
            predecessor_policy: UiProjectionPredecessorValuePolicy::Preserve,
            component_identity: input.component_identity,
            declaration_identity: input.declaration_identity,
            view_identity: input.view_identity,
            graph_node: input.graph_node,
            predecessor: input.predecessor,
            candidate: input.candidate,
            installed: input.installed,
        }
    }

    pub const fn kind(&self) -> UiProjectionSchemaTransitionKind {
        self.kind
    }

    pub const fn predecessor_policy(&self) -> UiProjectionPredecessorValuePolicy {
        self.predecessor_policy
    }

    pub fn component_identity(&self) -> &str {
        &self.component_identity
    }

    pub fn declaration_identity(&self) -> &str {
        &self.declaration_identity
    }

    pub fn view_identity(&self) -> &worth_ui_query_binding::WorthUiQueryViewIdentity {
        &self.view_identity
    }

    pub const fn graph_node(&self) -> crate::graph::UiGraphNodeIdentity {
        self.graph_node
    }

    pub const fn predecessor(&self) -> &UiProjectionSchemaRequirement {
        &self.predecessor
    }

    pub const fn candidate(&self) -> &UiProjectionSchemaRequirement {
        &self.candidate
    }

    pub const fn installed(&self) -> &UiProjectionSchemaRequirement {
        &self.installed
    }
}
