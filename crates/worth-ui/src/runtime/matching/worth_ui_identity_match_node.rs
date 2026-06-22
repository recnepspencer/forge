use crate::source::{WorthUiArtifactHandle, WorthUiArtifactNodeKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiIdentityMatchNodeSide {
    Active,
    Candidate,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthUiIdentityMatchNodeKind {
    Import,
    Component,
    Surface,
    Binding,
    Token,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiIdentityMatchNode {
    side: WorthUiIdentityMatchNodeSide,
    kind: WorthUiIdentityMatchNodeKind,
    handle: WorthUiArtifactHandle,
    identity_basis: String,
    stable_identity: bool,
    durable_state_eligible: bool,
}

impl WorthUiIdentityMatchNode {
    pub(crate) fn new(
        side: WorthUiIdentityMatchNodeSide,
        handle: WorthUiArtifactHandle,
        identity_basis: String,
        stable_identity: bool,
        durable_state_eligible: bool,
    ) -> Self {
        let kind = WorthUiIdentityMatchNodeKind::from_artifact_kind(handle.kind());
        Self {
            side,
            kind,
            handle,
            identity_basis,
            stable_identity,
            durable_state_eligible,
        }
    }

    pub fn side(&self) -> WorthUiIdentityMatchNodeSide {
        self.side
    }

    pub fn kind(&self) -> WorthUiIdentityMatchNodeKind {
        self.kind
    }

    pub fn identity_basis(&self) -> &str {
        &self.identity_basis
    }

    pub fn stable_identity(&self) -> bool {
        self.stable_identity
    }

    pub fn durable_state_eligible(&self) -> bool {
        self.durable_state_eligible
    }

    pub fn node_summary(&self) -> String {
        format!(
            "{:?}:{}:{}",
            self.kind,
            self.handle.module_id().as_str(),
            self.handle.node_index()
        )
    }

    pub(crate) fn handle(&self) -> &WorthUiArtifactHandle {
        &self.handle
    }
}

impl WorthUiIdentityMatchNodeKind {
    pub(crate) fn from_artifact_kind(kind: WorthUiArtifactNodeKind) -> Self {
        match kind {
            WorthUiArtifactNodeKind::Import => Self::Import,
            WorthUiArtifactNodeKind::Component => Self::Component,
            WorthUiArtifactNodeKind::Surface => Self::Surface,
            WorthUiArtifactNodeKind::Binding => Self::Binding,
            WorthUiArtifactNodeKind::Token => Self::Token,
        }
    }
}
