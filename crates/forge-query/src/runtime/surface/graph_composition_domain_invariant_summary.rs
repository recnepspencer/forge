use crate::identity::hash_parts;

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
    program_digest: String,
    breadth_digest: String,
    counter_snapshot: String,
    summary_digest: String,
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

        let counter_snapshot = format!(
            "components={};symbolic_entities={};symbolic_relations={};declared_collections={};declared_symbols={};target_combinations={};lifecycle_families={}",
            breadth.component_count(),
            breadth.symbolic_entity_declaration_count(),
            breadth.symbolic_relation_declaration_count(),
            declared_collections.len(),
            declared_symbols.len(),
            target_combination_families.len(),
            lifecycle_families.len(),
        );
        Self::from_parts(
            declared_collections,
            declared_symbols,
            target_combination_families,
            lifecycle_families,
            program.program_digest().to_string(),
            breadth.breadth_digest().to_string(),
            counter_snapshot,
        )
    }

    pub(crate) fn from_parts(
        declared_collections: Vec<String>,
        declared_symbols: Vec<String>,
        target_combination_families: Vec<String>,
        lifecycle_families: Vec<String>,
        program_digest: String,
        breadth_digest: String,
        counter_snapshot: String,
    ) -> Self {
        let summary_digest = hash_parts(&[
            "forge_query_graph_composition_domain_invariant_summary_v1".to_string(),
            format!("program:{program_digest}"),
            format!("breadth:{breadth_digest}"),
            format!("counters:{counter_snapshot}"),
            format!("collections:{}", declared_collections.join(",")),
            format!("symbols:{}", declared_symbols.join(",")),
            format!("targets:{}", target_combination_families.join(",")),
            format!("lifecycles:{}", lifecycle_families.join(",")),
        ]);
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
        &self.program_digest
    }

    pub fn breadth_digest(&self) -> &str {
        &self.breadth_digest
    }

    pub fn counter_snapshot(&self) -> &str {
        &self.counter_snapshot
    }

    pub fn summary_digest(&self) -> &str {
        &self.summary_digest
    }
}

fn command_carries_symbolic_relation_identity_edge(command: &ForgeQueryWriteCommand) -> bool {
    !command.symbolic_aspect_references().is_empty()
}

fn command_carries_mixed_existing_and_symbolic_identity_edge(
    command: &ForgeQueryWriteCommand,
) -> bool {
    command.existing_truth_binding().is_some() && !command.symbolic_aspect_references().is_empty()
}
