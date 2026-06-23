use crate::view_shape_live::ViewShapePatchEnvelope;
use forge_foundational::facade::{CanonicalFieldPath, FieldKey};
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
    native_field_path: CanonicalFieldPath,
}

impl ForgeQueryAspect {
    pub fn new(
        touch: crate::runtime::ForgeQueryAspectTouch,
        native_field_path: CanonicalFieldPath,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        ensure_touch_matches_native_field_path(&touch, &native_field_path)?;
        Ok(Self {
            touch,
            native_field_path,
        })
    }

    pub(crate) fn from_native_field_path(
        touch: crate::runtime::ForgeQueryAspectTouch,
        native_field_path: CanonicalFieldPath,
    ) -> Self {
        Self {
            touch,
            native_field_path,
        }
    }

    pub fn aspect_touch(&self) -> &crate::runtime::ForgeQueryAspectTouch {
        &self.touch
    }

    pub fn native_field_path(&self) -> &CanonicalFieldPath {
        &self.native_field_path
    }
}

fn ensure_touch_matches_native_field_path(
    touch: &crate::runtime::ForgeQueryAspectTouch,
    native_field_path: &CanonicalFieldPath,
) -> Result<(), ForgeQueryWorkspaceError> {
    let expected_aspect_root =
        FieldKey::new(touch.native_aspect_key().as_str()).ok_or_else(|| {
            ForgeQueryWorkspaceError::new(format!(
                "aspect `{}` cannot anchor a memory workspace native field path",
                touch.native_aspect_key().as_str()
            ))
        })?;
    let native_fields = native_field_path.fields();
    if native_fields.first() != Some(&expected_aspect_root) {
        return Err(ForgeQueryWorkspaceError::new(format!(
            "memory workspace aspect `{}` must use native field path rooted at `{}`",
            touch.admitted_touch_digest_part(),
            touch.native_aspect_key().as_str()
        )));
    }
    let Some(touch_field_path) = touch.native_field_path() else {
        return Ok(());
    };
    let expected = std::iter::once(expected_aspect_root)
        .chain(touch_field_path.fields().iter().cloned())
        .collect::<Vec<_>>();
    if native_fields != expected.as_slice() {
        return Err(ForgeQueryWorkspaceError::new(format!(
            "memory workspace aspect `{}` must use matching native field path `{}`",
            touch.admitted_touch_digest_part(),
            expected
                .iter()
                .map(|field| field.as_str())
                .collect::<Vec<_>>()
                .join(".")
        )));
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForgeQueryMutationKind {
    Created,
    Updated,
    Deleted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForgeQueryMutationDelta {
    pub(crate) collection_identity: crate::runtime::ForgeQueryMutationTargetCollectionIdentity,
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
        let collection = collection.into();
        Self {
            collection_identity: crate::runtime::ForgeQueryMutationTargetCollectionIdentity::new(
                "mutation-delta-collection",
                collection,
            ),
            entity_identity,
            kind,
            touched_aspects,
        }
    }

    pub(crate) fn from_collection_identity(
        collection_identity: crate::runtime::ForgeQueryMutationTargetCollectionIdentity,
        entity_identity: ForgeQueryEntityIdentity,
        kind: ForgeQueryMutationKind,
        touched_aspects: Vec<crate::runtime::ForgeQueryAspectTouch>,
    ) -> Self {
        Self {
            collection_identity,
            entity_identity,
            kind,
            touched_aspects,
        }
    }

    pub fn collection(&self) -> &str {
        self.collection_identity.as_str()
    }

    pub fn target_collection_identity(
        &self,
    ) -> &crate::runtime::ForgeQueryMutationTargetCollectionIdentity {
        &self.collection_identity
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
