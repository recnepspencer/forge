use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde_json::json;

use crate::runtime::AdapterSupport;

use super::adapter::WorkflowCertificationAdapter;
use super::capability::{
    ProfileConditionalGuarantee, WorkflowArtifactSurfaceCapability,
    WorkflowCertificationCapabilities,
};
use super::record::{
    ArtifactBundle, ArtifactClass, ArtifactSurface, CheckpointSemantics, InvariantCheck,
    InvariantReport, ReproductionMetadata, WorkflowCaptureRequest, WorkflowCheckpoint,
    WorkflowFailureContext, WorkflowPlan, WorkflowRuntimeProfile, WorkflowState, WorkflowStep,
    WorkflowStepOutcome,
};
use super::runner::WorkflowCertificationRunner;

#[derive(Debug, Clone, PartialEq, Eq)]
struct TestError(&'static str);

impl fmt::Display for TestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for TestError {}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SessionDouble {
    values: Vec<String>,
    checkpoints: usize,
}

struct AdapterDouble;

impl WorkflowCertificationAdapter for AdapterDouble {
    type Session = SessionDouble;
    type Step = &'static str;
    type Error = TestError;

    fn adapter_name(&self) -> &'static str {
        "adapter-double"
    }

    fn capabilities(&self) -> WorkflowCertificationCapabilities {
        WorkflowCertificationCapabilities {
            artifact_surfaces: vec![WorkflowArtifactSurfaceCapability {
                surface: ArtifactSurface::StepTrace,
                profiles: BTreeSet::from(["forensic".to_string()]),
            }],
            checkpoint_semantics: BTreeSet::from([CheckpointSemantics::AdapterDefined]),
            replay_recovery_support: BTreeSet::new(),
            differential_matrices: Vec::new(),
            unsupported_comparisons: Vec::new(),
            profile_guarantees: vec![ProfileConditionalGuarantee {
                profile_name: "forensic".to_string(),
                guarantee: "step trace capture".to_string(),
            }],
            budget_artifacts: AdapterSupport::Unsupported,
        }
    }

    fn initialize_session(
        &self,
        _plan: &WorkflowPlan<Self::Step>,
        _profile: &WorkflowRuntimeProfile,
    ) -> Result<Self::Session, Self::Error> {
        Ok(SessionDouble {
            values: Vec::new(),
            checkpoints: 0,
        })
    }

    fn execute_step(
        &self,
        session: &mut Self::Session,
        step: &WorkflowStep<Self::Step>,
        _injection: Option<&super::record::FailureInjectionPoint>,
    ) -> Result<WorkflowStepOutcome, Self::Error> {
        if step.operation == "fail" {
            return Err(TestError("step failed"));
        }
        session.values.push(step.operation.to_string());
        Ok(WorkflowStepOutcome::applied())
    }

    fn create_checkpoint(
        &self,
        session: &mut Self::Session,
        _checkpoint: &WorkflowCheckpoint,
    ) -> Result<(), Self::Error> {
        session.checkpoints += 1;
        Ok(())
    }

    fn capture_artifacts(
        &self,
        session: &Self::Session,
        request: &WorkflowCaptureRequest,
    ) -> Result<Vec<ArtifactBundle>, Self::Error> {
        Ok(vec![ArtifactBundle {
            artifact_class: ArtifactClass::Forensic,
            surface: ArtifactSurface::StepTrace,
            name: "trace".to_string(),
            boundary: request.boundary,
            payload: json!({
                "step_index": request.step_index,
                "values": session.values,
            }),
            attachments: Vec::new(),
            metadata: BTreeMap::new(),
        }])
    }

    fn run_invariants(
        &self,
        session: &Self::Session,
        boundary: WorkflowState,
        checks: &[InvariantCheck],
    ) -> Result<Vec<InvariantReport>, Self::Error> {
        Ok(checks
            .iter()
            .map(|check| InvariantReport {
                check_id: check.check_id.clone(),
                boundary,
                passed: !session.values.iter().any(|value| value == "bad"),
                detail: "checked".to_string(),
                fields: BTreeMap::new(),
            })
            .collect())
    }

    fn serialize_reproduction(
        &self,
        session: &Self::Session,
        _failure: &WorkflowFailureContext,
    ) -> Result<ReproductionMetadata, Self::Error> {
        Ok(ReproductionMetadata {
            format: "json".to_string(),
            payload: json!({ "values": session.values }).to_string(),
        })
    }
}

#[test]
fn workflow_state_machine_rejects_completed_to_step_applied() {
    assert!(!WorkflowState::Completed.can_transition_to(WorkflowState::StepApplied));
}

#[test]
fn certification_runner_enforces_step_checkpoint_inspect_completion_flow() {
    let runner = WorkflowCertificationRunner::new(AdapterDouble);
    let plan = WorkflowPlan::new("workflow", "scenario", "worth-signal", "fintech")
        .step(
            WorkflowStep::new("seed", "seed")
                .checkpoint_after()
                .capture_at(WorkflowState::StepApplied)
                .capture_at(WorkflowState::Checkpointed)
                .capture_at(WorkflowState::Inspected)
                .inspect_at(WorkflowState::Inspected),
        )
        .invariant(InvariantCheck::new(
            "post-step",
            "step should remain coherent",
            WorkflowState::Inspected,
        ));
    let profile = WorkflowRuntimeProfile::new("forensic");

    let report = runner.certify(&plan, &profile).unwrap();

    assert_eq!(report.session.state, WorkflowState::Completed);
    assert_eq!(report.session.session_data.checkpoints, 1);
    assert_eq!(report.session.step_trace.len(), 2);
    assert!(report.failure_bundle.is_none());
}

#[test]
fn certification_runner_emits_failure_bundle_for_invariant_failure() {
    let runner = WorkflowCertificationRunner::new(AdapterDouble);
    let plan = WorkflowPlan::new("workflow", "scenario", "worth-relational", "fintech")
        .with_seed(7)
        .step(
            WorkflowStep::new("mutate", "bad")
                .capture_at(WorkflowState::Inspected)
                .inspect_at(WorkflowState::Inspected),
        )
        .invariant(InvariantCheck::new(
            "post-step",
            "bad values are rejected",
            WorkflowState::Inspected,
        ));
    let profile = WorkflowRuntimeProfile::new("forensic");

    let report = runner.certify(&plan, &profile).unwrap();

    assert_eq!(report.session.state, WorkflowState::Failed);
    assert!(report.failure_bundle.is_some());
    assert_eq!(
        report.failure_bundle.unwrap().version,
        super::record::FailureBundleVersion::V1
    );
}
