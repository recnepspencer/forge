use super::*;

impl BridgeDiagnosticsState {
    pub(crate) fn record_writeback_admission(
        &mut self,
        record: BridgeWritebackFamilyAdmissionRecord,
        limit: usize,
    ) {
        let record = Arc::new(record);
        self.latest_writeback_admission_by_record_identity.insert(
            record.record_identity().as_str().to_string(),
            Arc::clone(&record),
        );
        self.latest_writeback_admission_by_contract_digest
            .insert(record.contract_digest().to_string(), Arc::clone(&record));
        self.writeback_admission_records.push_back(record);
        while self.writeback_admission_records.len() > limit.max(1) {
            if let Some(evicted) = self.writeback_admission_records.pop_front() {
                if self
                    .latest_writeback_admission_by_record_identity
                    .get(evicted.record_identity().as_str())
                    .is_some_and(|current| Arc::ptr_eq(current, &evicted))
                {
                    self.latest_writeback_admission_by_record_identity
                        .remove(evicted.record_identity().as_str());
                }
                if self
                    .latest_writeback_admission_by_contract_digest
                    .get(evicted.contract_digest())
                    .is_some_and(|current| Arc::ptr_eq(current, &evicted))
                {
                    self.latest_writeback_admission_by_contract_digest
                        .remove(evicted.contract_digest());
                }
            }
        }
    }

    pub(crate) fn record_writeback_mapped_input(
        &mut self,
        mapped_input: BridgeMappedWritebackFamilyInput,
        limit: usize,
    ) {
        let mapped_input = Arc::new(mapped_input);
        self.latest_writeback_mapped_input_by_identity.insert(
            mapped_input.mapped_input_identity().as_str().to_string(),
            Arc::clone(&mapped_input),
        );
        self.latest_writeback_mapped_input_by_digest
            .insert(mapped_input.digest().to_string(), Arc::clone(&mapped_input));
        self.writeback_mapped_family_inputs.push_back(mapped_input);
        while self.writeback_mapped_family_inputs.len() > limit.max(1) {
            if let Some(evicted) = self.writeback_mapped_family_inputs.pop_front() {
                if self
                    .latest_writeback_mapped_input_by_identity
                    .get(evicted.mapped_input_identity().as_str())
                    .is_some_and(|current| Arc::ptr_eq(current, &evicted))
                {
                    self.latest_writeback_mapped_input_by_identity
                        .remove(evicted.mapped_input_identity().as_str());
                }
                if self
                    .latest_writeback_mapped_input_by_digest
                    .get(evicted.digest())
                    .is_some_and(|current| Arc::ptr_eq(current, &evicted))
                {
                    self.latest_writeback_mapped_input_by_digest
                        .remove(evicted.digest());
                }
            }
        }
    }

    pub(crate) fn record_writeback_mapper_envelope(
        &mut self,
        envelope: BridgeWritebackMapperEnvelope,
        limit: usize,
    ) {
        let envelope = Arc::new(envelope);
        self.latest_writeback_mapper_envelope_by_identity.insert(
            envelope.envelope_identity().as_str().to_string(),
            Arc::clone(&envelope),
        );
        self.latest_writeback_mapper_envelope_by_digest
            .insert(envelope.digest().to_string(), Arc::clone(&envelope));
        self.writeback_mapper_envelopes.push_back(envelope);
        while self.writeback_mapper_envelopes.len() > limit.max(1) {
            if let Some(evicted) = self.writeback_mapper_envelopes.pop_front() {
                if self
                    .latest_writeback_mapper_envelope_by_identity
                    .get(evicted.envelope_identity().as_str())
                    .is_some_and(|current| Arc::ptr_eq(current, &evicted))
                {
                    self.latest_writeback_mapper_envelope_by_identity
                        .remove(evicted.envelope_identity().as_str());
                }
                if self
                    .latest_writeback_mapper_envelope_by_digest
                    .get(evicted.digest())
                    .is_some_and(|current| Arc::ptr_eq(current, &evicted))
                {
                    self.latest_writeback_mapper_envelope_by_digest
                        .remove(evicted.digest());
                }
            }
        }
    }

    pub(crate) fn record_writeback_mapper(
        &mut self,
        record: BridgeWritebackMapperRecord,
        limit: usize,
    ) {
        let record = Arc::new(record);
        self.latest_writeback_mapper_by_record_identity.insert(
            record.record_identity().as_str().to_string(),
            Arc::clone(&record),
        );
        self.latest_writeback_mapper_by_candidate_digest
            .insert(record.candidate_digest().to_string(), Arc::clone(&record));
        self.writeback_mapper_records.push_back(record);
        while self.writeback_mapper_records.len() > limit.max(1) {
            if let Some(evicted) = self.writeback_mapper_records.pop_front() {
                if self
                    .latest_writeback_mapper_by_record_identity
                    .get(evicted.record_identity().as_str())
                    .is_some_and(|current| Arc::ptr_eq(current, &evicted))
                {
                    self.latest_writeback_mapper_by_record_identity
                        .remove(evicted.record_identity().as_str());
                }
                if self
                    .latest_writeback_mapper_by_candidate_digest
                    .get(evicted.candidate_digest())
                    .is_some_and(|current| Arc::ptr_eq(current, &evicted))
                {
                    self.latest_writeback_mapper_by_candidate_digest
                        .remove(evicted.candidate_digest());
                }
            }
        }
    }

    pub(crate) fn record_writeback_execution(
        &mut self,
        record: BridgeWritebackExecutionRecord,
        limit: usize,
    ) {
        let record = Arc::new(record);
        self.latest_writeback_execution_by_record_identity.insert(
            record.record_identity().as_str().to_string(),
            Arc::clone(&record),
        );
        if let Some(candidate_digest) = record.candidate_digest() {
            self.latest_writeback_execution_by_candidate_digest
                .insert(candidate_digest.to_string(), Arc::clone(&record));
        }
        self.writeback_execution_records.push_back(record);
        while self.writeback_execution_records.len() > limit.max(1) {
            if let Some(evicted) = self.writeback_execution_records.pop_front() {
                if self
                    .latest_writeback_execution_by_record_identity
                    .get(evicted.record_identity().as_str())
                    .is_some_and(|current| Arc::ptr_eq(current, &evicted))
                {
                    self.latest_writeback_execution_by_record_identity
                        .remove(evicted.record_identity().as_str());
                }
                if let Some(candidate_digest) = evicted.candidate_digest() {
                    if self
                        .latest_writeback_execution_by_candidate_digest
                        .get(candidate_digest)
                        .is_some_and(|current| Arc::ptr_eq(current, &evicted))
                    {
                        self.latest_writeback_execution_by_candidate_digest
                            .remove(candidate_digest);
                    }
                }
            }
        }
    }

    pub(crate) fn record_writeback_replay(
        &mut self,
        record: BridgeWritebackReplayRecord,
        limit: usize,
    ) {
        let record = Arc::new(record);
        self.latest_writeback_replay_by_record_identity.insert(
            record.record_identity().as_str().to_string(),
            Arc::clone(&record),
        );
        self.writeback_replay_records.push_back(record);
        while self.writeback_replay_records.len() > limit.max(1) {
            if let Some(evicted) = self.writeback_replay_records.pop_front() {
                if self
                    .latest_writeback_replay_by_record_identity
                    .get(evicted.record_identity().as_str())
                    .is_some_and(|current| Arc::ptr_eq(current, &evicted))
                {
                    self.latest_writeback_replay_by_record_identity
                        .remove(evicted.record_identity().as_str());
                }
            }
        }
    }

    pub(crate) fn writeback_admission_records(&self) -> Vec<BridgeWritebackFamilyAdmissionRecord> {
        self.writeback_admission_records
            .iter()
            .map(|record| (**record).clone())
            .collect()
    }

    pub(crate) fn writeback_execution_records(&self) -> Vec<BridgeWritebackExecutionRecord> {
        self.writeback_execution_records
            .iter()
            .map(|record| (**record).clone())
            .collect()
    }

    pub(crate) fn writeback_mapper_envelopes(&self) -> Vec<BridgeWritebackMapperEnvelope> {
        self.writeback_mapper_envelopes
            .iter()
            .map(|record| (**record).clone())
            .collect()
    }

    pub(crate) fn writeback_mapped_family_inputs(&self) -> Vec<BridgeMappedWritebackFamilyInput> {
        self.writeback_mapped_family_inputs
            .iter()
            .map(|record| (**record).clone())
            .collect()
    }

    pub(crate) fn writeback_mapper_records(&self) -> Vec<BridgeWritebackMapperRecord> {
        self.writeback_mapper_records
            .iter()
            .map(|record| (**record).clone())
            .collect()
    }

    pub(crate) fn writeback_replay_records(&self) -> Vec<BridgeWritebackReplayRecord> {
        self.writeback_replay_records
            .iter()
            .map(|record| (**record).clone())
            .collect()
    }

    pub(crate) fn last_writeback_admission_record(
        &self,
    ) -> Option<BridgeWritebackFamilyAdmissionRecord> {
        self.writeback_admission_records
            .back()
            .map(|record| (**record).clone())
    }

    pub(crate) fn last_writeback_execution_record(&self) -> Option<BridgeWritebackExecutionRecord> {
        self.writeback_execution_records
            .back()
            .map(|record| (**record).clone())
    }

    pub(crate) fn last_writeback_mapper_envelope(&self) -> Option<BridgeWritebackMapperEnvelope> {
        self.writeback_mapper_envelopes
            .back()
            .map(|record| (**record).clone())
    }

    pub(crate) fn last_writeback_mapped_family_input(
        &self,
    ) -> Option<BridgeMappedWritebackFamilyInput> {
        self.writeback_mapped_family_inputs
            .back()
            .map(|record| (**record).clone())
    }

    pub(crate) fn last_writeback_mapper_record(&self) -> Option<BridgeWritebackMapperRecord> {
        self.writeback_mapper_records
            .back()
            .map(|record| (**record).clone())
    }

    pub(crate) fn last_writeback_replay_record(&self) -> Option<BridgeWritebackReplayRecord> {
        self.writeback_replay_records
            .back()
            .map(|record| (**record).clone())
    }

    pub(crate) fn writeback_admission_record_for_identity(
        &self,
        record_identity: &str,
    ) -> Option<BridgeWritebackFamilyAdmissionRecord> {
        self.latest_writeback_admission_by_record_identity
            .get(record_identity)
            .cloned()
            .map(|record| (*record).clone())
    }

    pub(crate) fn writeback_admission_record_for_contract_digest(
        &self,
        contract_digest: &str,
    ) -> Option<BridgeWritebackFamilyAdmissionRecord> {
        self.latest_writeback_admission_by_contract_digest
            .get(contract_digest)
            .cloned()
            .map(|record| (*record).clone())
    }

    pub(crate) fn writeback_execution_record_for_identity(
        &self,
        record_identity: &str,
    ) -> Option<BridgeWritebackExecutionRecord> {
        self.latest_writeback_execution_by_record_identity
            .get(record_identity)
            .cloned()
            .map(|record| (*record).clone())
    }

    pub(crate) fn writeback_execution_record_for_candidate_digest(
        &self,
        candidate_digest: &str,
    ) -> Option<BridgeWritebackExecutionRecord> {
        self.latest_writeback_execution_by_candidate_digest
            .get(candidate_digest)
            .cloned()
            .map(|record| (*record).clone())
    }

    pub(crate) fn writeback_mapper_envelope_for_identity(
        &self,
        envelope_identity: &str,
    ) -> Option<BridgeWritebackMapperEnvelope> {
        self.latest_writeback_mapper_envelope_by_identity
            .get(envelope_identity)
            .cloned()
            .map(|record| (*record).clone())
    }

    pub(crate) fn writeback_mapper_envelope_for_digest(
        &self,
        envelope_digest: &str,
    ) -> Option<BridgeWritebackMapperEnvelope> {
        self.latest_writeback_mapper_envelope_by_digest
            .get(envelope_digest)
            .cloned()
            .map(|record| (*record).clone())
    }

    pub(crate) fn writeback_mapped_family_input_for_identity(
        &self,
        mapped_input_identity: &str,
    ) -> Option<BridgeMappedWritebackFamilyInput> {
        self.latest_writeback_mapped_input_by_identity
            .get(mapped_input_identity)
            .cloned()
            .map(|record| (*record).clone())
    }

    pub(crate) fn writeback_mapped_family_input_for_digest(
        &self,
        mapped_input_digest: &str,
    ) -> Option<BridgeMappedWritebackFamilyInput> {
        self.latest_writeback_mapped_input_by_digest
            .get(mapped_input_digest)
            .cloned()
            .map(|record| (*record).clone())
    }

    pub(crate) fn writeback_mapper_record_for_identity(
        &self,
        record_identity: &str,
    ) -> Option<BridgeWritebackMapperRecord> {
        self.latest_writeback_mapper_by_record_identity
            .get(record_identity)
            .cloned()
            .map(|record| (*record).clone())
    }

    pub(crate) fn writeback_mapper_record_for_candidate_digest(
        &self,
        candidate_digest: &str,
    ) -> Option<BridgeWritebackMapperRecord> {
        self.latest_writeback_mapper_by_candidate_digest
            .get(candidate_digest)
            .cloned()
            .map(|record| (*record).clone())
    }

    pub(crate) fn writeback_replay_record_for_identity(
        &self,
        record_identity: &str,
    ) -> Option<BridgeWritebackReplayRecord> {
        self.latest_writeback_replay_by_record_identity
            .get(record_identity)
            .cloned()
            .map(|record| (*record).clone())
    }
}
