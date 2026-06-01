use std::sync::{Arc, RwLock};

use forge_relational::facade::bridge::bridge_snapshot_identity_for_handle;
use forge_relational::facade::runtime::{
    EntityReadRecord, RelationReadRecord, RelationalReadView, RelationalRuntime,
};
use forge_relational::facade::snapshots::SnapshotHandle;

#[derive(Debug, Clone)]
pub enum TopologyRuntimeBinding {
    CurrentHead(Arc<RwLock<RelationalRuntime>>),
    SnapshotReadOnly {
        read_view: Arc<RelationalReadView>,
        snapshot: SnapshotHandle,
    },
}

impl TopologyRuntimeBinding {
    pub fn current_head(runtime: RelationalRuntime) -> Self {
        Self::CurrentHead(Arc::new(RwLock::new(runtime)))
    }

    pub fn snapshot_read_only(read_view: RelationalReadView, snapshot: SnapshotHandle) -> Self {
        Self::SnapshotReadOnly {
            read_view: Arc::new(read_view),
            snapshot,
        }
    }

    pub(super) fn runtime(&self) -> Option<Arc<RwLock<RelationalRuntime>>> {
        match self {
            Self::CurrentHead(runtime) => Some(Arc::clone(runtime)),
            Self::SnapshotReadOnly { .. } => None,
        }
    }

    pub(super) fn entity_records(&self) -> Vec<EntityReadRecord> {
        match self {
            Self::CurrentHead(runtime) => {
                let runtime = runtime
                    .read()
                    .expect("topology runtime binding lock poisoned");
                let Some(version_id) = runtime
                    .publication()
                    .latest_bundle()
                    .map(|bundle| bundle.commit.version_id)
                else {
                    return Vec::new();
                };
                let read_view = runtime.read_truth().read_version(version_id);
                schema::facade::platform::entities::EntityKind::ALL
                    .into_iter()
                    .flat_map(|kind| {
                        read_view
                            .entities()
                            .iter()
                            .filter(move |record| record.kind.kind_id == kind.kind_id())
                            .cloned()
                    })
                    .collect()
            }
            Self::SnapshotReadOnly { read_view, .. } => read_view.entities().to_vec(),
        }
    }

    pub(super) fn relation_records(&self) -> Vec<RelationReadRecord> {
        match self {
            Self::CurrentHead(runtime) => {
                let runtime = runtime
                    .read()
                    .expect("topology runtime binding lock poisoned");
                let Some(version_id) = runtime
                    .publication()
                    .latest_bundle()
                    .map(|bundle| bundle.commit.version_id)
                else {
                    return Vec::new();
                };
                let read_view = runtime.read_truth().read_version(version_id);
                schema::facade::platform::relations::RelationKind::ALL
                    .into_iter()
                    .flat_map(|kind| {
                        read_view
                            .relations()
                            .iter()
                            .filter(move |record| record.kind.kind_id == kind.kind_id())
                            .cloned()
                    })
                    .collect()
            }
            Self::SnapshotReadOnly { read_view, .. } => read_view.relations().to_vec(),
        }
    }

    pub(super) fn snapshot_token(&self) -> String {
        match self {
            Self::CurrentHead(runtime) => runtime
                .read()
                .expect("topology runtime binding lock poisoned")
                .publication()
                .latest_bundle()
                .map(|bundle| {
                    forge_relational::facade::bridge::bridge_snapshot_identity_for_commit(
                        bundle.commit.commit_id,
                        bundle.commit.version_id,
                    )
                    .as_str()
                    .to_string()
                })
                .unwrap_or_else(|| "relational-snapshot:empty:version:0".to_string()),
            Self::SnapshotReadOnly { snapshot, .. } => {
                bridge_snapshot_identity_for_handle(snapshot)
                    .as_str()
                    .to_string()
            }
        }
    }
}
