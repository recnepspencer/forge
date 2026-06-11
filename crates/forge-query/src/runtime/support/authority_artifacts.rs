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
pub struct ForgeQueryBasisAdmissionEvidenceRow {
    kind: &'static str,
    value: String,
    row_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryBasisAdmissionEvidenceRow {
    pub fn tagged(kind: &'static str, value: impl Into<String>) -> Self {
        let value = value.into();
        let row_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::BasisAdmissionEvidenceRow)
                .field_shape(ForgeQueryEvidenceTag::new("kind"), kind)
                .field_value(ForgeQueryEvidenceTag::new("value"), value.as_str())
                .seal();
        Self {
            kind,
            value,
            row_digest,
        }
    }

    pub fn support_profile_token(token: impl Into<String>) -> Self {
        Self::tagged("support-profile-evidence", token)
    }

    pub fn rows_from_values(values: impl IntoIterator<Item = impl Into<String>>) -> Vec<Self> {
        values
            .into_iter()
            .map(|value| Self::tagged("basis-admission-evidence", value))
            .collect()
    }

    pub fn kind(&self) -> &'static str {
        self.kind
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn row_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPreviewBasisAdmission {
    label: ForgeQuerySessionLabel,
    effect_policy: ForgeQueryEffectPolicy,
    authority_lane: ForgeQueryAuthorityLane,
    evidence_rows: Vec<ForgeQueryBasisAdmissionEvidenceRow>,
    admission_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryPreviewBasisAdmission {
    pub fn new(
        _authority: &ForgeQueryRuntimeEvidenceAuthority,
        label: ForgeQuerySessionLabel,
        effect_policy: ForgeQueryEffectPolicy,
        evidence_rows: impl IntoIterator<Item = ForgeQueryBasisAdmissionEvidenceRow>,
    ) -> Self {
        let evidence_rows = evidence_rows.into_iter().collect::<Vec<_>>();
        let authority_lane = ForgeQueryAuthorityLane::PreviewTruth;
        let admission_digest = basis_admission_digest(
            ForgeQueryEvidenceScope::PreviewBasisAdmission,
            &label,
            effect_policy,
            authority_lane,
            &evidence_rows,
        );
        Self {
            label,
            effect_policy,
            authority_lane,
            evidence_rows,
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

    pub fn evidence_rows(&self) -> &[ForgeQueryBasisAdmissionEvidenceRow] {
        &self.evidence_rows
    }

    pub fn evidence(&self) -> Vec<String> {
        self.evidence_rows
            .iter()
            .map(|row| row.value().to_string())
            .collect()
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
    evidence_rows: Vec<ForgeQueryBasisAdmissionEvidenceRow>,
    admission_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryBranchBasisAdmission {
    pub fn new(
        _authority: &ForgeQueryRuntimeEvidenceAuthority,
        label: ForgeQuerySessionLabel,
        effect_policy: ForgeQueryEffectPolicy,
        evidence_rows: impl IntoIterator<Item = ForgeQueryBasisAdmissionEvidenceRow>,
    ) -> Self {
        let evidence_rows = evidence_rows.into_iter().collect::<Vec<_>>();
        let authority_lane = ForgeQueryAuthorityLane::BranchLocalTruth;
        let admission_digest = basis_admission_digest(
            ForgeQueryEvidenceScope::BranchBasisAdmission,
            &label,
            effect_policy,
            authority_lane,
            &evidence_rows,
        );
        Self {
            label,
            effect_policy,
            authority_lane,
            evidence_rows,
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

    pub fn evidence_rows(&self) -> &[ForgeQueryBasisAdmissionEvidenceRow] {
        &self.evidence_rows
    }

    pub fn evidence(&self) -> Vec<String> {
        self.evidence_rows
            .iter()
            .map(|row| row.value().to_string())
            .collect()
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
    evidence_rows: &[ForgeQueryBasisAdmissionEvidenceRow],
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
            ForgeQueryEvidenceTag::new("evidence_row"),
            evidence_rows.iter().map(|row| row.row_digest().as_str()),
        )
        .seal()
}
