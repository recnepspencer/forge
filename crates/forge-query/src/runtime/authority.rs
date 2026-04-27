#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryAuthorityLane {
    AuthoritativeTruth,
    BranchLocalTruth,
    PreviewTruth,
    DerivedRuntimeState,
    EffectDeliveryState,
    PendingWriteIntent,
    BridgeExternalState,
}

impl ForgeQueryAuthorityLane {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeTruth => "authoritative-truth",
            Self::BranchLocalTruth => "branch-local-truth",
            Self::PreviewTruth => "preview-truth",
            Self::DerivedRuntimeState => "derived-runtime-state",
            Self::EffectDeliveryState => "effect-delivery-state",
            Self::PendingWriteIntent => "pending-write-intent",
            Self::BridgeExternalState => "bridge-external-state",
        }
    }
}

impl std::fmt::Display for ForgeQueryAuthorityLane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryEffectAction {
    Derive,
    Deliver,
    WriteIntent,
}

impl ForgeQueryEffectAction {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Derive => "derive",
            Self::Deliver => "deliver",
            Self::WriteIntent => "write-intent",
        }
    }
}

impl std::fmt::Display for ForgeQueryEffectAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryEffectPolicy {
    #[default]
    DeriveOnly,
    Muted,
    Redirected,
    SandboxedWriteIntent,
    AuthoritativeAllowed,
}

impl ForgeQueryEffectPolicy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DeriveOnly => "derive-only",
            Self::Muted => "muted",
            Self::Redirected => "redirected",
            Self::SandboxedWriteIntent => "sandboxed-write-intent",
            Self::AuthoritativeAllowed => "authoritative-allowed",
        }
    }

    pub fn admit(
        self,
        action: ForgeQueryEffectAction,
        target_lane: ForgeQueryAuthorityLane,
    ) -> Result<ForgeQueryEffectAdmission, ForgeQueryEffectPolicyDenial> {
        let admitted = match self {
            Self::DeriveOnly => {
                action == ForgeQueryEffectAction::Derive
                    && target_lane == ForgeQueryAuthorityLane::DerivedRuntimeState
            }
            Self::Muted => false,
            Self::Redirected => match action {
                ForgeQueryEffectAction::Derive => {
                    target_lane == ForgeQueryAuthorityLane::DerivedRuntimeState
                }
                ForgeQueryEffectAction::Deliver => matches!(
                    target_lane,
                    ForgeQueryAuthorityLane::EffectDeliveryState
                        | ForgeQueryAuthorityLane::PreviewTruth
                        | ForgeQueryAuthorityLane::BranchLocalTruth
                ),
                ForgeQueryEffectAction::WriteIntent => false,
            },
            Self::SandboxedWriteIntent => match action {
                ForgeQueryEffectAction::Derive => {
                    target_lane == ForgeQueryAuthorityLane::DerivedRuntimeState
                }
                ForgeQueryEffectAction::WriteIntent => matches!(
                    target_lane,
                    ForgeQueryAuthorityLane::PreviewTruth
                        | ForgeQueryAuthorityLane::BranchLocalTruth
                        | ForgeQueryAuthorityLane::PendingWriteIntent
                ),
                ForgeQueryEffectAction::Deliver => false,
            },
            Self::AuthoritativeAllowed => true,
        };

        if admitted {
            Ok(ForgeQueryEffectAdmission {
                policy: self,
                action,
                target_lane,
            })
        } else {
            Err(ForgeQueryEffectPolicyDenial {
                policy: self,
                action,
                target_lane,
            })
        }
    }
}

impl std::fmt::Display for ForgeQueryEffectPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryEffectAdmission {
    policy: ForgeQueryEffectPolicy,
    action: ForgeQueryEffectAction,
    target_lane: ForgeQueryAuthorityLane,
}

impl ForgeQueryEffectAdmission {
    pub fn policy(&self) -> ForgeQueryEffectPolicy {
        self.policy
    }

    pub fn action(&self) -> ForgeQueryEffectAction {
        self.action
    }

    pub fn target_lane(&self) -> ForgeQueryAuthorityLane {
        self.target_lane
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryEffectPolicyDenial {
    policy: ForgeQueryEffectPolicy,
    action: ForgeQueryEffectAction,
    target_lane: ForgeQueryAuthorityLane,
}

impl ForgeQueryEffectPolicyDenial {
    pub fn policy(&self) -> ForgeQueryEffectPolicy {
        self.policy
    }

    pub fn action(&self) -> ForgeQueryEffectAction {
        self.action
    }

    pub fn target_lane(&self) -> ForgeQueryAuthorityLane {
        self.target_lane
    }
}

impl std::fmt::Display for ForgeQueryEffectPolicyDenial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "effect policy `{}` does not admit `{}` into `{}`",
            self.policy, self.action, self.target_lane
        )
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ForgeQueryPreviewOptions {
    effect_policy: ForgeQueryEffectPolicy,
}

impl ForgeQueryPreviewOptions {
    pub fn derive_only() -> Self {
        Self::default()
    }

    pub fn muted() -> Self {
        Self {
            effect_policy: ForgeQueryEffectPolicy::Muted,
        }
    }

    pub fn redirected_delivery() -> Self {
        Self {
            effect_policy: ForgeQueryEffectPolicy::Redirected,
        }
    }

    pub fn sandboxed_write_intent() -> Self {
        Self {
            effect_policy: ForgeQueryEffectPolicy::SandboxedWriteIntent,
        }
    }

    #[allow(dead_code)]
    pub(in crate::runtime) fn with_effect_policy(
        mut self,
        effect_policy: ForgeQueryEffectPolicy,
    ) -> Self {
        self.effect_policy = effect_policy;
        self
    }

    pub fn effect_policy(&self) -> ForgeQueryEffectPolicy {
        self.effect_policy
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ForgeQueryBranchOptions {
    effect_policy: ForgeQueryEffectPolicy,
}

impl ForgeQueryBranchOptions {
    pub fn derive_only() -> Self {
        Self::default()
    }

    pub fn muted() -> Self {
        Self {
            effect_policy: ForgeQueryEffectPolicy::Muted,
        }
    }

    pub fn redirected_delivery() -> Self {
        Self {
            effect_policy: ForgeQueryEffectPolicy::Redirected,
        }
    }

    pub fn sandboxed_write_intent() -> Self {
        Self {
            effect_policy: ForgeQueryEffectPolicy::SandboxedWriteIntent,
        }
    }

    #[allow(dead_code)]
    pub(in crate::runtime) fn with_effect_policy(
        mut self,
        effect_policy: ForgeQueryEffectPolicy,
    ) -> Self {
        self.effect_policy = effect_policy;
        self
    }

    pub fn effect_policy(&self) -> ForgeQueryEffectPolicy {
        self.effect_policy
    }
}
