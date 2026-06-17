use serde_json::Value;

use crate::memory_workspace::ForgeQueryEntityIdentity;
use crate::runtime::mutation::{
    command_declared_aspect_operations, command_declared_aspect_paths,
    ForgeQueryContinuityMutationIntent, ForgeQueryMutationMetadata, ForgeQueryNamingMutationIntent,
};
use crate::runtime::surface::mutation::ForgeQueryMutationFamily;
use crate::runtime::{
    ForgeQueryAspectMutationOperation, ForgeQueryAspectValue, ForgeQueryExistingTruthTargetBinding,
    ForgeQuerySymbolicAspectReference, ForgeQuerySymbolicTargetReference,
};

#[derive(Clone, Debug, PartialEq)]
pub enum ForgeQueryWriteCommand {
    InsertAspects {
        collection: String,
        aspects: Vec<ForgeQueryAspectValue>,
        symbolic_aspect_references: Vec<ForgeQuerySymbolicAspectReference>,
        metadata: ForgeQueryMutationMetadata,
        naming_intent: Option<ForgeQueryNamingMutationIntent>,
        continuity_intent: Option<ForgeQueryContinuityMutationIntent>,
        symbolic_target_reference: Option<ForgeQuerySymbolicTargetReference>,
    },
    UpdateAspect {
        entity_identity: ForgeQueryEntityIdentity,
        aspect_path: String,
        value: Value,
    },
    UpdateAspects {
        entity_identity: ForgeQueryEntityIdentity,
        aspects: Vec<ForgeQueryAspectValue>,
        metadata: ForgeQueryMutationMetadata,
        naming_intent: Option<ForgeQueryNamingMutationIntent>,
        continuity_intent: Option<ForgeQueryContinuityMutationIntent>,
    },
    UpdateExistingAspects {
        binding: ForgeQueryExistingTruthTargetBinding,
        aspects: Vec<ForgeQueryAspectValue>,
        metadata: ForgeQueryMutationMetadata,
        naming_intent: Option<ForgeQueryNamingMutationIntent>,
        continuity_intent: Option<ForgeQueryContinuityMutationIntent>,
    },
    VerifyThenUpdateExistingAspects {
        binding: ForgeQueryExistingTruthTargetBinding,
        asserted_aspects: Vec<ForgeQueryAspectValue>,
        aspects: Vec<ForgeQueryAspectValue>,
        symbolic_aspect_references: Vec<ForgeQuerySymbolicAspectReference>,
        metadata: ForgeQueryMutationMetadata,
        naming_intent: Option<ForgeQueryNamingMutationIntent>,
        continuity_intent: Option<ForgeQueryContinuityMutationIntent>,
    },
    VerifyThenDeleteExistingAspects {
        binding: ForgeQueryExistingTruthTargetBinding,
        asserted_aspects: Vec<ForgeQueryAspectValue>,
        touched_aspect_paths: Vec<String>,
        metadata: ForgeQueryMutationMetadata,
        naming_intent: Option<ForgeQueryNamingMutationIntent>,
    },
    AssertExistingAspects {
        binding: ForgeQueryExistingTruthTargetBinding,
        aspects: Vec<ForgeQueryAspectValue>,
        metadata: ForgeQueryMutationMetadata,
    },
    VerifyExistingAspects {
        binding: ForgeQueryExistingTruthTargetBinding,
        aspects: Vec<ForgeQueryAspectValue>,
        metadata: ForgeQueryMutationMetadata,
    },
    UpdateSymbolicAspects {
        reference: ForgeQuerySymbolicTargetReference,
        aspects: Vec<ForgeQueryAspectValue>,
        metadata: ForgeQueryMutationMetadata,
        naming_intent: Option<ForgeQueryNamingMutationIntent>,
        continuity_intent: Option<ForgeQueryContinuityMutationIntent>,
    },
    DeleteAspects {
        entity_identity: ForgeQueryEntityIdentity,
        declared_collection: Option<String>,
        touched_aspect_paths: Vec<String>,
        metadata: ForgeQueryMutationMetadata,
        naming_intent: Option<ForgeQueryNamingMutationIntent>,
    },
    DeleteExistingAspects {
        binding: ForgeQueryExistingTruthTargetBinding,
        touched_aspect_paths: Vec<String>,
        metadata: ForgeQueryMutationMetadata,
        naming_intent: Option<ForgeQueryNamingMutationIntent>,
    },
    DeleteSymbolicAspects {
        reference: ForgeQuerySymbolicTargetReference,
        touched_aspect_paths: Vec<String>,
        metadata: ForgeQueryMutationMetadata,
        naming_intent: Option<ForgeQueryNamingMutationIntent>,
    },
    Delete {
        entity_identity: ForgeQueryEntityIdentity,
    },
}

impl ForgeQueryWriteCommand {
    pub fn declared_aspect_paths(&self) -> Vec<String> {
        command_declared_aspect_paths(self)
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

    pub fn declared_collection(&self) -> Option<String> {
        self.declared_collection_ref().map(str::to_string)
    }

    pub fn declared_collection_ref(&self) -> Option<&str> {
        match self {
            Self::InsertAspects { collection, .. } => Some(collection.as_str()),
            Self::UpdateAspect { .. }
            | Self::UpdateAspects { .. }
            | Self::UpdateExistingAspects { .. }
            | Self::Delete { .. } => None,
            Self::VerifyThenUpdateExistingAspects {
                binding,
                symbolic_aspect_references,
                ..
            } => {
                if let Some(collection) = binding.target_collection() {
                    Some(collection)
                } else {
                    symbolic_aspect_references
                        .first()
                        .and_then(|reference| reference.reference().target_collection())
                }
            }
            Self::VerifyThenDeleteExistingAspects { binding, .. }
            | Self::AssertExistingAspects { binding, .. }
            | Self::VerifyExistingAspects { binding, .. }
            | Self::DeleteExistingAspects { binding, .. } => binding.target_collection(),
            Self::UpdateSymbolicAspects { reference, .. }
            | Self::DeleteSymbolicAspects { reference, .. } => reference.target_collection(),
            Self::DeleteAspects {
                declared_collection,
                ..
            } => declared_collection.as_deref(),
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

    pub fn aspect_values(&self) -> &[ForgeQueryAspectValue] {
        match self {
            Self::InsertAspects { aspects, .. }
            | Self::UpdateAspects { aspects, .. }
            | Self::UpdateExistingAspects { aspects, .. }
            | Self::VerifyThenUpdateExistingAspects { aspects, .. }
            | Self::AssertExistingAspects { aspects, .. }
            | Self::VerifyExistingAspects { aspects, .. }
            | Self::UpdateSymbolicAspects { aspects, .. } => aspects,
            Self::UpdateAspect { .. }
            | Self::VerifyThenDeleteExistingAspects { .. }
            | Self::DeleteAspects { .. }
            | Self::DeleteExistingAspects { .. }
            | Self::DeleteSymbolicAspects { .. }
            | Self::Delete { .. } => &[],
        }
    }

    pub fn asserted_aspect_values(&self) -> &[ForgeQueryAspectValue] {
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

    pub fn touched_aspect_paths(&self) -> &[String] {
        match self {
            Self::VerifyThenDeleteExistingAspects {
                touched_aspect_paths,
                ..
            }
            | Self::DeleteAspects {
                touched_aspect_paths,
                ..
            }
            | Self::DeleteExistingAspects {
                touched_aspect_paths,
                ..
            }
            | Self::DeleteSymbolicAspects {
                touched_aspect_paths,
                ..
            } => touched_aspect_paths,
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

    pub fn existing_truth_binding(&self) -> Option<&ForgeQueryExistingTruthTargetBinding> {
        match self {
            Self::UpdateExistingAspects { binding, .. }
            | Self::VerifyThenUpdateExistingAspects { binding, .. }
            | Self::VerifyThenDeleteExistingAspects { binding, .. }
            | Self::AssertExistingAspects { binding, .. }
            | Self::VerifyExistingAspects { binding, .. }
            | Self::DeleteExistingAspects { binding, .. } => Some(binding),
            _ => None,
        }
    }

    pub fn symbolic_target_reference(&self) -> Option<&ForgeQuerySymbolicTargetReference> {
        match self {
            Self::InsertAspects {
                symbolic_target_reference,
                ..
            } => symbolic_target_reference.as_ref(),
            Self::UpdateSymbolicAspects { reference, .. }
            | Self::DeleteSymbolicAspects { reference, .. } => Some(reference),
            _ => None,
        }
    }

    pub fn naming_intent(&self) -> Option<&ForgeQueryNamingMutationIntent> {
        match self {
            Self::InsertAspects { naming_intent, .. }
            | Self::UpdateAspects { naming_intent, .. }
            | Self::UpdateExistingAspects { naming_intent, .. }
            | Self::VerifyThenUpdateExistingAspects { naming_intent, .. }
            | Self::VerifyThenDeleteExistingAspects { naming_intent, .. }
            | Self::UpdateSymbolicAspects { naming_intent, .. }
            | Self::DeleteAspects { naming_intent, .. }
            | Self::DeleteExistingAspects { naming_intent, .. }
            | Self::DeleteSymbolicAspects { naming_intent, .. } => naming_intent.as_ref(),
            Self::AssertExistingAspects { .. }
            | Self::VerifyExistingAspects { .. }
            | Self::UpdateAspect { .. }
            | Self::Delete { .. } => None,
        }
    }

    pub fn continuity_intent(&self) -> Option<&ForgeQueryContinuityMutationIntent> {
        match self {
            Self::InsertAspects {
                continuity_intent, ..
            }
            | Self::UpdateAspects {
                continuity_intent, ..
            }
            | Self::UpdateExistingAspects {
                continuity_intent, ..
            }
            | Self::VerifyThenUpdateExistingAspects {
                continuity_intent, ..
            }
            | Self::UpdateSymbolicAspects {
                continuity_intent, ..
            } => continuity_intent.as_ref(),
            Self::VerifyThenDeleteExistingAspects { .. }
            | Self::AssertExistingAspects { .. }
            | Self::VerifyExistingAspects { .. }
            | Self::UpdateAspect { .. }
            | Self::DeleteAspects { .. }
            | Self::DeleteExistingAspects { .. }
            | Self::DeleteSymbolicAspects { .. }
            | Self::Delete { .. } => None,
        }
    }

    pub fn symbolic_aspect_references(&self) -> &[ForgeQuerySymbolicAspectReference] {
        match self {
            Self::InsertAspects {
                symbolic_aspect_references,
                ..
            }
            | Self::VerifyThenUpdateExistingAspects {
                symbolic_aspect_references,
                ..
            } => symbolic_aspect_references,
            Self::UpdateAspect { .. }
            | Self::UpdateAspects { .. }
            | Self::UpdateExistingAspects { .. }
            | Self::VerifyThenDeleteExistingAspects { .. }
            | Self::AssertExistingAspects { .. }
            | Self::VerifyExistingAspects { .. }
            | Self::UpdateSymbolicAspects { .. }
            | Self::DeleteAspects { .. }
            | Self::DeleteExistingAspects { .. }
            | Self::DeleteSymbolicAspects { .. }
            | Self::Delete { .. } => &[],
        }
    }
}
