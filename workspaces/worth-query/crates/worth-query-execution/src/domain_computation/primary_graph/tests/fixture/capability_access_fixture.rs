use worth_query_admission::facade::authenticated_principal::WorthQueryRequestScope;

use super::world_installation::AuthorizationWorld;
use super::{
    CapabilityAction, CapabilityDisclosure, CapabilityPurpose, CapabilityTouchInput,
    CapabilityTouchOperation, IdentityExecutionSchema, Principal, TouchAccountCapability,
};
use crate::domain_computation::primary_graph::{
    WorthQueryAuthenticatedPrincipal, WorthQueryOperationAuthorizationDenial,
    WorthQueryPreparedApplicationCapabilityAccess,
};

pub(in crate::domain_computation::primary_graph) fn admit_touch_account_capability(
    world: &AuthorizationWorld,
    principal: &WorthQueryAuthenticatedPrincipal<IdentityExecutionSchema, Principal, u64>,
    request: &WorthQueryRequestScope,
) -> Result<
    WorthQueryPreparedApplicationCapabilityAccess<
        IdentityExecutionSchema,
        TouchAccountCapability,
        CapabilityTouchOperation,
        CapabilityTouchInput,
    >,
    WorthQueryOperationAuthorizationDenial,
> {
    let capability = world
        .application
        .installed_schema()
        .capability(
            TouchAccountCapability::reference(),
            CapabilityTouchOperation::reference(),
        )
        .unwrap();
    world.application.prepare_capability_access(
        principal,
        &capability,
        CapabilityTouchInput {
            account: "account-1".to_owned(),
            action: CapabilityAction::Touch,
            purpose: CapabilityPurpose::AccountMaintenance,
            disclosure: CapabilityDisclosure::AccountActivity,
            caller_time: 100,
        },
        request,
    )
}
