use crate::runtime::{ForgeQueryAuthorityLane, ForgeQueryEffectPolicy};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryRuntimeBackendPosture {
    Primary,
    Scaffold,
}

impl ForgeQueryRuntimeBackendPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Scaffold => "scaffold",
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

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryRuntimeFamilyTeachingPosture {
    OrdinaryRuntimeDx,
    VisibleButDeferred,
    VisibleVocabularyOnly,
    SupportGateOnly,
}

impl ForgeQueryRuntimeFamilyTeachingPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OrdinaryRuntimeDx => "ordinary-runtime-dx",
            Self::VisibleButDeferred => "visible-but-deferred",
            Self::VisibleVocabularyOnly => "visible-vocabulary-only",
            Self::SupportGateOnly => "support-gate-only",
        }
    }
}

impl std::fmt::Display for ForgeQueryRuntimeFamilyTeachingPosture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimeFamilySupport {
    family: ForgeQueryRuntimeFacadeFamily,
    status: ForgeQueryRuntimeFamilySupportStatus,
    teaching_posture: ForgeQueryRuntimeFamilyTeachingPosture,
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
        Self::supported_with_teaching_posture(
            family,
            ForgeQueryRuntimeFamilyTeachingPosture::OrdinaryRuntimeDx,
            authority_lanes,
            effect_policies,
            evidence,
        )
    }

    pub fn supported_with_teaching_posture(
        family: ForgeQueryRuntimeFacadeFamily,
        teaching_posture: ForgeQueryRuntimeFamilyTeachingPosture,
        authority_lanes: impl IntoIterator<Item = ForgeQueryAuthorityLane>,
        effect_policies: impl IntoIterator<Item = ForgeQueryEffectPolicy>,
        evidence: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            family,
            status: ForgeQueryRuntimeFamilySupportStatus::Supported,
            teaching_posture,
            authority_lanes: authority_lanes.into_iter().collect(),
            effect_policies: effect_policies.into_iter().collect(),
            evidence: evidence.into_iter().map(Into::into).collect(),
            denial_reason: None,
        }
    }

    pub fn unsupported(family: ForgeQueryRuntimeFacadeFamily, reason: impl Into<String>) -> Self {
        Self::unsupported_with_evidence(family, reason, std::iter::empty::<String>())
    }

    pub fn unsupported_with_evidence(
        family: ForgeQueryRuntimeFacadeFamily,
        reason: impl Into<String>,
        evidence: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            family,
            status: ForgeQueryRuntimeFamilySupportStatus::Unsupported,
            teaching_posture: ForgeQueryRuntimeFamilyTeachingPosture::VisibleVocabularyOnly,
            authority_lanes: Vec::new(),
            effect_policies: Vec::new(),
            evidence: evidence.into_iter().map(Into::into).collect(),
            denial_reason: Some(reason.into()),
        }
    }

    pub fn deferred(family: ForgeQueryRuntimeFacadeFamily, reason: impl Into<String>) -> Self {
        Self {
            family,
            status: ForgeQueryRuntimeFamilySupportStatus::DeferredDebt,
            teaching_posture: ForgeQueryRuntimeFamilyTeachingPosture::VisibleButDeferred,
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

    pub fn teaching_posture(&self) -> ForgeQueryRuntimeFamilyTeachingPosture {
        self.teaching_posture
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

    pub fn ordinary_downstream_dx(&self) -> bool {
        self.teaching_posture == ForgeQueryRuntimeFamilyTeachingPosture::OrdinaryRuntimeDx
    }

    pub fn owner_closure(&self) -> &'static str {
        match (self.family, self.status) {
            (
                ForgeQueryRuntimeFacadeFamily::Read
                | ForgeQueryRuntimeFacadeFamily::Live
                | ForgeQueryRuntimeFacadeFamily::Computed
                | ForgeQueryRuntimeFacadeFamily::Effect
                | ForgeQueryRuntimeFacadeFamily::BranchPreview
                | ForgeQueryRuntimeFacadeFamily::Write
                | ForgeQueryRuntimeFacadeFamily::Inspect,
                ForgeQueryRuntimeFamilySupportStatus::Supported,
            ) => "Milestone 9.3",
            (
                ForgeQueryRuntimeFacadeFamily::Intent,
                ForgeQueryRuntimeFamilySupportStatus::Unsupported,
            ) => "Milestone 9.x intent-authority-adapter",
            (ForgeQueryRuntimeFacadeFamily::Temporal, _) => "Milestone 9.4",
            (ForgeQueryRuntimeFacadeFamily::AsyncResource, _) => "Milestone 9.4",
            (ForgeQueryRuntimeFacadeFamily::MixedCauseDelivery, _) => "Milestone 9.4",
            (ForgeQueryRuntimeFacadeFamily::StoreBackedExecution, _) => "Milestone 10",
            (ForgeQueryRuntimeFacadeFamily::DurableArtifacts, _) => "Milestone 11",
            _ => "current-runtime-support-profile",
        }
    }

    pub fn extension_rule(&self) -> &'static str {
        match (self.family, self.teaching_posture) {
            (
                ForgeQueryRuntimeFacadeFamily::Temporal
                | ForgeQueryRuntimeFacadeFamily::AsyncResource
                | ForgeQueryRuntimeFacadeFamily::MixedCauseDelivery,
                ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly,
            ) => "must-extend-stabilized-handle-state-lane-aspect-inspection-facade",
            (_, ForgeQueryRuntimeFamilyTeachingPosture::OrdinaryRuntimeDx) => {
                "stable-runtime-backed-handle-state-lane-aspect-inspection-facade"
            }
            (_, ForgeQueryRuntimeFamilyTeachingPosture::VisibleButDeferred) => {
                "must-extend-stabilized-handle-state-lane-aspect-inspection-facade"
            }
            (_, ForgeQueryRuntimeFamilyTeachingPosture::VisibleVocabularyOnly) => {
                "must-admit-through-runtime-support-profile-before-public-use"
            }
            (_, ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly) => {
                "support-matrix-only-certification-gate"
            }
        }
    }

    pub fn parallel_api_forbidden(&self) -> bool {
        true
    }

    pub fn admission_fail_closed(&self) -> bool {
        !matches!(self.status, ForgeQueryRuntimeFamilySupportStatus::Supported)
            || self.teaching_posture == ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly
    }
}
