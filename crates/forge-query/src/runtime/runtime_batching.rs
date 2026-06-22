use crate::evidence_identity::ForgeQueryEvidenceIdentity;
use crate::memory_workspace::ForgeQueryEntityIdentity;
use crate::memory_workspace::{ForgeQueryWorkspaceError, ForgeQueryWorkspaceErrorKind};
use crate::runtime::{
    ForgeQueryAspectMutationOperation, ForgeQueryContinuityMutationIntent,
    ForgeQueryExistingTruthTargetBinding, ForgeQueryMutationFamily, ForgeQueryMutationMetadata,
    ForgeQueryMutationTargetCollectionIdentity, ForgeQueryNamingMutationIntent,
    ForgeQueryRuntimeBackendPosture, ForgeQueryRuntimeError, ForgeQueryRuntimeSupportProfile,
    ForgeQuerySymbolicAspectReference, ForgeQuerySymbolicAspectResolutionEvidence,
    ForgeQuerySymbolicTargetReference, ForgeQueryVerifiedExistingTruthAssertion,
    ForgeQueryWriteCommand,
};

pub(crate) struct BatchCommandSummary {
    mutation_family: ForgeQueryMutationFamily,
    declared_collection_identity: Option<ForgeQueryMutationTargetCollectionIdentity>,
    declared_entity_identity: Option<ForgeQueryEntityIdentity>,
    existing_truth_binding: Option<ForgeQueryExistingTruthTargetBinding>,
    verified_existing_truth_assertion: Option<ForgeQueryVerifiedExistingTruthAssertion>,
    symbolic_target_reference: Option<ForgeQuerySymbolicTargetReference>,
    naming_intent: Option<ForgeQueryNamingMutationIntent>,
    continuity_intent: Option<ForgeQueryContinuityMutationIntent>,
    declared_aspect_operations: Vec<ForgeQueryAspectMutationOperation>,
    declared_aspect_value_digest: Option<ForgeQueryEvidenceIdentity>,
    symbolic_aspect_references: Vec<ForgeQuerySymbolicAspectReference>,
    symbolic_aspect_resolution_evidence: Vec<ForgeQuerySymbolicAspectResolutionEvidence>,
    mutation_metadata: ForgeQueryMutationMetadata,
}

impl BatchCommandSummary {
    pub(crate) fn new(
        mutation_family: ForgeQueryMutationFamily,
        declared_collection_identity: Option<ForgeQueryMutationTargetCollectionIdentity>,
        declared_entity_identity: Option<ForgeQueryEntityIdentity>,
        existing_truth_binding: Option<ForgeQueryExistingTruthTargetBinding>,
        verified_existing_truth_assertion: Option<ForgeQueryVerifiedExistingTruthAssertion>,
        symbolic_target_reference: Option<ForgeQuerySymbolicTargetReference>,
        naming_intent: Option<ForgeQueryNamingMutationIntent>,
        continuity_intent: Option<ForgeQueryContinuityMutationIntent>,
        declared_aspect_operations: Vec<ForgeQueryAspectMutationOperation>,
        declared_aspect_value_digest: Option<ForgeQueryEvidenceIdentity>,
        symbolic_aspect_references: Vec<ForgeQuerySymbolicAspectReference>,
        symbolic_aspect_resolution_evidence: Vec<ForgeQuerySymbolicAspectResolutionEvidence>,
        mutation_metadata: ForgeQueryMutationMetadata,
    ) -> Self {
        Self {
            mutation_family,
            declared_collection_identity,
            declared_entity_identity,
            existing_truth_binding,
            verified_existing_truth_assertion,
            symbolic_target_reference,
            naming_intent,
            continuity_intent,
            declared_aspect_operations,
            declared_aspect_value_digest,
            symbolic_aspect_references,
            symbolic_aspect_resolution_evidence,
            mutation_metadata,
        }
    }

    pub(crate) fn with_symbolic_aspect_resolution_evidence(
        mut self,
        symbolic_aspect_resolution_evidence: Vec<ForgeQuerySymbolicAspectResolutionEvidence>,
    ) -> Self {
        self.symbolic_aspect_resolution_evidence = symbolic_aspect_resolution_evidence;
        self
    }

    pub(crate) fn mutation_family(&self) -> ForgeQueryMutationFamily {
        self.mutation_family
    }

    pub(crate) fn declared_collection_identity(
        &self,
    ) -> Option<ForgeQueryMutationTargetCollectionIdentity> {
        self.declared_collection_identity.clone()
    }

    pub(crate) fn declared_entity_identity(&self) -> Option<ForgeQueryEntityIdentity> {
        self.declared_entity_identity.clone()
    }

    pub(crate) fn existing_truth_binding(&self) -> Option<ForgeQueryExistingTruthTargetBinding> {
        self.existing_truth_binding.clone()
    }

    pub(crate) fn verified_existing_truth_assertion(
        &self,
    ) -> Option<ForgeQueryVerifiedExistingTruthAssertion> {
        self.verified_existing_truth_assertion.clone()
    }

    pub(crate) fn symbolic_target_reference(&self) -> Option<ForgeQuerySymbolicTargetReference> {
        self.symbolic_target_reference.clone()
    }

    pub(crate) fn naming_intent(&self) -> Option<ForgeQueryNamingMutationIntent> {
        self.naming_intent.clone()
    }

    pub(crate) fn continuity_intent(&self) -> Option<ForgeQueryContinuityMutationIntent> {
        self.continuity_intent.clone()
    }

    pub(crate) fn declared_aspect_operations(&self) -> Vec<ForgeQueryAspectMutationOperation> {
        self.declared_aspect_operations.clone()
    }

    pub(crate) fn declared_aspect_value_digest(&self) -> Option<ForgeQueryEvidenceIdentity> {
        self.declared_aspect_value_digest.clone()
    }

    pub(crate) fn symbolic_aspect_resolution_evidence(
        &self,
    ) -> Vec<ForgeQuerySymbolicAspectResolutionEvidence> {
        self.symbolic_aspect_resolution_evidence.clone()
    }

    pub(crate) fn symbolic_aspect_references(&self) -> Vec<ForgeQuerySymbolicAspectReference> {
        self.symbolic_aspect_references.clone()
    }

    pub(crate) fn mutation_metadata(&self) -> ForgeQueryMutationMetadata {
        self.mutation_metadata.clone()
    }
}

pub(crate) fn should_use_backend_atomic_batch(
    support_profile: &ForgeQueryRuntimeSupportProfile,
    commands: &[ForgeQueryWriteCommand],
) -> bool {
    support_profile.posture() == ForgeQueryRuntimeBackendPosture::Primary && commands.len() > 1
}

pub(crate) fn deny_scaffold_multi_command_batch_without_atomic_authority(
    support_profile: &ForgeQueryRuntimeSupportProfile,
    commands: &[ForgeQueryWriteCommand],
) -> Result<(), ForgeQueryRuntimeError> {
    if support_profile.posture() == ForgeQueryRuntimeBackendPosture::Scaffold && commands.len() > 1
    {
        return Err(ForgeQueryRuntimeError::Workspace(
            ForgeQueryWorkspaceError::with_kind(
                ForgeQueryWorkspaceErrorKind::BatchAtomicityUnsupported,
                "scaffold runtime backends deny multi-command batches before execution unless they advertise an atomic batch authority",
            ),
        ));
    }
    Ok(())
}
