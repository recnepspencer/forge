#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ProjectionFactParityBasisLink {
    kind: ProjectionFactParityBasisLinkKind,
    identity: String,
}

impl ProjectionFactParityBasisLink {
    pub(crate) fn new(
        kind: ProjectionFactParityBasisLinkKind,
        identity: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            identity: identity.into(),
        }
    }

    pub(crate) fn kind(&self) -> ProjectionFactParityBasisLinkKind {
        self.kind
    }

    pub(crate) fn identity(&self) -> &str {
        &self.identity
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum ProjectionFactParityBasisLinkKind {
    RetainedFact,
    ProjectionConsumedFact,
    RecoveryPosture,
    DiagnosticBundle,
}
