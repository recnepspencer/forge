use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

use super::{
    ForgeQueryEffectAdmission, ForgeQueryEffectPolicy, ForgeQueryIntentDeclaration,
    ForgeQueryPreviewBasisAdmission,
};

pub(super) fn preview_intent_admission_identity(
    declaration: &ForgeQueryIntentDeclaration,
    effect_policy: ForgeQueryEffectPolicy,
    basis_admission: &ForgeQueryPreviewBasisAdmission,
    admission: ForgeQueryEffectAdmission,
    canonical_input_digest: &str,
) -> ForgeQueryEvidenceIdentity {
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
        .field_value(
            ForgeQueryEvidenceTag::new("canonical_input_digest"),
            canonical_input_digest,
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
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("basis_admission_identity"),
            basis_admission.admission_identity(),
        )
        .seal()
}

pub(super) fn preview_intent_receipt_identity(
    admission_identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::PreviewIntentReceipt)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("admission_identity"),
            admission_identity,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("posture"),
            "preview-local-staged-no-authoritative-execution",
        )
        .seal()
}
