use std::collections::{BTreeMap, BTreeSet};

use good_lp::{constraint, default_solver, variable, variables, Expression, Solution, SolverModel};

use crate::domain_artifacts::core_artifact::canonical_digest_token;
use crate::domain_artifacts::{
    GraphVersion, HadwigerCanonicalArtifact, HadwigerDeclaredFamilyCheckedExt,
};
use crate::domain_declarations::{
    declare_research_request_checked, FractionalChromaticScreeningDeclaration,
};
use crate::query_entry::HadwigerResearchHandle;

use super::evaluation::{
    CandidateScreeningEvaluation, CandidateScreeningEvaluationMode, CandidateScreeningVerdict,
};
use super::finite_graph_view::FiniteGraphView;
use super::optimization::{
    FractionalChromaticCertificate, ScreeningRational, ScreeningSolverTranscript,
};
use super::{
    CandidateScreeningError, CandidateScreeningInvariantCatalog, CandidateScreeningInvariantFamily,
};

const FRACTIONAL_COLOR_LIMIT: u32 = 6;
const FRACTIONAL_GRAPH_VERTEX_LIMIT: usize = 20;
const SOLVER_RATIONAL_DENOMINATOR: i128 = 1_000_000;

pub fn evaluate_fractional_chromatic_screening_checked(
    handle: &HadwigerResearchHandle,
    catalog: &CandidateScreeningInvariantCatalog,
    graph: &GraphVersion,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    let declaration = declare_research_request_checked(
        handle,
        FractionalChromaticScreeningDeclaration::new(
            graph.reference().stable_token(),
            FRACTIONAL_COLOR_LIMIT,
            "dual_lp_over_all_bounded_independent_sets",
        ),
    )
    .admitted()
    .ok_or(CandidateScreeningError::SolverCandidateUnavailable {
        family: CandidateScreeningInvariantFamily::FractionalChromaticNumber,
        reason: "query_fractional_screening_declaration_not_admitted",
    })?;
    let query_declaration_digest = canonical_digest_token(declaration.declaration_digest());
    let graph_view = FiniteGraphView::from_graph_version(graph);
    graph_view.require_subset_budget(FRACTIONAL_GRAPH_VERTEX_LIMIT)?;
    let transcript = solver_transcript(
        "good_lp",
        "fractional_chromatic_dual_lp",
        graph,
        &query_declaration_digest,
    )?;
    let certificate = solver_fractional_certificate(&graph_view, transcript).or_else(|_| {
        clique_fractional_certificate(&graph_view, graph, &query_declaration_digest)
    })?;
    evaluate_fractional_chromatic_certificate_checked_with_query_basis(
        catalog,
        graph,
        certificate,
        &query_declaration_digest,
    )
}

pub fn evaluate_fractional_chromatic_certificate_checked(
    catalog: &CandidateScreeningInvariantCatalog,
    graph: &GraphVersion,
    certificate: FractionalChromaticCertificate,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    evaluate_fractional_chromatic_certificate_checked_with_query_basis(
        catalog,
        graph,
        certificate,
        "external_fractional_chromatic_certificate",
    )
}

fn evaluate_fractional_chromatic_certificate_checked_with_query_basis(
    catalog: &CandidateScreeningInvariantCatalog,
    graph: &GraphVersion,
    certificate: FractionalChromaticCertificate,
    query_declaration_digest: &str,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    let family = CandidateScreeningInvariantFamily::FractionalChromaticNumber;
    require_catalog_family(catalog, family)?;
    let graph_view = FiniteGraphView::from_graph_version(graph);
    graph_view.require_subset_budget(FRACTIONAL_GRAPH_VERTEX_LIMIT)?;
    replay_fractional_certificate(&graph_view, &certificate)?;
    CandidateScreeningEvaluation::new(
        catalog,
        family,
        graph.reference(),
        verdict_bool(
            certificate
                .lower_bound()
                .cmp_integer(FRACTIONAL_COLOR_LIMIT as i128)
                .is_gt(),
        ),
        CandidateScreeningEvaluationMode::SolverBackedCertificate,
        format!(
            "query_declaration_digest={query_declaration_digest};fractional_dual_certificate={}",
            certificate.stable_token()
        ),
    )
    .map_err(Into::into)
}

fn solver_fractional_certificate(
    graph_view: &FiniteGraphView,
    transcript: ScreeningSolverTranscript,
) -> Result<FractionalChromaticCertificate, CandidateScreeningError> {
    let mut variables = variables!();
    let weights = graph_view
        .vertices()
        .iter()
        .map(|_| variables.add(variable().min(0.0)))
        .collect::<Vec<_>>();
    let objective = weights
        .iter()
        .fold(Expression::from(0.0), |sum, weight| sum + *weight);
    let mut problem = variables.maximise(objective).using(default_solver);
    for independent_set in graph_view.independent_sets() {
        let expression = independent_set
            .iter()
            .fold(Expression::from(0.0), |sum, index| sum + weights[*index]);
        problem = problem.with(constraint!(expression <= 1.0));
    }
    let solution =
        problem
            .solve()
            .map_err(|_| CandidateScreeningError::SolverCandidateUnavailable {
                family: CandidateScreeningInvariantFamily::FractionalChromaticNumber,
                reason: "good_lp_dual_solve_failed",
            })?;
    let vertex_weights = graph_view
        .vertices()
        .iter()
        .zip(weights.iter())
        .map(|(vertex, variable)| {
            ScreeningRational::approximate_from_f64(
                solution.value(*variable),
                SOLVER_RATIONAL_DENOMINATOR,
            )
            .map(|weight| (vertex.clone(), weight))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let lower_bound = vertex_weights
        .iter()
        .fold(ScreeningRational::integer(0), |sum, (_, weight)| {
            sum.add(weight)
        });
    FractionalChromaticCertificate::new(
        "good-lp-dual-candidate",
        vertex_weights,
        lower_bound,
        transcript,
    )
    .map_err(Into::into)
}

fn clique_fractional_certificate(
    graph_view: &FiniteGraphView,
    graph: &GraphVersion,
    query_declaration_digest: &str,
) -> Result<FractionalChromaticCertificate, CandidateScreeningError> {
    let clique = graph_view.maximum_clique_witness();
    let transcript = solver_transcript(
        "in_crate_exact_replay",
        "fractional_chromatic_clique_dual_fallback",
        graph,
        query_declaration_digest,
    )?;
    let clique_vertices = clique.into_iter().collect::<BTreeSet<_>>();
    let vertex_weights = graph_view
        .vertices()
        .iter()
        .enumerate()
        .map(|(index, label)| {
            let weight = if clique_vertices.contains(&index) {
                ScreeningRational::integer(1)
            } else {
                ScreeningRational::integer(0)
            };
            (label.clone(), weight)
        })
        .collect::<Vec<_>>();
    let lower_bound = vertex_weights
        .iter()
        .fold(ScreeningRational::integer(0), |sum, (_, weight)| {
            sum.add(weight)
        });
    FractionalChromaticCertificate::new(
        "exact-clique-dual-fallback",
        vertex_weights,
        lower_bound,
        transcript,
    )
    .map_err(Into::into)
}

fn replay_fractional_certificate(
    graph_view: &FiniteGraphView,
    certificate: &FractionalChromaticCertificate,
) -> Result<(), CandidateScreeningError> {
    let weights = certificate_weight_map(graph_view, certificate)?;
    let total = graph_view
        .vertices()
        .iter()
        .fold(ScreeningRational::integer(0), |sum, vertex| {
            let weight = weights
                .get(vertex.as_str())
                .cloned()
                .unwrap_or_else(|| ScreeningRational::integer(0));
            sum.add(&weight)
        });
    if total != *certificate.lower_bound() {
        return Err(replay_error("lower_bound_does_not_match_weight_sum"));
    }
    for independent_set in graph_view.independent_sets() {
        let set_sum = independent_set
            .iter()
            .fold(ScreeningRational::integer(0), |sum, index| {
                let vertex = &graph_view.vertices()[*index];
                let weight = weights
                    .get(vertex.as_str())
                    .cloned()
                    .unwrap_or_else(|| ScreeningRational::integer(0));
                sum.add(&weight)
            });
        if set_sum.cmp_integer(1).is_gt() {
            return Err(replay_error("independent_set_dual_constraint_violated"));
        }
    }
    Ok(())
}

fn certificate_weight_map<'a>(
    graph_view: &'a FiniteGraphView,
    certificate: &'a FractionalChromaticCertificate,
) -> Result<BTreeMap<&'a str, ScreeningRational>, CandidateScreeningError> {
    let graph_vertices = graph_view
        .vertices()
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let mut weights = BTreeMap::new();
    for (vertex, weight) in certificate.vertex_weights() {
        if !graph_vertices.contains(vertex.as_str()) {
            return Err(replay_error("unknown_certificate_vertex"));
        }
        if weights.insert(vertex.as_str(), weight.clone()).is_some() {
            return Err(replay_error("duplicate_certificate_vertex"));
        }
        if weight.is_negative() {
            return Err(replay_error("negative_certificate_weight"));
        }
    }
    Ok(weights)
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
        family: CandidateScreeningInvariantFamily::FractionalChromaticNumber,
        reason,
    }
}
