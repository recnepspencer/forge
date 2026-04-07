use std::sync::{Arc, RwLock};

use crate::error::{BridgeDeliveryError, BridgeReplayError};
use crate::policy::{BridgeDiagnosticsTier, BridgeRuntimePolicy};

use super::failure_source::BridgeFailureSource;
use super::handle::BridgeDiagnosticsHandle;
use super::records::{
    BridgeFailureClass, BridgeFailureRecord, BridgeRouteRecord,
};
use super::replay::{BridgeCanonicalRouteRecord, BridgeReplayRecord};
use super::sink::DiagnosticSink;
use super::state::{BridgeDiagnosticsConfig, BridgeDiagnosticsState};
use super::{BridgeRouteExplanation};

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
