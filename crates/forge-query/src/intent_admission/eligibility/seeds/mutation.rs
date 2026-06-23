use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::runtime::ForgeQueryAspectMutationOperation;
use crate::runtime::{
    command_declared_aspect_value_digest, ForgeQueryContinuityMutationDenial,
    ForgeQueryExistingTruthAssertionDenial, ForgeQueryExistingTruthBindingDenial,
    ForgeQueryGraphCompositionBreadth, ForgeQueryGraphCompositionProgram,
    ForgeQueryGraphTouchDescriptor, ForgeQueryGraphTouchDescriptorDenial,
    ForgeQueryMutationSymbolIdentity, ForgeQueryMutationTargetCollectionIdentity,
    ForgeQueryNamingMutationDenial, ForgeQuerySymbolicAspectReference,
    ForgeQuerySymbolicTargetReference, ForgeQuerySymbolicTargetReferenceDenial,
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

    pub fn graph_touch_descriptor(
        &self,
    ) -> Result<ForgeQueryGraphTouchDescriptor, ForgeQueryGraphTouchDescriptorDenial> {
        ForgeQueryGraphTouchDescriptor::from_authoritative_mutation_batch(
            &self.graph_composition_program,
            &self.graph_composition_breadth,
            &self.commands,
        )
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
        let command_identities = self
            .commands
            .iter()
            .map(mutation_input_identity)
            .collect::<Vec<_>>();
        forge_query_evidence_identity(ForgeQueryEvidenceScope::AuthoritativeMutationBatchIntentSeed)
            .field_evidence_identity_sequence(
                ForgeQueryEvidenceTag::new("commands"),
                command_identities.iter(),
            )
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("graph_breadth"),
                self.graph_composition_breadth.breadth_evidence_digest(),
            )
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("graph_program"),
                self.graph_composition_program.program_evidence_digest(),
            )
            .seal()
            .as_str()
            .to_string()
    }
}

fn mutation_intent_name(command: &ForgeQueryWriteCommand) -> String {
    let family = command.mutation_family().as_str();
    let target = mutation_target_label(command);
    format!("mutation.{family}.{target}")
}

fn mutation_input_digest(command: &ForgeQueryWriteCommand) -> String {
    mutation_input_identity(command).as_str().to_string()
}

fn mutation_input_identity(command: &ForgeQueryWriteCommand) -> ForgeQueryEvidenceIdentity {
    let declared_entity_identity = command
        .declared_entity_identity_ref()
        .map(|identity| identity.evidence_identity());
    let declared_collection_identity = command.declared_collection_identity();
    let aspect_value_identity = command_declared_aspect_value_digest(command).map(|digest| {
        forge_query_evidence_identity(ForgeQueryEvidenceScope::AuthoritativeMutationIntentSeed)
            .field_shape(ForgeQueryEvidenceTag::new("role"), "declared-aspect-values")
            .field_value(ForgeQueryEvidenceTag::new("digest"), digest)
            .seal()
    });
    let symbolic_target = command
        .symbolic_target_reference()
        .map(symbolic_target_reference_identity);
    let symbolic_aspects = command
        .symbolic_aspect_references()
        .iter()
        .map(symbolic_aspect_reference_identity)
        .collect::<Vec<_>>();
    let operation_identities = command
        .declared_aspect_operations()
        .into_iter()
        .map(declared_aspect_operation_identity)
        .collect::<Vec<_>>();
    let mut identity =
        forge_query_evidence_identity(ForgeQueryEvidenceScope::AuthoritativeMutationIntentSeed)
            .field_shape(
                ForgeQueryEvidenceTag::new("family"),
                command.mutation_family().as_str(),
            )
            .optional_evidence_identity(
                ForgeQueryEvidenceTag::new("collection"),
                declared_collection_identity
                    .as_ref()
                    .map(ForgeQueryMutationTargetCollectionIdentity::evidence_identity),
            )
            .optional_evidence_identity(
                ForgeQueryEvidenceTag::new("declared_entity_identity"),
                declared_entity_identity.as_ref(),
            )
            .optional_evidence_identity(
                ForgeQueryEvidenceTag::new("existing_truth_binding"),
                command
                    .existing_truth_binding()
                    .map(|binding| binding.binding_evidence_identity()),
            )
            .field_evidence_identity_sequence(
                ForgeQueryEvidenceTag::new("operations"),
                operation_identities.iter(),
            )
            .optional_evidence_identity(
                ForgeQueryEvidenceTag::new("aspect_values"),
                aspect_value_identity.as_ref(),
            )
            .optional_evidence_identity(
                ForgeQueryEvidenceTag::new("symbolic_target"),
                symbolic_target.as_ref(),
            )
            .field_evidence_identity_sequence(
                ForgeQueryEvidenceTag::new("symbolic_aspects"),
                symbolic_aspects.iter(),
            );
    if let Some(identity_evidence) = declared_entity_identity {
        identity = identity.field_evidence_identity(
            ForgeQueryEvidenceTag::new("target_label_identity"),
            &identity_evidence,
        );
    }
    identity.seal()
}

fn declared_aspect_operation_identity(
    operation: ForgeQueryAspectMutationOperation,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::AuthoritativeMutationIntentSeed)
        .field_shape(
            ForgeQueryEvidenceTag::new("role"),
            "declared-aspect-operation",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("kind"),
            operation.kind().as_str(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("admitted_aspect_touch"),
            operation.aspect_touch().admitted_touch_digest_part(),
        )
        .seal()
}

fn symbolic_target_reference_identity(
    reference: &ForgeQuerySymbolicTargetReference,
) -> ForgeQueryEvidenceIdentity {
    let symbol_identity =
        ForgeQueryMutationSymbolIdentity::new("authoritative-mutation-seed", reference.symbol());
    let collection_identity = reference.target_collection().map(|collection| {
        ForgeQueryMutationTargetCollectionIdentity::new(
            "authoritative-mutation-seed-symbolic-target",
            collection,
        )
    });
    forge_query_evidence_identity(ForgeQueryEvidenceScope::AuthoritativeMutationIntentSeed)
        .field_shape(ForgeQueryEvidenceTag::new("role"), "symbolic-target")
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("symbol"),
            symbol_identity.evidence_identity(),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("collection"),
            collection_identity
                .as_ref()
                .map(ForgeQueryMutationTargetCollectionIdentity::evidence_identity),
        )
        .seal()
}

fn symbolic_aspect_reference_identity(
    reference: &ForgeQuerySymbolicAspectReference,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::AuthoritativeMutationIntentSeed)
        .field_shape(ForgeQueryEvidenceTag::new("role"), "symbolic-aspect")
        .field_value(
            ForgeQueryEvidenceTag::new("admitted_aspect_touch"),
            reference.aspect_touch().admitted_touch_digest_part(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("family"),
            reference.family().as_str(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("target"),
            &symbolic_target_reference_identity(reference.reference()),
        )
        .seal()
}

fn mutation_target_label(command: &ForgeQueryWriteCommand) -> String {
    command
        .declared_entity_identity_ref()
        .map(|identity| {
            identity
                .evidence_identity()
                .reporting_projection()
                .to_string()
        })
        .or_else(|| {
            command
                .declared_collection_identity()
                .map(|collection| collection.as_str().to_string())
        })
        .unwrap_or_else(|| "unspecified-target".to_string())
}
