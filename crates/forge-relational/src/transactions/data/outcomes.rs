use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::Arc;

use crate::diagnostics::data::DiagnosticCode;
use crate::diagnostics::data::RelationalDiagnosticArtifact;
use crate::errors::data::{ErrorContext, ErrorOperation, RelationalSubsystem, SuggestedFix};
use crate::identity::data::{EntityId, RelationId, VersionId};
use crate::performance::data::RuntimeComplexityCounters;
use crate::publication::data::diff::PatchRecord;
use crate::publication::data::{PublicationError, PublicationStatus};
use crate::replay::data::CanonicalCommitEnvelope;
use crate::snapshots::data::SnapshotHandle;
use crate::validation::data::{InvariantExecutionPoint, InvariantGroupSet};
use crate::validation::engine::{
    InvariantExecutionDisposition, InvariantExecutionResult, InvariantObservationKind,
};

use super::{
    CommitChangeSummary, CommitHistorySummary, CommitLog, CommitPatchBudgetSummary,
    CommitPublicationSummary, CommitSummary, ExistingRecordTarget, MutationIntent, RecordRef,
    SavepointId, TransactionId,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergedCommitPlan {
    pub transaction_id: TransactionId,
    pub merged_intents: Vec<MutationIntent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthoritativeApplyPlan {
    pub transaction_id: TransactionId,
    pub version_id: VersionId,
    pub merged_intents: Vec<MutationIntent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UndoRecord {
    pub record: RecordRef,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitConflict {
    pub class: ConflictClass,
    pub code: DiagnosticCode,
    pub detail: String,
    pub context: ErrorContext,
}

impl CommitConflict {
    pub(crate) fn new(class: ConflictClass) -> Self {
        let code = class.code();
        let detail = class.detail();
        Self {
            class,
            code,
            detail,
            context: ErrorContext::new(RelationalSubsystem::Transaction, ErrorOperation::Validate)
                .with_fix(SuggestedFix::InspectDiagnostics),
        }
    }

    pub fn code(&self) -> DiagnosticCode {
        self.code
    }

    pub fn detail(&self) -> String {
        self.detail.clone()
    }

    pub fn with_context(mut self, context: ErrorContext) -> Self {
        self.context = context;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictClass {
    StaleTarget {
        target: ExistingRecordTarget,
        context: String,
    },
    InvalidRelationEndpoint {
        detail: String,
    },
    DuplicateRelationIdentity {
        detail: String,
    },
    InvariantViolation {
        code: DiagnosticCode,
        detail: String,
    },
    KindSchemaMismatch {
        detail: String,
    },
    ConflictingIntent {
        target: ExistingRecordTarget,
    },
    InvalidSavepoint {
        savepoint_id: SavepointId,
    },
    InvalidMergeParent {
        detail: String,
    },
    MergeConflictOverlap {
        detail: String,
    },
    MissingMergeBase {
        detail: String,
    },
}

impl ConflictClass {
    pub fn code(&self) -> DiagnosticCode {
        match self {
            Self::StaleTarget { .. } => DiagnosticCode::StaleHandle,
            Self::InvalidRelationEndpoint { .. } => DiagnosticCode::InvalidRelationEndpoint,
            Self::DuplicateRelationIdentity { .. } => DiagnosticCode::DuplicateRelationIdentity,
            Self::InvariantViolation { code, .. } => *code,
            Self::KindSchemaMismatch { .. } => DiagnosticCode::InvariantViolation,
            Self::ConflictingIntent { .. } => DiagnosticCode::ConflictingIntent,
            Self::InvalidSavepoint { .. } => DiagnosticCode::InvalidSavepoint,
            Self::InvalidMergeParent { .. } => DiagnosticCode::InvalidMergeParent,
            Self::MergeConflictOverlap { .. } => DiagnosticCode::MergeConflictOverlap,
            Self::MissingMergeBase { .. } => DiagnosticCode::MissingMergeBase,
        }
    }

    pub fn detail(&self) -> String {
        match self {
            Self::StaleTarget { target, context } => match target {
                ExistingRecordTarget::Entity(entity_id) => format!(
                    "entity {:?} changed before authoritative apply ({context})",
                    entity_id
                ),
                ExistingRecordTarget::Relation(relation_id) => format!(
                    "relation {:?} changed before authoritative apply ({context})",
                    relation_id
                ),
            },
            Self::InvalidRelationEndpoint { detail }
            | Self::DuplicateRelationIdentity { detail }
            | Self::KindSchemaMismatch { detail }
            | Self::InvalidMergeParent { detail }
            | Self::MergeConflictOverlap { detail }
            | Self::MissingMergeBase { detail } => detail.clone(),
            Self::InvariantViolation { detail, .. } => detail.clone(),
            Self::ConflictingIntent { target } => match target {
                ExistingRecordTarget::Entity(entity_id) => {
                    format!(
                        "conflicting entity intent for slot {}",
                        entity_id.local_slot.0
                    )
                }
                ExistingRecordTarget::Relation(relation_id) => {
                    format!(
                        "conflicting relation intent for slot {}",
                        relation_id.local_slot.0
                    )
                }
            },
            Self::InvalidSavepoint { savepoint_id } => {
                format!("savepoint {:?} does not exist", savepoint_id)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionCommitError {
    Conflict {
        error: CommitConflict,
        commit_log: CommitLog,
    },
    Publication {
        error: PublicationError,
        commit_log: CommitLog,
    },
}

impl TransactionCommitError {
    pub fn conflict(error: CommitConflict) -> Self {
        Self::Conflict {
            error,
            commit_log: CommitLog::new(),
        }
    }

    pub fn publication(error: PublicationError) -> Self {
        Self::Publication {
            error,
            commit_log: CommitLog::new(),
        }
    }

    pub fn with_commit_log(self, commit_log: CommitLog) -> Self {
        match self {
            Self::Conflict { error, .. } => Self::Conflict { error, commit_log },
            Self::Publication { error, .. } => Self::Publication { error, commit_log },
        }
    }

    pub fn context(&self) -> &ErrorContext {
        match self {
            Self::Conflict { error, .. } => &error.context,
            Self::Publication { error, .. } => &error.context,
        }
    }

    pub fn detail(&self) -> String {
        match self {
            Self::Conflict { error, .. } => error.detail(),
            Self::Publication { error, .. } => error.detail.clone(),
        }
    }

    pub fn commit_log(&self) -> &CommitLog {
        match self {
            Self::Conflict { commit_log, .. } => commit_log,
            Self::Publication { commit_log, .. } => commit_log,
        }
    }

    pub fn commit_summary(&self) -> &CommitSummary {
        self.commit_log().summary()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitOutcome {
    pub transaction_id: TransactionId,
    pub commit: crate::history::data::CommitReference,
    pub version_id: VersionId,
    pub snapshot: SnapshotHandle,
    pub changed_records: Vec<RecordRef>,
    pub publication_status: PublicationStatus,
    pub commit_log: CommitLog,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CommitPhaseTiming {
    pub working_state_preparation_micros: u64,
    pub invariant_pre_check_micros: u64,
    pub authoritative_mutation_micros: u64,
    pub history_resolution_micros: u64,
    pub invariant_post_check_micros: u64,
    pub artifact_assembly_micros: u64,
    pub durable_append_micros: u64,
    pub publication_micros: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitPublication {
    pub diagnostics: Vec<RelationalDiagnosticArtifact>,
    pub envelope: Arc<CanonicalCommitEnvelope>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitStructuralSummary {
    pub invariant_groups: InvariantGroupSet,
    pub commit_topology: super::CommitTopology,
    pub touched_partitions: Vec<crate::identity::data::PartitionId>,
    pub bulk_entity_slots_reserved: usize,
    pub bulk_relation_slots_reserved: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitValidation {
    pub summary: CommitValidationSummary,
    pub invariant_executions: Vec<InvariantExecutionResult>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CommitValidationSummary {
    pub execution_count: usize,
    pub executed_count: usize,
    pub skipped_count: usize,
    pub committed_observation_count: usize,
    pub speculative_observation_count: usize,
    pub plan_backed_execution_count: usize,
    pub commit_boundary_seen: bool,
    pub mutation_sensitive_seen: bool,
    pub snapshot_publication_seen: bool,
    pub harness_audit_seen: bool,
    pub consumed_groups: InvariantGroupSet,
    pub applicable_groups: InvariantGroupSet,
    pub result_count: usize,
    pub advisory_count: usize,
    pub violation_count: usize,
    pub blocking_violation: bool,
    pub publication_violation: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitExecution {
    pub phase_timing: CommitPhaseTiming,
    pub complexity_delta: RuntimeComplexityCounters,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitResult {
    pub outcome: CommitOutcome,
    pub summary: CommitSummary,
    pub structural_summary: CommitStructuralSummary,
    pub publication: CommitPublication,
    pub validation: CommitValidation,
    pub execution: CommitExecution,
}

impl Deref for CommitResult {
    type Target = CommitOutcome;

    fn deref(&self) -> &Self::Target {
        &self.outcome
    }
}

impl CommitOutcome {
    pub fn summary(&self) -> &CommitLog {
        &self.commit_log
    }

    pub fn commit_summary(&self) -> &CommitSummary {
        self.commit_log.summary()
    }

    pub fn history_summary(&self) -> Option<&CommitHistorySummary> {
        self.commit_summary().history_summary.as_ref()
    }

    pub fn patch_budget_summary(&self) -> Option<&CommitPatchBudgetSummary> {
        self.commit_summary().patch_budget_summary.as_ref()
    }

    pub fn change_summary(&self) -> Option<&CommitChangeSummary> {
        self.commit_summary().change_summary.as_ref()
    }

    pub fn publication_summary(&self) -> Option<&CommitPublicationSummary> {
        self.commit_summary().publication_summary.as_ref()
    }
}

impl CommitResult {
    pub fn commit_log(&self) -> &CommitLog {
        &self.outcome.commit_log
    }

    pub fn commit_summary(&self) -> &CommitSummary {
        &self.summary
    }

    pub fn publication(&self) -> &CommitPublication {
        &self.publication
    }

    pub fn structural_summary(&self) -> &CommitStructuralSummary {
        &self.structural_summary
    }

    pub fn history_summary(&self) -> Option<&CommitHistorySummary> {
        self.summary.history_summary.as_ref()
    }

    pub fn patch_budget_summary(&self) -> Option<&CommitPatchBudgetSummary> {
        self.summary.patch_budget_summary.as_ref()
    }

    pub fn change_summary(&self) -> Option<&CommitChangeSummary> {
        self.summary.change_summary.as_ref()
    }

    pub fn publication_summary(&self) -> Option<&CommitPublicationSummary> {
        self.summary.publication_summary.as_ref()
    }

    pub fn validation(&self) -> &CommitValidation {
        &self.validation
    }

    pub fn execution(&self) -> &CommitExecution {
        &self.execution
    }

    pub fn diagnostics(&self) -> &[RelationalDiagnosticArtifact] {
        &self.publication.diagnostics
    }

    pub fn patch(&self) -> &[PatchRecord] {
        &self.publication.envelope.patch.records
    }

    pub fn envelope(&self) -> &CanonicalCommitEnvelope {
        self.publication.envelope.as_ref()
    }

    pub fn invariant_executions(&self) -> &[InvariantExecutionResult] {
        self.validation.invariant_executions()
    }

    pub fn validation_summary(&self) -> CommitValidationSummary {
        self.validation.summary()
    }

    pub fn phase_timing(&self) -> &CommitPhaseTiming {
        &self.execution.phase_timing
    }

    pub fn complexity_delta(&self) -> &RuntimeComplexityCounters {
        &self.execution.complexity_delta
    }

    pub fn patch_position(&self) -> crate::publication::data::diff::PatchStreamPosition {
        self.publication.envelope.patch.position
    }

    pub fn final_snapshot_id(&self) -> crate::snapshots::data::SnapshotId {
        self.outcome.snapshot.snapshot_id
    }

    pub fn merge_parent_count(&self) -> usize {
        self.outcome.commit.parents.len().saturating_sub(1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RollbackEffect {
    RestoredEntity(EntityId),
    RestoredRelation(RelationId),
    DiscardedEntityCreation,
    DiscardedRelationCreation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct RollbackSummary {
    pub restored_entity_count: usize,
    pub restored_relation_count: usize,
    pub discarded_entity_creation_count: usize,
    pub discarded_relation_creation_count: usize,
}

impl RollbackSummary {
    pub fn from_effects(effects: &[RollbackEffect]) -> Self {
        let mut summary = Self::default();

        for effect in effects {
            match effect {
                RollbackEffect::RestoredEntity(_) => {
                    summary.restored_entity_count += 1;
                }
                RollbackEffect::RestoredRelation(_) => {
                    summary.restored_relation_count += 1;
                }
                RollbackEffect::DiscardedEntityCreation => {
                    summary.discarded_entity_creation_count += 1;
                }
                RollbackEffect::DiscardedRelationCreation => {
                    summary.discarded_relation_creation_count += 1;
                }
            }
        }

        summary
    }

    pub fn total_effect_count(&self) -> usize {
        self.restored_entity_count
            + self.restored_relation_count
            + self.discarded_entity_creation_count
            + self.discarded_relation_creation_count
    }

    pub fn restored_record_count(&self) -> usize {
        self.restored_entity_count + self.restored_relation_count
    }

    pub fn discarded_creation_count(&self) -> usize {
        self.discarded_entity_creation_count + self.discarded_relation_creation_count
    }

    pub fn has_restored_entity(&self) -> bool {
        self.restored_entity_count > 0
    }

    pub fn has_discarded_entity_creation(&self) -> bool {
        self.discarded_entity_creation_count > 0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackOutcome {
    pub transaction_id: TransactionId,
    pub summary: RollbackSummary,
    pub effects: Vec<RollbackEffect>,
}

impl RollbackOutcome {
    pub fn summary(&self) -> &RollbackSummary {
        &self.summary
    }

    pub fn effects(&self) -> &[RollbackEffect] {
        &self.effects
    }

    pub fn effect_count(&self) -> usize {
        self.summary.total_effect_count()
    }

    pub fn has_effects(&self) -> bool {
        self.effect_count() > 0
    }
}

impl CommitValidation {
    pub fn invariant_executions(&self) -> &[InvariantExecutionResult] {
        &self.invariant_executions
    }

    pub fn summarize(invariant_executions: &[InvariantExecutionResult]) -> CommitValidationSummary {
        let mut summary = CommitValidationSummary {
            execution_count: invariant_executions.len(),
            ..CommitValidationSummary::default()
        };

        for execution in invariant_executions {
            if execution.metadata().has_merged_plan() {
                summary.plan_backed_execution_count += 1;
            }

            match execution.metadata().observation_kind() {
                InvariantObservationKind::Committed => {
                    summary.committed_observation_count += 1;
                }
                InvariantObservationKind::Speculative => {
                    summary.speculative_observation_count += 1;
                }
            }

            match execution.metadata().execution_point() {
                InvariantExecutionPoint::CommitBoundary => {
                    summary.commit_boundary_seen = true;
                }
                InvariantExecutionPoint::MutationSensitive => {
                    summary.mutation_sensitive_seen = true;
                }
                InvariantExecutionPoint::SnapshotPublication => {
                    summary.snapshot_publication_seen = true;
                }
                InvariantExecutionPoint::HarnessAudit => {
                    summary.harness_audit_seen = true;
                }
            }

            summary.consumed_groups = summary
                .consumed_groups
                .union(execution.metadata().consumed_groups());
            summary.applicable_groups = summary
                .applicable_groups
                .union(execution.metadata().applicable_groups());

            match execution.metadata().disposition() {
                InvariantExecutionDisposition::Executed => {
                    summary.executed_count += 1;
                }
                InvariantExecutionDisposition::SkippedByPlanContract
                | InvariantExecutionDisposition::SkippedByMayBreakMask => {
                    summary.skipped_count += 1;
                }
            }

            let execution_summary = execution.summary();
            summary.result_count += execution_summary.result_count();
            summary.advisory_count += execution_summary.advisory_count();
            summary.violation_count += execution_summary.violation_count();
            summary.blocking_violation |= execution_summary.has_blocking_violation();
            summary.publication_violation |= execution_summary.has_publication_violation();
        }

        summary
    }

    pub fn summary(&self) -> CommitValidationSummary {
        self.summary
    }
}
