use crate::view_shape_live::ViewShapePatchEnvelope;
use forge_foundational::facade::AspectValue;
use forge_relational::facade::identity::{EntityId, KindId};
use forge_relational::facade::runtime::RelationalRuntime;
use forge_runtime_bridge::facade::BridgeMutationAuthorityBundle;
use serde_json::Value;
use std::collections::BTreeMap;

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
    identity: String,
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
        identity: impl Into<String>,
        aspect_values: BTreeMap<String, AspectValue>,
        external_projection: Value,
    ) -> Self {
        Self {
            identity: identity.into(),
            row: ForgeQueryEntityRow::AspectProjection {
                aspect_values,
                external_projection,
            },
        }
    }

    pub fn from_external_projection(
        identity: impl Into<String>,
        external_projection: Value,
    ) -> Self {
        Self {
            identity: identity.into(),
            row: ForgeQueryEntityRow::ExternalProjection(external_projection),
        }
    }

    pub fn identity(&self) -> &str {
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
