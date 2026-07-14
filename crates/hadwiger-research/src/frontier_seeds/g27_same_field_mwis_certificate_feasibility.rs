use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_same_field_fixed_dual_pricing_support::{
    clique_cover_weight_upper_bound, greedy_independent_witness, has_bit, BitWords,
};
use super::g27_same_field_lp_relaxation::stable_set_lp_relaxation_bound;
use super::g27_same_field_mwis_exact::exact_mwis;
use super::g27_same_field_threshold_mwis_bnb_setup::{
    threshold_mwis_alignment_channel_instance_sets, ThresholdMwisChannelInstance,
    ThresholdMwisInstance,
};

const TARGET_WEIGHT: i128 = 512_933;
const G27_ANCHOR_INDEX: usize = 7;
const W_ANCHOR_INDEX: usize = 300;
const ATOM_LIMIT: usize = 5;
const NEAR_MISS_MASKS: [u32; 2] = [101_719_589, 34_610_725];
const EXACT_SIDE_COMPONENT_LIMIT: usize = 32;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27MwisCertificateFeasibilityStatus {
    RootCertificateProvesBelowThreshold,
    NeedsBranchCertificate,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27MwisCertificateFeasibilityChannel {
    atom_mask: u32,
    dominant_vertex_count: usize,
    dominant_edge_count: usize,
    exact_side_component_weight: i128,
    dominant_threshold: i128,
    greedy_dominant_weight: i128,
    replayed_best_total_weight: i128,
    clique_cover_dominant_upper_bound: i128,
    edge_lp_total_ceiling: i128,
    clique_lp_total_ceiling: i128,
    odd_cycle_lp_total_ceiling: i128,
    odd_cycle_cut_count: usize,
    odd_cycle_round_count: usize,
    status: G27MwisCertificateFeasibilityStatus,
}

impl G27MwisCertificateFeasibilityChannel {
    pub fn atom_mask(&self) -> u32 {
        self.atom_mask
    }

    pub fn summary(&self) -> (usize, usize, i128, i128, i128, i128, i128, i128) {
        (
            self.dominant_vertex_count,
            self.dominant_edge_count,
            self.exact_side_component_weight,
            self.dominant_threshold,
            self.replayed_best_total_weight,
            self.clique_cover_dominant_upper_bound,
            self.clique_lp_total_ceiling,
            self.odd_cycle_lp_total_ceiling,
        )
    }

    pub fn cut_summary(&self) -> (usize, usize) {
        (self.odd_cycle_cut_count, self.odd_cycle_round_count)
    }

    pub fn status(&self) -> G27MwisCertificateFeasibilityStatus {
        self.status
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27MwisCertificateFeasibilityReport {
    g27_anchor: usize,
    w_anchor: usize,
    target_weight: i128,
    channels: Vec<G27MwisCertificateFeasibilityChannel>,
}

impl G27MwisCertificateFeasibilityReport {
    pub fn alignment(&self) -> (usize, usize) {
        (self.g27_anchor, self.w_anchor)
    }

    pub fn target_weight(&self) -> i128 {
        self.target_weight
    }

    pub fn channels(&self) -> &[G27MwisCertificateFeasibilityChannel] {
        &self.channels
    }

    pub fn admits_theorem_authority(&self) -> bool {
        self.channels.iter().all(|channel| {
            channel.status
                == G27MwisCertificateFeasibilityStatus::RootCertificateProvesBelowThreshold
        })
    }
}

pub fn screen_g27_same_field_mwis_certificate_feasibility_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27MwisCertificateFeasibilityReport, G27GeometricFractionalError> {
    let channel_sets = threshold_mwis_alignment_channel_instance_sets(
        handle,
        &[(G27_ANCHOR_INDEX, W_ANCHOR_INDEX)],
        ATOM_LIMIT,
    )?;
    let channels = channel_sets
        .into_iter()
        .next()
        .ok_or(G27GeometricFractionalError::MalformedData {
            source: "near_miss_alignment_channel_set",
        })?
        .into_iter()
        .filter(|channel| NEAR_MISS_MASKS.contains(&channel.atom_mask))
        .map(screen_channel)
        .collect::<Result<Vec<_>, _>>()?;
    if channels.len() != NEAR_MISS_MASKS.len() {
        return Err(G27GeometricFractionalError::MalformedData {
            source: "near_miss_channel_masks",
        });
    }
    Ok(G27MwisCertificateFeasibilityReport {
        g27_anchor: G27_ANCHOR_INDEX + 1,
        w_anchor: W_ANCHOR_INDEX + 1,
        target_weight: TARGET_WEIGHT,
        channels,
    })
}

fn screen_channel(
    channel: ThresholdMwisChannelInstance,
) -> Result<G27MwisCertificateFeasibilityChannel, G27GeometricFractionalError> {
    let (dominant, small_weight) = dominant_and_exact_side_weight(&channel.instance);
    let lp = stable_set_lp_relaxation_bound(
        &channel.instance.adjacency,
        &channel.instance.weights,
        &dominant,
    )?;
    let (greedy_weight, _) = greedy_independent_witness(
        &channel.instance.adjacency,
        &channel.instance.weights,
        &dominant,
    );
    let clique_cover = clique_cover_weight_upper_bound(
        &channel.instance.adjacency,
        &channel.instance.weights,
        &dominant,
    );
    let dominant_threshold = TARGET_WEIGHT - small_weight;
    let odd_total = small_weight + lp.odd_cycle_objective_ceiling;
    let status = if odd_total < TARGET_WEIGHT {
        G27MwisCertificateFeasibilityStatus::RootCertificateProvesBelowThreshold
    } else {
        G27MwisCertificateFeasibilityStatus::NeedsBranchCertificate
    };
    Ok(G27MwisCertificateFeasibilityChannel {
        atom_mask: channel.atom_mask,
        dominant_vertex_count: dominant.len(),
        dominant_edge_count: induced_edge_count(&channel.instance.adjacency, &dominant),
        exact_side_component_weight: small_weight,
        dominant_threshold,
        greedy_dominant_weight: greedy_weight,
        replayed_best_total_weight: small_weight + greedy_weight,
        clique_cover_dominant_upper_bound: clique_cover,
        edge_lp_total_ceiling: small_weight + lp.objective_ceiling,
        clique_lp_total_ceiling: small_weight + lp.clique_objective_ceiling,
        odd_cycle_lp_total_ceiling: odd_total,
        odd_cycle_cut_count: lp.odd_cycle_cut_count,
        odd_cycle_round_count: lp.odd_cycle_round_count,
        status,
    })
}

fn dominant_and_exact_side_weight(instance: &ThresholdMwisInstance) -> (Vec<usize>, i128) {
    let components = connected_components(&instance.adjacency, &instance.candidates);
    let mut dominant = Vec::new();
    let mut small_weight = 0;
    for (index, component) in components.iter().enumerate() {
        if index == 0 || component.len() > EXACT_SIDE_COMPONENT_LIMIT {
            dominant.extend(component);
        } else {
            small_weight += exact_mwis(&instance.adjacency, &instance.weights, component).0;
        }
    }
    dominant.sort_unstable();
    (dominant, small_weight)
}

fn connected_components(adjacency: &[BitWords], candidates: &[usize]) -> Vec<Vec<usize>> {
    let mut remaining = candidates.to_vec();
    remaining.sort_unstable();
    let mut components = Vec::new();
    while let Some(start) = remaining.pop() {
        let mut stack = vec![start];
        let mut component = Vec::new();
        while let Some(vertex) = stack.pop() {
            component.push(vertex);
            let mut index = 0;
            while index < remaining.len() {
                if has_bit(&adjacency[vertex], remaining[index]) {
                    stack.push(remaining.swap_remove(index));
                } else {
                    index += 1;
                }
            }
        }
        component.sort_unstable();
        components.push(component);
    }
    components.sort_by(|left, right| right.len().cmp(&left.len()).then_with(|| left.cmp(right)));
    components
}

fn induced_edge_count(adjacency: &[BitWords], candidates: &[usize]) -> usize {
    let mut count = 0;
    for left in 0..candidates.len() {
        for right in left + 1..candidates.len() {
            count += usize::from(has_bit(&adjacency[candidates[left]], candidates[right]));
        }
    }
    count
}
