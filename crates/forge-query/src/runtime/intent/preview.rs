use super::preview_receipt_identity::{
    preview_intent_admission_identity, preview_intent_receipt_identity,
};
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
    basis_evidence_identity: ForgeQueryEvidenceIdentity,
    admission_identity: ForgeQueryEvidenceIdentity,
    obligation_dispatch: Option<crate::runtime::ForgeQueryAuthoritativeMutationObligationDispatch>,
    receipt_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryPreviewIntentReceipt {
    pub(in crate::runtime) fn new(
        declaration: &ForgeQueryIntentDeclaration,
        effect_policy: ForgeQueryEffectPolicy,
        basis_admission: &ForgeQueryPreviewBasisAdmission,
        admission: ForgeQueryEffectAdmission,
        obligation_dispatch: Option<
            crate::runtime::ForgeQueryAuthoritativeMutationObligationDispatch,
        >,
    ) -> Self {
        let basis_evidence = basis_admission.evidence().to_vec();
        let canonical_input_digest = declaration.input_digest();
        let basis_evidence_identity =
            preview_intent_basis_evidence_identity(declaration.name(), basis_admission);
        let admission_identity = preview_intent_admission_identity(
            declaration,
            effect_policy,
            basis_admission,
            admission,
            &canonical_input_digest,
        );
        let receipt_identity =
            preview_intent_receipt_identity(&admission_identity, obligation_dispatch.as_ref());
        Self {
            intent_name: declaration.name().to_string(),
            strategy_identity: declaration.strategy_name().to_string(),
            strategy_version: declaration.strategy_version().to_string(),
            canonical_input_digest,
            source_lane: declaration.source_lane(),
            target_lane: declaration.target_lane(),
            effect_policy,
            basis_evidence,
            basis_evidence_identity,
            admission_identity,
            obligation_dispatch,
            receipt_identity,
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

    pub fn basis_evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.basis_evidence_identity
    }

    pub fn admission_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.admission_identity
    }

    pub fn admission_digest(&self) -> &str {
        self.admission_identity.as_str()
    }

    pub fn obligation_dispatch(
        &self,
    ) -> Option<&crate::runtime::ForgeQueryAuthoritativeMutationObligationDispatch> {
        self.obligation_dispatch.as_ref()
    }

    pub fn receipt_digest(&self) -> &str {
        self.receipt_identity.as_str()
    }

    pub fn receipt_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.receipt_identity
    }
}

fn preview_intent_basis_evidence_identity(
    intent_name: &str,
    basis_admission: &ForgeQueryPreviewBasisAdmission,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::PreviewIntentBasisEvidence)
        .field_shape(ForgeQueryEvidenceTag::new("intent_name"), intent_name)
        .field_usize(
            ForgeQueryEvidenceTag::new("basis_evidence_count"),
            basis_admission.evidence_rows().len(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("basis_admission_identity"),
            basis_admission.admission_identity(),
        )
        .seal()
}
