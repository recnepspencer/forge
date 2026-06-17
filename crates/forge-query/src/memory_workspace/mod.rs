use crate::view_shape_live::ViewShapePatchEnvelope;
use forge_foundational::facade::AspectValue;
use forge_relational::facade::identity::{EntityId, KindId};
use forge_relational::facade::runtime::RelationalRuntime;
use forge_runtime_bridge::facade::BridgeMutationAuthorityBundle;
use serde_json::Value;
use std::collections::BTreeMap;

mod external_projection;
mod identities;
mod runtime_identity;
#[cfg(test)]
mod tests;
mod truth_identity_admission;
mod workspace;

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
    label: String,
    external_projection_path: String,
}

impl ForgeQueryAspect {
    pub fn new(label: impl Into<String>, external_projection_path: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            external_projection_path: external_projection_path.into(),
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn external_projection_path(&self) -> &str {
        &self.external_projection_path
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForgeQueryEntity {
    identity: ForgeQueryEntityIdentity,
    row: ForgeQueryEntityRow,
}

#[derive(Debug, Clone, PartialEq)]
enum ForgeQueryEntityRow {
    AspectProjection {
        aspect_values: BTreeMap<String, AspectValue>,
        external_projection: Value,
    },
    ExternalProjection(Value),
}

impl ForgeQueryEntity {
    pub fn from_aspect_projection(
        identity: ForgeQueryEntityIdentity,
        aspect_values: BTreeMap<String, AspectValue>,
        external_projection: Value,
    ) -> Self {
        Self {
            identity,
            row: ForgeQueryEntityRow::AspectProjection {
                aspect_values,
                external_projection,
            },
        }
    }

    pub fn from_external_projection(
        identity: ForgeQueryEntityIdentity,
        external_projection: Value,
    ) -> Self {
        Self {
            identity,
            row: ForgeQueryEntityRow::ExternalProjection(external_projection),
        }
    }

    pub fn identity(&self) -> &ForgeQueryEntityIdentity {
        &self.identity
    }

    pub fn external_row(&self) -> &Value {
        match &self.row {
            ForgeQueryEntityRow::AspectProjection {
                external_projection,
                ..
            }
            | ForgeQueryEntityRow::ExternalProjection(external_projection) => external_projection,
        }
    }

    pub fn aspect_value(&self, aspect_path: &str) -> Option<&AspectValue> {
        match &self.row {
            ForgeQueryEntityRow::AspectProjection { aspect_values, .. } => {
                aspect_values.get(aspect_path)
            }
            ForgeQueryEntityRow::ExternalProjection(_) => None,
        }
    }

    pub fn aspect_values(&self) -> Box<dyn Iterator<Item = (&str, &AspectValue)> + '_> {
        match &self.row {
            ForgeQueryEntityRow::AspectProjection { aspect_values, .. } => Box::new(
                aspect_values
                    .iter()
                    .map(|(path, value)| (path.as_str(), value)),
            ),
            ForgeQueryEntityRow::ExternalProjection(_) => Box::new(std::iter::empty()),
        }
    }

    pub fn into_external_row(self) -> Value {
        match self.row {
            ForgeQueryEntityRow::AspectProjection {
                external_projection,
                ..
            }
            | ForgeQueryEntityRow::ExternalProjection(external_projection) => external_projection,
        }
    }

    pub fn external_row_path(&self, dotted_path: &str) -> Option<&Value> {
        let mut current = self.external_row();
        for segment in dotted_path.split('.') {
            current = current.get(segment)?;
        }
        Some(current)
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
    pub(crate) aspect_paths: Vec<String>,
}

impl ForgeQueryMutationDelta {
    pub fn new(
        collection: impl Into<String>,
        entity_identity: ForgeQueryEntityIdentity,
        kind: ForgeQueryMutationKind,
        aspect_paths: Vec<String>,
    ) -> Self {
        Self {
            collection: collection.into(),
            entity_identity,
            kind,
            aspect_paths,
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

    pub fn aspect_paths(&self) -> &[String] {
        &self.aspect_paths
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
    pub(crate) aspect_paths: Vec<String>,
    pub(crate) envelope: ViewShapePatchEnvelope,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForgeQueryWorkspaceError {
    message: String,
}

impl ForgeQueryWorkspaceError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ForgeQueryWorkspaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for ForgeQueryWorkspaceError {}

pub struct ForgeQueryMemoryWorkspace {
    runtime: RelationalRuntime,
    kind_id: KindId,
    kind_name: String,
    aspects: Vec<ForgeQueryAspect>,
    next_client_key: u64,
}
