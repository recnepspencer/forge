use crate::evidence_identity::WorthQueryEvidenceIdentity;
use crate::memory_workspace::WorthQueryEntityIdentity;
use crate::memory_workspace::{WorthQueryWorkspaceError, WorthQueryWorkspaceErrorKind};
use crate::runtime::{
    WorthQueryAspectMutationOperation, WorthQueryContinuityMutationIntent,
    WorthQueryExistingTruthTargetBinding, WorthQueryMutationFamily, WorthQueryMutationMetadata,
    WorthQueryMutationTargetCollectionIdentity, WorthQueryNamingMutationIntent,
    WorthQueryRuntimeBackendPosture, WorthQueryRuntimeError, WorthQueryRuntimeSupportProfile,
    WorthQuerySymbolicAspectReference, WorthQuerySymbolicAspectResolutionEvidence,
    WorthQuerySymbolicTargetReference, WorthQueryVerifiedExistingTruthAssertion,
    WorthQueryWriteCommand,
};

pub(crate) struct BatchCommandSummary {
    mutation_family: WorthQueryMutationFamily,
    declared_collection_identity: Option<WorthQueryMutationTargetCollectionIdentity>,
    declared_entity_identity: Option<WorthQueryEntityIdentity>,
    existing_truth_binding: Option<WorthQueryExistingTruthTargetBinding>,
    verified_existing_truth_assertion: Option<WorthQueryVerifiedExistingTruthAssertion>,
    symbolic_target_reference: Option<WorthQuerySymbolicTargetReference>,
    naming_intent: Option<WorthQueryNamingMutationIntent>,
    continuity_intent: Option<WorthQueryContinuityMutationIntent>,
    declared_aspect_operations: Vec<WorthQueryAspectMutationOperation>,
    declared_aspect_value_digest: Option<WorthQueryEvidenceIdentity>,
    symbolic_aspect_references: Vec<WorthQuerySymbolicAspectReference>,
    symbolic_aspect_resolution_evidence: Vec<WorthQuerySymbolicAspectResolutionEvidence>,
    mutation_metadata: WorthQueryMutationMetadata,
}

impl BatchCommandSummary {
    pub(crate) fn new(
        mutation_family: WorthQueryMutationFamily,
        declared_collection_identity: Option<WorthQueryMutationTargetCollectionIdentity>,
        declared_entity_identity: Option<WorthQueryEntityIdentity>,
        existing_truth_binding: Option<WorthQueryExistingTruthTargetBinding>,
        verified_existing_truth_assertion: Option<WorthQueryVerifiedExistingTruthAssertion>,
        symbolic_target_reference: Option<WorthQuerySymbolicTargetReference>,
        naming_intent: Option<WorthQueryNamingMutationIntent>,
        continuity_intent: Option<WorthQueryContinuityMutationIntent>,
        declared_aspect_operations: Vec<WorthQueryAspectMutationOperation>,
        declared_aspect_value_digest: Option<WorthQueryEvidenceIdentity>,
        symbolic_aspect_references: Vec<WorthQuerySymbolicAspectReference>,
        symbolic_aspect_resolution_evidence: Vec<WorthQuerySymbolicAspectResolutionEvidence>,
        mutation_metadata: WorthQueryMutationMetadata,
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
        symbolic_aspect_resolution_evidence: Vec<WorthQuerySymbolicAspectResolutionEvidence>,
    ) -> Self {
        self.symbolic_aspect_resolution_evidence = symbolic_aspect_resolution_evidence;
        self
    }

    pub(crate) fn mutation_family(&self) -> WorthQueryMutationFamily {
        self.mutation_family
    }

    pub(crate) fn declared_collection_identity(
        &self,
    ) -> Option<WorthQueryMutationTargetCollectionIdentity> {
        self.declared_collection_identity.clone()
    }

    pub(crate) fn declared_entity_identity(&self) -> Option<WorthQueryEntityIdentity> {
        self.declared_entity_identity.clone()
    }

    pub(crate) fn existing_truth_binding(&self) -> Option<WorthQueryExistingTruthTargetBinding> {
        self.existing_truth_binding.clone()
    }

    pub(crate) fn verified_existing_truth_assertion(
        &self,
    ) -> Option<WorthQueryVerifiedExistingTruthAssertion> {
        self.verified_existing_truth_assertion.clone()
    }

    pub(crate) fn symbolic_target_reference(&self) -> Option<WorthQuerySymbolicTargetReference> {
        self.symbolic_target_reference.clone()
    }

    pub(crate) fn naming_intent(&self) -> Option<WorthQueryNamingMutationIntent> {
        self.naming_intent.clone()
    }

    pub(crate) fn continuity_intent(&self) -> Option<WorthQueryContinuityMutationIntent> {
        self.continuity_intent.clone()
    }

    pub(crate) fn declared_aspect_operations(&self) -> Vec<WorthQueryAspectMutationOperation> {
        self.declared_aspect_operations.clone()
    }

    pub(crate) fn declared_aspect_value_digest(&self) -> Option<WorthQueryEvidenceIdentity> {
        self.declared_aspect_value_digest.clone()
    }

    pub(crate) fn symbolic_aspect_resolution_evidence(
        &self,
    ) -> Vec<WorthQuerySymbolicAspectResolutionEvidence> {
        self.symbolic_aspect_resolution_evidence.clone()
    }

    pub(crate) fn symbolic_aspect_references(&self) -> Vec<WorthQuerySymbolicAspectReference> {
        self.symbolic_aspect_references.clone()
    }

    pub(crate) fn mutation_metadata(&self) -> WorthQueryMutationMetadata {
        self.mutation_metadata.clone()
    }
}

pub(crate) fn should_use_backend_atomic_batch(
    support_profile: &WorthQueryRuntimeSupportProfile,
    commands: &[WorthQueryWriteCommand],
) -> bool {
    support_profile.posture() == WorthQueryRuntimeBackendPosture::Primary && commands.len() > 1
}

pub(crate) fn deny_scaffold_multi_command_batch_without_atomic_authority(
    support_profile: &WorthQueryRuntimeSupportProfile,
    commands: &[WorthQueryWriteCommand],
) -> Result<(), WorthQueryRuntimeError> {
    if support_profile.posture() == WorthQueryRuntimeBackendPosture::Scaffold && commands.len() > 1
    {
        return Err(WorthQueryRuntimeError::Workspace(
            WorthQueryWorkspaceError::with_kind(
                WorthQueryWorkspaceErrorKind::BatchAtomicityUnsupported,
                "scaffold runtime backends deny multi-command batches before execution unless they advertise an atomic batch authority",
            ),
        ));
    }
    Ok(())
}
