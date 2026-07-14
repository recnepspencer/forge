use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_same_field_alignment_mwis_sweep::{
    export_g27_same_field_retained_alignment_mwis_sweep_checked, G27AlignmentMwisSweepChannel,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27AlignmentMwisReplayStatus {
    ThresholdWitness,
    BelowThresholdIndependentSet,
    UnknownChannel,
    InvalidVertex,
    InvalidEdgeConflict,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27AlignmentMwisReplayReport {
    status: G27AlignmentMwisReplayStatus,
    g27_anchor: usize,
    w_anchor: usize,
    atom_mask: u32,
    dominant_weight: i128,
    total_weight: i128,
    selected_vertex_count: usize,
    first_invalid_pair: Option<(usize, usize)>,
}

impl G27AlignmentMwisReplayReport {
    pub fn status(&self) -> G27AlignmentMwisReplayStatus {
        self.status
    }

    pub fn summary(&self) -> (usize, usize, u32, i128, i128, usize, Option<(usize, usize)>) {
        (
            self.g27_anchor,
            self.w_anchor,
            self.atom_mask,
            self.dominant_weight,
            self.total_weight,
            self.selected_vertex_count,
            self.first_invalid_pair,
        )
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

pub fn replay_g27_same_field_retained_alignment_mwis_witnesses_checked(
    handle: &HadwigerResearchHandle,
    witnesses: &[(usize, usize, u32, Vec<usize>)],
) -> Result<Vec<G27AlignmentMwisReplayReport>, G27GeometricFractionalError> {
    let artifact = export_g27_same_field_retained_alignment_mwis_sweep_checked(handle)?;
    Ok(witnesses
        .iter()
        .map(|(g27_anchor, w_anchor, atom_mask, selected)| {
            artifact
                .alignments
                .iter()
                .find(|alignment| {
                    alignment.g27_anchor == *g27_anchor && alignment.w_anchor == *w_anchor
                })
                .and_then(|alignment| {
                    alignment
                        .channels
                        .iter()
                        .find(|channel| channel.atom_mask == *atom_mask)
                })
                .map(|channel| {
                    replay_channel(channel, *g27_anchor, *w_anchor, *atom_mask, selected)
                })
                .unwrap_or_else(|| unknown_channel(*g27_anchor, *w_anchor, *atom_mask, selected))
        })
        .collect())
}

fn replay_channel(
    channel: &G27AlignmentMwisSweepChannel,
    g27_anchor: usize,
    w_anchor: usize,
    atom_mask: u32,
    selected_w_vertices: &[usize],
) -> G27AlignmentMwisReplayReport {
    let mut selected = selected_w_vertices.to_vec();
    selected.sort_unstable();
    selected.dedup();
    if selected.len() != selected_w_vertices.len()
        || selected
            .iter()
            .any(|vertex| !channel.dominant_vertices.contains(vertex))
    {
        return replay_report(
            G27AlignmentMwisReplayStatus::InvalidVertex,
            g27_anchor,
            w_anchor,
            atom_mask,
            0,
            channel.exact_small_component_weight,
            selected.len(),
            None,
        );
    }
    for (index, left) in selected.iter().enumerate() {
        for right in &selected[index + 1..] {
            if channel_edge_conflict(channel, *left, *right) {
                return replay_report(
                    G27AlignmentMwisReplayStatus::InvalidEdgeConflict,
                    g27_anchor,
                    w_anchor,
                    atom_mask,
                    0,
                    channel.exact_small_component_weight,
                    selected.len(),
                    Some((*left, *right)),
                );
            }
        }
    }
    let dominant_weight = selected
        .iter()
        .map(|vertex| {
            let index = channel
                .dominant_vertices
                .iter()
                .position(|candidate| candidate == vertex)
                .unwrap();
            channel.dominant_weights[index]
        })
        .sum::<i128>();
    let status = if dominant_weight >= channel.dominant_required_weight {
        G27AlignmentMwisReplayStatus::ThresholdWitness
    } else {
        G27AlignmentMwisReplayStatus::BelowThresholdIndependentSet
    };
    replay_report(
        status,
        g27_anchor,
        w_anchor,
        atom_mask,
        dominant_weight,
        channel.exact_small_component_weight,
        selected.len(),
        None,
    )
}

fn channel_edge_conflict(
    channel: &G27AlignmentMwisSweepChannel,
    left: usize,
    right: usize,
) -> bool {
    let left_local = channel
        .dominant_vertices
        .iter()
        .position(|vertex| *vertex == left)
        .unwrap();
    let right_local = channel
        .dominant_vertices
        .iter()
        .position(|vertex| *vertex == right)
        .unwrap();
    channel
        .dominant_edges
        .contains(&(left_local.min(right_local), left_local.max(right_local)))
}

fn unknown_channel(
    g27_anchor: usize,
    w_anchor: usize,
    atom_mask: u32,
    selected: &[usize],
) -> G27AlignmentMwisReplayReport {
    replay_report(
        G27AlignmentMwisReplayStatus::UnknownChannel,
        g27_anchor,
        w_anchor,
        atom_mask,
        0,
        0,
        selected.len(),
        None,
    )
}

fn replay_report(
    status: G27AlignmentMwisReplayStatus,
    g27_anchor: usize,
    w_anchor: usize,
    atom_mask: u32,
    dominant_weight: i128,
    small_weight: i128,
    selected_vertex_count: usize,
    first_invalid_pair: Option<(usize, usize)>,
) -> G27AlignmentMwisReplayReport {
    G27AlignmentMwisReplayReport {
        status,
        g27_anchor,
        w_anchor,
        atom_mask,
        dominant_weight,
        total_weight: dominant_weight + small_weight,
        selected_vertex_count,
        first_invalid_pair,
    }
}
