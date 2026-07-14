use crate::domain_artifacts::core_artifact::{
    HadwigerArtifactAuthorityOwner, HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_same_field_fixed_dual_pricing_support::BitWords;
use super::g27_same_field_lp_relaxation::stable_set_lp_guidance_values;
use super::g27_same_field_mwis_branch_certificate_preflight::dominant_and_exact_side_weight;
use super::g27_same_field_mwis_branch_prefix_replay::{
    replay_g27_same_field_mwis_branch_prefix_checked, G27MwisBranchPrefixReplayStatus,
};
use super::g27_same_field_mwis_frontier_closure_campaign_support::{
    G27MwisFrontierCampaignNode, G27MwisFrontierCampaignRow, G27MwisFrontierCampaignRowClass,
    G27MwisFrontierCampaignStatus, G27MwisFrontierClosureCampaignScoutReport,
};
use super::g27_same_field_mwis_lp_guided_branch_support::{
    child_entries, initial_frontier, lp_guided_branch, node_digest, QueueEntry,
};
use super::g27_same_field_mwis_lp_guided_frontier_profiles::{
    EXPECTED_FRONTIER_DEPTHS, EXPECTED_FRONTIER_TOTALS,
};
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
const EXPECTED_BEST_OPEN_TOTAL: i128 = 518_612;
const CLOSED_PREFIX_COUNT: usize = 10;

pub fn scout_g27_same_field_mwis_frontier_closure_campaign_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27MwisFrontierClosureCampaignScoutReport, G27GeometricFractionalError> {
    trace("prefix");
    let prefix = replay_g27_same_field_mwis_branch_prefix_checked(handle)?;
    let (expanded, pruned, open, best_open_total, _, _) = prefix.summary();
    if prefix.status() != G27MwisBranchPrefixReplayStatus::BranchPrefixSemanticsPreflight
        || expanded != EXPECTED_PREFIX_EXPANDED
        || pruned != EXPECTED_PREFIX_PRUNED
        || open != EXPECTED_PREFIX_OPEN
        || best_open_total != EXPECTED_BEST_OPEN_TOTAL
    {
        return empty_report(G27MwisFrontierCampaignStatus::PrefixReplayMismatch);
    }
    trace("channel");
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
            source: "frontier_closure_campaign_channel",
        })?;
    let (dominant, small_weight) = dominant_and_exact_side_weight(&channel.instance);
    if small_weight != EXPECTED_EXACT_SIDE_WEIGHT {
        return empty_report(G27MwisFrontierCampaignStatus::FrozenInstanceMismatch);
    }
    trace("run_campaign");
    run_campaign_scout(
        &channel.instance.adjacency,
        &channel.instance.weights,
        &dominant,
        small_weight,
    )
}

fn run_campaign_scout(
    adjacency: &[BitWords],
    weights: &[i128],
    candidates: &[usize],
    exact_side_weight: i128,
) -> Result<G27MwisFrontierClosureCampaignScoutReport, G27GeometricFractionalError> {
    trace("frontier");
    let frontier = initial_frontier(
        adjacency,
        weights,
        candidates,
        EXPECTED_DOMINANT_THRESHOLD,
        EXPECTED_PREFIX_EXPANDED,
        EXPECTED_PREFIX_PRUNED,
        EXPECTED_PREFIX_OPEN,
    )?;
    if !frontier_shape_matches(&frontier, exact_side_weight) {
        return empty_report(G27MwisFrontierCampaignStatus::FrontierShapeMismatch);
    }
    trace("frontier_nodes");
    let frontier_nodes = frontier
        .iter()
        .enumerate()
        .map(|(index, entry)| G27MwisFrontierCampaignNode {
            index,
            total: exact_side_weight + entry.upper_bound,
            depth: entry.node.depth,
            digest: node_digest(entry),
            previously_closed: index < CLOSED_PREFIX_COUNT,
        })
        .collect::<Vec<_>>();
    trace("scout_rows");
    let mut scout_rows = Vec::new();
    for (index, entry) in frontier.iter().enumerate().skip(CLOSED_PREFIX_COUNT) {
        trace(&format!("scout_row_{index}"));
        scout_rows.push(scout_parent(
            adjacency,
            weights,
            exact_side_weight,
            index,
            entry,
        )?);
    }
    trace("report");
    Ok(report(
        G27MwisFrontierCampaignStatus::CampaignScoutReady,
        frontier_nodes,
        scout_rows,
    )?)
}

fn trace(stage: &str) {
    if std::env::var_os("HADWIGER_CAMPAIGN_TRACE").is_some() {
        eprintln!("campaign {stage}");
    }
}

fn scout_parent(
    adjacency: &[BitWords],
    weights: &[i128],
    exact_side_weight: i128,
    index: usize,
    parent: &QueueEntry,
) -> Result<G27MwisFrontierCampaignRow, G27GeometricFractionalError> {
    let first_guidance =
        match stable_set_lp_guidance_values(adjacency, weights, &parent.node.candidates) {
            Ok(values) => values,
            Err(_) => return Ok(failed_row(exact_side_weight, index, parent)),
        };
    let first_branch =
        lp_guided_branch(adjacency, weights, &parent.node.candidates, &first_guidance);
    let first_children = match child_entries(adjacency, weights, parent, first_branch) {
        Ok(children) => children,
        Err(_) => return Ok(failed_row(exact_side_weight, index, parent)),
    };
    let first_child_totals = [
        exact_side_weight + first_children[0].upper_bound,
        exact_side_weight + first_children[1].upper_bound,
    ];
    let worse_child = usize::from(first_children[1].upper_bound > first_children[0].upper_bound);
    let second_guidance = match stable_set_lp_guidance_values(
        adjacency,
        weights,
        &first_children[worse_child].node.candidates,
    ) {
        Ok(values) => values,
        Err(_) => return Ok(failed_row(exact_side_weight, index, parent)),
    };
    let second_branch = lp_guided_branch(
        adjacency,
        weights,
        &first_children[worse_child].node.candidates,
        &second_guidance,
    );
    let second_children = match child_entries(
        adjacency,
        weights,
        &first_children[worse_child],
        second_branch,
    ) {
        Ok(children) => children,
        Err(_) => return Ok(failed_row(exact_side_weight, index, parent)),
    };
    let terminal_totals = vec![
        exact_side_weight + first_children[1 - worse_child].upper_bound,
        exact_side_weight + second_children[0].upper_bound,
        exact_side_weight + second_children[1].upper_bound,
    ];
    let row_class = if terminal_totals.iter().all(|total| *total <= TARGET_WEIGHT) {
        G27MwisFrontierCampaignRowClass::ReadyForExactThreeLeafReplay
    } else {
        G27MwisFrontierCampaignRowClass::FloatingAboveTarget
    };
    Ok(G27MwisFrontierCampaignRow {
        index,
        parent_total: exact_side_weight + parent.upper_bound,
        parent_depth: parent.node.depth,
        parent_digest: node_digest(parent),
        first_branch,
        first_child_totals,
        worse_child,
        second_branch,
        terminal_totals,
        row_class,
    })
}

fn failed_row(
    exact_side_weight: i128,
    index: usize,
    parent: &QueueEntry,
) -> G27MwisFrontierCampaignRow {
    G27MwisFrontierCampaignRow {
        index,
        parent_total: exact_side_weight + parent.upper_bound,
        parent_depth: parent.node.depth,
        parent_digest: node_digest(parent),
        first_branch: 0,
        first_child_totals: [0, 0],
        worse_child: 0,
        second_branch: 0,
        terminal_totals: Vec::new(),
        row_class: G27MwisFrontierCampaignRowClass::LpSolveFailed,
    }
}

fn frontier_shape_matches(frontier: &[QueueEntry], exact_side_weight: i128) -> bool {
    frontier.len() == EXPECTED_PREFIX_OPEN
        && EXPECTED_FRONTIER_TOTALS
            .iter()
            .zip(EXPECTED_FRONTIER_DEPTHS.iter())
            .enumerate()
            .all(|(index, (total, depth))| {
                frontier.get(index).is_some_and(|entry| {
                    exact_side_weight + entry.upper_bound == *total && entry.node.depth == *depth
                })
            })
}

fn report(
    status: G27MwisFrontierCampaignStatus,
    frontier_nodes: Vec<G27MwisFrontierCampaignNode>,
    scout_rows: Vec<G27MwisFrontierCampaignRow>,
) -> Result<G27MwisFrontierClosureCampaignScoutReport, G27GeometricFractionalError> {
    let ready_count = scout_rows
        .iter()
        .filter(|row| {
            row.row_class == G27MwisFrontierCampaignRowClass::ReadyForExactThreeLeafReplay
        })
        .count();
    let failing_count = scout_rows.len().saturating_sub(ready_count);
    let worst_terminal_total = scout_rows
        .iter()
        .flat_map(|row| row.terminal_totals.iter().copied())
        .max()
        .unwrap_or(0);
    let continuation_max_total = scout_rows
        .iter()
        .flat_map(|row| row.terminal_totals.iter().copied())
        .filter(|total| *total > TARGET_WEIGHT)
        .max()
        .unwrap_or(0);
    let core = artifact_core(
        HadwigerArtifactKind::G27MwisFrontierClosureCampaignScoutReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_same_field_mwis_frontier_closure_campaign_scout".to_string(),
        },
        Vec::new(),
        payload(
            status,
            &frontier_nodes,
            &scout_rows,
            ready_count,
            failing_count,
        ),
    )?;
    Ok(G27MwisFrontierClosureCampaignScoutReport {
        core,
        status,
        frontier_nodes,
        scout_rows,
        ready_count,
        failing_count,
        worst_terminal_total,
        continuation_max_total,
    })
}

fn empty_report(
    status: G27MwisFrontierCampaignStatus,
) -> Result<G27MwisFrontierClosureCampaignScoutReport, G27GeometricFractionalError> {
    report(status, Vec::new(), Vec::new())
}

fn payload(
    status: G27MwisFrontierCampaignStatus,
    frontier_nodes: &[G27MwisFrontierCampaignNode],
    scout_rows: &[G27MwisFrontierCampaignRow],
    ready_count: usize,
    failing_count: usize,
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text("status", status.as_str()),
        HadwigerArtifactPayloadEntry::unsigned("target_weight", TARGET_WEIGHT as u128),
        HadwigerArtifactPayloadEntry::unsigned("closed_prefix_count", CLOSED_PREFIX_COUNT as u128),
        HadwigerArtifactPayloadEntry::unsigned("frontier_count", frontier_nodes.len() as u128),
        HadwigerArtifactPayloadEntry::unsigned("scout_count", scout_rows.len() as u128),
        HadwigerArtifactPayloadEntry::unsigned("ready_count", ready_count as u128),
        HadwigerArtifactPayloadEntry::unsigned("failing_count", failing_count as u128),
    ];
    for node in frontier_nodes {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "frontier_node",
            node_token(node),
        ));
    }
    for row in scout_rows {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "scout_row",
            row_token(row),
        ));
    }
    payload
}

fn node_token(node: &G27MwisFrontierCampaignNode) -> String {
    format!(
        "{}:{}:{}:{}:{}",
        node.index, node.total, node.depth, node.digest, node.previously_closed
    )
}

fn row_token(row: &G27MwisFrontierCampaignRow) -> String {
    format!(
        "{}:{}:{}:{}:{}:{:?}:{}:{:?}:{}",
        row.index,
        row.parent_total,
        row.parent_depth,
        row.parent_digest,
        row.first_branch,
        row.first_child_totals,
        row.second_branch,
        row.terminal_totals,
        row.row_class.as_str()
    )
}
