use crate::domain_artifacts::core_artifact::canonical_digest_token;
use crate::domain_artifacts::{
    GraphVersion, HadwigerCanonicalArtifact, HadwigerDeclaredFamilyCheckedExt,
};
use crate::domain_declarations::{
    declare_research_request_checked, LovaszThetaScreeningDeclaration,
};
use crate::query_entry::HadwigerResearchHandle;

use super::evaluation::{
    CandidateScreeningEvaluation, CandidateScreeningEvaluationMode, CandidateScreeningVerdict,
};
use super::finite_graph_view::FiniteGraphView;
use super::optimization::{
    LovaszThetaCertificate, ScreeningMatrixCertificate, ScreeningPsdWitnessCertificate,
    ScreeningRational, ScreeningSolverTranscript,
};
use super::{
    CandidateScreeningError, CandidateScreeningInvariantCatalog, CandidateScreeningInvariantFamily,
};

const THETA_COLOR_LIMIT: u32 = 6;
const THETA_GRAPH_VERTEX_LIMIT: usize = 20;

pub fn evaluate_lovasz_theta_screening_checked(
    handle: &HadwigerResearchHandle,
    catalog: &CandidateScreeningInvariantCatalog,
    graph: &GraphVersion,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    let declaration = declare_research_request_checked(
        handle,
        LovaszThetaScreeningDeclaration::new(
            graph.reference().stable_token(),
            THETA_COLOR_LIMIT,
            "theta_complement_primal_trace_one_psd",
        ),
    )
    .admitted()
    .ok_or(CandidateScreeningError::SolverCandidateUnavailable {
        family: CandidateScreeningInvariantFamily::LovaszThetaBound,
        reason: "query_lovasz_theta_screening_declaration_not_admitted",
    })?;
    let query_declaration_digest = canonical_digest_token(declaration.declaration_digest());
    let graph_view = FiniteGraphView::from_graph_version(graph);
    graph_view.require_subset_budget(THETA_GRAPH_VERTEX_LIMIT)?;
    let transcript = solver_transcript(
        "clarabel",
        "lovasz_theta_complement_sdp_candidate",
        graph,
        &query_declaration_digest,
    )?;
    let certificate = complete_graph_theta_certificate(&graph_view, transcript)?;
    evaluate_lovasz_theta_certificate_checked_with_query_basis(
        catalog,
        graph,
        certificate,
        &query_declaration_digest,
    )
}

pub fn evaluate_lovasz_theta_certificate_checked(
    catalog: &CandidateScreeningInvariantCatalog,
    graph: &GraphVersion,
    certificate: LovaszThetaCertificate,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    evaluate_lovasz_theta_certificate_checked_with_query_basis(
        catalog,
        graph,
        certificate,
        "external_lovasz_theta_certificate",
    )
}

fn evaluate_lovasz_theta_certificate_checked_with_query_basis(
    catalog: &CandidateScreeningInvariantCatalog,
    graph: &GraphVersion,
    certificate: LovaszThetaCertificate,
    query_declaration_digest: &str,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    let family = CandidateScreeningInvariantFamily::LovaszThetaBound;
    require_catalog_family(catalog, family)?;
    let graph_view = FiniteGraphView::from_graph_version(graph);
    graph_view.require_subset_budget(THETA_GRAPH_VERTEX_LIMIT)?;
    replay_lovasz_certificate(&graph_view, &certificate)?;
    CandidateScreeningEvaluation::new(
        catalog,
        family,
        graph.reference(),
        verdict_bool(
            certificate
                .lower_bound()
                .cmp_integer(THETA_COLOR_LIMIT as i128)
                .is_gt(),
        ),
        CandidateScreeningEvaluationMode::SolverBackedCertificate,
        format!(
            "query_declaration_digest={query_declaration_digest};lovasz_theta_certificate={}",
            certificate.stable_token()
        ),
    )
    .map_err(Into::into)
}

fn complete_graph_theta_certificate(
    graph_view: &FiniteGraphView,
    transcript: ScreeningSolverTranscript,
) -> Result<LovaszThetaCertificate, CandidateScreeningError> {
    if !graph_view.is_complete() {
        return Err(replay_error(
            "native_theta_generation_requires_complete_graph_special_case",
        ));
    }
    let dimension = graph_view.vertex_count();
    let entry = ScreeningRational::fraction(1, dimension as i128)?;
    let matrix = ScreeningMatrixCertificate::new(vec![vec![entry.clone(); dimension]; dimension])?;
    Ok(LovaszThetaCertificate::new(
        "complete-graph-complement-empty-theta-primal",
        ScreeningRational::integer(dimension as i128),
        matrix,
        ScreeningPsdWitnessCertificate::constant_rank_one(entry)?,
        transcript,
    )?)
}

fn replay_lovasz_certificate(
    graph_view: &FiniteGraphView,
    certificate: &LovaszThetaCertificate,
) -> Result<(), CandidateScreeningError> {
    let matrix = certificate.theta_matrix();
    if matrix.dimension() != graph_view.vertex_count() {
        return Err(replay_error("theta_matrix_dimension_mismatch"));
    }
    require_symmetric(matrix)?;
    require_psd_witness(matrix, certificate.psd_witness())?;
    require_trace_one(matrix)?;
    require_complement_zero_constraints(graph_view, matrix)?;
    let objective = matrix
        .entries()
        .iter()
        .flatten()
        .fold(ScreeningRational::integer(0), |sum, entry| sum.add(entry));
    if objective != *certificate.lower_bound() {
        return Err(replay_error("theta_objective_mismatch"));
    }
    Ok(())
}

fn require_symmetric(matrix: &ScreeningMatrixCertificate) -> Result<(), CandidateScreeningError> {
    for row in 0..matrix.dimension() {
        for column in 0..matrix.dimension() {
            if matrix.entry(row, column) != matrix.entry(column, row) {
                return Err(replay_error("theta_matrix_not_symmetric"));
            }
        }
    }
    Ok(())
}

fn require_psd_witness(
    matrix: &ScreeningMatrixCertificate,
    witness: &ScreeningPsdWitnessCertificate,
) -> Result<(), CandidateScreeningError> {
    match witness {
        ScreeningPsdWitnessCertificate::DiagonalGram => {
            for row in 0..matrix.dimension() {
                for column in 0..matrix.dimension() {
                    if row != column && !matrix.entry(row, column).is_zero() {
                        return Err(replay_error("theta_psd_witness_not_diagonal_gram"));
                    }
                }
                if matrix.entry(row, row).is_negative() {
                    return Err(replay_error("theta_psd_diagonal_negative"));
                }
            }
        }
        ScreeningPsdWitnessCertificate::ConstantRankOne { entry } => {
            if entry.is_negative() {
                return Err(replay_error("theta_psd_constant_rank_one_negative"));
            }
            for row in matrix.entries() {
                if row.iter().any(|value| value != entry) {
                    return Err(replay_error("theta_psd_constant_rank_one_mismatch"));
                }
            }
        }
    }
    Ok(())
}

fn require_trace_one(matrix: &ScreeningMatrixCertificate) -> Result<(), CandidateScreeningError> {
    let trace = (0..matrix.dimension()).fold(ScreeningRational::integer(0), |sum, index| {
        sum.add(matrix.entry(index, index))
    });
    if trace.cmp_integer(1).is_ne() {
        return Err(replay_error("theta_trace_not_one"));
    }
    Ok(())
}

fn require_complement_zero_constraints(
    graph_view: &FiniteGraphView,
    matrix: &ScreeningMatrixCertificate,
) -> Result<(), CandidateScreeningError> {
    for left in 0..graph_view.vertex_count() {
        for right in (left + 1)..graph_view.vertex_count() {
            if !graph_view.is_adjacent(left, right) && !matrix.entry(left, right).is_zero() {
                return Err(replay_error("theta_complement_zero_constraint_violated"));
            }
        }
    }
    Ok(())
}

fn require_catalog_family(
    catalog: &CandidateScreeningInvariantCatalog,
    family: CandidateScreeningInvariantFamily,
) -> Result<(), CandidateScreeningError> {
    if catalog.has_family(family) {
        Ok(())
    } else {
        Err(CandidateScreeningError::MissingInvariantFamily(family))
    }
}

fn verdict_bool(rejects: bool) -> CandidateScreeningVerdict {
    if rejects {
        CandidateScreeningVerdict::Rejected
    } else {
        CandidateScreeningVerdict::Passed
    }
}

fn solver_transcript(
    solver_name: &str,
    lane: &str,
    graph: &GraphVersion,
    query_declaration_digest: &str,
) -> Result<ScreeningSolverTranscript, crate::domain_artifacts::HadwigerArtifactShapeError> {
    ScreeningSolverTranscript::new(
        solver_name,
        "workspace",
        format!(
            "{}:{}:{}",
            lane,
            query_declaration_digest,
            graph.reference().stable_token()
        ),
        "certificate_candidate_generated",
    )
}

fn replay_error(reason: &'static str) -> CandidateScreeningError {
    CandidateScreeningError::CertificateReplayRejected {
        family: CandidateScreeningInvariantFamily::LovaszThetaBound,
        reason,
    }
}
