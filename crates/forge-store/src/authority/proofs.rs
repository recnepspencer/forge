use crate::authority::CanonicalDigest;
use crate::backend::records::{
    DurableCursorIdentityRecord, EmbeddedCheckpointRecord, LineageSupportRecord, SchemaSupportRecord,
    SubscriberCheckpointRecord,
};
use crate::evidence::CanonicalizationMetrics;
use forge_relational::facade::{
    history::{BranchHead, BranchId, CommitId},
    identity::LineageId,
    lineage::{
        LineageDecisionLogDigestBasis, LineageDigestBasis, LineageEventBatchDigestBasis,
        LineageEventRecord,
    },
    replay::CanonicalCommitEnvelope,
};

#[derive(Debug, Clone)]
pub struct RawRuntimeCommitEnvelope {
    envelope: CanonicalCommitEnvelope,
}

impl RawRuntimeCommitEnvelope {
    pub fn new(envelope: CanonicalCommitEnvelope) -> Self {
        Self { envelope }
    }

    pub fn envelope(&self) -> &CanonicalCommitEnvelope {
        &self.envelope
    }

    pub fn into_inner(self) -> CanonicalCommitEnvelope {
        self.envelope
    }
}

#[derive(Debug, Clone)]
pub struct CanonicalizedCommitEnvelope {
    envelope: CanonicalCommitEnvelope,
    digest: CanonicalDigest,
    canonicalization_version: u32,
    metrics: CanonicalizationMetrics,
}

impl CanonicalizedCommitEnvelope {
    pub(crate) fn new(
        envelope: CanonicalCommitEnvelope,
        digest: CanonicalDigest,
        canonicalization_version: u32,
        metrics: CanonicalizationMetrics,
    ) -> Self {
        Self {
            envelope,
            digest,
            canonicalization_version,
            metrics,
        }
    }

    pub fn envelope(&self) -> &CanonicalCommitEnvelope {
        &self.envelope
    }

    pub fn digest(&self) -> &CanonicalDigest {
        &self.digest
    }

    pub fn canonicalization_version(&self) -> u32 {
        self.canonicalization_version
    }

    pub fn metrics(&self) -> &CanonicalizationMetrics {
        &self.metrics
    }
}

#[derive(Debug, Clone)]
pub struct VerifiedAuthoritativeAppend {
    envelope: CanonicalCommitEnvelope,
    digest: CanonicalDigest,
    canonicalization_version: u32,
}

impl VerifiedAuthoritativeAppend {
    pub(crate) fn new(
        envelope: CanonicalCommitEnvelope,
        digest: CanonicalDigest,
        canonicalization_version: u32,
    ) -> Self {
        Self {
            envelope,
            digest,
            canonicalization_version,
        }
    }

    pub fn envelope(&self) -> &CanonicalCommitEnvelope {
        &self.envelope
    }

    pub fn digest(&self) -> &CanonicalDigest {
        &self.digest
    }

    pub fn canonicalization_version(&self) -> u32 {
        self.canonicalization_version
    }
}

#[derive(Debug, Clone)]
pub struct PersistedAuthoritativeCommit {
    envelope: CanonicalCommitEnvelope,
    digest: CanonicalDigest,
    canonicalization_version: u32,
    commit_sequence: u64,
}

impl PersistedAuthoritativeCommit {
    pub(crate) fn new(
        envelope: CanonicalCommitEnvelope,
        digest: CanonicalDigest,
        canonicalization_version: u32,
        commit_sequence: u64,
    ) -> Self {
        Self {
            envelope,
            digest,
            canonicalization_version,
            commit_sequence,
        }
    }

    pub fn envelope(&self) -> &CanonicalCommitEnvelope {
        &self.envelope
    }

    pub fn digest(&self) -> &CanonicalDigest {
        &self.digest
    }

    pub fn canonicalization_version(&self) -> u32 {
        self.canonicalization_version
    }

    pub fn commit_sequence(&self) -> u64 {
        self.commit_sequence
    }
}

#[derive(Debug, Clone)]
pub struct FetchedAuthoritativeCommit {
    envelope: CanonicalCommitEnvelope,
    digest: CanonicalDigest,
    canonicalization_version: u32,
    commit_sequence: u64,
}

impl FetchedAuthoritativeCommit {
    pub(crate) fn new(
        envelope: CanonicalCommitEnvelope,
        digest: CanonicalDigest,
        canonicalization_version: u32,
        commit_sequence: u64,
    ) -> Self {
        Self {
            envelope,
            digest,
            canonicalization_version,
            commit_sequence,
        }
    }

    pub fn envelope(&self) -> &CanonicalCommitEnvelope {
        &self.envelope
    }

    pub fn digest(&self) -> &CanonicalDigest {
        &self.digest
    }

    pub fn canonicalization_version(&self) -> u32 {
        self.canonicalization_version
    }

    pub fn commit_sequence(&self) -> u64 {
        self.commit_sequence
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthoritativeBranchHeadRecord {
    branch_id: BranchId,
    head: Option<forge_relational::facade::history::CommitReference>,
    head_update_sequence: u64,
}

impl AuthoritativeBranchHeadRecord {
    pub(crate) fn new(
        branch_id: BranchId,
        head: Option<forge_relational::facade::history::CommitReference>,
        head_update_sequence: u64,
    ) -> Self {
        Self {
            branch_id,
            head,
            head_update_sequence,
        }
    }

    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub fn head(&self) -> Option<&forge_relational::facade::history::CommitReference> {
        self.head.as_ref()
    }

    pub fn head_update_sequence(&self) -> u64 {
        self.head_update_sequence
    }

    pub fn branch_head(&self) -> BranchHead {
        BranchHead {
            branch_id: self.branch_id.clone(),
            head: self.head.clone(),
        }
    }

    pub fn head_commit_id(&self) -> Option<CommitId> {
        self.head.as_ref().map(|head| head.commit_id)
    }
}

#[derive(Debug, Clone)]
pub struct FetchedSchemaSupportArtifact {
    record: SchemaSupportRecord,
}

impl FetchedSchemaSupportArtifact {
    pub(crate) fn new(record: SchemaSupportRecord) -> Self {
        Self { record }
    }

    pub fn record(&self) -> &SchemaSupportRecord {
        &self.record
    }
}

pub type FetchedSchemaBoundaryArtifact = FetchedSchemaSupportArtifact;

#[derive(Debug, Clone)]
pub struct FetchedLineageSupportArtifact {
    record: LineageSupportRecord,
}

impl FetchedLineageSupportArtifact {
    pub(crate) fn new(record: LineageSupportRecord) -> Self {
        Self { record }
    }

    pub fn record(&self) -> &LineageSupportRecord {
        &self.record
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalIdentityRequest {
    commit_id: CommitId,
    branch_id: BranchId,
    lineage_id: LineageId,
}

impl HistoricalIdentityRequest {
    pub fn new(commit_id: CommitId, branch_id: BranchId, lineage_id: LineageId) -> Self {
        Self {
            commit_id,
            branch_id,
            lineage_id,
        }
    }

    pub fn commit_id(&self) -> CommitId {
        self.commit_id
    }

    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub fn lineage_id(&self) -> LineageId {
        self.lineage_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalIdentityResolution {
    commit_id: CommitId,
    branch_id: BranchId,
    lineage_id: LineageId,
    support_artifact_id: String,
    resolved_lineage_ids: Vec<LineageId>,
    matching_events: Vec<LineageEventRecord>,
    lineage_digest_basis: LineageDigestBasis,
    event_batch_digest_basis: LineageEventBatchDigestBasis,
    decision_log_digest_basis: LineageDecisionLogDigestBasis,
}

impl HistoricalIdentityResolution {
    pub(crate) fn new(
        commit_id: CommitId,
        branch_id: BranchId,
        lineage_id: LineageId,
        support_artifact_id: String,
        resolved_lineage_ids: Vec<LineageId>,
        matching_events: Vec<LineageEventRecord>,
        lineage_digest_basis: LineageDigestBasis,
        event_batch_digest_basis: LineageEventBatchDigestBasis,
        decision_log_digest_basis: LineageDecisionLogDigestBasis,
    ) -> Self {
        Self {
            commit_id,
            branch_id,
            lineage_id,
            support_artifact_id,
            resolved_lineage_ids,
            matching_events,
            lineage_digest_basis,
            event_batch_digest_basis,
            decision_log_digest_basis,
        }
    }

    pub fn commit_id(&self) -> CommitId {
        self.commit_id
    }

    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub fn lineage_id(&self) -> LineageId {
        self.lineage_id
    }

    pub fn support_artifact_id(&self) -> &str {
        &self.support_artifact_id
    }

    pub fn resolved_lineage_ids(&self) -> &[LineageId] {
        &self.resolved_lineage_ids
    }

    pub fn matching_events(&self) -> &[LineageEventRecord] {
        &self.matching_events
    }

    pub fn matching_event_ids(&self) -> Vec<u64> {
        self.matching_events
            .iter()
            .map(|event| event.event_id())
            .collect()
    }

    pub fn lineage_digest_basis(&self) -> &LineageDigestBasis {
        &self.lineage_digest_basis
    }

    pub fn event_batch_digest_basis(&self) -> &LineageEventBatchDigestBasis {
        &self.event_batch_digest_basis
    }

    pub fn decision_log_digest_basis(&self) -> &LineageDecisionLogDigestBasis {
        &self.decision_log_digest_basis
    }
}

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
}

impl PersistedSubscriberCheckpoint {
    pub(crate) fn new(record: SubscriberCheckpointRecord) -> Self {
        Self { record }
    }

    pub fn record(&self) -> &SubscriberCheckpointRecord {
        &self.record
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitCoupledSupportAppendWitness {
    commit_id: CommitId,
    branch_id: BranchId,
    emits_schema_support: bool,
    emits_lineage_support: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmbeddedCheckpointFetchRequest {
    checkpoint_id: String,
}

impl EmbeddedCheckpointFetchRequest {
    pub fn new(checkpoint_id: impl Into<String>) -> Self {
        Self {
            checkpoint_id: checkpoint_id.into(),
        }
    }

    pub fn checkpoint_id(&self) -> &str {
        &self.checkpoint_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistedEmbeddedCheckpoint {
    record: EmbeddedCheckpointRecord,
}

impl PersistedEmbeddedCheckpoint {
    pub(crate) fn new(record: EmbeddedCheckpointRecord) -> Self {
        Self { record }
    }

    pub fn checkpoint_id(&self) -> &str {
        &self.record.checkpoint_id
    }

    pub fn source_runtime_id(&self) -> &str {
        &self.record.source_runtime_id
    }

    pub fn classification(&self) -> crate::EmbeddedCheckpointClassification {
        match self.record.classification {
            crate::backend::records::EmbeddedCheckpointClassification::DerivedDurable => {
                crate::EmbeddedCheckpointClassification::DerivedDurable
            }
            crate::backend::records::EmbeddedCheckpointClassification::Ephemeral => {
                crate::EmbeddedCheckpointClassification::Ephemeral
            }
        }
    }

    pub fn record(&self) -> &EmbeddedCheckpointRecord {
        &self.record
    }

    pub fn basis_branch_id(&self) -> Option<&BranchId> {
        self.record.basis_branch_id.as_ref()
    }

    pub fn basis_commit_id(&self) -> Option<CommitId> {
        self.record.basis_commit_id
    }

    pub fn contained_commit_ids(&self) -> &[CommitId] {
        &self.record.contained_commit_ids
    }
}

impl CommitCoupledSupportAppendWitness {
    pub(crate) fn new(
        commit_id: CommitId,
        branch_id: BranchId,
        emits_schema_support: bool,
        emits_lineage_support: bool,
    ) -> Self {
        Self {
            commit_id,
            branch_id,
            emits_schema_support,
            emits_lineage_support,
        }
    }

    pub fn commit_id(&self) -> CommitId {
        self.commit_id
    }

    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub fn emits_schema_support(&self) -> bool {
        self.emits_schema_support
    }

    pub fn emits_lineage_support(&self) -> bool {
        self.emits_lineage_support
    }
}
