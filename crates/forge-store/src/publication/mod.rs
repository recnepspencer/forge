use crate::{
    backend::records::{SnapshotBasisRecord, SnapshotImageRecord, StoreState},
    facade::ForgeStore,
    failure::{StoreError, StoreErrorKind},
    media::{DurabilityBarrierClass, DurableBackendFamily, DurableMediaReport},
    modes::HostedRuntimeOwnershipProof,
    wal::{DurableMutationId, DurablePublicationPhase},
};
use forge_relational::facade::{
    history::CommitId, replay::CanonicalCommitEnvelope, runtime::RelationalRuntime,
};
use serde::Serialize;

const DEFAULT_RUNTIME_SESSION_ID: &str = "durable-runtime";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum PublicationFamily {
    WalIntent,
    WalCanonicalResult,
    WalPublicationProgress,
    AuthoritativeCommitAppendUnit,
    BranchHeadPublication,
    AcknowledgmentEligibility,
    SnapshotBasis,
    SnapshotImage,
}

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
    fn new(
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
    fn new(
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
}

impl PublicationWriteOutcome {
    fn new(
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct DurableRecoveryPublicationObservation {
    durable_mutation_id: DurableMutationId,
    publication: PublicationWriteOutcome,
    canonical_envelope: Option<CanonicalCommitEnvelope>,
    commit_id: Option<CommitId>,
    intent_present: bool,
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
    fn new(inner: T) -> Self {
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
struct PublicationRequest<F> {
    runtime_session_id: String,
    operation_name: String,
    execute: F,
}

impl<F> PublicationRequest<F> {
    fn new(runtime_session_id: &str, operation_name: String, execute: F) -> Self {
        Self {
            runtime_session_id: runtime_session_id.to_string(),
            operation_name,
            execute,
        }
    }
}

#[derive(Debug)]
struct AdmittedDurableMutation {
    runtime_session_id: String,
    operation_name: String,
    durable_mutation_id: DurableMutationId,
}

impl AdmittedDurableMutation {
    fn admit<F>(
        store: &mut ForgeStore,
        request: PublicationRequest<F>,
    ) -> Result<(Self, F), StoreError> {
        let durable_mutation_id =
            store.admit_durable_mutation(&request.runtime_session_id, &request.operation_name)?;
        Ok((
            Self {
                runtime_session_id: request.runtime_session_id,
                operation_name: request.operation_name,
                durable_mutation_id,
            },
            request.execute,
        ))
    }
}

#[derive(Debug)]
struct CanonicalResultRecorded {
    runtime_session_id: String,
    durable_mutation_id: DurableMutationId,
    commit_id: CommitId,
    canonical_envelope: CanonicalCommitEnvelope,
}

impl CanonicalResultRecorded {
    fn record_from_hosted_runtime<F>(
        admitted: AdmittedDurableMutation,
        ownership: &mut HostedRuntimeOwnershipProof,
        execute: F,
        store: &mut ForgeStore,
    ) -> Result<Self, StoreError>
    where
        F: FnOnce(&mut RelationalRuntime) -> Result<CommitId, StoreError>,
    {
        let commit_id = execute(ownership.runtime_mut())?;
        let canonical_envelope = ownership
            .runtime()
            .replay()
            .canonical_commit_envelope(commit_id)
            .cloned()
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::HostedRuntimeMutationProducedNoCommit,
                    format!(
                        "durable mutation `{}` returned commit {} but the hosted runtime has no matching canonical envelope",
                        admitted.operation_name, commit_id.0
                    ),
                )
            })?;
        store.record_hosted_runtime_commit_result(
            &admitted.runtime_session_id,
            admitted.durable_mutation_id,
            canonical_envelope.clone(),
        )?;
        store.record_publication_phase(
            &admitted.runtime_session_id,
            admitted.durable_mutation_id,
            DurablePublicationPhase::CanonicalCommitProduced,
            Some(commit_id),
        )?;
        Ok(Self {
            runtime_session_id: admitted.runtime_session_id,
            durable_mutation_id: admitted.durable_mutation_id,
            commit_id,
            canonical_envelope,
        })
    }
}

#[derive(Debug)]
struct AuthoritativePublicationRecorded {
    runtime_session_id: String,
    durable_mutation_id: DurableMutationId,
    persisted: crate::PersistedAuthoritativeCommit,
}

impl AuthoritativePublicationRecorded {
    fn publish(
        canonical_result: CanonicalResultRecorded,
        store: &mut ForgeStore,
    ) -> Result<Self, StoreError> {
        let persisted = store.append_runtime_envelope(canonical_result.canonical_envelope)?;
        store.record_publication_phase(
            &canonical_result.runtime_session_id,
            canonical_result.durable_mutation_id,
            DurablePublicationPhase::AuthoritativeAppendPublished,
            Some(canonical_result.commit_id),
        )?;
        Ok(Self {
            runtime_session_id: canonical_result.runtime_session_id,
            durable_mutation_id: canonical_result.durable_mutation_id,
            persisted,
        })
    }
}

#[derive(Debug)]
pub(crate) struct DurablePublicationResult {
    durable_mutation_id: DurableMutationId,
    persisted: Option<crate::PersistedAuthoritativeCommit>,
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

fn observed_family_state(
    family: PublicationFamily,
    contract: PublicationBarrierContract,
    present: bool,
    media_report: Option<DurableMediaReport>,
    source_admitted: bool,
) -> ObservedPublicationFamilyState {
    let observed_content_barrier = media_report.map(|report| report.content_barrier());
    let observed_metadata_barrier = media_report.map(|report| report.metadata_barrier());
    let state = if !present {
        if family == PublicationFamily::AcknowledgmentEligibility {
            PublicationState::BarrierCompleteButNotPublished
        } else {
            PublicationState::Unpublished
        }
    } else if !source_admitted {
        PublicationState::PublicationGap
    } else if let Some(report) = media_report {
        let content_ok = report.content_barrier() >= contract.required_content_barrier();
        let metadata_ok = contract
            .required_metadata_barrier()
            .map(|required| report.metadata_barrier() >= required)
            .unwrap_or(true);
        if content_ok && metadata_ok {
            PublicationState::Published
        } else {
            PublicationState::PartiallyDurable
        }
    } else {
        PublicationState::Published
    };

    ObservedPublicationFamilyState::new(
        family,
        state,
        contract,
        observed_content_barrier,
        observed_metadata_barrier,
        source_admitted,
    )
}

fn minimum_durable_truth_barrier(backend_report: DurableMediaReport) -> DurabilityBarrierClass {
    backend_report.ack_required_barrier()
}

fn contract_for_family(
    family: PublicationFamily,
    backend_report: DurableMediaReport,
) -> PublicationBarrierContract {
    let minimum_durable_barrier = minimum_durable_truth_barrier(backend_report);
    match family {
        PublicationFamily::WalIntent
        | PublicationFamily::WalCanonicalResult
        | PublicationFamily::WalPublicationProgress
        | PublicationFamily::AcknowledgmentEligibility => PublicationBarrierContract::new(
            PublicationStrategy::AppendOnly,
            minimum_durable_barrier,
            None,
            false,
            matches!(
                family,
                PublicationFamily::WalIntent
                    | PublicationFamily::WalCanonicalResult
                    | PublicationFamily::WalPublicationProgress
                    | PublicationFamily::AcknowledgmentEligibility
            ),
        ),
        PublicationFamily::AuthoritativeCommitAppendUnit
        | PublicationFamily::BranchHeadPublication => PublicationBarrierContract::new(
            PublicationStrategy::TransactionPublished,
            minimum_durable_barrier,
            Some(backend_report.metadata_barrier()),
            false,
            true,
        ),
        PublicationFamily::SnapshotBasis | PublicationFamily::SnapshotImage => {
            PublicationBarrierContract::new(
                PublicationStrategy::TransactionPublished,
                backend_report.ack_required_barrier(),
                Some(backend_report.metadata_barrier()),
                false,
                false,
            )
        }
    }
}

pub(crate) fn admit_local_snapshot_basis_source(
    record: SnapshotBasisRecord,
) -> Result<LocalAdmittedPublicationSource<SnapshotBasisRecord>, StoreError> {
    if record.snapshot_family_version != crate::snapshot::SNAPSHOT_FAMILY_VERSION {
        return Err(StoreError::new(
            StoreErrorKind::SnapshotFamilyVersionUnsupported,
            format!(
                "snapshot {} uses unsupported family version {}",
                record.snapshot_id.0, record.snapshot_family_version
            ),
        ));
    }
    if record.snapshot_basis_version != crate::snapshot::SNAPSHOT_BASIS_VERSION {
        return Err(StoreError::new(
            StoreErrorKind::SnapshotFamilyVersionUnsupported,
            format!(
                "snapshot {} uses unsupported basis version {}",
                record.snapshot_id.0, record.snapshot_basis_version
            ),
        ));
    }
    if record.snapshot_image_format_version != crate::snapshot::SNAPSHOT_IMAGE_FORMAT_VERSION {
        return Err(StoreError::new(
            StoreErrorKind::SnapshotFamilyVersionUnsupported,
            format!(
                "snapshot {} uses unsupported image format version {}",
                record.snapshot_id.0, record.snapshot_image_format_version
            ),
        ));
    }
    Ok(LocalAdmittedPublicationSource::new(record))
}

pub(crate) fn admit_local_snapshot_image_source(
    record: SnapshotImageRecord,
) -> Result<LocalAdmittedPublicationSource<SnapshotImageRecord>, StoreError> {
    if record.image.snapshot_family_version() != crate::snapshot::SNAPSHOT_FAMILY_VERSION {
        return Err(StoreError::new(
            StoreErrorKind::SnapshotFamilyVersionUnsupported,
            format!(
                "snapshot {} image uses unsupported family version {}",
                record.snapshot_id.0,
                record.image.snapshot_family_version()
            ),
        ));
    }
    if record.image.snapshot_basis_version() != crate::snapshot::SNAPSHOT_BASIS_VERSION {
        return Err(StoreError::new(
            StoreErrorKind::SnapshotFamilyVersionUnsupported,
            format!(
                "snapshot {} image uses unsupported basis version {}",
                record.snapshot_id.0,
                record.image.snapshot_basis_version()
            ),
        ));
    }
    if record.image.snapshot_image_format_version()
        != crate::snapshot::SNAPSHOT_IMAGE_FORMAT_VERSION
    {
        return Err(StoreError::new(
            StoreErrorKind::SnapshotFamilyVersionUnsupported,
            format!(
                "snapshot {} image uses unsupported image format version {}",
                record.snapshot_id.0,
                record.image.snapshot_image_format_version()
            ),
        ));
    }
    Ok(LocalAdmittedPublicationSource::new(record))
}

pub(crate) fn classify_durable_publication(
    backend_report: DurableMediaReport,
    facts: DurablePublicationFacts,
) -> PublicationWriteOutcome {
    let families = vec![
        observed_family_state(
            PublicationFamily::WalIntent,
            contract_for_family(PublicationFamily::WalIntent, backend_report),
            facts.intent_present,
            Some(backend_report),
            facts.intent_present,
        ),
        observed_family_state(
            PublicationFamily::WalCanonicalResult,
            contract_for_family(PublicationFamily::WalCanonicalResult, backend_report),
            facts.canonical_result_present,
            Some(backend_report),
            facts.canonical_result_present,
        ),
        observed_family_state(
            PublicationFamily::WalPublicationProgress,
            contract_for_family(PublicationFamily::WalPublicationProgress, backend_report),
            facts.authoritative_progress_present,
            Some(backend_report),
            facts.authoritative_progress_present,
        ),
        observed_family_state(
            PublicationFamily::AuthoritativeCommitAppendUnit,
            contract_for_family(
                PublicationFamily::AuthoritativeCommitAppendUnit,
                backend_report,
            ),
            facts.authoritative_commit_present,
            Some(backend_report),
            facts.authoritative_commit_present,
        ),
        observed_family_state(
            PublicationFamily::BranchHeadPublication,
            contract_for_family(PublicationFamily::BranchHeadPublication, backend_report),
            facts.branch_head_present,
            Some(backend_report),
            facts.branch_head_present,
        ),
        observed_family_state(
            PublicationFamily::AcknowledgmentEligibility,
            contract_for_family(PublicationFamily::AcknowledgmentEligibility, backend_report),
            facts.acknowledgment_marker_present,
            Some(backend_report),
            facts.acknowledgment_marker_present,
        ),
    ];
    let non_ack_states = families
        .iter()
        .filter(|state| state.family() != PublicationFamily::AcknowledgmentEligibility)
        .collect::<Vec<_>>();
    let prerequisites_published = non_ack_states
        .iter()
        .all(|state| state.state() == PublicationState::Published);
    let has_gap = families
        .iter()
        .any(|state| state.state() == PublicationState::PublicationGap);
    let has_partial = families
        .iter()
        .any(|state| state.state() == PublicationState::PartiallyDurable);
    let classification = classify_durable_publication_classification(
        facts,
        prerequisites_published,
        has_gap,
        has_partial,
    );

    PublicationWriteOutcome::new(
        backend_report.backend_family(),
        classification,
        classification == PublicationClassification::RetainTrusted,
        prerequisites_published && !has_gap && !has_partial,
        families,
    )
}

fn classify_durable_publication_classification(
    facts: DurablePublicationFacts,
    prerequisites_published: bool,
    has_gap: bool,
    has_partial: bool,
) -> PublicationClassification {
    let has_authority = facts.authoritative_commit_present || facts.branch_head_present;
    if has_gap {
        PublicationClassification::RequireRebuild
    } else if has_partial {
        PublicationClassification::RequireQuarantine
    } else if prerequisites_published && facts.acknowledgment_marker_present {
        PublicationClassification::RetainTrusted
    } else if prerequisites_published {
        PublicationClassification::FinishPublication
    } else if has_authority || facts.canonical_result_present {
        PublicationClassification::FinishPublication
    } else {
        PublicationClassification::DiscardUnpublished
    }
}

pub(crate) fn classify_snapshot_publication(
    backend_report: DurableMediaReport,
    basis: Option<SnapshotBasisRecord>,
    image: Option<SnapshotImageRecord>,
) -> Result<PublicationWriteOutcome, StoreError> {
    let admitted_basis = basis.map(admit_local_snapshot_basis_source).transpose()?;
    let admitted_image = image.map(admit_local_snapshot_image_source).transpose()?;
    let basis_present = admitted_basis.is_some();
    let image_present = admitted_image.is_some();

    let basis_state = observed_family_state(
        PublicationFamily::SnapshotBasis,
        contract_for_family(PublicationFamily::SnapshotBasis, backend_report),
        basis_present,
        Some(backend_report),
        basis_present,
    );
    let image_state = observed_family_state(
        PublicationFamily::SnapshotImage,
        contract_for_family(PublicationFamily::SnapshotImage, backend_report),
        image_present,
        Some(backend_report),
        image_present,
    );

    let classification = match (basis_present, image_present) {
        (true, true) => PublicationClassification::RetainTrusted,
        (true, false) => PublicationClassification::RequireRebuild,
        (false, true) => PublicationClassification::RequireQuarantine,
        (false, false) => PublicationClassification::DiscardUnpublished,
    };
    Ok(PublicationWriteOutcome::new(
        backend_report.backend_family(),
        classification,
        classification == PublicationClassification::RetainTrusted,
        false,
        vec![basis_state, image_state],
    ))
}

pub(crate) fn durable_publication_facts(
    state: &StoreState,
    durable_mutation_id: DurableMutationId,
    expected_commit_id: Option<CommitId>,
) -> Result<DurablePublicationFacts, StoreError> {
    let mut intent_present = false;
    let mut canonical_result_present = false;
    let mut authoritative_progress_present = false;
    let mut acknowledgment_marker_present = false;
    let mut published_commit_id = expected_commit_id;

    for record in state.wal_records_for_mutation(durable_mutation_id) {
        let admitted = admit_local_wal_record(record)?;
        match &admitted.inner().payload {
            crate::wal::WalRecordPayload::DurableMutationIntent(_) => {
                intent_present = true;
            }
            crate::wal::WalRecordPayload::HostedRuntimeCommitResult(result) => {
                canonical_result_present = true;
                published_commit_id =
                    published_commit_id.or(Some(result.canonical_envelope.commit.commit_id));
            }
            crate::wal::WalRecordPayload::BulkCheckpointPublicationIntent(_) => {}
            crate::wal::WalRecordPayload::DurablePublicationProgress(progress) => {
                if progress.phase == DurablePublicationPhase::AuthoritativeAppendPublished {
                    authoritative_progress_present = true;
                }
                if progress.phase == DurablePublicationPhase::AcknowledgmentEligible {
                    acknowledgment_marker_present = true;
                }
                published_commit_id = published_commit_id.or(progress.commit_id);
            }
            crate::wal::WalRecordPayload::RecoveryDecision(_) => {}
        }
    }

    let authoritative_commit_present = published_commit_id
        .map(|commit_id| state.has_commit(commit_id))
        .unwrap_or(false);
    let branch_head_present = published_commit_id
        .map(|commit_id| {
            state
                .branch_head_records
                .values()
                .any(|record| record.head_commit_id == Some(commit_id))
        })
        .unwrap_or(false);

    Ok(DurablePublicationFacts {
        intent_present,
        canonical_result_present,
        authoritative_progress_present,
        authoritative_commit_present,
        branch_head_present,
        acknowledgment_marker_present,
    })
}

pub(crate) fn observe_durable_recovery_publication<'a>(
    state: &StoreState,
    durable_mutation_id: DurableMutationId,
    wal_records: &[&'a crate::wal::WalRecord],
    backend_report: DurableMediaReport,
) -> Result<DurableRecoveryPublicationObservation, StoreError> {
    let mut canonical_envelope: Option<CanonicalCommitEnvelope> = None;
    let mut commit_id = None;
    let mut intent_present = false;

    for record in wal_records {
        let admitted = admit_local_wal_record(record)?;
        match &admitted.inner().payload {
            crate::wal::WalRecordPayload::DurableMutationIntent(_) => {
                intent_present = true;
            }
            crate::wal::WalRecordPayload::HostedRuntimeCommitResult(result) => {
                if let Some(existing) = &canonical_envelope {
                    if existing != &result.canonical_envelope {
                        return Err(StoreError::new(
                            StoreErrorKind::RecoverySourceConflict,
                            format!(
                                "durable mutation {} has conflicting hosted runtime canonical results",
                                durable_mutation_id.0
                            ),
                        ));
                    }
                } else {
                    canonical_envelope = Some(result.canonical_envelope.clone());
                }
                commit_id = Some(result.canonical_envelope.commit.commit_id);
            }
            crate::wal::WalRecordPayload::BulkCheckpointPublicationIntent(_) => {}
            crate::wal::WalRecordPayload::DurablePublicationProgress(progress) => {
                if let Some(progress_commit_id) = progress.commit_id {
                    if let Some(existing) = commit_id {
                        if existing != progress_commit_id {
                            return Err(StoreError::new(
                                StoreErrorKind::RecoverySourceConflict,
                                format!(
                                    "durable mutation {} has conflicting publication commit ids",
                                    durable_mutation_id.0
                                ),
                            ));
                        }
                    } else {
                        commit_id = Some(progress_commit_id);
                    }
                }
            }
            crate::wal::WalRecordPayload::RecoveryDecision(_) => {}
        }
    }

    let facts = durable_publication_facts(state, durable_mutation_id, commit_id)?;
    let publication = classify_durable_publication(backend_report, facts);
    Ok(DurableRecoveryPublicationObservation {
        durable_mutation_id,
        publication,
        canonical_envelope,
        commit_id,
        intent_present,
    })
}

pub(crate) fn admit_local_wal_record<'a>(
    record: &'a crate::wal::WalRecord,
) -> Result<LocalAdmittedPublicationSource<&'a crate::wal::WalRecord>, StoreError> {
    record.validate_integrity()?;
    Ok(LocalAdmittedPublicationSource::new(record))
}

pub(crate) fn execute_durable_publication<F>(
    store: &mut ForgeStore,
    ownership: &mut HostedRuntimeOwnershipProof,
    runtime_session_id: &str,
    operation_name: String,
    execute: F,
    crash_point: Option<SimulatedCrashPoint>,
) -> Result<DurablePublicationResult, StoreError>
where
    F: FnOnce(&mut RelationalRuntime) -> Result<CommitId, StoreError>,
{
    let request = PublicationRequest::new(runtime_session_id, operation_name, execute);
    let (admitted, execute) = AdmittedDurableMutation::admit(store, request)?;
    if crash_point == Some(SimulatedCrashPoint::AfterIntentRecorded) {
        return Ok(DurablePublicationResult {
            durable_mutation_id: admitted.durable_mutation_id,
            persisted: None,
        });
    }

    let canonical_result =
        CanonicalResultRecorded::record_from_hosted_runtime(admitted, ownership, execute, store)?;
    if crash_point == Some(SimulatedCrashPoint::AfterCanonicalResultRecorded) {
        return Ok(DurablePublicationResult {
            durable_mutation_id: canonical_result.durable_mutation_id,
            persisted: None,
        });
    }

    let authoritative_publication =
        AuthoritativePublicationRecorded::publish(canonical_result, store)?;
    let prerequisite_outcome = store.classify_durable_publication(
        authoritative_publication.durable_mutation_id,
        Some(
            authoritative_publication
                .persisted
                .envelope()
                .commit
                .commit_id,
        ),
    )?;
    if !prerequisite_outcome.acknowledgment_eligible() {
        return Err(StoreError::new(
            StoreErrorKind::AcknowledgmentBoundaryViolation,
            format!(
                "durable mutation {} is not acknowledgment-eligible under publication classification {:?}",
                authoritative_publication.durable_mutation_id.0,
                prerequisite_outcome.classification()
            ),
        ));
    }
    if crash_point == Some(SimulatedCrashPoint::AfterAuthoritativeAppendPublished) {
        return Ok(DurablePublicationResult {
            durable_mutation_id: authoritative_publication.durable_mutation_id,
            persisted: None,
        });
    }

    store.record_publication_phase(
        &authoritative_publication.runtime_session_id,
        authoritative_publication.durable_mutation_id,
        DurablePublicationPhase::AcknowledgmentEligible,
        Some(
            authoritative_publication
                .persisted
                .envelope()
                .commit
                .commit_id,
        ),
    )?;
    store.record_durable_commit_acknowledged();
    let final_outcome = store.classify_durable_publication(
        authoritative_publication.durable_mutation_id,
        Some(
            authoritative_publication
                .persisted
                .envelope()
                .commit
                .commit_id,
        ),
    )?;
    if !final_outcome.sufficient_for_published_truth() {
        return Err(StoreError::new(
            StoreErrorKind::AcknowledgmentBoundaryViolation,
            format!(
                "durable mutation {} did not reach published truth after acknowledgment marker; classification {:?}",
                authoritative_publication.durable_mutation_id.0,
                final_outcome.classification()
            ),
        ));
    }
    Ok(DurablePublicationResult {
        durable_mutation_id: authoritative_publication.durable_mutation_id,
        persisted: Some(authoritative_publication.persisted),
    })
}
