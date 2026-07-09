use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};
use crate::runtime::WorthQueryAspectMutationOperation;
use crate::runtime::{
    command_declared_aspect_value_digest, WorthQueryContinuityMutationDenial,
    WorthQueryExistingTruthAssertionDenial, WorthQueryExistingTruthBindingDenial,
    WorthQueryGraphCompositionBreadth, WorthQueryGraphCompositionProgram,
    WorthQueryGraphTouchDescriptor, WorthQueryGraphTouchDescriptorDenial,
    WorthQueryMutationSymbolIdentity, WorthQueryMutationTargetCollectionIdentity,
    WorthQueryNamingMutationDenial, WorthQuerySymbolicAspectReference,
    WorthQuerySymbolicTargetReference, WorthQuerySymbolicTargetReferenceDenial,
    WorthQueryVerifiedExistingTruthAssertion, WorthQueryWriteCommand,
};

#[derive(Clone, Debug, PartialEq)]
pub enum WorthQueryAuthoritativeMutationPreflight {
    Admitted {
        verified_existing_truth_assertion: Option<WorthQueryVerifiedExistingTruthAssertion>,
    },
    BindingDenied(WorthQueryExistingTruthBindingDenial),
    AssertionDenied(WorthQueryExistingTruthAssertionDenial),
    ContinuityDenied(WorthQueryContinuityMutationDenial),
    NamingDenied(WorthQueryNamingMutationDenial),
    TargetReferenceDenied(WorthQuerySymbolicTargetReferenceDenial),
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryAuthoritativeMutationIntentSeed {
    command: WorthQueryWriteCommand,
    preflight: WorthQueryAuthoritativeMutationPreflight,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryAuthoritativeMutationBatchIntentSeed {
    commands: Vec<WorthQueryWriteCommand>,
    graph_composition_breadth: WorthQueryGraphCompositionBreadth,
    graph_composition_program: WorthQueryGraphCompositionProgram,
}

impl WorthQueryAuthoritativeMutationIntentSeed {
    pub fn new(
        command: WorthQueryWriteCommand,
        preflight: WorthQueryAuthoritativeMutationPreflight,
    ) -> Self {
        Self { command, preflight }
    }

    pub fn command(&self) -> &WorthQueryWriteCommand {
        &self.command
    }

    pub fn preflight(&self) -> &WorthQueryAuthoritativeMutationPreflight {
        &self.preflight
    }

    pub fn verified_existing_truth_assertion(
        &self,
    ) -> Option<&WorthQueryVerifiedExistingTruthAssertion> {
        match &self.preflight {
            WorthQueryAuthoritativeMutationPreflight::Admitted {
                verified_existing_truth_assertion,
            } => verified_existing_truth_assertion.as_ref(),
            WorthQueryAuthoritativeMutationPreflight::BindingDenied(_)
            | WorthQueryAuthoritativeMutationPreflight::AssertionDenied(_)
            | WorthQueryAuthoritativeMutationPreflight::ContinuityDenied(_)
            | WorthQueryAuthoritativeMutationPreflight::NamingDenied(_)
            | WorthQueryAuthoritativeMutationPreflight::TargetReferenceDenied(_) => None,
        }
    }

    pub fn command_label(&self) -> String {
        mutation_intent_name(&self.command)
    }

    pub fn command_input_digest(&self) -> String {
        mutation_input_digest(&self.command)
    }
}

impl WorthQueryAuthoritativeMutationBatchIntentSeed {
    pub fn new(
        commands: Vec<WorthQueryWriteCommand>,
        graph_composition_breadth: WorthQueryGraphCompositionBreadth,
        graph_composition_program: WorthQueryGraphCompositionProgram,
    ) -> Self {
        Self {
            commands,
            graph_composition_breadth,
            graph_composition_program,
        }
    }

    pub fn commands(&self) -> &[WorthQueryWriteCommand] {
        &self.commands
    }

    pub fn graph_composition_breadth(&self) -> &WorthQueryGraphCompositionBreadth {
        &self.graph_composition_breadth
    }

    pub fn graph_composition_program(&self) -> &WorthQueryGraphCompositionProgram {
        &self.graph_composition_program
    }

    pub fn graph_touch_descriptor(
        &self,
    ) -> Result<WorthQueryGraphTouchDescriptor, WorthQueryGraphTouchDescriptorDenial> {
        WorthQueryGraphTouchDescriptor::from_authoritative_mutation_batch(
            &self.graph_composition_program,
            &self.graph_composition_breadth,
            &self.commands,
        )
    }

    pub fn into_parts(
        self,
    ) -> (
        Vec<WorthQueryWriteCommand>,
        WorthQueryGraphCompositionBreadth,
        WorthQueryGraphCompositionProgram,
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
        worth_query_evidence_identity(WorthQueryEvidenceScope::AuthoritativeMutationBatchIntentSeed)
            .field_evidence_identity_sequence(
                WorthQueryEvidenceTag::new("commands"),
                command_identities.iter(),
            )
            .field_evidence_identity(
                WorthQueryEvidenceTag::new("graph_breadth"),
                self.graph_composition_breadth.breadth_evidence_digest(),
            )
            .field_evidence_identity(
                WorthQueryEvidenceTag::new("graph_program"),
                self.graph_composition_program.program_evidence_digest(),
            )
            .seal()
            .as_str()
            .to_string()
    }
}

fn mutation_intent_name(command: &WorthQueryWriteCommand) -> String {
    let family = command.mutation_family().as_str();
    let target = mutation_target_label(command);
    format!("mutation.{family}.{target}")
}

fn mutation_input_digest(command: &WorthQueryWriteCommand) -> String {
    mutation_input_identity(command).as_str().to_string()
}

fn mutation_input_identity(command: &WorthQueryWriteCommand) -> WorthQueryEvidenceIdentity {
    let declared_entity_identity = command
        .declared_entity_identity_ref()
        .map(|identity| identity.evidence_identity());
    let declared_collection_identity = command.declared_collection_identity();
    let aspect_value_identity = command_declared_aspect_value_digest(command).map(|digest| {
        worth_query_evidence_identity(WorthQueryEvidenceScope::AuthoritativeMutationIntentSeed)
            .field_shape(WorthQueryEvidenceTag::new("role"), "declared-aspect-values")
            .field_value(WorthQueryEvidenceTag::new("digest"), digest)
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
        worth_query_evidence_identity(WorthQueryEvidenceScope::AuthoritativeMutationIntentSeed)
            .field_shape(
                WorthQueryEvidenceTag::new("family"),
                command.mutation_family().as_str(),
            )
            .optional_evidence_identity(
                WorthQueryEvidenceTag::new("collection"),
                declared_collection_identity
                    .as_ref()
                    .map(WorthQueryMutationTargetCollectionIdentity::evidence_identity),
            )
            .optional_evidence_identity(
                WorthQueryEvidenceTag::new("declared_entity_identity"),
                declared_entity_identity.as_ref(),
            )
            .optional_evidence_identity(
                WorthQueryEvidenceTag::new("existing_truth_binding"),
                command
                    .existing_truth_binding()
                    .map(|binding| binding.binding_evidence_identity()),
            )
            .field_evidence_identity_sequence(
                WorthQueryEvidenceTag::new("operations"),
                operation_identities.iter(),
            )
            .optional_evidence_identity(
                WorthQueryEvidenceTag::new("aspect_values"),
                aspect_value_identity.as_ref(),
            )
            .optional_evidence_identity(
                WorthQueryEvidenceTag::new("symbolic_target"),
                symbolic_target.as_ref(),
            )
            .field_evidence_identity_sequence(
                WorthQueryEvidenceTag::new("symbolic_aspects"),
                symbolic_aspects.iter(),
            );
    if let Some(identity_evidence) = declared_entity_identity {
        identity = identity.field_evidence_identity(
            WorthQueryEvidenceTag::new("target_label_identity"),
            &identity_evidence,
        );
    }
    identity.seal()
}

fn declared_aspect_operation_identity(
    operation: WorthQueryAspectMutationOperation,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::AuthoritativeMutationIntentSeed)
        .field_shape(
            WorthQueryEvidenceTag::new("role"),
            "declared-aspect-operation",
        )
        .field_shape(
            WorthQueryEvidenceTag::new("kind"),
            operation.kind().as_str(),
        )
        .field_value(
            WorthQueryEvidenceTag::new("admitted_aspect_touch"),
            operation.aspect_touch().admitted_touch_digest_part(),
        )
        .seal()
}

fn symbolic_target_reference_identity(
    reference: &WorthQuerySymbolicTargetReference,
) -> WorthQueryEvidenceIdentity {
    let symbol_identity =
        WorthQueryMutationSymbolIdentity::new("authoritative-mutation-seed", reference.symbol());
    let collection_identity = reference.target_collection().map(|collection| {
        WorthQueryMutationTargetCollectionIdentity::new(
            "authoritative-mutation-seed-symbolic-target",
            collection,
        )
    });
    worth_query_evidence_identity(WorthQueryEvidenceScope::AuthoritativeMutationIntentSeed)
        .field_shape(WorthQueryEvidenceTag::new("role"), "symbolic-target")
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("symbol"),
            symbol_identity.evidence_identity(),
        )
        .optional_evidence_identity(
            WorthQueryEvidenceTag::new("collection"),
            collection_identity
                .as_ref()
                .map(WorthQueryMutationTargetCollectionIdentity::evidence_identity),
        )
        .seal()
}

fn symbolic_aspect_reference_identity(
    reference: &WorthQuerySymbolicAspectReference,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::AuthoritativeMutationIntentSeed)
        .field_shape(WorthQueryEvidenceTag::new("role"), "symbolic-aspect")
        .field_value(
            WorthQueryEvidenceTag::new("admitted_aspect_touch"),
            reference.aspect_touch().admitted_touch_digest_part(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("family"),
            reference.family().as_str(),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("target"),
            &symbolic_target_reference_identity(reference.reference()),
        )
        .seal()
}

fn mutation_target_label(command: &WorthQueryWriteCommand) -> String {
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
