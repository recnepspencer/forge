use std::collections::{BTreeMap, BTreeSet};

use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::query_entry::HadwigerResearchHandle;

use serde::Deserialize;
use sha2::{Digest, Sha256};

use super::g27_finite_fractional_core_audit::audit_g27_w_circles_607_finite_fractional_core_checked;
use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_w_circles_branch_slack_support::{coefficient_map, BranchSlackArtifact};
use super::g27_w_circles_exact_geometry_support::{
    parse_w_integer_weights, parse_w_retained_edges, EXPECTED_EDGE_COUNT, EXPECTED_VERTEX_COUNT,
};
use super::g27_w_circles_full_terminal_export_support::{
    FreshReplay, FullTerminalArtifact, ProofRow, Terminal,
};
use super::g27_w_circles_gamma0_leaf_dual_support::{recompute_c0, BRANCH_VERTEX};
use super::g27_w_circles_semantic_partition_replay::replay_g27_w_circles_semantic_partition_checked;

const TERMINALS: &str = include_str!("../../docs/w607-full-terminal-export-preflight.json");
const FRESH: &str = include_str!("../../docs/w607-fresh-mixed-branch-replay.json");
const FIRST_FAMILY: &str = include_str!("../../docs/w607-full-tree-rank-family.json");
const EXCLUDE_CERT: &str = include_str!("../../docs/w607-v304-exclude-dual-cover-den1024.json");
const BRANCH_SLACK: &str = include_str!("../../docs/w607-branch-slack-parent-lift-diagnostic.json");
const PROJECTED_PARENT_RHS: i64 = 613_372_392;
const PROJECTED_PARENT_LIFT: i64 = 67_286_586;
const BRANCH_SLACK_RHS: i64 = 623_894_447_014;
const BRANCH_SLACK_LIFT: i64 = 64_809_127_989;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27WCirclesRowFamilySemanticsReplayStatus {
    RowFamilySemanticsPreflight,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27WCirclesRowFamilySemanticsReplayReport {
    core: HadwigerArtifactCore,
    checked_rows: usize,
    parent_lift_rows: usize,
    status: G27WCirclesRowFamilySemanticsReplayStatus,
    conclusion: String,
}

impl G27WCirclesRowFamilySemanticsReplayReport {
    pub fn summary(&self) -> (usize, usize) {
        (self.checked_rows, self.parent_lift_rows)
    }

    pub fn status(&self) -> G27WCirclesRowFamilySemanticsReplayStatus {
        self.status
    }

    pub fn conclusion(&self) -> &str {
        &self.conclusion
    }
}

impl_hadwiger_artifact!(G27WCirclesRowFamilySemanticsReplayReport, core);

pub fn replay_g27_w_circles_row_family_semantics_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27WCirclesRowFamilySemanticsReplayReport, G27GeometricFractionalError> {
    let source = audit_g27_w_circles_607_finite_fractional_core_checked(handle)?;
    let semantic = replay_g27_w_circles_semantic_partition_checked(handle)?;
    let (_, terminals, _rows) = semantic.summary();
    if terminals != 135 {
        return malformed("w607_row_family_semantic_composition");
    }
    let weights = parse_w_integer_weights()?;
    let edges = parse_w_retained_edges(EXPECTED_VERTEX_COUNT)?;
    if weights.len() != EXPECTED_VERTEX_COUNT || edges.len() != EXPECTED_EDGE_COUNT {
        return malformed("w607_row_family_graph_shape");
    }
    let artifact: FullTerminalArtifact =
        serde_json::from_str(TERMINALS).map_err(|_| malformed_err("w607_row_family_json"))?;
    let fresh: FreshReplay =
        serde_json::from_str(FRESH).map_err(|_| malformed_err("w607_row_family_fresh_json"))?;
    let first: FirstFamilySource = serde_json::from_str(FIRST_FAMILY)
        .map_err(|_| malformed_err("w607_row_family_first_json"))?;
    let first_rows = first_family_rows(&first);
    let parent_rows = parent_rows(&weights)?;
    let mut checked = 0_usize;
    let mut parent_count = 0_usize;
    for terminal in &artifact.terminals {
        let fixed = fixed_assignment(terminal, &fresh)?;
        for row in &terminal.certificate.positive_rows {
            checked += 1;
            match row.family.as_str() {
                "edges" => verify_edge(row, &edges)?,
                "triangles" => verify_triangle(row, &edges)?,
                "fixed_zero_literal" => verify_fixed(row, &fixed, false)?,
                "fixed_one_literal" => verify_fixed(row, &fixed, true)?,
                "first_family_leaf_rows" => {
                    verify_first_family(row, terminal, &weights, &first_rows)?
                }
                "parent_lifts" => {
                    verify_parent_lift(row, &parent_rows)?;
                    parent_count += 1;
                }
                _ => return malformed("w607_row_family_unknown"),
            }
        }
    }
    report(source.reference(), checked, parent_count)
}

fn verify_edge(
    row: &ProofRow,
    edges: &BTreeSet<(usize, usize)>,
) -> Result<(), G27GeometricFractionalError> {
    let vertices = unit_coeff_vertices(row, 2, 1)?;
    if row.rhs != 1 || !edges.contains(&edge_key(vertices[0], vertices[1])) {
        return malformed("w607_row_family_edge");
    }
    if row.id != format!("edge_{}_{}", vertices[0], vertices[1]) {
        return malformed("w607_row_family_edge_id");
    }
    Ok(())
}

fn verify_triangle(
    row: &ProofRow,
    edges: &BTreeSet<(usize, usize)>,
) -> Result<(), G27GeometricFractionalError> {
    let vertices = unit_coeff_vertices(row, 3, 1)?;
    let complete = [(0, 1), (0, 2), (1, 2)]
        .iter()
        .all(|(a, b)| edges.contains(&edge_key(vertices[*a], vertices[*b])));
    if row.rhs != 1 || !complete || !row.id.starts_with("triangle_") {
        return malformed("w607_row_family_triangle");
    }
    Ok(())
}

fn verify_fixed(
    row: &ProofRow,
    fixed: &BTreeMap<usize, bool>,
    one: bool,
) -> Result<(), G27GeometricFractionalError> {
    let expected_coeff = if one { -1 } else { 1 };
    let expected_rhs = if one { -1 } else { 0 };
    let vertices = unit_coeff_vertices(row, 1, expected_coeff)?;
    if row.rhs != expected_rhs || fixed.get(&vertices[0]) != Some(&one) {
        return malformed("w607_row_family_fixed");
    }
    let suffix = if one { "fixed_one" } else { "fixed_zero" };
    if row.id != format!("x{}_{}", vertices[0], suffix) {
        return malformed("w607_row_family_fixed_id");
    }
    Ok(())
}

fn verify_first_family(
    row: &ProofRow,
    terminal: &Terminal,
    weights: &[i128],
    first_rows: &BTreeSet<(usize, i64, String, usize)>,
) -> Result<(), G27GeometricFractionalError> {
    let coeffs = coefficient_map_row(row)?;
    if !row
        .id
        .starts_with(&format!("leaf{}_first_family_", terminal.leaf_index))
    {
        return malformed("w607_row_family_first_id");
    }
    for (vertex, coeff) in &coeffs {
        if *vertex == 0 || *vertex > weights.len() || *coeff != weights[*vertex - 1] as i64 {
            return malformed("w607_row_family_first_coeff");
        }
    }
    let digest = support_digest(coeffs.keys().copied().collect());
    let key = (terminal.leaf_index, row.rhs, digest, coeffs.len());
    if !first_rows.contains(&key) {
        return malformed("w607_row_family_first_source");
    }
    Ok(())
}

fn verify_parent_lift(
    row: &ProofRow,
    parent_rows: &BTreeMap<String, (i64, BTreeMap<usize, i64>)>,
) -> Result<(), G27GeometricFractionalError> {
    let Some((rhs, coeffs)) = parent_rows.get(&row.id) else {
        return malformed("w607_row_family_parent_id");
    };
    if row.rhs != *rhs || coefficient_map_row(row)? != *coeffs {
        return malformed("w607_row_family_parent_coeff");
    }
    Ok(())
}

fn parent_rows(
    weights: &[i128],
) -> Result<BTreeMap<String, (i64, BTreeMap<usize, i64>)>, G27GeometricFractionalError> {
    let c0 = recompute_c0(EXCLUDE_CERT, weights)?;
    let mut projected = BTreeMap::new();
    for (index, coeff) in c0.iter().enumerate() {
        if *coeff != 0 {
            projected.insert(index + 1, *coeff as i64);
        }
    }
    projected.insert(BRANCH_VERTEX + 1, PROJECTED_PARENT_LIFT);
    let branch_slack: BranchSlackArtifact = serde_json::from_str(BRANCH_SLACK)
        .map_err(|_| malformed_err("w607_row_family_branch_slack"))?;
    let p = coefficient_map(&branch_slack)?;
    let mut modified = BTreeMap::new();
    for (index, coeff) in c0.iter().enumerate() {
        let value = *coeff * 1024 + p.get(&index).copied().unwrap_or_default();
        if value != 0 {
            modified.insert(index + 1, value as i64);
        }
    }
    modified.insert(BRANCH_VERTEX + 1, BRANCH_SLACK_LIFT);
    Ok(BTreeMap::from([
        (
            "parent_lift_0".to_string(),
            (PROJECTED_PARENT_RHS, projected),
        ),
        ("parent_lift_1".to_string(), (BRANCH_SLACK_RHS, modified)),
    ]))
}

fn fixed_assignment(
    terminal: &Terminal,
    fresh: &FreshReplay,
) -> Result<BTreeMap<usize, bool>, G27GeometricFractionalError> {
    let leaf = fresh
        .leaves
        .iter()
        .find(|leaf| leaf.leaf_index == terminal.leaf_index)
        .ok_or(malformed_err("w607_row_family_leaf"))?;
    let mut fixed = BTreeMap::new();
    add_assignments(&mut fixed, &leaf.tier_a_assignment.included, true)?;
    add_assignments(&mut fixed, &leaf.tier_a_assignment.excluded, false)?;
    add_json_assignment(&mut fixed, &terminal.pool_assignment)?;
    Ok(fixed)
}

fn add_assignments(
    fixed: &mut BTreeMap<usize, bool>,
    vertices: &[usize],
    value: bool,
) -> Result<(), G27GeometricFractionalError> {
    for vertex in vertices {
        if fixed.insert(*vertex, value).is_some_and(|old| old != value) {
            return malformed("w607_row_family_fixed_conflict");
        }
    }
    Ok(())
}

fn add_json_assignment(
    fixed: &mut BTreeMap<usize, bool>,
    value: &serde_json::Value,
) -> Result<(), G27GeometricFractionalError> {
    for (vertex, state) in value.as_object().into_iter().flat_map(|map| map.iter()) {
        let vertex = vertex
            .parse::<usize>()
            .map_err(|_| malformed_err("w607_row_family_fixed_parse"))?;
        let state = state.as_f64().unwrap_or_default() > 0.5;
        if fixed.insert(vertex, state).is_some_and(|old| old != state) {
            return malformed("w607_row_family_fixed_conflict");
        }
    }
    Ok(())
}

fn unit_coeff_vertices(
    row: &ProofRow,
    len: usize,
    coeff: i64,
) -> Result<Vec<usize>, G27GeometricFractionalError> {
    if row.coefficients.len() != len || row.coefficients.iter().any(|(_, value)| *value != coeff) {
        return malformed("w607_row_family_unit_coeff");
    }
    let mut vertices = row
        .coefficients
        .iter()
        .map(|(vertex, _)| *vertex)
        .collect::<Vec<_>>();
    vertices.sort_unstable();
    vertices.dedup();
    if vertices.len() != len {
        return malformed("w607_row_family_unit_vertices");
    }
    Ok(vertices)
}

fn coefficient_map_row(
    row: &ProofRow,
) -> Result<BTreeMap<usize, i64>, G27GeometricFractionalError> {
    let mut out = BTreeMap::new();
    for (vertex, coeff) in &row.coefficients {
        if out.insert(*vertex, *coeff).is_some() {
            return malformed("w607_row_family_duplicate_coeff");
        }
    }
    Ok(out)
}

fn first_family_rows(source: &FirstFamilySource) -> BTreeSet<(usize, i64, String, usize)> {
    source
        .leaves
        .iter()
        .flat_map(|leaf| {
            leaf.accepted_rows.iter().map(|row| {
                (
                    leaf.leaf_index,
                    row.alpha_w,
                    row.support_digest.clone(),
                    row.size,
                )
            })
        })
        .collect()
}

fn support_digest(vertices: Vec<usize>) -> String {
    let payload = vertices
        .into_iter()
        .map(|vertex| vertex.to_string())
        .collect::<Vec<_>>()
        .join(",");
    format!("{:x}", Sha256::digest(payload.as_bytes()))
}

fn edge_key(left: usize, right: usize) -> (usize, usize) {
    if left < right {
        (left, right)
    } else {
        (right, left)
    }
}

fn report(
    source: crate::domain_artifacts::HadwigerArtifactReference,
    checked_rows: usize,
    parent_lift_rows: usize,
) -> Result<G27WCirclesRowFamilySemanticsReplayReport, G27GeometricFractionalError> {
    let conclusion = format!(
        "checked semantics for {checked_rows} exported rows, including {parent_lift_rows} parent-lift row occurrences; parent vectors matched existing digest-bound lift artifacts"
    );
    let core = artifact_core(
        HadwigerArtifactKind::G27WCirclesRowFamilySemanticsReplayReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_w_circles_row_family_semantics_replay".to_string(),
        },
        vec![source],
        vec![
            HadwigerArtifactPayloadEntry::text(
                "schema",
                "forge.hadwiger.w607_row_family_semantics_replay.v1",
            ),
            HadwigerArtifactPayloadEntry::unsigned("checked_rows", checked_rows as u128),
            HadwigerArtifactPayloadEntry::unsigned("parent_lift_rows", parent_lift_rows as u128),
            HadwigerArtifactPayloadEntry::text("conclusion", &conclusion),
        ],
    )?;
    Ok(G27WCirclesRowFamilySemanticsReplayReport {
        core,
        checked_rows,
        parent_lift_rows,
        status: G27WCirclesRowFamilySemanticsReplayStatus::RowFamilySemanticsPreflight,
        conclusion,
    })
}

#[derive(Deserialize)]
struct FirstFamilySource {
    leaves: Vec<FirstFamilyLeaf>,
}

#[derive(Deserialize)]
struct FirstFamilyLeaf {
    leaf_index: usize,
    accepted_rows: Vec<FirstFamilyRow>,
}

#[derive(Deserialize)]
struct FirstFamilyRow {
    support_digest: String,
    size: usize,
    alpha_w: i64,
}

fn malformed<T>(source: &'static str) -> Result<T, G27GeometricFractionalError> {
    Err(malformed_err(source))
}

fn malformed_err(source: &'static str) -> G27GeometricFractionalError {
    G27GeometricFractionalError::MalformedData { source }
}
