use crate::identity::hash_parts;

use super::super::super::{
    ForgeQueryAuthorityLane, ForgeQueryEffectPolicy, ForgeQueryIntentSourceLane,
    ForgeQueryPreviewIntentReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPreviewIntentReceiptInspection {
    intent_name: String,
    strategy_identity: String,
    strategy_version: String,
    canonical_input_digest: String,
    source_lane: ForgeQueryIntentSourceLane,
    target_lane: ForgeQueryAuthorityLane,
    effect_policy: ForgeQueryEffectPolicy,
    basis_evidence: Vec<String>,
    basis_digest: String,
    admission_digest: String,
    receipt_digest: String,
    inspection_digest: String,
}

impl ForgeQueryPreviewIntentReceiptInspection {
    pub(in crate::runtime) fn from_receipt(receipt: &ForgeQueryPreviewIntentReceipt) -> Self {
        let basis_evidence = receipt.basis_evidence().to_vec();
        let basis_digest = hash_parts(&[
            "forge_query_preview_intent_receipt_basis_v1".to_string(),
            format!("intent:{}", receipt.intent_name()),
            format!("basis:{}", basis_evidence.join("|")),
        ]);
        let inspection_digest = hash_parts(&[
            "forge_query_preview_intent_receipt_inspection_v1".to_string(),
            format!("intent:{}", receipt.intent_name()),
            format!("strategy:{}", receipt.strategy_identity()),
            format!("version:{}", receipt.strategy_version()),
            format!("input:{}", receipt.canonical_input_digest()),
            format!("source:{}", receipt.source_lane().as_str()),
            format!("target:{}", receipt.target_lane()),
            format!("policy:{}", receipt.effect_policy().as_str()),
            format!("basis:{basis_digest}"),
            format!("admission:{}", receipt.admission_digest()),
            format!("receipt:{}", receipt.receipt_digest()),
        ]);
        Self {
            intent_name: receipt.intent_name().to_string(),
            strategy_identity: receipt.strategy_identity().to_string(),
            strategy_version: receipt.strategy_version().to_string(),
            canonical_input_digest: receipt.canonical_input_digest().to_string(),
            source_lane: receipt.source_lane(),
            target_lane: receipt.target_lane(),
            effect_policy: receipt.effect_policy(),
            basis_evidence,
            basis_digest,
            admission_digest: receipt.admission_digest().to_string(),
            receipt_digest: receipt.receipt_digest().to_string(),
            inspection_digest,
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
    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }
    pub fn admission_digest(&self) -> &str {
        &self.admission_digest
    }
    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }
    pub fn inspection_digest(&self) -> &str {
        &self.inspection_digest
    }
}
