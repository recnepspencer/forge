#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiConsumedFactSelector {
    AuthoredDeclarationIdentity(Box<str>),
    Aspect(crate::declaration::UiAspectName),
    QueryProjection(worth_ui_query_binding::WorthUiQueryViewIdentity),
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
}
