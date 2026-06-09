use std::collections::{BTreeMap, BTreeSet};

use crate::domain_artifacts::{GraphVersion, HadwigerCanonicalArtifact};
use crate::domain_declarations::SymmetryOrbitReductionScreeningDeclaration;
use crate::query_entry::HadwigerResearchHandle;

use super::evaluation::{CandidateScreeningEvaluation, CandidateScreeningVerdict};
use super::graph_embedding_index::ScreeningFiniteGraphIndex;
use super::graph_embedding_screening_support::{
    declare_screening_request, replay_error, require_catalog_family, screening_evaluation,
};
use super::optimization::SymmetryOrbitReductionCertificate;
use super::{CandidateScreeningError, CandidateScreeningInvariantCatalog};
use crate::candidate_screening::CandidateScreeningInvariantFamily;

pub fn evaluate_symmetry_orbit_reduction_screening_checked(
    handle: &HadwigerResearchHandle,
    catalog: &CandidateScreeningInvariantCatalog,
    graph: &GraphVersion,
    certificate: SymmetryOrbitReductionCertificate,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    let family = CandidateScreeningInvariantFamily::SymmetryOrbitReduction;
    require_catalog_family(catalog, family)?;
    let query_digest = declare_screening_request(
        handle,
        family,
        SymmetryOrbitReductionScreeningDeclaration::new(
            graph.reference().stable_token(),
            certificate.stable_token(),
        ),
        "query_symmetry_orbit_reduction_screening_declaration_not_admitted",
    )?;
    let graph_index = ScreeningFiniteGraphIndex::from_graph_version(graph);
    for permutation in certificate.permutations() {
        if !graph_index.permutation_preserves_edges(permutation) {
            return Err(replay_error(
                family,
                "symmetry_permutation_not_automorphism",
            ));
        }
    }
    let orbit_count = orbit_partition(&graph_index, certificate.permutations()).len();
    screening_evaluation(
        catalog,
        family,
        graph.reference(),
        CandidateScreeningVerdict::Priority,
        &query_digest,
        format!(
            "automorphism_count={};orbit_count={orbit_count};certificate={}",
            certificate.permutations().len(),
            certificate.stable_token()
        ),
    )
}

fn orbit_partition(
    graph_index: &ScreeningFiniteGraphIndex,
    permutations: &[Vec<(String, String)>],
) -> Vec<BTreeSet<String>> {
    let mut parent = graph_index
        .vertices()
        .iter()
        .map(|vertex| (vertex.clone(), vertex.clone()))
        .collect::<BTreeMap<_, _>>();
    for permutation in permutations {
        for (left, right) in permutation {
            union(&mut parent, left, right);
        }
    }
    let mut orbits = BTreeMap::<String, BTreeSet<String>>::new();
    for vertex in graph_index.vertices() {
        let root = find(&mut parent, vertex);
        orbits.entry(root).or_default().insert(vertex.clone());
    }
    orbits.into_values().collect()
}

fn union(parent: &mut BTreeMap<String, String>, left: &str, right: &str) {
    let left_root = find(parent, left);
    let right_root = find(parent, right);
    if left_root != right_root {
        parent.insert(right_root, left_root);
    }
}

fn find(parent: &mut BTreeMap<String, String>, vertex: &str) -> String {
    let current = parent
        .get(vertex)
        .cloned()
        .unwrap_or_else(|| vertex.to_string());
    if current == vertex {
        current
    } else {
        let root = find(parent, &current);
        parent.insert(vertex.to_string(), root.clone());
        root
    }
}
