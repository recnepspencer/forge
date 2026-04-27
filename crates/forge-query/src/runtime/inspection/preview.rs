use crate::identity::hash_parts;

use super::super::{
    ForgeQueryAuthorityLane, ForgeQueryEffectPolicy, ForgeQueryIntentSourceLane,
    ForgeQueryPreviewCloseoutEvidence, ForgeQueryPreviewCloseoutKind,
    ForgeQueryPreviewHandleBindingEvidence, ForgeQueryPreviewHandleBindingFamily,
    ForgeQueryPreviewIntentReceipt, ForgeQueryPreviewOutcome,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPreviewBindingInspection {
    label: String,
    handle_name: String,
    family: ForgeQueryPreviewHandleBindingFamily,
    source_lane: ForgeQueryAuthorityLane,
    preview_lane: ForgeQueryAuthorityLane,
    effect_policy: ForgeQueryEffectPolicy,
    effect_disposition: Option<String>,
    basis_evidence: Vec<String>,
    basis_digest: String,
    effect_delivery_admitted: bool,
    pending_write_intent_admitted: bool,
    authoritative_side_effect_admitted: bool,
    admission_digest: String,
    inspection_digest: String,
}

impl ForgeQueryPreviewBindingInspection {
    pub(in crate::runtime) fn from_binding(
        binding: &ForgeQueryPreviewHandleBindingEvidence,
    ) -> Self {
        let basis_evidence = binding.basis_evidence().to_vec();
        let basis_digest = hash_parts(&[
            "forge_query_preview_binding_basis_v1".to_string(),
            format!("label:{}", binding.label()),
            format!("family:{}", binding.family().as_str()),
            format!("basis:{}", basis_evidence.join("|")),
        ]);
        let effect_disposition = binding
            .effect_disposition()
            .map(|disposition| disposition.as_str().to_string());
        let admission_digest = hash_parts(&[
            "forge_query_preview_binding_admission_v1".to_string(),
            format!("label:{}", binding.label()),
            format!("handle:{}", binding.handle_name()),
            format!("family:{}", binding.family().as_str()),
            format!("source:{}", binding.source_lane()),
            format!("preview:{}", binding.preview_lane()),
            format!("policy:{}", binding.effect_policy().as_str()),
            format!(
                "effect-disposition:{}",
                effect_disposition.as_deref().unwrap_or("none")
            ),
            format!("basis:{basis_digest}"),
            format!(
                "effect-delivery-admitted:{}",
                binding.effect_delivery_admitted()
            ),
            format!(
                "pending-write-intent-admitted:{}",
                binding.pending_write_intent_admitted()
            ),
            format!(
                "authoritative-side-effect-admitted:{}",
                binding.authoritative_side_effect_admitted()
            ),
        ]);
        let inspection_digest = hash_parts(&[
            "forge_query_preview_binding_inspection_v1".to_string(),
            admission_digest.clone(),
        ]);

        Self {
            label: binding.label().to_string(),
            handle_name: binding.handle_name().to_string(),
            family: binding.family(),
            source_lane: binding.source_lane(),
            preview_lane: binding.preview_lane(),
            effect_policy: binding.effect_policy(),
            effect_disposition,
            basis_evidence,
            basis_digest,
            effect_delivery_admitted: binding.effect_delivery_admitted(),
            pending_write_intent_admitted: binding.pending_write_intent_admitted(),
            authoritative_side_effect_admitted: binding.authoritative_side_effect_admitted(),
            admission_digest,
            inspection_digest,
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn handle_name(&self) -> &str {
        &self.handle_name
    }

    pub fn family(&self) -> ForgeQueryPreviewHandleBindingFamily {
        self.family
    }

    pub fn source_lane(&self) -> ForgeQueryAuthorityLane {
        self.source_lane
    }

    pub fn preview_lane(&self) -> ForgeQueryAuthorityLane {
        self.preview_lane
    }

    pub fn effect_policy(&self) -> ForgeQueryEffectPolicy {
        self.effect_policy
    }

    pub fn effect_disposition(&self) -> Option<&str> {
        self.effect_disposition.as_deref()
    }

    pub fn basis_evidence(&self) -> &[String] {
        &self.basis_evidence
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn effect_delivery_admitted(&self) -> bool {
        self.effect_delivery_admitted
    }

    pub fn pending_write_intent_admitted(&self) -> bool {
        self.pending_write_intent_admitted
    }

    pub fn authoritative_side_effect_admitted(&self) -> bool {
        self.authoritative_side_effect_admitted
    }

    pub fn admission_digest(&self) -> &str {
        &self.admission_digest
    }

    pub fn inspection_digest(&self) -> &str {
        &self.inspection_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPreviewOutcomeInspection {
    label: String,
    closeout_kind: ForgeQueryPreviewCloseoutKind,
    effect_policy: ForgeQueryEffectPolicy,
    promoted: bool,
    discarded: bool,
    write_count: usize,
    preview_binding_count: usize,
    live_binding_count: usize,
    computed_binding_count: usize,
    effect_binding_count: usize,
    subscription_residue_count: usize,
    derived_runtime_residue_count: usize,
    effect_delivery_residue_count: usize,
    pending_write_intent_residue_count: usize,
    preview_write_staging_count: usize,
    promoted_write_count: usize,
    authoritative_residue_count: usize,
    source_lane: ForgeQueryAuthorityLane,
    target_lane: ForgeQueryAuthorityLane,
    basis_evidence: Vec<String>,
    basis_digest: String,
    closeout_digest: String,
    residue_digest: String,
    inspection_digest: String,
}

impl ForgeQueryPreviewOutcomeInspection {
    pub(in crate::runtime) fn from_outcome(outcome: &ForgeQueryPreviewOutcome) -> Self {
        Self::from_closeout(
            outcome,
            outcome.closeout_evidence(),
            outcome.promoted(),
            outcome.discarded(),
            outcome.write_count(),
            outcome.preview_binding_count(),
            outcome.closeout_evidence().live_binding_count(),
            outcome.closeout_evidence().computed_binding_count(),
            outcome.effect_binding_count(),
            outcome.closeout_evidence().subscription_residue_count(),
            outcome.closeout_evidence().derived_runtime_residue_count(),
            outcome.effect_delivery_residue_count(),
            outcome.pending_write_intent_residue_count(),
            outcome.closeout_evidence().preview_write_staging_count(),
            outcome.closeout_evidence().promoted_write_count(),
            outcome.authoritative_residue_count(),
            outcome.source_lane(),
            outcome.target_lane(),
        )
    }

    fn from_closeout(
        outcome: &ForgeQueryPreviewOutcome,
        closeout: &ForgeQueryPreviewCloseoutEvidence,
        promoted: bool,
        discarded: bool,
        write_count: usize,
        preview_binding_count: usize,
        live_binding_count: usize,
        computed_binding_count: usize,
        effect_binding_count: usize,
        subscription_residue_count: usize,
        derived_runtime_residue_count: usize,
        effect_delivery_residue_count: usize,
        pending_write_intent_residue_count: usize,
        preview_write_staging_count: usize,
        promoted_write_count: usize,
        authoritative_residue_count: usize,
        source_lane: ForgeQueryAuthorityLane,
        target_lane: ForgeQueryAuthorityLane,
    ) -> Self {
        let basis_evidence = closeout.basis_evidence().to_vec();
        let basis_digest = hash_parts(&[
            "forge_query_preview_outcome_basis_v1".to_string(),
            format!("label:{}", outcome.label()),
            format!("basis:{}", basis_evidence.join("|")),
        ]);
        let residue_digest = hash_parts(&[
            "forge_query_preview_outcome_residue_v1".to_string(),
            format!("label:{}", outcome.label()),
            format!("subscription:{subscription_residue_count}"),
            format!("derived:{derived_runtime_residue_count}"),
            format!("effect-delivery:{effect_delivery_residue_count}"),
            format!("pending-write-intent:{pending_write_intent_residue_count}"),
            format!("preview-staging:{preview_write_staging_count}"),
            format!("promoted-writes:{promoted_write_count}"),
            format!("authoritative:{authoritative_residue_count}"),
        ]);
        let inspection_digest = hash_parts(&[
            "forge_query_preview_outcome_inspection_v1".to_string(),
            format!("label:{}", outcome.label()),
            format!("closeout-kind:{}", closeout.kind().as_str()),
            format!("policy:{}", outcome.effect_policy().as_str()),
            format!("promoted:{promoted}"),
            format!("discarded:{discarded}"),
            format!("write-count:{write_count}"),
            format!("preview-binding-count:{preview_binding_count}"),
            format!("live-binding-count:{live_binding_count}"),
            format!("computed-binding-count:{computed_binding_count}"),
            format!("effect-binding-count:{effect_binding_count}"),
            format!("source:{source_lane}"),
            format!("target:{target_lane}"),
            format!("basis:{basis_digest}"),
            format!("closeout:{}", closeout.closeout_digest()),
            format!("residue:{residue_digest}"),
        ]);

        Self {
            label: outcome.label().to_string(),
            closeout_kind: closeout.kind(),
            effect_policy: outcome.effect_policy(),
            promoted,
            discarded,
            write_count,
            preview_binding_count,
            live_binding_count,
            computed_binding_count,
            effect_binding_count,
            subscription_residue_count,
            derived_runtime_residue_count,
            effect_delivery_residue_count,
            pending_write_intent_residue_count,
            preview_write_staging_count,
            promoted_write_count,
            authoritative_residue_count,
            source_lane,
            target_lane,
            basis_evidence,
            basis_digest,
            closeout_digest: closeout.closeout_digest().to_string(),
            residue_digest,
            inspection_digest,
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }
    pub fn closeout_kind(&self) -> ForgeQueryPreviewCloseoutKind {
        self.closeout_kind
    }
    pub fn effect_policy(&self) -> ForgeQueryEffectPolicy {
        self.effect_policy
    }
    pub fn promoted(&self) -> bool {
        self.promoted
    }
    pub fn discarded(&self) -> bool {
        self.discarded
    }
    pub fn write_count(&self) -> usize {
        self.write_count
    }
    pub fn preview_binding_count(&self) -> usize {
        self.preview_binding_count
    }
    pub fn live_binding_count(&self) -> usize {
        self.live_binding_count
    }
    pub fn computed_binding_count(&self) -> usize {
        self.computed_binding_count
    }
    pub fn effect_binding_count(&self) -> usize {
        self.effect_binding_count
    }
    pub fn subscription_residue_count(&self) -> usize {
        self.subscription_residue_count
    }
    pub fn derived_runtime_residue_count(&self) -> usize {
        self.derived_runtime_residue_count
    }
    pub fn effect_delivery_residue_count(&self) -> usize {
        self.effect_delivery_residue_count
    }
    pub fn pending_write_intent_residue_count(&self) -> usize {
        self.pending_write_intent_residue_count
    }
    pub fn preview_write_staging_count(&self) -> usize {
        self.preview_write_staging_count
    }
    pub fn promoted_write_count(&self) -> usize {
        self.promoted_write_count
    }
    pub fn authoritative_residue_count(&self) -> usize {
        self.authoritative_residue_count
    }
    pub fn source_lane(&self) -> ForgeQueryAuthorityLane {
        self.source_lane
    }
    pub fn target_lane(&self) -> ForgeQueryAuthorityLane {
        self.target_lane
    }
    pub fn basis_evidence(&self) -> &[String] {
        &self.basis_evidence
    }
    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }
    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }
    pub fn residue_digest(&self) -> &str {
        &self.residue_digest
    }
    pub fn inspection_digest(&self) -> &str {
        &self.inspection_digest
    }
}

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
