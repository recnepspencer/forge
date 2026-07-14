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
use super::g27_w_circles_exact_geometry_support::{parse_w_integer_weights, EXPECTED_VERTEX_COUNT};
use super::g27_w_circles_full_terminal_export_support::{
    replay_terminal, FullTerminalArtifact, Rational, Terminal,
};

const CERT: &str = include_str!("../../docs/w607-full-terminal-export-preflight.json");
const EXPECTED_SCHEMA: &str = "forge.hadwiger.w607_full_terminal_export_preflight.v1";
const EXPECTED_AUTHORITY: &str =
    "full_terminal_export_preflight_only_no_root_or_semantic_partition_authority";
const EXPECTED_STATUS: &str = "fund_full_mixed_tree_terminal_replay_checker";
const EXPECTED_FRESH_DIGEST: &str =
    "8c2f68061175a81b575fc80b06f5e64392a7624aa466e2b199e30826f474a189";
const EXPECTED_FIRST_FAMILY_DIGEST: &str =
    "2ce678706ac4740bb1ae5a9733f2dc12151ecf576358e399230571ffb895aea8";
const EXPECTED_GRAPH_DIGEST: &str =
    "5ee6b6ce564e2034a89df8ba5cb0a2103d3fcd3a5aacea72b13445c456f57683";
const EXPECTED_ROOT_ROWS_DIGEST: &str =
    "89e4c264528c0c2bc2838c855c62b2577f68c92a30c653a9ebcb093555a7398f";
const EXPECTED_PARENT_ROWS_DIGEST: &str =
    "2d7e16c7d9266117b4395ebb68e337d511cc4ca7344ad7c3081dfef855451378";
const EXPECTED_TERMINALS: usize = 135;
const EXPECTED_TOTAL_ROWS: usize = 80_143;
const TOTAL_ROW_BUDGET: usize = 100_000;
const EXPORT_GATE: i64 = 586_500;
const ALLOWANCE: i64 = 300;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27WCirclesFullTerminalExportReplayStatus {
    ReplayedFullTerminalExportPreflight,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27WCirclesFullTerminalExportReplayReport {
    core: HadwigerArtifactCore,
    terminal_count: usize,
    total_positive_rows: usize,
    worst_objective_floor: i128,
    min_slack_floor: i128,
    status: G27WCirclesFullTerminalExportReplayStatus,
    conclusion: String,
}

impl G27WCirclesFullTerminalExportReplayReport {
    pub fn summary(&self) -> (usize, usize, i128, i128) {
        (
            self.terminal_count,
            self.total_positive_rows,
            self.worst_objective_floor,
            self.min_slack_floor,
        )
    }

    pub fn status(&self) -> G27WCirclesFullTerminalExportReplayStatus {
        self.status
    }

    pub fn conclusion(&self) -> &str {
        &self.conclusion
    }
}

impl_hadwiger_artifact!(G27WCirclesFullTerminalExportReplayReport, core);

pub fn replay_g27_w_circles_full_terminal_export_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27WCirclesFullTerminalExportReplayReport, G27GeometricFractionalError> {
    let source = audit_g27_w_circles_607_finite_fractional_core_checked(handle)?;
    let weights = parse_w_integer_weights()?;
    if weights.len() != EXPECTED_VERTEX_COUNT {
        return malformed("w607_full_terminal_weight_shape");
    }
    let artifact: FullTerminalArtifact =
        serde_json::from_str(CERT).map_err(|_| malformed_err("w607_full_terminal_json"))?;
    verify_header(&artifact)?;
    verify_manifest(&artifact)?;
    let mut total_rows = 0_usize;
    let mut worst_objective = Rational::zero();
    let mut min_slack: Option<Rational> = None;
    let mut argmax = None;
    for terminal in &artifact.terminals {
        verify_terminal_shape(terminal)?;
        let replay = replay_terminal(terminal, &weights, EXPECTED_VERTEX_COUNT)?;
        if replay.objective.gt_i64(EXPORT_GATE)
            || replay
                .objective
                .gt_i64(terminal.expected_bound_floor() + ALLOWANCE)
        {
            return malformed("w607_full_terminal_objective_gate");
        }
        if replay.objective > worst_objective {
            worst_objective = replay.objective.clone();
            argmax = Some((terminal.leaf_index, terminal.terminal_id.as_str()));
        }
        min_slack = Some(match min_slack {
            Some(current) => current.min(replay.min_slack),
            None => replay.min_slack,
        });
        total_rows += replay.rows;
    }
    if total_rows != EXPECTED_TOTAL_ROWS
        || total_rows > TOTAL_ROW_BUDGET
        || argmax != Some((1, "depth3_terminal_0"))
    {
        return malformed("w607_full_terminal_aggregate");
    }
    report(
        source.reference(),
        total_rows,
        worst_objective.floor_i128(),
        min_slack.unwrap_or_else(Rational::zero).floor_i128(),
    )
}

fn verify_header(artifact: &FullTerminalArtifact) -> Result<(), G27GeometricFractionalError> {
    let binding = &artifact.source_binding;
    if artifact.schema != EXPECTED_SCHEMA
        || artifact.authority != EXPECTED_AUTHORITY
        || artifact.status != EXPECTED_STATUS
        || !artifact.failure_reasons.is_empty()
        || binding.fresh_replay_digest != EXPECTED_FRESH_DIGEST
        || binding.first_family_digest != EXPECTED_FIRST_FAMILY_DIGEST
        || binding.graph_digest != EXPECTED_GRAPH_DIGEST
        || binding.root_rows_digest != EXPECTED_ROOT_ROWS_DIGEST
        || binding.parent_rows_digest != EXPECTED_PARENT_ROWS_DIGEST
        || artifact.summary.successes != EXPECTED_TERMINALS
        || artifact.summary.total != EXPECTED_TERMINALS
        || artifact.summary.total_success_rows != EXPECTED_TOTAL_ROWS
    {
        return malformed("w607_full_terminal_header");
    }
    Ok(())
}

fn verify_manifest(artifact: &FullTerminalArtifact) -> Result<(), G27GeometricFractionalError> {
    if artifact.manifest.expected_terminal_count != EXPECTED_TERMINALS
        || artifact.manifest.actual_terminal_count != EXPECTED_TERMINALS
        || !artifact.manifest.duplicate_keys.is_empty()
        || artifact.manifest.replaced_triggered_leaf0_terminals.len() != 1
    {
        return malformed("w607_full_terminal_manifest_header");
    }
    let replaced = &artifact.manifest.replaced_triggered_leaf0_terminals[0];
    if replaced.leaf_index != 0
        || replaced.terminal_id != "depth4_terminal_0"
        || replaced.replaced_by != 4
    {
        return malformed("w607_full_terminal_leaf0_replacement");
    }
    let mut keys = BTreeSet::new();
    let mut class_counts = BTreeMap::<&str, usize>::new();
    for terminal in &artifact.terminals {
        *class_counts
            .entry(terminal.mechanism_class.as_str())
            .or_default() += 1;
        let key = format!(
            "{}:{}:{:?}:{:?}",
            terminal.leaf_index,
            terminal.depth,
            terminal.pool_assignment,
            terminal.residual_pair_assignment
        );
        if !keys.insert(key) {
            return malformed("w607_full_terminal_duplicate_key");
        }
    }
    require_class(&class_counts, "high_parent_lift_depth3", 6)?;
    require_class(&class_counts, "ordinary_non_leaf0_depth3_compact", 114)?;
    require_class(&class_counts, "leaf0_residual_child", 4)?;
    require_class(&class_counts, "ordinary_leaf0_closed", 11)?;
    Ok(())
}

fn verify_terminal_shape(terminal: &Terminal) -> Result<(), G27GeometricFractionalError> {
    if !terminal.export_required
        || terminal.status != "export_success"
        || !terminal.failure_reasons.is_empty()
    {
        return malformed("w607_full_terminal_status");
    }
    let strategy = terminal.certificate.strategy.as_str();
    let expected = match terminal.mechanism_class.as_str() {
        "high_parent_lift_depth3" => "per_row_rational_reconstruction",
        "ordinary_non_leaf0_depth3_compact" | "leaf0_residual_child" | "ordinary_leaf0_closed" => {
            "common_denominator_upward_integer"
        }
        _ => return malformed("w607_full_terminal_mechanism"),
    };
    if strategy != expected || terminal.selected_strategy != expected {
        return malformed("w607_full_terminal_strategy");
    }
    Ok(())
}

fn report(
    source: crate::domain_artifacts::HadwigerArtifactReference,
    total_rows: usize,
    worst_objective_floor: i128,
    min_slack_floor: i128,
) -> Result<G27WCirclesFullTerminalExportReplayReport, G27GeometricFractionalError> {
    let conclusion = format!(
        "replayed H25 full-terminal export preflight: {EXPECTED_TERMINALS} terminals, {total_rows} positive rows, worst objective floor {worst_objective_floor}; terminal authority only"
    );
    let core = artifact_core(
        HadwigerArtifactKind::G27WCirclesFullTerminalExportReplayReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_w_circles_full_terminal_export_replay".to_string(),
        },
        vec![source],
        vec![
            HadwigerArtifactPayloadEntry::text(
                "schema",
                "forge.hadwiger.w607_full_terminal_export_replay.v1",
            ),
            HadwigerArtifactPayloadEntry::unsigned("terminal_count", EXPECTED_TERMINALS as u128),
            HadwigerArtifactPayloadEntry::unsigned("total_positive_rows", total_rows as u128),
            HadwigerArtifactPayloadEntry::unsigned(
                "worst_objective_floor",
                worst_objective_floor as u128,
            ),
            HadwigerArtifactPayloadEntry::unsigned("min_slack_floor", min_slack_floor as u128),
            HadwigerArtifactPayloadEntry::text("authority", EXPECTED_AUTHORITY),
            HadwigerArtifactPayloadEntry::text("conclusion", &conclusion),
        ],
    )?;
    Ok(G27WCirclesFullTerminalExportReplayReport {
        core,
        terminal_count: EXPECTED_TERMINALS,
        total_positive_rows: total_rows,
        worst_objective_floor,
        min_slack_floor,
        status: G27WCirclesFullTerminalExportReplayStatus::ReplayedFullTerminalExportPreflight,
        conclusion,
    })
}

fn require_class(
    counts: &BTreeMap<&str, usize>,
    name: &'static str,
    expected: usize,
) -> Result<(), G27GeometricFractionalError> {
    if counts.get(name).copied().unwrap_or_default() != expected {
        return malformed(name);
    }
    Ok(())
}

fn malformed<T>(source: &'static str) -> Result<T, G27GeometricFractionalError> {
    Err(malformed_err(source))
}

fn malformed_err(source: &'static str) -> G27GeometricFractionalError {
    G27GeometricFractionalError::MalformedData { source }
}
