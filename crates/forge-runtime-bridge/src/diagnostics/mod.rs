mod explanation;
mod records;
mod replay;

use std::collections::{BTreeMap, VecDeque};
use std::sync::{Arc, RwLock};

use crate::error::{BridgeDeliveryError, BridgeReplayError};
use crate::policy::{BridgeDiagnosticsTier, BridgeRuntimePolicy};
use crate::routing::{
    BridgeInvalidationIdentity, BridgeRouteContractProof, BridgeRouteIdentity,
    BridgeRoutingCounters, BridgeSubscriptionSliceIdentity,
};
use crate::snapshot::TruthSnapshotIdentity;

pub use explanation::{BridgeRouteExplanation, BridgeRouteExplanationEntry};
pub use records::{
    BridgeContractDiagnosticsRecord, BridgeFailureClass, BridgeFailureRecord,
    BridgeLoweringDiagnosticsRecord, BridgeRouteRecord, BridgeRouteRecordEntry,
    BridgeRouteRecordMatch, BridgeRouteSourceRecord, BridgeRoutingDiagnosticsRecord,
};
pub use replay::{
    BridgeCanonicalRouteRecord, BridgeReplayRecord, BridgeReplaySummary,
    BRIDGE_CANONICAL_ROUTE_RECORD_SCHEMA_V2,
};

pub(crate) trait DiagnosticSink: Send + Sync {
    fn record_route(&self, record: BridgeRouteRecord);
    fn record_delivery_failure(&self, source: BridgeFailureSource, error: &BridgeDeliveryError);
    fn record_replay_failure(&self, source: BridgeFailureSource, error: &BridgeReplayError);
}

#[derive(Debug, Clone)]
pub(crate) struct BridgeFailureSource {
    source_commit: crate::input::envelope::TruthCommitIdentity,
    source_patch: crate::input::envelope::TruthPatchIdentity,
    source_snapshot: TruthSnapshotIdentity,
    route_identity: Option<BridgeRouteIdentity>,
    invalidation_identity: Option<BridgeInvalidationIdentity>,
    subscription_slice_identity: Option<BridgeSubscriptionSliceIdentity>,
    contract_proof: Option<BridgeRouteContractProof>,
    counters: BridgeRoutingCounters,
}

impl BridgeFailureSource {
    pub(crate) fn new(
        source_commit: crate::input::envelope::TruthCommitIdentity,
        source_patch: crate::input::envelope::TruthPatchIdentity,
        source_snapshot: TruthSnapshotIdentity,
        counters: BridgeRoutingCounters,
    ) -> Self {
        Self {
            source_commit,
            source_patch,
            source_snapshot,
            route_identity: None,
            invalidation_identity: None,
            subscription_slice_identity: None,
            contract_proof: None,
            counters,
        }
    }

    pub(crate) fn with_route_identity(mut self, route_identity: BridgeRouteIdentity) -> Self {
        self.route_identity = Some(route_identity);
        self
    }

    pub(crate) fn with_invalidation_identity(
        mut self,
        invalidation_identity: BridgeInvalidationIdentity,
    ) -> Self {
        self.invalidation_identity = Some(invalidation_identity);
        self
    }

    pub(crate) fn with_subscription_slice_identity(
        mut self,
        subscription_slice_identity: BridgeSubscriptionSliceIdentity,
    ) -> Self {
        self.subscription_slice_identity = Some(subscription_slice_identity);
        self
    }

    pub(crate) fn with_contract_proof(mut self, contract_proof: BridgeRouteContractProof) -> Self {
        self.contract_proof = Some(contract_proof);
        self
    }

    pub(crate) fn with_counters(mut self, counters: BridgeRoutingCounters) -> Self {
        self.counters = counters;
        self
    }
}

#[derive(Debug, Default)]
struct BridgeDiagnosticsState {
    route_records: VecDeque<Arc<BridgeRouteRecord>>,
    failure_records: VecDeque<Arc<BridgeFailureRecord>>,
    latest_route_by_route_identity: BTreeMap<String, Arc<BridgeRouteRecord>>,
    latest_route_by_invalidation_identity: BTreeMap<String, Arc<BridgeRouteRecord>>,
    latest_route_by_source_commit: BTreeMap<String, Arc<BridgeRouteRecord>>,
}

#[derive(Debug, Clone)]
struct BridgeDiagnosticsConfig {
    tier: BridgeDiagnosticsTier,
    records_enabled: bool,
    replay_enabled: bool,
    route_record_limit: usize,
    failure_record_limit: usize,
}

impl BridgeDiagnosticsState {
    fn route_records(&self) -> Vec<BridgeRouteRecord> {
        self.route_records
            .iter()
            .map(|record| (**record).clone())
            .collect()
    }

    fn failure_records(&self) -> Vec<BridgeFailureRecord> {
        self.failure_records
            .iter()
            .map(|record| (**record).clone())
            .collect()
    }

    fn last_route_record(&self) -> Option<BridgeRouteRecord> {
        self.route_records.back().map(|record| (**record).clone())
    }

    fn last_failure_record(&self) -> Option<BridgeFailureRecord> {
        self.failure_records.back().map(|record| (**record).clone())
    }

    fn record_route(&mut self, record: BridgeRouteRecord, limit: usize) {
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

    fn record_failure(&mut self, record: BridgeFailureRecord, limit: usize) {
        self.failure_records.push_back(Arc::new(record));
        trim_retained_records(&mut self.failure_records, limit, |_| {});
    }

    fn route_record_for_route_identity(&self, route_identity: &str) -> Option<BridgeRouteRecord> {
        self.latest_route_by_route_identity
            .get(route_identity)
            .cloned()
            .map(|record| (*record).clone())
    }

    fn route_record_for_invalidation_identity(
        &self,
        invalidation_identity: &str,
    ) -> Option<BridgeRouteRecord> {
        self.latest_route_by_invalidation_identity
            .get(invalidation_identity)
            .cloned()
            .map(|record| (*record).clone())
    }

    fn route_record_for_source_commit(&self, source_commit: &str) -> Option<BridgeRouteRecord> {
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

#[derive(Debug, Clone)]
pub struct BridgeDiagnosticsFacade {
    config: Arc<BridgeDiagnosticsConfig>,
    state: Arc<RwLock<BridgeDiagnosticsState>>,
}

impl BridgeDiagnosticsFacade {
    pub(crate) fn new(policy: BridgeRuntimePolicy) -> Self {
        let retention_budget = policy.retention_budget();
        Self {
            config: Arc::new(BridgeDiagnosticsConfig {
                tier: policy.diagnostics_tier(),
                records_enabled: policy.record_route_artifacts(),
                replay_enabled: policy.allow_replay_artifacts(),
                route_record_limit: retention_budget.route_record_limit(),
                failure_record_limit: retention_budget.failure_record_limit(),
            }),
            state: Arc::new(RwLock::new(BridgeDiagnosticsState::default())),
        }
    }

    pub fn tier(&self) -> BridgeDiagnosticsTier {
        self.config.tier
    }

    pub fn records_enabled(&self) -> bool {
        self.config.records_enabled
    }

    pub fn route_record_limit(&self) -> usize {
        self.config.route_record_limit
    }

    pub fn failure_record_limit(&self) -> usize {
        self.config.failure_record_limit
    }

    pub fn route_records(&self) -> Vec<BridgeRouteRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .route_records()
    }

    pub fn failure_records(&self) -> Vec<BridgeFailureRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .failure_records()
    }

    pub fn last_failure_record(&self) -> Option<BridgeFailureRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .last_failure_record()
    }

    pub fn last_route_record(&self) -> Option<BridgeRouteRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .last_route_record()
    }

    pub fn route_record_for_route_identity(
        &self,
        route_identity: &str,
    ) -> Option<BridgeRouteRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .route_record_for_route_identity(route_identity)
    }

    pub fn route_record_for_invalidation_identity(
        &self,
        invalidation_identity: &str,
    ) -> Option<BridgeRouteRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .route_record_for_invalidation_identity(invalidation_identity)
    }

    pub fn route_record_for_source_commit(&self, source_commit: &str) -> Option<BridgeRouteRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .route_record_for_source_commit(source_commit)
    }

    pub fn replay_records(&self) -> Vec<BridgeReplayRecord> {
        if !self.config.replay_enabled {
            return Vec::new();
        }

        self.route_records()
            .into_iter()
            .map(BridgeReplayRecord::from_route_record)
            .collect()
    }

    pub fn canonical_route_records(&self) -> Vec<BridgeCanonicalRouteRecord> {
        self.route_records()
            .into_iter()
            .map(BridgeCanonicalRouteRecord::from_route_record)
            .collect()
    }

    pub fn explain_route_record(&self, record: &BridgeRouteRecord) -> BridgeRouteExplanation {
        BridgeRouteExplanation::from_route_record(record)
    }

    pub fn explain_last_route_record(&self) -> Option<BridgeRouteExplanation> {
        self.last_route_record()
            .map(|record| BridgeRouteExplanation::from_route_record(&record))
    }

    pub fn last_canonical_route_record(&self) -> Option<BridgeCanonicalRouteRecord> {
        self.last_route_record()
            .map(BridgeCanonicalRouteRecord::from_route_record)
    }

    pub fn handle(&self) -> BridgeDiagnosticsHandle {
        BridgeDiagnosticsHandle {
            config: Arc::clone(&self.config),
            state: Arc::clone(&self.state),
        }
    }

    pub(crate) fn record_route(&self, record: BridgeRouteRecord) {
        if !self.config.records_enabled {
            return;
        }

        self.state
            .write()
            .expect("bridge diagnostics lock poisoned")
            .record_route(record, self.config.route_record_limit);
    }

    pub(crate) fn record_failure(&self, record: BridgeFailureRecord) {
        self.state
            .write()
            .expect("bridge diagnostics lock poisoned")
            .record_failure(record, self.config.failure_record_limit);
    }

    pub(crate) fn record_delivery_failure(
        &self,
        source: BridgeFailureSource,
        error: &BridgeDeliveryError,
    ) {
        self.record_failure(BridgeFailureRecord::from_failure(
            source,
            BridgeFailureClass::Delivery(error.kind()),
            error.to_string(),
            error.context().clone(),
        ));
    }

    pub(crate) fn record_replay_failure(
        &self,
        source: BridgeFailureSource,
        error: &BridgeReplayError,
    ) {
        self.record_failure(BridgeFailureRecord::from_failure(
            source,
            BridgeFailureClass::Replay(error.kind()),
            error.to_string(),
            error.context().clone(),
        ));
    }
}

impl DiagnosticSink for BridgeDiagnosticsFacade {
    fn record_route(&self, record: BridgeRouteRecord) {
        BridgeDiagnosticsFacade::record_route(self, record);
    }

    fn record_delivery_failure(&self, source: BridgeFailureSource, error: &BridgeDeliveryError) {
        BridgeDiagnosticsFacade::record_delivery_failure(self, source, error);
    }

    fn record_replay_failure(&self, source: BridgeFailureSource, error: &BridgeReplayError) {
        BridgeDiagnosticsFacade::record_replay_failure(self, source, error);
    }
}

#[derive(Debug, Clone)]
pub struct BridgeDiagnosticsHandle {
    config: Arc<BridgeDiagnosticsConfig>,
    state: Arc<RwLock<BridgeDiagnosticsState>>,
}

impl BridgeDiagnosticsHandle {
    pub fn tier(&self) -> BridgeDiagnosticsTier {
        self.config.tier
    }

    pub fn records_enabled(&self) -> bool {
        self.config.records_enabled
    }

    pub fn replay_enabled(&self) -> bool {
        self.config.replay_enabled
    }

    pub fn route_record_limit(&self) -> usize {
        self.config.route_record_limit
    }

    pub fn failure_record_limit(&self) -> usize {
        self.config.failure_record_limit
    }

    pub fn route_records(&self) -> Vec<BridgeRouteRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .route_records()
    }

    pub fn failure_records(&self) -> Vec<BridgeFailureRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .failure_records()
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

fn evict_index_entry(
    index: &mut BTreeMap<String, Arc<BridgeRouteRecord>>,
    key: &str,
    evicted: &Arc<BridgeRouteRecord>,
) {
    if let Some(current) = index.get(key) {
        if Arc::ptr_eq(current, evicted) {
            index.remove(key);
        }
    }
}
