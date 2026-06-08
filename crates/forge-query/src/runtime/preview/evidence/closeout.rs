use super::super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryPreviewCloseoutKind {
    Discarded,
    Promoted,
}

impl ForgeQueryPreviewCloseoutKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Discarded => "discarded",
            Self::Promoted => "promoted",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryPreviewResidueClass {
    SubscriptionState,
    DerivedRuntimeState,
    EffectDeliveryState,
    PendingWriteIntent,
    PreviewWriteStaging,
    TemporalWakeState,
    AsyncResultState,
    MixedCauseState,
    CrossedAuthoritativeResidue,
    AuthoritativeResidue,
}

impl ForgeQueryPreviewResidueClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SubscriptionState => "subscription-state",
            Self::DerivedRuntimeState => "derived-runtime-state",
            Self::EffectDeliveryState => "effect-delivery-state",
            Self::PendingWriteIntent => "pending-write-intent",
            Self::PreviewWriteStaging => "preview-write-staging",
            Self::TemporalWakeState => "temporal-wake-state",
            Self::AsyncResultState => "async-result-state",
            Self::MixedCauseState => "mixed-cause-state",
            Self::CrossedAuthoritativeResidue => "crossed-authoritative-residue",
            Self::AuthoritativeResidue => "authoritative-residue",
        }
    }

    pub fn is_authoritative(self) -> bool {
        matches!(
            self,
            Self::CrossedAuthoritativeResidue | Self::AuthoritativeResidue
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPreviewCloseoutEvidence {
    label: String,
    kind: ForgeQueryPreviewCloseoutKind,
    effect_policy: ForgeQueryEffectPolicy,
    basis_evidence: Vec<String>,
    preview_basis_snapshot_token: String,
    target_basis_snapshot_token: String,
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
    rebinding_digest: Option<String>,
    closeout_digest: String,
}

impl ForgeQueryPreviewCloseoutEvidence {
    #[allow(clippy::too_many_arguments)]
    pub(in crate::runtime::preview) fn new(
        label: &str,
        kind: ForgeQueryPreviewCloseoutKind,
        effect_policy: ForgeQueryEffectPolicy,
        basis_admission: &ForgeQueryPreviewBasisAdmission,
        preview_basis_snapshot_token: &str,
        target_basis_snapshot_token: &str,
        preview_binding_count: usize,
        live_binding_count: usize,
        computed_binding_count: usize,
        effect_binding_count: usize,
        subscription_residue_count: usize,
        derived_runtime_residue_count: usize,
        preview_write_staging_count: usize,
        promoted_write_count: usize,
        temporal_wake_residue_count: usize,
        async_result_residue_count: usize,
        mixed_cause_residue_count: usize,
        crossed_authoritative_residue_count: usize,
        effect_delivery_residue_count: usize,
        pending_write_intent_residue_count: usize,
        authoritative_residue_count: usize,
        rebinding_digest: Option<String>,
    ) -> Self {
        let basis_evidence = basis_admission.evidence().to_vec();
        let closeout_digest = hash_parts(&[
            "forge_query_preview_closeout_evidence_v1".to_string(),
            format!("label:{label}"),
            format!("kind:{}", kind.as_str()),
            format!("policy:{}", effect_policy.as_str()),
            format!("basis_label:{}", basis_admission.label()),
            format!("basis_lane:{}", basis_admission.authority_lane()),
            format!("basis_evidence:{}", basis_evidence.join("|")),
            format!("preview_basis_snapshot:{preview_basis_snapshot_token}"),
            format!("target_basis_snapshot:{target_basis_snapshot_token}"),
            format!("preview_bindings:{preview_binding_count}"),
            format!("live_bindings:{live_binding_count}"),
            format!("computed_bindings:{computed_binding_count}"),
            format!("effect_bindings:{effect_binding_count}"),
            format!("subscription_residue:{subscription_residue_count}"),
            format!("derived_residue:{derived_runtime_residue_count}"),
            format!("effect_delivery_residue:{effect_delivery_residue_count}"),
            format!("pending_write_intent_residue:{pending_write_intent_residue_count}"),
            format!("preview_write_staging:{preview_write_staging_count}"),
            format!("promoted_writes:{promoted_write_count}"),
            format!("temporal_wake_residue:{temporal_wake_residue_count}"),
            format!("async_result_residue:{async_result_residue_count}"),
            format!("mixed_cause_residue:{mixed_cause_residue_count}"),
            format!("crossed_authoritative_residue:{crossed_authoritative_residue_count}"),
            format!("authoritative_residue:{authoritative_residue_count}"),
            format!(
                "rebinding:{}",
                rebinding_digest.as_deref().unwrap_or("none")
            ),
        ]);
        Self {
            label: label.to_string(),
            kind,
            effect_policy,
            basis_evidence,
            preview_basis_snapshot_token: preview_basis_snapshot_token.to_string(),
            target_basis_snapshot_token: target_basis_snapshot_token.to_string(),
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
            rebinding_digest,
            closeout_digest,
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn kind(&self) -> ForgeQueryPreviewCloseoutKind {
        self.kind
    }

    pub fn effect_policy(&self) -> ForgeQueryEffectPolicy {
        self.effect_policy
    }

    pub fn basis_evidence(&self) -> &[String] {
        &self.basis_evidence
    }

    pub fn preview_basis_snapshot_token(&self) -> &str {
        &self.preview_basis_snapshot_token
    }

    pub fn target_basis_snapshot_token(&self) -> &str {
        &self.target_basis_snapshot_token
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

    pub fn rebinding_digest(&self) -> Option<&str> {
        self.rebinding_digest.as_deref()
    }

    pub fn class_count(&self, residue_class: ForgeQueryPreviewResidueClass) -> usize {
        match residue_class {
            ForgeQueryPreviewResidueClass::SubscriptionState => self.subscription_residue_count,
            ForgeQueryPreviewResidueClass::DerivedRuntimeState => {
                self.derived_runtime_residue_count
            }
            ForgeQueryPreviewResidueClass::EffectDeliveryState => {
                self.effect_delivery_residue_count
            }
            ForgeQueryPreviewResidueClass::PendingWriteIntent => {
                self.pending_write_intent_residue_count
            }
            ForgeQueryPreviewResidueClass::PreviewWriteStaging => self.preview_write_staging_count,
            ForgeQueryPreviewResidueClass::TemporalWakeState => self.temporal_wake_residue_count,
            ForgeQueryPreviewResidueClass::AsyncResultState => self.async_result_residue_count,
            ForgeQueryPreviewResidueClass::MixedCauseState => self.mixed_cause_residue_count,
            ForgeQueryPreviewResidueClass::CrossedAuthoritativeResidue => {
                self.crossed_authoritative_residue_count
            }
            ForgeQueryPreviewResidueClass::AuthoritativeResidue => self.authoritative_residue_count,
        }
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }
}
