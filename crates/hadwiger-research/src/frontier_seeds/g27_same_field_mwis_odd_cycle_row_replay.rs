use sha2::{Digest, Sha256};

use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_same_field_mwis_branch_certificate_preflight::dominant_and_exact_side_weight;
use super::g27_same_field_mwis_odd_cycle_row_replay_support::{
    collect_row_records, write_record, TARGET_PRUNED_RECORDS,
};
use super::g27_same_field_threshold_mwis_bnb_setup::threshold_mwis_alignment_channel_instance_sets;

const TARGET_WEIGHT: i128 = 512_933;
const G27_ANCHOR_INDEX: usize = 7;
const W_ANCHOR_INDEX: usize = 300;
const ATOM_LIMIT: usize = 5;
const ATOM_MASK: u32 = 101_719_589;
const EXPECTED_EXACT_SIDE_WEIGHT: i128 = 61_655;
const EXPECTED_DOMINANT_THRESHOLD: i128 = 451_278;
const EXPECTED_ROOT_TOTAL: i128 = 543_428;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27MwisOddCycleRowReplayStatus {
    RowReplayStablePreflight,
    FrozenInstanceMismatch,
    RootObjectiveMismatch,
    MissingThresholdPrunes,
    CliqueEnumerationCapHit,
    UnstableRowMetadata,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27MwisOddCycleRowReplayReport {
    root_total_odd_cycle_bound: i128,
    pruned_node_count: usize,
    checked_node_count: usize,
    total_clique_rows: usize,
    total_odd_cycle_rows: usize,
    max_odd_cycle_length: usize,
    row_metadata_bytes: usize,
    row_digest: String,
    status: G27MwisOddCycleRowReplayStatus,
}

impl G27MwisOddCycleRowReplayReport {
    pub fn summary(&self) -> (i128, usize, usize, usize, usize) {
        (
            self.root_total_odd_cycle_bound,
            self.pruned_node_count,
            self.checked_node_count,
            self.total_clique_rows,
            self.total_odd_cycle_rows,
        )
    }

    pub fn metadata_summary(&self) -> (usize, usize, &str) {
        (
            self.max_odd_cycle_length,
            self.row_metadata_bytes,
            &self.row_digest,
        )
    }

    pub fn status(&self) -> G27MwisOddCycleRowReplayStatus {
        self.status
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

pub fn replay_g27_same_field_mwis_odd_cycle_rows_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27MwisOddCycleRowReplayReport, G27GeometricFractionalError> {
    let first = build_row_replay(handle)?;
    if first.status != G27MwisOddCycleRowReplayStatus::RowReplayStablePreflight {
        return Ok(first);
    }
    let second = build_row_replay(handle)?;
    if first.row_digest != second.row_digest || first.summary() != second.summary() {
        return Ok(G27MwisOddCycleRowReplayReport {
            status: G27MwisOddCycleRowReplayStatus::UnstableRowMetadata,
            ..first
        });
    }
    Ok(first)
}

fn build_row_replay(
    handle: &HadwigerResearchHandle,
) -> Result<G27MwisOddCycleRowReplayReport, G27GeometricFractionalError> {
    let mut channel_sets = threshold_mwis_alignment_channel_instance_sets(
        handle,
        &[(G27_ANCHOR_INDEX, W_ANCHOR_INDEX)],
        ATOM_LIMIT,
    )?;
    let channel = channel_sets
        .pop()
        .and_then(|channels| {
            channels
                .into_iter()
                .find(|channel| channel.atom_mask == ATOM_MASK)
        })
        .ok_or(G27GeometricFractionalError::MalformedData {
            source: "odd_cycle_row_replay_channel",
        })?;
    let (dominant, small_weight) = dominant_and_exact_side_weight(&channel.instance);
    let threshold = TARGET_WEIGHT - small_weight;
    if small_weight != EXPECTED_EXACT_SIDE_WEIGHT || threshold != EXPECTED_DOMINANT_THRESHOLD {
        return Ok(empty_report(
            G27MwisOddCycleRowReplayStatus::FrozenInstanceMismatch,
        ));
    }
    let records = collect_row_records(
        &channel.instance.adjacency,
        &channel.instance.weights,
        &dominant,
        threshold,
    )?;
    if records[0].rows.maximal_clique_cap_hit {
        return Ok(empty_report(
            G27MwisOddCycleRowReplayStatus::CliqueEnumerationCapHit,
        ));
    }
    let root_total = small_weight + records[0].upper_bound;
    if root_total != EXPECTED_ROOT_TOTAL {
        return Ok(empty_report(
            G27MwisOddCycleRowReplayStatus::RootObjectiveMismatch,
        ));
    }
    if records.len() < TARGET_PRUNED_RECORDS + 1 {
        return Ok(empty_report(
            G27MwisOddCycleRowReplayStatus::MissingThresholdPrunes,
        ));
    }
    Ok(summarize_records(root_total, records))
}

fn summarize_records(
    root_total: i128,
    records: Vec<super::g27_same_field_mwis_odd_cycle_row_replay_support::NodeRecord>,
) -> G27MwisOddCycleRowReplayReport {
    let mut payload = String::new();
    let mut clique_rows = 0;
    let mut odd_rows = 0;
    let mut max_cycle = 0;
    for record in &records {
        write_record(record, &mut payload);
        clique_rows += record.rows.clique_constraints.len();
        odd_rows += record.rows.odd_cycle_cuts.len();
        max_cycle = max_cycle.max(
            record
                .rows
                .odd_cycle_cuts
                .iter()
                .map(|cut| cut.witness.len())
                .max()
                .unwrap_or(0),
        );
    }
    G27MwisOddCycleRowReplayReport {
        root_total_odd_cycle_bound: root_total,
        pruned_node_count: records.len() - 1,
        checked_node_count: records.len(),
        total_clique_rows: clique_rows,
        total_odd_cycle_rows: odd_rows,
        max_odd_cycle_length: max_cycle,
        row_metadata_bytes: payload.len(),
        row_digest: format!("{:x}", Sha256::digest(payload.as_bytes())),
        status: G27MwisOddCycleRowReplayStatus::RowReplayStablePreflight,
    }
}

fn empty_report(status: G27MwisOddCycleRowReplayStatus) -> G27MwisOddCycleRowReplayReport {
    G27MwisOddCycleRowReplayReport {
        root_total_odd_cycle_bound: 0,
        pruned_node_count: 0,
        checked_node_count: 0,
        total_clique_rows: 0,
        total_odd_cycle_rows: 0,
        max_odd_cycle_length: 0,
        row_metadata_bytes: 0,
        row_digest: String::new(),
        status,
    }
}
