use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

use super::{
    ForgeQueryGraphCompositionBreadth, ForgeQueryGraphCompositionProgram,
    ForgeQueryGraphCompositionProgramStepKind,
};
use crate::runtime::ForgeQueryWriteCommand;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphCompositionDomainInvariantSummary {
    declared_collections: Vec<String>,
    declared_symbols: Vec<String>,
    target_combination_families: Vec<String>,
    lifecycle_families: Vec<String>,
    program_digest: ForgeQueryEvidenceIdentity,
    breadth_digest: ForgeQueryEvidenceIdentity,
    counter_snapshot: String,
    summary_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryGraphCompositionDomainInvariantSummary {
    pub(crate) fn derive(
        program: &ForgeQueryGraphCompositionProgram,
        breadth: &ForgeQueryGraphCompositionBreadth,
        commands: &[ForgeQueryWriteCommand],
    ) -> Self {
        let mut declared_collections = Vec::new();
        let mut declared_symbols = Vec::new();
        for step in program.steps() {
            if !declared_collections
                .iter()
                .any(|collection| collection == step.declared_collection())
            {
                declared_collections.push(step.declared_collection().to_string());
            }
            if let Some(symbol) = step
                .declared_symbol()
                .filter(|symbol| !declared_symbols.iter().any(|candidate| candidate == symbol))
            {
                declared_symbols.push(symbol.to_string());
            }
        }

        let mut target_combination_families = Vec::new();
        if breadth.symbolic_entity_declaration_count() > 0
            && program.steps().iter().any(|step| {
                matches!(
                    step.kind(),
                    ForgeQueryGraphCompositionProgramStepKind::RelationDeclaration
                        | ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationDeclaration
                ) && commands
                    .get(step.component_index())
                    .is_some_and(command_carries_symbolic_relation_identity_edge)
            })
        {
            target_combination_families
                .push("same_batch_entity_relation_identity_edges".to_string());
        }
        if breadth.symbolic_entity_declaration_count() > 0
            && program.steps().iter().any(|step| {
                matches!(
                    step.kind(),
                    ForgeQueryGraphCompositionProgramStepKind::ExistingTargetFollowupMutation
                        | ForgeQueryGraphCompositionProgramStepKind::ExistingTargetRetarget
                        | ForgeQueryGraphCompositionProgramStepKind::ExistingTargetSupersession
                        | ForgeQueryGraphCompositionProgramStepKind::ExistingTargetRetirement
                        | ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedFollowupMutation
                        | ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetarget
                        | ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedSupersession
                        | ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetirement
                ) && commands
                    .get(step.component_index())
                    .is_some_and(command_carries_mixed_existing_and_symbolic_identity_edge)
            })
        {
            target_combination_families
                .push("mixed_existing_and_symbolic_entity_identity_edges".to_string());
        }

        let mut lifecycle_families = Vec::new();
        for step in program.steps() {
            let family = match step.kind() {
                ForgeQueryGraphCompositionProgramStepKind::SymbolicEntityDeclaration
                | ForgeQueryGraphCompositionProgramStepKind::RelationDeclaration
                | ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationDeclaration => {
                    None
                }
                ForgeQueryGraphCompositionProgramStepKind::SymbolicEntityFollowupMutation => {
                    Some("same_batch_symbolic_entity_followup_mutation")
                }
                ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationFollowupMutation => {
                    Some("same_batch_symbolic_relation_followup_mutation")
                }
                ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationRetirement => {
                    Some("same_batch_symbolic_relation_retirement")
                }
                ForgeQueryGraphCompositionProgramStepKind::ExistingTargetFollowupMutation => {
                    Some("mixed_existing_target_followup_mutation")
                }
                ForgeQueryGraphCompositionProgramStepKind::ExistingTargetRetarget => {
                    Some("mixed_existing_target_retarget")
                }
                ForgeQueryGraphCompositionProgramStepKind::ExistingTargetSupersession => {
                    Some("mixed_existing_target_supersession")
                }
                ForgeQueryGraphCompositionProgramStepKind::ExistingTargetRetirement => {
                    Some("mixed_existing_target_retirement")
                }
                ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedFollowupMutation => {
                    Some("mixed_existing_target_verified_followup_mutation")
                }
                ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetarget => {
                    Some("mixed_existing_target_verified_retarget")
                }
                ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedSupersession => {
                    Some("mixed_existing_target_verified_supersession")
                }
                ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetirement => {
                    Some("mixed_existing_target_verified_retirement")
                }
            };
            if let Some(family) = family.filter(|family| {
                !lifecycle_families
                    .iter()
                    .any(|candidate| candidate == family)
            }) {
                lifecycle_families.push(family.to_string());
            }
        }

        let counter_snapshot = diagnostic_counter_snapshot(&[
            ("components", breadth.component_count()),
            (
                "symbolic_entities",
                breadth.symbolic_entity_declaration_count(),
            ),
            (
                "symbolic_relations",
                breadth.symbolic_relation_declaration_count(),
            ),
            ("declared_collections", declared_collections.len()),
            ("declared_symbols", declared_symbols.len()),
            ("target_combinations", target_combination_families.len()),
            ("lifecycle_families", lifecycle_families.len()),
        ]);
        Self::from_parts(
            declared_collections,
            declared_symbols,
            target_combination_families,
            lifecycle_families,
            program.program_evidence_digest().clone(),
            breadth.breadth_evidence_digest().clone(),
            counter_snapshot,
        )
    }

    pub(crate) fn from_parts(
        declared_collections: Vec<String>,
        declared_symbols: Vec<String>,
        target_combination_families: Vec<String>,
        lifecycle_families: Vec<String>,
        program_digest: ForgeQueryEvidenceIdentity,
        breadth_digest: ForgeQueryEvidenceIdentity,
        counter_snapshot: String,
    ) -> Self {
        let summary_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
                .field_shape(
                    ForgeQueryEvidenceTag::new("role"),
                    "graph-composition-domain-invariant-summary",
                )
                .field_evidence_identity(ForgeQueryEvidenceTag::new("program"), &program_digest)
                .field_evidence_identity(ForgeQueryEvidenceTag::new("breadth"), &breadth_digest)
                .field_usize(
                    ForgeQueryEvidenceTag::new("declared_collection_count"),
                    declared_collections.len(),
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("declared_symbol_count"),
                    declared_symbols.len(),
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("target_combination_count"),
                    target_combination_families.len(),
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("lifecycle_family_count"),
                    lifecycle_families.len(),
                )
                .field_value_sequence(
                    ForgeQueryEvidenceTag::new("declared_collection"),
                    declared_collections.iter().map(String::as_str),
                )
                .field_value_sequence(
                    ForgeQueryEvidenceTag::new("declared_symbol"),
                    declared_symbols.iter().map(String::as_str),
                )
                .field_value_sequence(
                    ForgeQueryEvidenceTag::new("target_combination"),
                    target_combination_families.iter().map(String::as_str),
                )
                .field_value_sequence(
                    ForgeQueryEvidenceTag::new("lifecycle_family"),
                    lifecycle_families.iter().map(String::as_str),
                )
                .seal();
        Self {
            declared_collections,
            declared_symbols,
            target_combination_families,
            lifecycle_families,
            program_digest,
            breadth_digest,
            counter_snapshot,
            summary_digest,
        }
    }

    pub fn declared_collections(&self) -> &[String] {
        &self.declared_collections
    }

    pub fn declared_symbols(&self) -> &[String] {
        &self.declared_symbols
    }

    pub fn target_combination_families(&self) -> &[String] {
        &self.target_combination_families
    }

    pub fn lifecycle_families(&self) -> &[String] {
        &self.lifecycle_families
    }

    pub fn program_digest(&self) -> &str {
        self.program_digest.as_str()
    }

    pub fn breadth_digest(&self) -> &str {
        self.breadth_digest.as_str()
    }

    pub fn counter_snapshot(&self) -> &str {
        &self.counter_snapshot
    }

    pub fn summary_digest(&self) -> &str {
        self.summary_digest.as_str()
    }

    pub fn summary_evidence_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.summary_digest
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

fn command_carries_symbolic_relation_identity_edge(command: &ForgeQueryWriteCommand) -> bool {
    !command.symbolic_aspect_references().is_empty()
}

fn command_carries_mixed_existing_and_symbolic_identity_edge(
    command: &ForgeQueryWriteCommand,
) -> bool {
    command.existing_truth_binding().is_some() && !command.symbolic_aspect_references().is_empty()
}
