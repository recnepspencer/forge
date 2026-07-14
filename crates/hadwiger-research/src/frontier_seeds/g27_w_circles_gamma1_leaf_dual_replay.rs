use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::query_entry::HadwigerResearchHandle;

use super::g27_finite_fractional_core_audit::audit_g27_w_circles_607_finite_fractional_core_checked;
use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_w_circles_exact_geometry_support::{
    parse_w_integer_weights, parse_w_retained_edges, EXPECTED_EDGE_COUNT, EXPECTED_VERTEX_COUNT,
};
use super::g27_w_circles_gamma0_leaf_dual_support::{
    active_vertices, adjacency_from_edges, one_based, recompute_c0, replay_leaf,
    verify_leaf_sets_with_base, verify_partition_with_base, Gamma1Artifact, BRANCH_VERTEX,
    DENOMINATOR,
};
use super::g27_w_circles_gamma0_rank_support::rank_registry;

const CERT: &str = include_str!("../../docs/w607-gamma1-leaf-dual-export.json");
const EXCLUDE_CERT: &str = include_str!("../../docs/w607-v304-exclude-dual-cover-den1024.json");
const TARGET_GAMMA1: i128 = 546_085_806;
const EXPECTED_LEAVES: usize = 16;
const EXPECTED_POSITIVE_ROWS: usize = 8_555;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27WCirclesGamma1LeafDualReplayStatus {
    ReplayedGamma1BranchCertificate,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27WCirclesGamma1LeafDualReplayReport {
    core: HadwigerArtifactCore,
    leaf_count: usize,
    total_positive_rows: usize,
    worst_leaf_objective_num: i128,
    min_leaf_slack: i128,
    status: G27WCirclesGamma1LeafDualReplayStatus,
    conclusion: String,
}

impl G27WCirclesGamma1LeafDualReplayReport {
    pub fn summary(&self) -> (usize, usize, i128, i128) {
        (
            self.leaf_count,
            self.total_positive_rows,
            self.worst_leaf_objective_num,
            self.min_leaf_slack,
        )
    }

    pub fn status(&self) -> G27WCirclesGamma1LeafDualReplayStatus {
        self.status
    }

    pub fn conclusion(&self) -> &str {
        &self.conclusion
    }
}

impl_hadwiger_artifact!(G27WCirclesGamma1LeafDualReplayReport, core);

pub fn replay_g27_w_circles_gamma1_leaf_dual_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27WCirclesGamma1LeafDualReplayReport, G27GeometricFractionalError> {
    let source = audit_g27_w_circles_607_finite_fractional_core_checked(handle)?;
    let weights = parse_w_integer_weights()?;
    let edges = parse_w_retained_edges(EXPECTED_VERTEX_COUNT)?;
    if weights.len() != EXPECTED_VERTEX_COUNT || edges.len() != EXPECTED_EDGE_COUNT {
        return malformed("w607_gamma1_shape");
    }
    let artifact: Gamma1Artifact =
        serde_json::from_str(CERT).map_err(|_| malformed_err("w607_gamma1_json"))?;
    verify_header(&artifact)?;
    let adjacency = adjacency_from_edges(&edges);
    let c0 = recompute_c0(EXCLUDE_CERT, &weights)?;
    let rank_registry = rank_registry(&weights, &adjacency)?;
    let base = [BRANCH_VERTEX];
    verify_partition_with_base(&artifact.leaves, &base, &adjacency)?;
    let mut worst_objective = 0_i128;
    let mut min_slack = i128::MAX;
    let mut total_rows = 0_usize;
    for leaf in &artifact.leaves {
        let mut included = one_based(&leaf.included)?;
        included.push(BRANCH_VERTEX);
        included.sort_unstable();
        let excluded = one_based(&leaf.excluded)?;
        verify_leaf_sets_with_base(&included, &excluded, &adjacency, true)?;
        let active = active_vertices(&included, &excluded, &adjacency);
        if active.len() != leaf.active_vertices {
            return malformed("w607_gamma1_active");
        }
        let success = leaf
            .success
            .as_ref()
            .ok_or(malformed_err("w607_gamma1_leaf_success"))?;
        if success.denominator != DENOMINATOR
            || success.target_num != TARGET_GAMMA1 * DENOMINATOR
            || success.positive_row_count != success.rows.len()
        {
            return malformed("w607_gamma1_leaf_header");
        }
        let included_c0: i128 = included.iter().map(|vertex| c0[*vertex]).sum();
        if included_c0 != leaf.included_c0_weight {
            return malformed("w607_gamma1_included_c0");
        }
        let (objective, slack) = replay_leaf(
            success,
            &included,
            &active,
            &weights,
            &c0,
            &adjacency,
            &rank_registry,
        )?;
        if objective != success.objective_num
            || objective > TARGET_GAMMA1 * DENOMINATOR
            || slack != success.min_slack
            || slack < 0
        {
            return malformed("w607_gamma1_leaf_replay");
        }
        worst_objective = worst_objective.max(objective);
        min_slack = min_slack.min(slack);
        total_rows += success.rows.len();
    }
    if total_rows != EXPECTED_POSITIVE_ROWS {
        return malformed("w607_gamma1_total_rows");
    }
    report(source.reference(), total_rows, worst_objective, min_slack)
}

fn verify_header(artifact: &Gamma1Artifact) -> Result<(), G27GeometricFractionalError> {
    if artifact.schema != "forge.hadwiger.w607_gamma1_leaf_dual_export.v1"
        || artifact.branch_domain != "x304=1_residual"
        || artifact.base_included_vertex != BRANCH_VERTEX + 1
        || artifact.target_gamma1 != TARGET_GAMMA1
        || artifact.max_success_denominator != Some(DENOMINATOR)
        || artifact.leaf_count != EXPECTED_LEAVES
        || artifact.successful_leaf_count != EXPECTED_LEAVES
        || artifact.status != "FundGamma1LeafDualReplay"
    {
        return malformed("w607_gamma1_header");
    }
    Ok(())
}

fn report(
    source: crate::domain_artifacts::HadwigerArtifactReference,
    total_rows: usize,
    worst_objective: i128,
    min_slack: i128,
) -> Result<G27WCirclesGamma1LeafDualReplayReport, G27GeometricFractionalError> {
    let conclusion = format!(
        "replayed gamma1 branch certificate: {EXPECTED_LEAVES} leaves, denominator {DENOMINATOR}, worst objective {worst_objective}/{DENOMINATOR} <= {TARGET_GAMMA1}"
    );
    let core = artifact_core(
        HadwigerArtifactKind::G27WCirclesGamma1LeafDualReplayReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_w_circles_gamma1_leaf_dual_replay".to_string(),
        },
        vec![source],
        payload(total_rows, worst_objective, min_slack, &conclusion),
    )?;
    Ok(G27WCirclesGamma1LeafDualReplayReport {
        core,
        leaf_count: EXPECTED_LEAVES,
        total_positive_rows: total_rows,
        worst_leaf_objective_num: worst_objective,
        min_leaf_slack: min_slack,
        status: G27WCirclesGamma1LeafDualReplayStatus::ReplayedGamma1BranchCertificate,
        conclusion,
    })
}

fn payload(
    total_rows: usize,
    worst_objective: i128,
    min_slack: i128,
    conclusion: &str,
) -> Vec<HadwigerArtifactPayloadEntry> {
    vec![
        HadwigerArtifactPayloadEntry::text(
            "schema",
            "forge.hadwiger.w607_gamma1_leaf_dual_replay.v1",
        ),
        HadwigerArtifactPayloadEntry::unsigned("leaf_count", EXPECTED_LEAVES as u128),
        HadwigerArtifactPayloadEntry::unsigned("total_positive_rows", total_rows as u128),
        HadwigerArtifactPayloadEntry::unsigned("worst_leaf_objective_num", worst_objective as u128),
        HadwigerArtifactPayloadEntry::unsigned("min_leaf_slack", min_slack as u128),
        HadwigerArtifactPayloadEntry::text("conclusion", conclusion),
    ]
}

fn malformed<T>(source: &'static str) -> Result<T, G27GeometricFractionalError> {
    Err(malformed_err(source))
}

fn malformed_err(source: &'static str) -> G27GeometricFractionalError {
    G27GeometricFractionalError::MalformedData { source }
}
