use num_traits::Zero;

use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_geometric_fractional_data::retained_g27_coefficients;
use super::g27_geometric_fractional_dual_replay::retained_g27_atom_slacks;
use super::g27_same_field_fixed_dual_pricing_support::{empty_words, set_bit, BitWords};
use super::g27_same_field_pressure_interface_support::{approx_unit_distance, g27_points};
use super::g27_w_circles_exact_geometry_audit::audit_g27_w_circles_607_exact_geometry_checked;
use super::g27_w_circles_exact_geometry_support::{
    parse_w_integer_weights, parse_w_retained_edges, parse_w_vertices, squared_distance,
    EXPECTED_VERTEX_COUNT,
};

const G27_ANCHOR_INDEX: usize = 22;
const W_ANCHOR_INDEX: usize = 253;
const PRICED_ATOM_LIMIT: usize = 10;
const RETAINED_TOP_COMPATIBLE_COUNT: usize = 502;

pub(super) struct ThresholdMwisInstance {
    pub(super) adjacency: Vec<BitWords>,
    pub(super) weights: Vec<i128>,
    pub(super) candidates: Vec<usize>,
}

pub(super) struct ThresholdMwisChannelInstance {
    pub(super) rank: usize,
    pub(super) atom_mask: u32,
    pub(super) atom_vertices: Vec<usize>,
    pub(super) contact_incidence_weight: i128,
    pub(super) instance: ThresholdMwisInstance,
}

pub(super) fn threshold_mwis_instance(
    handle: &HadwigerResearchHandle,
) -> Result<ThresholdMwisInstance, G27GeometricFractionalError> {
    let _exact_geometry = audit_g27_w_circles_607_exact_geometry_checked(handle)?;
    let g_points = g27_points(&retained_g27_coefficients()?)?;
    let w_points = parse_w_vertices()?;
    let weights = parse_w_integer_weights()?;
    let adjacency = w_adjacency()?;
    let cross_conflicts =
        cross_conflict_masks(&g_points, &w_points, G27_ANCHOR_INDEX, W_ANCHOR_INDEX);
    let atom_mask = retained_top_priced_atom(&cross_conflicts, &weights)?;
    Ok(ThresholdMwisInstance {
        adjacency,
        weights,
        candidates: compatible_w_candidates(atom_mask, &cross_conflicts, W_ANCHOR_INDEX),
    })
}

pub(super) fn threshold_mwis_channel_instances(
    handle: &HadwigerResearchHandle,
) -> Result<Vec<ThresholdMwisChannelInstance>, G27GeometricFractionalError> {
    let _exact_geometry = audit_g27_w_circles_607_exact_geometry_checked(handle)?;
    let g_points = g27_points(&retained_g27_coefficients()?)?;
    let w_points = parse_w_vertices()?;
    let weights = parse_w_integer_weights()?;
    let adjacency = w_adjacency()?;
    let cross_conflicts =
        cross_conflict_masks(&g_points, &w_points, G27_ANCHOR_INDEX, W_ANCHOR_INDEX);
    channel_instances(
        adjacency,
        weights,
        cross_conflicts,
        W_ANCHOR_INDEX,
        PRICED_ATOM_LIMIT,
    )
}

pub(super) fn threshold_mwis_alignment_channel_instance_sets(
    handle: &HadwigerResearchHandle,
    alignments: &[(usize, usize)],
    atom_limit: usize,
) -> Result<Vec<Vec<ThresholdMwisChannelInstance>>, G27GeometricFractionalError> {
    let _exact_geometry = audit_g27_w_circles_607_exact_geometry_checked(handle)?;
    let g_points = g27_points(&retained_g27_coefficients()?)?;
    let w_points = parse_w_vertices()?;
    let weights = parse_w_integer_weights()?;
    let adjacency = w_adjacency()?;
    alignments
        .iter()
        .map(|(g27_anchor_index, w_anchor_index)| {
            let cross_conflicts =
                cross_conflict_masks(&g_points, &w_points, *g27_anchor_index, *w_anchor_index);
            channel_instances(
                adjacency.clone(),
                weights.clone(),
                cross_conflicts,
                *w_anchor_index,
                atom_limit,
            )
        })
        .collect()
}

fn channel_instances(
    adjacency: Vec<BitWords>,
    weights: Vec<i128>,
    cross_conflicts: Vec<u32>,
    w_anchor_index: usize,
    atom_limit: usize,
) -> Result<Vec<ThresholdMwisChannelInstance>, G27GeometricFractionalError> {
    ranked_tight_atoms(&cross_conflicts, &weights).map(|atoms| {
        atoms
            .into_iter()
            .take(atom_limit)
            .enumerate()
            .map(|(rank, atom_mask)| ThresholdMwisChannelInstance {
                rank: rank + 1,
                atom_mask,
                atom_vertices: atom_vertices(atom_mask),
                contact_incidence_weight: contact_incidence_weight(
                    atom_mask,
                    &cross_conflicts,
                    &weights,
                ),
                instance: ThresholdMwisInstance {
                    adjacency: adjacency.clone(),
                    weights: weights.clone(),
                    candidates: compatible_w_candidates(
                        atom_mask,
                        &cross_conflicts,
                        w_anchor_index,
                    ),
                },
            })
            .collect()
    })
}

fn w_adjacency() -> Result<Vec<BitWords>, G27GeometricFractionalError> {
    let mut adjacency = vec![empty_words(); EXPECTED_VERTEX_COUNT];
    for (left, right) in parse_w_retained_edges(EXPECTED_VERTEX_COUNT)? {
        set_bit(&mut adjacency[left - 1], right - 1);
        set_bit(&mut adjacency[right - 1], left - 1);
    }
    Ok(adjacency)
}

fn retained_top_priced_atom(
    cross_conflicts: &[u32],
    weights: &[i128],
) -> Result<u32, G27GeometricFractionalError> {
    ranked_tight_atoms(cross_conflicts, weights)?
        .into_iter()
        .take(PRICED_ATOM_LIMIT)
        .find(|atom| {
            let candidates = compatible_w_candidates(*atom, cross_conflicts, W_ANCHOR_INDEX);
            candidates.len() == RETAINED_TOP_COMPATIBLE_COUNT
        })
        .ok_or(G27GeometricFractionalError::MalformedData {
            source: "retained_top_compatible_channel",
        })
}

fn ranked_tight_atoms(
    cross_conflicts: &[u32],
    weights: &[i128],
) -> Result<Vec<u32>, G27GeometricFractionalError> {
    let (_, atom_slacks) = retained_g27_atom_slacks()?;
    let mut tight_atoms = atom_slacks
        .into_iter()
        .filter(|(_, slack)| slack.is_zero())
        .map(|(atom, _)| atom)
        .collect::<Vec<_>>();
    tight_atoms.sort_by(|left, right| {
        contact_incidence_weight(*right, cross_conflicts, weights)
            .cmp(&contact_incidence_weight(*left, cross_conflicts, weights))
            .then_with(|| left.cmp(right))
    });
    Ok(tight_atoms)
}

fn atom_vertices(atom_mask: u32) -> Vec<usize> {
    (0..32)
        .filter_map(|index| (atom_mask & (1u32 << index) != 0).then_some(index + 1))
        .collect()
}

fn contact_incidence_weight(atom_mask: u32, cross_conflicts: &[u32], weights: &[i128]) -> i128 {
    cross_conflicts
        .iter()
        .enumerate()
        .filter(|(_, conflict_mask)| *conflict_mask & atom_mask != 0)
        .map(|(w_index, _)| weights[w_index])
        .sum()
}

fn compatible_w_candidates(
    atom_mask: u32,
    cross_conflicts: &[u32],
    w_anchor_index: usize,
) -> Vec<usize> {
    cross_conflicts
        .iter()
        .enumerate()
        .filter_map(|(w_index, conflict_mask)| {
            (w_index != w_anchor_index && conflict_mask & atom_mask == 0).then_some(w_index)
        })
        .collect()
}

fn cross_conflict_masks(
    g_points: &[super::g27_w_circles_exact_geometry_support::WExactPoint],
    w_points: &[super::g27_w_circles_exact_geometry_support::WExactPoint],
    g27_anchor_index: usize,
    w_anchor_index: usize,
) -> Vec<u32> {
    let translation = g_points[g27_anchor_index].sub(w_points[w_anchor_index]);
    w_points
        .iter()
        .enumerate()
        .map(|(w_index, w_point)| {
            let translated_w = w_point.add(translation);
            let mut mask = 0u32;
            for (g_index, g_point) in g_points.iter().enumerate() {
                if g_index == g27_anchor_index && w_index == w_anchor_index {
                    continue;
                }
                if approx_unit_distance(*g_point, translated_w)
                    && squared_distance(*g_point, translated_w).is_one()
                {
                    mask |= 1u32 << g_index;
                }
            }
            mask
        })
        .collect()
}
