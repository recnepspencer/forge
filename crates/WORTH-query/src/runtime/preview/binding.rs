use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryPreviewHandleBindingFamily {
    LiveView,
    ComputedView,
    Effect,
}

impl WorthQueryPreviewHandleBindingFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LiveView => "live-view",
            Self::ComputedView => "computed-view",
            Self::Effect => "effect",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryPreviewEffectBindingDisposition {
    MutedByDeriveOnly,
    Muted,
    RedirectedDelivery,
    SandboxedWriteIntent,
    AuthoritativeAllowed,
}

impl WorthQueryPreviewEffectBindingDisposition {
    pub(super) fn from_policy(
        policy: WorthQueryEffectPolicy,
        action: WorthQueryEffectAction,
        target_lane: WorthQueryAuthorityLane,
    ) -> Result<Self, WorthQueryRuntimeError> {
        match policy {
            WorthQueryEffectPolicy::DeriveOnly => Ok(Self::MutedByDeriveOnly),
            WorthQueryEffectPolicy::Muted => Ok(Self::Muted),
            WorthQueryEffectPolicy::Redirected => policy
                .admit(action, WorthQueryAuthorityLane::PreviewTruth)
                .map(|_| Self::RedirectedDelivery)
                .map_err(WorthQueryRuntimeError::EffectPolicyDenied),
            WorthQueryEffectPolicy::SandboxedWriteIntent => policy
                .admit(action, WorthQueryAuthorityLane::PreviewTruth)
                .map(|_| Self::SandboxedWriteIntent)
                .map_err(WorthQueryRuntimeError::EffectPolicyDenied),
            WorthQueryEffectPolicy::AuthoritativeAllowed => policy
                .admit(action, target_lane)
                .map(|_| Self::AuthoritativeAllowed)
                .map_err(WorthQueryRuntimeError::EffectPolicyDenied),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::MutedByDeriveOnly => "muted-by-derive-only",
            Self::Muted => "muted",
            Self::RedirectedDelivery => "redirected-delivery",
            Self::SandboxedWriteIntent => "sandboxed-write-intent",
            Self::AuthoritativeAllowed => "authoritative-allowed",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPreviewHandleBindingEvidence {
    label: WorthQuerySessionLabel,
    handle_name: String,
    pub(super) family: WorthQueryPreviewHandleBindingFamily,
    source_lane: WorthQueryAuthorityLane,
    preview_lane: WorthQueryAuthorityLane,
    effect_policy: WorthQueryEffectPolicy,
    effect_disposition: Option<WorthQueryPreviewEffectBindingDisposition>,
    basis_evidence: Vec<String>,
    effect_delivery_admitted: bool,
    pending_write_intent_admitted: bool,
    authoritative_side_effect_admitted: bool,
}

impl WorthQueryPreviewHandleBindingEvidence {
    pub(super) fn live_view(
        label: &WorthQuerySessionLabel,
        handle_name: &str,
        effect_policy: WorthQueryEffectPolicy,
        basis_evidence: &[String],
    ) -> Self {
        Self {
            label: label.clone(),
            handle_name: handle_name.to_string(),
            family: WorthQueryPreviewHandleBindingFamily::LiveView,
            source_lane: WorthQueryAuthorityLane::AuthoritativeTruth,
            preview_lane: WorthQueryAuthorityLane::PreviewTruth,
            effect_policy,
            effect_disposition: None,
            basis_evidence: basis_evidence.to_vec(),
            effect_delivery_admitted: false,
            pending_write_intent_admitted: false,
            authoritative_side_effect_admitted: false,
        }
    }

    pub(super) fn computed(
        label: &WorthQuerySessionLabel,
        handle_name: &str,
        effect_policy: WorthQueryEffectPolicy,
        basis_evidence: &[String],
    ) -> Self {
        Self {
            label: label.clone(),
            handle_name: handle_name.to_string(),
            family: WorthQueryPreviewHandleBindingFamily::ComputedView,
            source_lane: WorthQueryAuthorityLane::DerivedRuntimeState,
            preview_lane: WorthQueryAuthorityLane::PreviewTruth,
            effect_policy,
            effect_disposition: None,
            basis_evidence: basis_evidence.to_vec(),
            effect_delivery_admitted: false,
            pending_write_intent_admitted: false,
            authoritative_side_effect_admitted: false,
        }
    }

    pub(super) fn effect(
        label: &WorthQuerySessionLabel,
        handle_name: &str,
        source_lane: WorthQueryAuthorityLane,
        effect_policy: WorthQueryEffectPolicy,
        disposition: WorthQueryPreviewEffectBindingDisposition,
        basis_evidence: &[String],
    ) -> Self {
        Self {
            label: label.clone(),
            handle_name: handle_name.to_string(),
            family: WorthQueryPreviewHandleBindingFamily::Effect,
            source_lane,
            preview_lane: WorthQueryAuthorityLane::PreviewTruth,
            effect_policy,
            effect_disposition: Some(disposition),
            basis_evidence: basis_evidence.to_vec(),
            effect_delivery_admitted: disposition
                == WorthQueryPreviewEffectBindingDisposition::RedirectedDelivery,
            pending_write_intent_admitted: disposition
                == WorthQueryPreviewEffectBindingDisposition::SandboxedWriteIntent,
            authoritative_side_effect_admitted: disposition
                == WorthQueryPreviewEffectBindingDisposition::AuthoritativeAllowed,
        }
    }

    pub fn label(&self) -> &str {
        self.label.display()
    }

    pub fn session_label(&self) -> &WorthQuerySessionLabel {
        &self.label
    }

    pub fn label_identity(&self) -> &crate::evidence_identity::WorthQueryEvidenceIdentity {
        self.label.identity_digest()
    }

    pub fn handle_name(&self) -> &str {
        &self.handle_name
    }

    pub fn family(&self) -> WorthQueryPreviewHandleBindingFamily {
        self.family
    }

    pub fn source_lane(&self) -> WorthQueryAuthorityLane {
        self.source_lane
    }

    pub fn preview_lane(&self) -> WorthQueryAuthorityLane {
        self.preview_lane
    }

    pub fn effect_policy(&self) -> WorthQueryEffectPolicy {
        self.effect_policy
    }

    pub fn effect_disposition(&self) -> Option<WorthQueryPreviewEffectBindingDisposition> {
        self.effect_disposition
    }

    pub fn basis_evidence(&self) -> &[String] {
        &self.basis_evidence
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
}
