use crate::aspect_authority::ColorabilityAspectRecord;
use crate::domain_artifacts::core_artifact::require_non_empty;
use crate::domain_artifacts::{
    ColorabilityVerification, HadwigerArtifactReference, HadwigerArtifactShapeError,
    HadwigerCanonicalArtifact,
};

use super::motif_artifacts::MotifArtifact;
use super::motif_errors::MotifLanguageError;
use super::terminal_relations::{
    validate_checked_evidence, TerminalForcingRelation, TerminalForcingRelationKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalForcingRelationCertificate {
    relation_id: String,
    motif_reference: HadwigerArtifactReference,
    relation_kind: TerminalForcingRelationKind,
    terminal_labels: Vec<String>,
    color_count: u32,
    colorability_verification: ColorabilityVerification,
    not_k_colorable_aspect: ColorabilityAspectRecord,
}

impl TerminalForcingRelationCertificate {
    pub fn from_checked_colorability(
        relation_id: impl Into<String>,
        motif_reference: HadwigerArtifactReference,
        relation_kind: TerminalForcingRelationKind,
        terminal_labels: impl IntoIterator<Item = impl Into<String>>,
        colorability_verification: ColorabilityVerification,
        not_k_colorable_aspect: ColorabilityAspectRecord,
    ) -> Result<Self, MotifLanguageError> {
        validate_checked_evidence(&colorability_verification, &not_k_colorable_aspect)?;
        let mut terminal_labels = terminal_labels
            .into_iter()
            .map(|label| require_non_empty(label, "terminal_label"))
            .collect::<Result<Vec<_>, HadwigerArtifactShapeError>>()?;
        if terminal_labels.is_empty() {
            return Err(HadwigerArtifactShapeError::EmptyField {
                field: "terminal_labels",
            }
            .into());
        }
        terminal_labels.sort();
        reject_duplicate_terminal_labels(&terminal_labels)?;
        let color_count = colorability_verification.color_count();
        Ok(Self {
            relation_id: require_non_empty(relation_id, "relation_id")?,
            motif_reference,
            relation_kind,
            terminal_labels,
            color_count,
            colorability_verification,
            not_k_colorable_aspect,
        })
    }

    pub(crate) fn validate_against_motif(
        &self,
        motif: &MotifArtifact,
    ) -> Result<(), MotifLanguageError> {
        if self.motif_reference != motif.reference() {
            return Err(MotifLanguageError::TerminalRelationMotifMismatch);
        }
        for label in &self.terminal_labels {
            if !motif
                .terminals()
                .iter()
                .any(|terminal| terminal.label() == label)
            {
                return Err(MotifLanguageError::MissingMotifTerminal {
                    terminal_label: label.clone(),
                });
            }
        }
        Ok(())
    }

    pub(crate) fn relation_id(&self) -> &str {
        &self.relation_id
    }

    pub(crate) fn relation_kind(&self) -> TerminalForcingRelationKind {
        self.relation_kind
    }

    pub(crate) fn terminal_labels(&self) -> &[String] {
        &self.terminal_labels
    }

    pub(crate) fn color_count(&self) -> u32 {
        self.color_count
    }

    pub(crate) fn colorability_verification(&self) -> &ColorabilityVerification {
        &self.colorability_verification
    }

    pub(crate) fn not_k_colorable_aspect(&self) -> &ColorabilityAspectRecord {
        &self.not_k_colorable_aspect
    }
}

fn reject_duplicate_terminal_labels(terminal_labels: &[String]) -> Result<(), MotifLanguageError> {
    for pair in terminal_labels.windows(2) {
        if pair[0] == pair[1] {
            return Err(MotifLanguageError::DuplicateIdentityField {
                field: "terminal_label",
                value: pair[0].clone(),
            });
        }
    }
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalForcingRelationEvidence {
    relation_reference: HadwigerArtifactReference,
}

impl TerminalForcingRelationEvidence {
    pub fn from_checked_relation(relation: &TerminalForcingRelation) -> Self {
        Self {
            relation_reference: relation.reference(),
        }
    }

    pub fn relation_reference(&self) -> &HadwigerArtifactReference {
        &self.relation_reference
    }
}
