use super::super::artifacts::{S0ArtifactRowId, S0ArtifactRowStatus, S0ArtifactSubjectKind};
use super::super::capability::{BackendForbiddenClaim, Roadmap2SequenceId};
use super::super::evidence::S0EvidenceRef;
use super::fixtures::S1ForbiddenShortcut;
use super::maturity::{
    ForbiddenShortcutDetectionStatus, HarnessMaturityLevel, HarnessSubsystemMaturity,
};
use super::validation::{require_non_empty, S0HarnessMaturityBuildRejection};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HarnessMaturityRow {
    pub(super) row_id: S0ArtifactRowId,
    pub(super) subject_kind: S0ArtifactSubjectKind,
    pub(super) subject_path_or_symbol: String,
    pub(super) classification: String,
    pub(super) evidence_refs: Vec<S0EvidenceRef>,
    pub(super) forbidden_claims: Vec<BackendForbiddenClaim>,
    pub(super) deferred_s_sequences: Vec<Roadmap2SequenceId>,
    pub(super) status: S0ArtifactRowStatus,
    pub(super) notes: String,
    pub(super) subsystem: HarnessSubsystemMaturity,
    pub(super) maturity_level: HarnessMaturityLevel,
    pub(super) required_for_sequences: Vec<Roadmap2SequenceId>,
    pub(super) forbidden_shortcuts_covered: Vec<S1ForbiddenShortcut>,
    pub(super) detection_status: ForbiddenShortcutDetectionStatus,
}

impl HarnessMaturityRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        row_id: S0ArtifactRowId,
        subject_path_or_symbol: impl Into<String>,
        evidence_refs: Vec<S0EvidenceRef>,
        status: S0ArtifactRowStatus,
        notes: impl Into<String>,
        subsystem: HarnessSubsystemMaturity,
        maturity_level: HarnessMaturityLevel,
        required_for_sequences: Vec<Roadmap2SequenceId>,
        forbidden_shortcuts_covered: Vec<S1ForbiddenShortcut>,
        detection_status: ForbiddenShortcutDetectionStatus,
    ) -> Result<Self, S0HarnessMaturityBuildRejection> {
        let subject_path_or_symbol = require_non_empty(subject_path_or_symbol)
            .map_err(|_| S0HarnessMaturityBuildRejection::EmptyRequiredField)?;
        let notes = require_non_empty(notes)
            .map_err(|_| S0HarnessMaturityBuildRejection::EmptyRequiredField)?;
        if evidence_refs.is_empty() {
            return Err(S0HarnessMaturityBuildRejection::MissingEvidenceRef);
        }
        if required_for_sequences.is_empty() {
            return Err(S0HarnessMaturityBuildRejection::MissingRequiredSequence);
        }
        Ok(Self {
            row_id,
            subject_kind: S0ArtifactSubjectKind::Harness,
            subject_path_or_symbol,
            classification: "harness-maturity".to_string(),
            evidence_refs,
            forbidden_claims: Vec::new(),
            deferred_s_sequences: Vec::new(),
            status,
            notes,
            subsystem,
            maturity_level,
            required_for_sequences,
            forbidden_shortcuts_covered,
            detection_status,
        })
    }

    pub fn row_id(&self) -> &S0ArtifactRowId {
        &self.row_id
    }

    pub fn subsystem(&self) -> HarnessSubsystemMaturity {
        self.subsystem
    }

    pub fn maturity_level(&self) -> HarnessMaturityLevel {
        self.maturity_level
    }

    pub fn forbidden_shortcuts_covered(&self) -> &[S1ForbiddenShortcut] {
        &self.forbidden_shortcuts_covered
    }
}
