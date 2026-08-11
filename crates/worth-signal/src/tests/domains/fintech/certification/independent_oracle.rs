use std::collections::BTreeMap;

#[cfg(feature = "parallel")]
use std::collections::BTreeSet;

use worth_harness::facade::{InvariantCheck, InvariantReport};

use crate::facade::SignalError;

#[cfg(feature = "parallel")]
use worth_harness::facade::{
    ArtifactSurface, DifferentialOutcome, UnsupportedWorkflowComparison,
    WorkflowCertificationReport,
};

#[cfg(feature = "parallel")]
use crate::facade::{compare_lineage_records, compare_replay_slices};

use super::workflow_session::{
    CertifiedFintechWorkflowSession, SignalFintechWorkflowCertificationAdapter,
};

pub(super) fn check_invariant(
    session: &CertifiedFintechWorkflowSession,
    check: &InvariantCheck,
) -> Result<InvariantReport, SignalError> {
    let parts: Vec<_> = check.check_id.split(':').collect();
    let (passed, detail) = match parts.as_slice() {
        ["audit_eq", left, right] => {
            let left_value = session.named_audits.get(*left).ok_or_else(|| {
                SignalError::invalid_input(format!(
                    "unknown certified fintech audit alias `{left}`"
                ))
            })?;
            let right_value = session.named_audits.get(*right).ok_or_else(|| {
                SignalError::invalid_input(format!(
                    "unknown certified fintech audit alias `{right}`"
                ))
            })?;
            (
                left_value == right_value,
                format!("compare audit surfaces `{left}` and `{right}`"),
            )
        }
        ["replay_has_kind", alias, kind] => {
            let replay = session.replay(alias)?;
            let kind = SignalFintechWorkflowCertificationAdapter::parse_replay_kind(kind)?;
            (
                replay.frames.iter().any(|frame| frame.kind == kind),
                format!("replay `{alias}` should contain `{kind:?}`"),
            )
        }
        ["replay_branch_local", alias, branch_alias] => {
            let replay = session.replay(alias)?;
            let branch = session.branch(branch_alias)?;
            (
                replay
                    .frames
                    .iter()
                    .all(|frame| frame.branch_id == branch.id),
                format!("replay `{alias}` should remain local to branch `{branch_alias}`"),
            )
        }
        ["lineage_has_any", alias, events] => {
            let lineage = session.lineage(alias)?;
            let events = SignalFintechWorkflowCertificationAdapter::parse_lineage_events(events)?;
            (
                lineage
                    .iter()
                    .any(|record| events.iter().any(|event| event == record.label())),
                format!("lineage `{alias}` should contain one of `{events:?}`"),
            )
        }
        ["branch_head_matches_snapshot", branch_alias, snapshot_alias] => {
            let branch = session.branch(branch_alias)?;
            let snapshot = session.snapshot(snapshot_alias)?;
            (
                session.world.branch_head_snapshot_id(branch) == Some(snapshot.meta.snapshot_id),
                format!("branch `{branch_alias}` should keep head snapshot `{snapshot_alias}`"),
            )
        }
        ["replay_mentions_snapshot", alias, snapshot_alias] => {
            let replay = session.replay(alias)?;
            let snapshot = session.snapshot(snapshot_alias)?;
            (
                replay
                    .frames
                    .iter()
                    .any(|frame| frame.snapshot_id == Some(snapshot.meta.snapshot_id)),
                format!("replay `{alias}` should mention snapshot `{snapshot_alias}`"),
            )
        }
        _ => (
            false,
            format!(
                "unsupported certified fintech invariant `{}`",
                check.check_id
            ),
        ),
    };
    Ok(InvariantReport {
        check_id: check.check_id.clone(),
        boundary: check.boundary,
        passed,
        detail,
        fields: BTreeMap::new(),
    })
}

#[cfg(feature = "parallel")]
pub(super) fn compare_signal_fintech_overlap(
    left: &WorkflowCertificationReport<CertifiedFintechWorkflowSession>,
    right: &WorkflowCertificationReport<CertifiedFintechWorkflowSession>,
) -> DifferentialOutcome {
    let mut compared_surfaces = BTreeSet::new();
    let mut mismatches = Vec::new();
    let mut skipped_surfaces = Vec::new();

    compared_surfaces.insert(ArtifactSurface::BranchHeadState);
    let left_branch_heads = left
        .session
        .session_data
        .named_branches
        .iter()
        .map(|(alias, branch)| {
            (
                alias.clone(),
                left.session
                    .session_data
                    .world
                    .runtime
                    .branch_head_snapshot_id(branch.id),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let right_branch_heads = right
        .session
        .session_data
        .named_branches
        .iter()
        .map(|(alias, branch)| {
            (
                alias.clone(),
                right
                    .session
                    .session_data
                    .world
                    .runtime
                    .branch_head_snapshot_id(branch.id),
            )
        })
        .collect::<BTreeMap<_, _>>();
    if left_branch_heads != right_branch_heads {
        mismatches.push("branch head snapshot metadata diverged".to_string());
    }

    compared_surfaces.insert(ArtifactSurface::SnapshotVisibleTruth);
    if left.session.session_data.named_audits != right.session.session_data.named_audits {
        mismatches.push("snapshot-visible truth audit surfaces diverged".to_string());
    }

    compared_surfaces.insert(ArtifactSurface::ReplayRecoveryTruthState);
    for alias in ["analysis_replay_after", "correction_replay"] {
        let replay_diff = compare_replay_slices(
            left.session.session_data.named_replays.get(alias).unwrap(),
            right.session.session_data.named_replays.get(alias).unwrap(),
        );
        if !replay_diff.mismatches.is_empty() {
            mismatches.push(format!(
                "replay overlap drift for `{alias}`: {} mismatches",
                replay_diff.mismatches.len()
            ));
        }
    }
    skipped_surfaces.push(UnsupportedWorkflowComparison {
        surface: ArtifactSurface::ReplayRecoveryTruthState,
        reason: "main branch replay cursor/frame exactness is not yet guaranteed across executor variants for this hostile workflow".to_string(),
    });
    let lineage_diff = compare_lineage_records(
        left.session
            .session_data
            .named_lineages
            .get("correction_lineage")
            .unwrap(),
        right
            .session
            .session_data
            .named_lineages
            .get("correction_lineage")
            .unwrap(),
    );
    if !lineage_diff.mismatches.is_empty() {
        mismatches.push(format!(
            "lineage overlap drift: {} mismatches",
            lineage_diff.mismatches.len()
        ));
    }

    DifferentialOutcome {
        matched: mismatches.is_empty(),
        compared_surfaces,
        mismatches,
        skipped_surfaces,
    }
}
