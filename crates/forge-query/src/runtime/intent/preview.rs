use super::*;
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPreviewIntentReceipt {
    intent_name: String,
    strategy_identity: String,
    strategy_version: String,
    canonical_input_digest: String,
    source_lane: ForgeQueryIntentSourceLane,
    target_lane: ForgeQueryAuthorityLane,
    effect_policy: ForgeQueryEffectPolicy,
    basis_evidence: Vec<String>,
    admission_digest: ForgeQueryEvidenceIdentity,
    receipt_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryPreviewIntentReceipt {
    pub(in crate::runtime) fn new(
        declaration: &ForgeQueryIntentDeclaration,
        effect_policy: ForgeQueryEffectPolicy,
        basis_admission: &ForgeQueryPreviewBasisAdmission,
        admission: ForgeQueryEffectAdmission,
    ) -> Self {
        let basis_evidence = basis_admission.evidence().to_vec();
        let canonical_input_digest = declaration.input_digest();
        let admission_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::PreviewIntentAdmission)
                .field_shape(
                    ForgeQueryEvidenceTag::new("intent_name"),
                    declaration.name(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("strategy_identity"),
                    declaration.strategy_name(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("strategy_version"),
                    declaration.strategy_version(),
                )
                .field_identity(
                    ForgeQueryEvidenceTag::new("canonical_input_digest"),
                    &canonical_input_digest,
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("source_lane"),
                    declaration.source_lane().as_str(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("target_lane"),
                    declaration.target_lane().as_str(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("effect_policy"),
                    effect_policy.as_str(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("admitted_action"),
                    admission.action().as_str(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("admitted_lane"),
                    admission.target_lane().as_str(),
                )
                .field_identity(
                    ForgeQueryEvidenceTag::new("basis_admission_digest"),
                    basis_admission.admission_digest().as_str(),
                )
                .seal();
        let receipt_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::PreviewIntentReceipt)
                .field_identity(
                    ForgeQueryEvidenceTag::new("admission_digest"),
                    admission_digest.as_str(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("posture"),
                    "preview-local-staged-no-authoritative-execution",
                )
                .seal();
        Self {
            intent_name: declaration.name().to_string(),
            strategy_identity: declaration.strategy_name().to_string(),
            strategy_version: declaration.strategy_version().to_string(),
            canonical_input_digest,
            source_lane: declaration.source_lane(),
            target_lane: declaration.target_lane(),
            effect_policy,
            basis_evidence,
            admission_digest,
            receipt_digest,
        }
    }

    pub fn intent_name(&self) -> &str {
        &self.intent_name
    }

    pub fn strategy_identity(&self) -> &str {
        &self.strategy_identity
    }

    pub fn strategy_version(&self) -> &str {
        &self.strategy_version
    }

    pub fn canonical_input_digest(&self) -> &str {
        &self.canonical_input_digest
    }

    pub fn source_lane(&self) -> ForgeQueryIntentSourceLane {
        self.source_lane
    }

    pub fn target_lane(&self) -> ForgeQueryAuthorityLane {
        self.target_lane
    }

    pub fn effect_policy(&self) -> ForgeQueryEffectPolicy {
        self.effect_policy
    }

    pub fn basis_evidence(&self) -> &[String] {
        &self.basis_evidence
    }

    pub fn admission_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.admission_digest
    }

    pub fn receipt_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.receipt_digest
    }
}
