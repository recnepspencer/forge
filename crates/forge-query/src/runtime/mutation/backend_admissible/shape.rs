use crate::memory_workspace::ForgeQueryEntityIdentity;
use crate::runtime::{
    ForgeQueryAdmittedAspectValue, ForgeQueryAspectTouch, ForgeQueryContinuityMutationIntent,
    ForgeQueryExistingTruthTargetBinding, ForgeQueryMutationMetadata,
    ForgeQueryMutationTargetCollectionIdentity, ForgeQueryNamingMutationIntent,
    ForgeQuerySymbolicAspectReference, ForgeQuerySymbolicTargetReference, ForgeQueryWriteCommand,
};

#[derive(Clone, Debug, PartialEq)]
pub(super) enum ForgeQueryBackendAdmissibleMutationShape {
    Insert {
        collection: ForgeQueryMutationTargetCollectionIdentity,
        aspects: Vec<ForgeQueryAdmittedAspectValue>,
        symbolic_aspect_references: Vec<ForgeQuerySymbolicAspectReference>,
        metadata: ForgeQueryMutationMetadata,
        naming_intent: Option<ForgeQueryNamingMutationIntent>,
        continuity_intent: Option<ForgeQueryContinuityMutationIntent>,
        symbolic_target_reference: Option<ForgeQuerySymbolicTargetReference>,
    },
    UpdateDirect {
        entity_identity: ForgeQueryEntityIdentity,
        aspects: Vec<ForgeQueryAdmittedAspectValue>,
        metadata: ForgeQueryMutationMetadata,
        naming_intent: Option<ForgeQueryNamingMutationIntent>,
        continuity_intent: Option<ForgeQueryContinuityMutationIntent>,
    },
    UpdateExisting {
        binding: ForgeQueryExistingTruthTargetBinding,
        asserted_aspects: Vec<ForgeQueryAdmittedAspectValue>,
        aspects: Vec<ForgeQueryAdmittedAspectValue>,
        symbolic_aspect_references: Vec<ForgeQuerySymbolicAspectReference>,
        metadata: ForgeQueryMutationMetadata,
        naming_intent: Option<ForgeQueryNamingMutationIntent>,
        continuity_intent: Option<ForgeQueryContinuityMutationIntent>,
    },
    Assertion {
        binding: ForgeQueryExistingTruthTargetBinding,
        aspects: Vec<ForgeQueryAdmittedAspectValue>,
        metadata: ForgeQueryMutationMetadata,
    },
    UpdateSymbolic {
        reference: ForgeQuerySymbolicTargetReference,
        aspects: Vec<ForgeQueryAdmittedAspectValue>,
        metadata: ForgeQueryMutationMetadata,
        naming_intent: Option<ForgeQueryNamingMutationIntent>,
        continuity_intent: Option<ForgeQueryContinuityMutationIntent>,
    },
    DeleteDirect {
        entity_identity: ForgeQueryEntityIdentity,
        declared_collection: Option<ForgeQueryMutationTargetCollectionIdentity>,
        touched_aspects: Vec<ForgeQueryAspectTouch>,
        metadata: ForgeQueryMutationMetadata,
        naming_intent: Option<ForgeQueryNamingMutationIntent>,
    },
    DeleteExisting {
        binding: ForgeQueryExistingTruthTargetBinding,
        asserted_aspects: Vec<ForgeQueryAdmittedAspectValue>,
        touched_aspects: Vec<ForgeQueryAspectTouch>,
        metadata: ForgeQueryMutationMetadata,
        naming_intent: Option<ForgeQueryNamingMutationIntent>,
    },
    DeleteSymbolic {
        reference: ForgeQuerySymbolicTargetReference,
        touched_aspects: Vec<ForgeQueryAspectTouch>,
        metadata: ForgeQueryMutationMetadata,
        naming_intent: Option<ForgeQueryNamingMutationIntent>,
    },
}

impl ForgeQueryBackendAdmissibleMutationShape {
    pub(super) fn from_admitted_command(command: ForgeQueryWriteCommand) -> Self {
        match command {
            ForgeQueryWriteCommand::InsertAspects {
                collection,
                aspects,
                symbolic_aspect_references,
                metadata,
                naming_intent,
                continuity_intent,
                symbolic_target_reference,
            } => Self::Insert {
                collection: ForgeQueryMutationTargetCollectionIdentity::new(
                    "backend-admissible-declared",
                    collection.as_str(),
                ),
                aspects,
                symbolic_aspect_references,
                metadata,
                naming_intent,
                continuity_intent,
                symbolic_target_reference,
            },
            ForgeQueryWriteCommand::UpdateAspect {
                entity_identity,
                aspect,
            } => Self::UpdateDirect {
                entity_identity,
                aspects: vec![aspect],
                metadata: ForgeQueryMutationMetadata::default(),
                naming_intent: None,
                continuity_intent: None,
            },
            ForgeQueryWriteCommand::UpdateAspects {
                entity_identity,
                aspects,
                metadata,
                naming_intent,
                continuity_intent,
            } => Self::UpdateDirect {
                entity_identity,
                aspects,
                metadata,
                naming_intent,
                continuity_intent,
            },
            ForgeQueryWriteCommand::UpdateExistingAspects {
                binding,
                aspects,
                metadata,
                naming_intent,
                continuity_intent,
            } => Self::UpdateExisting {
                binding,
                asserted_aspects: Vec::new(),
                aspects,
                symbolic_aspect_references: Vec::new(),
                metadata,
                naming_intent,
                continuity_intent,
            },
            ForgeQueryWriteCommand::VerifyThenUpdateExistingAspects {
                binding,
                asserted_aspects,
                aspects,
                symbolic_aspect_references,
                metadata,
                naming_intent,
                continuity_intent,
            } => Self::UpdateExisting {
                binding,
                asserted_aspects,
                aspects,
                symbolic_aspect_references,
                metadata,
                naming_intent,
                continuity_intent,
            },
            ForgeQueryWriteCommand::AssertExistingAspects {
                binding,
                aspects,
                metadata,
            }
            | ForgeQueryWriteCommand::VerifyExistingAspects {
                binding,
                aspects,
                metadata,
            } => Self::Assertion {
                binding,
                aspects,
                metadata,
            },
            ForgeQueryWriteCommand::UpdateSymbolicAspects {
                reference,
                aspects,
                metadata,
                naming_intent,
                continuity_intent,
            } => Self::UpdateSymbolic {
                reference,
                aspects,
                metadata,
                naming_intent,
                continuity_intent,
            },
            ForgeQueryWriteCommand::DeleteAspects {
                entity_identity,
                declared_collection,
                touched_aspects,
                metadata,
                naming_intent,
            } => Self::DeleteDirect {
                entity_identity,
                declared_collection: declared_collection.map(|collection| {
                    ForgeQueryMutationTargetCollectionIdentity::new(
                        "backend-admissible-declared",
                        collection.as_str(),
                    )
                }),
                touched_aspects,
                metadata,
                naming_intent,
            },
            ForgeQueryWriteCommand::Delete { entity_identity } => Self::DeleteDirect {
                entity_identity,
                declared_collection: None,
                touched_aspects: Vec::new(),
                metadata: ForgeQueryMutationMetadata::default(),
                naming_intent: None,
            },
            ForgeQueryWriteCommand::VerifyThenDeleteExistingAspects {
                binding,
                asserted_aspects,
                touched_aspects,
                metadata,
                naming_intent,
            } => Self::DeleteExisting {
                binding,
                asserted_aspects,
                touched_aspects,
                metadata,
                naming_intent,
            },
            ForgeQueryWriteCommand::DeleteExistingAspects {
                binding,
                touched_aspects,
                metadata,
                naming_intent,
            } => Self::DeleteExisting {
                binding,
                asserted_aspects: Vec::new(),
                touched_aspects,
                metadata,
                naming_intent,
            },
            ForgeQueryWriteCommand::DeleteSymbolicAspects {
                reference,
                touched_aspects,
                metadata,
                naming_intent,
            } => Self::DeleteSymbolic {
                reference,
                touched_aspects,
                metadata,
                naming_intent,
            },
        }
    }
}
