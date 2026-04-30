use serde_json::Value;

use crate::runtime::mutation::{
    command_declared_aspect_operations, command_declared_aspect_paths,
    ForgeQueryContinuityMutationIntent, ForgeQueryMutationMetadata, ForgeQueryNamingMutationIntent,
};
use crate::runtime::{
    ForgeQueryAspectMutationOperation, ForgeQueryAspectValue, ForgeQueryExistingTruthTargetBinding,
    ForgeQuerySymbolicTargetReference,
};

#[derive(Clone, Debug, PartialEq)]
pub enum ForgeQueryWriteCommand {
    #[deprecated(
        note = "payload-first insert is a compatibility path; prefer workspace.insert(...) or preview.insert(...) with aspect-native authoring"
    )]
    Insert {
        collection: String,
        payload: Value,
    },
    InsertAspects {
        collection: String,
        aspects: Vec<ForgeQueryAspectValue>,
        metadata: ForgeQueryMutationMetadata,
        naming_intent: Option<ForgeQueryNamingMutationIntent>,
        continuity_intent: Option<ForgeQueryContinuityMutationIntent>,
        symbolic_target_reference: Option<ForgeQuerySymbolicTargetReference>,
    },
    UpdateAspect {
        entity_identity: String,
        aspect_path: String,
        value: Value,
    },
    UpdateAspects {
        entity_identity: String,
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
        entity_identity: String,
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
        entity_identity: String,
    },
}

impl ForgeQueryWriteCommand {
    #[allow(deprecated)]
    pub(crate) fn declared_aspect_paths(&self) -> Vec<String> {
        command_declared_aspect_paths(self)
    }

    pub(crate) fn declared_aspect_operations(&self) -> Vec<ForgeQueryAspectMutationOperation> {
        command_declared_aspect_operations(self)
    }

    #[allow(deprecated)]
    pub(crate) fn mutation_family(&self) -> ForgeQueryMutationFamily {
        match self {
            Self::Insert { .. } | Self::InsertAspects { .. } => ForgeQueryMutationFamily::Insert,
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

    #[allow(deprecated)]
    pub(crate) fn declared_collection(&self) -> Option<String> {
        match self {
            Self::Insert { collection, .. } | Self::InsertAspects { collection, .. } => {
                Some(collection.clone())
            }
            Self::UpdateAspect { .. }
            | Self::UpdateAspects { .. }
            | Self::UpdateExistingAspects { .. }
            | Self::VerifyThenUpdateExistingAspects { .. }
            | Self::Delete { .. } => None,
            Self::VerifyThenDeleteExistingAspects { binding, .. } => {
                binding.target_collection().map(str::to_string)
            }
            Self::AssertExistingAspects { binding, .. } => {
                binding.target_collection().map(str::to_string)
            }
            Self::VerifyExistingAspects { binding, .. } => {
                binding.target_collection().map(str::to_string)
            }
            Self::UpdateSymbolicAspects { reference, .. }
            | Self::DeleteSymbolicAspects { reference, .. } => {
                reference.target_collection().map(str::to_string)
            }
            Self::DeleteAspects {
                declared_collection,
                ..
            } => declared_collection.clone(),
            Self::DeleteExistingAspects { binding, .. } => {
                binding.target_collection().map(str::to_string)
            }
        }
    }

    #[allow(deprecated)]
    pub(crate) fn declared_entity_identity(&self) -> Option<String> {
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
            Self::VerifyThenUpdateExistingAspects { binding, .. } => {
                Some(binding.resolved_target_identity().to_string())
            }
            Self::VerifyThenDeleteExistingAspects { binding, .. } => {
                Some(binding.resolved_target_identity().to_string())
            }
            Self::AssertExistingAspects { binding, .. } => {
                Some(binding.resolved_target_identity().to_string())
            }
            Self::VerifyExistingAspects { binding, .. } => {
                Some(binding.resolved_target_identity().to_string())
            }
            Self::UpdateSymbolicAspects { .. } | Self::DeleteSymbolicAspects { .. } => None,
            Self::UpdateExistingAspects { .. }
            | Self::DeleteExistingAspects { .. }
            | Self::Insert { .. }
            | Self::InsertAspects { .. } => None,
        }
    }

    #[allow(deprecated)]
    pub(crate) fn mutation_metadata(&self) -> ForgeQueryMutationMetadata {
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
            | Self::DeleteSymbolicAspects { metadata, .. } => metadata.clone(),
            Self::Insert { .. } | Self::UpdateAspect { .. } | Self::Delete { .. } => {
                ForgeQueryMutationMetadata::default()
            }
        }
    }

    pub(crate) fn existing_truth_binding(&self) -> Option<&ForgeQueryExistingTruthTargetBinding> {
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

    pub(crate) fn symbolic_target_reference(&self) -> Option<&ForgeQuerySymbolicTargetReference> {
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

    #[allow(deprecated)]
    pub(crate) fn naming_intent(&self) -> Option<&ForgeQueryNamingMutationIntent> {
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
            Self::AssertExistingAspects { .. } | Self::VerifyExistingAspects { .. } => None,
            Self::Insert { .. } | Self::UpdateAspect { .. } | Self::Delete { .. } => None,
        }
    }

    #[allow(deprecated)]
    pub(crate) fn continuity_intent(&self) -> Option<&ForgeQueryContinuityMutationIntent> {
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
            Self::VerifyThenDeleteExistingAspects { .. } => None,
            Self::AssertExistingAspects { .. } | Self::VerifyExistingAspects { .. } => None,
            Self::Insert { .. }
            | Self::UpdateAspect { .. }
            | Self::DeleteAspects { .. }
            | Self::DeleteExistingAspects { .. }
            | Self::DeleteSymbolicAspects { .. }
            | Self::Delete { .. } => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryMutationFamily {
    Insert,
    Update,
    Assertion,
    Delete,
}

impl ForgeQueryMutationFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Insert => "insert",
            Self::Update => "update",
            Self::Assertion => "assertion",
            Self::Delete => "delete",
        }
    }
}

impl std::fmt::Display for ForgeQueryMutationFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
