use super::*;
use crate::speculation::{
    BridgePreviewDiscardRecordIdentity, BridgePreviewPromotionRecordIdentity,
    BridgePreviewSessionIdentity, PreviewExecutionRecordIdentity,
};

impl BridgeDiagnosticsFacade {
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

    pub fn preview_execution_record_for_identity(
        &self,
        record_identity: &PreviewExecutionRecordIdentity,
    ) -> Option<BridgePreviewExecutionRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .preview_execution_record_for_identity(record_identity)
    }

    pub fn preview_execution_record_for_session_identity(
        &self,
        session_identity: &BridgePreviewSessionIdentity,
    ) -> Option<BridgePreviewExecutionRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .preview_execution_record_for_session_identity(session_identity)
    }

    pub fn preview_discard_record_for_identity(
        &self,
        record_identity: &BridgePreviewDiscardRecordIdentity,
    ) -> Option<BridgePreviewDiscardRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .preview_discard_record_for_identity(record_identity)
    }

    pub fn preview_discard_record_for_session_identity(
        &self,
        session_identity: &BridgePreviewSessionIdentity,
    ) -> Option<BridgePreviewDiscardRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .preview_discard_record_for_session_identity(session_identity)
    }

    pub fn preview_promotion_record_for_identity(
        &self,
        record_identity: &BridgePreviewPromotionRecordIdentity,
    ) -> Option<BridgePreviewPromotionRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .preview_promotion_record_for_identity(record_identity)
    }

    pub fn preview_promotion_record_for_session_identity(
        &self,
        session_identity: &BridgePreviewSessionIdentity,
    ) -> Option<BridgePreviewPromotionRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .preview_promotion_record_for_session_identity(session_identity)
    }
}
