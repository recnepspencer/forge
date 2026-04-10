use std::sync::{Arc, RwLock};

use crate::routing::BridgeCanonicalBulkPlanRecord;
use crate::speculation::{
    BridgePreviewDiscardRecord, BridgePreviewExecutionRecord, BridgePreviewPromotionRecord,
};
use crate::writeback::{
    BridgeMappedWritebackFamilyInput, BridgeWritebackExecutionRecord,
    BridgeWritebackFamilyAdmissionRecord,
    BridgeWritebackMapperEnvelope, BridgeWritebackMapperRecord, BridgeWritebackReplayRecord,
};

use super::continuity::BridgeCanonicalContinuityRecord;
use super::history::{
    BridgeCanonicalHistoricalEvaluationRecord, BridgeHistoricalEvaluationFailureRecord,
};
use super::records::{BridgeFailureRecord, BridgeRouteRecord};
use super::state::{BridgeDiagnosticsConfig, BridgeDiagnosticsState};

#[derive(Debug, Clone)]
pub struct BridgeDiagnosticsHandle {
    pub(super) config: Arc<BridgeDiagnosticsConfig>,
    pub(super) state: Arc<RwLock<BridgeDiagnosticsState>>,
}

impl BridgeDiagnosticsHandle {
    pub fn tier(&self) -> crate::policy::BridgeDiagnosticsTier {
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

    pub fn preview_execution_records(&self) -> Vec<BridgePreviewExecutionRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .preview_execution_records()
    }

    pub fn preview_discard_records(&self) -> Vec<BridgePreviewDiscardRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .preview_discard_records()
    }

    pub fn preview_promotion_records(&self) -> Vec<BridgePreviewPromotionRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .preview_promotion_records()
    }

    pub fn writeback_admission_records(&self) -> Vec<BridgeWritebackFamilyAdmissionRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .writeback_admission_records()
    }

    pub fn writeback_execution_records(&self) -> Vec<BridgeWritebackExecutionRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .writeback_execution_records()
    }

    pub fn writeback_mapper_envelopes(&self) -> Vec<BridgeWritebackMapperEnvelope> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .writeback_mapper_envelopes()
    }

    pub fn writeback_mapped_family_inputs(&self) -> Vec<BridgeMappedWritebackFamilyInput> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .writeback_mapped_family_inputs()
    }

    pub fn writeback_mapper_records(&self) -> Vec<BridgeWritebackMapperRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .writeback_mapper_records()
    }

    pub fn writeback_replay_records(&self) -> Vec<BridgeWritebackReplayRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .writeback_replay_records()
    }

    pub fn last_preview_execution_record(&self) -> Option<BridgePreviewExecutionRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .last_preview_execution_record()
    }

    pub fn last_preview_discard_record(&self) -> Option<BridgePreviewDiscardRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .last_preview_discard_record()
    }

    pub fn last_preview_promotion_record(&self) -> Option<BridgePreviewPromotionRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .last_preview_promotion_record()
    }

    pub fn last_writeback_admission_record(&self) -> Option<BridgeWritebackFamilyAdmissionRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .last_writeback_admission_record()
    }

    pub fn last_writeback_execution_record(&self) -> Option<BridgeWritebackExecutionRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .last_writeback_execution_record()
    }

    pub fn last_writeback_mapper_envelope(&self) -> Option<BridgeWritebackMapperEnvelope> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .last_writeback_mapper_envelope()
    }

    pub fn last_writeback_mapped_family_input(&self) -> Option<BridgeMappedWritebackFamilyInput> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .last_writeback_mapped_family_input()
    }

    pub fn last_writeback_mapper_record(&self) -> Option<BridgeWritebackMapperRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .last_writeback_mapper_record()
    }

    pub fn last_writeback_replay_record(&self) -> Option<BridgeWritebackReplayRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .last_writeback_replay_record()
    }

    pub fn preview_execution_record_for_identity(
        &self,
        record_identity: &str,
    ) -> Option<BridgePreviewExecutionRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .preview_execution_record_for_identity(record_identity)
    }

    pub fn preview_execution_record_for_session_identity(
        &self,
        session_identity: &str,
    ) -> Option<BridgePreviewExecutionRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .preview_execution_record_for_session_identity(session_identity)
    }

    pub fn preview_discard_record_for_identity(
        &self,
        record_identity: &str,
    ) -> Option<BridgePreviewDiscardRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .preview_discard_record_for_identity(record_identity)
    }

    pub fn preview_discard_record_for_session_identity(
        &self,
        session_identity: &str,
    ) -> Option<BridgePreviewDiscardRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .preview_discard_record_for_session_identity(session_identity)
    }

    pub fn preview_promotion_record_for_identity(
        &self,
        record_identity: &str,
    ) -> Option<BridgePreviewPromotionRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .preview_promotion_record_for_identity(record_identity)
    }

    pub fn preview_promotion_record_for_session_identity(
        &self,
        session_identity: &str,
    ) -> Option<BridgePreviewPromotionRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .preview_promotion_record_for_session_identity(session_identity)
    }

    pub fn writeback_admission_record_for_identity(
        &self,
        record_identity: &str,
    ) -> Option<BridgeWritebackFamilyAdmissionRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .writeback_admission_record_for_identity(record_identity)
    }

    pub fn writeback_admission_record_for_contract_digest(
        &self,
        contract_digest: &str,
    ) -> Option<BridgeWritebackFamilyAdmissionRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .writeback_admission_record_for_contract_digest(contract_digest)
    }

    pub fn writeback_execution_record_for_identity(
        &self,
        record_identity: &str,
    ) -> Option<BridgeWritebackExecutionRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .writeback_execution_record_for_identity(record_identity)
    }

    pub fn writeback_execution_record_for_candidate_digest(
        &self,
        candidate_digest: &str,
    ) -> Option<BridgeWritebackExecutionRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .writeback_execution_record_for_candidate_digest(candidate_digest)
    }

    pub fn writeback_mapper_envelope_for_identity(
        &self,
        envelope_identity: &str,
    ) -> Option<BridgeWritebackMapperEnvelope> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .writeback_mapper_envelope_for_identity(envelope_identity)
    }

    pub fn writeback_mapper_envelope_for_digest(
        &self,
        envelope_digest: &str,
    ) -> Option<BridgeWritebackMapperEnvelope> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .writeback_mapper_envelope_for_digest(envelope_digest)
    }

    pub fn writeback_mapped_family_input_for_identity(
        &self,
        mapped_input_identity: &str,
    ) -> Option<BridgeMappedWritebackFamilyInput> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .writeback_mapped_family_input_for_identity(mapped_input_identity)
    }

    pub fn writeback_mapped_family_input_for_digest(
        &self,
        mapped_input_digest: &str,
    ) -> Option<BridgeMappedWritebackFamilyInput> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .writeback_mapped_family_input_for_digest(mapped_input_digest)
    }

    pub fn writeback_mapper_record_for_identity(
        &self,
        record_identity: &str,
    ) -> Option<BridgeWritebackMapperRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .writeback_mapper_record_for_identity(record_identity)
    }

    pub fn writeback_mapper_record_for_candidate_digest(
        &self,
        candidate_digest: &str,
    ) -> Option<BridgeWritebackMapperRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .writeback_mapper_record_for_candidate_digest(candidate_digest)
    }

    pub fn writeback_replay_record_for_identity(
        &self,
        record_identity: &str,
    ) -> Option<BridgeWritebackReplayRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .writeback_replay_record_for_identity(record_identity)
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

    pub fn last_historical_evaluation_failure(
        &self,
    ) -> Option<BridgeHistoricalEvaluationFailureRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .last_historical_failure()
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

    pub fn bulk_record_for_workload_identity(
        &self,
        workload_identity: &str,
    ) -> Option<BridgeCanonicalBulkPlanRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .bulk_record_for_workload_identity(workload_identity)
    }
}
