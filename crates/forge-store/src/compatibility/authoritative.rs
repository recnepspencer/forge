use super::admission::{
    CompatibilityAdmissionCounters, CompatibilityDecision, CompatibilityRejection,
    CompatibilityRejectionKind, CompatibilityRelation, ReadCompatibilityReceipt,
};
use super::decoding::CompatibilityCheckedArtifact;
use super::manifests::{ArtifactFamilyId, ArtifactSemanticVersion};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AuthoritativeCompatibilityWitness {
    family_id: ArtifactFamilyId,
}

impl AuthoritativeCompatibilityWitness {
    pub(crate) fn new(family_id: ArtifactFamilyId) -> Self {
        Self { family_id }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AuthoritativeMeaningDeclaration {
    family_id: ArtifactFamilyId,
    semantic_version: ArtifactSemanticVersion,
    meaning_label: String,
}

impl AuthoritativeMeaningDeclaration {
    pub(crate) fn new(
        family_id: ArtifactFamilyId,
        semantic_version: ArtifactSemanticVersion,
        meaning_label: impl Into<String>,
    ) -> Self {
        Self {
            family_id,
            semantic_version,
            meaning_label: meaning_label.into(),
        }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn semantic_version(&self) -> ArtifactSemanticVersion {
        self.semantic_version
    }

    pub fn meaning_label(&self) -> &str {
        &self.meaning_label
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AuthoritativeUnknownMeaning {
    family_id: ArtifactFamilyId,
    semantic_version: ArtifactSemanticVersion,
}

impl AuthoritativeUnknownMeaning {
    pub fn new(family_id: ArtifactFamilyId, semantic_version: ArtifactSemanticVersion) -> Self {
        Self {
            family_id,
            semantic_version,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AuthoritativePartialTruthRejection {
    family_id: ArtifactFamilyId,
    semantic_version: ArtifactSemanticVersion,
    reason: String,
}

impl AuthoritativePartialTruthRejection {
    pub(crate) fn new(
        family_id: ArtifactFamilyId,
        semantic_version: ArtifactSemanticVersion,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            family_id,
            semantic_version,
            reason: reason.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AuthoritativeAdmissionReport {
    family_id: ArtifactFamilyId,
    semantic_version: ArtifactSemanticVersion,
    relation: CompatibilityRelation,
    admitted: bool,
    rejection_kind: Option<CompatibilityRejectionKind>,
}

impl AuthoritativeAdmissionReport {
    pub(crate) fn admitted(
        declaration: &AuthoritativeMeaningDeclaration,
        relation: CompatibilityRelation,
    ) -> Self {
        Self {
            family_id: declaration.family_id().clone(),
            semantic_version: declaration.semantic_version(),
            relation,
            admitted: true,
            rejection_kind: None,
        }
    }

    pub(crate) fn rejected(
        family_id: ArtifactFamilyId,
        semantic_version: ArtifactSemanticVersion,
        relation: CompatibilityRelation,
        rejection_kind: CompatibilityRejectionKind,
    ) -> Self {
        Self {
            family_id,
            semantic_version,
            relation,
            admitted: false,
            rejection_kind: Some(rejection_kind),
        }
    }

    pub fn admitted_status(&self) -> bool {
        self.admitted
    }

    pub fn rejection_kind(&self) -> Option<CompatibilityRejectionKind> {
        self.rejection_kind
    }
}

pub(crate) fn declare_authoritative_meaning(
    family_id: ArtifactFamilyId,
    semantic_version: ArtifactSemanticVersion,
    meaning_label: impl Into<String>,
) -> AuthoritativeMeaningDeclaration {
    AuthoritativeMeaningDeclaration::new(family_id, semantic_version, meaning_label)
}

pub(crate) fn admit_authoritative_meaning(
    counters: &mut CompatibilityAdmissionCounters,
    checked_artifact: &CompatibilityCheckedArtifact,
    read_receipt: &ReadCompatibilityReceipt,
    meaning: Option<&AuthoritativeMeaningDeclaration>,
) -> Result<
    (
        AuthoritativeCompatibilityWitness,
        AuthoritativeAdmissionReport,
    ),
    CompatibilityRejection,
> {
    let relation = read_receipt.receipt().relation();
    let family_id = checked_artifact.family_id().clone();
    let reject = |counters: &mut CompatibilityAdmissionCounters,
                  kind: CompatibilityRejectionKind,
                  reason: &str| {
        counters.record_authoritative_partial_truth_rejection();
        Err(CompatibilityRejection::new(kind, family_id.clone(), reason))
    };

    if checked_artifact.family_id() != read_receipt.receipt().family_id() {
        return reject(
            counters,
            CompatibilityRejectionKind::ReceiptArtifactMismatch,
            "checked artifact and read receipt do not describe the same family",
        );
    }

    let Some(meaning) = meaning else {
        return reject(
            counters,
            CompatibilityRejectionKind::AuthoritativePartialTruthRejected,
            "authoritative semantic meaning is undeclared",
        );
    };

    if meaning.family_id() != checked_artifact.family_id()
        || meaning.semantic_version() != read_receipt.receipt().target_semantic_version()
    {
        return reject(
            counters,
            CompatibilityRejectionKind::AuthoritativePartialTruthRejected,
            "authoritative meaning declaration does not match admitted receipt",
        );
    }

    match checked_artifact.decision() {
        CompatibilityDecision::Admit(CompatibilityRelation::Native)
        | CompatibilityDecision::Admit(CompatibilityRelation::ForwardRead)
        | CompatibilityDecision::Admit(CompatibilityRelation::BackwardRead) => Ok((
            AuthoritativeCompatibilityWitness::new(family_id),
            AuthoritativeAdmissionReport::admitted(meaning, relation),
        )),
        CompatibilityDecision::Admit(CompatibilityRelation::AdapterRequired) => reject(
            counters,
            CompatibilityRejectionKind::AuthoritativePartialTruthRejected,
            "adapter-required authoritative meaning needs an adapter parity witness",
        ),
        CompatibilityDecision::Admit(CompatibilityRelation::DerivedRebuildRequired)
        | CompatibilityDecision::Admit(CompatibilityRelation::Incompatible)
        | CompatibilityDecision::Reject(_) => reject(
            counters,
            CompatibilityRejectionKind::AuthoritativePartialTruthRejected,
            "checked artifact decision cannot admit authoritative meaning",
        ),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ForwardAuthoritativeReadPlan {
    family_id: ArtifactFamilyId,
    target_semantic_version: ArtifactSemanticVersion,
}

impl ForwardAuthoritativeReadPlan {
    pub fn new(
        family_id: ArtifactFamilyId,
        target_semantic_version: ArtifactSemanticVersion,
    ) -> Self {
        Self {
            family_id,
            target_semantic_version,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BackwardAuthoritativeReadPlan {
    family_id: ArtifactFamilyId,
    target_semantic_version: ArtifactSemanticVersion,
}

impl BackwardAuthoritativeReadPlan {
    pub fn new(
        family_id: ArtifactFamilyId,
        target_semantic_version: ArtifactSemanticVersion,
    ) -> Self {
        Self {
            family_id,
            target_semantic_version,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UnsupportedAuthoritativeVersion {
    family_id: ArtifactFamilyId,
    semantic_version: ArtifactSemanticVersion,
}

impl UnsupportedAuthoritativeVersion {
    pub fn new(family_id: ArtifactFamilyId, semantic_version: ArtifactSemanticVersion) -> Self {
        Self {
            family_id,
            semantic_version,
        }
    }
}
