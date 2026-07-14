use std::collections::{BTreeMap, BTreeSet};

use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::query_entry::HadwigerResearchHandle;

use sha2::{Digest, Sha256};

use super::g27_finite_fractional_core_audit::audit_g27_w_circles_607_finite_fractional_core_checked;
use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_w_circles_full_terminal_export_replay::replay_g27_w_circles_full_terminal_export_checked;
use super::g27_w_circles_full_terminal_export_support::{FreshLeaf, FreshReplay};
use super::g27_w_circles_full_terminal_export_support::{FullTerminalArtifact, Terminal};

const FRESH: &str = include_str!("../../docs/w607-fresh-mixed-branch-replay.json");
const TERMINALS: &str = include_str!("../../docs/w607-full-terminal-export-preflight.json");
const FRESH_RAW_SHA256: &str = "4a4fad2119dc3776ae431580e9afc3e6bfb2d462a2d151cdab5f61c043a25780";
const TERMINAL_RAW_SHA256: &str =
    "64a137144944f345073ac3294f967e78576b3c91ad5c8c222ac3ba7304068a4c";
const EXPECTED_FRESH_CANONICAL: &str =
    "8c2f68061175a81b575fc80b06f5e64392a7624aa466e2b199e30826f474a189";
const STATUS: &str = "semantic_partition_terminal_composition_preflight";
const TIER_A: [usize; 6] = [223, 224, 303, 305, 384, 385];
const POOL: [usize; 6] = [152, 222, 225, 383, 386, 456];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27WCirclesSemanticPartitionReplayStatus {
    SemanticPartitionTerminalCompositionPreflight,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27WCirclesSemanticPartitionReplayReport {
    core: HadwigerArtifactCore,
    tier_assignments: usize,
    terminal_count: usize,
    total_positive_rows: usize,
    status: G27WCirclesSemanticPartitionReplayStatus,
    conclusion: String,
}

impl G27WCirclesSemanticPartitionReplayReport {
    pub fn summary(&self) -> (usize, usize, usize) {
        (
            self.tier_assignments,
            self.terminal_count,
            self.total_positive_rows,
        )
    }

    pub fn status(&self) -> G27WCirclesSemanticPartitionReplayStatus {
        self.status
    }

    pub fn conclusion(&self) -> &str {
        &self.conclusion
    }
}

impl_hadwiger_artifact!(G27WCirclesSemanticPartitionReplayReport, core);

pub fn replay_g27_w_circles_semantic_partition_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27WCirclesSemanticPartitionReplayReport, G27GeometricFractionalError> {
    require_sha(FRESH, FRESH_RAW_SHA256, "w607_semantic_fresh_digest")?;
    require_sha(
        TERMINALS,
        TERMINAL_RAW_SHA256,
        "w607_semantic_terminal_digest",
    )?;
    let source = audit_g27_w_circles_607_finite_fractional_core_checked(handle)?;
    let terminal_replay = replay_g27_w_circles_full_terminal_export_checked(handle)?;
    let (_terminals, rows, _worst, _slack) = terminal_replay.summary();
    let fresh: FreshReplay =
        serde_json::from_str(FRESH).map_err(|_| malformed_err("w607_semantic_fresh_json"))?;
    let terminal_artifact: FullTerminalArtifact = serde_json::from_str(TERMINALS)
        .map_err(|_| malformed_err("w607_semantic_terminal_json"))?;
    verify_headers(&fresh, &terminal_artifact)?;
    verify_tier_partition(&fresh.leaves)?;
    let by_leaf = terminals_by_leaf(&terminal_artifact.terminals);
    for leaf in &fresh.leaves {
        let exports = by_leaf
            .get(&leaf.leaf_index)
            .ok_or(malformed_err("w607_semantic_missing_leaf_exports"))?;
        if leaf.leaf_index == 0 {
            verify_leaf0(leaf, exports)?;
        } else {
            verify_non_leaf(leaf, exports)?;
        }
    }
    report(source.reference(), rows)
}

fn verify_headers(
    fresh: &FreshReplay,
    terminal_artifact: &FullTerminalArtifact,
) -> Result<(), G27GeometricFractionalError> {
    if fresh.schema != "forge.hadwiger.w607_fresh_mixed_branch_replay.v1"
        || fresh.authority_label != "fresh_replay_diagnostic_branch_authority_only"
        || fresh.status != "fund_export_lift_design"
        || fresh.leaf_count != 16
        || !fresh.failure_reasons.is_empty()
        || terminal_artifact.source_binding.fresh_replay_digest != EXPECTED_FRESH_CANONICAL
        || terminal_artifact.terminals.len() != 135
    {
        return malformed("w607_semantic_header");
    }
    Ok(())
}

fn verify_tier_partition(leaves: &[FreshLeaf]) -> Result<(), G27GeometricFractionalError> {
    if leaves.len() != 16 {
        return malformed("w607_semantic_tier_leaf_count");
    }
    let clauses = leaves
        .iter()
        .map(|leaf| {
            (
                leaf.leaf_index,
                clause(
                    &leaf.tier_a_assignment.included,
                    &leaf.tier_a_assignment.excluded,
                ),
            )
        })
        .collect::<Vec<_>>();
    verify_cube_partition(&TIER_A, &clauses, "w607_semantic_tier_partition")
}

fn verify_non_leaf(
    leaf: &FreshLeaf,
    exports: &[&Terminal],
) -> Result<(), G27GeometricFractionalError> {
    if leaf.exceptional_rule != "none"
        || leaf.terminal_certificates.len() != 8
        || exports.len() != 8
    {
        return malformed("w607_semantic_nonleaf_shape");
    }
    verify_terminal_match(leaf, exports, false)?;
    let clauses = leaf
        .terminal_certificates
        .iter()
        .enumerate()
        .map(|(index, terminal)| (index, pool_clause(&terminal.pool_assignment)))
        .collect::<Vec<_>>();
    verify_cube_partition(&POOL, &clauses, "w607_semantic_pool_partition")
}

fn verify_leaf0(
    leaf: &FreshLeaf,
    exports: &[&Terminal],
) -> Result<(), G27GeometricFractionalError> {
    if leaf.exceptional_rule != "leaf0_depth4_residual_pair_closure"
        || leaf.residual_closures.len() != 12
        || exports.len() != 15
    {
        return malformed("w607_semantic_leaf0_shape");
    }
    let triggered = leaf
        .residual_closures
        .iter()
        .filter(|closure| closure.triggered)
        .collect::<Vec<_>>();
    if triggered.len() != 1 || triggered[0].children.len() != 4 {
        return malformed("w607_semantic_leaf0_trigger");
    }
    verify_terminal_match(leaf, exports, true)?;
    let mut clauses = Vec::new();
    for (index, closure) in leaf.residual_closures.iter().enumerate() {
        if closure.triggered {
            for child in &closure.children {
                clauses.push((index, pool_clause(&child.pool_assignment)));
            }
        } else {
            clauses.push((index, pool_clause(&closure.terminal.pool_assignment)));
        }
    }
    verify_cube_partition(&POOL, &clauses, "w607_semantic_leaf0_partition")
}

fn verify_terminal_match(
    leaf: &FreshLeaf,
    exports: &[&Terminal],
    leaf0: bool,
) -> Result<(), G27GeometricFractionalError> {
    let expected = expected_terminal_keys(leaf, leaf0)?;
    let actual = exports
        .iter()
        .map(|terminal| assignment_key(&terminal.pool_assignment))
        .collect::<BTreeSet<_>>();
    if expected != actual {
        return malformed("w607_semantic_terminal_match");
    }
    Ok(())
}

fn expected_terminal_keys(
    leaf: &FreshLeaf,
    leaf0: bool,
) -> Result<BTreeSet<String>, G27GeometricFractionalError> {
    let mut keys = BTreeSet::new();
    if leaf0 {
        for closure in &leaf.residual_closures {
            if closure.triggered {
                for child in &closure.children {
                    keys.insert(assignment_key(&child.pool_assignment));
                }
            } else {
                keys.insert(assignment_key(&closure.terminal.pool_assignment));
            }
        }
    } else {
        for terminal in &leaf.terminal_certificates {
            keys.insert(assignment_key(&terminal.pool_assignment));
        }
    }
    Ok(keys)
}

fn verify_cube_partition(
    universe: &[usize],
    clauses: &[(usize, BTreeMap<usize, bool>)],
    error: &'static str,
) -> Result<(), G27GeometricFractionalError> {
    let mut reachable = BTreeSet::new();
    for mask in 0..(1_usize << universe.len()) {
        let assignment = universe
            .iter()
            .enumerate()
            .map(|(index, vertex)| (*vertex, ((mask >> index) & 1) == 1))
            .collect::<BTreeMap<_, _>>();
        let hits = clauses
            .iter()
            .filter(|(_, clause)| clause_matches(clause, &assignment))
            .map(|(id, _)| *id)
            .collect::<Vec<_>>();
        if hits.len() != 1 {
            return malformed(error);
        }
        reachable.insert(hits[0]);
    }
    if reachable.len()
        != clauses
            .iter()
            .map(|(id, _)| *id)
            .collect::<BTreeSet<_>>()
            .len()
    {
        return malformed(error);
    }
    Ok(())
}

fn clause_matches(clause: &BTreeMap<usize, bool>, assignment: &BTreeMap<usize, bool>) -> bool {
    clause
        .iter()
        .all(|(vertex, value)| assignment.get(vertex) == Some(value))
}

fn clause(included: &[usize], excluded: &[usize]) -> BTreeMap<usize, bool> {
    let mut out = BTreeMap::new();
    for vertex in included {
        out.insert(*vertex, true);
    }
    for vertex in excluded {
        out.insert(*vertex, false);
    }
    out
}

fn pool_clause(value: &serde_json::Value) -> BTreeMap<usize, bool> {
    value
        .as_object()
        .into_iter()
        .flat_map(|map| map.iter())
        .map(|(key, value)| {
            (
                key.parse::<usize>().expect("vertex key parses"),
                value.as_f64().unwrap_or_default() > 0.5,
            )
        })
        .collect()
}

fn terminals_by_leaf(terminals: &[Terminal]) -> BTreeMap<usize, Vec<&Terminal>> {
    let mut out = BTreeMap::<usize, Vec<&Terminal>>::new();
    for terminal in terminals {
        out.entry(terminal.leaf_index).or_default().push(terminal);
    }
    out
}

fn assignment_key(value: &serde_json::Value) -> String {
    let clause = pool_clause(value);
    POOL.iter()
        .filter_map(|vertex| clause.get(vertex).map(|state| format!("{vertex}={state}")))
        .collect::<Vec<_>>()
        .join(",")
}

fn require_sha(
    text: &str,
    expected: &str,
    error: &'static str,
) -> Result<(), G27GeometricFractionalError> {
    let actual = format!("{:x}", Sha256::digest(text.as_bytes()));
    if actual != expected {
        return malformed(error);
    }
    Ok(())
}

fn report(
    source: crate::domain_artifacts::HadwigerArtifactReference,
    rows: usize,
) -> Result<G27WCirclesSemanticPartitionReplayReport, G27GeometricFractionalError> {
    let conclusion = format!(
        "semantic branch partition composes with H26 terminal replay: 64 Tier-A assignments, 135 terminals, {rows} positive rows; not root theorem authority"
    );
    let core = artifact_core(
        HadwigerArtifactKind::G27WCirclesSemanticPartitionReplayReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_w_circles_semantic_partition_replay".to_string(),
        },
        vec![source],
        vec![
            HadwigerArtifactPayloadEntry::text(
                "schema",
                "forge.hadwiger.w607_semantic_partition_replay.v1",
            ),
            HadwigerArtifactPayloadEntry::text("status", STATUS),
            HadwigerArtifactPayloadEntry::unsigned("tier_assignments", 64),
            HadwigerArtifactPayloadEntry::unsigned("terminal_count", 135),
            HadwigerArtifactPayloadEntry::unsigned("total_positive_rows", rows as u128),
            HadwigerArtifactPayloadEntry::text("conclusion", &conclusion),
        ],
    )?;
    Ok(G27WCirclesSemanticPartitionReplayReport {
        core,
        tier_assignments: 64,
        terminal_count: 135,
        total_positive_rows: rows,
        status:
            G27WCirclesSemanticPartitionReplayStatus::SemanticPartitionTerminalCompositionPreflight,
        conclusion,
    })
}

fn malformed<T>(source: &'static str) -> Result<T, G27GeometricFractionalError> {
    Err(malformed_err(source))
}

fn malformed_err(source: &'static str) -> G27GeometricFractionalError {
    G27GeometricFractionalError::MalformedData { source }
}
