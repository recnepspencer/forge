use crate::identity::hash_parts;
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
    execution_provenance_chain_digest: String,
}

impl ForgeQueryIntentExecutionProvenance {
    pub(in crate::runtime) fn for_authoritative_binding(
        binding: &ForgeQueryAuthoritativeIntentExecutionBinding,
        execution_outcome_digest: &str,
        snapshot_token: &str,
    ) -> Self {
        Self::new(
            binding.family(),
            binding.entrypoint(),
            binding.execution_seam(),
            binding.handoff().decision_digest(),
            binding.handoff().handoff_digest(),
            binding.binding_digest(),
            execution_outcome_digest,
            snapshot_token,
        )
    }

    pub(in crate::runtime) fn for_effect_binding(
        binding: &ForgeQueryEffectTriggeredIntentExecutionBinding,
        execution_outcome_digest: &str,
        snapshot_token: &str,
    ) -> Self {
        Self::new(
            binding.family(),
            binding.entrypoint(),
            binding.execution_seam(),
            binding.handoff().decision_digest(),
            binding.handoff().handoff_digest(),
            binding.binding_digest(),
            execution_outcome_digest,
            snapshot_token,
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
        snapshot_token: &str,
    ) -> Self {
        Self::new(
            family,
            entrypoint,
            execution_seam,
            admission_decision_digest,
            execution_handoff_digest,
            execution_binding_digest,
            execution_outcome_digest,
            snapshot_token,
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
        snapshot_token: &str,
    ) -> Self {
        let execution_provenance_chain_digest = hash_parts(&[
            "forge_query_intent_execution_provenance_chain_v2".to_string(),
            format!("family:{}", family.as_str()),
            format!("entrypoint:{}", entrypoint.as_str()),
            format!("seam:{}", execution_seam.as_str()),
            format!("decision:{admission_decision_digest}"),
            format!("handoff:{execution_handoff_digest}"),
            format!("binding:{execution_binding_digest}"),
            format!("outcome:{execution_outcome_digest}"),
            format!("snapshot:{snapshot_token}"),
        ]);
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
        &self.execution_provenance_chain_digest
    }
}
