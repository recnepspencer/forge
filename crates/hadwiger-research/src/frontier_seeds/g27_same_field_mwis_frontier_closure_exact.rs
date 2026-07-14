use crate::domain_artifacts::core_artifact::HadwigerArtifactReference;
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_same_field_fixed_dual_pricing_support::BitWords;
use super::g27_same_field_lp_relaxation::{
    stable_set_lp_guidance_values, stable_set_lp_relaxation_rows,
};
use super::g27_same_field_mwis_branch_certificate_preflight::dominant_and_exact_side_weight;
use super::g27_same_field_mwis_frontier_closure_campaign::scout_g27_same_field_mwis_frontier_closure_campaign_checked;
use super::g27_same_field_mwis_frontier_closure_campaign_support::G27MwisFrontierClosureCampaignScoutReport;
use super::g27_same_field_mwis_frontier_closure_exact_gates::{
    frontier_matches_scout, parent_identity_matches, scout_row_matches, scout_source_is_ready,
    valid_scope,
};
use super::g27_same_field_mwis_frontier_closure_exact_payload::{
    certificate_digest, digest_text, max_denominator, node_from_work, report, LeafReplay,
    ParentWork,
};
use super::g27_same_field_mwis_frontier_closure_exact_support::{
    G27MwisFrontierClosureExactLeaf, G27MwisFrontierClosureExactLeafStatus,
    G27MwisFrontierClosureExactNode, G27MwisFrontierClosureExactReplayReport,
    G27MwisFrontierClosureExactStatus,
};
use super::g27_same_field_mwis_lp_guided_branch_support::{
    child_entries, initial_frontier, lp_guided_branch, node_digest, QueueEntry,
};
use super::g27_same_field_mwis_lp_guided_micro_dual::solve_one_sided_dual;
use super::g27_same_field_mwis_lp_guided_micro_dual_support::{
    explicit_rows, validate_rows, write_record,
};
use super::g27_same_field_mwis_odd_cycle_dual_replay_support::replay_certificate_for_candidates;
use super::g27_same_field_threshold_mwis_bnb_setup::threshold_mwis_alignment_channel_instance_sets;

const TARGET_WEIGHT: i128 = 512_933;
const G27_ANCHOR_INDEX: usize = 7;
const W_ANCHOR_INDEX: usize = 300;
const ATOM_LIMIT: usize = 5;
const ATOM_MASK: u32 = 101_719_589;
const EXPECTED_EXACT_SIDE_WEIGHT: i128 = 61_655;
const EXPECTED_DOMINANT_THRESHOLD: i128 = 451_278;
const EXPECTED_PREFIX_EXPANDED: usize = 29;
const EXPECTED_PREFIX_PRUNED: usize = 2;
const EXPECTED_PREFIX_OPEN: usize = 28;
const EXPECTED_H44_SCOUT_DIGEST: &str =
    "6bd5dc2d60e45cb2f05bf0dd4beb7fcabf2ffae6684ddf8f383a2563601a6865";

pub fn replay_g27_same_field_mwis_frontier_closure_exact_chunk_checked(
    handle: &HadwigerResearchHandle,
    selected_start: usize,
    selected_end: usize,
) -> Result<G27MwisFrontierClosureExactReplayReport, G27GeometricFractionalError> {
    let scout = scout_g27_same_field_mwis_frontier_closure_campaign_checked(handle)?;
    let scout_digest = scout.artifact_digest().stable_token().to_string();
    let source = scout.reference();
    if !valid_scope(selected_start, selected_end) {
        return report(
            source,
            scout_digest,
            selected_start,
            selected_end,
            G27MwisFrontierClosureExactStatus::InvalidChunkScope,
            Vec::new(),
        );
    }
    if scout_digest != EXPECTED_H44_SCOUT_DIGEST {
        return report(
            source,
            scout_digest,
            selected_start,
            selected_end,
            G27MwisFrontierClosureExactStatus::ScoutDigestMismatch,
            Vec::new(),
        );
    }
    if !scout_source_is_ready(&scout) {
        return report(
            source,
            scout_digest,
            selected_start,
            selected_end,
            G27MwisFrontierClosureExactStatus::ScoutSourceMismatch,
            Vec::new(),
        );
    }
    run_exact_chunk(
        handle,
        &scout,
        source,
        scout_digest,
        selected_start,
        selected_end,
    )
}

fn run_exact_chunk(
    handle: &HadwigerResearchHandle,
    scout: &G27MwisFrontierClosureCampaignScoutReport,
    source: HadwigerArtifactReference,
    scout_digest: String,
    selected_start: usize,
    selected_end: usize,
) -> Result<G27MwisFrontierClosureExactReplayReport, G27GeometricFractionalError> {
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
            source: "frontier_closure_exact_channel",
        })?;
    let (dominant, exact_side_weight) = dominant_and_exact_side_weight(&channel.instance);
    if exact_side_weight != EXPECTED_EXACT_SIDE_WEIGHT {
        return report(
            source,
            scout_digest,
            selected_start,
            selected_end,
            G27MwisFrontierClosureExactStatus::FrozenInstanceMismatch,
            Vec::new(),
        );
    }
    let frontier = initial_frontier(
        &channel.instance.adjacency,
        &channel.instance.weights,
        &dominant,
        EXPECTED_DOMINANT_THRESHOLD,
        EXPECTED_PREFIX_EXPANDED,
        EXPECTED_PREFIX_PRUNED,
        EXPECTED_PREFIX_OPEN,
    )?;
    if !frontier_matches_scout(&frontier, exact_side_weight, scout) {
        return report(
            source,
            scout_digest,
            selected_start,
            selected_end,
            G27MwisFrontierClosureExactStatus::FrontierShapeMismatch,
            Vec::new(),
        );
    }
    let mut nodes = Vec::new();
    for index in selected_start..selected_end {
        trace(&format!("exact_node_{index}"));
        let node = certify_parent(
            &channel.instance.adjacency,
            &channel.instance.weights,
            exact_side_weight,
            &frontier,
            scout,
            index,
        )?;
        let status = node.status();
        nodes.push(node);
        if status != G27MwisFrontierClosureExactStatus::ExactChunkCertified {
            return report(
                source,
                scout_digest,
                selected_start,
                selected_end,
                status,
                nodes,
            );
        }
    }
    report(
        source,
        scout_digest,
        selected_start,
        selected_end,
        G27MwisFrontierClosureExactStatus::ExactChunkCertified,
        nodes,
    )
}

fn certify_parent(
    adjacency: &[BitWords],
    weights: &[i128],
    exact_side_weight: i128,
    frontier: &[QueueEntry],
    scout: &G27MwisFrontierClosureCampaignScoutReport,
    index: usize,
) -> Result<G27MwisFrontierClosureExactNode, G27GeometricFractionalError> {
    let parent = frontier
        .get(index)
        .ok_or(G27GeometricFractionalError::MalformedData {
            source: "frontier_closure_exact_parent",
        })?;
    let work = parent_work(adjacency, weights, exact_side_weight, index, parent)?;
    if !parent_identity_matches(&work) {
        return Ok(node_from_work(
            work,
            Vec::new(),
            String::new(),
            G27MwisFrontierClosureExactStatus::ParentIdentityMismatch,
        ));
    }
    if !scout_row_matches(scout.scout_rows(), &work) {
        return Ok(node_from_work(
            work,
            Vec::new(),
            String::new(),
            G27MwisFrontierClosureExactStatus::ScoutRowMismatch,
        ));
    }
    if work.terminal_totals.len() != 3 {
        return Ok(node_from_work(
            work,
            Vec::new(),
            String::new(),
            G27MwisFrontierClosureExactStatus::IncompleteLeafPartition,
        ));
    }
    if work
        .terminal_totals
        .iter()
        .any(|total| *total > TARGET_WEIGHT)
    {
        return Ok(node_from_work(
            work,
            Vec::new(),
            String::new(),
            G27MwisFrontierClosureExactStatus::FloatingBoundAboveTarget,
        ));
    }
    let mut leaves = Vec::new();
    let mut node_payload = String::new();
    for (leaf_index, child) in work.leaves.iter().enumerate() {
        trace(&format!("exact_node_{}_leaf_{leaf_index}", work.index));
        let replay = certify_leaf(adjacency, weights, exact_side_weight, leaf_index, child)?;
        node_payload.push_str(&replay.row_payload);
        let failed =
            replay.leaf.status() != G27MwisFrontierClosureExactLeafStatus::ExactLeafCertified;
        leaves.push(replay.leaf);
        if failed {
            return Ok(node_from_work(
                work,
                leaves,
                digest_text(&node_payload),
                G27MwisFrontierClosureExactStatus::ExactReplayFailed,
            ));
        }
    }
    Ok(node_from_work(
        work,
        leaves,
        digest_text(&node_payload),
        G27MwisFrontierClosureExactStatus::ExactChunkCertified,
    ))
}

fn parent_work(
    adjacency: &[BitWords],
    weights: &[i128],
    exact_side_weight: i128,
    index: usize,
    parent: &QueueEntry,
) -> Result<ParentWork, G27GeometricFractionalError> {
    let first_guidance =
        stable_set_lp_guidance_values(adjacency, weights, &parent.node.candidates)?;
    let first_branch =
        lp_guided_branch(adjacency, weights, &parent.node.candidates, &first_guidance);
    let first_children = child_entries(adjacency, weights, parent, first_branch)?;
    let first_child_totals = [
        exact_side_weight + first_children[0].upper_bound,
        exact_side_weight + first_children[1].upper_bound,
    ];
    let worse_child = usize::from(first_children[1].upper_bound > first_children[0].upper_bound);
    let second_guidance = stable_set_lp_guidance_values(
        adjacency,
        weights,
        &first_children[worse_child].node.candidates,
    )?;
    let second_branch = lp_guided_branch(
        adjacency,
        weights,
        &first_children[worse_child].node.candidates,
        &second_guidance,
    );
    let second_children = child_entries(
        adjacency,
        weights,
        &first_children[worse_child],
        second_branch,
    )?;
    let leaves = vec![
        first_children[1 - worse_child].clone(),
        second_children[0].clone(),
        second_children[1].clone(),
    ];
    let terminal_totals = leaves
        .iter()
        .map(|entry| exact_side_weight + entry.upper_bound)
        .collect::<Vec<_>>();
    Ok(ParentWork {
        index,
        parent_total: exact_side_weight + parent.upper_bound,
        parent_depth: parent.node.depth,
        parent_digest: node_digest(parent),
        first_branch,
        first_child_totals,
        worse_child,
        second_branch,
        terminal_totals,
        leaves,
    })
}

fn certify_leaf(
    adjacency: &[BitWords],
    weights: &[i128],
    exact_side_weight: i128,
    leaf_index: usize,
    child: &QueueEntry,
) -> Result<LeafReplay, G27GeometricFractionalError> {
    let rows = stable_set_lp_relaxation_rows(adjacency, weights, &child.node.candidates)?;
    validate_rows(adjacency, &child.node.candidates, &rows)?;
    let mut row_payload = String::new();
    write_record(leaf_index, child, &rows, &mut row_payload);
    let explicit = explicit_rows(adjacency, &child.node.candidates, &rows);
    let certificate = solve_one_sided_dual(weights, &child.node.candidates, &explicit)?;
    let replay =
        replay_certificate_for_candidates(weights, &child.node.candidates, &explicit, &certificate);
    let certified_bound = child.node.chosen_weight + replay.objective_ceil;
    let certified_total = exact_side_weight + certified_bound;
    let objective_excess = replay.objective_ceil - rows.odd_cycle_objective_ceiling;
    let status = if !replay.coverage_ok {
        G27MwisFrontierClosureExactLeafStatus::DualCoverageFailed
    } else if replay.objective_ceil != rows.odd_cycle_objective_ceiling {
        G27MwisFrontierClosureExactLeafStatus::DualObjectiveMismatch
    } else if certified_bound > EXPECTED_DOMINANT_THRESHOLD || certified_total > TARGET_WEIGHT {
        G27MwisFrontierClosureExactLeafStatus::BoundAboveThreshold
    } else {
        G27MwisFrontierClosureExactLeafStatus::ExactLeafCertified
    };
    Ok(LeafReplay {
        row_payload: row_payload.clone(),
        leaf: G27MwisFrontierClosureExactLeaf {
            leaf_index,
            terminal_total: exact_side_weight + child.upper_bound,
            certified_total,
            explicit_rows: explicit.len(),
            positive_dual_rows: certificate.len(),
            max_denominator: max_denominator(&certificate),
            min_slack_floor: replay.min_slack_floor,
            objective_excess,
            row_digest: digest_text(&row_payload),
            dual_digest: certificate_digest(&certificate),
            status,
        },
    })
}

fn trace(stage: &str) {
    if std::env::var_os("HADWIGER_CAMPAIGN_TRACE").is_some() {
        eprintln!("exact {stage}");
    }
}
