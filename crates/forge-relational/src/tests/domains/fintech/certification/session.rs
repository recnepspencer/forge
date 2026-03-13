use std::collections::BTreeMap;

use serde_json::Value;

use crate::facade::history::BranchId;
use crate::facade::replay::RelationalReplayOutcome;
use crate::facade::snapshots::SnapshotHandle;

use super::super::fixture::FintechWorld;

#[derive(Debug)]
pub(super) struct CertifiedRelationalFintechSession {
    pub(super) world: FintechWorld,
    pub(super) named_branches: BTreeMap<String, BranchId>,
    pub(super) named_snapshots: BTreeMap<String, SnapshotHandle>,
    pub(super) named_reads: BTreeMap<String, Value>,
    pub(super) named_replays: BTreeMap<String, RelationalReplayOutcome>,
    pub(super) executed_steps: Vec<String>,
    pub(super) checkpoints: Vec<String>,
}

impl CertifiedRelationalFintechSession {
    pub(super) fn branch(&self, alias: &str) -> Result<BranchId, String> {
        self.named_branches
            .get(alias)
            .cloned()
            .ok_or_else(|| format!("unknown certified fintech branch alias `{alias}`"))
    }

    pub(super) fn snapshot(&self, alias: &str) -> Result<SnapshotHandle, String> {
        self.named_snapshots
            .get(alias)
            .cloned()
            .ok_or_else(|| format!("unknown certified fintech snapshot alias `{alias}`"))
    }
}
