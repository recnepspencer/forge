use std::sync::{Arc, RwLock};

use crate::error::{BridgeDeliveryError, BridgeReplayError};
use crate::policy::{BridgeDiagnosticsTier, BridgeRuntimePolicy};
use crate::routing::BridgeCanonicalBulkPlanRecord;

use super::bulk::BridgeBulkPlanExplanation;
use super::continuity::{BridgeCanonicalContinuityRecord, BridgeContinuityExplanation};
use super::failure_source::BridgeFailureSource;
use super::handle::BridgeDiagnosticsHandle;
use super::history::{
    BridgeCanonicalHistoricalEvaluationRecord, BridgeHistoricalEvaluationExplanation,
    BridgeHistoricalEvaluationFailureRecord,
};
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

    pub fn bulk_records(&self) -> Vec<BridgeCanonicalBulkPlanRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .bulk_records()
    }

    pub fn continuity_records(&self) -> Vec<BridgeCanonicalContinuityRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .continuity_records()
    }

    pub fn historical_evaluation_records(&self) -> Vec<BridgeCanonicalHistoricalEvaluationRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .historical_records()
    }

    pub fn historical_evaluation_failures(&self) -> Vec<BridgeHistoricalEvaluationFailureRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .historical_failures()
    }

    pub fn last_failure_record(&self) -> Option<BridgeFailureRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .last_failure_record()
    }

    pub fn last_canonical_continuity_record(&self) -> Option<BridgeCanonicalContinuityRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .last_continuity_record()
    }

    pub fn last_bulk_record(&self) -> Option<BridgeCanonicalBulkPlanRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .last_bulk_record()
    }

    pub fn last_historical_evaluation_record(&self) -> Option<BridgeCanonicalHistoricalEvaluationRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .last_historical_record()
    }

    pub fn last_historical_evaluation_failure(&self) -> Option<BridgeHistoricalEvaluationFailureRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .last_historical_failure()
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

    pub fn continuity_record_for_route_identity(
        &self,
        route_identity: &str,
    ) -> Option<BridgeCanonicalContinuityRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .continuity_record_for_route_identity(route_identity)
    }

    pub fn bulk_record_for_workload_identity(
        &self,
        workload_identity: &str,
    ) -> Option<BridgeCanonicalBulkPlanRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .bulk_record_for_workload_identity(workload_identity)
    }

    pub fn historical_record_for_record_identity(
        &self,
        record_identity: &str,
    ) -> Option<BridgeCanonicalHistoricalEvaluationRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .historical_record_for_record_identity(record_identity)
    }

    pub fn historical_record_for_decision_log_identity(
        &self,
        decision_log_identity: &str,
    ) -> Option<BridgeCanonicalHistoricalEvaluationRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .historical_record_for_decision_log_identity(decision_log_identity)
    }

    pub fn historical_failure_for_declaration_identity(
        &self,
        declaration_identity: &str,
    ) -> Option<BridgeHistoricalEvaluationFailureRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .historical_failure_for_declaration_identity(declaration_identity)
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

    pub fn explain_continuity_record(
        &self,
        record: &BridgeCanonicalContinuityRecord,
    ) -> BridgeContinuityExplanation {
        BridgeContinuityExplanation::from_canonical_record(record)
    }

    pub fn explain_last_continuity_record(&self) -> Option<BridgeContinuityExplanation> {
        self.last_canonical_continuity_record()
            .map(|record| BridgeContinuityExplanation::from_canonical_record(&record))
    }

    pub fn explain_bulk_record(
        &self,
        record: &BridgeCanonicalBulkPlanRecord,
    ) -> BridgeBulkPlanExplanation {
        BridgeBulkPlanExplanation::from_canonical_record(record)
    }

    pub fn explain_last_bulk_record(&self) -> Option<BridgeBulkPlanExplanation> {
        self.last_bulk_record()
            .map(|record| BridgeBulkPlanExplanation::from_canonical_record(&record))
    }

    pub fn explain_historical_evaluation_record(
        &self,
        record: &BridgeCanonicalHistoricalEvaluationRecord,
    ) -> BridgeHistoricalEvaluationExplanation {
        BridgeHistoricalEvaluationExplanation::from_canonical_record(record)
    }

    pub fn explain_last_historical_evaluation_record(
        &self,
    ) -> Option<BridgeHistoricalEvaluationExplanation> {
        self.last_historical_evaluation_record()
            .map(|record| BridgeHistoricalEvaluationExplanation::from_canonical_record(&record))
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

    pub(crate) fn record_continuity(&self, record: BridgeCanonicalContinuityRecord) {
        if !self.config.records_enabled {
            return;
        }

        self.state
            .write()
            .expect("bridge diagnostics lock poisoned")
            .record_continuity(record, self.config.route_record_limit);
    }

    pub(crate) fn record_bulk(&self, record: BridgeCanonicalBulkPlanRecord) {
        if !self.config.records_enabled {
            return;
        }

        self.state
            .write()
            .expect("bridge diagnostics lock poisoned")
            .record_bulk(record, self.config.route_record_limit);
    }

    pub(crate) fn record_historical_evaluation(
        &self,
        record: BridgeCanonicalHistoricalEvaluationRecord,
    ) {
        if !self.config.records_enabled {
            return;
        }

        self.state
            .write()
            .expect("bridge diagnostics lock poisoned")
            .record_historical(record, self.config.route_record_limit);
    }

    pub(crate) fn record_historical_evaluation_failure(
        &self,
        record: BridgeHistoricalEvaluationFailureRecord,
    ) {
        if !self.config.records_enabled {
            return;
        }

        self.state
            .write()
            .expect("bridge diagnostics lock poisoned")
            .record_historical_failure(record, self.config.route_record_limit);
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

    fn record_historical_evaluation(&self, record: BridgeCanonicalHistoricalEvaluationRecord) {
        BridgeDiagnosticsFacade::record_historical_evaluation(self, record);
    }

    fn record_historical_evaluation_failure(&self, record: BridgeHistoricalEvaluationFailureRecord) {
        BridgeDiagnosticsFacade::record_historical_evaluation_failure(self, record);
    }

    fn record_delivery_failure(&self, source: BridgeFailureSource, error: &BridgeDeliveryError) {
        BridgeDiagnosticsFacade::record_delivery_failure(self, source, error);
    }

    fn record_replay_failure(&self, source: BridgeFailureSource, error: &BridgeReplayError) {
        BridgeDiagnosticsFacade::record_replay_failure(self, source, error);
    }
}
