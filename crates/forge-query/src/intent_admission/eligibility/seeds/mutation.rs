use crate::identity::hash_parts;
use crate::runtime::{
    command_declared_aspect_value_digest, ForgeQueryContinuityMutationDenial,
    ForgeQueryExistingTruthAssertionDenial, ForgeQueryExistingTruthBindingDenial,
    ForgeQueryGraphCompositionBreadth, ForgeQueryGraphCompositionProgram,
    ForgeQueryNamingMutationDenial, ForgeQuerySymbolicTargetReferenceDenial,
    ForgeQueryVerifiedExistingTruthAssertion, ForgeQueryWriteCommand,
};

#[derive(Clone, Debug, PartialEq)]
pub enum ForgeQueryAuthoritativeMutationPreflight {
    Admitted {
        verified_existing_truth_assertion: Option<ForgeQueryVerifiedExistingTruthAssertion>,
    },
    BindingDenied(ForgeQueryExistingTruthBindingDenial),
    AssertionDenied(ForgeQueryExistingTruthAssertionDenial),
    ContinuityDenied(ForgeQueryContinuityMutationDenial),
    NamingDenied(ForgeQueryNamingMutationDenial),
    TargetReferenceDenied(ForgeQuerySymbolicTargetReferenceDenial),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryAuthoritativeMutationIntentSeed {
    command: ForgeQueryWriteCommand,
    preflight: ForgeQueryAuthoritativeMutationPreflight,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryAuthoritativeMutationBatchIntentSeed {
    commands: Vec<ForgeQueryWriteCommand>,
    graph_composition_breadth: ForgeQueryGraphCompositionBreadth,
    graph_composition_program: ForgeQueryGraphCompositionProgram,
}

impl ForgeQueryAuthoritativeMutationIntentSeed {
    pub fn new(
        command: ForgeQueryWriteCommand,
        preflight: ForgeQueryAuthoritativeMutationPreflight,
    ) -> Self {
        Self { command, preflight }
    }

    pub fn command(&self) -> &ForgeQueryWriteCommand {
        &self.command
    }

    pub fn preflight(&self) -> &ForgeQueryAuthoritativeMutationPreflight {
        &self.preflight
    }

    pub fn verified_existing_truth_assertion(
        &self,
    ) -> Option<&ForgeQueryVerifiedExistingTruthAssertion> {
        match &self.preflight {
            ForgeQueryAuthoritativeMutationPreflight::Admitted {
                verified_existing_truth_assertion,
            } => verified_existing_truth_assertion.as_ref(),
            ForgeQueryAuthoritativeMutationPreflight::BindingDenied(_)
            | ForgeQueryAuthoritativeMutationPreflight::AssertionDenied(_)
            | ForgeQueryAuthoritativeMutationPreflight::ContinuityDenied(_)
            | ForgeQueryAuthoritativeMutationPreflight::NamingDenied(_)
            | ForgeQueryAuthoritativeMutationPreflight::TargetReferenceDenied(_) => None,
        }
    }

    pub fn command_label(&self) -> String {
        mutation_intent_name(&self.command)
    }

    pub fn command_input_digest(&self) -> String {
        mutation_input_digest(&self.command)
    }
}

impl ForgeQueryAuthoritativeMutationBatchIntentSeed {
    pub fn new(
        commands: Vec<ForgeQueryWriteCommand>,
        graph_composition_breadth: ForgeQueryGraphCompositionBreadth,
        graph_composition_program: ForgeQueryGraphCompositionProgram,
    ) -> Self {
        Self {
            commands,
            graph_composition_breadth,
            graph_composition_program,
        }
    }

    pub fn commands(&self) -> &[ForgeQueryWriteCommand] {
        &self.commands
    }

    pub fn graph_composition_breadth(&self) -> &ForgeQueryGraphCompositionBreadth {
        &self.graph_composition_breadth
    }

    pub fn graph_composition_program(&self) -> &ForgeQueryGraphCompositionProgram {
        &self.graph_composition_program
    }

    pub fn into_parts(
        self,
    ) -> (
        Vec<ForgeQueryWriteCommand>,
        ForgeQueryGraphCompositionBreadth,
        ForgeQueryGraphCompositionProgram,
    ) {
        (
            self.commands,
            self.graph_composition_breadth,
            self.graph_composition_program,
        )
    }

    pub fn batch_label(&self) -> String {
        format!("mutation.batch.{}", self.commands.len())
    }

    pub fn batch_input_digest(&self) -> String {
        hash_parts(
            &std::iter::once("forge_query_authoritative_mutation_batch_input_v1".to_string())
                .chain(
                    self.commands
                        .iter()
                        .map(|command| format!("command:{}", mutation_input_digest(command))),
                )
                .chain(std::iter::once(format!(
                    "graph-breadth:{}",
                    self.graph_composition_breadth.breadth_digest()
                )))
                .chain(std::iter::once(format!(
                    "graph-program:{}",
                    self.graph_composition_program.program_digest()
                )))
                .collect::<Vec<_>>(),
        )
    }
}

fn mutation_intent_name(command: &ForgeQueryWriteCommand) -> String {
    let family = command.mutation_family().as_str();
    let target = command
        .declared_entity_identity_ref()
        .or_else(|| command.declared_collection_ref())
        .unwrap_or("unspecified-target");
    format!("mutation.{family}.{target}")
}

fn mutation_input_digest(command: &ForgeQueryWriteCommand) -> String {
    let operations = command
        .declared_aspect_operations()
        .into_iter()
        .map(|operation| format!("{}:{}", operation.kind().as_str(), operation.aspect_path()))
        .collect::<Vec<_>>();
    let declared_collection = command.declared_collection_ref().unwrap_or("none");
    let declared_entity_identity = command.declared_entity_identity_ref().unwrap_or("none");
    let existing_truth_binding = command
        .existing_truth_binding()
        .map(|binding| binding.binding_digest().to_string())
        .unwrap_or_else(|| "none".to_string());
    let symbolic_target = command
        .symbolic_target_reference()
        .map(|reference| {
            format!(
                "{}:{}",
                reference.symbol(),
                reference.target_collection().unwrap_or("none")
            )
        })
        .unwrap_or_else(|| "none".to_string());
    let symbolic_aspects = command
        .symbolic_aspect_references()
        .iter()
        .map(|reference| {
            format!(
                "{}:{}:{}",
                reference.aspect_path(),
                reference.family(),
                reference.reference().symbol()
            )
        })
        .collect::<Vec<_>>();
    hash_parts(&[
        "forge_query_authoritative_mutation_intent_input_v1".to_string(),
        format!("family:{}", command.mutation_family().as_str()),
        format!("collection:{declared_collection}"),
        format!("entity:{declared_entity_identity}"),
        format!("binding:{existing_truth_binding}"),
        format!("operations:{}", operations.join("|")),
        format!(
            "aspect-values:{}",
            command_declared_aspect_value_digest(command).unwrap_or_else(|| "none".to_string())
        ),
        format!("symbolic-target:{symbolic_target}"),
        format!("symbolic-aspects:{}", symbolic_aspects.join("|")),
    ])
}
