use std::collections::{BTreeMap, VecDeque};
use std::sync::Arc;

use crate::routing::BridgeCanonicalBulkPlanRecord;

use super::continuity::BridgeCanonicalContinuityRecord;
use super::history::{
    BridgeCanonicalHistoricalEvaluationRecord, BridgeHistoricalEvaluationFailureRecord,
};
use super::records::{BridgeFailureRecord, BridgeRouteRecord};

#[derive(Debug, Default)]
pub(super) struct BridgeDiagnosticsState {
    route_records: VecDeque<Arc<BridgeRouteRecord>>,
    bulk_records: VecDeque<Arc<BridgeCanonicalBulkPlanRecord>>,
    continuity_records: VecDeque<Arc<BridgeCanonicalContinuityRecord>>,
    historical_records: VecDeque<Arc<BridgeCanonicalHistoricalEvaluationRecord>>,
    historical_failures: VecDeque<Arc<BridgeHistoricalEvaluationFailureRecord>>,
    failure_records: VecDeque<Arc<BridgeFailureRecord>>,
    latest_route_by_route_identity: BTreeMap<String, Arc<BridgeRouteRecord>>,
    latest_bulk_by_workload_identity: BTreeMap<String, Arc<BridgeCanonicalBulkPlanRecord>>,
    latest_continuity_by_route_identity: BTreeMap<String, Arc<BridgeCanonicalContinuityRecord>>,
    latest_historical_by_record_identity: BTreeMap<String, Arc<BridgeCanonicalHistoricalEvaluationRecord>>,
    latest_historical_by_decision_log_identity: BTreeMap<String, Arc<BridgeCanonicalHistoricalEvaluationRecord>>,
    latest_historical_failure_by_declaration_identity: BTreeMap<String, Arc<BridgeHistoricalEvaluationFailureRecord>>,
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

    pub(super) fn bulk_records(&self) -> Vec<BridgeCanonicalBulkPlanRecord> {
        self.bulk_records
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

    pub(super) fn continuity_records(&self) -> Vec<BridgeCanonicalContinuityRecord> {
        self.continuity_records
            .iter()
            .map(|record| (**record).clone())
            .collect()
    }

    pub(super) fn historical_records(&self) -> Vec<BridgeCanonicalHistoricalEvaluationRecord> {
        self.historical_records
            .iter()
            .map(|record| (**record).clone())
            .collect()
    }

    pub(super) fn historical_failures(&self) -> Vec<BridgeHistoricalEvaluationFailureRecord> {
        self.historical_failures
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

    pub(super) fn last_bulk_record(&self) -> Option<BridgeCanonicalBulkPlanRecord> {
        self.bulk_records.back().map(|record| (**record).clone())
    }

    pub(super) fn last_continuity_record(&self) -> Option<BridgeCanonicalContinuityRecord> {
        self.continuity_records
            .back()
            .map(|record| (**record).clone())
    }

    pub(super) fn last_historical_record(&self) -> Option<BridgeCanonicalHistoricalEvaluationRecord> {
        self.historical_records
            .back()
            .map(|record| (**record).clone())
    }

    pub(super) fn last_historical_failure(&self) -> Option<BridgeHistoricalEvaluationFailureRecord> {
        self.historical_failures
            .back()
            .map(|record| (**record).clone())
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

    pub(super) fn record_bulk(&mut self, record: BridgeCanonicalBulkPlanRecord, limit: usize) {
        let record = Arc::new(record);
        self.latest_bulk_by_workload_identity.insert(
            record.workload_identity().as_str().to_string(),
            Arc::clone(&record),
        );
        self.bulk_records.push_back(record);
        while self.bulk_records.len() > limit.max(1) {
            if let Some(evicted) = self.bulk_records.pop_front() {
                self.evict_bulk_indexes(&evicted);
            }
        }
    }

    pub(super) fn record_failure(&mut self, record: BridgeFailureRecord, limit: usize) {
        self.failure_records.push_back(Arc::new(record));
        trim_retained_records(&mut self.failure_records, limit, |_| {});
    }

    pub(super) fn record_continuity(
        &mut self,
        record: BridgeCanonicalContinuityRecord,
        limit: usize,
    ) {
        let record = Arc::new(record);
        self.latest_continuity_by_route_identity.insert(
            record.route_identity().as_str().to_string(),
            Arc::clone(&record),
        );
        self.continuity_records.push_back(record);
        while self.continuity_records.len() > limit.max(1) {
            if let Some(evicted) = self.continuity_records.pop_front() {
                self.evict_continuity_indexes(&evicted);
            }
        }
    }

    pub(super) fn record_historical(
        &mut self,
        record: BridgeCanonicalHistoricalEvaluationRecord,
        limit: usize,
    ) {
        let record = Arc::new(record);
        self.latest_historical_by_record_identity.insert(
            record.record_identity().as_str().to_string(),
            Arc::clone(&record),
        );
        self.latest_historical_by_decision_log_identity.insert(
            record.decision_log().decision_log_identity().as_str().to_string(),
            Arc::clone(&record),
        );
        self.historical_records.push_back(record);
        while self.historical_records.len() > limit.max(1) {
            if let Some(evicted) = self.historical_records.pop_front() {
                self.evict_historical_indexes(&evicted);
            }
        }
    }

    pub(super) fn record_historical_failure(
        &mut self,
        record: BridgeHistoricalEvaluationFailureRecord,
        limit: usize,
    ) {
        let record = Arc::new(record);
        self.latest_historical_failure_by_declaration_identity.insert(
            record.declaration_identity().as_str().to_string(),
            Arc::clone(&record),
        );
        self.historical_failures.push_back(record);
        while self.historical_failures.len() > limit.max(1) {
            if let Some(evicted) = self.historical_failures.pop_front() {
                self.evict_historical_failure_indexes(&evicted);
            }
        }
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

    pub(super) fn continuity_record_for_route_identity(
        &self,
        route_identity: &str,
    ) -> Option<BridgeCanonicalContinuityRecord> {
        self.latest_continuity_by_route_identity
            .get(route_identity)
            .cloned()
            .map(|record| (*record).clone())
    }

    pub(super) fn bulk_record_for_workload_identity(
        &self,
        workload_identity: &str,
    ) -> Option<BridgeCanonicalBulkPlanRecord> {
        self.latest_bulk_by_workload_identity
            .get(workload_identity)
            .cloned()
            .map(|record| (*record).clone())
    }

    pub(super) fn historical_record_for_record_identity(
        &self,
        record_identity: &str,
    ) -> Option<BridgeCanonicalHistoricalEvaluationRecord> {
        self.latest_historical_by_record_identity
            .get(record_identity)
            .cloned()
            .map(|record| (*record).clone())
    }

    pub(super) fn historical_record_for_decision_log_identity(
        &self,
        decision_log_identity: &str,
    ) -> Option<BridgeCanonicalHistoricalEvaluationRecord> {
        self.latest_historical_by_decision_log_identity
            .get(decision_log_identity)
            .cloned()
            .map(|record| (*record).clone())
    }

    pub(super) fn historical_failure_for_declaration_identity(
        &self,
        declaration_identity: &str,
    ) -> Option<BridgeHistoricalEvaluationFailureRecord> {
        self.latest_historical_failure_by_declaration_identity
            .get(declaration_identity)
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

    fn evict_bulk_indexes(&mut self, evicted: &Arc<BridgeCanonicalBulkPlanRecord>) {
        evict_index_entry(
            &mut self.latest_bulk_by_workload_identity,
            evicted.workload_identity().as_str(),
            evicted,
        );
    }

    fn evict_continuity_indexes(&mut self, evicted: &Arc<BridgeCanonicalContinuityRecord>) {
        evict_index_entry(
            &mut self.latest_continuity_by_route_identity,
            evicted.route_identity().as_str(),
            evicted,
        );
    }

    fn evict_historical_indexes(&mut self, evicted: &Arc<BridgeCanonicalHistoricalEvaluationRecord>) {
        evict_index_entry(
            &mut self.latest_historical_by_record_identity,
            evicted.record_identity().as_str(),
            evicted,
        );
        evict_index_entry(
            &mut self.latest_historical_by_decision_log_identity,
            evicted.decision_log().decision_log_identity().as_str(),
            evicted,
        );
    }

    fn evict_historical_failure_indexes(
        &mut self,
        evicted: &Arc<BridgeHistoricalEvaluationFailureRecord>,
    ) {
        evict_index_entry(
            &mut self.latest_historical_failure_by_declaration_identity,
            evicted.declaration_identity().as_str(),
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
