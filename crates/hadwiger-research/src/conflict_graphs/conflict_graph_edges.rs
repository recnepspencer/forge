use crate::candidate_screening::{
    PeriodicQuotientConflictCertificate, ScreeningFiniteGraphIndex,
    TranslationRotationClosureCertificate,
};
use crate::domain_artifacts::core_artifact::canonical_digest_token;
use crate::domain_artifacts::{GraphVersion, HadwigerArtifactReference, HadwigerCanonicalArtifact};
use crate::periodic_patterns::GeneratedPatternReplayReport;
use crate::tiling_geometry::{TilingContactReplayReport, TilingContactRole};

use super::conflict_graph_errors::{require_conflict_non_empty, ConflictGraphError};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum TilingConflictEdgeBasis {
    ExactContactReplay,
    PeriodicGeneratedReplay,
}

impl TilingConflictEdgeBasis {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExactContactReplay => "exact_contact_replay",
            Self::PeriodicGeneratedReplay => "periodic_generated_replay",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TilingConflictEdge {
    left_vertex_label: String,
    right_vertex_label: String,
    basis: TilingConflictEdgeBasis,
    source_evidence_reference: HadwigerArtifactReference,
    exact_evidence_digest: String,
    translated_boundary: Option<String>,
}

impl TilingConflictEdge {
    pub(crate) fn from_contact_report(
        report: &TilingContactReplayReport,
    ) -> Result<Self, ConflictGraphError> {
        if !report.evaluation().rejects_candidate() {
            return Err(ConflictGraphError::ContactReportDoesNotReject);
        }
        let contact = report.contact_fact();
        let edge = Self::new(
            contact.left_tile_id(),
            contact.right_tile_id(),
            TilingConflictEdgeBasis::ExactContactReplay,
            report.reference(),
            canonical_digest_token(report.evaluation().artifact_digest().canonical()),
            contact_translation_label(contact.role()),
        )?;
        Ok(edge)
    }

    pub(crate) fn from_periodic_quotient_conflict(
        report: &GeneratedPatternReplayReport,
        certificate: &PeriodicQuotientConflictCertificate,
    ) -> Result<Self, ConflictGraphError> {
        Self::new(
            certificate.left_tile_id(),
            certificate.right_tile_id(),
            TilingConflictEdgeBasis::PeriodicGeneratedReplay,
            report.reference(),
            certificate.stable_token(),
            Some(format!(
                "periodic_translation:{}:{}",
                certificate.translation_dx().stable_token(),
                certificate.translation_dy().stable_token()
            )),
        )
    }

    pub(crate) fn from_translation_rotation_closure(
        report: &GeneratedPatternReplayReport,
        graph: &GraphVersion,
        certificate: &TranslationRotationClosureCertificate,
    ) -> Result<Vec<Self>, ConflictGraphError> {
        let graph_index = ScreeningFiniteGraphIndex::from_graph_version(graph);
        let mut edges = Vec::new();
        for (left, right) in certificate
            .same_color_pairs()
            .iter()
            .filter(|(left, right)| graph_index.is_adjacent_label(left, right))
        {
            edges.push(Self::new(
                left,
                right,
                TilingConflictEdgeBasis::PeriodicGeneratedReplay,
                report.reference(),
                certificate.stable_token(),
                Some("translation_rotation_closure".to_string()),
            )?);
        }
        Ok(edges)
    }

    pub fn left_vertex_label(&self) -> &str {
        &self.left_vertex_label
    }

    pub fn right_vertex_label(&self) -> &str {
        &self.right_vertex_label
    }

    pub fn basis(&self) -> TilingConflictEdgeBasis {
        self.basis
    }

    pub fn source_evidence_reference(&self) -> &HadwigerArtifactReference {
        &self.source_evidence_reference
    }

    pub fn exact_evidence_digest(&self) -> &str {
        &self.exact_evidence_digest
    }

    pub fn translated_boundary(&self) -> Option<&str> {
        self.translated_boundary.as_deref()
    }

    pub fn has_exact_conflict_evidence(&self) -> bool {
        !self.exact_evidence_digest.is_empty()
    }

    pub fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}:{}",
            self.left_vertex_label,
            self.right_vertex_label,
            self.basis.as_str(),
            self.source_evidence_reference.stable_token(),
            self.exact_evidence_digest,
            self.translated_boundary.as_deref().unwrap_or("none")
        )
    }

    fn new(
        left_vertex_label: impl Into<String>,
        right_vertex_label: impl Into<String>,
        basis: TilingConflictEdgeBasis,
        source_evidence_reference: HadwigerArtifactReference,
        exact_evidence_digest: impl Into<String>,
        translated_boundary: Option<String>,
    ) -> Result<Self, ConflictGraphError> {
        let mut left_vertex_label =
            require_conflict_non_empty(left_vertex_label, "left_vertex_label")?;
        let mut right_vertex_label =
            require_conflict_non_empty(right_vertex_label, "right_vertex_label")?;
        if right_vertex_label < left_vertex_label {
            std::mem::swap(&mut left_vertex_label, &mut right_vertex_label);
        }
        Ok(Self {
            left_vertex_label,
            right_vertex_label,
            basis,
            source_evidence_reference,
            exact_evidence_digest: require_conflict_non_empty(
                exact_evidence_digest,
                "exact_evidence_digest",
            )?,
            translated_boundary,
        })
    }
}

fn contact_translation_label(role: TilingContactRole) -> Option<String> {
    match role {
        TilingContactRole::BoundaryContact | TilingContactRole::MinkowskiUnitContact => {
            Some(role.as_str().to_string())
        }
        TilingContactRole::SameColorConflictCandidate | TilingContactRole::DiameterSafety => None,
    }
}
