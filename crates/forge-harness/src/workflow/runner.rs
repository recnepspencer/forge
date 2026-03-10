use super::adapter::{WorkflowCertificationAdapter, WorkflowCertificationError};
use super::record::{
    ArtifactBundle, CheckpointSemantics, FailureBundle, FailureBundleVersion,
    FailureInjectionPoint, InvariantReport, WorkflowCaptureRequest,
    WorkflowCertificationReport, WorkflowCheckpoint, WorkflowCheckpointTraceEntry,
    WorkflowFailureContext, WorkflowPlan, WorkflowRuntimeProfile, WorkflowSession, WorkflowState,
    WorkflowStep, WorkflowStepTraceEntry,
};

pub struct WorkflowCertificationRunner<A> {
    adapter: A,
}

impl<A> WorkflowCertificationRunner<A> {
    pub fn new(adapter: A) -> Self {
        Self { adapter }
    }

    pub fn adapter(&self) -> &A {
        &self.adapter
    }
}

impl<A> WorkflowCertificationRunner<A>
where
    A: WorkflowCertificationAdapter,
{
    fn transition<SessionData>(
        session: &mut WorkflowSession<SessionData>,
        next: WorkflowState,
    ) -> Result<(), WorkflowCertificationError<A::Error>> {
        if !session.state.can_transition_to(next) {
            return Err(WorkflowCertificationError::InvalidStateTransition {
                from: session.state,
                to: next,
            });
        }
        session.state = next;
        Ok(())
    }

    fn capture_requested_artifacts(
        &self,
        raw_session: &A::Session,
        step_index: Option<usize>,
        step: Option<&WorkflowStep<A::Step>>,
        boundary: WorkflowState,
    ) -> Result<Vec<ArtifactBundle>, WorkflowCertificationError<A::Error>> {
        let requested_surfaces = self
            .adapter
            .capabilities()
            .artifact_surfaces
            .into_iter()
            .map(|capability| capability.surface)
            .collect();
        let should_capture = step
            .map(|step| step.capture_boundaries.contains(&boundary))
            .unwrap_or(boundary == WorkflowState::Completed || boundary == WorkflowState::Failed);
        if !should_capture {
            return Ok(Vec::new());
        }
        self.adapter
            .capture_artifacts(
                raw_session,
                &WorkflowCaptureRequest {
                    step_index,
                    boundary,
                    requested_surfaces,
                },
            )
            .map_err(WorkflowCertificationError::Adapter)
    }

    fn run_requested_invariants(
        &self,
        raw_session: &A::Session,
        plan: &WorkflowPlan<A::Step>,
        step: Option<&WorkflowStep<A::Step>>,
        boundary: WorkflowState,
    ) -> Result<Vec<InvariantReport>, WorkflowCertificationError<A::Error>> {
        let checks: Vec<_> = plan
            .invariants
            .iter()
            .filter(|check| {
                check.boundary == boundary
                    && step.map(|step| step.invariant_boundaries.contains(&boundary)).unwrap_or(
                        boundary == WorkflowState::Completed || boundary == WorkflowState::Failed,
                    )
            })
            .cloned()
            .collect();
        if checks.is_empty() {
            return Ok(Vec::new());
        }
        self.adapter
            .run_invariants(raw_session, boundary, &checks)
            .map_err(WorkflowCertificationError::Adapter)
    }

    fn build_failure_bundle(
        &self,
        raw_session: &A::Session,
        session: &WorkflowSession<A::Session>,
        plan: &WorkflowPlan<A::Step>,
        profile: &WorkflowRuntimeProfile,
        step_index: Option<usize>,
        injection: Option<FailureInjectionPoint>,
    ) -> Result<FailureBundle, WorkflowCertificationError<A::Error>> {
        let invariant_failures = session
            .invariant_reports
            .iter()
            .filter(|report| !report.passed)
            .cloned()
            .collect();
        let reproduction = self
            .adapter
            .serialize_reproduction(
                raw_session,
                &WorkflowFailureContext {
                    step_index,
                    state: session.state,
                    failure_injection: injection.clone(),
                },
            )
            .map_err(WorkflowCertificationError::Adapter)?;
        Ok(FailureBundle {
            version: FailureBundleVersion::V1,
            crate_name: plan.crate_name.clone(),
            domain_name: plan.domain_name.clone(),
            workflow_name: plan.workflow_name.clone(),
            scenario_name: plan.scenario_name.clone(),
            seed: plan.seed,
            runtime_profile: profile.runtime_profile.clone(),
            policy_name: profile.policy_name.clone(),
            executor_name: profile.executor_name.clone(),
            step_trace: session.step_trace.clone(),
            checkpoint_trace: session.checkpoint_trace.clone(),
            failure_injection_point: injection,
            invariant_failures,
            artifact_diffs: Vec::new(),
            reproduction,
        })
    }

    pub fn certify(
        &self,
        plan: &WorkflowPlan<A::Step>,
        profile: &WorkflowRuntimeProfile,
    ) -> Result<WorkflowCertificationReport<A::Session>, WorkflowCertificationError<A::Error>> {
        let raw_session = self
            .adapter
            .initialize_session(plan, profile)
            .map_err(WorkflowCertificationError::Adapter)?;
        let mut session = WorkflowSession {
            adapter_name: self.adapter.adapter_name().to_string(),
            workflow_name: plan.workflow_name.clone(),
            scenario_name: plan.scenario_name.clone(),
            state: WorkflowState::Initialized,
            next_step_index: 0,
            step_trace: Vec::new(),
            checkpoint_trace: Vec::new(),
            artifacts: Vec::new(),
            invariant_reports: Vec::new(),
            session_data: raw_session,
        };

        for (step_index, step) in plan.steps.iter().enumerate() {
            let injection = step.failure_injection.clone();
            let outcome = match self
                .adapter
                .execute_step(&mut session.session_data, step, injection.as_ref())
            {
                Ok(outcome) => outcome,
                Err(_) => {
                    session.state = WorkflowState::Failed;
                    let artifacts = self.capture_requested_artifacts(
                        &session.session_data,
                        Some(step_index),
                        Some(step),
                        WorkflowState::Failed,
                    )?;
                    session.artifacts.extend(artifacts);
                    let reports = self.run_requested_invariants(
                        &session.session_data,
                        plan,
                        Some(step),
                        WorkflowState::Failed,
                    )?;
                    session.invariant_reports.extend(reports);
                    let failure_bundle = self.build_failure_bundle(
                        &session.session_data,
                        &session,
                        plan,
                        profile,
                        Some(step_index),
                        injection,
                    )?;
                    return Ok(WorkflowCertificationReport {
                        session,
                        failure_bundle: Some(failure_bundle),
                    });
                }
            };

            Self::transition(&mut session, WorkflowState::StepApplied)?;
            session.step_trace.push(WorkflowStepTraceEntry {
                step_index,
                step_name: step.name.clone(),
                state: WorkflowState::StepApplied,
                detail: outcome.detail.clone(),
            });
            let artifacts = self.capture_requested_artifacts(
                &session.session_data,
                Some(step_index),
                Some(step),
                WorkflowState::StepApplied,
            )?;
            session.artifacts.extend(artifacts);

            if step.checkpoint_after || outcome.request_checkpoint {
                let checkpoint = WorkflowCheckpoint::new(
                    format!("{}-checkpoint", step.name),
                    CheckpointSemantics::AdapterDefined,
                    step_index,
                );
                if self
                    .adapter
                    .create_checkpoint(&mut session.session_data, &checkpoint)
                    .is_err()
                {
                    session.state = WorkflowState::Failed;
                    let failure_bundle = self.build_failure_bundle(
                        &session.session_data,
                        &session,
                        plan,
                        profile,
                        Some(step_index),
                        injection,
                    )?;
                    return Ok(WorkflowCertificationReport {
                        session,
                        failure_bundle: Some(failure_bundle),
                    });
                }
                Self::transition(&mut session, WorkflowState::Checkpointed)?;
                session.checkpoint_trace.push(WorkflowCheckpointTraceEntry {
                    step_index,
                    checkpoint_name: checkpoint.checkpoint_name,
                    semantics: checkpoint.semantics,
                });
                let artifacts = self.capture_requested_artifacts(
                    &session.session_data,
                    Some(step_index),
                    Some(step),
                    WorkflowState::Checkpointed,
                )?;
                session.artifacts.extend(artifacts);
            }

            Self::transition(&mut session, WorkflowState::Inspected)?;
            session.step_trace.push(WorkflowStepTraceEntry {
                step_index,
                step_name: step.name.clone(),
                state: WorkflowState::Inspected,
                detail: None,
            });
            let artifacts = self.capture_requested_artifacts(
                &session.session_data,
                Some(step_index),
                Some(step),
                WorkflowState::Inspected,
            )?;
            session.artifacts.extend(artifacts);
            let reports = self.run_requested_invariants(
                &session.session_data,
                plan,
                Some(step),
                WorkflowState::Inspected,
            )?;
            session.invariant_reports.extend(reports.clone());
            if reports.iter().any(|report| !report.passed) {
                session.state = WorkflowState::Failed;
                let artifacts = self.capture_requested_artifacts(
                    &session.session_data,
                    Some(step_index),
                    Some(step),
                    WorkflowState::Failed,
                )?;
                session.artifacts.extend(artifacts);
                let failed_reports = self.run_requested_invariants(
                    &session.session_data,
                    plan,
                    Some(step),
                    WorkflowState::Failed,
                )?;
                session.invariant_reports.extend(failed_reports);
                let failure_bundle = self.build_failure_bundle(
                    &session.session_data,
                    &session,
                    plan,
                    profile,
                    Some(step_index),
                    injection,
                )?;
                return Ok(WorkflowCertificationReport {
                    session,
                    failure_bundle: Some(failure_bundle),
                });
            }
            session.next_step_index = step_index + 1;
        }

        Self::transition(&mut session, WorkflowState::Completed)?;
        let artifacts = self.capture_requested_artifacts(
            &session.session_data,
            None,
            None,
            WorkflowState::Completed,
        )?;
        session.artifacts.extend(artifacts);
        let reports = self.run_requested_invariants(
            &session.session_data,
            plan,
            None,
            WorkflowState::Completed,
        )?;
        session.invariant_reports.extend(reports);
        Ok(WorkflowCertificationReport {
            session,
            failure_bundle: None,
        })
    }
}
