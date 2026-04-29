use crate::runtime::{ForgeQueryAuthorityLane, ForgeQueryEffectPolicy};

#[derive(Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimeEvidenceAuthority {
    _private: (),
}

impl ForgeQueryRuntimeEvidenceAuthority {
    pub(crate) fn new() -> Self {
        Self { _private: () }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPreviewBasisAdmission {
    label: String,
    effect_policy: ForgeQueryEffectPolicy,
    authority_lane: ForgeQueryAuthorityLane,
    evidence: Vec<String>,
}

impl ForgeQueryPreviewBasisAdmission {
    pub fn new(
        _authority: &ForgeQueryRuntimeEvidenceAuthority,
        label: impl Into<String>,
        effect_policy: ForgeQueryEffectPolicy,
        evidence: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            label: label.into(),
            effect_policy,
            authority_lane: ForgeQueryAuthorityLane::PreviewTruth,
            evidence: evidence.into_iter().map(Into::into).collect(),
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn effect_policy(&self) -> ForgeQueryEffectPolicy {
        self.effect_policy
    }

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.authority_lane
    }

    pub fn evidence(&self) -> &[String] {
        &self.evidence
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBranchBasisAdmission {
    label: String,
    effect_policy: ForgeQueryEffectPolicy,
    authority_lane: ForgeQueryAuthorityLane,
    evidence: Vec<String>,
}

impl ForgeQueryBranchBasisAdmission {
    pub fn new(
        _authority: &ForgeQueryRuntimeEvidenceAuthority,
        label: impl Into<String>,
        effect_policy: ForgeQueryEffectPolicy,
        evidence: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            label: label.into(),
            effect_policy,
            authority_lane: ForgeQueryAuthorityLane::BranchLocalTruth,
            evidence: evidence.into_iter().map(Into::into).collect(),
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn effect_policy(&self) -> ForgeQueryEffectPolicy {
        self.effect_policy
    }

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.authority_lane
    }

    pub fn evidence(&self) -> &[String] {
        &self.evidence
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimeInspectionEvidence {
    artifact_family: String,
    authority_lane: ForgeQueryAuthorityLane,
    evidence: Vec<String>,
}

impl ForgeQueryRuntimeInspectionEvidence {
    pub fn new(
        _authority: &ForgeQueryRuntimeEvidenceAuthority,
        artifact_family: impl Into<String>,
        authority_lane: ForgeQueryAuthorityLane,
        evidence: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            artifact_family: artifact_family.into(),
            authority_lane,
            evidence: evidence.into_iter().map(Into::into).collect(),
        }
    }

    pub fn artifact_family(&self) -> &str {
        &self.artifact_family
    }

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.authority_lane
    }

    pub fn evidence(&self) -> &[String] {
        &self.evidence
    }
}
