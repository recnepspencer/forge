use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::diagnostics::data::DiagnosticCode;
use crate::diagnostics::data::RelationalDiagnosticArtifact;
use crate::errors::data::{ErrorContext, ErrorOperation, RelationalSubsystem, SuggestedFix};
use crate::identity::data::{EntityId, RelationId, VersionId};
use crate::performance::data::RuntimeComplexityCounters;
use crate::publication::data::diff::PatchRecord;
use crate::publication::data::{PublicationError, PublicationStatus};
use crate::replay::data::CanonicalCommitEnvelope;
use crate::snapshots::data::SnapshotHandle;
use crate::validation::data::InvariantCheckResult;

use super::{
    CommitLog, ExistingRecordTarget, MutationIntent, RecordRef, SavepointId, TransactionId,
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
            context: ErrorContext::new(
                RelationalSubsystem::Transaction,
                ErrorOperation::Validate,
            )
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
                    format!("conflicting entity intent for slot {}", entity_id.local_slot.0)
                }
                ExistingRecordTarget::Relation(relation_id) => {
                    format!("conflicting relation intent for slot {}", relation_id.local_slot.0)
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
    pub draft_preparation_micros: u64,
    pub invariant_pre_check_micros: u64,
    pub authoritative_mutation_micros: u64,
    pub history_resolution_micros: u64,
    pub invariant_post_check_micros: u64,
    pub artifact_assembly_micros: u64,
    pub durable_append_micros: u64,
    pub publication_micros: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitResult {
    pub outcome: CommitOutcome,
    pub diagnostics: Vec<RelationalDiagnosticArtifact>,
    pub patch: Vec<PatchRecord>,
    pub envelope: CanonicalCommitEnvelope,
    pub phase_timing: CommitPhaseTiming,
    pub invariant_results: Vec<InvariantCheckResult>,
    pub complexity_delta: RuntimeComplexityCounters,
}

impl Deref for CommitResult {
    type Target = CommitOutcome;

    fn deref(&self) -> &Self::Target {
        &self.outcome
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RollbackEffect {
    RestoredEntity(EntityId),
    RestoredRelation(RelationId),
    DiscardedEntityCreation,
    DiscardedRelationCreation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackOutcome {
    pub transaction_id: TransactionId,
    pub effects: Vec<RollbackEffect>,
}
