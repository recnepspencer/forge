use sha2::{Digest, Sha256};

use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_same_field_fixed_dual_pricing_support::{greedy_independent_witness, has_bit};
use super::g27_same_field_structure_preflight_support::{
    connected_components, exact_small_component_weight,
};
use super::g27_same_field_threshold_mwis_bnb_setup::threshold_mwis_instance;

const TARGET_WEIGHT: i128 = 512_933;
const SCHEMA: &str = "forge.hadwiger.g27_same_field_dominant_mwis.v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27DominantMwisArtifact {
    schema: &'static str,
    compatible_w_vertices: Vec<usize>,
    dominant_vertices: Vec<usize>,
    dominant_weights: Vec<i128>,
    dominant_edges: Vec<(usize, usize)>,
    exact_small_component_weight: i128,
    dominant_required_weight: i128,
    incumbent_vertices: Vec<usize>,
    incumbent_dominant_weight: i128,
    stable_digest: String,
}

impl G27DominantMwisArtifact {
    pub fn instance_summary(&self) -> (usize, usize, usize, i128, i128, i128) {
        (
            self.compatible_w_vertices.len(),
            self.dominant_vertices.len(),
            self.dominant_edges.len(),
            self.exact_small_component_weight,
            self.dominant_required_weight,
            self.incumbent_dominant_weight,
        )
    }

    pub fn stable_digest(&self) -> &str {
        &self.stable_digest
    }

    pub fn dominant_vertices(&self) -> &[usize] {
        &self.dominant_vertices
    }

    pub fn dominant_weights(&self) -> &[i128] {
        &self.dominant_weights
    }

    pub fn dominant_edges(&self) -> &[(usize, usize)] {
        &self.dominant_edges
    }

    pub fn incumbent_vertices(&self) -> &[usize] {
        &self.incumbent_vertices
    }

    pub fn line_export(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!("schema {SCHEMA}"));
        lines.push("provenance g27_anchor=23 w_anchor=254 retained_top_slack=0".to_string());
        lines.push(format!("global_target {TARGET_WEIGHT}"));
        lines.push(format!(
            "small_component_weight {}",
            self.exact_small_component_weight
        ));
        lines.push(format!("dominant_target {}", self.dominant_required_weight));
        lines.push(format!(
            "compatible_count {}",
            self.compatible_w_vertices.len()
        ));
        lines.push(format!("dominant_count {}", self.dominant_vertices.len()));
        lines.push(format!("edge_count {}", self.dominant_edges.len()));
        for (local, (vertex, weight)) in self
            .dominant_vertices
            .iter()
            .zip(self.dominant_weights.iter())
            .enumerate()
        {
            lines.push(format!("v {} {} {}", local + 1, vertex, weight));
        }
        for (left, right) in &self.dominant_edges {
            lines.push(format!(
                "e {} {} {} {}",
                left + 1,
                right + 1,
                self.dominant_vertices[*left],
                self.dominant_vertices[*right]
            ));
        }
        let incumbent = self
            .incumbent_vertices
            .iter()
            .map(usize::to_string)
            .collect::<Vec<_>>()
            .join(",");
        lines.push(format!("incumbent {}", incumbent));
        lines.push(format!("digest {}", self.stable_digest));
        lines.join("\n")
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27MwisWitnessReplayStatus {
    ThresholdWitness,
    BelowThresholdIndependentSet,
    InvalidVertex,
    InvalidEdgeConflict,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27MwisWitnessReplayReport {
    status: G27MwisWitnessReplayStatus,
    dominant_weight: i128,
    total_weight: i128,
    selected_vertex_count: usize,
    first_invalid_pair: Option<(usize, usize)>,
}

impl G27MwisWitnessReplayReport {
    pub fn status(&self) -> G27MwisWitnessReplayStatus {
        self.status
    }

    pub fn summary(&self) -> (i128, i128, usize, Option<(usize, usize)>) {
        (
            self.dominant_weight,
            self.total_weight,
            self.selected_vertex_count,
            self.first_invalid_pair,
        )
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

pub fn export_g27_same_field_dominant_mwis_artifact_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27DominantMwisArtifact, G27GeometricFractionalError> {
    let instance = threshold_mwis_instance(handle)?;
    let components = connected_components(&instance.adjacency, &instance.candidates);
    let small_weight =
        exact_small_component_weight(&instance.adjacency, &instance.weights, &components);
    let dominant = components[0].clone();
    let (incumbent_weight, incumbent_vertices) =
        greedy_independent_witness(&instance.adjacency, &instance.weights, &dominant);
    let dominant_vertices = dominant.iter().map(|vertex| vertex + 1).collect::<Vec<_>>();
    let dominant_weights = dominant
        .iter()
        .map(|vertex| instance.weights[*vertex])
        .collect::<Vec<_>>();
    let dominant_edges = induced_edges(&instance.adjacency, &dominant);
    let compatible_w_vertices = instance
        .candidates
        .iter()
        .map(|vertex| vertex + 1)
        .collect::<Vec<_>>();
    let mut artifact = G27DominantMwisArtifact {
        schema: SCHEMA,
        compatible_w_vertices,
        dominant_vertices,
        dominant_weights,
        dominant_edges,
        exact_small_component_weight: small_weight,
        dominant_required_weight: TARGET_WEIGHT - small_weight,
        incumbent_vertices: incumbent_vertices.iter().map(|vertex| vertex + 1).collect(),
        incumbent_dominant_weight: incumbent_weight,
        stable_digest: String::new(),
    };
    artifact.stable_digest = stable_digest(&artifact);
    Ok(artifact)
}

pub fn replay_g27_same_field_dominant_mwis_witness_checked(
    handle: &HadwigerResearchHandle,
    selected_w_vertices: &[usize],
) -> Result<G27MwisWitnessReplayReport, G27GeometricFractionalError> {
    let instance = threshold_mwis_instance(handle)?;
    let components = connected_components(&instance.adjacency, &instance.candidates);
    let small_weight =
        exact_small_component_weight(&instance.adjacency, &instance.weights, &components);
    let dominant = &components[0];
    let mut selected = selected_w_vertices
        .iter()
        .map(|vertex| vertex.checked_sub(1))
        .collect::<Option<Vec<_>>>()
        .unwrap_or_default();
    selected.sort_unstable();
    selected.dedup();
    if selected.len() != selected_w_vertices.len()
        || selected.iter().any(|vertex| !dominant.contains(vertex))
    {
        return Ok(replay_report(
            G27MwisWitnessReplayStatus::InvalidVertex,
            0,
            small_weight,
            selected.len(),
            None,
        ));
    }
    for (index, left) in selected.iter().enumerate() {
        for right in &selected[index + 1..] {
            if has_bit(&instance.adjacency[*left], *right) {
                return Ok(replay_report(
                    G27MwisWitnessReplayStatus::InvalidEdgeConflict,
                    0,
                    small_weight,
                    selected.len(),
                    Some((left + 1, right + 1)),
                ));
            }
        }
    }
    let dominant_weight = selected
        .iter()
        .map(|vertex| instance.weights[*vertex])
        .sum::<i128>();
    let status = if dominant_weight >= TARGET_WEIGHT - small_weight {
        G27MwisWitnessReplayStatus::ThresholdWitness
    } else {
        G27MwisWitnessReplayStatus::BelowThresholdIndependentSet
    };
    Ok(replay_report(
        status,
        dominant_weight,
        small_weight,
        selected.len(),
        None,
    ))
}

fn replay_report(
    status: G27MwisWitnessReplayStatus,
    dominant_weight: i128,
    small_weight: i128,
    selected_vertex_count: usize,
    first_invalid_pair: Option<(usize, usize)>,
) -> G27MwisWitnessReplayReport {
    G27MwisWitnessReplayReport {
        status,
        dominant_weight,
        total_weight: dominant_weight + small_weight,
        selected_vertex_count,
        first_invalid_pair,
    }
}

fn induced_edges(
    adjacency: &[super::g27_same_field_fixed_dual_pricing_support::BitWords],
    vertices: &[usize],
) -> Vec<(usize, usize)> {
    let mut edges = Vec::new();
    for (left_local, left_global) in vertices.iter().enumerate() {
        for (right_local, right_global) in vertices.iter().enumerate().skip(left_local + 1) {
            if has_bit(&adjacency[*left_global], *right_global) {
                edges.push((left_local, right_local));
            }
        }
    }
    edges
}

fn stable_digest(artifact: &G27DominantMwisArtifact) -> String {
    let mut hasher = Sha256::new();
    hasher.update(artifact.schema.as_bytes());
    hasher.update(b"\n");
    for (vertex, weight) in artifact
        .dominant_vertices
        .iter()
        .zip(artifact.dominant_weights.iter())
    {
        hasher.update(format!("v:{vertex}:{weight}\n").as_bytes());
    }
    for (left, right) in &artifact.dominant_edges {
        hasher.update(format!("e:{left}:{right}\n").as_bytes());
    }
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
