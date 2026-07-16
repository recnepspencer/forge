use crate::memory_workspace::WorthQueryEntityIdentity;
use crate::runtime::mutation::{
    command_declared_aspect_operations, command_declared_aspect_touches, WorthQueryAspectTouch,
    WorthQueryContinuityMutationIntent, WorthQueryMutationMetadata, WorthQueryNamingMutationIntent,
};
use crate::runtime::surface::mutation::WorthQueryMutationFamily;
use crate::runtime::{
    WorthQueryAspectMutationOperation, WorthQueryAuthoredAspectMutation,
    WorthQueryExistingTruthTargetBinding, WorthQueryMutationTargetCollectionIdentity,
    WorthQuerySymbolicAspectReference, WorthQuerySymbolicTargetReference,
};

#[path = "command/accessors.rs"]
mod accessors;

#[derive(Clone, Debug, PartialEq)]
pub enum WorthQueryWriteCommand {
    #[non_exhaustive]
    InsertAspects {
        collection: WorthQueryMutationTargetCollectionIdentity,
        aspects: Vec<WorthQueryAuthoredAspectMutation>,
        symbolic_aspect_references: Vec<WorthQuerySymbolicAspectReference>,
        metadata: WorthQueryMutationMetadata,
        naming_intent: Option<WorthQueryNamingMutationIntent>,
        continuity_intent: Option<WorthQueryContinuityMutationIntent>,
        symbolic_target_reference: Option<WorthQuerySymbolicTargetReference>,
    },
    #[non_exhaustive]
    UpdateAspect {
        entity_identity: WorthQueryEntityIdentity,
        aspect: WorthQueryAuthoredAspectMutation,
    },
    #[non_exhaustive]
    UpdateAspects {
        entity_identity: WorthQueryEntityIdentity,
        aspects: Vec<WorthQueryAuthoredAspectMutation>,
        metadata: WorthQueryMutationMetadata,
        naming_intent: Option<WorthQueryNamingMutationIntent>,
        continuity_intent: Option<WorthQueryContinuityMutationIntent>,
    },
    #[non_exhaustive]
    UpdateExistingAspects {
        binding: WorthQueryExistingTruthTargetBinding,
        aspects: Vec<WorthQueryAuthoredAspectMutation>,
        metadata: WorthQueryMutationMetadata,
        naming_intent: Option<WorthQueryNamingMutationIntent>,
        continuity_intent: Option<WorthQueryContinuityMutationIntent>,
    },
    #[non_exhaustive]
    VerifyThenUpdateExistingAspects {
        binding: WorthQueryExistingTruthTargetBinding,
        asserted_aspects: Vec<WorthQueryAuthoredAspectMutation>,
        aspects: Vec<WorthQueryAuthoredAspectMutation>,
        symbolic_aspect_references: Vec<WorthQuerySymbolicAspectReference>,
        metadata: WorthQueryMutationMetadata,
        naming_intent: Option<WorthQueryNamingMutationIntent>,
        continuity_intent: Option<WorthQueryContinuityMutationIntent>,
    },
    #[non_exhaustive]
    VerifyThenDeleteExistingAspects {
        binding: WorthQueryExistingTruthTargetBinding,
        asserted_aspects: Vec<WorthQueryAuthoredAspectMutation>,
        touched_aspects: Vec<WorthQueryAspectTouch>,
        metadata: WorthQueryMutationMetadata,
        naming_intent: Option<WorthQueryNamingMutationIntent>,
    },
    #[non_exhaustive]
    AssertExistingAspects {
        binding: WorthQueryExistingTruthTargetBinding,
        aspects: Vec<WorthQueryAuthoredAspectMutation>,
        metadata: WorthQueryMutationMetadata,
    },
    #[non_exhaustive]
    VerifyExistingAspects {
        binding: WorthQueryExistingTruthTargetBinding,
        aspects: Vec<WorthQueryAuthoredAspectMutation>,
        metadata: WorthQueryMutationMetadata,
    },
    #[non_exhaustive]
    UpdateSymbolicAspects {
        reference: WorthQuerySymbolicTargetReference,
        aspects: Vec<WorthQueryAuthoredAspectMutation>,
        metadata: WorthQueryMutationMetadata,
        naming_intent: Option<WorthQueryNamingMutationIntent>,
        continuity_intent: Option<WorthQueryContinuityMutationIntent>,
    },
    #[non_exhaustive]
    DeleteAspects {
        entity_identity: WorthQueryEntityIdentity,
        declared_collection: Option<WorthQueryMutationTargetCollectionIdentity>,
        touched_aspects: Vec<WorthQueryAspectTouch>,
        metadata: WorthQueryMutationMetadata,
        naming_intent: Option<WorthQueryNamingMutationIntent>,
    },
    #[non_exhaustive]
    DeleteExistingAspects {
        binding: WorthQueryExistingTruthTargetBinding,
        touched_aspects: Vec<WorthQueryAspectTouch>,
        metadata: WorthQueryMutationMetadata,
        naming_intent: Option<WorthQueryNamingMutationIntent>,
    },
    #[non_exhaustive]
    DeleteSymbolicAspects {
        reference: WorthQuerySymbolicTargetReference,
        touched_aspects: Vec<WorthQueryAspectTouch>,
        metadata: WorthQueryMutationMetadata,
        naming_intent: Option<WorthQueryNamingMutationIntent>,
    },
    #[non_exhaustive]
    Delete {
        entity_identity: WorthQueryEntityIdentity,
    },
}

impl WorthQueryWriteCommand {
    pub fn declared_aspect_touches(&self) -> Vec<WorthQueryAspectTouch> {
        command_declared_aspect_touches(self)
    }

    pub fn declared_aspect_operations(&self) -> Vec<WorthQueryAspectMutationOperation> {
        command_declared_aspect_operations(self)
    }

    pub fn mutation_family(&self) -> WorthQueryMutationFamily {
        match self {
            Self::InsertAspects { .. } => WorthQueryMutationFamily::Insert,
            Self::UpdateAspect { .. }
            | Self::UpdateAspects { .. }
            | Self::UpdateExistingAspects { .. }
            | Self::VerifyThenUpdateExistingAspects { .. }
            | Self::UpdateSymbolicAspects { .. } => WorthQueryMutationFamily::Update,
            Self::AssertExistingAspects { .. } | Self::VerifyExistingAspects { .. } => {
                WorthQueryMutationFamily::Assertion
            }
            Self::DeleteAspects { .. }
            | Self::VerifyThenDeleteExistingAspects { .. }
            | Self::DeleteExistingAspects { .. }
            | Self::DeleteSymbolicAspects { .. }
            | Self::Delete { .. } => WorthQueryMutationFamily::Delete,
        }
    }

    pub fn declared_collection_identity(
        &self,
    ) -> Option<WorthQueryMutationTargetCollectionIdentity> {
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

    pub fn declared_entity_identity(&self) -> Option<WorthQueryEntityIdentity> {
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

    pub fn declared_entity_identity_ref(&self) -> Option<&WorthQueryEntityIdentity> {
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

    pub fn mutation_metadata(&self) -> WorthQueryMutationMetadata {
        self.mutation_metadata_ref().cloned().unwrap_or_default()
    }

    pub fn mutation_metadata_ref(&self) -> Option<&WorthQueryMutationMetadata> {
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

    pub fn admitted_aspect_values(&self) -> &[WorthQueryAuthoredAspectMutation] {
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

    pub fn asserted_admitted_aspect_values(&self) -> &[WorthQueryAuthoredAspectMutation] {
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

    pub fn admitted_touched_aspects(&self) -> &[WorthQueryAspectTouch] {
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
