use std::collections::BTreeMap;

use forge_harness::facade::{
    ArtifactBundle, ArtifactClass, ArtifactSurface, WorkflowCaptureRequest,
};
use serde_json::{json, Value};

use crate::facade::replay::RelationalReplayOutcome;
use crate::facade::snapshots::SnapshotHandle;
use crate::tests::support::read_entity_field;

use super::super::complexity::workflow_budgets;
use super::super::fixture::FintechCaseRole;
use super::super::probes::{capture_case_truth_probe, ProbeStage};
use super::session::CertifiedRelationalFintechSession;

pub(super) fn read_summary(
    session: &CertifiedRelationalFintechSession,
    snapshot: SnapshotHandle,
) -> Result<Value, String> {
    let read = session
        .world
        .runtime
        .read_truth()
        .read_snapshot(&snapshot)
        .ok_or_else(|| format!("snapshot `{}` is unavailable", snapshot.snapshot_id.0))?;
    let corrected_trades = read
        .entities()
        .iter()
        .filter(|entity| read_entity_field(entity, "corrected") == Some("true".into()))
        .count();
    let repaired_settlements = read
        .entities()
        .iter()
        .filter(|entity| {
            read_entity_field(entity, "entity_type") == Some("settlement".into())
                && read_entity_field(entity, "status") == Some("repaired".into())
        })
        .count();
    let open_breaches = read
        .entities()
        .iter()
        .filter(|entity| {
            read_entity_field(entity, "entity_type") == Some("limit_breach".into())
                && read_entity_field(entity, "status") == Some("open".into())
        })
        .count();
    Ok(json!({
        "snapshot_id": snapshot.snapshot_id.0,
        "entity_count": read.entities().len(),
        "relation_count": read.relations().len(),
        "corrected_trade_count": corrected_trades,
        "repaired_settlement_count": repaired_settlements,
        "open_breach_count": open_breaches,
    }))
}

pub(super) fn case_read_summary(
    session: &CertifiedRelationalFintechSession,
    case_role: FintechCaseRole,
) -> Value {
    let probe = capture_case_truth_probe(&session.world, case_role, ProbeStage::PostMutation);
    json!({
        "snapshot_id": probe.snapshot_id,
        "entity_count": probe.entity_count,
        "relation_count": probe.relation_count,
        "corrected_trade_count": probe.corrected_trade_count,
        "repaired_settlement_count": probe.repaired_settlement_count,
        "open_breach_count": probe.open_breach_count,
        "audit_record_count": probe.audit_record_count,
        "case_role": format!("{:?}", probe.case_role),
    })
}

fn replay_summary(replay: &RelationalReplayOutcome) -> Value {
    json!({
        "commit_id": replay.commit.as_ref().map(|commit| commit.commit_id.0),
        "commit_closure_len": replay.reconstructed_commit_closure.len(),
        "compared_surfaces": replay.compared_surfaces.iter().map(|surface| format!("{surface:?}")).collect::<Vec<_>>(),
        "mismatch_count": replay.mismatches.len(),
        "failure": replay.failure.as_ref().map(|failure| format!("{failure:?}")),
        "lineage_authority_basis_kind": replay.lineage_authority_basis.as_ref().map(|basis| format!("{:?}", basis.kind())),
        "lineage_authority_digest_mode": replay.lineage_authority_basis.as_ref().map(|basis| format!("{:?}", basis.digest_mode())),
        "lineage_authority_event_count": replay.lineage_authority_basis.as_ref().map(|basis| basis.lineage_event_count() as u64),
        "lineage_authority_decision_count": replay.lineage_authority_basis.as_ref().map(|basis| basis.lineage_decision_count() as u64),
    })
}

fn branch_summary(session: &CertifiedRelationalFintechSession) -> Value {
    let latest_commit = session
        .world
        .runtime
        .history()
        .latest_commit()
        .map(|commit| commit.commit_id.0);
    let branches = session
        .named_branches
        .iter()
        .map(|(alias, branch)| {
            (
                alias.clone(),
                json!({
                    "branch_id": branch.0,
                    "head_commit": session.world.runtime.history().branch_head(branch).map(|head| head.commit_id.0),
                }),
            )
        })
        .collect::<BTreeMap<_, _>>();
    json!({
        "latest_commit": latest_commit,
        "branches": branches,
    })
}

fn diagnostics_summary(session: &CertifiedRelationalFintechSession) -> Value {
    let recovery = session.world.runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
    let publication_diagnostics = session
        .world
        .runtime
        .publication()
        .diagnostic_access()
        .snapshot();
    let observation = &publication_diagnostics.observation;
    json!({
        "latest_patch_present": observation.latest_patch_present,
        "latest_replay_present": observation.latest_replay_present,
        "diagnostics_artifact_count": observation.diagnostics_artifact_count,
        "checkpoint_count": recovery
            .store
            .as_ref()
            .map(|store| store.checkpoints.len())
            .unwrap_or(session.checkpoints.len()),
        "tail_log_len": recovery.tail_log.len(),
        "selected_checkpoint": recovery.cursor.checkpoint_id.as_ref().map(|id| id.0),
    })
}

fn patch_summary(session: &CertifiedRelationalFintechSession) -> Value {
    let publication = session.world.runtime.publication();
    let artifacts = publication.artifact_snapshot();
    let observation = &artifacts.observation;
    json!({
        "latest_commit": observation.latest_commit_id.map(|commit_id| commit_id.0),
        "publication_snapshot": observation.publication_snapshot_id.map(|snapshot_id| snapshot_id.0),
        "publication_status": observation.publication_status.as_ref().map(|status| format!("{status:?}")),
        "patch_position": observation.latest_patch_position.map(|position| position.0),
        "patch_record_count": observation.latest_patch_record_count,
        "replay_commit": observation.latest_replay_commit_id.map(|commit_id| commit_id.0),
        "snapshot_patch_matches_latest_patch": artifacts
            .latest_patch
            .as_ref()
            .zip(publication.latest_patch())
            .map(|(snapshot_patch, patch)| snapshot_patch == patch)
            .unwrap_or(false),
        "snapshot_replay_matches_latest_replay": artifacts
            .latest_replay
            .as_ref()
            .zip(publication.latest_replay())
            .map(|(snapshot_replay, replay)| snapshot_replay == replay)
            .unwrap_or(false),
    })
}

fn complexity_summary(session: &CertifiedRelationalFintechSession) -> Value {
    let counters = session.world.runtime.performance_access().counters();
    json!({
        "full_state_clones": counters.full_state_clones,
        "snapshot_pin_full_rebuilds": counters.snapshot_pin_full_rebuilds,
        "partitions_touched_by_commit": counters.partitions_touched_by_commit,
        "entity_slots_touched_by_commit": counters.entity_slots_touched_by_commit,
        "visibility_entity_slot_scans": counters.visibility_entity_slot_scans,
        "visibility_relation_slot_scans": counters.visibility_relation_slot_scans,
    })
}

fn budget_summary(session: &CertifiedRelationalFintechSession) -> Value {
    let counters = session.world.runtime.performance_access().counters();
    let checks = workflow_budgets()
        .into_iter()
        .map(|budget| {
            let actual = (budget.selector)(&counters);
            json!({
                "label": budget.label,
                "max": budget.max,
                "actual": actual,
                "passed": actual <= budget.max,
            })
        })
        .collect::<Vec<_>>();
    let all_passed = checks
        .iter()
        .all(|check| check.get("passed").and_then(|value| value.as_bool()) == Some(true));
    json!({
        "all_passed": all_passed,
        "checks": checks,
    })
}

pub(super) fn capture_artifacts(
    session: &CertifiedRelationalFintechSession,
    request: &WorkflowCaptureRequest,
) -> Vec<ArtifactBundle> {
    let mut artifacts = Vec::new();
    for surface in &request.requested_surfaces {
        match surface {
            ArtifactSurface::SnapshotVisibleTruth => artifacts.push(ArtifactBundle {
                artifact_class: ArtifactClass::Truth,
                surface: ArtifactSurface::SnapshotVisibleTruth,
                name: "snapshot-visible-truth".to_string(),
                boundary: request.boundary,
                payload: json!(session.named_reads),
                attachments: Vec::new(),
                metadata: BTreeMap::new(),
            }),
            ArtifactSurface::BranchHeadState => artifacts.push(ArtifactBundle {
                artifact_class: ArtifactClass::Truth,
                surface: ArtifactSurface::BranchHeadState,
                name: "branch-head-state".to_string(),
                boundary: request.boundary,
                payload: branch_summary(session),
                attachments: Vec::new(),
                metadata: BTreeMap::new(),
            }),
            ArtifactSurface::ReplayRecoveryTruthState => artifacts.push(ArtifactBundle {
                artifact_class: ArtifactClass::Truth,
                surface: ArtifactSurface::ReplayRecoveryTruthState,
                name: "replay-recovery-truth".to_string(),
                boundary: request.boundary,
                payload: json!(session
                    .named_replays
                    .iter()
                    .map(|(alias, replay)| (alias.clone(), replay_summary(replay)))
                    .collect::<BTreeMap<_, _>>()),
                attachments: Vec::new(),
                metadata: BTreeMap::new(),
            }),
            ArtifactSurface::Diagnostics => artifacts.push(ArtifactBundle {
                artifact_class: ArtifactClass::Observability,
                surface: ArtifactSurface::Diagnostics,
                name: "diagnostics".to_string(),
                boundary: request.boundary,
                payload: diagnostics_summary(session),
                attachments: Vec::new(),
                metadata: BTreeMap::new(),
            }),
            ArtifactSurface::PatchChangeSurface => artifacts.push(ArtifactBundle {
                artifact_class: ArtifactClass::Observability,
                surface: ArtifactSurface::PatchChangeSurface,
                name: "patch-change-surface".to_string(),
                boundary: request.boundary,
                payload: patch_summary(session),
                attachments: Vec::new(),
                metadata: BTreeMap::new(),
            }),
            ArtifactSurface::StepTrace => artifacts.push(ArtifactBundle {
                artifact_class: ArtifactClass::Forensic,
                surface: ArtifactSurface::StepTrace,
                name: "step-trace".to_string(),
                boundary: request.boundary,
                payload: json!(session.executed_steps),
                attachments: Vec::new(),
                metadata: BTreeMap::new(),
            }),
            ArtifactSurface::CheckpointTrace => artifacts.push(ArtifactBundle {
                artifact_class: ArtifactClass::Forensic,
                surface: ArtifactSurface::CheckpointTrace,
                name: "checkpoint-trace".to_string(),
                boundary: request.boundary,
                payload: json!({
                    "checkpoints": session.checkpoints,
                    "snapshots": session.named_snapshots.keys().collect::<Vec<_>>(),
                }),
                attachments: Vec::new(),
                metadata: BTreeMap::new(),
            }),
            ArtifactSurface::ComplexityCounters => artifacts.push(ArtifactBundle {
                artifact_class: ArtifactClass::Performance,
                surface: ArtifactSurface::ComplexityCounters,
                name: "complexity-counters".to_string(),
                boundary: request.boundary,
                payload: complexity_summary(session),
                attachments: Vec::new(),
                metadata: BTreeMap::new(),
            }),
            ArtifactSurface::BudgetOutcome => artifacts.push(ArtifactBundle {
                artifact_class: ArtifactClass::Performance,
                surface: ArtifactSurface::BudgetOutcome,
                name: "budget-outcome".to_string(),
                boundary: request.boundary,
                payload: budget_summary(session),
                attachments: Vec::new(),
                metadata: BTreeMap::new(),
            }),
            _ => {}
        }
    }
    artifacts
}
