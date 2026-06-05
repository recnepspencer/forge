use std::collections::{BTreeMap, BTreeSet};

use crate::domain_artifacts::{
    GraphVersion, HadwigerArtifactShapeError, HadwigerCanonicalArtifact,
};
use crate::mathematical_verification::verify_k_colorability_checked;
use crate::query_entry::HadwigerResearchHandle;

use super::evaluation::{
    CandidateScreeningCertificate, CandidateScreeningEvaluation, CandidateScreeningEvaluationMode,
    CandidateScreeningEvaluationReport, CandidateScreeningVerdict,
};
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
            graph_view.require_exhaustive_subset_budget(20)?;
            let omega = graph_view.clique_number();
            (
                verdict_for_bound(omega, 6),
                format!("omega={omega};color_limit=6"),
            )
        }
        CandidateScreeningInvariantFamily::IndependenceNumberLowerBound => {
            graph_view.require_exhaustive_subset_budget(20)?;
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
        CandidateScreeningInvariantFamily::HallRatioSubpatchIndependenceBound => {
            graph_view.require_exhaustive_subset_budget(20)?;
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
        CandidateScreeningInvariantFamily::SatIlpSixColorability => {
            graph_view.require_color_replay_budget(24)?;
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

struct FiniteGraphView {
    vertices: Vec<String>,
    adjacency: Vec<Vec<bool>>,
}

impl FiniteGraphView {
    fn from_graph_version(graph: &GraphVersion) -> Self {
        let vertices = graph
            .vertices()
            .iter()
            .map(|vertex| vertex.vertex_label().to_string())
            .collect::<Vec<_>>();
        let index = vertices
            .iter()
            .enumerate()
            .map(|(index, label)| (label.clone(), index))
            .collect::<BTreeMap<_, _>>();
        let mut adjacency = vec![vec![false; vertices.len()]; vertices.len()];
        for edge in graph.edges() {
            let (left, right) = edge.endpoints();
            let Some(&left_index) = index.get(left) else {
                continue;
            };
            let Some(&right_index) = index.get(right) else {
                continue;
            };
            adjacency[left_index][right_index] = true;
            adjacency[right_index][left_index] = true;
        }
        Self {
            vertices,
            adjacency,
        }
    }

    fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    fn maximum_degree(&self) -> usize {
        self.adjacency
            .iter()
            .map(|row| row.iter().filter(|adjacent| **adjacent).count())
            .max()
            .unwrap_or(0)
    }

    fn require_exhaustive_subset_budget(
        &self,
        exact_limit: usize,
    ) -> Result<(), CandidateScreeningError> {
        if self.vertex_count() <= exact_limit {
            Ok(())
        } else {
            Err(CandidateScreeningError::GraphScreeningBudgetExceeded {
                vertex_count: self.vertex_count(),
                exact_limit,
            })
        }
    }

    fn require_color_replay_budget(
        &self,
        exact_limit: usize,
    ) -> Result<(), CandidateScreeningError> {
        self.require_exhaustive_subset_budget(exact_limit)
    }

    fn clique_number(&self) -> usize {
        self.maximum_subset_size(true)
    }

    fn independence_number(&self) -> usize {
        self.maximum_subset_size(false)
    }

    fn maximum_subset_size(&self, require_edges: bool) -> usize {
        let mut best = 0;
        for subset in self.subsets() {
            if subset.len() > best && self.subset_pair_relation_holds(&subset, require_edges) {
                best = subset.len();
            }
        }
        best
    }

    fn max_hall_ratio_witness(&self) -> (usize, usize) {
        let mut best = (0, 1);
        for subset in self.subsets() {
            let alpha = self.independence_number_for_subset(&subset).max(1);
            if subset.len() * best.1 > best.0 * alpha {
                best = (subset.len(), alpha);
            }
        }
        best
    }

    fn independence_number_for_subset(&self, subset: &[usize]) -> usize {
        let set = subset.iter().copied().collect::<BTreeSet<_>>();
        self.subsets_from(subset)
            .into_iter()
            .filter(|candidate| candidate.iter().all(|index| set.contains(index)))
            .filter(|candidate| self.subset_pair_relation_holds(candidate, false))
            .map(|candidate| candidate.len())
            .max()
            .unwrap_or(0)
    }

    fn k_core_size(&self, k: usize) -> usize {
        let mut active = vec![true; self.vertex_count()];
        loop {
            let removed = (0..self.vertex_count())
                .filter(|index| active[*index] && self.active_degree(*index, &active) < k)
                .collect::<Vec<_>>();
            if removed.is_empty() {
                break;
            }
            for index in removed {
                active[index] = false;
            }
        }
        active.into_iter().filter(|value| *value).count()
    }

    fn is_k_colorable(&self, color_count: usize) -> bool {
        let mut colors = vec![None; self.vertex_count()];
        self.color_vertex(0, color_count, &mut colors)
    }

    fn color_vertex(
        &self,
        vertex_index: usize,
        color_count: usize,
        colors: &mut [Option<usize>],
    ) -> bool {
        if vertex_index == self.vertex_count() {
            return true;
        }
        for color in 0..color_count {
            if self.can_use_color(vertex_index, color, colors) {
                colors[vertex_index] = Some(color);
                if self.color_vertex(vertex_index + 1, color_count, colors) {
                    return true;
                }
                colors[vertex_index] = None;
            }
        }
        false
    }

    fn can_use_color(&self, vertex_index: usize, color: usize, colors: &[Option<usize>]) -> bool {
        self.adjacency[vertex_index]
            .iter()
            .enumerate()
            .all(|(neighbor, adjacent)| !adjacent || colors[neighbor] != Some(color))
    }

    fn active_degree(&self, index: usize, active: &[bool]) -> usize {
        self.adjacency[index]
            .iter()
            .enumerate()
            .filter(|(candidate, adjacent)| active[*candidate] && **adjacent)
            .count()
    }

    fn subset_pair_relation_holds(&self, subset: &[usize], require_edges: bool) -> bool {
        for left in 0..subset.len() {
            for right in (left + 1)..subset.len() {
                if self.adjacency[subset[left]][subset[right]] != require_edges {
                    return false;
                }
            }
        }
        true
    }

    fn subsets(&self) -> Vec<Vec<usize>> {
        let indices = (0..self.vertex_count()).collect::<Vec<_>>();
        self.subsets_from(&indices)
    }

    fn subsets_from(&self, indices: &[usize]) -> Vec<Vec<usize>> {
        let mut subsets = Vec::new();
        for mask in 1usize..(1usize << indices.len()) {
            let mut subset = Vec::new();
            for (bit, index) in indices.iter().enumerate() {
                if (mask & (1usize << bit)) != 0 {
                    subset.push(*index);
                }
            }
            subsets.push(subset);
        }
        subsets
    }
}
