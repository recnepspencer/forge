use super::ForgeQueryGraphCompositionProgram;
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::runtime::ForgeQueryGraphCompositionProgramStepKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryGraphCompositionLifecycleOutcomeKind {
    Created,
    UpdatedIdentityPreserved,
    RetargetedIdentityPreserved,
    RetiredCurrentTruth,
    SupersededWithLineage,
    DeletedIfUncommitted,
    DeniedBeforeExecution,
}

impl ForgeQueryGraphCompositionLifecycleOutcomeKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::UpdatedIdentityPreserved => "updated-identity-preserved",
            Self::RetargetedIdentityPreserved => "retargeted-identity-preserved",
            Self::RetiredCurrentTruth => "retired-current-truth",
            Self::SupersededWithLineage => "superseded-with-lineage",
            Self::DeletedIfUncommitted => "deleted-if-uncommitted",
            Self::DeniedBeforeExecution => "denied-before-execution",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphCompositionLifecycleOutcomeEntry {
    component_index: usize,
    outcome_kind: ForgeQueryGraphCompositionLifecycleOutcomeKind,
    declared_collection: String,
    declared_symbol: Option<String>,
}

impl ForgeQueryGraphCompositionLifecycleOutcomeEntry {
    fn new(
        component_index: usize,
        outcome_kind: ForgeQueryGraphCompositionLifecycleOutcomeKind,
        declared_collection: impl Into<String>,
        declared_symbol: Option<String>,
    ) -> Self {
        Self {
            component_index,
            outcome_kind,
            declared_collection: declared_collection.into(),
            declared_symbol,
        }
    }

    pub fn component_index(&self) -> usize {
        self.component_index
    }

    pub fn outcome_kind(&self) -> ForgeQueryGraphCompositionLifecycleOutcomeKind {
        self.outcome_kind
    }

    pub fn declared_collection(&self) -> &str {
        &self.declared_collection
    }

    pub fn declared_symbol(&self) -> Option<&str> {
        self.declared_symbol.as_deref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphCompositionLifecycleOutcomes {
    entries: Vec<ForgeQueryGraphCompositionLifecycleOutcomeEntry>,
    lifecycle_digest: ForgeQueryEvidenceIdentity,
    counter_snapshot: String,
}

impl ForgeQueryGraphCompositionLifecycleOutcomes {
    pub(crate) fn derive(program: &ForgeQueryGraphCompositionProgram) -> Option<Self> {
        if program.is_empty() {
            return None;
        }

        let entries = program
            .steps()
            .iter()
            .map(|step| {
                let outcome_kind = match step.kind() {
                    ForgeQueryGraphCompositionProgramStepKind::SymbolicEntityDeclaration
                    | ForgeQueryGraphCompositionProgramStepKind::RelationDeclaration
                    | ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationDeclaration => {
                        ForgeQueryGraphCompositionLifecycleOutcomeKind::Created
                    }
                    ForgeQueryGraphCompositionProgramStepKind::SymbolicEntityFollowupMutation
                    | ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationFollowupMutation => {
                        ForgeQueryGraphCompositionLifecycleOutcomeKind::UpdatedIdentityPreserved
                    }
                    ForgeQueryGraphCompositionProgramStepKind::ExistingTargetFollowupMutation
                    | ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedFollowupMutation => {
                        ForgeQueryGraphCompositionLifecycleOutcomeKind::UpdatedIdentityPreserved
                    }
                    ForgeQueryGraphCompositionProgramStepKind::ExistingTargetRetarget
                    | ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetarget => {
                        ForgeQueryGraphCompositionLifecycleOutcomeKind::RetargetedIdentityPreserved
                    }
                    ForgeQueryGraphCompositionProgramStepKind::ExistingTargetSupersession
                    | ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedSupersession => {
                        ForgeQueryGraphCompositionLifecycleOutcomeKind::SupersededWithLineage
                    }
                    ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationRetirement
                    | ForgeQueryGraphCompositionProgramStepKind::ExistingTargetRetirement
                    | ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetirement => {
                        ForgeQueryGraphCompositionLifecycleOutcomeKind::RetiredCurrentTruth
                    }
                };
                ForgeQueryGraphCompositionLifecycleOutcomeEntry::new(
                    step.component_index(),
                    outcome_kind,
                    step.declared_collection(),
                    step.declared_symbol().map(str::to_string),
                )
            })
            .collect::<Vec<_>>();

        let created_count = entries
            .iter()
            .filter(|entry| {
                entry.outcome_kind() == ForgeQueryGraphCompositionLifecycleOutcomeKind::Created
            })
            .count();
        let updated_identity_preserved_count = entries
            .iter()
            .filter(|entry| {
                entry.outcome_kind()
                    == ForgeQueryGraphCompositionLifecycleOutcomeKind::UpdatedIdentityPreserved
            })
            .count();
        let retargeted_identity_preserved_count = entries
            .iter()
            .filter(|entry| {
                entry.outcome_kind()
                    == ForgeQueryGraphCompositionLifecycleOutcomeKind::RetargetedIdentityPreserved
            })
            .count();
        let retired_current_truth_count = entries
            .iter()
            .filter(|entry| {
                entry.outcome_kind()
                    == ForgeQueryGraphCompositionLifecycleOutcomeKind::RetiredCurrentTruth
            })
            .count();
        let superseded_with_lineage_count = entries
            .iter()
            .filter(|entry| {
                entry.outcome_kind()
                    == ForgeQueryGraphCompositionLifecycleOutcomeKind::SupersededWithLineage
            })
            .count();
        let deleted_if_uncommitted_count = entries
            .iter()
            .filter(|entry| {
                entry.outcome_kind()
                    == ForgeQueryGraphCompositionLifecycleOutcomeKind::DeletedIfUncommitted
            })
            .count();
        let denied_before_execution_count = entries
            .iter()
            .filter(|entry| {
                entry.outcome_kind()
                    == ForgeQueryGraphCompositionLifecycleOutcomeKind::DeniedBeforeExecution
            })
            .count();
        let counter_snapshot = diagnostic_counter_snapshot(&[
            ("created", created_count),
            (
                "updated_identity_preserved",
                updated_identity_preserved_count,
            ),
            (
                "retargeted_identity_preserved",
                retargeted_identity_preserved_count,
            ),
            ("retired_current_truth", retired_current_truth_count),
            ("superseded_with_lineage", superseded_with_lineage_count),
            ("deleted_if_uncommitted", deleted_if_uncommitted_count),
            ("denied_before_execution", denied_before_execution_count),
        ]);
        let lifecycle_entry_digests = entries
            .iter()
            .map(|entry| {
                forge_query_evidence_identity(
                    ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest,
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("role"),
                    "graph-composition-lifecycle-entry",
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("component"),
                    entry.component_index(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("outcome"),
                    entry.outcome_kind().as_str(),
                )
                .field_value(
                    ForgeQueryEvidenceTag::new("declared_collection"),
                    entry.declared_collection(),
                )
                .optional_value(
                    ForgeQueryEvidenceTag::new("declared_symbol"),
                    entry.declared_symbol(),
                )
                .seal()
            })
            .collect::<Vec<_>>();
        let lifecycle_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
                .field_shape(
                    ForgeQueryEvidenceTag::new("role"),
                    "graph-composition-lifecycle-outcomes",
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("program"),
                    program.program_evidence_digest(),
                )
                .field_usize(ForgeQueryEvidenceTag::new("created_count"), created_count)
                .field_usize(
                    ForgeQueryEvidenceTag::new("updated_identity_preserved_count"),
                    updated_identity_preserved_count,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("retargeted_identity_preserved_count"),
                    retargeted_identity_preserved_count,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("retired_current_truth_count"),
                    retired_current_truth_count,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("superseded_with_lineage_count"),
                    superseded_with_lineage_count,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("deleted_if_uncommitted_count"),
                    deleted_if_uncommitted_count,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("denied_before_execution_count"),
                    denied_before_execution_count,
                )
                .field_evidence_identity_sequence(
                    ForgeQueryEvidenceTag::new("entry"),
                    lifecycle_entry_digests.iter(),
                )
                .seal();

        Some(Self {
            entries,
            lifecycle_digest,
            counter_snapshot,
        })
    }

    pub fn entries(&self) -> &[ForgeQueryGraphCompositionLifecycleOutcomeEntry] {
        &self.entries
    }

    pub fn lifecycle_digest(&self) -> &str {
        self.lifecycle_digest.as_str()
    }

    pub fn lifecycle_evidence_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.lifecycle_digest
    }

    pub fn counter_snapshot(&self) -> &str {
        &self.counter_snapshot
    }
}

fn diagnostic_counter_snapshot(fields: &[(&str, usize)]) -> String {
    let mut snapshot = String::new();
    for (index, (label, value)) in fields.iter().enumerate() {
        if index > 0 {
            snapshot.push(';');
        }
        snapshot.push_str(label);
        snapshot.push('=');
        snapshot.push_str(&value.to_string());
    }
    snapshot
}
