use std::collections::{BTreeMap, BTreeSet};

use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::operation_authority_chain::{
    mint_operation_phase_proof, operation_phase_basis, WorthQueryOperationPhaseProof,
    WorthQueryResourceAdmittedOperationPhase,
};
use worth_proof::TransitionOutcome;
use worth_query_declaration::facade::domain_computation::WorthQueryExecutionResourceRequest;

use super::{
    admit_execution_resource_plan, WorthQueryAdmittedWorkflowResourcePlan,
    WorthQueryExecutionProviderSession, WorthQueryExecutionResourceAdmissionCounters,
    WorthQueryExecutionResourceAdmissionDenial,
    WorthQueryExecutionResourceAdmissionDenialKind as Kind, WorthQueryExecutionResourceSupport,
    WorthQueryExecutionResourceSupportSnapshot, WorthQueryWorkflowExecutionResourceAttempt,
};

pub type WorthQueryWorkflowResourceAdmissionOutcome<D, O, F, L> = TransitionOutcome<
    WorthQueryAdmittedWorkflowOperation<D, O, F, L>,
    WorthQueryExecutionResourceAdmissionDenial,
    WorthQueryExecutionResourceAdmissionDenial,
    WorthQueryExecutionResourceAdmissionDenial,
    WorthQueryExecutionResourceAdmissionDenial,
    WorthQueryExecutionResourceAdmissionDenial,
>;

pub struct WorthQueryAdmittedWorkflowOperation<D, O, F, L: BasisOperationLane> {
    pub(crate) bound: crate::domain_installation::WorthQueryBoundDomainOperation<D, O, F, L>,
    pub(crate) resource_attempt: WorthQueryWorkflowExecutionResourceAttempt,
    pub(crate) phase_proof: WorthQueryOperationPhaseProof<WorthQueryResourceAdmittedOperationPhase>,
}

impl<D, O, F, L: BasisOperationLane> WorthQueryAdmittedWorkflowOperation<D, O, F, L> {
    pub fn resources(&self) -> &WorthQueryAdmittedWorkflowResourcePlan {
        self.resource_attempt.resources()
    }

    pub fn provider_session(&self) -> &WorthQueryExecutionProviderSession {
        self.resource_attempt.provider_session()
    }
}

impl<D: 'static, O: 'static, F: 'static, L: BasisOperationLane>
    crate::domain_installation::WorthQueryBoundDomainOperation<D, O, F, L>
where
    O: crate::domain_installation::WorthQueryExecutableDomainOperation<
        D,
        F,
        Execution = crate::domain_installation::WorthQueryWorkflowOperation,
    >,
{
    pub fn admit_workflow_resources(
        self,
        request: WorthQueryExecutionResourceRequest,
        workspace: &crate::runtime::WorthQueryWorkspace,
    ) -> WorthQueryWorkflowResourceAdmissionOutcome<D, O, F, L> {
        let counters = WorthQueryExecutionResourceAdmissionCounters {
            runtime_authority_checks: 1,
            ..Default::default()
        };
        let witness =
            crate::domain_installation::WorthQueryInstalledDomainAuthorityWitness::from_authority(
                std::sync::Arc::clone(self.operation().domain_authority()),
            );
        if let Err(denial) = workspace.validate_installed_domain_witness::<D>(&witness) {
            let kind = denial.kind();
            let denial = WorthQueryExecutionResourceAdmissionDenial::new(
                Kind::RuntimeAuthority(kind),
                format!("{denial:?}"),
                counters,
            );
            return runtime_denial(kind, denial);
        }
        let Some(executor) = self.workflow_executor() else {
            return TransitionOutcome::Failed(WorthQueryExecutionResourceAdmissionDenial::new(
                Kind::ExecutorSupportMissing,
                "bound workflow operation has no installed stage executor support snapshot",
                counters,
            ));
        };
        let support = WorthQueryExecutionResourceSupportSnapshot::new(
            executor.resource_support.clone(),
            super::operation_conditional_supports(&self),
            Vec::new(),
            Vec::new(),
            self.workflow_parallel_admission_provider()
                .map(|provider| provider.resource_support().clone()),
        );
        let operation = match admit_execution_resource_plan(
            self.binding_identity(),
            &self.definition().semantics().resources,
            &request,
            support.clone(),
            counters,
        ) {
            Ok(plan) => plan,
            Err(denial) => return admission_denial_outcome(denial),
        };
        let stages = match lower_stages(
            &self,
            &request,
            &executor.resource_support,
            WorthQueryExecutionResourceAdmissionCounters::default(),
        ) {
            Ok(stages) => stages,
            Err(denial) => return admission_denial_outcome(denial),
        };
        let resources = WorthQueryAdmittedWorkflowResourcePlan::new(operation, stages);
        let resource_attempt = WorthQueryWorkflowExecutionResourceAttempt::start(resources);
        let mut basis = operation_phase_basis(self.authority_proof()).clone();
        basis.resource_admission_identity =
            Some(resource_attempt.resources().identity().to_owned());
        let phase_proof = mint_operation_phase_proof(
            resource_attempt.resources().identity().to_owned(),
            Some(self.authority_proof().payload().identity()),
            basis,
        );
        TransitionOutcome::Success(WorthQueryAdmittedWorkflowOperation {
            bound: self,
            resource_attempt,
            phase_proof,
        })
    }
}

fn lower_stages<D, O, F, L: BasisOperationLane>(
    bound: &crate::domain_installation::WorthQueryBoundDomainOperation<D, O, F, L>,
    request: &WorthQueryExecutionResourceRequest,
    executor_support: &WorthQueryExecutionResourceSupport,
    counters: WorthQueryExecutionResourceAdmissionCounters,
) -> Result<
    BTreeMap<String, super::WorthQueryAdmittedExecutionResourcePlan>,
    WorthQueryExecutionResourceAdmissionDenial,
> {
    let workflow = match &bound.definition().semantics().workflow {
        worth_query_installation::facade::WorthQueryOperationWorkflowContract::Declared(
            workflow,
        ) => workflow,
        worth_query_installation::facade::WorthQueryOperationWorkflowContract::NotRequired => {
            return Err(WorthQueryExecutionResourceAdmissionDenial::new(
                Kind::ResourceContract,
                "workflow resource admission requires an installed workflow definition",
                counters,
            ))
        }
    };
    workflow
        .stages()
        .iter()
        .map(|stage| {
            let support = stage_support_snapshot(bound, stage, executor_support);
            admit_execution_resource_plan(
                &format!("{}:{}", bound.binding_identity(), stage.identity()),
                &stage.semantics().resources,
                request,
                support,
                counters,
            )
            .map(|plan| (stage.identity().to_owned(), plan))
        })
        .collect()
}

fn stage_support_snapshot<D, O, F, L: BasisOperationLane>(
    bound: &crate::domain_installation::WorthQueryBoundDomainOperation<D, O, F, L>,
    stage: &worth_query_installation::facade::WorthQueryPortableWorkflowStage,
    executor_support: &WorthQueryExecutionResourceSupport,
) -> WorthQueryExecutionResourceSupportSnapshot {
    let roles = stage
        .semantics()
        .graph_read_roles
        .iter()
        .chain(&stage.semantics().touch_roles)
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let touch_roles = stage
        .semantics()
        .touch_roles
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    WorthQueryExecutionResourceSupportSnapshot::new(
        executor_support.clone(),
        super::stage_conditional_supports(bound, stage.identity()),
        bound
            .graph_participations()
            .iter()
            .filter(|participation| roles.contains(participation.role.as_str()))
            .map(|participation| {
                (
                    participation.role.clone(),
                    participation.record.resource_support.clone(),
                )
            })
            .collect(),
        super::commit_supports_for_roles(bound, &touch_roles),
        None,
    )
}

fn runtime_denial<T>(
    kind: crate::domain_installation::WorthQueryDomainHandleDenialKind,
    denial: WorthQueryExecutionResourceAdmissionDenial,
) -> TransitionOutcome<
    T,
    WorthQueryExecutionResourceAdmissionDenial,
    WorthQueryExecutionResourceAdmissionDenial,
    WorthQueryExecutionResourceAdmissionDenial,
    WorthQueryExecutionResourceAdmissionDenial,
    WorthQueryExecutionResourceAdmissionDenial,
> {
    use crate::domain_installation::WorthQueryDomainHandleDenialKind as Runtime;
    match kind {
        Runtime::StaleInstallationGeneration => TransitionOutcome::Stale(denial),
        Runtime::PackageIdentityChanged => TransitionOutcome::RebindRequired(denial),
        Runtime::DomainNotInstalled | Runtime::ForeignRuntime => TransitionOutcome::Denied(denial),
    }
}

fn admission_denial_outcome<T>(
    denial: WorthQueryExecutionResourceAdmissionDenial,
) -> TransitionOutcome<
    T,
    WorthQueryExecutionResourceAdmissionDenial,
    WorthQueryExecutionResourceAdmissionDenial,
    WorthQueryExecutionResourceAdmissionDenial,
    WorthQueryExecutionResourceAdmissionDenial,
    WorthQueryExecutionResourceAdmissionDenial,
> {
    match denial.kind() {
        Kind::Backpressured | Kind::AsyncExecutionRequired => TransitionOutcome::Deferred(denial),
        _ => TransitionOutcome::Denied(denial),
    }
}
