use std::collections::{BTreeMap, VecDeque};
use std::sync::Arc;

use super::records::{BridgeFailureRecord, BridgeRouteRecord};

#[derive(Debug, Default)]
pub(super) struct BridgeDiagnosticsState {
    route_records: VecDeque<Arc<BridgeRouteRecord>>,
    failure_records: VecDeque<Arc<BridgeFailureRecord>>,
    latest_route_by_route_identity: BTreeMap<String, Arc<BridgeRouteRecord>>,
    latest_route_by_invalidation_identity: BTreeMap<String, Arc<BridgeRouteRecord>>,
    latest_route_by_source_commit: BTreeMap<String, Arc<BridgeRouteRecord>>,
}

#[derive(Debug, Clone)]
pub(super) struct BridgeDiagnosticsConfig {
    pub(super) tier: crate::policy::BridgeDiagnosticsTier,
    pub(super) records_enabled: bool,
    pub(super) replay_enabled: bool,
    pub(super) route_record_limit: usize,
    pub(super) failure_record_limit: usize,
}

impl BridgeDiagnosticsState {
    pub(super) fn route_records(&self) -> Vec<BridgeRouteRecord> {
        self.route_records
            .iter()
            .map(|record| (**record).clone())
            .collect()
    }

    pub(super) fn failure_records(&self) -> Vec<BridgeFailureRecord> {
        self.failure_records
            .iter()
            .map(|record| (**record).clone())
            .collect()
    }

    pub(super) fn last_route_record(&self) -> Option<BridgeRouteRecord> {
        self.route_records.back().map(|record| (**record).clone())
    }

    pub(super) fn last_failure_record(&self) -> Option<BridgeFailureRecord> {
        self.failure_records.back().map(|record| (**record).clone())
    }

    pub(super) fn record_route(&mut self, record: BridgeRouteRecord, limit: usize) {
        let record = Arc::new(record);
        self.latest_route_by_route_identity.insert(
            record.route_identity().as_str().to_string(),
            Arc::clone(&record),
        );
        self.latest_route_by_invalidation_identity.insert(
            record.invalidation_identity().as_str().to_string(),
            Arc::clone(&record),
        );
        self.latest_route_by_source_commit.insert(
            record.source_commit().as_str().to_string(),
            Arc::clone(&record),
        );
        self.route_records.push_back(record);
        while self.route_records.len() > limit.max(1) {
            if let Some(evicted) = self.route_records.pop_front() {
                self.evict_route_indexes(&evicted);
            }
        }
    }

    pub(super) fn record_failure(&mut self, record: BridgeFailureRecord, limit: usize) {
        self.failure_records.push_back(Arc::new(record));
        trim_retained_records(&mut self.failure_records, limit, |_| {});
    }

    pub(super) fn route_record_for_route_identity(
        &self,
        route_identity: &str,
    ) -> Option<BridgeRouteRecord> {
        self.latest_route_by_route_identity
            .get(route_identity)
            .cloned()
            .map(|record| (*record).clone())
    }

    pub(super) fn route_record_for_invalidation_identity(
        &self,
        invalidation_identity: &str,
    ) -> Option<BridgeRouteRecord> {
        self.latest_route_by_invalidation_identity
            .get(invalidation_identity)
            .cloned()
            .map(|record| (*record).clone())
    }

    pub(super) fn route_record_for_source_commit(
        &self,
        source_commit: &str,
    ) -> Option<BridgeRouteRecord> {
        self.latest_route_by_source_commit
            .get(source_commit)
            .cloned()
            .map(|record| (*record).clone())
    }

    fn evict_route_indexes(&mut self, evicted: &Arc<BridgeRouteRecord>) {
        evict_index_entry(
            &mut self.latest_route_by_route_identity,
            evicted.route_identity().as_str(),
            evicted,
        );
        evict_index_entry(
            &mut self.latest_route_by_invalidation_identity,
            evicted.invalidation_identity().as_str(),
            evicted,
        );
        evict_index_entry(
            &mut self.latest_route_by_source_commit,
            evicted.source_commit().as_str(),
            evicted,
        );
    }
}

fn trim_retained_records<T, F>(records: &mut VecDeque<T>, limit: usize, mut on_evict: F)
where
    F: FnMut(&T),
{
    while records.len() > limit.max(1) {
        if let Some(evicted) = records.pop_front() {
            on_evict(&evicted);
        }
    }
}

fn evict_index_entry<T: Clone>(
    index: &mut BTreeMap<String, Arc<T>>,
    key: &str,
    evicted: &Arc<T>,
) {
    let Some(latest) = index.get(key) else {
        return;
    };

    if Arc::ptr_eq(latest, evicted) {
        index.remove(key);
    }
}
