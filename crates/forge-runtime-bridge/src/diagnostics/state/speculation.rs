use super::*;

impl BridgeDiagnosticsState {
    pub(crate) fn reserve_preview_session_identity(
        &mut self,
        session_identity: &BridgePreviewSessionIdentity,
    ) -> bool {
        self.reserved_preview_session_identities
            .insert(session_identity.clone())
    }

    pub(crate) fn record_preview_execution(
        &mut self,
        record: BridgePreviewExecutionRecord,
        limit: usize,
    ) {
        let record = Arc::new(record);
        self.latest_preview_execution_by_record_identity.insert(
            record.record_identity().as_str().to_string(),
            Arc::clone(&record),
        );
        self.latest_preview_execution_by_session_identity.insert(
            record.preview_session_identity().to_string(),
            Arc::clone(&record),
        );
        self.preview_execution_records.push_back(record);
        while self.preview_execution_records.len() > limit.max(1) {
            if let Some(evicted) = self.preview_execution_records.pop_front() {
                if self
                    .latest_preview_execution_by_record_identity
                    .get(evicted.record_identity().as_str())
                    .is_some_and(|current| Arc::ptr_eq(current, &evicted))
                {
                    self.latest_preview_execution_by_record_identity
                        .remove(evicted.record_identity().as_str());
                }
                if self
                    .latest_preview_execution_by_session_identity
                    .get(evicted.preview_session_identity())
                    .is_some_and(|current| Arc::ptr_eq(current, &evicted))
                {
                    self.latest_preview_execution_by_session_identity
                        .remove(evicted.preview_session_identity());
                }
            }
        }
    }

    pub(crate) fn record_preview_discard(
        &mut self,
        record: BridgePreviewDiscardRecord,
        limit: usize,
    ) {
        let record = Arc::new(record);
        self.latest_preview_discard_by_record_identity.insert(
            record.record_identity().as_str().to_string(),
            Arc::clone(&record),
        );
        self.latest_preview_discard_by_session_identity.insert(
            record.preview_session_identity().to_string(),
            Arc::clone(&record),
        );
        self.preview_discard_records.push_back(record);
        while self.preview_discard_records.len() > limit.max(1) {
            if let Some(evicted) = self.preview_discard_records.pop_front() {
                if self
                    .latest_preview_discard_by_record_identity
                    .get(evicted.record_identity().as_str())
                    .is_some_and(|current| Arc::ptr_eq(current, &evicted))
                {
                    self.latest_preview_discard_by_record_identity
                        .remove(evicted.record_identity().as_str());
                }
                if self
                    .latest_preview_discard_by_session_identity
                    .get(evicted.preview_session_identity())
                    .is_some_and(|current| Arc::ptr_eq(current, &evicted))
                {
                    self.latest_preview_discard_by_session_identity
                        .remove(evicted.preview_session_identity());
                }
            }
        }
    }

    pub(crate) fn record_preview_promotion(
        &mut self,
        record: BridgePreviewPromotionRecord,
        limit: usize,
    ) {
        let record = Arc::new(record);
        self.latest_preview_promotion_by_record_identity.insert(
            record.record_identity().as_str().to_string(),
            Arc::clone(&record),
        );
        self.latest_preview_promotion_by_session_identity.insert(
            record.preview_session_identity().to_string(),
            Arc::clone(&record),
        );
        self.preview_promotion_records.push_back(record);
        while self.preview_promotion_records.len() > limit.max(1) {
            if let Some(evicted) = self.preview_promotion_records.pop_front() {
                if self
                    .latest_preview_promotion_by_record_identity
                    .get(evicted.record_identity().as_str())
                    .is_some_and(|current| Arc::ptr_eq(current, &evicted))
                {
                    self.latest_preview_promotion_by_record_identity
                        .remove(evicted.record_identity().as_str());
                }
                if self
                    .latest_preview_promotion_by_session_identity
                    .get(evicted.preview_session_identity())
                    .is_some_and(|current| Arc::ptr_eq(current, &evicted))
                {
                    self.latest_preview_promotion_by_session_identity
                        .remove(evicted.preview_session_identity());
                }
            }
        }
    }

    pub(crate) fn preview_execution_records(&self) -> Vec<BridgePreviewExecutionRecord> {
        self.preview_execution_records
            .iter()
            .map(|record| (**record).clone())
            .collect()
    }

    pub(crate) fn preview_discard_records(&self) -> Vec<BridgePreviewDiscardRecord> {
        self.preview_discard_records
            .iter()
            .map(|record| (**record).clone())
            .collect()
    }

    pub(crate) fn preview_promotion_records(&self) -> Vec<BridgePreviewPromotionRecord> {
        self.preview_promotion_records
            .iter()
            .map(|record| (**record).clone())
            .collect()
    }

    pub(crate) fn last_preview_execution_record(&self) -> Option<BridgePreviewExecutionRecord> {
        self.preview_execution_records
            .back()
            .map(|record| (**record).clone())
    }

    pub(crate) fn last_preview_discard_record(&self) -> Option<BridgePreviewDiscardRecord> {
        self.preview_discard_records
            .back()
            .map(|record| (**record).clone())
    }

    pub(crate) fn last_preview_promotion_record(&self) -> Option<BridgePreviewPromotionRecord> {
        self.preview_promotion_records
            .back()
            .map(|record| (**record).clone())
    }

    pub(crate) fn preview_execution_record_for_identity(
        &self,
        record_identity: &PreviewExecutionRecordIdentity,
    ) -> Option<BridgePreviewExecutionRecord> {
        self.latest_preview_execution_by_record_identity
            .get(record_identity.as_str())
            .cloned()
            .map(|record| (*record).clone())
    }

    pub(crate) fn preview_execution_record_for_session_identity(
        &self,
        session_identity: &BridgePreviewSessionIdentity,
    ) -> Option<BridgePreviewExecutionRecord> {
        self.latest_preview_execution_by_session_identity
            .get(session_identity.as_str())
            .cloned()
            .map(|record| (*record).clone())
    }

    pub(crate) fn preview_discard_record_for_identity(
        &self,
        record_identity: &BridgePreviewDiscardRecordIdentity,
    ) -> Option<BridgePreviewDiscardRecord> {
        self.latest_preview_discard_by_record_identity
            .get(record_identity.as_str())
            .cloned()
            .map(|record| (*record).clone())
    }

    pub(crate) fn preview_discard_record_for_session_identity(
        &self,
        session_identity: &BridgePreviewSessionIdentity,
    ) -> Option<BridgePreviewDiscardRecord> {
        self.latest_preview_discard_by_session_identity
            .get(session_identity.as_str())
            .cloned()
            .map(|record| (*record).clone())
    }

    pub(crate) fn preview_promotion_record_for_identity(
        &self,
        record_identity: &BridgePreviewPromotionRecordIdentity,
    ) -> Option<BridgePreviewPromotionRecord> {
        self.latest_preview_promotion_by_record_identity
            .get(record_identity.as_str())
            .cloned()
            .map(|record| (*record).clone())
    }

    pub(crate) fn preview_promotion_record_for_session_identity(
        &self,
        session_identity: &BridgePreviewSessionIdentity,
    ) -> Option<BridgePreviewPromotionRecord> {
        self.latest_preview_promotion_by_session_identity
            .get(session_identity.as_str())
            .cloned()
            .map(|record| (*record).clone())
    }
}
