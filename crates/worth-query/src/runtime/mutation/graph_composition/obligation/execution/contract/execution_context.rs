use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};

use super::artifact_policy::WorthQueryGraphObligationArtifactPolicy;
use super::preflight_witness::WorthQueryGraphObligationPreflightWitness;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationExecutionContext {
    artifact_policy: WorthQueryGraphObligationArtifactPolicy,
    preflight_witness: WorthQueryGraphObligationPreflightWitness,
    context_digest: WorthQueryEvidenceIdentity,
}

impl WorthQueryGraphObligationExecutionContext {
    pub fn bounded() -> Self {
        Self::default()
    }

    pub fn with_artifact_policy(
        self,
        artifact_policy: WorthQueryGraphObligationArtifactPolicy,
    ) -> Self {
        Self::new(artifact_policy, self.preflight_witness)
    }

    pub fn with_preflight_witness(
        self,
        preflight_witness: WorthQueryGraphObligationPreflightWitness,
    ) -> Self {
        Self::new(self.artifact_policy, preflight_witness)
    }

    pub fn artifact_policy(&self) -> WorthQueryGraphObligationArtifactPolicy {
        self.artifact_policy
    }

    pub fn preflight_witness(&self) -> &WorthQueryGraphObligationPreflightWitness {
        &self.preflight_witness
    }

    pub fn context_digest(&self) -> &str {
        self.context_digest.as_str()
    }

    pub(crate) fn context_evidence_digest(&self) -> &WorthQueryEvidenceIdentity {
        &self.context_digest
    }

    fn new(
        artifact_policy: WorthQueryGraphObligationArtifactPolicy,
        preflight_witness: WorthQueryGraphObligationPreflightWitness,
    ) -> Self {
        let context_digest =
            worth_query_evidence_identity(WorthQueryEvidenceScope::GraphObligationExecutionContext)
                .field_shape(
                    WorthQueryEvidenceTag::new("artifact_policy"),
                    artifact_policy.as_str(),
                )
                .field_shape(
                    WorthQueryEvidenceTag::new("preflight_witness"),
                    preflight_witness.as_str(),
                )
                .optional_value(
                    WorthQueryEvidenceTag::new("preflight_witness_digest"),
                    preflight_witness.witness_digest(),
                )
                .seal();
        Self {
            artifact_policy,
            preflight_witness,
            context_digest,
        }
    }
}

impl Default for WorthQueryGraphObligationExecutionContext {
    fn default() -> Self {
        Self::new(
            WorthQueryGraphObligationArtifactPolicy::default(),
            WorthQueryGraphObligationPreflightWitness::default(),
        )
    }
}
