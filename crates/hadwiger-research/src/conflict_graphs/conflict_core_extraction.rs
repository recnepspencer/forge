use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::{
    ColorabilityVerification, ColorabilityVerificationPosture, HadwigerArtifactReference,
    HadwigerCanonicalArtifact, HadwigerDeclaredFamilyCheckedExt,
};
use crate::domain_declarations::{declare_research_request_checked, CoreExtractionDeclaration};
use crate::mathematical_verification::{
    verify_k_colorability_checked, KColorabilityVerificationChecked,
};
use crate::query_entry::HadwigerResearchHandle;

use super::conflict_core_counters::ConflictCoreExtractionCounters;
use super::conflict_graph_artifacts::TilingConflictGraph;
use super::conflict_graph_errors::{require_conflict_non_empty, ConflictGraphError};
use super::core_deletion_proof_validation::validate_deletion_proof_certificate;
use super::core_minimization_certificates::{
    ConflictCoreDeletionCheckKind, ConflictCoreMinimalityCertificate,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ConflictCoreExtractionPosture {
    Colorable,
    NonColorableNotMinimized,
    VertexMinimal,
    EdgeMinimal,
    VertexAndEdgeMinimal,
    UnsupportedMinimality,
}

impl ConflictCoreExtractionPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Colorable => "colorable",
            Self::NonColorableNotMinimized => "non_colorable_not_minimized",
            Self::VertexMinimal => "vertex_minimal",
            Self::EdgeMinimal => "edge_minimal",
            Self::VertexAndEdgeMinimal => "vertex_and_edge_minimal",
            Self::UnsupportedMinimality => "unsupported_minimality",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConflictCoreExtractionRequest {
    core_extraction_id: String,
    conflict_graph: TilingConflictGraph,
    color_count: u32,
    vertex_minimization_budget: Option<usize>,
    edge_minimization_budget: Option<usize>,
    minimality_certificate: Option<ConflictCoreMinimalityCertificate>,
}

impl ConflictCoreExtractionRequest {
    pub fn new(
        core_extraction_id: impl Into<String>,
        conflict_graph: &TilingConflictGraph,
        color_count: u32,
    ) -> Self {
        Self::try_new(core_extraction_id, conflict_graph, color_count)
            .expect("core_extraction_id must be non-empty and color_count must be non-zero")
    }

    pub fn try_new(
        core_extraction_id: impl Into<String>,
        conflict_graph: &TilingConflictGraph,
        color_count: u32,
    ) -> Result<Self, ConflictGraphError> {
        if color_count == 0 {
            return Err(ConflictGraphError::EmptyField {
                field: "color_count",
            });
        }
        Ok(Self {
            core_extraction_id: require_conflict_non_empty(
                core_extraction_id,
                "core_extraction_id",
            )?,
            conflict_graph: conflict_graph.clone(),
            color_count,
            vertex_minimization_budget: None,
            edge_minimization_budget: None,
            minimality_certificate: None,
        })
    }

    pub fn with_vertex_minimization_budget(
        mut self,
        budget: usize,
    ) -> Result<Self, ConflictGraphError> {
        if budget == 0 {
            return Err(ConflictGraphError::EmptyField {
                field: "vertex_minimization_budget",
            });
        }
        self.vertex_minimization_budget = Some(budget);
        Ok(self)
    }

    pub fn with_edge_minimization_budget(
        mut self,
        budget: usize,
    ) -> Result<Self, ConflictGraphError> {
        if budget == 0 {
            return Err(ConflictGraphError::EmptyField {
                field: "edge_minimization_budget",
            });
        }
        self.edge_minimization_budget = Some(budget);
        Ok(self)
    }

    pub fn with_minimality_certificate(
        mut self,
        certificate: ConflictCoreMinimalityCertificate,
    ) -> Result<Self, ConflictGraphError> {
        self.minimality_certificate = Some(certificate);
        Ok(self)
    }

    pub(crate) fn conflict_graph(&self) -> &TilingConflictGraph {
        &self.conflict_graph
    }

    pub(crate) fn color_count(&self) -> u32 {
        self.color_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConflictCoreExtractionReport {
    core: HadwigerArtifactCore,
    core_extraction_id: String,
    conflict_graph: TilingConflictGraph,
    colorability_verification: Option<ColorabilityVerification>,
    posture: ConflictCoreExtractionPosture,
    counters: ConflictCoreExtractionCounters,
    query_declaration_digest: String,
}

impl ConflictCoreExtractionReport {
    pub fn posture(&self) -> ConflictCoreExtractionPosture {
        self.posture
    }

    pub fn colorability_verification(&self) -> Option<&ColorabilityVerification> {
        self.colorability_verification.as_ref()
    }

    pub fn counters(&self) -> &ConflictCoreExtractionCounters {
        &self.counters
    }

    pub fn query_declaration_digest(&self) -> &str {
        &self.query_declaration_digest
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(ConflictCoreExtractionReport, core);

pub fn extract_conflict_core_checked(
    handle: &HadwigerResearchHandle,
    request: ConflictCoreExtractionRequest,
) -> Result<ConflictCoreExtractionReport, ConflictGraphError> {
    let query_declaration_digest = declare_core_extraction(handle, &request)?;
    let checked = verify_k_colorability_checked(
        handle,
        request.conflict_graph.graph_version(),
        request.color_count,
    )?;
    let colorability_verification = checked.colorability_verification().clone();
    let posture = classify_core_posture(&request, &checked)?;
    let counters = core_counters(&request, &posture);
    let core = artifact_core(
        HadwigerArtifactKind::ConflictCoreExtractionReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "conflict_core_extraction".to_string(),
        },
        core_parents(&request, &colorability_verification),
        core_payload(
            &request,
            &posture,
            &counters,
            &query_declaration_digest,
            &colorability_verification,
        ),
    )?;
    Ok(ConflictCoreExtractionReport {
        core,
        core_extraction_id: request.core_extraction_id,
        conflict_graph: request.conflict_graph,
        colorability_verification: Some(colorability_verification),
        posture,
        counters,
        query_declaration_digest,
    })
}

fn classify_core_posture(
    request: &ConflictCoreExtractionRequest,
    checked: &KColorabilityVerificationChecked,
) -> Result<ConflictCoreExtractionPosture, ConflictGraphError> {
    match checked.colorability_verification().posture() {
        ColorabilityVerificationPosture::SatModelVerified => {
            Ok(ConflictCoreExtractionPosture::Colorable)
        }
        ColorabilityVerificationPosture::UnsupportedCertificateBudget => {
            Ok(ConflictCoreExtractionPosture::UnsupportedMinimality)
        }
        ColorabilityVerificationPosture::Rejected
        | ColorabilityVerificationPosture::UnsatVerified => {
            classify_minimality_certificate(request)
        }
    }
}

fn classify_minimality_certificate(
    request: &ConflictCoreExtractionRequest,
) -> Result<ConflictCoreExtractionPosture, ConflictGraphError> {
    let Some(certificate) = &request.minimality_certificate else {
        return Ok(ConflictCoreExtractionPosture::UnsupportedMinimality);
    };
    validate_deletion_proof_certificate(request, certificate)?;
    let vertex_count = request.conflict_graph.graph_version().vertices().len();
    let edge_count = request.conflict_graph.graph_version().edges().len();
    let vertex_checks =
        admitted_checks_for(certificate, ConflictCoreDeletionCheckKind::VertexRemoval);
    let edge_checks = admitted_checks_for(certificate, ConflictCoreDeletionCheckKind::EdgeRemoval);
    if vertex_checks + edge_checks != certificate.deletion_checks().len() {
        return Ok(ConflictCoreExtractionPosture::UnsupportedMinimality);
    }
    Ok(
        match (vertex_checks == vertex_count, edge_checks == edge_count) {
            (true, true) => ConflictCoreExtractionPosture::VertexAndEdgeMinimal,
            (true, false) => ConflictCoreExtractionPosture::VertexMinimal,
            (false, true) => ConflictCoreExtractionPosture::EdgeMinimal,
            (false, false) => ConflictCoreExtractionPosture::NonColorableNotMinimized,
        },
    )
}

fn admitted_checks_for(
    certificate: &ConflictCoreMinimalityCertificate,
    kind: ConflictCoreDeletionCheckKind,
) -> usize {
    certificate
        .deletion_checks()
        .iter()
        .filter(|check| check.kind() == kind && check.proves_colorable_deletion())
        .count()
}

fn core_counters(
    request: &ConflictCoreExtractionRequest,
    posture: &ConflictCoreExtractionPosture,
) -> ConflictCoreExtractionCounters {
    let deletion_candidates = request.conflict_graph.graph_version().vertices().len()
        + request.conflict_graph.graph_version().edges().len();
    let deletion_checks_attempted = request
        .minimality_certificate
        .as_ref()
        .map(|certificate| certificate.deletion_checks().len())
        .unwrap_or(0);
    let deletion_checks_admitted = request
        .minimality_certificate
        .as_ref()
        .map(|certificate| {
            certificate
                .deletion_checks()
                .iter()
                .filter(|check| check.proves_colorable_deletion())
                .count()
        })
        .unwrap_or(0);
    ConflictCoreExtractionCounters::new(
        request.conflict_graph.graph_version().vertices().len(),
        request.conflict_graph.graph_version().edges().len(),
        deletion_candidates,
        deletion_checks_attempted,
        deletion_checks_admitted,
        usize::from(matches!(
            posture,
            ConflictCoreExtractionPosture::UnsupportedMinimality
        )),
        1,
        1,
    )
}

fn declare_core_extraction(
    handle: &HadwigerResearchHandle,
    request: &ConflictCoreExtractionRequest,
) -> Result<String, ConflictGraphError> {
    let checked = declare_research_request_checked(
        handle,
        CoreExtractionDeclaration::new(
            &request.core_extraction_id,
            request.conflict_graph.reference().stable_token(),
        ),
    );
    let admitted = checked
        .admitted()
        .ok_or(ConflictGraphError::QueryDeclarationNotAdmitted {
            declaration: "core_extraction",
        })?;
    Ok(
        crate::domain_artifacts::core_artifact::canonical_digest_token(
            admitted.declaration_digest(),
        ),
    )
}

fn core_parents(
    request: &ConflictCoreExtractionRequest,
    verification: &ColorabilityVerification,
) -> Vec<HadwigerArtifactReference> {
    let mut parents = vec![request.conflict_graph.reference(), verification.reference()];
    if let Some(certificate) = &request.minimality_certificate {
        parents.extend(
            certificate
                .deletion_checks()
                .iter()
                .filter_map(|check| check.colorability_verification())
                .map(ColorabilityVerification::reference),
        );
    }
    parents
}

fn core_payload(
    request: &ConflictCoreExtractionRequest,
    posture: &ConflictCoreExtractionPosture,
    counters: &ConflictCoreExtractionCounters,
    query_declaration_digest: &str,
    verification: &ColorabilityVerification,
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text("schema", "forge.hadwiger.conflict_core.v1"),
        HadwigerArtifactPayloadEntry::text(
            "core_extraction_id",
            request.core_extraction_id.clone(),
        ),
        HadwigerArtifactPayloadEntry::text("posture", posture.as_str()),
        HadwigerArtifactPayloadEntry::text("query_declaration_digest", query_declaration_digest),
        HadwigerArtifactPayloadEntry::text(
            "colorability_verification",
            verification.reference().stable_token(),
        ),
        HadwigerArtifactPayloadEntry::text("counters", counters.stable_token()),
    ];
    if let Some(certificate) = &request.minimality_certificate {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "minimality_certificate",
            certificate.stable_token(),
        ));
    }
    payload
}
