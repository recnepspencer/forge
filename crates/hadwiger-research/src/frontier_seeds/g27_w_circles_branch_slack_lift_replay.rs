use std::collections::{BTreeMap, BTreeSet};

use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::query_entry::HadwigerResearchHandle;

use super::g27_finite_fractional_core_audit::audit_g27_w_circles_607_finite_fractional_core_checked;
use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_same_field_fixed_dual_pricing_support::{has_bit, BitWords};
use super::g27_w_circles_branch_slack_support::{
    coefficient_map, fraction_to_d1024, verify_branch_partition, BranchSlackArtifact,
};
use super::g27_w_circles_exact_geometry_support::{
    parse_w_integer_weights, parse_w_retained_edges, EXPECTED_EDGE_COUNT, EXPECTED_VERTEX_COUNT,
};
use super::g27_w_circles_gamma0_leaf_dual_replay::replay_g27_w_circles_gamma0_leaf_dual_checked;
use super::g27_w_circles_gamma0_leaf_dual_support::{
    active_vertices, adjacency_from_edges, one_based, recompute_c0, verify_leaf_sets_with_base,
    Gamma0Artifact, LeafSuccess, BRANCH_VERTEX, DENOMINATOR,
};
use super::g27_w_circles_gamma0_rank_support::{rank_registry, RankCut};
use super::g27_w_circles_projected_parent_lift_replay::replay_g27_w_circles_projected_parent_lift_checked;

const CERT: &str = include_str!("../../docs/w607-branch-slack-parent-lift-diagnostic.json");
const GAMMA0_CERT: &str = include_str!("../../docs/w607-gamma0-leaf-dual-export.json");
const EXCLUDE_CERT: &str = include_str!("../../docs/w607-v304-exclude-dual-cover-den1024.json");
const GAMMA0_MOD_NUM: i128 = 623_894_447_014;
const GAMMA1_MOD_NUM: i128 = 559_085_319_025;
const LIFT_NUM: i128 = 64_809_127_989;
const EXPECTED_ROWS: usize = 8_555;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27WCirclesBranchSlackLiftReplayStatus {
    ReplayedBranchSlackLift,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27WCirclesBranchSlackLiftReplayReport {
    core: HadwigerArtifactCore,
    gamma0_modified_num: i128,
    gamma1_modified_num: i128,
    lift_num: i128,
    modified_gamma1_rows: usize,
    status: G27WCirclesBranchSlackLiftReplayStatus,
    conclusion: String,
}

impl G27WCirclesBranchSlackLiftReplayReport {
    pub fn summary(&self) -> (i128, i128, i128, usize) {
        (
            self.gamma0_modified_num,
            self.gamma1_modified_num,
            self.lift_num,
            self.modified_gamma1_rows,
        )
    }

    pub fn status(&self) -> G27WCirclesBranchSlackLiftReplayStatus {
        self.status
    }

    pub fn conclusion(&self) -> &str {
        &self.conclusion
    }
}

impl_hadwiger_artifact!(G27WCirclesBranchSlackLiftReplayReport, core);

pub fn replay_g27_w_circles_branch_slack_lift_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27WCirclesBranchSlackLiftReplayReport, G27GeometricFractionalError> {
    let source = audit_g27_w_circles_607_finite_fractional_core_checked(handle)?;
    let parent = replay_g27_w_circles_projected_parent_lift_checked(handle)?;
    let gamma0 = replay_g27_w_circles_gamma0_leaf_dual_checked(handle)?;
    let weights = parse_w_integer_weights()?;
    let edges = parse_w_retained_edges(EXPECTED_VERTEX_COUNT)?;
    if weights.len() != EXPECTED_VERTEX_COUNT || edges.len() != EXPECTED_EDGE_COUNT {
        return malformed("w607_branch_slack_shape");
    }
    let adjacency = adjacency_from_edges(&edges);
    let c0 = recompute_c0(EXCLUDE_CERT, &weights)?;
    let ranks = rank_registry(&weights, &adjacency)?;
    let artifact: BranchSlackArtifact =
        serde_json::from_str(CERT).map_err(|_| malformed_err("w607_branch_slack_json"))?;
    verify_header(&artifact)?;
    let p = coefficient_map(&artifact)?;
    let cmod = modified_coefficients(&c0, &p);
    verify_gamma0_slack(&p, &adjacency)?;
    let rows = replay_modified_gamma1(&artifact, &cmod, &weights, &adjacency, &ranks)?;
    let conclusion =
        format!("replayed branch-slack lift: 1024*(c0+p)*x + {LIFT_NUM}*x_304 <= {GAMMA0_MOD_NUM}");
    let core = artifact_core(
        HadwigerArtifactKind::G27WCirclesBranchSlackLiftReplayReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_w_circles_branch_slack_lift_replay".to_string(),
        },
        vec![source.reference(), parent.reference(), gamma0.reference()],
        payload(rows, &conclusion),
    )?;
    Ok(G27WCirclesBranchSlackLiftReplayReport {
        core,
        gamma0_modified_num: GAMMA0_MOD_NUM,
        gamma1_modified_num: GAMMA1_MOD_NUM,
        lift_num: LIFT_NUM,
        modified_gamma1_rows: rows,
        status: G27WCirclesBranchSlackLiftReplayStatus::ReplayedBranchSlackLift,
        conclusion,
    })
}

fn verify_header(artifact: &BranchSlackArtifact) -> Result<(), G27GeometricFractionalError> {
    if artifact.schema != "forge.hadwiger.w607_branch_slack_parent_lift_diagnostic.v1"
        || artifact.canonical_denominator != DENOMINATOR
        || artifact.gamma0_modified_num_d1024 != GAMMA0_MOD_NUM
        || artifact.worst_gamma1_modified_num_d1024 != GAMMA1_MOD_NUM
        || artifact.lift_coefficient_num_d1024 != LIFT_NUM
        || artifact.status != "FundBranchSlackParentLift"
        || artifact.leaf_count != 16
        || artifact.successful_leaf_count != 16
    {
        return malformed("w607_branch_slack_header");
    }
    if artifact.gamma0_modified_num_d1024 - artifact.worst_gamma1_modified_num_d1024 != LIFT_NUM {
        return malformed("w607_branch_slack_lift");
    }
    Ok(())
}

fn verify_gamma0_slack(
    p: &BTreeMap<usize, i128>,
    adjacency: &[BitWords],
) -> Result<(), G27GeometricFractionalError> {
    let gamma0: Gamma0Artifact = serde_json::from_str(GAMMA0_CERT)
        .map_err(|_| malformed_err("w607_branch_slack_g0_json"))?;
    for leaf in gamma0.leaves {
        let included = one_based(&leaf.included)?;
        let excluded = one_based(&leaf.excluded)?;
        let active = active_vertices(&included, &excluded, adjacency)
            .into_iter()
            .collect::<BTreeSet<_>>();
        let included = included.into_iter().collect::<BTreeSet<_>>();
        let excluded = excluded.into_iter().collect::<BTreeSet<_>>();
        let success = leaf
            .success
            .ok_or(malformed_err("w607_branch_slack_g0_leaf"))?;
        let charge = p
            .iter()
            .filter(|(vertex, _)| {
                included.contains(vertex) || (active.contains(vertex) && !excluded.contains(vertex))
            })
            .map(|(_, num)| *num)
            .sum::<i128>();
        if success.objective_num + charge > GAMMA0_MOD_NUM {
            return malformed("w607_branch_slack_g0_charge");
        }
    }
    Ok(())
}

fn replay_modified_gamma1(
    artifact: &BranchSlackArtifact,
    cmod: &[i128],
    weights: &[i128],
    adjacency: &[BitWords],
    ranks: &[RankCut],
) -> Result<usize, G27GeometricFractionalError> {
    let base = [BRANCH_VERTEX];
    verify_branch_partition(&artifact.leaf_reports, &base, adjacency)?;
    let mut total_rows = 0_usize;
    let mut worst = 0_i128;
    for leaf in &artifact.leaf_reports {
        let mut included = one_based(&leaf.included)?;
        included.push(BRANCH_VERTEX);
        included.sort_unstable();
        let excluded = one_based(&leaf.excluded)?;
        verify_leaf_sets_with_base(&included, &excluded, adjacency, true)?;
        let active = active_vertices(&included, &excluded, adjacency);
        if active.len() != leaf.active_vertices {
            return malformed("w607_branch_slack_active");
        }
        let success = leaf
            .success
            .as_ref()
            .ok_or(malformed_err("w607_branch_slack_leaf"))?;
        let included_num = included.iter().map(|vertex| cmod[*vertex]).sum::<i128>();
        if included_num != fraction_to_d1024(&leaf.included_modified_weight)? {
            return malformed("w607_branch_slack_included");
        }
        let (objective, slack) = replay_scaled_leaf(
            success,
            included_num,
            &included,
            &active,
            weights,
            cmod,
            adjacency,
            ranks,
        )?;
        if objective != success.objective_num
            || slack != success.min_slack
            || slack < 0
            || objective > GAMMA1_MOD_NUM
        {
            return malformed("w607_branch_slack_leaf_replay");
        }
        worst = worst.max(objective);
        total_rows += success.rows.len();
    }
    if worst != GAMMA1_MOD_NUM || total_rows != EXPECTED_ROWS {
        return malformed("w607_branch_slack_rows");
    }
    Ok(total_rows)
}

fn replay_scaled_leaf(
    success: &LeafSuccess,
    included_num: i128,
    included: &[usize],
    active: &[usize],
    weights: &[i128],
    cmod: &[i128],
    adjacency: &[BitWords],
    ranks: &[RankCut],
) -> Result<(i128, i128), G27GeometricFractionalError> {
    let active_set = active.iter().copied().collect::<BTreeSet<_>>();
    let included_set = included.iter().copied().collect::<BTreeSet<_>>();
    let mut coverage = vec![0_i128; EXPECTED_VERTEX_COUNT];
    let mut objective = included_num;
    for row in &success.rows {
        objective += row.numerator * row.rhs;
        let vertices = one_based(&row.vertices)?;
        match row.kind.as_str() {
            "edge" => replay_unit(row, &vertices, 2, adjacency, &active_set, &mut coverage)?,
            "triangle" => replay_unit(row, &vertices, 3, adjacency, &active_set, &mut coverage)?,
            "rank" => replay_rank(
                row,
                &vertices,
                weights,
                &included_set,
                &active_set,
                ranks,
                &mut coverage,
            )?,
            _ => return malformed("w607_branch_slack_row_kind"),
        }
    }
    let min_slack = active
        .iter()
        .map(|vertex| coverage[*vertex] - cmod[*vertex])
        .min()
        .unwrap_or(0);
    Ok((objective, min_slack))
}

fn replay_unit(
    row: &super::g27_w_circles_gamma0_leaf_dual_support::LeafRow,
    vertices: &[usize],
    size: usize,
    adjacency: &[BitWords],
    active: &BTreeSet<usize>,
    coverage: &mut [i128],
) -> Result<(), G27GeometricFractionalError> {
    if row.rhs != 1 || vertices.len() != size || vertices.iter().any(|v| !active.contains(v)) {
        return malformed("w607_branch_slack_unit");
    }
    if size == 2 && !has_bit(&adjacency[vertices[0]], vertices[1]) {
        return malformed("w607_branch_slack_edge");
    }
    if size == 3
        && (!has_bit(&adjacency[vertices[0]], vertices[1])
            || !has_bit(&adjacency[vertices[0]], vertices[2])
            || !has_bit(&adjacency[vertices[1]], vertices[2]))
    {
        return malformed("w607_branch_slack_triangle");
    }
    for vertex in vertices {
        coverage[*vertex] += row.numerator;
    }
    Ok(())
}

fn replay_rank(
    row: &super::g27_w_circles_gamma0_leaf_dual_support::LeafRow,
    vertices: &[usize],
    weights: &[i128],
    included: &BTreeSet<usize>,
    active: &BTreeSet<usize>,
    ranks: &[RankCut],
    coverage: &mut [i128],
) -> Result<(), G27GeometricFractionalError> {
    let name = row
        .name
        .as_deref()
        .ok_or(malformed_err("w607_branch_slack_rank"))?;
    let cut = ranks
        .iter()
        .find(|cut| cut.name == name)
        .ok_or(malformed_err("w607_branch_slack_rank_name"))?;
    let local = cut
        .support
        .iter()
        .copied()
        .filter(|v| active.contains(v))
        .collect::<Vec<_>>();
    let used = cut
        .support
        .iter()
        .filter(|v| included.contains(v))
        .map(|v| weights[*v])
        .sum::<i128>();
    if vertices != local
        || row.rhs != cut.alpha_w - used
        || row.full_support_size != Some(cut.support.len())
    {
        return malformed("w607_branch_slack_rank_row");
    }
    for vertex in vertices {
        coverage[*vertex] += row.numerator * weights[*vertex];
    }
    Ok(())
}

fn modified_coefficients(c0: &[i128], p: &BTreeMap<usize, i128>) -> Vec<i128> {
    let mut out = c0
        .iter()
        .map(|value| value * DENOMINATOR)
        .collect::<Vec<_>>();
    for (vertex, num) in p {
        out[*vertex] += *num;
    }
    out
}

fn payload(rows: usize, conclusion: &str) -> Vec<HadwigerArtifactPayloadEntry> {
    vec![
        HadwigerArtifactPayloadEntry::text(
            "schema",
            "forge.hadwiger.w607_branch_slack_lift_replay.v1",
        ),
        HadwigerArtifactPayloadEntry::unsigned("gamma0_modified_num", GAMMA0_MOD_NUM as u128),
        HadwigerArtifactPayloadEntry::unsigned("gamma1_modified_num", GAMMA1_MOD_NUM as u128),
        HadwigerArtifactPayloadEntry::unsigned("lift_num", LIFT_NUM as u128),
        HadwigerArtifactPayloadEntry::unsigned("modified_gamma1_rows", rows as u128),
        HadwigerArtifactPayloadEntry::text("conclusion", conclusion),
    ]
}

fn malformed<T>(source: &'static str) -> Result<T, G27GeometricFractionalError> {
    Err(malformed_err(source))
}

fn malformed_err(source: &'static str) -> G27GeometricFractionalError {
    G27GeometricFractionalError::MalformedData { source }
}
