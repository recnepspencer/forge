use serde::{Deserialize, Serialize};
use serde_json::Value;
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
use crate::schema::data::{
    DescriptorSemanticsVersion, SchemaTransitionSummary, SchemaVersionId,
};
use crate::snapshots::data::SnapshotHandle;
use crate::validation::data::{InvariantExecutionPoint, InvariantGroupSet};
use crate::validation::engine::{
    InvariantExecutionDisposition, InvariantExecutionResult, InvariantObservationKind,
};

use super::{
    AspectEmissionTrace, AspectEvaluationTrace, AspectTagAccuracyReport, CommitAspectSummary,
    CommitChangeSummary, CommitHistorySummary, CommitLog, CommitPatchBudgetSummary,
    CommitPublicationSummary, CommitSummary, ExistingRecordTarget, MutationIntent,
    PatchVsTruthDeltaReport, RecordRef, SavepointId, TransactionId,
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
    pub fields: Option<Value>,
    pub context: ErrorContext,
}

impl CommitConflict {
    pub(crate) fn new(class: ConflictClass) -> Self {
        let code = class.code();
        let detail = class.detail();
        let fields = class.fields();
        Self {
            class,
            code,
            detail,
            fields,
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

    pub fn fields(&self) -> Option<&Value> {
        self.fields.as_ref()
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
        fields: Value,
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
    UndeclaredSchemaTransition {
        previous_schema_version: SchemaVersionId,
        current_schema_version: SchemaVersionId,
        previous_descriptor_semantics_version: DescriptorSemanticsVersion,
        current_descriptor_semantics_version: DescriptorSemanticsVersion,
    },
    DescriptorVersionIncompatibility {
        previous_descriptor_semantics_version: DescriptorSemanticsVersion,
        current_descriptor_semantics_version: DescriptorSemanticsVersion,
    },
    InvalidSchemaTransitionSourceBasis {
        declared_schema_id: crate::schema::data::SchemaId,
        declared_schema_version: SchemaVersionId,
        expected_schema_id: crate::schema::data::SchemaId,
        expected_schema_version: SchemaVersionId,
    },
    InvalidSchemaTransitionTargetBasis {
        declared_schema_id: crate::schema::data::SchemaId,
        declared_schema_version: SchemaVersionId,
        expected_schema_id: crate::schema::data::SchemaId,
        expected_schema_version: SchemaVersionId,
    },
    MissingSchemaBasisForTransition {
        role: String,
    },
    UnsupportedBridgeDescriptor {
        detail: String,
    },
    HistoricalReinterpretationViolation {
        detail: String,
    },
    TypeIncompatibleSchemaTransition {
        detail: String,
    },
    StructuralIncompatibleSchemaTransition {
        detail: String,
    },
    DirectionalityMismatchUnderCanonicalReconciliation {
        detail: String,
    },
    InvalidSchemaTransitionShape {
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
            Self::UndeclaredSchemaTransition { .. }
            | Self::DescriptorVersionIncompatibility { .. }
            | Self::InvalidSchemaTransitionSourceBasis { .. }
            | Self::InvalidSchemaTransitionTargetBasis { .. }
            | Self::MissingSchemaBasisForTransition { .. }
            | Self::UnsupportedBridgeDescriptor { .. }
            | Self::HistoricalReinterpretationViolation { .. }
            | Self::TypeIncompatibleSchemaTransition { .. }
            | Self::StructuralIncompatibleSchemaTransition { .. }
            | Self::DirectionalityMismatchUnderCanonicalReconciliation { .. }
            | Self::InvalidSchemaTransitionShape { .. } => {
                DiagnosticCode::SchemaContinuityViolation
            }
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
            | Self::MissingMergeBase { detail }
            | Self::UnsupportedBridgeDescriptor { detail }
            | Self::HistoricalReinterpretationViolation { detail }
            | Self::TypeIncompatibleSchemaTransition { detail }
            | Self::StructuralIncompatibleSchemaTransition { detail }
            | Self::DirectionalityMismatchUnderCanonicalReconciliation { detail } => detail.clone(),
            Self::InvariantViolation { detail, .. } => detail.clone(),
            Self::InvalidSchemaTransitionShape { detail } => detail.clone(),
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
            Self::UndeclaredSchemaTransition {
                previous_schema_version,
                current_schema_version,
                previous_descriptor_semantics_version,
                current_descriptor_semantics_version,
            } => format!(
                "schema continuity violation: branch head schema {:?} cannot continue into {:?} without an explicit schema transition (descriptor semantics {:?} -> {:?})",
                previous_schema_version,
                current_schema_version,
                previous_descriptor_semantics_version,
                current_descriptor_semantics_version
            ),
            Self::DescriptorVersionIncompatibility {
                previous_descriptor_semantics_version,
                current_descriptor_semantics_version,
            } => format!(
                "descriptor semantics version mismatch: branch head uses {:?} but runtime requires {:?}",
                previous_descriptor_semantics_version,
                current_descriptor_semantics_version
            ),
            Self::InvalidSchemaTransitionSourceBasis {
                declared_schema_id,
                declared_schema_version,
                expected_schema_id,
                expected_schema_version,
            } => format!(
                "declared schema transition source {:?}/{:?} does not match authoritative prior schema basis {:?}/{:?}",
                declared_schema_id,
                declared_schema_version,
                expected_schema_id,
                expected_schema_version
            ),
            Self::InvalidSchemaTransitionTargetBasis {
                declared_schema_id,
                declared_schema_version,
                expected_schema_id,
                expected_schema_version,
            } => format!(
                "declared schema transition target {:?}/{:?} does not match authoritative runtime schema basis {:?}/{:?}",
                declared_schema_id,
                declared_schema_version,
                expected_schema_id,
                expected_schema_version
            ),
            Self::MissingSchemaBasisForTransition { role } => {
                format!("declared schema transition requires a non-empty {role} schema basis")
            }
        }
    }

    pub fn fields(&self) -> Option<Value> {
        match self {
            Self::InvariantViolation { fields, .. } => Some(fields.clone()),
            Self::UndeclaredSchemaTransition {
                previous_schema_version,
                current_schema_version,
                previous_descriptor_semantics_version,
                current_descriptor_semantics_version,
            } => Some(serde_json::json!({
                "previous_schema_version": previous_schema_version.0,
                "current_schema_version": current_schema_version.0,
                "previous_descriptor_semantics_version": previous_descriptor_semantics_version.0,
                "current_descriptor_semantics_version": current_descriptor_semantics_version.0,
            })),
            Self::DescriptorVersionIncompatibility {
                previous_descriptor_semantics_version,
                current_descriptor_semantics_version,
            } => Some(serde_json::json!({
                "previous_descriptor_semantics_version": previous_descriptor_semantics_version.0,
                "current_descriptor_semantics_version": current_descriptor_semantics_version.0,
            })),
            Self::InvalidSchemaTransitionShape { detail } => Some(serde_json::json!({
                "detail": detail,
            })),
            Self::InvalidSchemaTransitionSourceBasis {
                declared_schema_id,
                declared_schema_version,
                expected_schema_id,
                expected_schema_version,
            } => Some(serde_json::json!({
                "declared_schema_id": declared_schema_id.0,
                "declared_schema_version": declared_schema_version.0,
                "expected_schema_id": expected_schema_id.0,
                "expected_schema_version": expected_schema_version.0,
                "role": "source",
            })),
            Self::InvalidSchemaTransitionTargetBasis {
                declared_schema_id,
                declared_schema_version,
                expected_schema_id,
                expected_schema_version,
            } => Some(serde_json::json!({
                "declared_schema_id": declared_schema_id.0,
                "declared_schema_version": declared_schema_version.0,
                "expected_schema_id": expected_schema_id.0,
                "expected_schema_version": expected_schema_version.0,
                "role": "target",
            })),
            Self::MissingSchemaBasisForTransition { role } => Some(serde_json::json!({
                "role": role,
            })),
            Self::UnsupportedBridgeDescriptor { detail } => Some(serde_json::json!({
                "detail": detail,
                "class": "unsupported_bridge_descriptor",
            })),
            Self::HistoricalReinterpretationViolation { detail } => Some(serde_json::json!({
                "detail": detail,
                "class": "historical_reinterpretation_violation",
            })),
            Self::TypeIncompatibleSchemaTransition { detail } => Some(serde_json::json!({
                "detail": detail,
                "class": "type_incompatible_schema_transition",
            })),
            Self::StructuralIncompatibleSchemaTransition { detail } => Some(serde_json::json!({
                "detail": detail,
                "class": "structural_incompatible_schema_transition",
            })),
            Self::DirectionalityMismatchUnderCanonicalReconciliation { detail } => Some(serde_json::json!({
                "detail": detail,
                "class": "directionality_mismatch_under_canonical_reconciliation",
            })),
            _ => None,
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
    pub aspect_evaluation_traces: Vec<AspectEvaluationTrace>,
    pub aspect_emission_traces: Vec<AspectEmissionTrace>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitStructuralSummary {
    pub invariant_groups: InvariantGroupSet,
    pub commit_topology: super::CommitTopology,
    pub touched_partitions: Vec<crate::identity::data::PartitionId>,
    pub bulk_entity_slots_reserved: usize,
    pub bulk_relation_slots_reserved: usize,
    pub descriptor_semantics_version: DescriptorSemanticsVersion,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitSchemaSummary {
    pub transition: Option<SchemaTransitionSummary>,
    pub descriptor_semantics_version: DescriptorSemanticsVersion,
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
    pub schema_summary: CommitSchemaSummary,
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

    pub fn aspect_summary(&self) -> Option<&CommitAspectSummary> {
        self.commit_summary().aspect_summary.as_ref()
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

    pub fn aspect_summary(&self) -> Option<&CommitAspectSummary> {
        self.summary.aspect_summary.as_ref()
    }

    pub fn validation(&self) -> &CommitValidation {
        &self.validation
    }

    pub fn schema_summary(&self) -> &CommitSchemaSummary {
        &self.schema_summary
    }

    pub fn execution(&self) -> &CommitExecution {
        &self.execution
    }

    pub fn diagnostics(&self) -> &[RelationalDiagnosticArtifact] {
        &self.publication.diagnostics
    }

    pub fn aspect_evaluation_traces(&self) -> &[AspectEvaluationTrace] {
        &self.publication.aspect_evaluation_traces
    }

    pub fn aspect_emission_traces(&self) -> &[AspectEmissionTrace] {
        &self.publication.aspect_emission_traces
    }

    pub fn patch_vs_truth_delta_report(&self) -> PatchVsTruthDeltaReport {
        let patch = self.patch();
        let traces = self.aspect_emission_traces();
        let records_checked = patch.len().min(traces.len());
        let mut mismatched_targets = Vec::new();
        let mut structural_mismatches = 0;
        let mut aspect_mismatches = 0;
        let mut degraded_precision_mismatches = 0;

        for (record, trace) in patch.iter().zip(traces.iter()) {
            if record.target != trace.target {
                mismatched_targets.push(record.target.clone());
            }
            if record.structural_change != trace.structural_change {
                structural_mismatches += 1;
            }
            if record.aspects != trace.changed_aspects {
                aspect_mismatches += 1;
            }
            if record.contains_degraded_precision != trace.contains_degraded_precision {
                degraded_precision_mismatches += 1;
            }
        }

        PatchVsTruthDeltaReport {
            records_checked,
            exact_match: mismatched_targets.is_empty()
                && structural_mismatches == 0
                && aspect_mismatches == 0
                && degraded_precision_mismatches == 0
                && patch.len() == traces.len(),
            mismatched_targets,
            structural_mismatches,
            aspect_mismatches,
            degraded_precision_mismatches,
        }
    }

    pub fn aspect_tag_accuracy_report(&self) -> AspectTagAccuracyReport {
        let traces = self.aspect_emission_traces();
        AspectTagAccuracyReport {
            records_checked: traces.len(),
            correctly_tagged_records: self
                .patch_vs_truth_delta_report()
                .exact_match
                .then_some(traces.len())
                .unwrap_or_else(|| {
                    traces
                        .iter()
                        .zip(self.patch().iter())
                        .filter(|(trace, record)| {
                            trace.changed_aspects == record.aspects
                                && trace.contains_degraded_precision
                                    == record.contains_degraded_precision
                        })
                        .count()
                }),
            touched_aspects: crate::publication::patch::data::CanonicalAspectSet::new(
                traces
                    .iter()
                    .flat_map(|trace| trace.changed_aspects.iter().cloned()),
            ),
            degraded_precision_record_count: traces
                .iter()
                .filter(|trace| trace.contains_degraded_precision)
                .count(),
        }
    }

    pub fn patch(&self) -> &[PatchRecord] {
        &self.publication.envelope.patch.records
    }

    pub fn envelope(&self) -> &CanonicalCommitEnvelope {
        self.publication.envelope.as_ref()
    }

    pub fn schema_transition_summary(&self) -> Option<&SchemaTransitionSummary> {
        self.schema_summary.transition.as_ref()
    }

    pub fn descriptor_semantics_version(&self) -> DescriptorSemanticsVersion {
        self.schema_summary.descriptor_semantics_version
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
