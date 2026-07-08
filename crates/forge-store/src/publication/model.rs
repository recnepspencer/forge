use crate::{
    media::{DurabilityBarrierClass, DurableBackendFamily},
    wal::DurableMutationId,
    ForegroundIsolationOutcome,
};
pub use forge_store_contracts::PublicationFamily;
use forge_relational::facade::{history::CommitId, replay::CanonicalCommitEnvelope};
use serde::Serialize;

const DEFAULT_RUNTIME_SESSION_ID: &str = "durable-runtime";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum PublicationStrategy {
    AppendOnly,
    ReplaceInPlace,
    RenamePublished,
    TransactionPublished,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum PublicationState {
    Unpublished,
    PartiallyDurable,
    BarrierCompleteButNotPublished,
    Published,
    PublicationGap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum PublicationClassification {
    RetainTrusted,
    FinishPublication,
    DiscardUnpublished,
    RequireRebuild,
    RequireQuarantine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct PublicationBarrierContract {
    strategy: PublicationStrategy,
    required_content_barrier: DurabilityBarrierClass,
    required_metadata_barrier: Option<DurabilityBarrierClass>,
    requires_directory_barrier: bool,
    acknowledgment_relevant: bool,
}

impl PublicationBarrierContract {
    pub(crate) fn new(
        strategy: PublicationStrategy,
        required_content_barrier: DurabilityBarrierClass,
        required_metadata_barrier: Option<DurabilityBarrierClass>,
        requires_directory_barrier: bool,
        acknowledgment_relevant: bool,
    ) -> Self {
        Self {
            strategy,
            required_content_barrier,
            required_metadata_barrier,
            requires_directory_barrier,
            acknowledgment_relevant,
        }
    }
    pub fn strategy(&self) -> PublicationStrategy {
        self.strategy
    }
    pub fn required_content_barrier(&self) -> DurabilityBarrierClass {
        self.required_content_barrier
    }
    pub fn required_metadata_barrier(&self) -> Option<DurabilityBarrierClass> {
        self.required_metadata_barrier
    }
    pub fn requires_directory_barrier(&self) -> bool {
        self.requires_directory_barrier
    }
    pub fn acknowledgment_relevant(&self) -> bool {
        self.acknowledgment_relevant
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ObservedPublicationFamilyState {
    family: PublicationFamily,
    state: PublicationState,
    contract: PublicationBarrierContract,
    observed_content_barrier: Option<DurabilityBarrierClass>,
    observed_metadata_barrier: Option<DurabilityBarrierClass>,
    source_admitted: bool,
}

impl ObservedPublicationFamilyState {
    pub(crate) fn new(
        family: PublicationFamily,
        state: PublicationState,
        contract: PublicationBarrierContract,
        observed_content_barrier: Option<DurabilityBarrierClass>,
        observed_metadata_barrier: Option<DurabilityBarrierClass>,
        source_admitted: bool,
    ) -> Self {
        Self {
            family,
            state,
            contract,
            observed_content_barrier,
            observed_metadata_barrier,
            source_admitted,
        }
    }
    pub fn family(&self) -> PublicationFamily {
        self.family
    }
    pub fn state(&self) -> PublicationState {
        self.state
    }
    pub fn contract(&self) -> PublicationBarrierContract {
        self.contract
    }
    pub fn observed_content_barrier(&self) -> Option<DurabilityBarrierClass> {
        self.observed_content_barrier
    }
    pub fn observed_metadata_barrier(&self) -> Option<DurabilityBarrierClass> {
        self.observed_metadata_barrier
    }
    pub fn source_admitted(&self) -> bool {
        self.source_admitted
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PublicationWriteOutcome {
    backend_family: DurableBackendFamily,
    classification: PublicationClassification,
    sufficient_for_published_truth: bool,
    acknowledgment_eligible: bool,
    family_states: Vec<ObservedPublicationFamilyState>,
    foreground_write_isolation: Option<ForegroundIsolationOutcome>,
}

impl PublicationWriteOutcome {
    pub(crate) fn new(
        backend_family: DurableBackendFamily,
        classification: PublicationClassification,
        sufficient_for_published_truth: bool,
        acknowledgment_eligible: bool,
        family_states: Vec<ObservedPublicationFamilyState>,
    ) -> Self {
        Self {
            backend_family,
            classification,
            sufficient_for_published_truth,
            acknowledgment_eligible,
            family_states,
            foreground_write_isolation: None,
        }
    }
    pub fn backend_family(&self) -> DurableBackendFamily {
        self.backend_family
    }
    pub fn classification(&self) -> PublicationClassification {
        self.classification
    }
    pub fn sufficient_for_published_truth(&self) -> bool {
        self.sufficient_for_published_truth
    }
    pub fn acknowledgment_eligible(&self) -> bool {
        self.acknowledgment_eligible
    }
    pub fn family_states(&self) -> &[ObservedPublicationFamilyState] {
        &self.family_states
    }
    pub fn foreground_write_isolation(&self) -> Option<&ForegroundIsolationOutcome> {
        self.foreground_write_isolation.as_ref()
    }
    pub(crate) fn with_foreground_write_isolation(
        mut self,
        foreground_write_isolation: ForegroundIsolationOutcome,
    ) -> Self {
        self.foreground_write_isolation = Some(foreground_write_isolation);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct DurableRecoveryPublicationObservation {
    pub(crate) durable_mutation_id: DurableMutationId,
    pub(crate) publication: PublicationWriteOutcome,
    pub(crate) canonical_envelope: Option<CanonicalCommitEnvelope>,
    pub(crate) commit_id: Option<CommitId>,
    pub(crate) intent_present: bool,
}

impl DurableRecoveryPublicationObservation {
    pub(crate) fn publication(&self) -> &PublicationWriteOutcome {
        &self.publication
    }
    pub(crate) fn canonical_envelope(&self) -> Option<&CanonicalCommitEnvelope> {
        self.canonical_envelope.as_ref()
    }
    pub(crate) fn commit_id(&self) -> Option<CommitId> {
        self.commit_id
    }
    pub(crate) fn intent_present(&self) -> bool {
        self.intent_present
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct DurablePublicationFacts {
    pub(crate) intent_present: bool,
    pub(crate) canonical_result_present: bool,
    pub(crate) authoritative_progress_present: bool,
    pub(crate) authoritative_commit_present: bool,
    pub(crate) branch_head_present: bool,
    pub(crate) acknowledgment_marker_present: bool,
}

#[derive(Debug)]
pub(crate) struct LocalAdmittedPublicationSource<T> {
    inner: T,
}

impl<T> LocalAdmittedPublicationSource<T> {
    pub(crate) fn new(inner: T) -> Self {
        Self { inner }
    }
    pub(crate) fn into_inner(self) -> T {
        self.inner
    }
    pub(crate) fn inner(&self) -> &T {
        &self.inner
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SimulatedCrashPoint {
    AfterIntentRecorded,
    AfterCanonicalResultRecorded,
    AfterAuthoritativeAppendPublished,
}

#[derive(Debug)]
pub(crate) struct DurablePublicationResult {
    pub(crate) durable_mutation_id: DurableMutationId,
    pub(crate) persisted: Option<crate::PersistedAuthoritativeCommit>,
}

impl DurablePublicationResult {
    pub(crate) fn durable_mutation_id(&self) -> DurableMutationId {
        self.durable_mutation_id
    }
    pub(crate) fn into_persisted(self) -> Option<crate::PersistedAuthoritativeCommit> {
        self.persisted
    }
}

pub(crate) fn default_runtime_session_id() -> &'static str {
    DEFAULT_RUNTIME_SESSION_ID
}
