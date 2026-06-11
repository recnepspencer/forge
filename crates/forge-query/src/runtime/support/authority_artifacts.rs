use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::runtime::{ForgeQueryAuthorityLane, ForgeQueryEffectPolicy};
use crate::session_label::ForgeQuerySessionLabel;

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
    label: ForgeQuerySessionLabel,
    effect_policy: ForgeQueryEffectPolicy,
    authority_lane: ForgeQueryAuthorityLane,
    evidence: Vec<String>,
    admission_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryPreviewBasisAdmission {
    pub fn new(
        _authority: &ForgeQueryRuntimeEvidenceAuthority,
        label: ForgeQuerySessionLabel,
        effect_policy: ForgeQueryEffectPolicy,
        evidence: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        let evidence = evidence.into_iter().map(Into::into).collect::<Vec<_>>();
        let authority_lane = ForgeQueryAuthorityLane::PreviewTruth;
        let admission_digest = basis_admission_digest(
            ForgeQueryEvidenceScope::PreviewBasisAdmission,
            &label,
            effect_policy,
            authority_lane,
            &evidence,
        );
        Self {
            label,
            effect_policy,
            authority_lane,
            evidence,
            admission_digest,
        }
    }

    pub fn label(&self) -> &str {
        self.label.display()
    }

    pub fn session_label(&self) -> &ForgeQuerySessionLabel {
        &self.label
    }

    pub fn label_identity(&self) -> &ForgeQueryEvidenceIdentity {
        self.label.identity_digest()
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

    pub fn admission_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.admission_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBranchBasisAdmission {
    label: ForgeQuerySessionLabel,
    effect_policy: ForgeQueryEffectPolicy,
    authority_lane: ForgeQueryAuthorityLane,
    evidence: Vec<String>,
    admission_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryBranchBasisAdmission {
    pub fn new(
        _authority: &ForgeQueryRuntimeEvidenceAuthority,
        label: ForgeQuerySessionLabel,
        effect_policy: ForgeQueryEffectPolicy,
        evidence: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        let evidence = evidence.into_iter().map(Into::into).collect::<Vec<_>>();
        let authority_lane = ForgeQueryAuthorityLane::BranchLocalTruth;
        let admission_digest = basis_admission_digest(
            ForgeQueryEvidenceScope::BranchBasisAdmission,
            &label,
            effect_policy,
            authority_lane,
            &evidence,
        );
        Self {
            label,
            effect_policy,
            authority_lane,
            evidence,
            admission_digest,
        }
    }

    pub fn label(&self) -> &str {
        self.label.display()
    }

    pub fn session_label(&self) -> &ForgeQuerySessionLabel {
        &self.label
    }

    pub fn label_identity(&self) -> &ForgeQueryEvidenceIdentity {
        self.label.identity_digest()
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

    pub fn admission_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.admission_digest
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

fn basis_admission_digest(
    scope: ForgeQueryEvidenceScope,
    label: &ForgeQuerySessionLabel,
    effect_policy: ForgeQueryEffectPolicy,
    authority_lane: ForgeQueryAuthorityLane,
    evidence: &[String],
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(scope)
        .field_identity(
            ForgeQueryEvidenceTag::new("session_label_identity"),
            label.identity_digest().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("effect_policy"),
            effect_policy.as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("authority_lane"),
            authority_lane.as_str(),
        )
        .field_identity_sequence(
            ForgeQueryEvidenceTag::new("evidence"),
            evidence.iter().map(String::as_str),
        )
        .seal()
}
