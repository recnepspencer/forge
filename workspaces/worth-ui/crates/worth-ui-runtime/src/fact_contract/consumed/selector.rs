#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiConsumedFactSelector {
    AuthoredDeclarationIdentity(Box<str>),
    Aspect(crate::declaration::UiAspectName),
    QueryProjection(worth_ui_query_binding::WorthUiQueryViewIdentity),
    IntentPostureGraphNode(crate::graph::UiGraphNodeIdentity),
}

impl UiConsumedFactSelector {
    pub(crate) fn authored_declaration(identity: impl Into<Box<str>>) -> Self {
        Self::AuthoredDeclarationIdentity(identity.into())
    }

    pub(crate) fn aspect(aspect: crate::declaration::UiAspectName) -> Self {
        Self::Aspect(aspect)
    }

    pub(crate) fn query_projection(
        identity: worth_ui_query_binding::WorthUiQueryViewIdentity,
    ) -> Self {
        Self::QueryProjection(identity)
    }

    pub(crate) const fn intent_posture_graph_node(
        identity: crate::graph::UiGraphNodeIdentity,
    ) -> Self {
        Self::IntentPostureGraphNode(identity)
    }
}
