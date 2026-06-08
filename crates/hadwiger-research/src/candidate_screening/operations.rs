use crate::domain_artifacts::{
    GraphVersion, HadwigerArtifactShapeError, HadwigerCanonicalArtifact,
};
use crate::mathematical_verification::verify_k_colorability_checked;
use crate::query_entry::HadwigerResearchHandle;

use super::evaluation::{
    CandidateScreeningCertificate, CandidateScreeningEvaluation, CandidateScreeningEvaluationMode,
    CandidateScreeningEvaluationReport, CandidateScreeningVerdict,
};
use super::finite_graph_view::FiniteGraphView;
use super::{CandidateScreeningInvariantCatalog, CandidateScreeningInvariantFamily};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CandidateScreeningError {
    Shape(HadwigerArtifactShapeError),
    MissingInvariantFamily(CandidateScreeningInvariantFamily),
    GraphScreeningBudgetExceeded {
        vertex_count: usize,
        exact_limit: usize,
    },
    CertificateFamilyMismatch {
        requested: CandidateScreeningInvariantFamily,
        certificate: CandidateScreeningInvariantFamily,
    },
    CertificateReplayRejected {
        family: CandidateScreeningInvariantFamily,
        reason: &'static str,
    },
}

impl From<HadwigerArtifactShapeError> for CandidateScreeningError {
    fn from(value: HadwigerArtifactShapeError) -> Self {
        Self::Shape(value)
    }
}

pub fn evaluate_graph_screening_invariant_checked(
    handle: &HadwigerResearchHandle,
    catalog: &CandidateScreeningInvariantCatalog,
    family: CandidateScreeningInvariantFamily,
    graph: &GraphVersion,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    require_catalog_family(catalog, family)?;
    let graph_view = FiniteGraphView::from_graph_version(graph);
    let (verdict, evidence) = match family {
        CandidateScreeningInvariantFamily::CliqueNumberLowerBound => {
            graph_view.require_subset_budget(20)?;
            let omega = graph_view.clique_number();
            (
                verdict_for_bound(omega, 6),
                format!("omega={omega};color_limit=6"),
            )
        }
        CandidateScreeningInvariantFamily::IndependenceNumberLowerBound => {
            graph_view.require_subset_budget(20)?;
            let alpha = graph_view.independence_number().max(1);
            let rejects = graph_view.vertex_count() > 6 * alpha;
            (
                verdict_bool(rejects),
                format!(
                    "vertices={};alpha={alpha};color_limit=6",
                    graph_view.vertex_count()
                ),
            )
        }
        CandidateScreeningInvariantFamily::WeightedIndependenceNumberBound => {
            graph_view.require_subset_budget(20)?;
            let alpha = graph_view.independence_number().max(1);
            let rejects = graph_view.vertex_count() > 6 * alpha;
            (
                verdict_bool(rejects),
                format!(
                    "unit_weights=true;total_weight={};alpha_weight={alpha};color_limit=6",
                    graph_view.vertex_count()
                ),
            )
        }
        CandidateScreeningInvariantFamily::HallRatioSubpatchIndependenceBound => {
            graph_view.require_subset_budget(20)?;
            let (vertices, alpha) = graph_view.max_hall_ratio_witness();
            let rejects = vertices > 6 * alpha.max(1);
            (
                verdict_bool(rejects),
                format!("subpatch_vertices={vertices};subpatch_alpha={alpha};color_limit=6"),
            )
        }
        CandidateScreeningInvariantFamily::DegeneracyKCoreFilter => {
            let core_size = graph_view.k_core_size(6);
            (
                if core_size == 0 {
                    CandidateScreeningVerdict::Passed
                } else {
                    CandidateScreeningVerdict::Priority
                },
                format!("six_core_size={core_size}"),
            )
        }
        CandidateScreeningInvariantFamily::MaximumDegreeSanityCheck => {
            let max_degree = graph_view.maximum_degree();
            (
                if max_degree <= 6 {
                    CandidateScreeningVerdict::Priority
                } else {
                    CandidateScreeningVerdict::Passed
                },
                format!("maximum_degree={max_degree};sanity_threshold=6"),
            )
        }
        CandidateScreeningInvariantFamily::PerfectGraphSanityCheck => {
            graph_view.require_subset_budget(20)?;
            let omega = graph_view.clique_number();
            let bipartite = graph_view.is_bipartite();
            (
                if bipartite && omega <= 6 {
                    CandidateScreeningVerdict::Rejected
                } else {
                    CandidateScreeningVerdict::Passed
                },
                format!("perfect_subclass=bipartite;detected={bipartite};omega={omega}"),
            )
        }
        CandidateScreeningInvariantFamily::SpectralHoffmanBound => {
            let (verdict, evidence) = hoffman_bound_screening(&graph_view);
            (verdict, evidence)
        }
        CandidateScreeningInvariantFamily::SatIlpSixColorability => {
            graph_view.require_subset_budget(24)?;
            let checked = verify_k_colorability_checked(handle, graph, 6).map_err(|_| {
                CandidateScreeningError::Shape(HadwigerArtifactShapeError::EmptyField {
                    field: "sat_screening",
                })
            })?;
            let posture = checked.colorability_verification().posture();
            let six_colorable = graph_view.is_k_colorable(6);
            (
                verdict_bool(!six_colorable),
                format!(
                    "color_limit=6;exact_replay_colorable={six_colorable};verification_posture={};artifact={}",
                    posture.as_str(),
                    checked
                        .colorability_verification()
                        .reference()
                        .stable_token()
                ),
            )
        }
        CandidateScreeningInvariantFamily::CriticalSubgraphExtraction => {
            graph_view.require_subset_budget(16)?;
            let colorable = graph_view.is_k_colorable(6);
            let smaller_obstruction =
                !colorable && graph_view.has_smaller_non_k_colorable_subgraph(6);
            (
                if smaller_obstruction {
                    CandidateScreeningVerdict::Rejected
                } else if colorable {
                    CandidateScreeningVerdict::Passed
                } else {
                    CandidateScreeningVerdict::Priority
                },
                format!(
                    "color_limit=6;colorable={colorable};smaller_non_colorable_subgraph={smaller_obstruction}"
                ),
            )
        }
        _ => {
            return Err(CandidateScreeningError::Shape(
                HadwigerArtifactShapeError::EmptyField {
                    field: "direct_graph_screening_family",
                },
            ))
        }
    };
    CandidateScreeningEvaluation::new(
        catalog,
        family,
        graph.reference(),
        verdict,
        CandidateScreeningEvaluationMode::DirectGraphAlgorithm,
        evidence,
    )
    .map_err(Into::into)
}

pub fn evaluate_certificate_screening_invariant_checked(
    _handle: &HadwigerResearchHandle,
    catalog: &CandidateScreeningInvariantCatalog,
    family: CandidateScreeningInvariantFamily,
    certificate: CandidateScreeningCertificate,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    require_catalog_family(catalog, family)?;
    if certificate.family() != family {
        return Err(CandidateScreeningError::CertificateFamilyMismatch {
            requested: family,
            certificate: certificate.family(),
        });
    }
    CandidateScreeningEvaluation::new(
        catalog,
        family,
        certificate.subject().clone(),
        certificate.verdict(),
        CandidateScreeningEvaluationMode::CheckedCertificate,
        certificate.stable_token(),
    )
    .map_err(Into::into)
}

pub fn assemble_candidate_screening_report_checked(
    _handle: &HadwigerResearchHandle,
    catalog: &CandidateScreeningInvariantCatalog,
    evaluations: Vec<CandidateScreeningEvaluation>,
) -> Result<CandidateScreeningEvaluationReport, CandidateScreeningError> {
    CandidateScreeningEvaluationReport::new(catalog, evaluations).map_err(Into::into)
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

fn verdict_for_bound(value: usize, color_limit: usize) -> CandidateScreeningVerdict {
    verdict_bool(value > color_limit)
}

fn verdict_bool(rejects: bool) -> CandidateScreeningVerdict {
    if rejects {
        CandidateScreeningVerdict::Rejected
    } else {
        CandidateScreeningVerdict::Passed
    }
}

fn hoffman_bound_screening(graph_view: &FiniteGraphView) -> (CandidateScreeningVerdict, String) {
    let Some(degree) = graph_view.is_regular() else {
        return (
            CandidateScreeningVerdict::Passed,
            "regular=false;hoffman_bound=unsupported".to_string(),
        );
    };
    if graph_view.is_complete() && graph_view.vertex_count() > 1 {
        let bound = graph_view.vertex_count();
        return (
            verdict_for_bound(bound, 6),
            format!(
                "regular=true;complete_graph=true;degree={degree};lambda_min=-1;hoffman_bound={bound}"
            ),
        );
    }
    (
        CandidateScreeningVerdict::Passed,
        format!("regular=true;degree={degree};hoffman_bound=certificate_required"),
    )
}
