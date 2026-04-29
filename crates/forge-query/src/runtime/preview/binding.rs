use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryPreviewHandleBindingFamily {
    LiveView,
    ComputedView,
    Effect,
}

impl ForgeQueryPreviewHandleBindingFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LiveView => "live-view",
            Self::ComputedView => "computed-view",
            Self::Effect => "effect",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryPreviewEffectBindingDisposition {
    MutedByDeriveOnly,
    Muted,
    RedirectedDelivery,
    SandboxedWriteIntent,
    AuthoritativeAllowed,
}

impl ForgeQueryPreviewEffectBindingDisposition {
    pub(super) fn from_policy(
        policy: ForgeQueryEffectPolicy,
        action: ForgeQueryEffectAction,
        target_lane: ForgeQueryAuthorityLane,
    ) -> Result<Self, ForgeQueryRuntimeError> {
        match policy {
            ForgeQueryEffectPolicy::DeriveOnly => Ok(Self::MutedByDeriveOnly),
            ForgeQueryEffectPolicy::Muted => Ok(Self::Muted),
            ForgeQueryEffectPolicy::Redirected => policy
                .admit(action, ForgeQueryAuthorityLane::PreviewTruth)
                .map(|_| Self::RedirectedDelivery)
                .map_err(ForgeQueryRuntimeError::EffectPolicyDenied),
            ForgeQueryEffectPolicy::SandboxedWriteIntent => policy
                .admit(action, ForgeQueryAuthorityLane::PreviewTruth)
                .map(|_| Self::SandboxedWriteIntent)
                .map_err(ForgeQueryRuntimeError::EffectPolicyDenied),
            ForgeQueryEffectPolicy::AuthoritativeAllowed => policy
                .admit(action, target_lane)
                .map(|_| Self::AuthoritativeAllowed)
                .map_err(ForgeQueryRuntimeError::EffectPolicyDenied),
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
pub struct ForgeQueryPreviewHandleBindingEvidence {
    label: String,
    handle_name: String,
    pub(super) family: ForgeQueryPreviewHandleBindingFamily,
    source_lane: ForgeQueryAuthorityLane,
    preview_lane: ForgeQueryAuthorityLane,
    effect_policy: ForgeQueryEffectPolicy,
    effect_disposition: Option<ForgeQueryPreviewEffectBindingDisposition>,
    basis_evidence: Vec<String>,
    effect_delivery_admitted: bool,
    pending_write_intent_admitted: bool,
    authoritative_side_effect_admitted: bool,
}

impl ForgeQueryPreviewHandleBindingEvidence {
    pub(super) fn live_view(
        label: &str,
        handle_name: &str,
        effect_policy: ForgeQueryEffectPolicy,
        basis_evidence: &[String],
    ) -> Self {
        Self {
            label: label.to_string(),
            handle_name: handle_name.to_string(),
            family: ForgeQueryPreviewHandleBindingFamily::LiveView,
            source_lane: ForgeQueryAuthorityLane::AuthoritativeTruth,
            preview_lane: ForgeQueryAuthorityLane::PreviewTruth,
            effect_policy,
            effect_disposition: None,
            basis_evidence: basis_evidence.to_vec(),
            effect_delivery_admitted: false,
            pending_write_intent_admitted: false,
            authoritative_side_effect_admitted: false,
        }
    }

    pub(super) fn computed(
        label: &str,
        handle_name: &str,
        effect_policy: ForgeQueryEffectPolicy,
        basis_evidence: &[String],
    ) -> Self {
        Self {
            label: label.to_string(),
            handle_name: handle_name.to_string(),
            family: ForgeQueryPreviewHandleBindingFamily::ComputedView,
            source_lane: ForgeQueryAuthorityLane::DerivedRuntimeState,
            preview_lane: ForgeQueryAuthorityLane::PreviewTruth,
            effect_policy,
            effect_disposition: None,
            basis_evidence: basis_evidence.to_vec(),
            effect_delivery_admitted: false,
            pending_write_intent_admitted: false,
            authoritative_side_effect_admitted: false,
        }
    }

    pub(super) fn effect(
        label: &str,
        handle_name: &str,
        source_lane: ForgeQueryAuthorityLane,
        effect_policy: ForgeQueryEffectPolicy,
        disposition: ForgeQueryPreviewEffectBindingDisposition,
        basis_evidence: &[String],
    ) -> Self {
        Self {
            label: label.to_string(),
            handle_name: handle_name.to_string(),
            family: ForgeQueryPreviewHandleBindingFamily::Effect,
            source_lane,
            preview_lane: ForgeQueryAuthorityLane::PreviewTruth,
            effect_policy,
            effect_disposition: Some(disposition),
            basis_evidence: basis_evidence.to_vec(),
            effect_delivery_admitted: disposition
                == ForgeQueryPreviewEffectBindingDisposition::RedirectedDelivery,
            pending_write_intent_admitted: disposition
                == ForgeQueryPreviewEffectBindingDisposition::SandboxedWriteIntent,
            authoritative_side_effect_admitted: disposition
                == ForgeQueryPreviewEffectBindingDisposition::AuthoritativeAllowed,
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

    pub fn effect_disposition(&self) -> Option<ForgeQueryPreviewEffectBindingDisposition> {
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
