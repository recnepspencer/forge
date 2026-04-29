use crate::runtime::{ForgeQueryAuthorityLane, ForgeQueryEffectPolicy};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryRuntimeBackendPosture {
    Primary,
    Compatibility,
}

impl ForgeQueryRuntimeBackendPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Compatibility => "compatibility",
        }
    }
}

impl std::fmt::Display for ForgeQueryRuntimeBackendPosture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryRuntimeFacadeFamily {
    Read,
    Live,
    Computed,
    Effect,
    BranchPreview,
    Write,
    Intent,
    Inspect,
    Temporal,
    AsyncResource,
    MixedCauseDelivery,
    StoreBackedExecution,
    DurableArtifacts,
}

impl ForgeQueryRuntimeFacadeFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Read => "read",
            Self::Live => "live",
            Self::Computed => "computed",
            Self::Effect => "effect",
            Self::BranchPreview => "branch-preview",
            Self::Write => "write",
            Self::Intent => "intent",
            Self::Inspect => "inspect",
            Self::Temporal => "temporal",
            Self::AsyncResource => "async-resource",
            Self::MixedCauseDelivery => "mixed-cause-delivery",
            Self::StoreBackedExecution => "store-backed-execution",
            Self::DurableArtifacts => "durable-artifacts",
        }
    }
}

impl std::fmt::Display for ForgeQueryRuntimeFacadeFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryRuntimeFamilySupportStatus {
    Supported,
    DeferredDebt,
    Unsupported,
}

impl ForgeQueryRuntimeFamilySupportStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::DeferredDebt => "deferred-debt",
            Self::Unsupported => "unsupported",
        }
    }
}

impl std::fmt::Display for ForgeQueryRuntimeFamilySupportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimeFamilySupport {
    family: ForgeQueryRuntimeFacadeFamily,
    status: ForgeQueryRuntimeFamilySupportStatus,
    authority_lanes: Vec<ForgeQueryAuthorityLane>,
    effect_policies: Vec<ForgeQueryEffectPolicy>,
    evidence: Vec<String>,
    denial_reason: Option<String>,
}

impl ForgeQueryRuntimeFamilySupport {
    pub fn supported(
        family: ForgeQueryRuntimeFacadeFamily,
        authority_lanes: impl IntoIterator<Item = ForgeQueryAuthorityLane>,
        effect_policies: impl IntoIterator<Item = ForgeQueryEffectPolicy>,
        evidence: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            family,
            status: ForgeQueryRuntimeFamilySupportStatus::Supported,
            authority_lanes: authority_lanes.into_iter().collect(),
            effect_policies: effect_policies.into_iter().collect(),
            evidence: evidence.into_iter().map(Into::into).collect(),
            denial_reason: None,
        }
    }

    pub fn unsupported(family: ForgeQueryRuntimeFacadeFamily, reason: impl Into<String>) -> Self {
        Self {
            family,
            status: ForgeQueryRuntimeFamilySupportStatus::Unsupported,
            authority_lanes: Vec::new(),
            effect_policies: Vec::new(),
            evidence: Vec::new(),
            denial_reason: Some(reason.into()),
        }
    }

    pub fn deferred(family: ForgeQueryRuntimeFacadeFamily, reason: impl Into<String>) -> Self {
        Self {
            family,
            status: ForgeQueryRuntimeFamilySupportStatus::DeferredDebt,
            authority_lanes: Vec::new(),
            effect_policies: Vec::new(),
            evidence: Vec::new(),
            denial_reason: Some(reason.into()),
        }
    }

    pub fn family(&self) -> ForgeQueryRuntimeFacadeFamily {
        self.family
    }

    pub fn status(&self) -> ForgeQueryRuntimeFamilySupportStatus {
        self.status
    }

    pub fn authority_lanes(&self) -> &[ForgeQueryAuthorityLane] {
        &self.authority_lanes
    }

    pub fn effect_policies(&self) -> &[ForgeQueryEffectPolicy] {
        &self.effect_policies
    }

    pub fn evidence(&self) -> &[String] {
        &self.evidence
    }

    pub fn denial_reason(&self) -> Option<&str> {
        self.denial_reason.as_deref()
    }
}
