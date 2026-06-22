use super::provenance_identity::{
    intent_execution_provenance_chain_identity, IntentExecutionProvenanceIdentityParts,
};
use crate::evidence_identity::ForgeQueryEvidenceIdentity;
use crate::intent_admission::{
    ForgeQueryAuthoritativeIntentExecutionBinding, ForgeQueryEffectTriggeredIntentExecutionBinding,
    ForgeQueryIntentAdmissionCoveredEntrypoint, ForgeQueryIntentAdmissionExecutionSeam,
    ForgeQueryIntentAdmissionFamily,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentExecutionProvenance {
    family: ForgeQueryIntentAdmissionFamily,
    entrypoint: ForgeQueryIntentAdmissionCoveredEntrypoint,
    execution_seam: ForgeQueryIntentAdmissionExecutionSeam,
    admission_decision_digest: String,
    execution_handoff_digest: String,
    execution_binding_digest: String,
    execution_provenance_chain_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryIntentExecutionProvenance {
    pub(in crate::runtime) fn for_authoritative_binding(
        binding: &ForgeQueryAuthoritativeIntentExecutionBinding,
        execution_outcome_digest: &str,
        snapshot_evidence_identity: &ForgeQueryEvidenceIdentity,
    ) -> Self {
        Self::new(
            binding.family(),
            binding.entrypoint(),
            binding.execution_seam(),
            binding.handoff().decision_digest(),
            binding.handoff().handoff_digest(),
            binding.binding_digest(),
            execution_outcome_digest,
            snapshot_evidence_identity,
        )
    }

    pub(in crate::runtime) fn for_effect_binding(
        binding: &ForgeQueryEffectTriggeredIntentExecutionBinding,
        execution_outcome_digest: &str,
        snapshot_evidence_identity: &ForgeQueryEvidenceIdentity,
    ) -> Self {
        Self::new(
            binding.family(),
            binding.entrypoint(),
            binding.execution_seam(),
            binding.handoff().decision_digest(),
            binding.handoff().handoff_digest(),
            binding.binding_digest(),
            execution_outcome_digest,
            snapshot_evidence_identity,
        )
    }

    pub(in crate::runtime) fn for_shared_execution_parts(
        family: ForgeQueryIntentAdmissionFamily,
        entrypoint: ForgeQueryIntentAdmissionCoveredEntrypoint,
        execution_seam: ForgeQueryIntentAdmissionExecutionSeam,
        admission_decision_digest: &str,
        execution_handoff_digest: &str,
        execution_binding_digest: &str,
        execution_outcome_digest: &str,
        snapshot_evidence_identity: &ForgeQueryEvidenceIdentity,
    ) -> Self {
        Self::new(
            family,
            entrypoint,
            execution_seam,
            admission_decision_digest,
            execution_handoff_digest,
            execution_binding_digest,
            execution_outcome_digest,
            snapshot_evidence_identity,
        )
    }

    pub(in crate::runtime) fn for_shared_execution_typed_parts(
        family: ForgeQueryIntentAdmissionFamily,
        entrypoint: ForgeQueryIntentAdmissionCoveredEntrypoint,
        execution_seam: ForgeQueryIntentAdmissionExecutionSeam,
        admission_decision_digest: &str,
        execution_handoff_digest: &str,
        execution_binding_digest: &str,
        execution_outcome_digest: &str,
        snapshot_evidence_identity: &ForgeQueryEvidenceIdentity,
    ) -> Self {
        Self::for_shared_execution_parts(
            family,
            entrypoint,
            execution_seam,
            admission_decision_digest,
            execution_handoff_digest,
            execution_binding_digest,
            execution_outcome_digest,
            snapshot_evidence_identity,
        )
    }

    fn new(
        family: ForgeQueryIntentAdmissionFamily,
        entrypoint: ForgeQueryIntentAdmissionCoveredEntrypoint,
        execution_seam: ForgeQueryIntentAdmissionExecutionSeam,
        admission_decision_digest: &str,
        execution_handoff_digest: &str,
        execution_binding_digest: &str,
        execution_outcome_digest: &str,
        snapshot_evidence_identity: &ForgeQueryEvidenceIdentity,
    ) -> Self {
        let execution_provenance_chain_digest =
            intent_execution_provenance_chain_identity(IntentExecutionProvenanceIdentityParts {
                family: family.as_str(),
                entrypoint: entrypoint.as_str(),
                execution_seam: execution_seam.as_str(),
                admission_decision_digest,
                execution_handoff_digest,
                execution_binding_digest,
                execution_outcome_digest,
                snapshot_evidence_identity,
            });
        Self {
            family,
            entrypoint,
            execution_seam,
            admission_decision_digest: admission_decision_digest.to_string(),
            execution_handoff_digest: execution_handoff_digest.to_string(),
            execution_binding_digest: execution_binding_digest.to_string(),
            execution_provenance_chain_digest,
        }
    }

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        self.family
    }

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
        self.entrypoint
    }

    pub fn execution_seam(&self) -> ForgeQueryIntentAdmissionExecutionSeam {
        self.execution_seam
    }

    pub fn admission_decision_digest(&self) -> &str {
        &self.admission_decision_digest
    }

    pub fn execution_handoff_digest(&self) -> &str {
        &self.execution_handoff_digest
    }

    pub fn execution_binding_digest(&self) -> &str {
        &self.execution_binding_digest
    }

    pub fn execution_provenance_chain_digest(&self) -> &str {
        self.execution_provenance_chain_digest.as_str()
    }

    pub fn execution_provenance_chain_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.execution_provenance_chain_digest
    }
}
