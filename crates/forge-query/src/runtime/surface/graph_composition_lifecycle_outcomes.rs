use crate::identity::hash_parts;

use super::ForgeQueryGraphCompositionProgram;
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
    lifecycle_digest: String,
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

        let counter_snapshot = format!(
            "created={};updated_identity_preserved={};retargeted_identity_preserved={};retired_current_truth={};superseded_with_lineage={};deleted_if_uncommitted={};denied_before_execution={}",
            entries.iter().filter(|entry| entry.outcome_kind() == ForgeQueryGraphCompositionLifecycleOutcomeKind::Created).count(),
            entries.iter().filter(|entry| entry.outcome_kind() == ForgeQueryGraphCompositionLifecycleOutcomeKind::UpdatedIdentityPreserved).count(),
            entries.iter().filter(|entry| entry.outcome_kind() == ForgeQueryGraphCompositionLifecycleOutcomeKind::RetargetedIdentityPreserved).count(),
            entries.iter().filter(|entry| entry.outcome_kind() == ForgeQueryGraphCompositionLifecycleOutcomeKind::RetiredCurrentTruth).count(),
            entries.iter().filter(|entry| entry.outcome_kind() == ForgeQueryGraphCompositionLifecycleOutcomeKind::SupersededWithLineage).count(),
            entries.iter().filter(|entry| entry.outcome_kind() == ForgeQueryGraphCompositionLifecycleOutcomeKind::DeletedIfUncommitted).count(),
            entries.iter().filter(|entry| entry.outcome_kind() == ForgeQueryGraphCompositionLifecycleOutcomeKind::DeniedBeforeExecution).count(),
        );
        let lifecycle_digest = hash_parts(
            &std::iter::once("forge_query_graph_composition_lifecycle_outcomes_v1".to_string())
                .chain(std::iter::once(format!(
                    "program:{}",
                    program.program_digest()
                )))
                .chain(std::iter::once(format!("counters:{counter_snapshot}")))
                .chain(entries.iter().map(|entry| {
                    format!(
                        "{}:{}:{}:{}",
                        entry.component_index(),
                        entry.outcome_kind().as_str(),
                        entry.declared_collection(),
                        entry.declared_symbol().unwrap_or("none")
                    )
                }))
                .collect::<Vec<_>>(),
        );

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
        &self.lifecycle_digest
    }

    pub fn counter_snapshot(&self) -> &str {
        &self.counter_snapshot
    }
}
