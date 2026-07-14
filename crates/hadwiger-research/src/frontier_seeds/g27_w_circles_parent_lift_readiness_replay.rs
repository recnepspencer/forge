use std::collections::BTreeMap;

use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::query_entry::HadwigerResearchHandle;

use serde::Serialize;
use sha2::{Digest, Sha256};

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_w_circles_branch_slack_lift_replay::replay_g27_w_circles_branch_slack_lift_checked;
use super::g27_w_circles_branch_slack_support::{coefficient_map, BranchSlackArtifact};
use super::g27_w_circles_exact_geometry_support::parse_w_integer_weights;
use super::g27_w_circles_full_terminal_export_support::{FullTerminalArtifact, ProofRow};
use super::g27_w_circles_gamma0_leaf_dual_support::{recompute_c0, BRANCH_VERTEX};
use super::g27_w_circles_projected_parent_lift_replay::replay_g27_w_circles_projected_parent_lift_checked;
use super::g27_w_circles_row_family_semantics_replay::replay_g27_w_circles_row_family_semantics_checked;

const TERMINALS: &str = include_str!("../../docs/w607-full-terminal-export-preflight.json");
const FRESH: &str = include_str!("../../docs/w607-fresh-mixed-branch-replay.json");
const FIRST_FAMILY: &str = include_str!("../../docs/w607-full-tree-rank-family.json");
const EXCLUDE_CERT: &str = include_str!("../../docs/w607-v304-exclude-dual-cover-den1024.json");
const GAMMA0: &str = include_str!("../../docs/w607-gamma0-leaf-dual-export.json");
const GAMMA1: &str = include_str!("../../docs/w607-gamma1-leaf-dual-export.json");
const BRANCH_SLACK: &str = include_str!("../../docs/w607-branch-slack-parent-lift-diagnostic.json");
const PROJECTED_PARENT_RHS: i64 = 613_372_392;
const PROJECTED_PARENT_LIFT: i64 = 67_286_586;
const BRANCH_SLACK_RHS: i64 = 623_894_447_014;
const BRANCH_SLACK_LIFT: i64 = 64_809_127_989;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27WCirclesParentLiftReadinessReplayStatus {
    ParentLiftProvenanceReadinessPreflight,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27WCirclesParentLiftReadinessReplayReport {
    core: HadwigerArtifactCore,
    checked_rows: usize,
    parent_lift_rows: usize,
    parent_lift_ids: Vec<String>,
    status: G27WCirclesParentLiftReadinessReplayStatus,
    theorem_authority: bool,
    conclusion: String,
}

impl G27WCirclesParentLiftReadinessReplayReport {
    pub fn summary(&self) -> (usize, usize, bool) {
        (
            self.checked_rows,
            self.parent_lift_rows,
            self.theorem_authority,
        )
    }

    pub fn parent_lift_ids(&self) -> &[String] {
        &self.parent_lift_ids
    }

    pub fn status(&self) -> G27WCirclesParentLiftReadinessReplayStatus {
        self.status
    }

    pub fn conclusion(&self) -> &str {
        &self.conclusion
    }
}

impl_hadwiger_artifact!(G27WCirclesParentLiftReadinessReplayReport, core);

pub fn replay_g27_w_circles_parent_lift_readiness_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27WCirclesParentLiftReadinessReplayReport, G27GeometricFractionalError> {
    let row_family = replay_g27_w_circles_row_family_semantics_checked(handle)?;
    let projected = replay_g27_w_circles_projected_parent_lift_checked(handle)?;
    let branch_slack = replay_g27_w_circles_branch_slack_lift_checked(handle)?;
    if row_family.summary() != (80_143, 6) {
        return malformed("w607_readiness_row_family_summary");
    }
    if projected.summary() != (304, 613_372_392, 546_085_806, 67_286_586) {
        return malformed("w607_readiness_projected_summary");
    }
    if branch_slack.summary() != (623_894_447_014, 559_085_319_025, 64_809_127_989, 8_555) {
        return malformed("w607_readiness_branch_slack_summary");
    }
    let canonical = canonical_parent_rows()?;
    let artifact: FullTerminalArtifact =
        serde_json::from_str(TERMINALS).map_err(|_| malformed_err("w607_readiness_json"))?;
    let mut seen = BTreeMap::<String, BTreeMap<usize, i64>>::new();
    let mut count = 0_usize;
    for terminal in &artifact.terminals {
        for row in terminal
            .certificate
            .positive_rows
            .iter()
            .filter(|row| row.family == "parent_lifts")
        {
            count += 1;
            verify_parent_row(row, &canonical, &mut seen)?;
        }
    }
    if count != 6 || seen.keys().cloned().collect::<Vec<_>>() != ["parent_lift_1"] {
        return malformed("w607_readiness_parent_ids");
    }
    report(
        row_family,
        projected,
        branch_slack,
        count,
        seen.keys().cloned().collect(),
    )
}

fn verify_parent_row(
    row: &ProofRow,
    canonical: &BTreeMap<String, (i64, BTreeMap<usize, i64>)>,
    seen: &mut BTreeMap<String, BTreeMap<usize, i64>>,
) -> Result<(), G27GeometricFractionalError> {
    let Some((rhs, coeffs)) = canonical.get(&row.id) else {
        return malformed("w607_readiness_parent_id");
    };
    let row_coeffs = coefficient_map_row(row)?;
    if row.rhs != *rhs || row_coeffs != *coeffs {
        return malformed("w607_readiness_parent_coeff");
    }
    if seen
        .insert(row.id.clone(), row_coeffs)
        .is_some_and(|old| old != *coeffs)
    {
        return malformed("w607_readiness_parent_duplicate");
    }
    Ok(())
}

fn canonical_parent_rows(
) -> Result<BTreeMap<String, (i64, BTreeMap<usize, i64>)>, G27GeometricFractionalError> {
    let weights = parse_w_integer_weights()?;
    let c0 = recompute_c0(EXCLUDE_CERT, &weights)?;
    let mut projected = nonzero_map(c0.iter().copied());
    projected.insert(BRANCH_VERTEX + 1, PROJECTED_PARENT_LIFT);
    let branch_slack: BranchSlackArtifact = serde_json::from_str(BRANCH_SLACK)
        .map_err(|_| malformed_err("w607_readiness_branch_slack"))?;
    let p = coefficient_map(&branch_slack)?;
    let modified = nonzero_map(
        c0.iter()
            .enumerate()
            .map(|(index, coeff)| *coeff * 1024 + p.get(&index).copied().unwrap_or_default()),
    );
    let mut modified = modified;
    modified.insert(BRANCH_VERTEX + 1, BRANCH_SLACK_LIFT);
    Ok(BTreeMap::from([
        (
            "parent_lift_0".to_string(),
            (PROJECTED_PARENT_RHS, projected),
        ),
        ("parent_lift_1".to_string(), (BRANCH_SLACK_RHS, modified)),
    ]))
}

fn nonzero_map(values: impl Iterator<Item = i128>) -> BTreeMap<usize, i64> {
    values
        .enumerate()
        .filter_map(|(index, value)| (value != 0).then_some((index + 1, value as i64)))
        .collect()
}

fn coefficient_map_row(
    row: &ProofRow,
) -> Result<BTreeMap<usize, i64>, G27GeometricFractionalError> {
    let mut out = BTreeMap::new();
    for (vertex, coeff) in &row.coefficients {
        if out.insert(*vertex, *coeff).is_some() {
            return malformed("w607_readiness_duplicate_coeff");
        }
    }
    Ok(out)
}

fn report(
    row_family: super::g27_w_circles_row_family_semantics_replay::G27WCirclesRowFamilySemanticsReplayReport,
    projected: super::g27_w_circles_projected_parent_lift_replay::G27WCirclesProjectedParentLiftReplayReport,
    branch_slack: super::g27_w_circles_branch_slack_lift_replay::G27WCirclesBranchSlackLiftReplayReport,
    parent_lift_rows: usize,
    parent_lift_ids: Vec<String>,
) -> Result<G27WCirclesParentLiftReadinessReplayReport, G27GeometricFractionalError> {
    let (checked_rows, _) = row_family.summary();
    let digests = digest_report();
    let conclusion = format!(
        "validated parent-lift provenance for {parent_lift_rows} terminal row occurrences across {:?}; proof-plumbing readiness only, not root theorem authority",
        parent_lift_ids
    );
    let payload_json =
        serde_json::to_string(&digests).map_err(|_| malformed_err("w607_readiness_payload"))?;
    let core = artifact_core(
        HadwigerArtifactKind::G27WCirclesParentLiftReadinessReplayReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_w_circles_parent_lift_readiness_replay".to_string(),
        },
        vec![
            row_family.reference(),
            projected.reference(),
            branch_slack.reference(),
        ],
        vec![
            HadwigerArtifactPayloadEntry::text(
                "schema",
                "forge.hadwiger.w607_parent_lift_readiness_replay.v1",
            ),
            HadwigerArtifactPayloadEntry::unsigned("checked_rows", checked_rows as u128),
            HadwigerArtifactPayloadEntry::unsigned("parent_lift_rows", parent_lift_rows as u128),
            HadwigerArtifactPayloadEntry::text("parent_lift_ids", &parent_lift_ids.join(",")),
            HadwigerArtifactPayloadEntry::text("source_sha256_json", &payload_json),
            HadwigerArtifactPayloadEntry::text("theorem_authority", "false"),
            HadwigerArtifactPayloadEntry::text("conclusion", &conclusion),
        ],
    )?;
    Ok(G27WCirclesParentLiftReadinessReplayReport {
        core,
        checked_rows,
        parent_lift_rows,
        parent_lift_ids,
        status: G27WCirclesParentLiftReadinessReplayStatus::ParentLiftProvenanceReadinessPreflight,
        theorem_authority: false,
        conclusion,
    })
}

#[derive(Serialize)]
struct SourceDigests {
    terminal_rows: String,
    fresh_mixed_replay: String,
    first_family: String,
    exclude_certificate: String,
    gamma0_leaf_dual: String,
    gamma1_leaf_dual: String,
    branch_slack_lift: String,
}

fn digest_report() -> SourceDigests {
    SourceDigests {
        terminal_rows: digest(TERMINALS),
        fresh_mixed_replay: digest(FRESH),
        first_family: digest(FIRST_FAMILY),
        exclude_certificate: digest(EXCLUDE_CERT),
        gamma0_leaf_dual: digest(GAMMA0),
        gamma1_leaf_dual: digest(GAMMA1),
        branch_slack_lift: digest(BRANCH_SLACK),
    }
}

fn digest(text: &str) -> String {
    format!("{:x}", Sha256::digest(text.as_bytes()))
}

fn malformed<T>(source: &'static str) -> Result<T, G27GeometricFractionalError> {
    Err(malformed_err(source))
}

fn malformed_err(source: &'static str) -> G27GeometricFractionalError {
    G27GeometricFractionalError::MalformedData { source }
}
