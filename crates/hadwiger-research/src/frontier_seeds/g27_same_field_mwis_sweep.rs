use sha2::{Digest, Sha256};

use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_same_field_fixed_dual_pricing_support::{greedy_independent_witness, has_bit};
use super::g27_same_field_structure_preflight_support::{
    connected_components, exact_small_component_weight,
};
use super::g27_same_field_threshold_mwis_bnb_setup::{
    threshold_mwis_channel_instances, ThresholdMwisInstance,
};

const TARGET_WEIGHT: i128 = 512_933;
const SCHEMA: &str = "forge.hadwiger.g27_same_field_top10_mwis_sweep.v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27MwisSweepArtifact {
    channels: Vec<G27MwisSweepChannel>,
}

impl G27MwisSweepArtifact {
    pub fn channels(&self) -> &[G27MwisSweepChannel] {
        &self.channels
    }

    pub fn summary(&self) -> (usize, usize, usize) {
        let duplicate_count = self.channels.len()
            - self
                .channels
                .iter()
                .map(|channel| channel.stable_digest.as_str())
                .collect::<std::collections::BTreeSet<_>>()
                .len();
        (
            self.channels.len(),
            self.best_incumbent_total(),
            duplicate_count,
        )
    }

    pub fn best_incumbent_total(&self) -> usize {
        self.channels
            .iter()
            .map(|channel| channel.incumbent_total_weight as usize)
            .max()
            .unwrap_or(0)
    }

    pub fn line_export(&self) -> String {
        let mut lines = vec![format!("schema {SCHEMA}")];
        lines.push("provenance g27_anchor=23 w_anchor=254 retained_top10_tight_atoms".to_string());
        lines.push(format!("global_target {TARGET_WEIGHT}"));
        for channel in &self.channels {
            channel.append_export_lines(&mut lines);
        }
        lines.join("\n")
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27MwisSweepChannel {
    pub(super) rank: usize,
    pub(super) atom_mask: u32,
    atom_vertices: Vec<usize>,
    contact_incidence_weight: i128,
    compatible_w_vertex_count: usize,
    pub(super) dominant_vertices: Vec<usize>,
    pub(super) dominant_weights: Vec<i128>,
    pub(super) dominant_edges: Vec<(usize, usize)>,
    pub(super) exact_small_component_weight: i128,
    pub(super) dominant_required_weight: i128,
    incumbent_vertices: Vec<usize>,
    incumbent_dominant_weight: i128,
    incumbent_total_weight: i128,
    stable_digest: String,
}

impl G27MwisSweepChannel {
    pub fn channel_summary(&self) -> (usize, u32, usize, usize, usize, i128, i128, i128) {
        (
            self.rank,
            self.atom_mask,
            self.compatible_w_vertex_count,
            self.dominant_vertices.len(),
            self.dominant_edges.len(),
            self.exact_small_component_weight,
            self.dominant_required_weight,
            self.incumbent_total_weight,
        )
    }

    pub fn stable_digest(&self) -> &str {
        &self.stable_digest
    }

    fn append_export_lines(&self, lines: &mut Vec<String>) {
        lines.push(format!(
            "channel {} atom_mask {} contact {} small {} target {} digest {}",
            self.rank,
            self.atom_mask,
            self.contact_incidence_weight,
            self.exact_small_component_weight,
            self.dominant_required_weight,
            self.stable_digest
        ));
        lines.push(format!(
            "atom_vertices {}",
            join_numbers(&self.atom_vertices)
        ));
        lines.push(format!(
            "compatible_count {}",
            self.compatible_w_vertex_count
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
        lines.push(format!(
            "incumbent {}",
            join_numbers(&self.incumbent_vertices)
        ));
        lines.push("end_channel".to_string());
    }
}

pub fn export_g27_same_field_top10_mwis_sweep_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27MwisSweepArtifact, G27GeometricFractionalError> {
    let channels = threshold_mwis_channel_instances(handle)?
        .into_iter()
        .map(|channel| {
            build_channel(
                channel.rank,
                channel.atom_mask,
                channel.atom_vertices,
                channel.contact_incidence_weight,
                channel.instance,
            )
        })
        .collect::<Vec<_>>();
    Ok(G27MwisSweepArtifact { channels })
}

fn build_channel(
    rank: usize,
    atom_mask: u32,
    atom_vertices: Vec<usize>,
    contact_incidence_weight: i128,
    instance: ThresholdMwisInstance,
) -> G27MwisSweepChannel {
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
    let mut channel = G27MwisSweepChannel {
        rank,
        atom_mask,
        atom_vertices,
        contact_incidence_weight,
        compatible_w_vertex_count: instance.candidates.len(),
        dominant_vertices,
        dominant_weights,
        dominant_edges,
        exact_small_component_weight: small_weight,
        dominant_required_weight: TARGET_WEIGHT - small_weight,
        incumbent_vertices: incumbent_vertices.iter().map(|vertex| vertex + 1).collect(),
        incumbent_dominant_weight: incumbent_weight,
        incumbent_total_weight: incumbent_weight + small_weight,
        stable_digest: String::new(),
    };
    channel.stable_digest = stable_digest(&channel);
    channel
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

fn stable_digest(channel: &G27MwisSweepChannel) -> String {
    let mut hasher = Sha256::new();
    hasher.update(format!("{SCHEMA}:{}:{}\n", channel.rank, channel.atom_mask).as_bytes());
    for (vertex, weight) in channel
        .dominant_vertices
        .iter()
        .zip(channel.dominant_weights.iter())
    {
        hasher.update(format!("v:{vertex}:{weight}\n").as_bytes());
    }
    for (left, right) in &channel.dominant_edges {
        hasher.update(format!("e:{left}:{right}\n").as_bytes());
    }
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn join_numbers(numbers: &[usize]) -> String {
    numbers
        .iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join(",")
}
