use std::collections::{BTreeMap, BTreeSet};

use crate::domain_artifacts::core_artifact::canonical_digest_token;
use crate::domain_artifacts::{
    GraphVersion, HadwigerCanonicalArtifact, HadwigerDeclaredFamilyCheckedExt,
};
use crate::domain_declarations::{
    declare_research_request_checked, GeometricFractionalChromaticScreeningDeclaration,
};
use crate::query_entry::HadwigerResearchHandle;

use super::evaluation::{
    CandidateScreeningEvaluation, CandidateScreeningEvaluationMode, CandidateScreeningVerdict,
};
use super::finite_graph_view::FiniteGraphView;
use super::optimization::{
    GeometricFractionalChromaticCertificate, GeometricFractionalEqualityAdjustment,
    GeometricSubsetIsometryWitness, ScreeningRational,
};
use super::{
    CandidateScreeningError, CandidateScreeningInvariantCatalog, CandidateScreeningInvariantFamily,
};

const GEOMETRIC_FRACTIONAL_GRAPH_VERTEX_LIMIT: usize = 30;

pub fn evaluate_geometric_fractional_chromatic_screening_checked(
    handle: &HadwigerResearchHandle,
    catalog: &CandidateScreeningInvariantCatalog,
    graph: &GraphVersion,
    certificate: GeometricFractionalChromaticCertificate,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    let declaration = declare_research_request_checked(
        handle,
        GeometricFractionalChromaticScreeningDeclaration::new(
            graph.reference().stable_token(),
            certificate.target_lower_bound().stable_token(),
            "geometric_fractional_dual_with_exact_isometry_replay",
        ),
    )
    .admitted()
    .ok_or(CandidateScreeningError::SolverCandidateUnavailable {
        family: CandidateScreeningInvariantFamily::GeometricFractionalChromaticNumber,
        reason: "query_geometric_fractional_screening_declaration_not_admitted",
    })?;
    let query_declaration_digest = canonical_digest_token(declaration.declaration_digest());
    evaluate_geometric_fractional_chromatic_certificate_checked_with_query_basis(
        catalog,
        graph,
        certificate,
        &query_declaration_digest,
    )
}

pub fn evaluate_geometric_fractional_chromatic_certificate_checked(
    catalog: &CandidateScreeningInvariantCatalog,
    graph: &GraphVersion,
    certificate: GeometricFractionalChromaticCertificate,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    evaluate_geometric_fractional_chromatic_certificate_checked_with_query_basis(
        catalog,
        graph,
        certificate,
        "external_geometric_fractional_chromatic_certificate",
    )
}

fn evaluate_geometric_fractional_chromatic_certificate_checked_with_query_basis(
    catalog: &CandidateScreeningInvariantCatalog,
    graph: &GraphVersion,
    certificate: GeometricFractionalChromaticCertificate,
    query_declaration_digest: &str,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    let family = CandidateScreeningInvariantFamily::GeometricFractionalChromaticNumber;
    require_catalog_family(catalog, family)?;
    let graph_view = FiniteGraphView::from_graph_version(graph);
    graph_view.require_subset_budget(GEOMETRIC_FRACTIONAL_GRAPH_VERTEX_LIMIT)?;
    replay_geometric_fractional_certificate(&graph_view, &certificate)?;
    let verdict = if certificate.lower_bound() >= certificate.target_lower_bound() {
        CandidateScreeningVerdict::Priority
    } else {
        CandidateScreeningVerdict::Passed
    };
    CandidateScreeningEvaluation::new(
        catalog,
        family,
        graph.reference(),
        verdict,
        CandidateScreeningEvaluationMode::CheckedCertificate,
        format!(
            "query_declaration_digest={query_declaration_digest};geometric_fractional_certificate={}",
            certificate.stable_token()
        ),
    )
    .map_err(Into::into)
}

fn replay_geometric_fractional_certificate(
    graph_view: &FiniteGraphView,
    certificate: &GeometricFractionalChromaticCertificate,
) -> Result<(), CandidateScreeningError> {
    if certificate
        .search_scope()
        .suppresses_improvement_without_escape()
        && certificate.target_lower_bound().cmp_integer(4).is_gt()
    {
        return Err(replay_error(
            "suppressed_moser_scope_without_escape_evidence",
        ));
    }
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
    for adjustment in certificate.equality_adjustments() {
        replay_adjustment(graph_view, adjustment)?;
    }
    let vertex_index = graph_view
        .vertices()
        .iter()
        .enumerate()
        .map(|(index, vertex)| (vertex.as_str(), index))
        .collect::<BTreeMap<_, _>>();
    for independent_set in graph_view.independent_sets() {
        let set_sum = weighted_independent_set_sum(graph_view, &weights, &independent_set);
        let adjusted_sum =
            certificate
                .equality_adjustments()
                .iter()
                .fold(set_sum, |sum, adjustment| {
                    sum.add(&adjustment_contribution(
                        adjustment,
                        &vertex_index,
                        &independent_set,
                    ))
                });
        if adjusted_sum.cmp_integer(1).is_gt() {
            return Err(replay_error(
                "geometric_independent_set_constraint_violated",
            ));
        }
    }
    Ok(())
}

fn replay_adjustment(
    graph_view: &FiniteGraphView,
    adjustment: &GeometricFractionalEqualityAdjustment,
) -> Result<(), CandidateScreeningError> {
    let graph_vertices = graph_view
        .vertices()
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    require_known_subset(&graph_vertices, adjustment.left_subset())?;
    require_known_subset(&graph_vertices, adjustment.right_subset())?;
    replay_isometry_witness(
        adjustment.left_subset(),
        adjustment.right_subset(),
        adjustment.isometry_witness(),
    )
}

fn replay_isometry_witness(
    left_subset: &[String],
    right_subset: &[String],
    witness: &GeometricSubsetIsometryWitness,
) -> Result<(), CandidateScreeningError> {
    let left_expected = left_subset
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let right_expected = right_subset
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let mut left_seen = BTreeSet::new();
    let mut right_seen = BTreeSet::new();
    let mut mapping = BTreeMap::new();
    for (left, right) in witness.mapping() {
        if !left_seen.insert(left.as_str()) || !right_seen.insert(right.as_str()) {
            return Err(replay_error("duplicate_isometry_mapping_vertex"));
        }
        mapping.insert(left.as_str(), right.as_str());
    }
    if left_seen != left_expected || right_seen != right_expected {
        return Err(replay_error("isometry_mapping_does_not_match_subsets"));
    }
    let distance_map = witness
        .pairwise_squared_distances()
        .iter()
        .map(|distance| {
            (
                (distance.left_pair().clone(), distance.right_pair().clone()),
                (
                    distance.left_squared_distance().clone(),
                    distance.right_squared_distance().clone(),
                ),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let left_vertices = left_expected.into_iter().collect::<Vec<_>>();
    for left_index in 0..left_vertices.len() {
        for right_index in (left_index + 1)..left_vertices.len() {
            let left_pair = normalized_pair(left_vertices[left_index], left_vertices[right_index]);
            let mapped_left = mapping[left_vertices[left_index]];
            let mapped_right = mapping[left_vertices[right_index]];
            let right_pair = normalized_pair(mapped_left, mapped_right);
            let Some((left_distance, right_distance)) = distance_map.get(&(left_pair, right_pair))
            else {
                return Err(replay_error("missing_pairwise_isometry_distance"));
            };
            if left_distance != right_distance {
                return Err(replay_error("pairwise_isometry_distance_mismatch"));
            }
        }
    }
    Ok(())
}

fn certificate_weight_map<'a>(
    graph_view: &'a FiniteGraphView,
    certificate: &'a GeometricFractionalChromaticCertificate,
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

fn weighted_independent_set_sum(
    graph_view: &FiniteGraphView,
    weights: &BTreeMap<&str, ScreeningRational>,
    independent_set: &[usize],
) -> ScreeningRational {
    independent_set
        .iter()
        .fold(ScreeningRational::integer(0), |sum, index| {
            let vertex = &graph_view.vertices()[*index];
            let weight = weights
                .get(vertex.as_str())
                .cloned()
                .unwrap_or_else(|| ScreeningRational::integer(0));
            sum.add(&weight)
        })
}

fn adjustment_contribution(
    adjustment: &GeometricFractionalEqualityAdjustment,
    vertex_index: &BTreeMap<&str, usize>,
    independent_set: &[usize],
) -> ScreeningRational {
    let independent_vertices = independent_set.iter().copied().collect::<BTreeSet<_>>();
    let left = subset_indicator(
        adjustment.left_subset(),
        vertex_index,
        &independent_vertices,
    );
    let right = subset_indicator(
        adjustment.right_subset(),
        vertex_index,
        &independent_vertices,
    );
    adjustment
        .multiplier()
        .mul(&ScreeningRational::integer(left - right))
}

fn subset_indicator(
    subset: &[String],
    vertex_index: &BTreeMap<&str, usize>,
    independent_vertices: &BTreeSet<usize>,
) -> i128 {
    if subset
        .iter()
        .all(|vertex| independent_vertices.contains(&vertex_index[vertex.as_str()]))
    {
        1
    } else {
        0
    }
}

fn require_known_subset(
    graph_vertices: &BTreeSet<&str>,
    subset: &[String],
) -> Result<(), CandidateScreeningError> {
    if subset
        .iter()
        .all(|vertex| graph_vertices.contains(vertex.as_str()))
    {
        Ok(())
    } else {
        Err(replay_error("unknown_geometric_subset_vertex"))
    }
}

fn normalized_pair(left: &str, right: &str) -> (String, String) {
    if left <= right {
        (left.to_string(), right.to_string())
    } else {
        (right.to_string(), left.to_string())
    }
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

fn replay_error(reason: &'static str) -> CandidateScreeningError {
    CandidateScreeningError::CertificateReplayRejected {
        family: CandidateScreeningInvariantFamily::GeometricFractionalChromaticNumber,
        reason,
    }
}
