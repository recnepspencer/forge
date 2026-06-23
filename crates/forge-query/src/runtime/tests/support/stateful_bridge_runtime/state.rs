use forge_runtime_bridge::facade::RuntimeBridge;
use std::collections::{BTreeMap, BTreeSet};

use crate::memory_workspace::ForgeQueryEntityIdentity;
use crate::runtime::tests::support::test_bridge_with_writeback_authority;
use crate::runtime::{ForgeQueryLiveArtifactTarget, ForgeQueryMutationTargetCollectionIdentity};
use forge_foundational::facade::{AspectValue, CanonicalFieldPath};

pub(super) type NativeExternalRow = BTreeMap<CanonicalFieldPath, AspectValue>;

pub(super) struct StatefulBridgeState {
    pub(super) live_views:
        BTreeMap<ForgeQueryLiveArtifactTarget, ForgeQueryMutationTargetCollectionIdentity>,
    pub(super) installed_collections: BTreeSet<String>,
    pub(super) rows_by_collection: BTreeMap<String, BTreeMap<String, NativeExternalRow>>,
    pub(super) collection_by_identity: BTreeMap<String, String>,
    pub(super) identity_by_symbol: BTreeMap<String, ForgeQueryEntityIdentity>,
    pub(super) identity_text_by_symbol: BTreeMap<String, String>,
    pub(super) identity_by_storage_key: BTreeMap<String, ForgeQueryEntityIdentity>,
    pub(super) next_entity_identity: usize,
    pub(super) next_commit_identity: usize,
    pub(super) next_snapshot_token: usize,
    pub(super) bridge: RuntimeBridge,
}

impl StatefulBridgeState {
    pub(super) fn new(installed_collections: BTreeSet<String>) -> Self {
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
            bridge: test_bridge_with_writeback_authority(),
        }
    }
}
