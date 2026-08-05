use worth_query_declaration::facade::{
    application_capability::{
        ApplicationCapabilityDelegationRequest, ApplicationCapabilityRequest,
    },
    application_schema::TypedMutationPreconditions,
};
use worth_query_installation::facade::{
    ApplicationSchema, WorthQueryInstalledApplicationCapability,
    WorthQueryInstalledApplicationOperation,
};

use super::capability_operation_progression::{
    progress_capability_operation, WorthQueryCapabilityOperationProgression,
};
use super::{
    WorthQueryAdmittedApplicationCapabilityAccess, WorthQueryAdmittedApplicationOperation,
    WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind,
};
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;

mod binding;
use binding::bind_activation;
pub(in crate::domain_computation) use binding::WorthQueryDelegationActivationBinding;
mod narrowing;
use narrowing::observe_narrowing;
mod support;
use support::authorize_target_support;

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    #[allow(clippy::too_many_arguments)]
    pub fn authorize_capability_delegation<
        CommandCapability,
        TargetCapability,
        TargetOperation,
        TargetInput,
        Operation,
        Input,
    >(
        &self,
        access: WorthQueryAdmittedApplicationCapabilityAccess<
            Schema,
            CommandCapability,
            Operation,
            Input,
        >,
        target: &WorthQueryInstalledApplicationCapability<
            Schema,
            TargetCapability,
            TargetOperation,
            TargetInput,
        >,
        operation: &WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
        preconditions: TypedMutationPreconditions<
            Schema,
            Operation,
            <Input as ApplicationCapabilityRequest<Schema, CommandCapability>>::Scope,
        >,
    ) -> Result<
        WorthQueryAdmittedApplicationOperation<
            Schema,
            Operation,
            Input,
            <Input as ApplicationCapabilityRequest<Schema, CommandCapability>>::Scope,
        >,
        WorthQueryOperationAuthorizationDenial,
    >
    where
        Input: ApplicationCapabilityRequest<Schema, CommandCapability>
            + ApplicationCapabilityDelegationRequest<Schema, TargetCapability>,
    {
        let proposed = access.input.delegation_request().map_err(|rejection| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::DelegationRejected,
                rejection.subject(),
            )
        })?;
        let installed = self
            .authorization
            .capability_plan(target)
            .ok_or_else(|| stale(target.contract().name()))?;
        validate_activation_operation(installed, operation)?;
        let (resolved, supporting) =
            authorize_target_support(self, target, installed, &access, &proposed)?;
        let required_program_targets = target
            .delegation_activation_program_targets()
            .ok_or_else(|| delegation_denial(installed))?;
        let proposal_budget = operation
            .contracts()
            .delegation_activation_proposal_canonical_work_budget()
            .ok_or_else(|| delegation_denial(installed))?;
        let prepared = bind_activation(
            self,
            installed,
            &proposed,
            resolved,
            *target.identity().bytes(),
            required_program_targets,
            proposal_budget,
        )?;
        let mut admitted = progress_capability_operation(
            self,
            access,
            operation,
            preconditions,
            WorthQueryCapabilityOperationProgression::DelegationActivation,
        )?;
        admitted.retain_delegation_proposal_canonical_work(prepared.canonical_work);
        admitted
            .authorization_mut()
            .and_then(|authorization| authorization.capability_authorization_mut())
            .ok_or_else(|| inconsistent(operation.operation()))?
            .retain_supporting(supporting)
            .map_err(|()| inconsistent(operation.operation()))?;
        admitted.graph_work_mut().record_decision_facts(1);
        admitted.bind_delegation_activation(prepared.binding)
    }
}

fn validate_activation_operation<Schema, Operation, Input>(
    installed: &super::capability_registry::WorthQueryInstalledCapabilityPlan,
    operation: &WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
) -> Result<(), WorthQueryOperationAuthorizationDenial> {
    let activation = installed
        .delegation
        .activation
        .as_ref()
        .ok_or_else(|| delegation_denial(installed))?;
    if activation.operation == operation.operation()
        && activation.operation_type == std::any::type_name::<Operation>()
        && activation.input_type == std::any::type_name::<Input>()
    {
        Ok(())
    } else {
        Err(delegation_denial(installed))
    }
}

fn stale(subject: impl Into<String>) -> WorthQueryOperationAuthorizationDenial {
    denial(
        WorthQueryOperationAuthorizationDenialKind::StaleInstalledOperation,
        subject,
    )
}

fn inconsistent(subject: impl Into<String>) -> WorthQueryOperationAuthorizationDenial {
    denial(
        WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
        subject,
    )
}

fn delegation_denial(
    installed: &super::capability_registry::WorthQueryInstalledCapabilityPlan,
) -> WorthQueryOperationAuthorizationDenial {
    denial(
        WorthQueryOperationAuthorizationDenialKind::DelegationRejected,
        installed.contract.name(),
    )
}

fn denial(
    kind: WorthQueryOperationAuthorizationDenialKind,
    subject: impl Into<String>,
) -> WorthQueryOperationAuthorizationDenial {
    WorthQueryOperationAuthorizationDenial::new(kind, subject)
}
