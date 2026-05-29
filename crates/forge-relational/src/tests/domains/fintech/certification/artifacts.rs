use std::collections::BTreeMap;

use forge_harness::facade::{
    ArtifactBundle, ArtifactClass, ArtifactSurface, WorkflowCaptureRequest,
};

use crate::facade::replay::RelationalReplayOutcome;

use super::super::complexity::workflow_budgets;
use super::certification_artifact_value::{
    artifact_field, artifact_object, bool_field, dynamic_artifact_object, optional_string_field,
    optional_u64_field, optional_usize_field, string_array_field, string_field, usize_field,
    CertificationArtifactValue,
};
use super::read_summaries::read_summary_artifacts;
use super::session::CertifiedRelationalFintechSession;

fn replay_summary(replay: &RelationalReplayOutcome) -> CertificationArtifactValue {
    artifact_object([
        optional_u64_field(
            "commit_id",
            replay.commit.as_ref().map(|commit| commit.commit_id.0),
        ),
        usize_field(
            "commit_closure_len",
            replay.reconstructed_commit_closure.len(),
        ),
        string_array_field(
            "compared_surfaces",
            replay
                .compared_surfaces
                .iter()
                .map(|surface| format!("{surface:?}")),
        ),
        usize_field("mismatch_count", replay.mismatches.len()),
        optional_string_field(
            "failure",
            replay
                .failure
                .as_ref()
                .map(|failure| format!("{failure:?}")),
        ),
        optional_string_field(
            "lineage_authority_basis_kind",
            replay
                .lineage_authority_basis
                .as_ref()
                .map(|basis| format!("{:?}", basis.kind())),
        ),
        optional_string_field(
            "lineage_authority_digest_mode",
            replay
                .lineage_authority_basis
                .as_ref()
                .map(|basis| format!("{:?}", basis.digest_mode())),
        ),
        optional_u64_field(
            "lineage_authority_event_count",
            replay
                .lineage_authority_basis
                .as_ref()
                .map(|basis| basis.lineage_event_count() as u64),
        ),
        optional_u64_field(
            "lineage_authority_decision_count",
            replay
                .lineage_authority_basis
                .as_ref()
                .map(|basis| basis.lineage_decision_count() as u64),
        ),
    ])
}

fn branch_summary(session: &CertifiedRelationalFintechSession) -> CertificationArtifactValue {
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
                artifact_object([
                    string_field("branch_id", branch.0.clone()),
                    optional_u64_field(
                        "head_commit",
                        session
                            .world
                            .runtime
                            .history()
                            .branch_head(branch)
                            .map(|head| head.commit_id.0),
                    ),
                ]),
            )
        })
        .collect::<Vec<_>>();
    artifact_object([
        optional_u64_field("latest_commit", latest_commit),
        artifact_field("branches", dynamic_artifact_object(branches)),
    ])
}

fn diagnostics_summary(session: &CertifiedRelationalFintechSession) -> CertificationArtifactValue {
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
    artifact_object([
        bool_field("latest_patch_present", observation.latest_patch_present),
        bool_field("latest_replay_present", observation.latest_replay_present),
        usize_field(
            "diagnostics_artifact_count",
            observation.diagnostics_artifact_count,
        ),
        usize_field(
            "checkpoint_count",
            recovery
                .store
                .as_ref()
                .map(|store| store.checkpoints.len())
                .unwrap_or(session.checkpoints.len()),
        ),
        usize_field("tail_log_len", recovery.tail_log.len()),
        optional_u64_field(
            "selected_checkpoint",
            recovery.cursor.checkpoint_id.as_ref().map(|id| id.0),
        ),
    ])
}

fn patch_summary(session: &CertifiedRelationalFintechSession) -> CertificationArtifactValue {
    let publication = session.world.runtime.publication();
    let artifacts = publication.artifact_snapshot();
    let observation = &artifacts.observation;
    artifact_object([
        optional_u64_field(
            "latest_commit",
            observation.latest_commit_id.map(|commit_id| commit_id.0),
        ),
        optional_u64_field(
            "publication_snapshot",
            observation
                .publication_snapshot_id
                .map(|snapshot_id| snapshot_id.0),
        ),
        optional_string_field(
            "publication_status",
            observation
                .publication_status
                .as_ref()
                .map(|status| format!("{status:?}")),
        ),
        optional_u64_field(
            "patch_position",
            observation.latest_patch_position.map(|position| position.0),
        ),
        optional_usize_field("patch_record_count", observation.latest_patch_record_count),
        optional_u64_field(
            "replay_commit",
            observation
                .latest_replay_commit_id
                .map(|commit_id| commit_id.0),
        ),
        bool_field(
            "snapshot_patch_matches_latest_patch",
            artifacts
                .latest_patch
                .as_ref()
                .zip(publication.latest_patch())
                .map(|(snapshot_patch, patch)| snapshot_patch == patch)
                .unwrap_or(false),
        ),
        bool_field(
            "snapshot_replay_matches_latest_replay",
            artifacts
                .latest_replay
                .as_ref()
                .zip(publication.latest_replay())
                .map(|(snapshot_replay, replay)| snapshot_replay == replay)
                .unwrap_or(false),
        ),
    ])
}

fn complexity_summary(session: &CertifiedRelationalFintechSession) -> CertificationArtifactValue {
    let counters = session.world.runtime.performance_access().counters();
    artifact_object([
        usize_field("full_state_clones", counters.full_state_clones),
        usize_field(
            "snapshot_pin_full_rebuilds",
            counters.snapshot_pin_full_rebuilds,
        ),
        usize_field(
            "partitions_touched_by_commit",
            counters.partitions_touched_by_commit,
        ),
        usize_field(
            "entity_slots_touched_by_commit",
            counters.entity_slots_touched_by_commit,
        ),
        usize_field(
            "visibility_entity_slot_scans",
            counters.visibility_entity_slot_scans,
        ),
        usize_field(
            "visibility_relation_slot_scans",
            counters.visibility_relation_slot_scans,
        ),
    ])
}

fn budget_summary(session: &CertifiedRelationalFintechSession) -> CertificationArtifactValue {
    let counters = session.world.runtime.performance_access().counters();
    let mut all_passed = true;
    let checks = workflow_budgets()
        .into_iter()
        .map(|budget| {
            let actual = (budget.selector)(&counters);
            let passed = actual <= budget.max;
            all_passed &= passed;
            artifact_object([
                string_field("label", budget.label.to_string()),
                usize_field("max", budget.max),
                usize_field("actual", actual),
                bool_field("passed", passed),
            ])
        })
        .collect::<Vec<_>>();
    artifact_object([
        bool_field("all_passed", all_passed),
        artifact_field("checks", CertificationArtifactValue::Array(checks)),
    ])
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
                payload: read_summary_artifacts(&session.named_reads).into_json(),
                attachments: Vec::new(),
                metadata: BTreeMap::new(),
            }),
            ArtifactSurface::BranchHeadState => artifacts.push(ArtifactBundle {
                artifact_class: ArtifactClass::Truth,
                surface: ArtifactSurface::BranchHeadState,
                name: "branch-head-state".to_string(),
                boundary: request.boundary,
                payload: branch_summary(session).into_json(),
                attachments: Vec::new(),
                metadata: BTreeMap::new(),
            }),
            ArtifactSurface::ReplayRecoveryTruthState => artifacts.push(ArtifactBundle {
                artifact_class: ArtifactClass::Truth,
                surface: ArtifactSurface::ReplayRecoveryTruthState,
                name: "replay-recovery-truth".to_string(),
                boundary: request.boundary,
                payload: dynamic_artifact_object(
                    session
                        .named_replays
                        .iter()
                        .map(|(alias, replay)| (alias.clone(), replay_summary(replay)))
                        .collect::<Vec<_>>(),
                )
                .into_json(),
                attachments: Vec::new(),
                metadata: BTreeMap::new(),
            }),
            ArtifactSurface::Diagnostics => artifacts.push(ArtifactBundle {
                artifact_class: ArtifactClass::Observability,
                surface: ArtifactSurface::Diagnostics,
                name: "diagnostics".to_string(),
                boundary: request.boundary,
                payload: diagnostics_summary(session).into_json(),
                attachments: Vec::new(),
                metadata: BTreeMap::new(),
            }),
            ArtifactSurface::PatchChangeSurface => artifacts.push(ArtifactBundle {
                artifact_class: ArtifactClass::Observability,
                surface: ArtifactSurface::PatchChangeSurface,
                name: "patch-change-surface".to_string(),
                boundary: request.boundary,
                payload: patch_summary(session).into_json(),
                attachments: Vec::new(),
                metadata: BTreeMap::new(),
            }),
            ArtifactSurface::StepTrace => artifacts.push(ArtifactBundle {
                artifact_class: ArtifactClass::Forensic,
                surface: ArtifactSurface::StepTrace,
                name: "step-trace".to_string(),
                boundary: request.boundary,
                payload: CertificationArtifactValue::Array(
                    session
                        .executed_steps
                        .iter()
                        .cloned()
                        .map(CertificationArtifactValue::String)
                        .collect(),
                )
                .into_json(),
                attachments: Vec::new(),
                metadata: BTreeMap::new(),
            }),
            ArtifactSurface::CheckpointTrace => artifacts.push(ArtifactBundle {
                artifact_class: ArtifactClass::Forensic,
                surface: ArtifactSurface::CheckpointTrace,
                name: "checkpoint-trace".to_string(),
                boundary: request.boundary,
                payload: artifact_object([
                    string_array_field("checkpoints", session.checkpoints.iter().cloned()),
                    string_array_field("snapshots", session.named_snapshots.keys().cloned()),
                ])
                .into_json(),
                attachments: Vec::new(),
                metadata: BTreeMap::new(),
            }),
            ArtifactSurface::ComplexityCounters => artifacts.push(ArtifactBundle {
                artifact_class: ArtifactClass::Performance,
                surface: ArtifactSurface::ComplexityCounters,
                name: "complexity-counters".to_string(),
                boundary: request.boundary,
                payload: complexity_summary(session).into_json(),
                attachments: Vec::new(),
                metadata: BTreeMap::new(),
            }),
            ArtifactSurface::BudgetOutcome => artifacts.push(ArtifactBundle {
                artifact_class: ArtifactClass::Performance,
                surface: ArtifactSurface::BudgetOutcome,
                name: "budget-outcome".to_string(),
                boundary: request.boundary,
                payload: budget_summary(session).into_json(),
                attachments: Vec::new(),
                metadata: BTreeMap::new(),
            }),
            _ => {}
        }
    }
    artifacts
}
