use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use crate::declarative_live::{DeclarativeLiveQuerySession, DeclarativeLiveViewShape};
use crate::live::BridgeChangeSummary;
use crate::schema_view::QuerySchemaView;
use crate::view_shape_live::ViewShapePatchEnvelope;
use forge_relational::facade::identity::{EntityId, KindId};
use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::symbols::InternedString;
use forge_runtime_bridge::facade::{BridgeMutationAuthorityBundle, RuntimeBridge};
use serde_json::Value;

mod app_construction;
mod app_mutations;
mod app_writeback;
mod backend;
mod bridge;
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
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForgeQueryEntity {
    pub identity: String,
    pub payload: Value,
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
    next_client_key: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForgeQueryCollection {
    name: String,
    aspects: Vec<ForgeQueryAspect>,
}

impl ForgeQueryCollection {
    pub fn new(
        name: impl Into<String>,
        aspects: impl IntoIterator<Item = ForgeQueryAspect>,
    ) -> Self {
        Self {
            name: name.into(),
            aspects: aspects.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ForgeQueryCollectionRuntime {
    kind_id: KindId,
    next_client_key: u64,
}

#[derive(Debug, Clone)]
struct ForgeQueryLiveViewRuntime {
    session: DeclarativeLiveQuerySession,
    patches: Vec<ForgeQueryLivePatch>,
}

#[derive(Clone, Debug)]
struct ForgeQueryBridgeSource;

#[derive(Clone, Debug)]
struct ForgeQueryBridgeSnapshotReader {
    identity: forge_runtime_bridge::facade::TruthSnapshotIdentity,
}

#[derive(Clone, Debug)]
struct ForgeQueryBridgeSink;

#[derive(Clone)]
struct ForgeQueryWritebackAuthority {
    state: Arc<Mutex<ForgeQueryAuthorityState>>,
}

struct ForgeQueryAuthorityState {
    runtime: RelationalRuntime,
    pending: BTreeMap<String, ForgeQueryPendingWriteback>,
    completed: BTreeMap<String, ForgeQueryMutationReceipt>,
}

#[derive(Clone, Debug)]
struct ForgeQueryPendingWriteback {
    collection: String,
    kind: ForgeQueryMutationKind,
    aspect_paths: Vec<String>,
    operation: ForgeQueryPendingOperation,
}

#[derive(Clone, Debug)]
enum ForgeQueryPendingOperation {
    Insert {
        kind_id: KindId,
        client_key: InternedString,
        payload: Value,
    },
    Update {
        entity_id: EntityId,
        payload: Value,
        existing_truth_binding: Option<crate::runtime::ForgeQueryExistingTruthTargetBinding>,
    },
    Delete {
        entity_id: EntityId,
        existing_truth_binding: Option<crate::runtime::ForgeQueryExistingTruthTargetBinding>,
    },
}

pub struct ForgeQueryMemoryApp {
    authority_state: Arc<Mutex<ForgeQueryAuthorityState>>,
    bridge: RuntimeBridge,
    collections: BTreeMap<String, ForgeQueryCollectionRuntime>,
    entity_collections: BTreeMap<String, String>,
    live_views: BTreeMap<String, ForgeQueryLiveViewRuntime>,
}
