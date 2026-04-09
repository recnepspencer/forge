use super::*;

impl BridgeDiagnosticsFacade {
    pub(crate) fn reserve_preview_session_identity(
        &self,
        session_identity: &str,
    ) -> bool {
        self.state
            .write()
            .expect("bridge diagnostics lock poisoned")
            .reserve_preview_session_identity(session_identity)
    }

    pub(crate) fn record_preview_execution(&self, record: BridgePreviewExecutionRecord) {
        if !self.config.records_enabled {
            return;
        }

        self.state
            .write()
            .expect("bridge diagnostics lock poisoned")
            .record_preview_execution(record, self.config.route_record_limit);
    }

    pub(crate) fn record_preview_discard(&self, record: BridgePreviewDiscardRecord) {
        if !self.config.records_enabled {
            return;
        }

        self.state
            .write()
            .expect("bridge diagnostics lock poisoned")
            .record_preview_discard(record, self.config.route_record_limit);
    }

    pub(crate) fn record_preview_promotion(&self, record: BridgePreviewPromotionRecord) {
        if !self.config.records_enabled {
            return;
        }

        self.state
            .write()
            .expect("bridge diagnostics lock poisoned")
            .record_preview_promotion(record, self.config.route_record_limit);
    }
}
