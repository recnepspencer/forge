use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

use super::artifact_policy::ForgeQueryGraphObligationArtifactPolicy;
use super::preflight_witness::ForgeQueryGraphObligationPreflightWitness;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationExecutionContext {
    artifact_policy: ForgeQueryGraphObligationArtifactPolicy,
    preflight_witness: ForgeQueryGraphObligationPreflightWitness,
    context_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryGraphObligationExecutionContext {
    pub fn bounded() -> Self {
        Self::default()
    }

    pub fn with_artifact_policy(
        self,
        artifact_policy: ForgeQueryGraphObligationArtifactPolicy,
    ) -> Self {
        Self::new(artifact_policy, self.preflight_witness)
    }

    pub fn with_preflight_witness(
        self,
        preflight_witness: ForgeQueryGraphObligationPreflightWitness,
    ) -> Self {
        Self::new(self.artifact_policy, preflight_witness)
    }

    pub fn artifact_policy(&self) -> ForgeQueryGraphObligationArtifactPolicy {
        self.artifact_policy
    }

    pub fn preflight_witness(&self) -> &ForgeQueryGraphObligationPreflightWitness {
        &self.preflight_witness
    }

    pub fn context_digest(&self) -> &str {
        self.context_digest.as_str()
    }

    pub(crate) fn context_evidence_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.context_digest
    }

    fn new(
        artifact_policy: ForgeQueryGraphObligationArtifactPolicy,
        preflight_witness: ForgeQueryGraphObligationPreflightWitness,
    ) -> Self {
        let context_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::GraphObligationExecutionContext)
                .field_shape(
                    ForgeQueryEvidenceTag::new("artifact_policy"),
                    artifact_policy.as_str(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("preflight_witness"),
                    preflight_witness.as_str(),
                )
                .optional_value(
                    ForgeQueryEvidenceTag::new("preflight_witness_digest"),
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

impl Default for ForgeQueryGraphObligationExecutionContext {
    fn default() -> Self {
        Self::new(
            ForgeQueryGraphObligationArtifactPolicy::default(),
            ForgeQueryGraphObligationPreflightWitness::default(),
        )
    }
}
