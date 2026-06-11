use crate::domain_artifacts::{HadwigerCanonicalArtifact, HadwigerDeclaredFamilyCheckedExt};
use crate::domain_declarations::{
    declare_research_request_checked, ConflictGraphExtractionDeclaration,
};
use crate::periodic_patterns::GeneratedPatternReplayChecked;
use crate::query_entry::HadwigerResearchHandle;
use crate::tiling_geometry::TilingContactReplayReport;

use super::conflict_graph_artifacts::TilingConflictGraph;
use super::conflict_graph_edges::TilingConflictEdge;
use super::conflict_graph_errors::{require_conflict_non_empty, ConflictGraphError};
use super::conflict_graph_index::ConflictGraphExtractionIndex;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TilingConflictGraphExtractionRequest {
    extraction_id: String,
    required_color_count: Option<u32>,
    source: TilingConflictGraphExtractionSource,
}

impl TilingConflictGraphExtractionRequest {
    pub fn from_tiling_contact_report(
        extraction_id: impl Into<String>,
        report: TilingContactReplayReport,
    ) -> Self {
        Self::try_from_tiling_contact_report(extraction_id, report)
            .expect("extraction_id must be non-empty")
    }

    pub fn try_from_tiling_contact_report(
        extraction_id: impl Into<String>,
        report: TilingContactReplayReport,
    ) -> Result<Self, ConflictGraphError> {
        Ok(Self {
            extraction_id: require_conflict_non_empty(extraction_id, "extraction_id")?,
            required_color_count: None,
            source: TilingConflictGraphExtractionSource::ContactReport(report),
        })
    }

    pub fn from_generated_pattern_replay(
        extraction_id: impl Into<String>,
        checked: &GeneratedPatternReplayChecked,
    ) -> Self {
        Self::try_from_generated_pattern_replay(extraction_id, checked)
            .expect("extraction_id must be non-empty")
    }

    pub fn try_from_generated_pattern_replay(
        extraction_id: impl Into<String>,
        checked: &GeneratedPatternReplayChecked,
    ) -> Result<Self, ConflictGraphError> {
        Ok(Self {
            extraction_id: require_conflict_non_empty(extraction_id, "extraction_id")?,
            required_color_count: None,
            source: TilingConflictGraphExtractionSource::GeneratedPatternReplay(checked.clone()),
        })
    }

    pub fn with_required_color_count(
        mut self,
        color_count: u32,
    ) -> Result<Self, ConflictGraphError> {
        if color_count == 0 {
            return Err(ConflictGraphError::EmptyField {
                field: "required_color_count",
            });
        }
        self.required_color_count = Some(color_count);
        Ok(self)
    }

    pub fn extraction_id(&self) -> &str {
        &self.extraction_id
    }

    pub fn required_color_count(&self) -> Option<u32> {
        self.required_color_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum TilingConflictGraphExtractionSource {
    ContactReport(TilingContactReplayReport),
    GeneratedPatternReplay(GeneratedPatternReplayChecked),
}

pub fn extract_conflict_graph_checked(
    handle: &HadwigerResearchHandle,
    request: TilingConflictGraphExtractionRequest,
) -> Result<TilingConflictGraph, ConflictGraphError> {
    let subject_ref = request.source.subject_stable_token();
    let query_reference = declare_conflict_graph_extraction(handle, &request, &subject_ref)?;
    let edges = request.source.extract_edges()?;
    let index = ConflictGraphExtractionIndex::from_edges(edges)?;
    TilingConflictGraph::checked(
        request.extraction_id,
        query_reference,
        index,
        request.required_color_count,
    )
}

impl TilingConflictGraphExtractionSource {
    fn subject_stable_token(&self) -> String {
        match self {
            Self::ContactReport(report) => report.reference().stable_token(),
            Self::GeneratedPatternReplay(checked) => checked.report().reference().stable_token(),
        }
    }

    fn distance_certificate_family(&self) -> &'static str {
        match self {
            Self::ContactReport(_) => "tiling_contact_replay",
            Self::GeneratedPatternReplay(_) => "generated_pattern_replay",
        }
    }

    fn extract_edges(&self) -> Result<Vec<TilingConflictEdge>, ConflictGraphError> {
        match self {
            Self::ContactReport(report) => {
                Ok(vec![TilingConflictEdge::from_contact_report(report)?])
            }
            Self::GeneratedPatternReplay(checked) => {
                let mut edges = Vec::new();
                if !checked.report().has_rejected_generated_rule() {
                    return Err(ConflictGraphError::GeneratedReplayHasNoRejectedEvidence);
                }
                for certificate in checked.suite().periodic_quotient_conflicts() {
                    edges.push(TilingConflictEdge::from_periodic_quotient_conflict(
                        checked.report(),
                        &certificate.certificate(),
                    )?);
                }
                for certificate in checked.suite().translation_rotation_certificates() {
                    edges.extend(TilingConflictEdge::from_translation_rotation_closure(
                        checked.report(),
                        certificate.graph(),
                        &certificate.certificate(),
                    )?);
                }
                if edges.is_empty() {
                    return Err(ConflictGraphError::GeneratedReplayHasNoExtractableConflictEdges);
                }
                Ok(edges)
            }
        }
    }
}

fn declare_conflict_graph_extraction(
    handle: &HadwigerResearchHandle,
    request: &TilingConflictGraphExtractionRequest,
    subject_ref: &str,
) -> Result<crate::domain_artifacts::HadwigerQueryDeclarationReference, ConflictGraphError> {
    let checked = declare_research_request_checked(
        handle,
        conflict_graph_extraction_declaration(request, subject_ref)?,
    );
    checked
        .admitted()
        .map(Into::into)
        .ok_or(ConflictGraphError::QueryDeclarationNotAdmitted {
            declaration: "conflict_graph_extraction",
        })
}

fn conflict_graph_extraction_declaration(
    request: &TilingConflictGraphExtractionRequest,
    subject_ref: &str,
) -> Result<ConflictGraphExtractionDeclaration, ConflictGraphError> {
    let mut declaration =
        ConflictGraphExtractionDeclaration::new(request.extraction_id(), subject_ref)
            .try_with_distance_certificate_family(request.source.distance_certificate_family())?;
    if let Some(color_count) = request.required_color_count() {
        declaration = declaration.try_with_required_color_count(color_count)?;
    }
    Ok(declaration)
}
