use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_same_field_mwis_sweep::{
    export_g27_same_field_top10_mwis_sweep_checked, G27MwisSweepChannel,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27MwisSweepReplayStatus {
    ThresholdWitness,
    BelowThresholdIndependentSet,
    UnknownAtomMask,
    InvalidVertex,
    InvalidEdgeConflict,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27MwisSweepReplayReport {
    status: G27MwisSweepReplayStatus,
    atom_mask: u32,
    dominant_weight: i128,
    total_weight: i128,
    selected_vertex_count: usize,
    first_invalid_pair: Option<(usize, usize)>,
}

impl G27MwisSweepReplayReport {
    pub fn status(&self) -> G27MwisSweepReplayStatus {
        self.status
    }

    pub fn summary(&self) -> (u32, i128, i128, usize, Option<(usize, usize)>) {
        (
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

pub fn replay_g27_same_field_top10_mwis_witness_checked(
    handle: &HadwigerResearchHandle,
    atom_mask: u32,
    selected_w_vertices: &[usize],
) -> Result<G27MwisSweepReplayReport, G27GeometricFractionalError> {
    let artifact = export_g27_same_field_top10_mwis_sweep_checked(handle)?;
    let Some(channel) = artifact
        .channels()
        .iter()
        .find(|channel| channel.atom_mask == atom_mask)
    else {
        return Ok(replay_report(
            G27MwisSweepReplayStatus::UnknownAtomMask,
            atom_mask,
            0,
            0,
            selected_w_vertices.len(),
            None,
        ));
    };
    Ok(replay_channel(channel, atom_mask, selected_w_vertices))
}

pub fn replay_g27_same_field_top10_mwis_witnesses_checked(
    handle: &HadwigerResearchHandle,
    witnesses: &[(u32, Vec<usize>)],
) -> Result<Vec<G27MwisSweepReplayReport>, G27GeometricFractionalError> {
    let artifact = export_g27_same_field_top10_mwis_sweep_checked(handle)?;
    Ok(witnesses
        .iter()
        .map(|(atom_mask, selected)| {
            artifact
                .channels()
                .iter()
                .find(|channel| channel.atom_mask == *atom_mask)
                .map(|channel| replay_channel(channel, *atom_mask, selected))
                .unwrap_or_else(|| unknown_atom(*atom_mask, selected.len()))
        })
        .collect())
}

fn replay_channel(
    channel: &G27MwisSweepChannel,
    atom_mask: u32,
    selected_w_vertices: &[usize],
) -> G27MwisSweepReplayReport {
    let mut selected = selected_w_vertices.to_vec();
    selected.sort_unstable();
    selected.dedup();
    if selected.len() != selected_w_vertices.len()
        || selected
            .iter()
            .any(|vertex| !channel.dominant_vertices.contains(vertex))
    {
        return replay_report(
            G27MwisSweepReplayStatus::InvalidVertex,
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
                    G27MwisSweepReplayStatus::InvalidEdgeConflict,
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
        G27MwisSweepReplayStatus::ThresholdWitness
    } else {
        G27MwisSweepReplayStatus::BelowThresholdIndependentSet
    };
    replay_report(
        status,
        atom_mask,
        dominant_weight,
        channel.exact_small_component_weight,
        selected.len(),
        None,
    )
}

fn channel_edge_conflict(channel: &G27MwisSweepChannel, left: usize, right: usize) -> bool {
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

fn unknown_atom(atom_mask: u32, selected_vertex_count: usize) -> G27MwisSweepReplayReport {
    replay_report(
        G27MwisSweepReplayStatus::UnknownAtomMask,
        atom_mask,
        0,
        0,
        selected_vertex_count,
        None,
    )
}

fn replay_report(
    status: G27MwisSweepReplayStatus,
    atom_mask: u32,
    dominant_weight: i128,
    small_weight: i128,
    selected_vertex_count: usize,
    first_invalid_pair: Option<(usize, usize)>,
) -> G27MwisSweepReplayReport {
    G27MwisSweepReplayReport {
        status,
        atom_mask,
        dominant_weight,
        total_weight: dominant_weight + small_weight,
        selected_vertex_count,
        first_invalid_pair,
    }
}
