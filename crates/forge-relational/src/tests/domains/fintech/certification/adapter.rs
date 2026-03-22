use std::collections::{BTreeMap, BTreeSet};

use forge_harness::facade::{
    AdapterSupport, ArtifactBundle, ArtifactSurface, CheckpointSemantics,
    DifferentialMatrixCapability, FailureInjectionPoint, InvariantCheck, InvariantReport,
    ProfileConditionalGuarantee, ReproductionMetadata, UnsupportedWorkflowComparison,
    WorkflowArtifactSurfaceCapability, WorkflowCaptureRequest, WorkflowCertificationAdapter,
    WorkflowCertificationCapabilities, WorkflowCheckpoint, WorkflowFailureContext, WorkflowPlan,
    WorkflowRuntimeProfile, WorkflowState, WorkflowStep, WorkflowStepOutcome,
};
use serde_json::json;

use crate::facade::history::BranchId;
use crate::facade::replay::{
    RelationalReplayRequest, ReplayExecutionMode, ReplayVerificationMode,
};

use super::super::actions::{
    correct_seeded_trade_candidate, open_analysis_branch, refresh_risk_views,
    repair_seeded_failed_settlement, shock_market_on_branch, stress_seeded_intraday_risk,
};
use super::super::fixture::FintechWorkflowCase;
use super::super::scenarios::{setup_world_for, FintechScenario};
use super::artifacts::{capture_artifacts, case_read_summary, read_summary};
use super::invariants::run_checks;
use super::session::CertifiedRelationalFintechSession;
use super::steps::{FintechCaseRef, FintechWorkflowStep};

#[derive(Debug, Default, Clone, Copy)]
pub(super) struct RelationalFintechWorkflowCertificationAdapter;

impl RelationalFintechWorkflowCertificationAdapter {
    fn case_for(
        session: &CertifiedRelationalFintechSession,
        case: FintechCaseRef,
    ) -> FintechWorkflowCase {
        match case {
            FintechCaseRef::LateTradeCorrection => session.world.late_trade_correction_case(),
            FintechCaseRef::IntradayRisk => session.world.intraday_risk_case(),
            FintechCaseRef::FailedSettlementRepair => session.world.failed_settlement_repair_case(),
        }
    }
}

impl WorkflowCertificationAdapter for RelationalFintechWorkflowCertificationAdapter {
    type Session = CertifiedRelationalFintechSession;
    type Step = FintechWorkflowStep;
    type Error = String;

    fn adapter_name(&self) -> &'static str {
        "forge-relational-fintech-workflow"
    }

    fn capabilities(&self) -> WorkflowCertificationCapabilities {
        WorkflowCertificationCapabilities {
            artifact_surfaces: vec![
                WorkflowArtifactSurfaceCapability {
                    surface: ArtifactSurface::SnapshotVisibleTruth,
                    profiles: BTreeSet::from(["fintech-development".to_string()]),
                },
                WorkflowArtifactSurfaceCapability {
                    surface: ArtifactSurface::BranchHeadState,
                    profiles: BTreeSet::from(["fintech-development".to_string()]),
                },
                WorkflowArtifactSurfaceCapability {
                    surface: ArtifactSurface::ReplayRecoveryTruthState,
                    profiles: BTreeSet::from(["fintech-development".to_string()]),
                },
                WorkflowArtifactSurfaceCapability {
                    surface: ArtifactSurface::Diagnostics,
                    profiles: BTreeSet::from(["fintech-development".to_string()]),
                },
                WorkflowArtifactSurfaceCapability {
                    surface: ArtifactSurface::PatchChangeSurface,
                    profiles: BTreeSet::from(["fintech-development".to_string()]),
                },
                WorkflowArtifactSurfaceCapability {
                    surface: ArtifactSurface::StepTrace,
                    profiles: BTreeSet::from(["fintech-development".to_string()]),
                },
                WorkflowArtifactSurfaceCapability {
                    surface: ArtifactSurface::CheckpointTrace,
                    profiles: BTreeSet::from(["fintech-development".to_string()]),
                },
                WorkflowArtifactSurfaceCapability {
                    surface: ArtifactSurface::ComplexityCounters,
                    profiles: BTreeSet::from(["fintech-development".to_string()]),
                },
                WorkflowArtifactSurfaceCapability {
                    surface: ArtifactSurface::BudgetOutcome,
                    profiles: BTreeSet::from(["fintech-development".to_string()]),
                },
            ],
            checkpoint_semantics: BTreeSet::from([CheckpointSemantics::AdapterDefined]),
            replay_recovery_support: BTreeSet::from([
                ArtifactSurface::BranchHeadState,
                ArtifactSurface::ReplayRecoveryTruthState,
                ArtifactSurface::SnapshotVisibleTruth,
            ]),
            differential_matrices: vec![DifferentialMatrixCapability {
                matrix_name: "relational-fintech-baseline".to_string(),
                profiles: BTreeSet::from(["fintech-development".to_string()]),
                guaranteed_surfaces: BTreeSet::from([
                    ArtifactSurface::BranchHeadState,
                    ArtifactSurface::ReplayRecoveryTruthState,
                    ArtifactSurface::SnapshotVisibleTruth,
                ]),
            }],
            unsupported_comparisons: vec![UnsupportedWorkflowComparison {
                surface: ArtifactSurface::ComplexityCounters,
                reason: "relational fintech workflow certification exposes raw counters but does not yet compare them across profiles"
                    .to_string(),
            }],
            profile_guarantees: vec![ProfileConditionalGuarantee {
                profile_name: "fintech-development".to_string(),
                guarantee:
                    "baseline branch/snapshot/replay artifacts stay crate-local and workflow-driven"
                        .to_string(),
            }],
            budget_artifacts: AdapterSupport::Supported,
        }
    }

    fn initialize_session(
        &self,
        _plan: &WorkflowPlan<Self::Step>,
        _profile: &WorkflowRuntimeProfile,
    ) -> Result<Self::Session, Self::Error> {
        Ok(CertifiedRelationalFintechSession {
            world: setup_world_for(FintechScenario::SmokeBook),
            named_branches: BTreeMap::from([("main".to_string(), BranchId("main".to_string()))]),
            named_snapshots: BTreeMap::new(),
            named_reads: BTreeMap::new(),
            named_replays: BTreeMap::new(),
            executed_steps: Vec::new(),
            checkpoints: Vec::new(),
        })
    }

    fn execute_step(
        &self,
        session: &mut Self::Session,
        step: &WorkflowStep<Self::Step>,
        _injection: Option<&FailureInjectionPoint>,
    ) -> Result<WorkflowStepOutcome, Self::Error> {
        session.executed_steps.push(step.name.clone());
        match &step.operation {
            FintechWorkflowStep::CaptureMainSnapshot { alias } => {
                let snapshot = session.world.runtime.visibility_authority().snapshot();
                session
                    .named_snapshots
                    .insert((*alias).to_string(), snapshot);
                Ok(WorkflowStepOutcome {
                    detail: Some(format!("captured main snapshot `{alias}`")),
                    request_checkpoint: true,
                })
            }
            FintechWorkflowStep::OpenAnalysisBranch { alias } => {
                let branch = open_analysis_branch(&mut session.world);
                session.named_branches.insert((*alias).to_string(), branch);
                Ok(WorkflowStepOutcome::applied())
            }
            FintechWorkflowStep::ShockMarket { branch_alias } => {
                let branch = session.branch(branch_alias)?;
                shock_market_on_branch(&mut session.world, branch);
                Ok(WorkflowStepOutcome::applied())
            }
            FintechWorkflowStep::CorrectCaseTrade { branch_alias, case } => {
                let branch = session.branch(branch_alias)?;
                match case {
                    FintechCaseRef::LateTradeCorrection => {
                        correct_seeded_trade_candidate(&mut session.world, branch);
                    }
                    other => {
                        return Err(format!(
                            "unsupported trade-correction case for certification: {other:?}"
                        ))
                    }
                }
                Ok(WorkflowStepOutcome::applied())
            }
            FintechWorkflowStep::StressCaseRisk { branch_alias, case } => {
                let branch = session.branch(branch_alias)?;
                match case {
                    FintechCaseRef::IntradayRisk => {
                        stress_seeded_intraday_risk(&mut session.world, branch);
                    }
                    other => {
                        return Err(format!(
                            "unsupported risk-stress case for certification: {other:?}"
                        ))
                    }
                }
                Ok(WorkflowStepOutcome::applied())
            }
            FintechWorkflowStep::RepairCaseSettlement { branch_alias, case } => {
                let branch = session.branch(branch_alias)?;
                match case {
                    FintechCaseRef::FailedSettlementRepair => {
                        repair_seeded_failed_settlement(&mut session.world, branch);
                    }
                    other => {
                        return Err(format!(
                            "unsupported settlement-repair case for certification: {other:?}"
                        ))
                    }
                }
                Ok(WorkflowStepOutcome::applied())
            }
            FintechWorkflowStep::RefreshRisk { branch_alias } => {
                let branch = session.branch(branch_alias)?;
                refresh_risk_views(&mut session.world, branch);
                Ok(WorkflowStepOutcome::applied())
            }
            FintechWorkflowStep::ReadSnapshot {
                snapshot_alias,
                read_alias,
            } => {
                let snapshot = session.snapshot(snapshot_alias)?;
                let summary = read_summary(session, snapshot)?;
                session
                    .named_reads
                    .insert((*read_alias).to_string(), summary);
                Ok(WorkflowStepOutcome::applied())
            }
            FintechWorkflowStep::ReadCaseProbe { case, read_alias } => {
                let workflow_case = Self::case_for(session, *case);
                session.named_reads.insert(
                    (*read_alias).to_string(),
                    case_read_summary(session, workflow_case.role),
                );
                Ok(WorkflowStepOutcome::applied())
            }
            FintechWorkflowStep::CaptureReplay {
                branch_alias,
                alias,
            } => {
                let branch = session.branch(branch_alias)?;
                let commit_id = session
                    .world
                    .runtime
                    .history_access()
                    .latest_commit()
                    .ok_or_else(|| "latest commit unavailable for replay capture".to_string())?
                    .commit_id;
                let replay = session.world.runtime.replay_authority().replay_commit(
                    RelationalReplayRequest {
                        commit_id,
                        branch_id: branch,
                        execution_mode: ReplayExecutionMode::SerialDeterministic,
                        verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
                    },
                );
                session.named_replays.insert((*alias).to_string(), replay);
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
        session
            .world
            .runtime
            .durability_authority()
            .checkpoint()
            .map(|_| ())
            .map_err(|error| error.detail)
    }

    fn capture_artifacts(
        &self,
        session: &Self::Session,
        request: &WorkflowCaptureRequest,
    ) -> Result<Vec<ArtifactBundle>, Self::Error> {
        Ok(capture_artifacts(session, request))
    }

    fn run_invariants(
        &self,
        session: &Self::Session,
        _boundary: WorkflowState,
        checks: &[InvariantCheck],
    ) -> Result<Vec<InvariantReport>, Self::Error> {
        run_checks(session, checks)
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
                "known_branches": session.named_branches.keys().collect::<Vec<_>>(),
                "known_snapshots": session.named_snapshots.keys().collect::<Vec<_>>(),
                "known_reads": session.named_reads.keys().collect::<Vec<_>>(),
                "known_replays": session.named_replays.keys().collect::<Vec<_>>(),
            })
            .to_string(),
        })
    }
}
