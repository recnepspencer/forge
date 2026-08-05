use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::telemetry::{CheckpointTelemetry, RuntimeTelemetry};
use crate::data::temporal::{RuntimeClockBasis, TemporalWakeSummary};
use crate::diagnostics::replay::{ReplayEvent, ReplayEventKind};
use crate::diagnostics::ReplayCursor;
use crate::logic::checkpoint::CheckpointRuntime;
use crate::logic::planner::{ExecutionRecordId, SemanticSegmentId};
use crate::logic::transaction::runtime::config::SignalRuntimeConfig;
use crate::logic::transaction::TransactionReplayEntry;
use crate::state::{SignalBranchId, SignalSnapshotId};

use super::super::transaction::TemporalTransactionEvidence;
use super::merge::canonical_digest;
use super::resource::ResourceRuntimeState;
use super::temporal::TemporalRuntimeState;

#[derive(Debug, Clone)]
pub(in crate::logic::transaction::runtime) struct AuthorityState<T>
where
    T: Copy + Ord,
{
    pub graph: SignalGraph,
    pub config: SignalRuntimeConfig<T>,
}

impl<T> AuthorityState<T>
where
    T: Copy + Ord,
{
    pub fn capture(graph: &SignalGraph, config: &SignalRuntimeConfig<T>) -> Self {
        Self {
            graph: graph.clone_stateful(),
            config: config.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub(in crate::logic::transaction::runtime) struct DerivedState<D, I>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
{
    pub checkpoint: CheckpointRuntime<D, I>,
    pub resource: ResourceRuntimeState,
    pub temporal: TemporalRuntimeState,
    pub telemetry: RuntimeTelemetry,
}

impl<D, I> DerivedState<D, I>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
{
    pub fn capture(
        checkpoint: &CheckpointRuntime<D, I>,
        resource: &ResourceRuntimeState,
        temporal: &TemporalRuntimeState,
        telemetry: &RuntimeTelemetry,
    ) -> Self {
        Self {
            checkpoint: checkpoint.clone(),
            resource: resource.clone(),
            temporal: temporal.clone(),
            telemetry: *telemetry,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CheckpointRecord {
    pub checkpoint_flushes: u64,
    pub checkpoint_flush_nanos: u128,
    pub rollback_count: u64,
    pub checkpoint_size: u64,
    pub journal_replay_span: u64,
}

impl CheckpointRecord {
    pub fn from_checkpoint_telemetry(telemetry: CheckpointTelemetry) -> Self {
        Self {
            checkpoint_flushes: telemetry.checkpoint_flushes,
            checkpoint_flush_nanos: telemetry.checkpoint_flush_nanos,
            rollback_count: telemetry.rollback_count,
            checkpoint_size: telemetry.checkpoint_size,
            journal_replay_span: telemetry.journal_replay_span,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct JournalSegment {
    pub replay_event_count: u32,
    pub first_execution_record_id: Option<ExecutionRecordId>,
    pub last_execution_record_id: Option<ExecutionRecordId>,
    pub first_semantic_segment_id: Option<SemanticSegmentId>,
    pub last_semantic_segment_id: Option<SemanticSegmentId>,
    pub contains_rollback: bool,
    pub contains_failure: bool,
}

impl JournalSegment {
    pub fn from_entries(entries: &[TransactionReplayEntry]) -> Self {
        let mut segment = Self {
            replay_event_count: entries.len() as u32,
            ..Self::default()
        };
        for entry in entries {
            segment.first_execution_record_id = segment
                .first_execution_record_id
                .or(entry.execution_record_id);
            segment.last_execution_record_id = entry
                .execution_record_id
                .or(segment.last_execution_record_id);
            segment.first_semantic_segment_id = segment
                .first_semantic_segment_id
                .or(entry.semantic_segment_id);
            segment.last_semantic_segment_id = entry
                .semantic_segment_id
                .or(segment.last_semantic_segment_id);
            segment.contains_rollback |=
                matches!(entry.kind, ReplayEventKind::TransactionRolledBack);
            segment.contains_failure |= matches!(entry.kind, ReplayEventKind::FailureRecorded);
        }
        segment
    }

    pub fn from_replay_events(entries: &[ReplayEvent]) -> Self {
        let mut segment = Self {
            replay_event_count: entries.len() as u32,
            ..Self::default()
        };
        for entry in entries {
            let execution_record_id = entry
                .execution_record_id
                .map(crate::logic::planner::ExecutionRecordId);
            let semantic_segment_id = entry
                .semantic_segment_id
                .map(crate::logic::planner::SemanticSegmentId);
            segment.first_execution_record_id =
                segment.first_execution_record_id.or(execution_record_id);
            segment.last_execution_record_id =
                execution_record_id.or(segment.last_execution_record_id);
            segment.first_semantic_segment_id =
                segment.first_semantic_segment_id.or(semantic_segment_id);
            segment.last_semantic_segment_id =
                semantic_segment_id.or(segment.last_semantic_segment_id);
            segment.contains_rollback |=
                matches!(entry.kind, ReplayEventKind::TransactionRolledBack);
            segment.contains_failure |= matches!(entry.kind, ReplayEventKind::FailureRecorded);
        }
        segment
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckpointBoundary {
    pub authority_branch_id: SignalBranchId,
    pub authority_snapshot_id: Option<SignalSnapshotId>,
    pub replay_head: Option<ReplayCursor>,
    pub checkpoint: CheckpointRecord,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundedJournalSegment {
    pub replay_head: Option<ReplayCursor>,
    pub replay_event_count: u32,
    pub first_execution_record_id: Option<ExecutionRecordId>,
    pub last_execution_record_id: Option<ExecutionRecordId>,
    pub first_semantic_segment_id: Option<SemanticSegmentId>,
    pub last_semantic_segment_id: Option<SemanticSegmentId>,
    pub contains_rollback: bool,
    pub contains_failure: bool,
}

impl BoundedJournalSegment {
    pub fn from_record(replay_head: Option<ReplayCursor>, segment: &JournalSegment) -> Self {
        Self {
            replay_head,
            replay_event_count: segment.replay_event_count,
            first_execution_record_id: segment.first_execution_record_id,
            last_execution_record_id: segment.last_execution_record_id,
            first_semantic_segment_id: segment.first_semantic_segment_id,
            last_semantic_segment_id: segment.last_semantic_segment_id,
            contains_rollback: segment.contains_rollback,
            contains_failure: segment.contains_failure,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyIndexRebuildProof {
    pub authority_branch_id: SignalBranchId,
    pub authority_snapshot_id: Option<SignalSnapshotId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplaySuffixRebuildProof {
    pub replay_head: Option<ReplayCursor>,
    pub replay_event_count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeSupportRebuildProof {
    pub authority_branch_id: SignalBranchId,
    pub replay_event_count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RequiredDerivedRebuildSet {
    DependencyIndexes(DependencyIndexRebuildProof),
    ReplaySuffix(ReplaySuffixRebuildProof),
    MergeSupport(MergeSupportRebuildProof),
    TemporalState(TemporalStateRebuildProof),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporalStateRebuildProof {
    pub authority_branch_id: SignalBranchId,
    pub authority_snapshot_id: Option<SignalSnapshotId>,
    pub scheduled_wake_count: u64,
    pub ready_wake_count: u64,
    pub retired_wake_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporalReconstructabilityArtifact {
    pub clock_basis: RuntimeClockBasis,
    pub wake_summary: TemporalWakeSummary,
    pub eligibility_fact_count: u64,
    pub scheduled_wake_count: u64,
    pub ready_wake_count: u64,
    pub retired_wake_count: u64,
    pub rescheduled_wake_count: u64,
    pub reused_wake_count: u64,
    pub interval_regeneration_count: u64,
    pub previous_value_reference_count: u64,
    pub clock_checkpoint_digest: String,
    pub scheduled_wake_digest: String,
    pub ready_wake_digest: String,
    pub retired_wake_digest: String,
    pub rescheduled_wake_digest: String,
    pub reused_wake_digest: String,
    pub interval_regeneration_digest: String,
    pub temporal_eligibility_digest: String,
    pub previous_value_reference_digest: String,
    pub certification_digest: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TemporalReplayMismatchClass {
    ClockCheckpointDigestMismatch,
    ScheduledWakeDigestMismatch,
    ReadyWakeDigestMismatch,
    RetiredWakeDigestMismatch,
    RescheduledWakeDigestMismatch,
    ReusedWakeDigestMismatch,
    IntervalRegenerationDigestMismatch,
    TemporalEligibilityDigestMismatch,
    PreviousValueReferenceDigestMismatch,
    CertificationDigestMismatch,
    WakeSummaryMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemporalReplayParityReport {
    pub proof_schema_version: String,
    pub expected: TemporalReconstructabilityArtifact,
    pub replayed: TemporalReconstructabilityArtifact,
    pub parity: bool,
    pub mismatch_classes: Vec<TemporalReplayMismatchClass>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TemporalCertificationFamily {
    TemporalEligibilityReplayParity,
    TemporalBranchRestoreEquivalence,
    TemporalWakeBoundedness,
    PreviousValueTimeGatedEquivalence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemporalCertificationRecord {
    pub family: TemporalCertificationFamily,
    pub artifact: TemporalReconstructabilityArtifact,
    pub parity: Option<TemporalReplayParityReport>,
    pub passed: bool,
}

#[derive(Debug, Clone, Default)]
pub struct TemporalCertificationBuilder {
    temporal_eligibility_replay_parity: Option<TemporalCertificationRecord>,
    temporal_branch_restore_equivalence: Option<TemporalCertificationRecord>,
    temporal_wake_boundedness: Option<TemporalCertificationRecord>,
    previous_value_time_gated_equivalence: Option<TemporalCertificationRecord>,
}

pub const REQUIRED_TEMPORAL_CERTIFICATION_FAMILIES: [TemporalCertificationFamily; 4] = [
    TemporalCertificationFamily::TemporalEligibilityReplayParity,
    TemporalCertificationFamily::TemporalBranchRestoreEquivalence,
    TemporalCertificationFamily::TemporalWakeBoundedness,
    TemporalCertificationFamily::PreviousValueTimeGatedEquivalence,
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TemporalCertificationFailure {
    MissingRequiredFamily {
        family: TemporalCertificationFamily,
    },
    DuplicateFamily {
        family: TemporalCertificationFamily,
        count: u32,
    },
    FailedFamily {
        family: TemporalCertificationFamily,
    },
    ParityMismatch {
        family: TemporalCertificationFamily,
        mismatch_classes: Vec<TemporalReplayMismatchClass>,
    },
    EmptyCertificationDigest {
        family: TemporalCertificationFamily,
    },
}

impl TemporalCertificationFailure {
    pub fn family(&self) -> TemporalCertificationFamily {
        match self {
            Self::MissingRequiredFamily { family }
            | Self::DuplicateFamily { family, .. }
            | Self::FailedFamily { family }
            | Self::ParityMismatch { family, .. }
            | Self::EmptyCertificationDigest { family } => *family,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemporalCertificationSummary {
    pub required_family_count: u32,
    pub provided_record_count: u32,
    pub passed_family_count: u32,
    pub failed_family_count: u32,
    pub missing_family_count: u32,
    pub duplicate_family_count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemporalCertificationBundle {
    pub schema_version: String,
    pub records: Vec<TemporalCertificationRecord>,
    pub summary: TemporalCertificationSummary,
    pub bundle_digest: String,
    pub passed: bool,
    pub failures: Vec<TemporalCertificationFailure>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TemporalCertificationBundleMismatchClass {
    SchemaVersionMismatch,
    BundleDigestMismatch,
    PassStatusMismatch,
    SummaryMismatch,
    FailureSetMismatch,
    RecordSetMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemporalCertificationBundleParityReport {
    pub proof_schema_version: String,
    pub expected: TemporalCertificationBundle,
    pub replayed: TemporalCertificationBundle,
    pub parity: bool,
    pub mismatch_classes: Vec<TemporalCertificationBundleMismatchClass>,
}

pub const TEMPORAL_REPLAY_PARITY_SCHEMA_VERSION: &str = "worth-signal-temporal-replay-parity-v1";
pub const TEMPORAL_CERTIFICATION_BUNDLE_SCHEMA_VERSION: &str =
    "worth-signal-temporal-certification-bundle-v1";
pub const TEMPORAL_CERTIFICATION_BUNDLE_PARITY_SCHEMA_VERSION: &str =
    "worth-signal-temporal-certification-bundle-parity-v1";

impl Default for TemporalReconstructabilityArtifact {
    fn default() -> Self {
        Self::from_evidence(
            TemporalWakeSummary::default(),
            &TemporalTransactionEvidence::default(),
        )
    }
}

pub fn temporal_replay_parity_report(
    expected: &TemporalReconstructabilityArtifact,
    replayed: &TemporalReconstructabilityArtifact,
) -> TemporalReplayParityReport {
    let mut mismatch_classes = Vec::new();
    if expected.clock_checkpoint_digest != replayed.clock_checkpoint_digest {
        mismatch_classes.push(TemporalReplayMismatchClass::ClockCheckpointDigestMismatch);
    }
    if expected.scheduled_wake_digest != replayed.scheduled_wake_digest {
        mismatch_classes.push(TemporalReplayMismatchClass::ScheduledWakeDigestMismatch);
    }
    if expected.ready_wake_digest != replayed.ready_wake_digest {
        mismatch_classes.push(TemporalReplayMismatchClass::ReadyWakeDigestMismatch);
    }
    if expected.retired_wake_digest != replayed.retired_wake_digest {
        mismatch_classes.push(TemporalReplayMismatchClass::RetiredWakeDigestMismatch);
    }
    if expected.rescheduled_wake_digest != replayed.rescheduled_wake_digest {
        mismatch_classes.push(TemporalReplayMismatchClass::RescheduledWakeDigestMismatch);
    }
    if expected.reused_wake_digest != replayed.reused_wake_digest {
        mismatch_classes.push(TemporalReplayMismatchClass::ReusedWakeDigestMismatch);
    }
    if expected.interval_regeneration_digest != replayed.interval_regeneration_digest {
        mismatch_classes.push(TemporalReplayMismatchClass::IntervalRegenerationDigestMismatch);
    }
    if expected.temporal_eligibility_digest != replayed.temporal_eligibility_digest {
        mismatch_classes.push(TemporalReplayMismatchClass::TemporalEligibilityDigestMismatch);
    }
    if expected.previous_value_reference_digest != replayed.previous_value_reference_digest {
        mismatch_classes.push(TemporalReplayMismatchClass::PreviousValueReferenceDigestMismatch);
    }
    if expected.certification_digest != replayed.certification_digest {
        mismatch_classes.push(TemporalReplayMismatchClass::CertificationDigestMismatch);
    }
    if expected.wake_summary != replayed.wake_summary {
        mismatch_classes.push(TemporalReplayMismatchClass::WakeSummaryMismatch);
    }
    TemporalReplayParityReport {
        proof_schema_version: TEMPORAL_REPLAY_PARITY_SCHEMA_VERSION.to_owned(),
        expected: expected.clone(),
        replayed: replayed.clone(),
        parity: mismatch_classes.is_empty(),
        mismatch_classes,
    }
}

pub fn temporal_certification_record(
    family: TemporalCertificationFamily,
    artifact: TemporalReconstructabilityArtifact,
    parity: Option<TemporalReplayParityReport>,
) -> TemporalCertificationRecord {
    let passed = parity.as_ref().map(|report| report.parity).unwrap_or(true);
    TemporalCertificationRecord {
        family,
        artifact,
        parity,
        passed,
    }
}

pub fn temporal_certification_builder() -> TemporalCertificationBuilder {
    TemporalCertificationBuilder::new()
}

impl TemporalCertificationBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_temporal_eligibility_replay_parity(
        mut self,
        artifact: TemporalReconstructabilityArtifact,
        parity: TemporalReplayParityReport,
    ) -> Result<Self, SignalError> {
        Self::ensure_empty(
            &self.temporal_eligibility_replay_parity,
            TemporalCertificationFamily::TemporalEligibilityReplayParity,
        )?;
        Self::ensure_parity_evidence(
            TemporalCertificationFamily::TemporalEligibilityReplayParity,
            &artifact,
            &parity,
        )?;
        if artifact.eligibility_fact_count == 0 {
            return Err(Self::invalid_family_evidence(
                TemporalCertificationFamily::TemporalEligibilityReplayParity,
                "requires at least one lowered temporal eligibility fact",
            ));
        }
        self.temporal_eligibility_replay_parity = Some(temporal_certification_record(
            TemporalCertificationFamily::TemporalEligibilityReplayParity,
            artifact,
            Some(parity),
        ));
        Ok(self)
    }

    pub fn with_temporal_branch_restore_equivalence(
        mut self,
        artifact: TemporalReconstructabilityArtifact,
        parity: TemporalReplayParityReport,
    ) -> Result<Self, SignalError> {
        Self::ensure_empty(
            &self.temporal_branch_restore_equivalence,
            TemporalCertificationFamily::TemporalBranchRestoreEquivalence,
        )?;
        Self::ensure_parity_evidence(
            TemporalCertificationFamily::TemporalBranchRestoreEquivalence,
            &artifact,
            &parity,
        )?;
        if artifact.wake_summary.scheduled_count() == 0
            && artifact.wake_summary.ready_count() == 0
            && artifact.wake_summary.retired_count() == 0
        {
            return Err(Self::invalid_family_evidence(
                TemporalCertificationFamily::TemporalBranchRestoreEquivalence,
                "requires restored branch-local temporal wake state",
            ));
        }
        self.temporal_branch_restore_equivalence = Some(temporal_certification_record(
            TemporalCertificationFamily::TemporalBranchRestoreEquivalence,
            artifact,
            Some(parity),
        ));
        Ok(self)
    }

    pub fn with_temporal_wake_boundedness(
        mut self,
        artifact: TemporalReconstructabilityArtifact,
    ) -> Result<Self, SignalError> {
        Self::ensure_empty(
            &self.temporal_wake_boundedness,
            TemporalCertificationFamily::TemporalWakeBoundedness,
        )?;
        Self::ensure_non_default_artifact(
            TemporalCertificationFamily::TemporalWakeBoundedness,
            &artifact,
        )?;
        if artifact.interval_regeneration_count == 0 {
            return Err(Self::invalid_family_evidence(
                TemporalCertificationFamily::TemporalWakeBoundedness,
                "requires interval regeneration evidence",
            ));
        }
        self.temporal_wake_boundedness = Some(temporal_certification_record(
            TemporalCertificationFamily::TemporalWakeBoundedness,
            artifact,
            None,
        ));
        Ok(self)
    }

    pub fn with_previous_value_time_gated_equivalence(
        mut self,
        artifact: TemporalReconstructabilityArtifact,
    ) -> Result<Self, SignalError> {
        Self::ensure_empty(
            &self.previous_value_time_gated_equivalence,
            TemporalCertificationFamily::PreviousValueTimeGatedEquivalence,
        )?;
        Self::ensure_non_default_artifact(
            TemporalCertificationFamily::PreviousValueTimeGatedEquivalence,
            &artifact,
        )?;
        if artifact.previous_value_reference_count == 0 {
            return Err(Self::invalid_family_evidence(
                TemporalCertificationFamily::PreviousValueTimeGatedEquivalence,
                "requires at least one temporal previous-value reference",
            ));
        }
        self.previous_value_time_gated_equivalence = Some(temporal_certification_record(
            TemporalCertificationFamily::PreviousValueTimeGatedEquivalence,
            artifact,
            None,
        ));
        Ok(self)
    }

    pub fn build(self) -> Result<TemporalCertificationBundle, SignalError> {
        let records = [
            self.temporal_eligibility_replay_parity,
            self.temporal_branch_restore_equivalence,
            self.temporal_wake_boundedness,
            self.previous_value_time_gated_equivalence,
        ];
        let mut complete = Vec::with_capacity(REQUIRED_TEMPORAL_CERTIFICATION_FAMILIES.len());
        for (family, record) in REQUIRED_TEMPORAL_CERTIFICATION_FAMILIES
            .into_iter()
            .zip(records)
        {
            let Some(record) = record else {
                return Err(Self::invalid_family_evidence(
                    family,
                    "required certification family was not supplied",
                ));
            };
            complete.push(record);
        }

        let bundle = temporal_certification_bundle(complete);
        bundle.ensure_passed()?;
        Ok(bundle)
    }

    fn ensure_empty(
        existing: &Option<TemporalCertificationRecord>,
        family: TemporalCertificationFamily,
    ) -> Result<(), SignalError> {
        if existing.is_none() {
            return Ok(());
        }
        Err(Self::invalid_family_evidence(
            family,
            "duplicate certification family evidence",
        ))
    }

    fn ensure_parity_evidence(
        family: TemporalCertificationFamily,
        artifact: &TemporalReconstructabilityArtifact,
        parity: &TemporalReplayParityReport,
    ) -> Result<(), SignalError> {
        Self::ensure_non_default_artifact(family, artifact)?;
        if !parity.parity {
            return Err(Self::invalid_family_evidence(
                family,
                "requires a passing temporal replay parity report",
            ));
        }
        if &parity.replayed != artifact {
            return Err(Self::invalid_family_evidence(
                family,
                "record artifact must be the replayed temporal artifact from the parity report",
            ));
        }
        Ok(())
    }

    fn ensure_non_default_artifact(
        family: TemporalCertificationFamily,
        artifact: &TemporalReconstructabilityArtifact,
    ) -> Result<(), SignalError> {
        if artifact.certification_digest.is_empty() {
            return Err(Self::invalid_family_evidence(
                family,
                "artifact certification digest is empty",
            ));
        }
        if artifact == &TemporalReconstructabilityArtifact::default() {
            return Err(Self::invalid_family_evidence(
                family,
                "default temporal artifact is not certification evidence",
            ));
        }
        Ok(())
    }

    fn invalid_family_evidence(
        family: TemporalCertificationFamily,
        reason: &'static str,
    ) -> SignalError {
        SignalError::invalid_input(format!(
            "invalid temporal certification evidence for {family:?}: {reason}"
        ))
    }
}

pub fn temporal_certification_bundle(
    records: impl IntoIterator<Item = TemporalCertificationRecord>,
) -> TemporalCertificationBundle {
    let mut records = records.into_iter().collect::<Vec<_>>();
    records.sort_by_key(|record| record.family);

    let mut by_family: BTreeMap<TemporalCertificationFamily, Vec<&TemporalCertificationRecord>> =
        BTreeMap::new();
    for record in &records {
        by_family.entry(record.family).or_default().push(record);
    }

    let mut failures = Vec::new();
    for family in REQUIRED_TEMPORAL_CERTIFICATION_FAMILIES {
        match by_family.get(&family) {
            None => failures.push(TemporalCertificationFailure::MissingRequiredFamily { family }),
            Some(records_for_family) if records_for_family.len() > 1 => {
                failures.push(TemporalCertificationFailure::DuplicateFamily {
                    family,
                    count: records_for_family.len() as u32,
                });
            }
            Some(_) => {}
        }
    }

    for record in &records {
        if record.artifact.certification_digest.is_empty() {
            failures.push(TemporalCertificationFailure::EmptyCertificationDigest {
                family: record.family,
            });
        }
        if !record.passed {
            failures.push(TemporalCertificationFailure::FailedFamily {
                family: record.family,
            });
        }
        if let Some(parity) = record.parity.as_ref() {
            if !parity.parity {
                failures.push(TemporalCertificationFailure::ParityMismatch {
                    family: record.family,
                    mismatch_classes: parity.mismatch_classes.clone(),
                });
            }
        }
    }

    let failed_families = failures
        .iter()
        .map(TemporalCertificationFailure::family)
        .collect::<BTreeSet<_>>();
    let passed_family_count = REQUIRED_TEMPORAL_CERTIFICATION_FAMILIES
        .iter()
        .filter(|family| {
            by_family
                .get(family)
                .is_some_and(|records_for_family| records_for_family.len() == 1)
                && !failed_families.contains(family)
        })
        .count() as u32;
    let missing_family_count = REQUIRED_TEMPORAL_CERTIFICATION_FAMILIES
        .iter()
        .filter(|family| !by_family.contains_key(family))
        .count() as u32;
    let duplicate_family_count = by_family
        .values()
        .filter(|records_for_family| records_for_family.len() > 1)
        .count() as u32;
    let failed_family_count = failed_families.len() as u32;
    let summary = TemporalCertificationSummary {
        required_family_count: REQUIRED_TEMPORAL_CERTIFICATION_FAMILIES.len() as u32,
        provided_record_count: records.len() as u32,
        passed_family_count,
        failed_family_count,
        missing_family_count,
        duplicate_family_count,
    };
    let digest = canonical_digest(&TemporalCertificationBundleDigestBasis {
        schema_version: TEMPORAL_CERTIFICATION_BUNDLE_SCHEMA_VERSION,
        records: &records,
    });
    let passed = failures.is_empty();
    TemporalCertificationBundle {
        schema_version: TEMPORAL_CERTIFICATION_BUNDLE_SCHEMA_VERSION.to_owned(),
        records,
        summary,
        bundle_digest: digest,
        passed,
        failures,
    }
}

impl TemporalCertificationBundle {
    pub fn ensure_passed(&self) -> Result<(), SignalError> {
        if self.passed {
            return Ok(());
        }
        Err(SignalError::invalid_input(format!(
            "temporal certification bundle failed with {} failure(s)",
            self.failures.len()
        )))
    }
}

pub fn temporal_certification_bundle_parity_report(
    expected: &TemporalCertificationBundle,
    replayed: &TemporalCertificationBundle,
) -> TemporalCertificationBundleParityReport {
    let mut mismatch_classes = Vec::new();
    if expected.schema_version != replayed.schema_version {
        mismatch_classes.push(TemporalCertificationBundleMismatchClass::SchemaVersionMismatch);
    }
    if expected.bundle_digest != replayed.bundle_digest {
        mismatch_classes.push(TemporalCertificationBundleMismatchClass::BundleDigestMismatch);
    }
    if expected.passed != replayed.passed {
        mismatch_classes.push(TemporalCertificationBundleMismatchClass::PassStatusMismatch);
    }
    if expected.summary != replayed.summary {
        mismatch_classes.push(TemporalCertificationBundleMismatchClass::SummaryMismatch);
    }
    if expected.failures != replayed.failures {
        mismatch_classes.push(TemporalCertificationBundleMismatchClass::FailureSetMismatch);
    }
    if expected.records != replayed.records {
        mismatch_classes.push(TemporalCertificationBundleMismatchClass::RecordSetMismatch);
    }
    TemporalCertificationBundleParityReport {
        proof_schema_version: TEMPORAL_CERTIFICATION_BUNDLE_PARITY_SCHEMA_VERSION.to_owned(),
        expected: expected.clone(),
        replayed: replayed.clone(),
        parity: mismatch_classes.is_empty(),
        mismatch_classes,
    }
}

#[derive(Debug, Serialize)]
struct TemporalCertificationBundleDigestBasis<'a> {
    schema_version: &'static str,
    records: &'a [TemporalCertificationRecord],
}

impl TemporalReconstructabilityArtifact {
    pub fn from_evidence(
        wake_summary: TemporalWakeSummary,
        evidence: &TemporalTransactionEvidence,
    ) -> Self {
        let clock_checkpoint_digest = canonical_digest(&evidence.clock_basis);
        let scheduled_wake_digest = canonical_digest(&evidence.scheduled_wakes);
        let ready_wake_digest = canonical_digest(&evidence.ready_wakes);
        let retired_wake_digest = canonical_digest(&evidence.retired_wakes);
        let rescheduled_wake_digest = canonical_digest(&evidence.rescheduled_wakes);
        let reused_wake_digest = canonical_digest(&evidence.reused_wakes);
        let interval_regeneration_digest = canonical_digest(&evidence.interval_regenerations);
        let temporal_eligibility_digest = canonical_digest(&evidence.eligibility_facts);
        let previous_value_reference_digest = canonical_digest(&evidence.previous_value_references);
        let certification_digest = canonical_digest(&TemporalCertificationDigestBasis {
            clock_checkpoint_digest: &clock_checkpoint_digest,
            scheduled_wake_digest: &scheduled_wake_digest,
            ready_wake_digest: &ready_wake_digest,
            retired_wake_digest: &retired_wake_digest,
            rescheduled_wake_digest: &rescheduled_wake_digest,
            reused_wake_digest: &reused_wake_digest,
            interval_regeneration_digest: &interval_regeneration_digest,
            temporal_eligibility_digest: &temporal_eligibility_digest,
            previous_value_reference_digest: &previous_value_reference_digest,
        });
        Self {
            clock_basis: evidence.clock_basis,
            wake_summary,
            eligibility_fact_count: evidence.eligibility_facts.len() as u64,
            scheduled_wake_count: evidence.scheduled_wakes.len() as u64,
            ready_wake_count: evidence.ready_wakes.len() as u64,
            retired_wake_count: evidence.retired_wakes.len() as u64,
            rescheduled_wake_count: evidence.rescheduled_wakes.len() as u64,
            reused_wake_count: evidence.reused_wakes.len() as u64,
            interval_regeneration_count: evidence.interval_regenerations.len() as u64,
            previous_value_reference_count: evidence.previous_value_references.len() as u64,
            clock_checkpoint_digest,
            scheduled_wake_digest,
            ready_wake_digest,
            retired_wake_digest,
            rescheduled_wake_digest,
            reused_wake_digest,
            interval_regeneration_digest,
            temporal_eligibility_digest,
            previous_value_reference_digest,
            certification_digest,
        }
    }

    pub(in crate::logic::transaction::runtime) fn from_temporal_state(
        temporal: &TemporalRuntimeState,
    ) -> Self {
        let evidence = TemporalTransactionEvidence {
            clock_basis: temporal.clock_basis(),
            eligibility_facts: Vec::new(),
            scheduled_wakes: temporal.scheduled_wake_evidence(),
            ready_wakes: temporal.ready_wake_evidence(),
            retired_wakes: temporal.retired_wake_evidence(),
            rescheduled_wakes: Vec::new(),
            reused_wakes: Vec::new(),
            interval_regenerations: Vec::new(),
            previous_value_references: Vec::new(),
        };
        Self::from_evidence(temporal.wake_summary(), &evidence)
    }
}

#[derive(Debug, Serialize)]
struct TemporalCertificationDigestBasis<'a> {
    clock_checkpoint_digest: &'a str,
    scheduled_wake_digest: &'a str,
    ready_wake_digest: &'a str,
    retired_wake_digest: &'a str,
    rescheduled_wake_digest: &'a str,
    reused_wake_digest: &'a str,
    interval_regeneration_digest: &'a str,
    temporal_eligibility_digest: &'a str,
    previous_value_reference_digest: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReconstructabilityProof {
    pub checkpoint: CheckpointBoundary,
    pub journal: BoundedJournalSegment,
    pub temporal: TemporalReconstructabilityArtifact,
    pub required_rebuild: Vec<RequiredDerivedRebuildSet>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReconstructabilityRecord {
    pub authority_branch_id: SignalBranchId,
    pub authority_snapshot_id: Option<SignalSnapshotId>,
    pub replay_head: Option<ReplayCursor>,
    pub checkpoint: CheckpointRecord,
    pub journal: JournalSegment,
    #[serde(default)]
    pub temporal: TemporalReconstructabilityArtifact,
}

impl ReconstructabilityRecord {
    pub fn from_transaction_boundary(
        authority_branch_id: SignalBranchId,
        authority_snapshot_id: Option<SignalSnapshotId>,
        replay_head: Option<ReplayCursor>,
        checkpoint: CheckpointRecord,
        replay_entries: &[TransactionReplayEntry],
        temporal: TemporalReconstructabilityArtifact,
    ) -> Self {
        Self {
            authority_branch_id,
            authority_snapshot_id,
            replay_head,
            checkpoint,
            journal: JournalSegment::from_entries(replay_entries),
            temporal,
        }
    }

    pub fn from_snapshot_boundary(
        authority_branch_id: SignalBranchId,
        authority_snapshot_id: SignalSnapshotId,
        replay_head: Option<ReplayCursor>,
        mut checkpoint: CheckpointRecord,
        replay_entries: &[ReplayEvent],
        temporal: TemporalReconstructabilityArtifact,
    ) -> Self {
        let journal = JournalSegment::from_replay_events(replay_entries);
        checkpoint.journal_replay_span = journal.replay_event_count as u64;
        Self {
            authority_branch_id,
            authority_snapshot_id: Some(authority_snapshot_id),
            replay_head,
            checkpoint,
            journal,
            temporal,
        }
    }

    pub fn checkpoint_boundary(&self) -> CheckpointBoundary {
        CheckpointBoundary {
            authority_branch_id: self.authority_branch_id,
            authority_snapshot_id: self.authority_snapshot_id,
            replay_head: self.replay_head,
            checkpoint: self.checkpoint,
        }
    }

    pub fn required_derived_rebuild_set(&self) -> Vec<RequiredDerivedRebuildSet> {
        let mut rebuild = vec![RequiredDerivedRebuildSet::DependencyIndexes(
            DependencyIndexRebuildProof {
                authority_branch_id: self.authority_branch_id,
                authority_snapshot_id: self.authority_snapshot_id,
            },
        )];
        rebuild.push(RequiredDerivedRebuildSet::ReplaySuffix(
            ReplaySuffixRebuildProof {
                replay_head: self.replay_head,
                replay_event_count: self.journal.replay_event_count,
            },
        ));
        if self.journal.replay_event_count > 0 {
            rebuild.push(RequiredDerivedRebuildSet::MergeSupport(
                MergeSupportRebuildProof {
                    authority_branch_id: self.authority_branch_id,
                    replay_event_count: self.journal.replay_event_count,
                },
            ));
        }
        rebuild.push(RequiredDerivedRebuildSet::TemporalState(
            TemporalStateRebuildProof {
                authority_branch_id: self.authority_branch_id,
                authority_snapshot_id: self.authority_snapshot_id,
                scheduled_wake_count: self.temporal.wake_summary.scheduled_count(),
                ready_wake_count: self.temporal.wake_summary.ready_count(),
                retired_wake_count: self.temporal.wake_summary.retired_count(),
            },
        ));
        rebuild
    }

    pub fn proof(&self) -> ReconstructabilityProof {
        ReconstructabilityProof {
            checkpoint: self.checkpoint_boundary(),
            journal: BoundedJournalSegment::from_record(self.replay_head, &self.journal),
            temporal: self.temporal.clone(),
            required_rebuild: self.required_derived_rebuild_set(),
        }
    }
}
