use crate::memory_workspace::ForgeQueryEntityIdentity;
use crate::runtime::{
    ForgeQueryAspectMutationOperation, ForgeQueryAspectMutationOperationKind,
    ForgeQueryAspectTouch, ForgeQueryAspectValue, ForgeQueryContinuityMutationIntent,
    ForgeQueryExistingTruthTargetBinding, ForgeQueryMutationFamily, ForgeQueryMutationMetadata,
    ForgeQueryMutationTargetCollectionIdentity, ForgeQueryNamingMutationIntent,
    ForgeQuerySymbolicAspectReference, ForgeQuerySymbolicTargetReference, ForgeQueryWriteCommand,
};

#[path = "backend_admissible/shape.rs"]
mod shape;

use shape::ForgeQueryBackendAdmissibleMutationShape;

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryBackendAdmissibleMutation {
    shape: ForgeQueryBackendAdmissibleMutationShape,
}

impl ForgeQueryBackendAdmissibleMutation {
    pub(crate) fn from_admitted_command(command: ForgeQueryWriteCommand) -> Self {
        Self {
            shape: ForgeQueryBackendAdmissibleMutationShape::from_admitted_command(command),
        }
    }

    pub fn mutation_family(&self) -> ForgeQueryMutationFamily {
        match &self.shape {
            ForgeQueryBackendAdmissibleMutationShape::Insert { .. } => {
                ForgeQueryMutationFamily::Insert
            }
            ForgeQueryBackendAdmissibleMutationShape::UpdateDirect { .. }
            | ForgeQueryBackendAdmissibleMutationShape::UpdateExisting { .. }
            | ForgeQueryBackendAdmissibleMutationShape::UpdateSymbolic { .. } => {
                ForgeQueryMutationFamily::Update
            }
            ForgeQueryBackendAdmissibleMutationShape::Assertion { .. } => {
                ForgeQueryMutationFamily::Assertion
            }
            ForgeQueryBackendAdmissibleMutationShape::DeleteDirect { .. }
            | ForgeQueryBackendAdmissibleMutationShape::DeleteExisting { .. }
            | ForgeQueryBackendAdmissibleMutationShape::DeleteSymbolic { .. } => {
                ForgeQueryMutationFamily::Delete
            }
        }
    }

    pub fn declared_collection_identity(
        &self,
    ) -> Option<ForgeQueryMutationTargetCollectionIdentity> {
        match &self.shape {
            ForgeQueryBackendAdmissibleMutationShape::Insert { collection, .. } => {
                Some(ForgeQueryMutationTargetCollectionIdentity::new(
                    "backend-admissible-declared",
                    collection,
                ))
            }
            ForgeQueryBackendAdmissibleMutationShape::UpdateExisting {
                binding,
                symbolic_aspect_references,
                ..
            } => binding.target_collection_identity().cloned().or_else(|| {
                symbolic_aspect_references
                    .first()
                    .and_then(|reference| reference.reference().target_collection_identity())
                    .cloned()
            }),
            ForgeQueryBackendAdmissibleMutationShape::Assertion { binding, .. }
            | ForgeQueryBackendAdmissibleMutationShape::DeleteExisting { binding, .. } => {
                binding.target_collection_identity().cloned()
            }
            ForgeQueryBackendAdmissibleMutationShape::UpdateSymbolic { reference, .. }
            | ForgeQueryBackendAdmissibleMutationShape::DeleteSymbolic { reference, .. } => {
                reference.target_collection_identity().cloned()
            }
            ForgeQueryBackendAdmissibleMutationShape::DeleteDirect {
                declared_collection,
                ..
            } => declared_collection.as_ref().map(|collection| {
                ForgeQueryMutationTargetCollectionIdentity::new(
                    "backend-admissible-declared",
                    collection,
                )
            }),
            ForgeQueryBackendAdmissibleMutationShape::UpdateDirect { .. } => None,
        }
    }

    pub fn declared_entity_identity_ref(&self) -> Option<&ForgeQueryEntityIdentity> {
        match &self.shape {
            ForgeQueryBackendAdmissibleMutationShape::UpdateDirect {
                entity_identity, ..
            }
            | ForgeQueryBackendAdmissibleMutationShape::DeleteDirect {
                entity_identity, ..
            } => Some(entity_identity),
            _ => None,
        }
    }

    pub fn declared_entity_identity(&self) -> Option<ForgeQueryEntityIdentity> {
        match &self.shape {
            ForgeQueryBackendAdmissibleMutationShape::UpdateExisting { binding, .. }
            | ForgeQueryBackendAdmissibleMutationShape::DeleteExisting { binding, .. }
            | ForgeQueryBackendAdmissibleMutationShape::Assertion { binding, .. } => {
                Some(binding.resolved_entity_artifact_identity())
            }
            _ => self.declared_entity_identity_ref().cloned(),
        }
    }

    pub fn existing_truth_binding(&self) -> Option<&ForgeQueryExistingTruthTargetBinding> {
        match &self.shape {
            ForgeQueryBackendAdmissibleMutationShape::UpdateExisting { binding, .. }
            | ForgeQueryBackendAdmissibleMutationShape::Assertion { binding, .. }
            | ForgeQueryBackendAdmissibleMutationShape::DeleteExisting { binding, .. } => {
                Some(binding)
            }
            _ => None,
        }
    }

    pub fn symbolic_target_reference(&self) -> Option<&ForgeQuerySymbolicTargetReference> {
        match &self.shape {
            ForgeQueryBackendAdmissibleMutationShape::Insert {
                symbolic_target_reference,
                ..
            } => symbolic_target_reference.as_ref(),
            ForgeQueryBackendAdmissibleMutationShape::UpdateSymbolic { reference, .. }
            | ForgeQueryBackendAdmissibleMutationShape::DeleteSymbolic { reference, .. } => {
                Some(reference)
            }
            _ => None,
        }
    }

    pub fn admitted_aspect_values(&self) -> &[ForgeQueryAspectValue] {
        match &self.shape {
            ForgeQueryBackendAdmissibleMutationShape::Insert { aspects, .. }
            | ForgeQueryBackendAdmissibleMutationShape::UpdateDirect { aspects, .. }
            | ForgeQueryBackendAdmissibleMutationShape::UpdateExisting { aspects, .. }
            | ForgeQueryBackendAdmissibleMutationShape::Assertion { aspects, .. }
            | ForgeQueryBackendAdmissibleMutationShape::UpdateSymbolic { aspects, .. } => aspects,
            ForgeQueryBackendAdmissibleMutationShape::DeleteDirect { .. }
            | ForgeQueryBackendAdmissibleMutationShape::DeleteExisting { .. }
            | ForgeQueryBackendAdmissibleMutationShape::DeleteSymbolic { .. } => &[],
        }
    }

    pub fn asserted_admitted_aspect_values(&self) -> &[ForgeQueryAspectValue] {
        match &self.shape {
            ForgeQueryBackendAdmissibleMutationShape::UpdateExisting {
                asserted_aspects, ..
            }
            | ForgeQueryBackendAdmissibleMutationShape::DeleteExisting {
                asserted_aspects, ..
            } => asserted_aspects,
            _ => &[],
        }
    }

    pub fn symbolic_aspect_references(&self) -> &[ForgeQuerySymbolicAspectReference] {
        match &self.shape {
            ForgeQueryBackendAdmissibleMutationShape::Insert {
                symbolic_aspect_references,
                ..
            }
            | ForgeQueryBackendAdmissibleMutationShape::UpdateExisting {
                symbolic_aspect_references,
                ..
            } => symbolic_aspect_references,
            _ => &[],
        }
    }

    pub fn admitted_touched_aspects(&self) -> &[ForgeQueryAspectTouch] {
        match &self.shape {
            ForgeQueryBackendAdmissibleMutationShape::DeleteDirect {
                touched_aspects, ..
            }
            | ForgeQueryBackendAdmissibleMutationShape::DeleteExisting {
                touched_aspects, ..
            }
            | ForgeQueryBackendAdmissibleMutationShape::DeleteSymbolic {
                touched_aspects, ..
            } => touched_aspects,
            _ => &[],
        }
    }

    pub fn declared_aspect_touches(&self) -> Vec<ForgeQueryAspectTouch> {
        self.declared_aspect_operations()
            .into_iter()
            .map(|operation| operation.aspect_touch().clone())
            .collect()
    }

    pub fn declared_aspect_operations(&self) -> Vec<ForgeQueryAspectMutationOperation> {
        match self.mutation_family() {
            ForgeQueryMutationFamily::Insert
            | ForgeQueryMutationFamily::Update
            | ForgeQueryMutationFamily::Assertion => self
                .admitted_aspect_values()
                .iter()
                .map(ForgeQueryAspectValue::declared_operation)
                .chain(self.symbolic_aspect_references().iter().map(|reference| {
                    ForgeQueryAspectMutationOperation::from_touch(
                        reference.aspect_touch().clone(),
                        ForgeQueryAspectMutationOperationKind::Set,
                    )
                }))
                .collect(),
            ForgeQueryMutationFamily::Delete => self
                .admitted_touched_aspects()
                .iter()
                .cloned()
                .map(ForgeQueryAspectMutationOperation::clear)
                .collect(),
        }
    }

    pub fn mutation_metadata_ref(&self) -> Option<&ForgeQueryMutationMetadata> {
        match &self.shape {
            ForgeQueryBackendAdmissibleMutationShape::Insert { metadata, .. }
            | ForgeQueryBackendAdmissibleMutationShape::UpdateDirect { metadata, .. }
            | ForgeQueryBackendAdmissibleMutationShape::UpdateExisting { metadata, .. }
            | ForgeQueryBackendAdmissibleMutationShape::Assertion { metadata, .. }
            | ForgeQueryBackendAdmissibleMutationShape::UpdateSymbolic { metadata, .. }
            | ForgeQueryBackendAdmissibleMutationShape::DeleteDirect { metadata, .. }
            | ForgeQueryBackendAdmissibleMutationShape::DeleteExisting { metadata, .. }
            | ForgeQueryBackendAdmissibleMutationShape::DeleteSymbolic { metadata, .. } => {
                Some(metadata)
            }
        }
    }

    pub fn mutation_metadata(&self) -> ForgeQueryMutationMetadata {
        self.mutation_metadata_ref().cloned().unwrap_or_default()
    }

    pub fn naming_intent(&self) -> Option<&ForgeQueryNamingMutationIntent> {
        match &self.shape {
            ForgeQueryBackendAdmissibleMutationShape::Insert { naming_intent, .. }
            | ForgeQueryBackendAdmissibleMutationShape::UpdateDirect { naming_intent, .. }
            | ForgeQueryBackendAdmissibleMutationShape::UpdateExisting { naming_intent, .. }
            | ForgeQueryBackendAdmissibleMutationShape::UpdateSymbolic { naming_intent, .. }
            | ForgeQueryBackendAdmissibleMutationShape::DeleteDirect { naming_intent, .. }
            | ForgeQueryBackendAdmissibleMutationShape::DeleteExisting { naming_intent, .. }
            | ForgeQueryBackendAdmissibleMutationShape::DeleteSymbolic { naming_intent, .. } => {
                naming_intent.as_ref()
            }
            ForgeQueryBackendAdmissibleMutationShape::Assertion { .. } => None,
        }
    }

    pub fn continuity_intent(&self) -> Option<&ForgeQueryContinuityMutationIntent> {
        match &self.shape {
            ForgeQueryBackendAdmissibleMutationShape::Insert {
                continuity_intent, ..
            }
            | ForgeQueryBackendAdmissibleMutationShape::UpdateDirect {
                continuity_intent, ..
            }
            | ForgeQueryBackendAdmissibleMutationShape::UpdateExisting {
                continuity_intent, ..
            }
            | ForgeQueryBackendAdmissibleMutationShape::UpdateSymbolic {
                continuity_intent, ..
            } => continuity_intent.as_ref(),
            ForgeQueryBackendAdmissibleMutationShape::Assertion { .. }
            | ForgeQueryBackendAdmissibleMutationShape::DeleteDirect { .. }
            | ForgeQueryBackendAdmissibleMutationShape::DeleteExisting { .. }
            | ForgeQueryBackendAdmissibleMutationShape::DeleteSymbolic { .. } => None,
        }
    }
}
