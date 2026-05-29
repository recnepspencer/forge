use crate::view_shape_live::ViewShapePatchEnvelope;
use forge_relational::facade::identity::{EntityId, KindId};
use forge_relational::facade::runtime::RelationalRuntime;
use forge_runtime_bridge::facade::BridgeMutationAuthorityBundle;
use serde_json::Value;

mod helpers;
#[cfg(test)]
mod tests;
mod workspace;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForgeQueryAspect {
    label: String,
    payload_path: String,
}

impl ForgeQueryAspect {
    pub fn new(label: impl Into<String>, payload_path: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            payload_path: payload_path.into(),
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn payload_path(&self) -> &str {
        &self.payload_path
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForgeQueryEntity {
    pub identity: String,
    pub payload: Value,
}

impl ForgeQueryEntity {
    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn external_row(&self) -> &Value {
        &self.payload
    }

    pub fn into_external_row(self) -> Value {
        self.payload
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
    pub collection: String,
    pub entity_identity: String,
    pub kind: ForgeQueryMutationKind,
    pub aspect_paths: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForgeQueryMutationReceipt {
    pub commit_identity: String,
    pub snapshot_token: String,
    pub deltas: Vec<ForgeQueryMutationDelta>,
    pub bridge_authority: Option<BridgeMutationAuthorityBundle>,
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
    pub view_name: String,
    pub commit_identity: String,
    pub entity_identity: String,
    pub mutation_kind: ForgeQueryMutationKind,
    pub aspect_paths: Vec<String>,
    pub envelope: ViewShapePatchEnvelope,
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
