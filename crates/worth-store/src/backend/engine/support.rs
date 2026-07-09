use crate::authority::{
    FetchedLineageSupportArtifact, FetchedSchemaSupportArtifact, HistoricalIdentityRequest,
    HistoricalIdentityResolution,
};
use crate::failure::{StoreError, StoreErrorKind};
use worth_relational::facade::history::CommitId;
use worth_relational::facade::identity::LineageId;
use worth_relational::facade::lineage::LineageEventRecord;

use super::{StateBackedStoreBackend, StatePersistence};
use crate::backend::records::{LineageSupportRecord, SchemaSupportRecord};

impl<P: StatePersistence> StateBackedStoreBackend<P> {
    pub fn fetch_schema_support(
        &self,
        commit_id: CommitId,
    ) -> Result<FetchedSchemaSupportArtifact, StoreError> {
        let record = self.fetch_verified_schema_support_record(commit_id)?;
        self.counters.record_schema_boundary_fetch(1, 1);
        Ok(FetchedSchemaSupportArtifact::new(record))
    }

    pub fn fetch_lineage_support(
        &self,
        commit_id: CommitId,
    ) -> Result<FetchedLineageSupportArtifact, StoreError> {
        let record = self.fetch_verified_lineage_support_record(commit_id)?;
        self.record_lineage_lookup(&record);
        Ok(FetchedLineageSupportArtifact::new(record))
    }

    pub fn fetch_lineage_history(
        &self,
        request: HistoricalIdentityRequest,
    ) -> Result<HistoricalIdentityResolution, StoreError> {
        let record = self.fetch_verified_lineage_support_record(request.commit_id())?;
        self.record_lineage_lookup(&record);
        if record.branch_id != *request.branch_id() {
            return Err(StoreError::new(
                StoreErrorKind::HistoricalIdentityResolutionGap,
                format!(
                    "historical identity request for commit {} expected branch `{}` but durable lineage support belongs to `{}`",
                    request.commit_id().0,
                    request.branch_id().0,
                    record.branch_id.0
                ),
            ));
        }
        let matching_events = record
            .lineage_events
            .iter()
            .filter(|event| lineage_event_touches(event, request.lineage_id()))
            .cloned()
            .collect::<Vec<_>>();
        if matching_events.is_empty() {
            return Err(StoreError::new(
                StoreErrorKind::HistoricalIdentityResolutionGap,
                format!(
                    "historical identity request for lineage {} found no durable lineage neighborhood in commit {} on branch `{}`",
                    request.lineage_id().0,
                    request.commit_id().0,
                    request.branch_id().0
                ),
            ));
        }
        let mut resolved_lineage_ids = matching_events
            .iter()
            .flat_map(|event: &LineageEventRecord| {
                event
                    .sources()
                    .iter()
                    .chain(event.targets().iter())
                    .copied()
            })
            .collect::<Vec<_>>();
        resolved_lineage_ids.sort_unstable();
        resolved_lineage_ids.dedup();
        Ok(HistoricalIdentityResolution::new(
            request.commit_id(),
            request.branch_id().clone(),
            request.lineage_id(),
            record.artifact_id,
            resolved_lineage_ids,
            matching_events,
            record.lineage_digest_basis,
            record.event_batch_digest_basis,
            record.decision_log_digest_basis,
        ))
    }

    fn fetch_verified_schema_support_record(
        &self,
        commit_id: CommitId,
    ) -> Result<SchemaSupportRecord, StoreError> {
        let artifact_id = super::super::integrity::schema_support_artifact_id(commit_id);
        let record = self
            .state
            .schema_support_records
            .get(&artifact_id)
            .cloned()
            .ok_or_else(|| {
                self.counters.record_commit_support_publication_gap();
                StoreError::new(
                    StoreErrorKind::SchemaBoundaryArtifactMissing,
                    format!(
                        "schema support artifact for commit {} not found",
                        commit_id.0
                    ),
                )
            })?;
        let verification = self.state.verify_schema_support_record(&record);
        if verification.is_err() {
            self.counters.record_commit_support_publication_gap();
        }
        verification?;
        Ok(record)
    }

    fn fetch_verified_lineage_support_record(
        &self,
        commit_id: CommitId,
    ) -> Result<LineageSupportRecord, StoreError> {
        let artifact_id = super::super::integrity::lineage_support_artifact_id(commit_id);
        let record = self
            .state
            .lineage_support_records
            .get(&artifact_id)
            .cloned()
            .ok_or_else(|| {
                self.counters.record_commit_support_publication_gap();
                StoreError::new(
                    StoreErrorKind::LineageArtifactMissing,
                    format!(
                        "lineage support artifact for commit {} not found",
                        commit_id.0
                    ),
                )
            })?;
        let verification = self.state.verify_lineage_support_record(&record);
        if verification.is_err() {
            self.counters.record_commit_support_publication_gap();
        }
        verification?;
        Ok(record)
    }

    fn record_lineage_lookup(&self, record: &LineageSupportRecord) {
        self.counters
            .record_lineage_lookup(1, record.lineage_events.len() as u64);
    }
}

fn lineage_event_touches(event: &LineageEventRecord, lineage_id: LineageId) -> bool {
    event.sources().contains(&lineage_id) || event.targets().contains(&lineage_id)
}
