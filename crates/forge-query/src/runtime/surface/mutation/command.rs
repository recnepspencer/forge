use crate::memory_workspace::ForgeQueryEntityIdentity;
use crate::runtime::mutation::{
    command_declared_aspect_operations, command_declared_aspect_touches, ForgeQueryAspectTouch,
    ForgeQueryContinuityMutationIntent, ForgeQueryMutationMetadata, ForgeQueryNamingMutationIntent,
};
use crate::runtime::surface::mutation::ForgeQueryMutationFamily;
use crate::runtime::{
    ForgeQueryAdmittedAspectValue, ForgeQueryAspectMutationOperation,
    ForgeQueryExistingTruthTargetBinding, ForgeQueryMutationTargetCollectionIdentity,
    ForgeQuerySymbolicAspectReference, ForgeQuerySymbolicTargetReference,
};

#[path = "command/accessors.rs"]
mod accessors;

#[derive(Clone, Debug, PartialEq)]
pub enum ForgeQueryWriteCommand {
    #[non_exhaustive]
    InsertAspects {
        collection: ForgeQueryMutationTargetCollectionIdentity,
        aspects: Vec<ForgeQueryAdmittedAspectValue>,
        symbolic_aspect_references: Vec<ForgeQuerySymbolicAspectReference>,
        metadata: ForgeQueryMutationMetadata,
        naming_intent: Option<ForgeQueryNamingMutationIntent>,
        continuity_intent: Option<ForgeQueryContinuityMutationIntent>,
        symbolic_target_reference: Option<ForgeQuerySymbolicTargetReference>,
    },
    #[non_exhaustive]
    UpdateAspect {
        entity_identity: ForgeQueryEntityIdentity,
        aspect: ForgeQueryAdmittedAspectValue,
    },
    #[non_exhaustive]
    UpdateAspects {
        entity_identity: ForgeQueryEntityIdentity,
        aspects: Vec<ForgeQueryAdmittedAspectValue>,
        metadata: ForgeQueryMutationMetadata,
        naming_intent: Option<ForgeQueryNamingMutationIntent>,
        continuity_intent: Option<ForgeQueryContinuityMutationIntent>,
    },
    #[non_exhaustive]
    UpdateExistingAspects {
        binding: ForgeQueryExistingTruthTargetBinding,
        aspects: Vec<ForgeQueryAdmittedAspectValue>,
        metadata: ForgeQueryMutationMetadata,
        naming_intent: Option<ForgeQueryNamingMutationIntent>,
        continuity_intent: Option<ForgeQueryContinuityMutationIntent>,
    },
    #[non_exhaustive]
    VerifyThenUpdateExistingAspects {
        binding: ForgeQueryExistingTruthTargetBinding,
        asserted_aspects: Vec<ForgeQueryAdmittedAspectValue>,
        aspects: Vec<ForgeQueryAdmittedAspectValue>,
        symbolic_aspect_references: Vec<ForgeQuerySymbolicAspectReference>,
        metadata: ForgeQueryMutationMetadata,
        naming_intent: Option<ForgeQueryNamingMutationIntent>,
        continuity_intent: Option<ForgeQueryContinuityMutationIntent>,
    },
    #[non_exhaustive]
    VerifyThenDeleteExistingAspects {
        binding: ForgeQueryExistingTruthTargetBinding,
        asserted_aspects: Vec<ForgeQueryAdmittedAspectValue>,
        touched_aspects: Vec<ForgeQueryAspectTouch>,
        metadata: ForgeQueryMutationMetadata,
        naming_intent: Option<ForgeQueryNamingMutationIntent>,
    },
    #[non_exhaustive]
    AssertExistingAspects {
        binding: ForgeQueryExistingTruthTargetBinding,
        aspects: Vec<ForgeQueryAdmittedAspectValue>,
        metadata: ForgeQueryMutationMetadata,
    },
    #[non_exhaustive]
    VerifyExistingAspects {
        binding: ForgeQueryExistingTruthTargetBinding,
        aspects: Vec<ForgeQueryAdmittedAspectValue>,
        metadata: ForgeQueryMutationMetadata,
    },
    #[non_exhaustive]
    UpdateSymbolicAspects {
        reference: ForgeQuerySymbolicTargetReference,
        aspects: Vec<ForgeQueryAdmittedAspectValue>,
        metadata: ForgeQueryMutationMetadata,
        naming_intent: Option<ForgeQueryNamingMutationIntent>,
        continuity_intent: Option<ForgeQueryContinuityMutationIntent>,
    },
    #[non_exhaustive]
    DeleteAspects {
        entity_identity: ForgeQueryEntityIdentity,
        declared_collection: Option<ForgeQueryMutationTargetCollectionIdentity>,
        touched_aspects: Vec<ForgeQueryAspectTouch>,
        metadata: ForgeQueryMutationMetadata,
        naming_intent: Option<ForgeQueryNamingMutationIntent>,
    },
    #[non_exhaustive]
    DeleteExistingAspects {
        binding: ForgeQueryExistingTruthTargetBinding,
        touched_aspects: Vec<ForgeQueryAspectTouch>,
        metadata: ForgeQueryMutationMetadata,
        naming_intent: Option<ForgeQueryNamingMutationIntent>,
    },
    #[non_exhaustive]
    DeleteSymbolicAspects {
        reference: ForgeQuerySymbolicTargetReference,
        touched_aspects: Vec<ForgeQueryAspectTouch>,
        metadata: ForgeQueryMutationMetadata,
        naming_intent: Option<ForgeQueryNamingMutationIntent>,
    },
    #[non_exhaustive]
    Delete {
        entity_identity: ForgeQueryEntityIdentity,
    },
}

impl ForgeQueryWriteCommand {
    pub fn declared_aspect_touches(&self) -> Vec<ForgeQueryAspectTouch> {
        command_declared_aspect_touches(self)
    }

    pub fn declared_aspect_operations(&self) -> Vec<ForgeQueryAspectMutationOperation> {
        command_declared_aspect_operations(self)
    }

    pub fn mutation_family(&self) -> ForgeQueryMutationFamily {
        match self {
            Self::InsertAspects { .. } => ForgeQueryMutationFamily::Insert,
            Self::UpdateAspect { .. }
            | Self::UpdateAspects { .. }
            | Self::UpdateExistingAspects { .. }
            | Self::VerifyThenUpdateExistingAspects { .. }
            | Self::UpdateSymbolicAspects { .. } => ForgeQueryMutationFamily::Update,
            Self::AssertExistingAspects { .. } | Self::VerifyExistingAspects { .. } => {
                ForgeQueryMutationFamily::Assertion
            }
            Self::DeleteAspects { .. }
            | Self::VerifyThenDeleteExistingAspects { .. }
            | Self::DeleteExistingAspects { .. }
            | Self::DeleteSymbolicAspects { .. }
            | Self::Delete { .. } => ForgeQueryMutationFamily::Delete,
        }
    }

    pub fn declared_collection_identity(
        &self,
    ) -> Option<ForgeQueryMutationTargetCollectionIdentity> {
        match self {
            Self::InsertAspects { collection, .. } => Some(collection.clone()),
            Self::VerifyThenUpdateExistingAspects {
                binding,
                symbolic_aspect_references,
                ..
            } => binding.target_collection_identity().cloned().or_else(|| {
                symbolic_aspect_references
                    .first()
                    .and_then(|reference| reference.reference().target_collection_identity())
                    .cloned()
            }),
            Self::VerifyThenDeleteExistingAspects { binding, .. }
            | Self::AssertExistingAspects { binding, .. }
            | Self::VerifyExistingAspects { binding, .. }
            | Self::DeleteExistingAspects { binding, .. } => {
                binding.target_collection_identity().cloned()
            }
            Self::UpdateSymbolicAspects { reference, .. }
            | Self::DeleteSymbolicAspects { reference, .. } => {
                reference.target_collection_identity().cloned()
            }
            Self::DeleteAspects {
                declared_collection,
                ..
            } => declared_collection.clone(),
            Self::UpdateAspect { .. }
            | Self::UpdateAspects { .. }
            | Self::UpdateExistingAspects { .. }
            | Self::Delete { .. } => None,
        }
    }

    pub fn declared_entity_identity(&self) -> Option<ForgeQueryEntityIdentity> {
        match self {
            Self::UpdateAspect {
                entity_identity, ..
            }
            | Self::UpdateAspects {
                entity_identity, ..
            }
            | Self::DeleteAspects {
                entity_identity, ..
            }
            | Self::Delete { entity_identity } => Some(entity_identity.clone()),
            Self::VerifyThenUpdateExistingAspects { binding, .. }
            | Self::VerifyThenDeleteExistingAspects { binding, .. }
            | Self::AssertExistingAspects { binding, .. }
            | Self::VerifyExistingAspects { binding, .. } => {
                Some(binding.resolved_entity_artifact_identity())
            }
            Self::UpdateSymbolicAspects { .. }
            | Self::DeleteSymbolicAspects { .. }
            | Self::UpdateExistingAspects { .. }
            | Self::DeleteExistingAspects { .. }
            | Self::InsertAspects { .. } => None,
        }
    }

    pub fn declared_entity_identity_ref(&self) -> Option<&ForgeQueryEntityIdentity> {
        match self {
            Self::UpdateAspect {
                entity_identity, ..
            }
            | Self::UpdateAspects {
                entity_identity, ..
            }
            | Self::DeleteAspects {
                entity_identity, ..
            }
            | Self::Delete { entity_identity } => Some(entity_identity),
            Self::VerifyThenUpdateExistingAspects { .. }
            | Self::VerifyThenDeleteExistingAspects { .. }
            | Self::AssertExistingAspects { .. }
            | Self::VerifyExistingAspects { .. } => None,
            Self::UpdateSymbolicAspects { .. }
            | Self::DeleteSymbolicAspects { .. }
            | Self::UpdateExistingAspects { .. }
            | Self::DeleteExistingAspects { .. }
            | Self::InsertAspects { .. } => None,
        }
    }

    pub fn mutation_metadata(&self) -> ForgeQueryMutationMetadata {
        self.mutation_metadata_ref().cloned().unwrap_or_default()
    }

    pub fn mutation_metadata_ref(&self) -> Option<&ForgeQueryMutationMetadata> {
        match self {
            Self::InsertAspects { metadata, .. }
            | Self::UpdateAspects { metadata, .. }
            | Self::UpdateExistingAspects { metadata, .. }
            | Self::VerifyThenUpdateExistingAspects { metadata, .. }
            | Self::VerifyThenDeleteExistingAspects { metadata, .. }
            | Self::AssertExistingAspects { metadata, .. }
            | Self::VerifyExistingAspects { metadata, .. }
            | Self::UpdateSymbolicAspects { metadata, .. }
            | Self::DeleteAspects { metadata, .. }
            | Self::DeleteExistingAspects { metadata, .. }
            | Self::DeleteSymbolicAspects { metadata, .. } => Some(metadata),
            Self::UpdateAspect { .. } | Self::Delete { .. } => None,
        }
    }

    pub fn admitted_aspect_values(&self) -> &[ForgeQueryAdmittedAspectValue] {
        match self {
            Self::InsertAspects { aspects, .. }
            | Self::UpdateAspects { aspects, .. }
            | Self::UpdateExistingAspects { aspects, .. }
            | Self::VerifyThenUpdateExistingAspects { aspects, .. }
            | Self::AssertExistingAspects { aspects, .. }
            | Self::VerifyExistingAspects { aspects, .. }
            | Self::UpdateSymbolicAspects { aspects, .. } => aspects,
            Self::UpdateAspect { aspect, .. } => std::slice::from_ref(aspect),
            Self::VerifyThenDeleteExistingAspects { .. }
            | Self::DeleteAspects { .. }
            | Self::DeleteExistingAspects { .. }
            | Self::DeleteSymbolicAspects { .. }
            | Self::Delete { .. } => &[],
        }
    }

    pub fn asserted_admitted_aspect_values(&self) -> &[ForgeQueryAdmittedAspectValue] {
        match self {
            Self::VerifyThenUpdateExistingAspects {
                asserted_aspects, ..
            }
            | Self::VerifyThenDeleteExistingAspects {
                asserted_aspects, ..
            } => asserted_aspects,
            Self::InsertAspects { .. }
            | Self::UpdateAspect { .. }
            | Self::UpdateAspects { .. }
            | Self::UpdateExistingAspects { .. }
            | Self::AssertExistingAspects { .. }
            | Self::VerifyExistingAspects { .. }
            | Self::UpdateSymbolicAspects { .. }
            | Self::DeleteAspects { .. }
            | Self::DeleteExistingAspects { .. }
            | Self::DeleteSymbolicAspects { .. }
            | Self::Delete { .. } => &[],
        }
    }

    pub fn admitted_touched_aspects(&self) -> &[ForgeQueryAspectTouch] {
        match self {
            Self::VerifyThenDeleteExistingAspects {
                touched_aspects, ..
            }
            | Self::DeleteAspects {
                touched_aspects, ..
            }
            | Self::DeleteExistingAspects {
                touched_aspects, ..
            }
            | Self::DeleteSymbolicAspects {
                touched_aspects, ..
            } => touched_aspects,
            Self::InsertAspects { .. }
            | Self::UpdateAspect { .. }
            | Self::UpdateAspects { .. }
            | Self::UpdateExistingAspects { .. }
            | Self::VerifyThenUpdateExistingAspects { .. }
            | Self::AssertExistingAspects { .. }
            | Self::VerifyExistingAspects { .. }
            | Self::UpdateSymbolicAspects { .. }
            | Self::Delete { .. } => &[],
        }
    }
}
