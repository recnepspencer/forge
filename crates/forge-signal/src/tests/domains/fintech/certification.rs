use std::collections::{BTreeMap, BTreeSet};

use forge_harness::facade::{
    AdapterSupport, ArtifactBundle, ArtifactClass, ArtifactSurface, CheckpointSemantics,
    DifferentialMatrixCapability, FailureInjectionPoint, InvariantCheck, InvariantReport,
    ProfileConditionalGuarantee, RegressionTarget, RegressionTargetKind, ReproductionMetadata,
    UnsupportedWorkflowComparison, WorkflowArtifactSurfaceCapability, WorkflowCaptureRequest,
    WorkflowCertificationAdapter, WorkflowCertificationCapabilities, WorkflowCertificationRunner,
    WorkflowCheckpoint, WorkflowFailureContext, WorkflowPlan, WorkflowRuntimeProfile,
    WorkflowState, WorkflowStep, WorkflowStepOutcome,
};
use serde_json::{json, Value};

#[allow(unused_imports)]
use crate::facade::*;
#[cfg(feature = "parallel")]
use forge_harness::facade::{DifferentialOutcome, WorkflowCertificationReport};

use super::aspects::{ALERT, CURVE, LIQUIDITY, PRICE, RISK, VOL};
use super::audit_surface::PrimaryAuditSurface;
use super::certification_naming::artifact_aliases;
use super::certification_naming::invariant_names;
use super::certification_naming::workflow_names;
use super::fixture::FintechWorld;
use super::market_seed::MarketSeed;
use super::regimes::MarketRegime;
use super::scales::FintechScale;
use super::world_assembly::WorldAssembly;

#[derive(Debug, Clone)]
enum FintechWorkflowStep {
    SeedRegime {
        regime: MarketRegime,
        seed: u64,
    },
    CaptureActiveSnapshot {
        alias: &'static str,
    },
    OpenBranch {
        branch_name: &'static str,
        alias: &'static str,
    },
    SwitchBranch {
        alias: &'static str,
    },
    ReadPrimaryAuditSurface {
        alias: &'static str,
    },
    InjectSyntheticRollback,
    RestoreSnapshot {
        branch_alias: &'static str,
        snapshot_alias: &'static str,
    },
    CaptureReplay {
        branch_alias: &'static str,
        alias: &'static str,
    },
    CaptureReplayAroundSnapshot {
        snapshot_alias: &'static str,
        alias: &'static str,
    },
    CaptureMainRiskLineage {
        alias: &'static str,
    },
}

struct CertifiedFintechWorkflowSession {
    world: FintechWorld,
    executor: StageExecutor,
    policy: SignalRuntimePolicy,
    named_branches: BTreeMap<String, SignalBranchHandle>,
    named_snapshots: BTreeMap<String, SignalSnapshotV1>,
    named_audits: BTreeMap<String, PrimaryAuditSurface>,
    named_replays: BTreeMap<String, ReplaySlice>,
    named_lineages: BTreeMap<String, Vec<LineageRecord>>,
    executed_steps: Vec<String>,
    checkpoints: Vec<String>,
    failure_injections: Vec<String>,
}

impl CertifiedFintechWorkflowSession {
    fn branch(&self, alias: &str) -> Result<SignalBranchHandle, SignalError> {
        self.named_branches.get(alias).cloned().ok_or_else(|| {
            SignalError::invalid_input(format!("unknown certified fintech branch alias `{alias}`"))
        })
    }

    fn snapshot(&self, alias: &str) -> Result<SignalSnapshotV1, SignalError> {
        self.named_snapshots.get(alias).cloned().ok_or_else(|| {
            SignalError::invalid_input(format!(
                "unknown certified fintech snapshot alias `{alias}`"
            ))
        })
    }

    fn replay(&self, alias: &str) -> Result<&ReplaySlice, SignalError> {
        self.named_replays.get(alias).ok_or_else(|| {
            SignalError::invalid_input(format!("unknown certified fintech replay alias `{alias}`"))
        })
    }

    fn lineage(&self, alias: &str) -> Result<&Vec<LineageRecord>, SignalError> {
        self.named_lineages.get(alias).ok_or_else(|| {
            SignalError::invalid_input(format!("unknown certified fintech lineage alias `{alias}`"))
        })
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct SignalFintechWorkflowCertificationAdapter;

impl SignalFintechWorkflowCertificationAdapter {
    fn runtime_policy(
        profile: &WorkflowRuntimeProfile,
    ) -> Result<SignalRuntimePolicy, SignalError> {
        match profile.policy_name.as_deref().unwrap_or("fintech") {
            "fintech" => Ok(SignalRuntimePolicy::fintech()
                .with_history_limit(8)
                .with_detail_limit(4)),
            "development" => Ok(SignalRuntimePolicy::development()
                .with_history_limit(8)
                .with_detail_limit(4)),
            "forensic" => Ok(SignalRuntimePolicy::forensic()
                .with_history_limit(8)
                .with_detail_limit(4)),
            other => Err(SignalError::invalid_input(format!(
                "unsupported workflow certification policy `{other}`"
            ))),
        }
    }

    fn executor(profile: &WorkflowRuntimeProfile) -> Result<StageExecutor, SignalError> {
        match profile.executor_name.as_deref().unwrap_or("serial") {
            "serial" => Ok(StageExecutor::Serial),
            "aggressive-parallel" => {
                #[cfg(feature = "parallel")]
                {
                    Ok(StageExecutor::aggressive_parallel())
                }
                #[cfg(not(feature = "parallel"))]
                {
                    Err(SignalError::invalid_input(
                        "aggressive-parallel workflow certification requires the `parallel` feature",
                    ))
                }
            }
            other => Err(SignalError::invalid_input(format!(
                "unsupported workflow certification executor `{other}`"
            ))),
        }
    }

    fn version_summary(version: &AspectVersion) -> Value {
        json!({
            "price": version.get(PRICE),
            "vol": version.get(VOL),
            "curve": version.get(CURVE),
            "liquidity": version.get(LIQUIDITY),
            "risk": version.get(RISK),
            "alert": version.get(ALERT),
        })
    }

    fn replay_summary(replay: &ReplaySlice) -> Value {
        let mut kinds = BTreeMap::new();
        for frame in &replay.frames {
            *kinds.entry(format!("{:?}", frame.kind)).or_insert(0usize) += 1;
        }
        json!({
            "frame_count": replay.frames.len(),
            "start": replay.start.map(|cursor| cursor.0),
            "end": replay.end.map(|cursor| cursor.0),
            "kinds": kinds,
        })
    }

    fn lineage_summary(lineage: &[LineageRecord]) -> Value {
        let mut events = BTreeMap::new();
        for record in lineage {
            *events.entry(record.label().to_string()).or_insert(0usize) += 1;
        }
        json!({
            "record_count": lineage.len(),
            "events": events,
        })
    }

    fn parse_replay_kind(value: &str) -> Result<ReplayEventKind, SignalError> {
        match value {
            "TaskApplied" => Ok(ReplayEventKind::TaskApplied),
            "TransactionCommitted" => Ok(ReplayEventKind::TransactionCommitted),
            "TransactionRolledBack" => Ok(ReplayEventKind::TransactionRolledBack),
            "FailureRecorded" => Ok(ReplayEventKind::FailureRecorded),
            "SnapshotCaptured" => Ok(ReplayEventKind::SnapshotCaptured),
            "SnapshotRestored" => Ok(ReplayEventKind::SnapshotRestored),
            "BranchCreated" => Ok(ReplayEventKind::BranchCreated),
            "BranchSwitched" => Ok(ReplayEventKind::BranchSwitched),
            other => Err(SignalError::invalid_input(format!(
                "unknown replay event kind `{other}`"
            ))),
        }
    }

    fn parse_lineage_events(value: &str) -> Result<Vec<String>, SignalError> {
        value
            .split(',')
            .map(|event| match event {
                "Refreshed"
                | "Replaced"
                | "Restored"
                | "BranchedFrom"
                | "BranchSwitched"
                | "MergedFrom"
                | "MemoizedReuse"
                | "InvalidatedWithoutReplacement" => Ok(event.to_string()),
                other => Err(SignalError::invalid_input(format!(
                    "unknown lineage event `{other}`"
                ))),
            })
            .collect()
    }

    fn check_invariant(
        &self,
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
                let kind = Self::parse_replay_kind(kind)?;
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
                let events = Self::parse_lineage_events(events)?;
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
                    session.world.branch_head_snapshot_id(branch)
                        == Some(snapshot.meta.snapshot_id),
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
}

impl WorkflowCertificationAdapter for SignalFintechWorkflowCertificationAdapter {
    type Session = CertifiedFintechWorkflowSession;
    type Step = FintechWorkflowStep;
    type Error = SignalError;

    fn adapter_name(&self) -> &'static str {
        "forge-signal-fintech-workflow"
    }

    fn capabilities(&self) -> WorkflowCertificationCapabilities {
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
                reason:
                    "signal workflow certification has not yet frozen diagnostics-order overlap"
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
                    guarantee:
                        "failure reproduction includes branch-local replay and lineage evidence"
                            .to_string(),
                },
            ],
            budget_artifacts: AdapterSupport::Unsupported,
        }
    }

    fn initialize_session(
        &self,
        _plan: &WorkflowPlan<Self::Step>,
        profile: &WorkflowRuntimeProfile,
    ) -> Result<Self::Session, Self::Error> {
        let policy = Self::runtime_policy(profile)?;
        let executor = Self::executor(profile)?;
        let mut world = WorldAssembly::new(FintechScale::smoke())
            .without_market_seed()
            .build();
        world.runtime.set_runtime_policy(policy);
        let main = world.current_branch();
        Ok(CertifiedFintechWorkflowSession {
            world,
            executor,
            policy,
            named_branches: BTreeMap::from([("main".to_string(), main)]),
            named_snapshots: BTreeMap::new(),
            named_audits: BTreeMap::new(),
            named_replays: BTreeMap::new(),
            named_lineages: BTreeMap::new(),
            executed_steps: Vec::new(),
            checkpoints: Vec::new(),
            failure_injections: Vec::new(),
        })
    }

    fn execute_step(
        &self,
        session: &mut Self::Session,
        step: &WorkflowStep<Self::Step>,
        injection: Option<&FailureInjectionPoint>,
    ) -> Result<WorkflowStepOutcome, Self::Error> {
        session.executed_steps.push(step.name.clone());
        match &step.operation {
            FintechWorkflowStep::SeedRegime { regime, seed } => {
                session.world.seed_market(MarketSeed::new(*regime, *seed))?;
                Ok(WorkflowStepOutcome::applied())
            }
            FintechWorkflowStep::CaptureActiveSnapshot { alias } => {
                let snapshot = session.world.capture_world_snapshot();
                session
                    .named_snapshots
                    .insert((*alias).to_string(), snapshot);
                Ok(WorkflowStepOutcome {
                    detail: Some(format!("captured snapshot `{alias}`")),
                    request_checkpoint: true,
                })
            }
            FintechWorkflowStep::OpenBranch { branch_name, alias } => {
                let branch = session.world.open_branch(branch_name)?;
                session.named_branches.insert((*alias).to_string(), branch);
                Ok(WorkflowStepOutcome::applied())
            }
            FintechWorkflowStep::SwitchBranch { alias } => {
                let branch = session.branch(alias)?;
                session.world.runtime.switch_branch(branch)?;
                Ok(WorkflowStepOutcome::applied())
            }
            FintechWorkflowStep::ReadPrimaryAuditSurface { alias } => {
                let value = session.world.read_primary_audit_surface(session.executor)?;
                session.named_audits.insert((*alias).to_string(), value);
                Ok(WorkflowStepOutcome::applied())
            }
            FintechWorkflowStep::InjectSyntheticRollback => {
                session
                    .world
                    .inject_primary_market_rollback(session.executor)?;
                if let Some(injection) = injection {
                    session
                        .failure_injections
                        .push(format!("{:?}:{}", injection.boundary, injection.location));
                }
                Ok(WorkflowStepOutcome {
                    detail: Some("synthetic rollback captured".to_string()),
                    request_checkpoint: false,
                })
            }
            FintechWorkflowStep::RestoreSnapshot {
                branch_alias,
                snapshot_alias,
            } => {
                let branch = session.branch(branch_alias)?;
                let snapshot = session.snapshot(snapshot_alias)?;
                session
                    .world
                    .runtime
                    .restore_branch_snapshot(branch, &snapshot)?;
                Ok(WorkflowStepOutcome::applied())
            }
            FintechWorkflowStep::CaptureReplay {
                branch_alias,
                alias,
            } => {
                let branch = session.branch(branch_alias)?;
                let replay = session.world.replay_for_branch(branch);
                session.named_replays.insert((*alias).to_string(), replay);
                Ok(WorkflowStepOutcome::applied())
            }
            FintechWorkflowStep::CaptureReplayAroundSnapshot {
                snapshot_alias,
                alias,
            } => {
                let snapshot = session.snapshot(snapshot_alias)?;
                let replay = session.world.replay_around_saved_snapshot(&snapshot);
                session.named_replays.insert((*alias).to_string(), replay);
                Ok(WorkflowStepOutcome::applied())
            }
            FintechWorkflowStep::CaptureMainRiskLineage { alias } => {
                let lineage = session.world.main_risk_lineage();
                session.named_lineages.insert((*alias).to_string(), lineage);
                Ok(WorkflowStepOutcome::applied())
            }
        }
    }

    fn create_checkpoint(
        &self,
        session: &mut Self::Session,
        checkpoint: &WorkflowCheckpoint,
    ) -> Result<(), Self::Error> {
        session.checkpoints.push(checkpoint.checkpoint_name.clone());
        Ok(())
    }

    fn capture_artifacts(
        &self,
        session: &Self::Session,
        request: &WorkflowCaptureRequest,
    ) -> Result<Vec<ArtifactBundle>, Self::Error> {
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
                                    "desk": Self::version_summary(&audit.desk),
                                    "scenario": Self::version_summary(&audit.scenario),
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
                        .map(|(alias, replay)| (alias.clone(), Self::replay_summary(replay)))
                        .collect::<BTreeMap<_, _>>();
                    let lineage = session
                        .named_lineages
                        .iter()
                        .map(|(alias, lineage)| {
                            (alias.clone(), Self::lineage_summary(lineage.as_slice()))
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

    fn run_invariants(
        &self,
        session: &Self::Session,
        _boundary: WorkflowState,
        checks: &[InvariantCheck],
    ) -> Result<Vec<InvariantReport>, Self::Error> {
        checks
            .iter()
            .map(|check| self.check_invariant(session, check))
            .collect()
    }

    fn serialize_reproduction(
        &self,
        session: &Self::Session,
        failure: &WorkflowFailureContext,
    ) -> Result<ReproductionMetadata, Self::Error> {
        Ok(ReproductionMetadata {
            format: "json".to_string(),
            payload: json!({
                "state": format!("{:?}", failure.state),
                "step_index": failure.step_index,
                "failure_injection": failure.failure_injection.as_ref().map(|injection| {
                    json!({
                        "boundary": format!("{:?}", injection.boundary),
                        "location": injection.location,
                        "detail": injection.detail,
                    })
                }),
                "current_branch": session.world.current_branch().name,
                "known_branches": session.named_branches.keys().collect::<Vec<_>>(),
                "known_snapshots": session.named_snapshots.keys().collect::<Vec<_>>(),
                "known_audits": session.named_audits.keys().collect::<Vec<_>>(),
                "policy": format!("{:?}", session.policy.tier),
            })
            .to_string(),
        })
    }
}

fn certified_step(
    name: impl Into<String>,
    operation: FintechWorkflowStep,
) -> WorkflowStep<FintechWorkflowStep> {
    WorkflowStep::new(name, operation)
        .capture_at(WorkflowState::Inspected)
        .inspect_at(WorkflowState::Inspected)
}

fn checkpoint_step(
    name: impl Into<String>,
    operation: FintechWorkflowStep,
) -> WorkflowStep<FintechWorkflowStep> {
    certified_step(name, operation)
        .checkpoint_after()
        .capture_at(WorkflowState::Checkpointed)
}

fn hostile_branch_replay_and_audit_plan() -> WorkflowPlan<FintechWorkflowStep> {
    WorkflowPlan::new(
        workflow_names::HOSTILE_BRANCH_REPLAY_AUDIT,
        "intraday-pricing-and-risk",
        "forge-signal",
        "fintech",
    )
    .with_seed(7)
    .with_regression_target(RegressionTarget {
        kind: RegressionTargetKind::ExpectedFailure,
        issue_key: "signal-workflow-certification-bootstrap".to_string(),
        summary: "Bootstrap hostile fintech certification through the new workflow runner"
            .to_string(),
        reproduction_hint: None,
    })
    .step(certified_step(
        "seed-main-regime",
        FintechWorkflowStep::SeedRegime {
            regime: MarketRegime::Calm,
            seed: 7,
        },
    ))
    .step(certified_step(
        "read-main-audit-surface",
        FintechWorkflowStep::ReadPrimaryAuditSurface {
            alias: artifact_aliases::BASELINE_AUDIT,
        },
    ))
    .step(checkpoint_step(
        "capture-main-snapshot",
        FintechWorkflowStep::CaptureActiveSnapshot {
            alias: artifact_aliases::MAIN_SNAPSHOT,
        },
    ))
    .step(certified_step(
        "open-analysis-branch",
        FintechWorkflowStep::OpenBranch {
            branch_name: "analysis-risk",
            alias: artifact_aliases::ANALYSIS_BRANCH,
        },
    ))
    .step(certified_step(
        "seed-analysis-regime",
        FintechWorkflowStep::SeedRegime {
            regime: MarketRegime::HighVol,
            seed: 17,
        },
    ))
    .step(certified_step(
        "read-analysis-audit-surface",
        FintechWorkflowStep::ReadPrimaryAuditSurface {
            alias: artifact_aliases::ANALYSIS_AUDIT,
        },
    ))
    .step(checkpoint_step(
        "capture-analysis-snapshot",
        FintechWorkflowStep::CaptureActiveSnapshot {
            alias: artifact_aliases::ANALYSIS_SNAPSHOT,
        },
    ))
    .step(certified_step(
        "capture-analysis-replay-before",
        FintechWorkflowStep::CaptureReplay {
            branch_alias: artifact_aliases::ANALYSIS_BRANCH,
            alias: artifact_aliases::ANALYSIS_REPLAY_BEFORE,
        },
    ))
    .step(
        certified_step(
            "inject-analysis-rollback",
            FintechWorkflowStep::InjectSyntheticRollback,
        )
        .capture_at(WorkflowState::Failed)
        .with_failure_injection(FailureInjectionPoint {
            boundary: WorkflowState::StepApplied,
            location: "analysis synthetic rollback".to_string(),
            detail: Some("branch-local failure injection during hostile correction".to_string()),
        }),
    )
    .step(certified_step(
        "capture-analysis-replay-after",
        FintechWorkflowStep::CaptureReplay {
            branch_alias: artifact_aliases::ANALYSIS_BRANCH,
            alias: artifact_aliases::ANALYSIS_REPLAY_AFTER,
        },
    ))
    .step(certified_step(
        "restore-analysis-snapshot",
        FintechWorkflowStep::RestoreSnapshot {
            branch_alias: artifact_aliases::ANALYSIS_BRANCH,
            snapshot_alias: artifact_aliases::ANALYSIS_SNAPSHOT,
        },
    ))
    .step(certified_step(
        "read-restored-analysis-audit-surface",
        FintechWorkflowStep::ReadPrimaryAuditSurface {
            alias: artifact_aliases::RESTORED_ANALYSIS_AUDIT,
        },
    ))
    .step(certified_step(
        "open-correction-branch",
        FintechWorkflowStep::OpenBranch {
            branch_name: "correction",
            alias: artifact_aliases::CORRECTION_BRANCH,
        },
    ))
    .step(certified_step(
        "seed-correction-regime",
        FintechWorkflowStep::SeedRegime {
            regime: MarketRegime::FxDislocation,
            seed: 29,
        },
    ))
    .step(certified_step(
        "read-correction-audit-surface",
        FintechWorkflowStep::ReadPrimaryAuditSurface {
            alias: "correction_audit",
        },
    ))
    .step(certified_step(
        "switch-main",
        FintechWorkflowStep::SwitchBranch {
            alias: artifact_aliases::MAIN_BRANCH,
        },
    ))
    .step(certified_step(
        "restore-main-snapshot",
        FintechWorkflowStep::RestoreSnapshot {
            branch_alias: artifact_aliases::MAIN_BRANCH,
            snapshot_alias: artifact_aliases::MAIN_SNAPSHOT,
        },
    ))
    .step(certified_step(
        "read-restored-main-audit-surface",
        FintechWorkflowStep::ReadPrimaryAuditSurface {
            alias: artifact_aliases::RESTORED_MAIN_AUDIT,
        },
    ))
    .step(certified_step(
        "capture-main-replay",
        FintechWorkflowStep::CaptureReplay {
            branch_alias: artifact_aliases::MAIN_BRANCH,
            alias: artifact_aliases::MAIN_REPLAY,
        },
    ))
    .step(certified_step(
        "capture-correction-replay",
        FintechWorkflowStep::CaptureReplay {
            branch_alias: artifact_aliases::CORRECTION_BRANCH,
            alias: artifact_aliases::CORRECTION_REPLAY,
        },
    ))
    .step(certified_step(
        "capture-analysis-replay-around-snapshot",
        FintechWorkflowStep::CaptureReplayAroundSnapshot {
            snapshot_alias: artifact_aliases::ANALYSIS_SNAPSHOT,
            alias: artifact_aliases::ANALYSIS_AROUND_SNAPSHOT,
        },
    ))
    .step(certified_step(
        "switch-correction",
        FintechWorkflowStep::SwitchBranch {
            alias: artifact_aliases::CORRECTION_BRANCH,
        },
    ))
    .step(certified_step(
        "capture-correction-lineage",
        FintechWorkflowStep::CaptureMainRiskLineage {
            alias: artifact_aliases::CORRECTION_LINEAGE,
        },
    ))
    .invariant(InvariantCheck::new(
        invariant_names::ANALYSIS_RESTORE_MATCHES,
        "analysis snapshot restore should preserve branch-local desk truth",
        WorkflowState::Completed,
    ))
    .invariant(InvariantCheck::new(
        invariant_names::MAIN_RESTORE_MATCHES,
        "main snapshot restore should preserve baseline desk truth",
        WorkflowState::Completed,
    ))
    .invariant(InvariantCheck::new(
        invariant_names::ANALYSIS_REPLAY_HAS_ROLLBACK,
        "analysis replay should retain rollback evidence",
        WorkflowState::Completed,
    ))
    .invariant(InvariantCheck::new(
        invariant_names::MAIN_REPLAY_BRANCH_LOCAL,
        "main replay should stay branch-local",
        WorkflowState::Completed,
    ))
    .invariant(InvariantCheck::new(
        invariant_names::CORRECTION_REPLAY_HAS_BRANCH_SWITCH,
        "correction replay should preserve branch activation",
        WorkflowState::Completed,
    ))
    .invariant(InvariantCheck::new(
        invariant_names::CORRECTION_LINEAGE_HAS_RECOVERY,
        "correction lineage should preserve risk evolution events",
        WorkflowState::Completed,
    ))
    .invariant(InvariantCheck::new(
        invariant_names::MAIN_BRANCH_HEAD_MATCHES,
        "main branch should retain its head snapshot metadata",
        WorkflowState::Completed,
    ))
    .invariant(InvariantCheck::new(
        invariant_names::ANALYSIS_REPLAY_MENTIONS_SNAPSHOT,
        "replay around snapshot should reference the saved analysis snapshot",
        WorkflowState::Completed,
    ))
}

fn development_serial_profile() -> WorkflowRuntimeProfile {
    WorkflowRuntimeProfile {
        runtime_profile: "fintech-development".to_string(),
        policy_name: Some("fintech".to_string()),
        executor_name: Some("serial".to_string()),
        capability_profile: Some("serial-vs-parallel-hostile".to_string()),
    }
}

#[cfg(feature = "parallel")]
fn development_parallel_profile() -> WorkflowRuntimeProfile {
    WorkflowRuntimeProfile {
        runtime_profile: "fintech-development".to_string(),
        policy_name: Some("fintech".to_string()),
        executor_name: Some("aggressive-parallel".to_string()),
        capability_profile: Some("serial-vs-parallel-hostile".to_string()),
    }
}

#[cfg(feature = "parallel")]
fn compare_signal_fintech_overlap(
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

#[test]
fn workflow_certification_runner_proves_hostile_fintech_branch_replay_and_audit() {
    let runner = WorkflowCertificationRunner::new(SignalFintechWorkflowCertificationAdapter);
    let plan = hostile_branch_replay_and_audit_plan();
    let report = runner
        .certify(&plan, &development_serial_profile())
        .unwrap();

    assert_eq!(report.session.state, WorkflowState::Completed);
    assert!(report.failure_bundle.is_none());
    assert!(report
        .session
        .session_data
        .named_replays
        .contains_key("analysis_replay_after"));
    assert!(report
        .session
        .session_data
        .named_lineages
        .contains_key("correction_lineage"));
}

#[cfg(feature = "parallel")]
#[test]
fn workflow_certification_runner_keeps_serial_parallel_fintech_overlap_honest() {
    let runner = WorkflowCertificationRunner::new(SignalFintechWorkflowCertificationAdapter);
    let plan = hostile_branch_replay_and_audit_plan();
    let serial = runner
        .certify(&plan, &development_serial_profile())
        .unwrap();
    let parallel = runner
        .certify(&plan, &development_parallel_profile())
        .unwrap();

    assert_eq!(serial.session.state, WorkflowState::Completed);
    assert_eq!(parallel.session.state, WorkflowState::Completed);

    let outcome = compare_signal_fintech_overlap(&serial, &parallel);
    assert!(
        outcome.matched,
        "serial-vs-parallel fintech overlap drift: {:?}",
        outcome.mismatches
    );
}
