use std::sync::{Arc, Mutex, Weak};

use worth_query_admission::facade::resource_admission::{
    WorthQueryAdmittedExecutionResourcePlan, WorthQueryAdmittedWorkflowResourcePlan,
};
use worth_query_admission::integration::WorthQueryCapacityReservedWorkflowResourcePlan;

use super::{WorthQueryExecutionProviderSession, WorthQueryExecutionResourceAttemptEvidence};
pub struct WorthQueryWorkflowExecutionResourceAttempt {
    reserved: WorthQueryCapacityReservedWorkflowResourcePlan,
    provider_session: WorthQueryExecutionProviderSession,
    evidence: WorthQueryExecutionResourceAttemptEvidence,
    artifact_run: Mutex<WorthQueryWorkflowArtifactRunState>,
}

struct WorthQueryWorkflowArtifactRunState {
    next_generation: u64,
    active_registry:
        Option<Weak<crate::domain_computation::artifact_owner::WorthQueryWorkflowArtifactRegistry>>,
}

impl WorthQueryWorkflowExecutionResourceAttempt {
    pub(crate) fn start(
        mut reserved: WorthQueryCapacityReservedWorkflowResourcePlan,
        binding_authority: &crate::domain_computation::operation_binding::WorthQueryExecutionBoundOperationAuthority,
    ) -> Self {
        let provider_session = WorthQueryExecutionProviderSession::mint(
            reserved.resources().identity(),
            binding_authority,
        );
        reserved.resources_mut().record_provider_session_mint();
        let evidence = WorthQueryExecutionResourceAttemptEvidence::capture(
            reserved.resources().operation(),
            &provider_session,
        );
        Self {
            reserved,
            provider_session,
            evidence,
            artifact_run: Mutex::new(WorthQueryWorkflowArtifactRunState {
                next_generation: 1,
                active_registry: None,
            }),
        }
    }

    pub fn resources(&self) -> &WorthQueryAdmittedWorkflowResourcePlan {
        self.reserved.resources()
    }

    pub fn operation_resources(&self) -> &WorthQueryAdmittedExecutionResourcePlan {
        self.reserved.resources().operation()
    }

    pub fn provider_session(&self) -> &WorthQueryExecutionProviderSession {
        &self.provider_session
    }

    pub fn evidence(&self) -> &WorthQueryExecutionResourceAttemptEvidence {
        &self.evidence
    }

    pub fn bind_workflow_artifacts(
        &self,
    ) -> Result<
        crate::domain_computation::artifact_owner::WorthQueryWorkflowArtifactAuthority,
        crate::domain_computation::artifact_owner::WorthQueryArtifactDenial,
    > {
        let mut run = self
            .artifact_run
            .lock()
            .expect("workflow artifact run lock must remain available");
        if run
            .active_registry
            .as_ref()
            .and_then(Weak::upgrade)
            .is_some_and(|registry| !registry.is_closed())
        {
            return Err(
                crate::domain_computation::artifact_owner::WorthQueryArtifactDenial::new(
                    crate::domain_computation::artifact_owner::WorthQueryArtifactDenialKind::ActiveWorkflowRun,
                    None,
                    "execution resource attempt already owns an active workflow artifact run",
                ),
            );
        }
        let run_generation = run.next_generation;
        run.next_generation = run.next_generation.checked_add(1).ok_or_else(|| {
            crate::domain_computation::artifact_owner::WorthQueryArtifactDenial::new(
                crate::domain_computation::artifact_owner::WorthQueryArtifactDenialKind::StaleLifecycleGeneration,
                None,
                "workflow artifact run generation is exhausted",
            )
        })?;
        let authority =
            crate::domain_computation::artifact_owner::WorthQueryWorkflowArtifactAuthority::mint(
                self.provider_session.retain_binding_authority(),
                self.provider_session.identity(),
                run_generation,
            )?;
        run.active_registry = Some(Arc::downgrade(&authority.registry()));
        Ok(authority)
    }

    pub fn stage_resources_and_evidence(
        &self,
        stage_identity: &str,
    ) -> Option<(
        Arc<WorthQueryAdmittedExecutionResourcePlan>,
        WorthQueryExecutionResourceAttemptEvidence,
    )> {
        let resources = self.reserved.resources().shared_stage(stage_identity)?;
        let evidence =
            WorthQueryExecutionResourceAttemptEvidence::capture(&resources, &self.provider_session);
        Some((resources, evidence))
    }
}
