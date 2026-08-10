use super::super::capability::Roadmap2SequenceId;
use super::super::evidence::S0EvidenceRef;
use super::validation::TerminologyCleanupRejection;
use serde::{Deserialize, Serialize};

pub(super) const TERMINOLOGY_RISK_PHRASES: [&str; 14] = [
    "production-grade",
    "platform-grade",
    "database",
    "embedded backend",
    "wal",
    "crash recovery",
    "durability",
    "physical",
    "blob",
    "replication",
    "certification",
    "bounded",
    "integrity",
    "repair",
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum TerminologyAllowedUse {
    AllowedSemanticUse,
    QualifiedPhysicalDebt {
        deferred_sequence: Roadmap2SequenceId,
    },
    ClosedFoundationEvidence {
        evidence_ref: S0EvidenceRef,
    },
    OverclaimedPhysicalPosture,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TerminologyAllowlistEntry {
    pub(super) path: String,
    pub(super) line_number: u64,
    pub(super) phrase: String,
    pub(super) allowed_use: TerminologyAllowedUse,
}

impl TerminologyAllowlistEntry {
    pub fn new(
        path: impl Into<String>,
        line_number: u64,
        phrase: impl Into<String>,
        allowed_use: TerminologyAllowedUse,
    ) -> Result<Self, TerminologyCleanupRejection> {
        let path = super::validation::normalize_relative_path(path)?;
        let phrase = phrase.into();
        if phrase.trim().is_empty() {
            return Err(TerminologyCleanupRejection::EmptyRequiredField);
        }
        if line_number == 0 {
            return Err(TerminologyCleanupRejection::InvalidLineNumber);
        }
        if matches!(
            allowed_use,
            TerminologyAllowedUse::QualifiedPhysicalDebt { .. }
        ) && !phrase_requires_qualification(&phrase)
        {
            return Err(TerminologyCleanupRejection::QualifierAppliedToNonRiskPhrase);
        }
        Ok(Self {
            path,
            line_number,
            phrase: phrase.to_ascii_lowercase(),
            allowed_use,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerminologyRequiredQualifier {
    SemanticOnly,
    NamesDeferredSequence,
    ReferencesClosedFoundationEvidence,
    RejectAsOverclaim,
}

pub(super) fn required_qualifier(
    allowed_use: &TerminologyAllowedUse,
) -> TerminologyRequiredQualifier {
    match allowed_use {
        TerminologyAllowedUse::AllowedSemanticUse => TerminologyRequiredQualifier::SemanticOnly,
        TerminologyAllowedUse::QualifiedPhysicalDebt { .. } => {
            TerminologyRequiredQualifier::NamesDeferredSequence
        }
        TerminologyAllowedUse::ClosedFoundationEvidence { .. } => {
            TerminologyRequiredQualifier::ReferencesClosedFoundationEvidence
        }
        TerminologyAllowedUse::OverclaimedPhysicalPosture => {
            TerminologyRequiredQualifier::RejectAsOverclaim
        }
    }
}

pub(super) fn allowed_use_basis(allowed_use: &TerminologyAllowedUse) -> String {
    match allowed_use {
        TerminologyAllowedUse::AllowedSemanticUse => "allowed_semantic_use".to_string(),
        TerminologyAllowedUse::QualifiedPhysicalDebt { deferred_sequence } => {
            format!("qualified_physical_debt:{}", deferred_sequence.as_str())
        }
        TerminologyAllowedUse::ClosedFoundationEvidence { evidence_ref } => format!(
            "closed_foundation_evidence:{:?}:{}",
            evidence_ref.artifact_kind(),
            evidence_ref.digest().as_str()
        ),
        TerminologyAllowedUse::OverclaimedPhysicalPosture => {
            "overclaimed_physical_posture".to_string()
        }
    }
}

pub(super) fn phrase_requires_qualification(phrase: &str) -> bool {
    TERMINOLOGY_RISK_PHRASES.contains(&phrase.to_ascii_lowercase().as_str())
}
