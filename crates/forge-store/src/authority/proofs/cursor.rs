use crate::backend::records::{DurableCursorIdentityRecord, SubscriberCheckpointRecord};
use crate::ForegroundIsolationOutcome;
use forge_relational::facade::history::{BranchId, CommitId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurableCursorAcknowledgeRequest {
    cursor_id: String,
    subscriber_id: String,
    branch_id: BranchId,
    feed_shape_id: String,
    schema_interpretation_id: String,
    cursor_semantics_version: u32,
    basis_commit_id: CommitId,
    schema_support_artifact_id: Option<String>,
}
impl DurableCursorAcknowledgeRequest {
    pub fn new(
        cursor_id: impl Into<String>,
        subscriber_id: impl Into<String>,
        branch_id: BranchId,
        feed_shape_id: impl Into<String>,
        schema_interpretation_id: impl Into<String>,
        cursor_semantics_version: u32,
        basis_commit_id: CommitId,
    ) -> Self {
        Self {
            cursor_id: cursor_id.into(),
            subscriber_id: subscriber_id.into(),
            branch_id,
            feed_shape_id: feed_shape_id.into(),
            schema_interpretation_id: schema_interpretation_id.into(),
            cursor_semantics_version,
            basis_commit_id,
            schema_support_artifact_id: None,
        }
    }
    pub fn with_schema_support_artifact_id(
        mut self,
        schema_support_artifact_id: impl Into<String>,
    ) -> Self {
        self.schema_support_artifact_id = Some(schema_support_artifact_id.into());
        self
    }
    pub fn cursor_id(&self) -> &str {
        &self.cursor_id
    }
    pub fn subscriber_id(&self) -> &str {
        &self.subscriber_id
    }
    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }
    pub fn feed_shape_id(&self) -> &str {
        &self.feed_shape_id
    }
    pub fn schema_interpretation_id(&self) -> &str {
        &self.schema_interpretation_id
    }
    pub fn cursor_semantics_version(&self) -> u32 {
        self.cursor_semantics_version
    }
    pub fn basis_commit_id(&self) -> CommitId {
        self.basis_commit_id
    }
    pub fn schema_support_artifact_id(&self) -> Option<&str> {
        self.schema_support_artifact_id.as_deref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurableCursorResumeRequest {
    cursor_id: String,
    subscriber_id: String,
    branch_id: BranchId,
    feed_shape_id: String,
    schema_interpretation_id: String,
    cursor_semantics_version: u32,
}
impl DurableCursorResumeRequest {
    pub fn new(
        cursor_id: impl Into<String>,
        subscriber_id: impl Into<String>,
        branch_id: BranchId,
        feed_shape_id: impl Into<String>,
        schema_interpretation_id: impl Into<String>,
        cursor_semantics_version: u32,
    ) -> Self {
        Self {
            cursor_id: cursor_id.into(),
            subscriber_id: subscriber_id.into(),
            branch_id,
            feed_shape_id: feed_shape_id.into(),
            schema_interpretation_id: schema_interpretation_id.into(),
            cursor_semantics_version,
        }
    }
    pub fn cursor_id(&self) -> &str {
        &self.cursor_id
    }
    pub fn subscriber_id(&self) -> &str {
        &self.subscriber_id
    }
    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }
    pub fn feed_shape_id(&self) -> &str {
        &self.feed_shape_id
    }
    pub fn schema_interpretation_id(&self) -> &str {
        &self.schema_interpretation_id
    }
    pub fn cursor_semantics_version(&self) -> u32 {
        self.cursor_semantics_version
    }
}

#[derive(Debug, Clone)]
pub struct FetchedDurableCursorIdentity {
    record: DurableCursorIdentityRecord,
}
impl FetchedDurableCursorIdentity {
    pub(crate) fn new(record: DurableCursorIdentityRecord) -> Self {
        Self { record }
    }
    pub fn record(&self) -> &DurableCursorIdentityRecord {
        &self.record
    }
}

#[derive(Debug, Clone)]
pub struct PersistedSubscriberCheckpoint {
    record: SubscriberCheckpointRecord,
    foreground_isolation: ForegroundIsolationOutcome,
}
impl PersistedSubscriberCheckpoint {
    pub(crate) fn new(record: SubscriberCheckpointRecord) -> Self {
        Self {
            record,
            foreground_isolation: ForegroundIsolationOutcome::stayed_isolated(
                crate::ForegroundReservationClass::Write,
            ),
        }
    }
    pub fn record(&self) -> &SubscriberCheckpointRecord {
        &self.record
    }
    pub fn foreground_isolation(&self) -> &ForegroundIsolationOutcome {
        &self.foreground_isolation
    }
    pub(crate) fn with_foreground_isolation(
        mut self,
        foreground_isolation: ForegroundIsolationOutcome,
    ) -> Self {
        self.foreground_isolation = foreground_isolation;
        self
    }
}

#[derive(Debug, Clone)]
pub struct DurableCursorResumePlan {
    identity: DurableCursorIdentityRecord,
    latest_checkpoint: SubscriberCheckpointRecord,
}
impl DurableCursorResumePlan {
    pub(crate) fn new(
        identity: DurableCursorIdentityRecord,
        latest_checkpoint: SubscriberCheckpointRecord,
    ) -> Self {
        Self {
            identity,
            latest_checkpoint,
        }
    }
    pub fn identity(&self) -> &DurableCursorIdentityRecord {
        &self.identity
    }
    pub fn latest_checkpoint(&self) -> &SubscriberCheckpointRecord {
        &self.latest_checkpoint
    }
}

#[derive(Debug, Clone)]
pub struct ResumeAdmittedCursor {
    plan: DurableCursorResumePlan,
}
impl ResumeAdmittedCursor {
    pub(crate) fn new(plan: DurableCursorResumePlan) -> Self {
        Self { plan }
    }
    pub fn plan(&self) -> &DurableCursorResumePlan {
        &self.plan
    }
    pub fn identity(&self) -> &DurableCursorIdentityRecord {
        self.plan.identity()
    }
    pub fn latest_checkpoint(&self) -> &SubscriberCheckpointRecord {
        self.plan.latest_checkpoint()
    }
}

#[derive(Debug, Clone)]
pub struct AdvanceCursorWitness {
    request: DurableCursorAcknowledgeRequest,
}
impl AdvanceCursorWitness {
    pub(crate) fn new(request: DurableCursorAcknowledgeRequest) -> Self {
        Self { request }
    }
    pub fn request(&self) -> &DurableCursorAcknowledgeRequest {
        &self.request
    }
    pub fn cursor_id(&self) -> &str {
        self.request.cursor_id()
    }
    pub(crate) fn into_request(self) -> DurableCursorAcknowledgeRequest {
        self.request
    }
}
