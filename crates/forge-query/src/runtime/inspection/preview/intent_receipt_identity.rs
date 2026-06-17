use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::runtime::ForgeQueryPreviewIntentReceipt;

pub(super) fn preview_intent_receipt_inspection_basis_identity(
    receipt: &ForgeQueryPreviewIntentReceipt,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::PreviewIntentReceiptInspectionBasis)
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

pub(super) fn preview_intent_receipt_inspection_identity(
    receipt: &ForgeQueryPreviewIntentReceipt,
    basis_identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::PreviewIntentReceiptInspection)
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
