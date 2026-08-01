use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use worth_query::facade::runtime::{
    WorthQueryLiveArtifactTarget, WorthQueryMutationTargetCollectionIdentity,
};

use super::super::WorthUiScalarProjectionSourceRecord;

pub(crate) type SharedSourceState = Rc<RefCell<WorthUiScalarProjectionSourceState>>;

pub(crate) fn shared_source_state() -> SharedSourceState {
    Rc::new(RefCell::new(WorthUiScalarProjectionSourceState::default()))
}

#[derive(Default)]
pub(crate) struct WorthUiScalarProjectionSourceState {
    record: Option<WorthUiScalarProjectionSourceRecord>,
    live_targets:
        BTreeMap<WorthQueryLiveArtifactTarget, WorthQueryMutationTargetCollectionIdentity>,
    next_commit_identity: u64,
    next_snapshot_version: u64,
}

impl WorthUiScalarProjectionSourceState {
    pub(crate) fn publish(&mut self, record: WorthUiScalarProjectionSourceRecord) {
        self.record = Some(record);
    }

    pub(crate) fn record(&self) -> Option<&WorthUiScalarProjectionSourceRecord> {
        self.record.as_ref()
    }

    pub(crate) fn register_live_target(
        &mut self,
        target: WorthQueryLiveArtifactTarget,
        collection: WorthQueryMutationTargetCollectionIdentity,
    ) {
        self.live_targets.insert(target, collection);
    }

    pub(crate) fn remove_live_target(&mut self, target: &WorthQueryLiveArtifactTarget) {
        self.live_targets.remove(target);
    }

    pub(crate) fn live_collection(
        &self,
        target: &WorthQueryLiveArtifactTarget,
    ) -> Option<&WorthQueryMutationTargetCollectionIdentity> {
        self.live_targets.get(target)
    }

    pub(crate) fn live_targets(
        &self,
    ) -> impl Iterator<
        Item = (
            &WorthQueryLiveArtifactTarget,
            &WorthQueryMutationTargetCollectionIdentity,
        ),
    > {
        self.live_targets.iter()
    }

    pub(crate) fn live_source_count(&self) -> usize {
        self.live_targets.len()
    }

    pub(crate) fn next_authoritative_positions(&self) -> (u64, u64) {
        (
            self.next_commit_identity.saturating_add(1),
            self.next_snapshot_version.saturating_add(1),
        )
    }

    pub(crate) fn commit_action(
        &mut self,
        expected_revision: u64,
        expected_positions: (u64, u64),
        record: WorthUiScalarProjectionSourceRecord,
    ) -> Result<(), &'static str> {
        if self
            .record
            .as_ref()
            .map(WorthUiScalarProjectionSourceRecord::revision)
            != Some(expected_revision)
        {
            return Err("product source revision changed before intent commit");
        }
        if self.next_authoritative_positions() != expected_positions {
            return Err("product Query authority positions changed before intent commit");
        }
        self.next_commit_identity = expected_positions.0;
        self.next_snapshot_version = expected_positions.1;
        self.record = Some(record);
        Ok(())
    }

    pub(crate) fn current_snapshot_version(&self) -> u64 {
        self.next_snapshot_version
    }
}
