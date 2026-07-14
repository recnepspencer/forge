use sha2::{Digest, Sha256};

use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_same_field_alignment_mwis_candidates::{
    retained_alternate_alignments, AlignmentCandidate,
};
use super::g27_same_field_fixed_dual_pricing_support::{greedy_independent_witness, has_bit};
use super::g27_same_field_mwis_exact::exact_mwis;
use super::g27_same_field_structure_preflight_support::connected_components;
use super::g27_same_field_threshold_mwis_bnb_setup::{
    threshold_mwis_alignment_channel_instance_sets, ThresholdMwisInstance,
};

const TARGET_WEIGHT: i128 = 512_933;
const ATOM_LIMIT: usize = 5;
const EXACT_SIDE_COMPONENT_LIMIT: usize = 32;
const SCHEMA: &str = "forge.hadwiger.g27_same_field_retained_alignment_mwis_sweep.v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27AlignmentMwisSweepArtifact {
    pub(super) alignments: Vec<G27AlignmentMwisSweepAlignment>,
}

impl G27AlignmentMwisSweepArtifact {
    pub fn alignments(&self) -> &[G27AlignmentMwisSweepAlignment] {
        &self.alignments
    }

    pub fn summary(&self) -> (usize, usize, usize) {
        (
            self.alignments.len(),
            self.alignments
                .iter()
                .map(|alignment| alignment.channels.len())
                .sum(),
            ATOM_LIMIT,
        )
    }

    pub fn line_export(&self) -> String {
        let mut lines = vec![format!("schema {SCHEMA}")];
        lines.push("candidate_source retained_pressure_and_slack_halo_union".to_string());
        lines.push(format!("global_target {TARGET_WEIGHT}"));
        for alignment in &self.alignments {
            alignment.append_export_lines(&mut lines);
        }
        lines.join("\n")
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27AlignmentMwisSweepAlignment {
    pub(super) g27_anchor: usize,
    pub(super) w_anchor: usize,
    source_rank: usize,
    source_label: String,
    pub(super) channels: Vec<G27AlignmentMwisSweepChannel>,
}

impl G27AlignmentMwisSweepAlignment {
    pub fn identity(&self) -> (usize, usize, &str, usize) {
        (
            self.g27_anchor,
            self.w_anchor,
            &self.source_label,
            self.source_rank,
        )
    }

    pub fn channels(&self) -> &[G27AlignmentMwisSweepChannel] {
        &self.channels
    }

    fn append_export_lines(&self, lines: &mut Vec<String>) {
        lines.push(format!(
            "alignment g27 {} w {} source {} source_rank {}",
            self.g27_anchor, self.w_anchor, self.source_label, self.source_rank
        ));
        for channel in &self.channels {
            channel.append_export_lines(lines, self.g27_anchor, self.w_anchor);
        }
        lines.push("end_alignment".to_string());
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27AlignmentMwisSweepChannel {
    pub(super) atom_rank: usize,
    pub(super) atom_mask: u32,
    atom_vertices: Vec<usize>,
    contact_incidence_weight: i128,
    compatible_w_vertex_count: usize,
    pub(super) dominant_vertices: Vec<usize>,
    pub(super) dominant_weights: Vec<i128>,
    pub(super) dominant_edges: Vec<(usize, usize)>,
    pub(super) exact_small_component_weight: i128,
    pub(super) dominant_required_weight: i128,
    incumbent_total_weight: i128,
    stable_digest: String,
}

impl G27AlignmentMwisSweepChannel {
    pub fn summary(&self) -> (usize, u32, usize, usize, usize, i128, i128, i128) {
        (
            self.atom_rank,
            self.atom_mask,
            self.compatible_w_vertex_count,
            self.dominant_vertices.len(),
            self.dominant_edges.len(),
            self.exact_small_component_weight,
            self.dominant_required_weight,
            self.incumbent_total_weight,
        )
    }

    fn append_export_lines(&self, lines: &mut Vec<String>, g27_anchor: usize, w_anchor: usize) {
        lines.push(format!(
            "channel atom_rank {} atom_mask {} contact {} small {} target {} digest {}",
            self.atom_rank,
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
        lines.push(format!("alignment_key {} {}", g27_anchor, w_anchor));
        lines.push("end_channel".to_string());
    }
}

pub fn export_g27_same_field_retained_alignment_mwis_sweep_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27AlignmentMwisSweepArtifact, G27GeometricFractionalError> {
    let candidates = retained_alternate_alignments()?;
    let alignment_keys = candidates
        .iter()
        .map(|candidate| (candidate.g27_anchor - 1, candidate.w_anchor - 1))
        .collect::<Vec<_>>();
    let channel_sets =
        threshold_mwis_alignment_channel_instance_sets(handle, &alignment_keys, ATOM_LIMIT)?;
    let alignments = candidates
        .into_iter()
        .zip(channel_sets)
        .map(|(candidate, channels)| build_alignment(candidate, channels))
        .collect::<Vec<_>>();
    Ok(G27AlignmentMwisSweepArtifact { alignments })
}

fn build_alignment(
    candidate: AlignmentCandidate,
    channels: Vec<super::g27_same_field_threshold_mwis_bnb_setup::ThresholdMwisChannelInstance>,
) -> G27AlignmentMwisSweepAlignment {
    let channels = channels
        .into_iter()
        .map(|channel| {
            build_channel(
                candidate.g27_anchor,
                candidate.w_anchor,
                channel.rank,
                channel.atom_mask,
                channel.atom_vertices,
                channel.contact_incidence_weight,
                channel.instance,
            )
        })
        .collect::<Vec<_>>();
    G27AlignmentMwisSweepAlignment {
        g27_anchor: candidate.g27_anchor,
        w_anchor: candidate.w_anchor,
        source_rank: candidate.source_rank,
        source_label: candidate.source_label,
        channels,
    }
}

fn build_channel(
    g27_anchor: usize,
    w_anchor: usize,
    atom_rank: usize,
    atom_mask: u32,
    atom_vertices: Vec<usize>,
    contact_incidence_weight: i128,
    instance: ThresholdMwisInstance,
) -> G27AlignmentMwisSweepChannel {
    let components = connected_components(&instance.adjacency, &instance.candidates);
    let (dominant, small_weight) = split_dominant_and_exact_side_components(
        &instance.adjacency,
        &instance.weights,
        &components,
    );
    let (incumbent_weight, _) =
        greedy_independent_witness(&instance.adjacency, &instance.weights, &dominant);
    let dominant_vertices = dominant.iter().map(|vertex| vertex + 1).collect::<Vec<_>>();
    let dominant_weights = dominant
        .iter()
        .map(|vertex| instance.weights[*vertex])
        .collect::<Vec<_>>();
    let dominant_edges = induced_edges(&instance.adjacency, &dominant);
    let mut channel = G27AlignmentMwisSweepChannel {
        atom_rank,
        atom_mask,
        atom_vertices,
        contact_incidence_weight,
        compatible_w_vertex_count: instance.candidates.len(),
        dominant_vertices,
        dominant_weights,
        dominant_edges,
        exact_small_component_weight: small_weight,
        dominant_required_weight: TARGET_WEIGHT - small_weight,
        incumbent_total_weight: incumbent_weight + small_weight,
        stable_digest: String::new(),
    };
    channel.stable_digest = stable_digest(&channel, g27_anchor, w_anchor);
    channel
}

fn split_dominant_and_exact_side_components(
    adjacency: &[super::g27_same_field_fixed_dual_pricing_support::BitWords],
    weights: &[i128],
    components: &[Vec<usize>],
) -> (Vec<usize>, i128) {
    let mut dominant = Vec::new();
    let mut small_weight = 0;
    for (index, component) in components.iter().enumerate() {
        if index == 0 || component.len() > EXACT_SIDE_COMPONENT_LIMIT {
            dominant.extend(component);
        } else {
            small_weight += exact_mwis(adjacency, weights, component).0;
        }
    }
    dominant.sort_unstable();
    (dominant, small_weight)
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

fn stable_digest(
    channel: &G27AlignmentMwisSweepChannel,
    g27_anchor: usize,
    w_anchor: usize,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(
        format!(
            "{SCHEMA}:g{g27_anchor}:w{w_anchor}:rank{}:mask{}\n",
            channel.atom_rank, channel.atom_mask
        )
        .as_bytes(),
    );
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
