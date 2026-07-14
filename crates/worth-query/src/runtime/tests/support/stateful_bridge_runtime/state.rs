use std::collections::{BTreeMap, BTreeSet};
use worth_relational::facade::runtime::RelationalRuntime;
use worth_runtime_bridge::facade::RuntimeBridge;

use crate::memory_workspace::WorthQueryEntityIdentity;
use crate::runtime::tests::support::test_bridge_with_writeback_authority;
use crate::runtime::{WorthQueryLiveArtifactTarget, WorthQueryMutationTargetCollectionIdentity};
use worth_foundational::facade::{AspectValue, CanonicalFieldPath};

pub(super) type NativeExternalRow = BTreeMap<CanonicalFieldPath, AspectValue>;

pub(super) struct StatefulBridgeState {
    pub(super) live_views:
        BTreeMap<WorthQueryLiveArtifactTarget, WorthQueryMutationTargetCollectionIdentity>,
    pub(super) installed_collections: BTreeSet<String>,
    pub(super) rows_by_collection: BTreeMap<String, BTreeMap<String, NativeExternalRow>>,
    pub(super) collection_by_identity: BTreeMap<String, String>,
    pub(super) identity_by_symbol: BTreeMap<String, WorthQueryEntityIdentity>,
    pub(super) identity_text_by_symbol: BTreeMap<String, String>,
    pub(super) identity_by_storage_key: BTreeMap<String, WorthQueryEntityIdentity>,
    pub(super) next_entity_identity: usize,
    pub(super) next_commit_identity: usize,
    pub(super) next_snapshot_token: usize,
    pub(super) bridge: RuntimeBridge,
    pub(super) relational_runtime: Option<RelationalRuntime>,
}

impl StatefulBridgeState {
    pub(super) fn new(installed_collections: BTreeSet<String>) -> Self {
        Self::with_bridge(
            installed_collections,
            test_bridge_with_writeback_authority(),
        )
    }

    pub(super) fn with_bridge(
        installed_collections: BTreeSet<String>,
        bridge: RuntimeBridge,
    ) -> Self {
        Self {
            installed_collections,
            live_views: BTreeMap::new(),
            rows_by_collection: BTreeMap::new(),
            collection_by_identity: BTreeMap::new(),
            identity_by_symbol: BTreeMap::new(),
            identity_text_by_symbol: BTreeMap::new(),
            identity_by_storage_key: BTreeMap::new(),
            next_entity_identity: 0,
            next_commit_identity: 0,
            next_snapshot_token: 0,
            bridge,
            relational_runtime: None,
        }
    }

    pub(super) fn with_relational_runtime(mut self, runtime: RelationalRuntime) -> Self {
        self.relational_runtime = Some(runtime);
        self
    }
}
