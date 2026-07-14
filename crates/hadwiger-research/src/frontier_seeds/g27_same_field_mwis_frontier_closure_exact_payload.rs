use sha2::{Digest, Sha256};

use crate::domain_artifacts::core_artifact::{
    HadwigerArtifactAuthorityOwner, HadwigerArtifactKind, HadwigerArtifactReference,
    HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_same_field_mwis_frontier_closure_exact_support::{
    G27MwisFrontierClosureExactLeaf, G27MwisFrontierClosureExactLeafStatus,
    G27MwisFrontierClosureExactNode, G27MwisFrontierClosureExactReplayReport,
    G27MwisFrontierClosureExactStatus,
};
use super::g27_same_field_mwis_lp_guided_branch_support::QueueEntry;
use super::g27_same_field_mwis_odd_cycle_dual_replay_support::CertifiedRow;

const TARGET_WEIGHT: i128 = 512_933;
const EXPECTED_PREFIX_OPEN: usize = 28;

pub(super) struct ParentWork {
    pub(super) index: usize,
    pub(super) parent_total: i128,
    pub(super) parent_depth: usize,
    pub(super) parent_digest: String,
    pub(super) first_branch: usize,
    pub(super) first_child_totals: [i128; 2],
    pub(super) worse_child: usize,
    pub(super) second_branch: usize,
    pub(super) terminal_totals: Vec<i128>,
    pub(super) leaves: Vec<QueueEntry>,
}

pub(super) struct LeafReplay {
    pub(super) leaf: G27MwisFrontierClosureExactLeaf,
    pub(super) row_payload: String,
}

pub(super) fn report(
    source: HadwigerArtifactReference,
    scout_digest: String,
    selected_start: usize,
    selected_end: usize,
    status: G27MwisFrontierClosureExactStatus,
    nodes: Vec<G27MwisFrontierClosureExactNode>,
) -> Result<G27MwisFrontierClosureExactReplayReport, G27GeometricFractionalError> {
    let checked_nodes = nodes.len();
    let certified_nodes = nodes
        .iter()
        .filter(|node| node.status == G27MwisFrontierClosureExactStatus::ExactChunkCertified)
        .count();
    let certified_leaves = nodes.iter().map(|node| node.certified_leaves).sum();
    let explicit_rows = nodes.iter().map(|node| node.explicit_rows).sum();
    let positive_dual_rows = nodes.iter().map(|node| node.positive_dual_rows).sum();
    let max_denominator = nodes
        .iter()
        .map(|node| node.max_denominator)
        .max()
        .unwrap_or(1);
    let min_slack_floor = nodes
        .iter()
        .map(|node| node.min_slack_floor)
        .min()
        .unwrap_or(i128::MAX);
    let max_objective_excess = nodes
        .iter()
        .map(|node| node.max_objective_excess)
        .max()
        .unwrap_or(0);
    let worst_terminal_total = nodes
        .iter()
        .flat_map(|node| node.terminal_totals.iter().copied())
        .max()
        .unwrap_or(0);
    let unresolved_start = selected_end.min(EXPECTED_PREFIX_OPEN);
    let unresolved_end = EXPECTED_PREFIX_OPEN;
    let core = artifact_core(
        HadwigerArtifactKind::G27MwisFrontierClosureExactReplayReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_same_field_mwis_frontier_closure_exact_chunk".to_string(),
        },
        vec![source],
        payload(
            &scout_digest,
            selected_start,
            selected_end,
            unresolved_start,
            unresolved_end,
            status,
            &nodes,
        ),
    )?;
    Ok(G27MwisFrontierClosureExactReplayReport {
        core,
        scout_digest,
        selected_start,
        selected_end,
        unresolved_start,
        unresolved_end,
        checked_nodes,
        certified_nodes,
        certified_leaves,
        explicit_rows,
        positive_dual_rows,
        max_denominator,
        min_slack_floor,
        max_objective_excess,
        worst_terminal_total,
        nodes,
        status,
    })
}

pub(super) fn node_from_work(
    work: ParentWork,
    leaves: Vec<G27MwisFrontierClosureExactLeaf>,
    row_digest: String,
    status: G27MwisFrontierClosureExactStatus,
) -> G27MwisFrontierClosureExactNode {
    let certified_leaves = leaves
        .iter()
        .filter(|leaf| leaf.status == G27MwisFrontierClosureExactLeafStatus::ExactLeafCertified)
        .count();
    let explicit_rows = leaves.iter().map(|leaf| leaf.explicit_rows).sum();
    let positive_dual_rows = leaves.iter().map(|leaf| leaf.positive_dual_rows).sum();
    let max_denominator = leaves
        .iter()
        .map(|leaf| leaf.max_denominator)
        .max()
        .unwrap_or(1);
    let min_slack_floor = leaves
        .iter()
        .map(|leaf| leaf.min_slack_floor)
        .min()
        .unwrap_or(i128::MAX);
    let max_objective_excess = leaves
        .iter()
        .map(|leaf| leaf.objective_excess.max(0))
        .max()
        .unwrap_or(0);
    G27MwisFrontierClosureExactNode {
        index: work.index,
        parent_total: work.parent_total,
        parent_depth: work.parent_depth,
        parent_digest: work.parent_digest,
        first_branch: work.first_branch,
        first_child_totals: work.first_child_totals,
        worse_child: work.worse_child,
        second_branch: work.second_branch,
        terminal_totals: work.terminal_totals,
        certified_leaves,
        explicit_rows,
        positive_dual_rows,
        max_denominator,
        min_slack_floor,
        max_objective_excess,
        row_digest,
        leaves,
        status,
    }
}

pub(super) fn max_denominator(certificate: &[CertifiedRow]) -> i128 {
    certificate
        .iter()
        .map(|row| row.multiplier.den_i128())
        .max()
        .unwrap_or(1)
}

pub(super) fn certificate_digest(certificate: &[CertifiedRow]) -> String {
    let mut payload = String::new();
    for row in certificate {
        payload.push_str(&format!(
            "{}:{}/{}\n",
            row.index,
            row.multiplier.num_i128(),
            row.multiplier.den_i128()
        ));
    }
    digest_text(&payload)
}

pub(super) fn digest_text(payload: &str) -> String {
    format!("{:x}", Sha256::digest(payload.as_bytes()))
}

fn payload(
    scout_digest: &str,
    selected_start: usize,
    selected_end: usize,
    unresolved_start: usize,
    unresolved_end: usize,
    status: G27MwisFrontierClosureExactStatus,
    nodes: &[G27MwisFrontierClosureExactNode],
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text(
            "schema",
            "forge.hadwiger.g27_mwis_frontier_closure_exact_chunk.v1",
        ),
        HadwigerArtifactPayloadEntry::text("status", status.as_str()),
        HadwigerArtifactPayloadEntry::text("source_scout_digest", scout_digest),
        HadwigerArtifactPayloadEntry::unsigned("target_weight", TARGET_WEIGHT as u128),
        HadwigerArtifactPayloadEntry::unsigned("selected_start", selected_start as u128),
        HadwigerArtifactPayloadEntry::unsigned("selected_end", selected_end as u128),
        HadwigerArtifactPayloadEntry::text(
            "unresolved_suffix",
            format!("{unresolved_start}..{unresolved_end}"),
        ),
        HadwigerArtifactPayloadEntry::text("theorem_authority", "false"),
    ];
    for node in nodes {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "exact_node",
            node_token(node),
        ));
        for leaf in &node.leaves {
            payload.push(HadwigerArtifactPayloadEntry::text(
                "exact_leaf",
                leaf_token(node.index, leaf),
            ));
        }
    }
    payload
}

fn node_token(node: &G27MwisFrontierClosureExactNode) -> String {
    format!(
        "{}:{}:{}:{}:{}:{:?}:{}:{}:{:?}:{}:{}:{}:{}:{}:{}:{}:{}",
        node.index,
        node.parent_total,
        node.parent_depth,
        node.parent_digest,
        node.first_branch,
        node.first_child_totals,
        node.worse_child,
        node.second_branch,
        node.terminal_totals,
        node.certified_leaves,
        node.explicit_rows,
        node.positive_dual_rows,
        node.max_denominator,
        node.min_slack_floor,
        node.max_objective_excess,
        node.row_digest,
        node.status.as_str()
    )
}

fn leaf_token(node_index: usize, leaf: &G27MwisFrontierClosureExactLeaf) -> String {
    format!(
        "{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
        node_index,
        leaf.leaf_index,
        leaf.terminal_total,
        leaf.certified_total,
        leaf.explicit_rows,
        leaf.positive_dual_rows,
        leaf.max_denominator,
        leaf.min_slack_floor,
        leaf.objective_excess,
        leaf.row_digest,
        leaf.dual_digest,
        leaf.status.as_str()
    )
}
