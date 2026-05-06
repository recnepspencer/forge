use std::collections::BTreeMap;

use serde_json::Value;

#[derive(Default)]
pub(super) struct PublicBridgeRuntimeState {
    pub(super) live_views: BTreeMap<String, String>,
    pub(super) rows_by_collection: BTreeMap<String, BTreeMap<String, Value>>,
    pub(super) collection_by_identity: BTreeMap<String, String>,
    pub(super) identity_by_symbol: BTreeMap<String, String>,
    pub(super) existing_truth_values: BTreeMap<(String, String, String), Value>,
    pub(super) next_entity_identity: usize,
    pub(super) next_commit_identity: usize,
    pub(super) next_snapshot_token: usize,
}
