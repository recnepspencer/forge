use crate::capability::{MosaicResizePermission, MosaicSizingContractId};
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
    authored_provenance_digest: u64,
    semantic_meaning: crate::source::WorthUiArtifactNode,
    stable_identity: bool,
    durable_state_eligible: bool,
    resize_contract_id: Option<MosaicSizingContractId>,
    resize_permission: Option<MosaicResizePermission>,
    resize_shape_digest: Option<u64>,
}

pub(crate) struct WorthUiIdentityMatchNodeInput {
    pub side: WorthUiIdentityMatchNodeSide,
    pub handle: WorthUiArtifactHandle,
    pub identity_basis: String,
    pub authored_provenance_digest: u64,
    pub semantic_meaning: crate::source::WorthUiArtifactNode,
    pub stable_identity: bool,
    pub durable_state_eligible: bool,
    pub resize_contract_id: Option<MosaicSizingContractId>,
    pub resize_permission: Option<MosaicResizePermission>,
    pub resize_shape_digest: Option<u64>,
}

impl WorthUiIdentityMatchNode {
    pub(crate) fn new(input: WorthUiIdentityMatchNodeInput) -> Self {
        let WorthUiIdentityMatchNodeInput {
            side,
            handle,
            identity_basis,
            authored_provenance_digest,
            semantic_meaning,
            stable_identity,
            durable_state_eligible,
            resize_contract_id,
            resize_permission,
            resize_shape_digest,
        } = input;
        let kind = WorthUiIdentityMatchNodeKind::from_artifact_kind(handle.kind());
        Self {
            side,
            kind,
            handle,
            identity_basis,
            authored_provenance_digest,
            semantic_meaning,
            stable_identity,
            durable_state_eligible,
            resize_contract_id,
            resize_permission,
            resize_shape_digest,
        }
    }

    #[cfg(test)]
    pub fn side(&self) -> WorthUiIdentityMatchNodeSide {
        self.side
    }

    pub fn kind(&self) -> WorthUiIdentityMatchNodeKind {
        self.kind
    }

    pub fn identity_basis(&self) -> &str {
        &self.identity_basis
    }

    pub fn authored_provenance_digest(&self) -> u64 {
        self.authored_provenance_digest
    }

    pub(crate) fn has_same_semantic_meaning(&self, other: &Self) -> bool {
        self.semantic_meaning
            .has_same_semantic_meaning_ignoring_location(&other.semantic_meaning)
    }

    #[cfg(test)]
    pub fn stable_identity(&self) -> bool {
        self.stable_identity
    }

    pub fn durable_state_eligible(&self) -> bool {
        self.durable_state_eligible
    }

    pub fn resize_contract_id(&self) -> Option<&MosaicSizingContractId> {
        self.resize_contract_id.as_ref()
    }

    pub fn resize_permission(&self) -> Option<&MosaicResizePermission> {
        self.resize_permission.as_ref()
    }

    pub fn resize_shape_digest(&self) -> Option<u64> {
        self.resize_shape_digest
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
