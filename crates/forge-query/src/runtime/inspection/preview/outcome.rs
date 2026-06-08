use crate::identity::hash_parts;

use super::super::super::{
    ForgeQueryAuthorityLane, ForgeQueryEffectPolicy, ForgeQueryPreviewCloseoutEvidence,
    ForgeQueryPreviewCloseoutKind, ForgeQueryPreviewOutcome,
};

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
    temporal_wake_residue_count: usize,
    async_result_residue_count: usize,
    mixed_cause_residue_count: usize,
    crossed_authoritative_residue_count: usize,
    authoritative_residue_count: usize,
    source_lane: ForgeQueryAuthorityLane,
    target_lane: ForgeQueryAuthorityLane,
    basis_evidence: Vec<String>,
    basis_digest: String,
    preview_basis_snapshot_token: String,
    target_basis_snapshot_token: String,
    closeout_digest: String,
    residue_digest: String,
    rebinding_digest: Option<String>,
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
            outcome.closeout_evidence().temporal_wake_residue_count(),
            outcome.closeout_evidence().async_result_residue_count(),
            outcome.closeout_evidence().mixed_cause_residue_count(),
            outcome
                .closeout_evidence()
                .crossed_authoritative_residue_count(),
            outcome.authoritative_residue_count(),
            outcome.source_lane(),
            outcome.target_lane(),
        )
    }

    #[allow(clippy::too_many_arguments)]
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
        temporal_wake_residue_count: usize,
        async_result_residue_count: usize,
        mixed_cause_residue_count: usize,
        crossed_authoritative_residue_count: usize,
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
            format!("temporal-wake:{temporal_wake_residue_count}"),
            format!("async-result:{async_result_residue_count}"),
            format!("mixed-cause:{mixed_cause_residue_count}"),
            format!("crossed-authoritative:{crossed_authoritative_residue_count}"),
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
            format!(
                "preview-basis-snapshot:{}",
                closeout.preview_basis_snapshot_token()
            ),
            format!(
                "target-basis-snapshot:{}",
                closeout.target_basis_snapshot_token()
            ),
            format!("closeout:{}", closeout.closeout_digest()),
            format!("residue:{residue_digest}"),
            format!(
                "rebinding:{}",
                closeout.rebinding_digest().unwrap_or("none")
            ),
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
            temporal_wake_residue_count,
            async_result_residue_count,
            mixed_cause_residue_count,
            crossed_authoritative_residue_count,
            authoritative_residue_count,
            source_lane,
            target_lane,
            basis_evidence,
            basis_digest,
            preview_basis_snapshot_token: closeout.preview_basis_snapshot_token().to_string(),
            target_basis_snapshot_token: closeout.target_basis_snapshot_token().to_string(),
            closeout_digest: closeout.closeout_digest().to_string(),
            residue_digest,
            rebinding_digest: closeout.rebinding_digest().map(str::to_string),
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
    pub fn temporal_wake_residue_count(&self) -> usize {
        self.temporal_wake_residue_count
    }
    pub fn async_result_residue_count(&self) -> usize {
        self.async_result_residue_count
    }
    pub fn mixed_cause_residue_count(&self) -> usize {
        self.mixed_cause_residue_count
    }
    pub fn crossed_authoritative_residue_count(&self) -> usize {
        self.crossed_authoritative_residue_count
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
    pub fn preview_basis_snapshot_token(&self) -> &str {
        &self.preview_basis_snapshot_token
    }
    pub fn target_basis_snapshot_token(&self) -> &str {
        &self.target_basis_snapshot_token
    }
    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }
    pub fn residue_digest(&self) -> &str {
        &self.residue_digest
    }
    pub fn rebinding_digest(&self) -> Option<&str> {
        self.rebinding_digest.as_deref()
    }
    pub fn inspection_digest(&self) -> &str {
        &self.inspection_digest
    }
}
