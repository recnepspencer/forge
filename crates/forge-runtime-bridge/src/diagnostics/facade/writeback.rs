use super::*;

impl BridgeDiagnosticsFacade {
    pub(crate) fn record_writeback_admission(&self, record: BridgeWritebackFamilyAdmissionRecord) {
        if !self.config.records_enabled {
            return;
        }

        self.state
            .write()
            .expect("bridge diagnostics lock poisoned")
            .record_writeback_admission(record, self.config.route_record_limit);
    }

    pub(crate) fn record_writeback_mapped_family_input(
        &self,
        mapped_input: BridgeMappedWritebackFamilyInput,
    ) {
        if !self.config.records_enabled {
            return;
        }

        self.state
            .write()
            .expect("bridge diagnostics lock poisoned")
            .record_writeback_mapped_input(mapped_input, self.config.route_record_limit);
    }

    pub(crate) fn record_writeback_mapper_envelope(&self, envelope: BridgeWritebackMapperEnvelope) {
        if !self.config.records_enabled {
            return;
        }

        self.state
            .write()
            .expect("bridge diagnostics lock poisoned")
            .record_writeback_mapper_envelope(envelope, self.config.route_record_limit);
    }

    pub(crate) fn record_writeback_mapper(&self, record: BridgeWritebackMapperRecord) {
        if !self.config.records_enabled {
            return;
        }

        self.state
            .write()
            .expect("bridge diagnostics lock poisoned")
            .record_writeback_mapper(record, self.config.route_record_limit);
    }

    pub(crate) fn record_writeback_execution(&self, record: BridgeWritebackExecutionRecord) {
        if !self.config.records_enabled {
            return;
        }

        self.state
            .write()
            .expect("bridge diagnostics lock poisoned")
            .record_writeback_execution(record, self.config.route_record_limit);
    }

    pub(crate) fn annotate_last_writeback_execution_record(
        &self,
        execution_receipt_digest: impl Into<std::sync::Arc<str>>,
    ) {
        if !self.config.records_enabled {
            return;
        }

        self.state
            .write()
            .expect("bridge diagnostics lock poisoned")
            .annotate_last_writeback_execution_record(execution_receipt_digest);
    }

    pub(crate) fn record_writeback_replay(&self, record: BridgeWritebackReplayRecord) {
        if !self.config.records_enabled {
            return;
        }

        self.state
            .write()
            .expect("bridge diagnostics lock poisoned")
            .record_writeback_replay(record, self.config.route_record_limit);
    }
}
