use super::support::*;
use crate::{ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag};

pub(super) fn compose_preview_intent_receipt_identity(
    receipt: &ForgeQueryPreviewIntentReceipt,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::PreviewIntentReceipt)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("admission_identity"),
            receipt.admission_identity(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("posture"),
            "preview-local-staged-no-authoritative-execution",
        )
        .seal()
}

pub(super) fn compose_preview_intent_receipt_inspection_basis_identity(
    receipt: &ForgeQueryPreviewIntentReceipt,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(
        ForgeQueryEvidenceScope::PreviewIntentReceiptInspectionBasis,
    )
    .field_shape(
        ForgeQueryEvidenceTag::new("intent_name"),
        receipt.intent_name(),
    )
    .field_usize(
        ForgeQueryEvidenceTag::new("basis_evidence_count"),
        receipt.basis_evidence().len(),
    )
    .field_evidence_identity(
        ForgeQueryEvidenceTag::new("basis_evidence"),
        receipt.basis_evidence_identity(),
    )
    .seal()
}

pub(super) fn compose_preview_intent_receipt_inspection_identity(
    receipt: &ForgeQueryPreviewIntentReceipt,
    basis_identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::PreviewIntentReceiptInspection)
        .field_shape(
            ForgeQueryEvidenceTag::new("intent_name"),
            receipt.intent_name(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("strategy_identity"),
            receipt.strategy_identity(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("strategy_version"),
            receipt.strategy_version(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("canonical_input_digest"),
            receipt.canonical_input_digest(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("source_lane"),
            receipt.source_lane().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("target_lane"),
            receipt.target_lane().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("effect_policy"),
            receipt.effect_policy().as_str(),
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("basis_identity"), basis_identity)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("admission_identity"),
            receipt.admission_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("receipt_identity"),
            receipt.receipt_identity(),
        )
        .seal()
}

pub(super) fn compose_preview_intent_receipt_inspection_identity_for_inspection(
    inspection: &ForgeQueryPreviewIntentReceiptInspection,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::PreviewIntentReceiptInspection)
        .field_shape(
            ForgeQueryEvidenceTag::new("intent_name"),
            inspection.intent_name(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("strategy_identity"),
            inspection.strategy_identity(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("strategy_version"),
            inspection.strategy_version(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("canonical_input_digest"),
            inspection.canonical_input_digest(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("source_lane"),
            inspection.source_lane().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("target_lane"),
            inspection.target_lane().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("effect_policy"),
            inspection.effect_policy().as_str(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("basis_identity"),
            inspection.basis_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("admission_identity"),
            inspection.admission_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("receipt_identity"),
            inspection.receipt_identity(),
        )
        .seal()
}
