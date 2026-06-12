use crate::aspect_authority::{ColorabilityAspectRecord, HadwigerAspectPosture};
use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactReference, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::{
    ColorabilityVerification, ColorabilityVerificationPosture, HadwigerCanonicalArtifact,
    HadwigerQueryDeclarationReference,
};

use super::motif_artifacts::MotifArtifact;
use super::motif_errors::MotifLanguageError;
use super::terminal_relation_certificates::TerminalForcingRelationCertificate;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminalForcingRelationKind {
    MustDiffer,
    CannotShareColorSubset,
    RequiresDistinctColorCount,
}

impl TerminalForcingRelationKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MustDiffer => "must_differ",
            Self::CannotShareColorSubset => "cannot_share_color_subset",
            Self::RequiresDistinctColorCount => "requires_distinct_color_count",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminalForcingRelationPosture {
    Candidate,
    Blocked,
    Checked,
}

impl TerminalForcingRelationPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Candidate => "candidate",
            Self::Blocked => "blocked",
            Self::Checked => "checked",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalForcingRelation {
    core: HadwigerArtifactCore,
    relation_id: String,
    relation_kind: TerminalForcingRelationKind,
    posture: TerminalForcingRelationPosture,
    motif_reference: HadwigerArtifactReference,
    terminal_labels: Vec<String>,
    color_count: u32,
    evidence: TerminalForcingRelationEvidenceSnapshot,
}

impl TerminalForcingRelation {
    pub(crate) fn checked(
        source_declaration: HadwigerQueryDeclarationReference,
        motif: &MotifArtifact,
        certificate: TerminalForcingRelationCertificate,
    ) -> Result<Self, MotifLanguageError> {
        certificate.validate_against_motif(motif)?;
        let evidence = TerminalForcingRelationEvidenceSnapshot::from_certificate(&certificate);
        let mut terminal_labels = certificate.terminal_labels().to_vec();
        terminal_labels.sort();
        let mut entries = vec![
            HadwigerArtifactPayloadEntry::text("relation_id", certificate.relation_id()),
            HadwigerArtifactPayloadEntry::text(
                "relation_kind",
                certificate.relation_kind().as_str(),
            ),
            HadwigerArtifactPayloadEntry::text(
                "posture",
                TerminalForcingRelationPosture::Checked.as_str(),
            ),
            HadwigerArtifactPayloadEntry::unsigned(
                "color_count",
                certificate.color_count() as u128,
            ),
            HadwigerArtifactPayloadEntry::text("motif_reference", motif.reference().stable_token()),
            HadwigerArtifactPayloadEntry::text("evidence", evidence.stable_token()),
        ];
        for terminal in &terminal_labels {
            entries.push(HadwigerArtifactPayloadEntry::text("terminal", terminal));
        }
        let core = artifact_core(
            HadwigerArtifactKind::TerminalForcingRelation,
            HadwigerArtifactAuthorityOwner::Checker,
            HadwigerArtifactSourceReference::QueryDeclaration(source_declaration),
            vec![
                motif.reference(),
                certificate.colorability_verification().reference(),
                certificate
                    .not_k_colorable_aspect()
                    .artifact_reference()
                    .clone(),
            ],
            entries,
        )?;
        Ok(Self {
            core,
            relation_id: certificate.relation_id().to_string(),
            relation_kind: certificate.relation_kind(),
            posture: TerminalForcingRelationPosture::Checked,
            motif_reference: motif.reference(),
            terminal_labels,
            color_count: certificate.color_count(),
            evidence,
        })
    }

    pub fn relation_id(&self) -> &str {
        &self.relation_id
    }

    pub fn relation_kind(&self) -> TerminalForcingRelationKind {
        self.relation_kind
    }

    pub fn posture(&self) -> TerminalForcingRelationPosture {
        self.posture
    }

    pub fn is_checked(&self) -> bool {
        self.posture == TerminalForcingRelationPosture::Checked
    }

    pub fn satisfies_terminal_relation_dependency(&self) -> bool {
        self.is_checked()
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn motif_reference(&self) -> &HadwigerArtifactReference {
        &self.motif_reference
    }

    pub fn terminal_labels(&self) -> &[String] {
        &self.terminal_labels
    }

    pub fn color_count(&self) -> u32 {
        self.color_count
    }

    pub fn evidence(&self) -> &TerminalForcingRelationEvidenceSnapshot {
        &self.evidence
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalForcingRelationEvidenceSnapshot {
    colorability_verification_reference: HadwigerArtifactReference,
    not_k_colorable_aspect_token: String,
}

impl TerminalForcingRelationEvidenceSnapshot {
    fn from_certificate(certificate: &TerminalForcingRelationCertificate) -> Self {
        Self {
            colorability_verification_reference: certificate
                .colorability_verification()
                .reference(),
            not_k_colorable_aspect_token: certificate.not_k_colorable_aspect().stable_token(),
        }
    }

    pub fn stable_token(&self) -> String {
        format!(
            "{}:{}",
            self.colorability_verification_reference.stable_token(),
            self.not_k_colorable_aspect_token
        )
    }
}

impl_hadwiger_artifact!(TerminalForcingRelation, core);

pub(crate) fn validate_checked_evidence(
    verification: &ColorabilityVerification,
    aspect: &ColorabilityAspectRecord,
) -> Result<(), MotifLanguageError> {
    if verification.posture() != ColorabilityVerificationPosture::UnsatVerified {
        return Err(MotifLanguageError::TerminalRelationEvidenceNotAdmitted);
    }
    if aspect.aspect_posture() != HadwigerAspectPosture::Admitted {
        return Err(MotifLanguageError::TerminalRelationEvidenceNotAdmitted);
    }
    if !aspect.satisfies_mathematical_dependency() {
        return Err(MotifLanguageError::TerminalRelationEvidenceNotAdmitted);
    }
    if verification.color_count() != aspect.color_count() {
        return Err(MotifLanguageError::TerminalRelationColorCountMismatch {
            expected: verification.color_count(),
            actual: aspect.color_count(),
        });
    }
    if verification.graph_version_reference() != aspect.artifact_reference() {
        return Err(MotifLanguageError::TerminalRelationEvidenceNotAdmitted);
    }
    Ok(())
}
