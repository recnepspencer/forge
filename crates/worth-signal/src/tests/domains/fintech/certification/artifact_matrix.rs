use std::collections::{BTreeMap, BTreeSet};

use crate::facade::SignalError;
use serde_json::json;
use worth_harness::facade::{
    AdapterSupport, ArtifactBundle, ArtifactClass, ArtifactSurface, CheckpointSemantics,
    DifferentialMatrixCapability, ProfileConditionalGuarantee, UnsupportedWorkflowComparison,
    WorkflowArtifactSurfaceCapability, WorkflowCaptureRequest, WorkflowCertificationCapabilities,
};

use super::workflow_session::{
    CertifiedFintechWorkflowSession, SignalFintechWorkflowCertificationAdapter,
};

pub(super) fn capabilities() -> WorkflowCertificationCapabilities {
    WorkflowCertificationCapabilities {
        artifact_surfaces: vec![
            WorkflowArtifactSurfaceCapability {
                surface: ArtifactSurface::SnapshotVisibleTruth,
                profiles: BTreeSet::from([
                    "fintech-development".to_string(),
                    "fintech-forensic".to_string(),
                ]),
            },
            WorkflowArtifactSurfaceCapability {
                surface: ArtifactSurface::BranchHeadState,
                profiles: BTreeSet::from([
                    "fintech-development".to_string(),
                    "fintech-forensic".to_string(),
                ]),
            },
            WorkflowArtifactSurfaceCapability {
                surface: ArtifactSurface::ReplayRecoveryTruthState,
                profiles: BTreeSet::from([
                    "fintech-development".to_string(),
                    "fintech-forensic".to_string(),
                ]),
            },
            WorkflowArtifactSurfaceCapability {
                surface: ArtifactSurface::StepTrace,
                profiles: BTreeSet::from([
                    "fintech-development".to_string(),
                    "fintech-forensic".to_string(),
                ]),
            },
            WorkflowArtifactSurfaceCapability {
                surface: ArtifactSurface::CheckpointTrace,
                profiles: BTreeSet::from([
                    "fintech-development".to_string(),
                    "fintech-forensic".to_string(),
                ]),
            },
            WorkflowArtifactSurfaceCapability {
                surface: ArtifactSurface::FailureInjectionTrace,
                profiles: BTreeSet::from([
                    "fintech-development".to_string(),
                    "fintech-forensic".to_string(),
                ]),
            },
        ],
        checkpoint_semantics: BTreeSet::from([
            CheckpointSemantics::BranchHeadSnapshot,
            CheckpointSemantics::SnapshotRestore,
            CheckpointSemantics::ReplayAnchor,
        ]),
        replay_recovery_support: BTreeSet::from([
            ArtifactSurface::BranchHeadState,
            ArtifactSurface::SnapshotVisibleTruth,
            ArtifactSurface::ReplayRecoveryTruthState,
        ]),
        differential_matrices: vec![DifferentialMatrixCapability {
            matrix_name: "serial-vs-parallel-hostile".to_string(),
            profiles: BTreeSet::from([
                "fintech-development".to_string(),
                "fintech-forensic".to_string(),
            ]),
            guaranteed_surfaces: BTreeSet::from([
                ArtifactSurface::BranchHeadState,
                ArtifactSurface::SnapshotVisibleTruth,
                ArtifactSurface::ReplayRecoveryTruthState,
            ]),
        }],
        unsupported_comparisons: vec![UnsupportedWorkflowComparison {
            surface: ArtifactSurface::Diagnostics,
            reason: "signal workflow certification has not yet frozen diagnostics-order overlap"
                .to_string(),
        }],
        profile_guarantees: vec![
            ProfileConditionalGuarantee {
                profile_name: "fintech-development".to_string(),
                guarantee: "branch/snapshot/replay overlap is stable across hostile workflows"
                    .to_string(),
            },
            ProfileConditionalGuarantee {
                profile_name: "fintech-forensic".to_string(),
                guarantee: "failure reproduction includes branch-local replay and lineage evidence"
                    .to_string(),
            },
        ],
        budget_artifacts: AdapterSupport::Unsupported,
    }
}

pub(super) fn capture_artifacts(
    session: &CertifiedFintechWorkflowSession,
    request: &WorkflowCaptureRequest,
) -> Result<Vec<ArtifactBundle>, SignalError> {
    let mut artifacts = Vec::new();
    for surface in &request.requested_surfaces {
        match surface {
            ArtifactSurface::SnapshotVisibleTruth => {
                let audits = session
                        .named_audits
                        .iter()
                        .map(|(alias, audit)| {
                            (
                                alias.clone(),
                                json!({
                                    "desk": SignalFintechWorkflowCertificationAdapter::version_summary(&audit.desk),
                                    "scenario": SignalFintechWorkflowCertificationAdapter::version_summary(&audit.scenario),
                                }),
                            )
                        })
                        .collect::<BTreeMap<_, _>>();
                artifacts.push(ArtifactBundle {
                    artifact_class: ArtifactClass::Truth,
                    surface: ArtifactSurface::SnapshotVisibleTruth,
                    name: "snapshot-visible-truth".to_string(),
                    boundary: request.boundary,
                    payload: json!(audits),
                    attachments: Vec::new(),
                    metadata: BTreeMap::new(),
                });
            }
            ArtifactSurface::BranchHeadState => {
                let branch_heads = session
                        .named_branches
                        .iter()
                        .map(|(alias, branch)| {
                            (
                                alias.clone(),
                                json!({
                                    "branch_name": branch.name,
                                    "head_snapshot": session.world.branch_head_snapshot_id(branch.clone()).map(|id| id.0),
                                }),
                            )
                        })
                        .collect::<BTreeMap<_, _>>();
                artifacts.push(ArtifactBundle {
                    artifact_class: ArtifactClass::Truth,
                    surface: ArtifactSurface::BranchHeadState,
                    name: "branch-head-state".to_string(),
                    boundary: request.boundary,
                    payload: json!(branch_heads),
                    attachments: Vec::new(),
                    metadata: BTreeMap::new(),
                });
            }
            ArtifactSurface::ReplayRecoveryTruthState => {
                let replay = session
                    .named_replays
                    .iter()
                    .map(|(alias, replay)| {
                        (
                            alias.clone(),
                            SignalFintechWorkflowCertificationAdapter::replay_summary(replay),
                        )
                    })
                    .collect::<BTreeMap<_, _>>();
                let lineage = session
                    .named_lineages
                    .iter()
                    .map(|(alias, lineage)| {
                        (
                            alias.clone(),
                            SignalFintechWorkflowCertificationAdapter::lineage_summary(
                                lineage.as_slice(),
                            ),
                        )
                    })
                    .collect::<BTreeMap<_, _>>();
                artifacts.push(ArtifactBundle {
                    artifact_class: ArtifactClass::Truth,
                    surface: ArtifactSurface::ReplayRecoveryTruthState,
                    name: "replay-recovery-truth".to_string(),
                    boundary: request.boundary,
                    payload: json!({
                        "replays": replay,
                        "lineages": lineage,
                    }),
                    attachments: Vec::new(),
                    metadata: BTreeMap::new(),
                });
            }
            ArtifactSurface::StepTrace => {
                artifacts.push(ArtifactBundle {
                    artifact_class: ArtifactClass::Forensic,
                    surface: ArtifactSurface::StepTrace,
                    name: "step-trace".to_string(),
                    boundary: request.boundary,
                    payload: json!(session.executed_steps),
                    attachments: Vec::new(),
                    metadata: BTreeMap::new(),
                });
            }
            ArtifactSurface::CheckpointTrace => {
                artifacts.push(ArtifactBundle {
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
                });
            }
            ArtifactSurface::FailureInjectionTrace => {
                artifacts.push(ArtifactBundle {
                    artifact_class: ArtifactClass::Forensic,
                    surface: ArtifactSurface::FailureInjectionTrace,
                    name: "failure-injection-trace".to_string(),
                    boundary: request.boundary,
                    payload: json!(session.failure_injections),
                    attachments: Vec::new(),
                    metadata: BTreeMap::new(),
                });
            }
            _ => {}
        }
    }
    Ok(artifacts)
}
