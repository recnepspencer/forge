use crate::view_shape_live::ViewShapePatchEnvelope;
use forge_foundational::facade::CanonicalFieldPath;
use forge_relational::facade::identity::{EntityId, KindId};
use forge_relational::facade::runtime::RelationalRuntime;
use forge_runtime_bridge::facade::BridgeMutationAuthorityBundle;

mod entity_row;
mod identities;
mod runtime_identity;
#[cfg(test)]
mod tests;
mod truth_identity_admission;
mod workspace;

pub use entity_row::ForgeQueryEntity;
pub use identities::{
    ForgeQueryCommitIdentity, ForgeQueryEntityIdentity, ForgeQuerySnapshotIdentity,
};
#[cfg(test)]
pub(crate) use truth_identity_admission::admit_external_commit_label;
pub(crate) use truth_identity_admission::{
    admit_authored_entity_label, admit_external_snapshot_label,
};
pub use truth_identity_admission::{
    admit_authored_entity_token, admit_external_commit_token, admit_external_snapshot_token,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForgeQueryAspect {
    touch: crate::runtime::ForgeQueryAspectTouch,
    external_projection_path: CanonicalFieldPath,
}

impl ForgeQueryAspect {
    pub fn new(
        touch: crate::runtime::ForgeQueryAspectTouch,
        external_projection_path: CanonicalFieldPath,
    ) -> Self {
        Self {
            touch,
            external_projection_path,
        }
    }

    pub(crate) fn from_native_external_projection_path(
        touch: crate::runtime::ForgeQueryAspectTouch,
        external_projection_path: CanonicalFieldPath,
    ) -> Self {
        Self {
            touch,
            external_projection_path,
        }
    }

    pub fn aspect_touch(&self) -> &crate::runtime::ForgeQueryAspectTouch {
        &self.touch
    }

    pub fn external_projection_path(&self) -> &CanonicalFieldPath {
        &self.external_projection_path
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForgeQueryMutationKind {
    Created,
    Updated,
    Deleted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForgeQueryMutationDelta {
    pub(crate) collection: String,
    pub(crate) entity_identity: ForgeQueryEntityIdentity,
    pub(crate) kind: ForgeQueryMutationKind,
    pub(crate) touched_aspects: Vec<crate::runtime::ForgeQueryAspectTouch>,
}

impl ForgeQueryMutationDelta {
    pub fn from_touched_aspects(
        collection: impl Into<String>,
        entity_identity: ForgeQueryEntityIdentity,
        kind: ForgeQueryMutationKind,
        touched_aspects: Vec<crate::runtime::ForgeQueryAspectTouch>,
    ) -> Self {
        Self {
            collection: collection.into(),
            entity_identity,
            kind,
            touched_aspects,
        }
    }

    pub fn collection(&self) -> &str {
        &self.collection
    }

    pub fn entity_identity(&self) -> &ForgeQueryEntityIdentity {
        &self.entity_identity
    }

    pub fn kind(&self) -> &ForgeQueryMutationKind {
        &self.kind
    }

    pub fn admitted_touched_aspects(&self) -> &[crate::runtime::ForgeQueryAspectTouch] {
        &self.touched_aspects
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForgeQueryMutationReceipt {
    pub(crate) commit_identity: ForgeQueryCommitIdentity,
    pub(crate) snapshot_identity: ForgeQuerySnapshotIdentity,
    pub(crate) deltas: Vec<ForgeQueryMutationDelta>,
    pub(crate) bridge_authority: Option<BridgeMutationAuthorityBundle>,
}

impl ForgeQueryMutationReceipt {
    pub fn from_authoritative_parts(
        commit_identity: ForgeQueryCommitIdentity,
        snapshot_identity: ForgeQuerySnapshotIdentity,
        deltas: Vec<ForgeQueryMutationDelta>,
    ) -> Self {
        Self {
            commit_identity,
            snapshot_identity,
            deltas,
            bridge_authority: None,
        }
    }

    pub fn from_bridge_authoritative_parts(
        commit_identity: ForgeQueryCommitIdentity,
        snapshot_identity: ForgeQuerySnapshotIdentity,
        deltas: Vec<ForgeQueryMutationDelta>,
        bridge_authority: BridgeMutationAuthorityBundle,
    ) -> Self {
        Self {
            commit_identity,
            snapshot_identity,
            deltas,
            bridge_authority: Some(bridge_authority),
        }
    }

    pub fn commit_identity(&self) -> &ForgeQueryCommitIdentity {
        &self.commit_identity
    }

    pub fn snapshot_identity(&self) -> &ForgeQuerySnapshotIdentity {
        &self.snapshot_identity
    }

    pub fn deltas(&self) -> &[ForgeQueryMutationDelta] {
        &self.deltas
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForgeQueryLiveViewHandle {
    name: String,
}

impl ForgeQueryLiveViewHandle {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForgeQueryLivePatch {
    pub(crate) view_name: String,
    pub(crate) commit_identity: ForgeQueryCommitIdentity,
    pub(crate) entity_identity: ForgeQueryEntityIdentity,
    pub(crate) mutation_kind: ForgeQueryMutationKind,
    pub(crate) touched_aspects: Vec<crate::runtime::ForgeQueryAspectTouch>,
    pub(crate) envelope: ViewShapePatchEnvelope,
}

impl ForgeQueryLivePatch {
    pub fn admitted_touched_aspects(&self) -> &[crate::runtime::ForgeQueryAspectTouch] {
        &self.touched_aspects
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForgeQueryWorkspaceError {
    kind: ForgeQueryWorkspaceErrorKind,
    message: String,
}

impl ForgeQueryWorkspaceError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            kind: ForgeQueryWorkspaceErrorKind::Unclassified,
            message: message.into(),
        }
    }

    pub fn with_kind(kind: ForgeQueryWorkspaceErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    pub fn kind(&self) -> ForgeQueryWorkspaceErrorKind {
        self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for ForgeQueryWorkspaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for ForgeQueryWorkspaceError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForgeQueryWorkspaceErrorKind {
    Unclassified,
    UnsupportedCollection,
    UnsupportedWriteFamily,
    EmptySchema,
    BatchAtomicityUnsupported,
}

pub struct ForgeQueryMemoryWorkspace {
    runtime: RelationalRuntime,
    kind_id: KindId,
    kind_name: String,
    aspects: Vec<ForgeQueryAspect>,
    next_client_key: u64,
}
