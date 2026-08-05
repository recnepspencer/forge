use bank_domain::estate::EstateAction;
use bank_server::{BankAuthenticatedPrincipal, BankEstateProgressionDenial, BankIdentityRuntime};
use worth_query_host::facade::{
    admission::authenticated_principal::WorthQueryRequestScope,
    primary_graph::{
        WorthQueryApplicationIdempotencyBinding, WorthQueryApprovedElevation,
        WorthQueryMandatoryReview, WorthQueryRequestedElevation,
    },
};

pub struct EstateCommandInputs {
    pub actions: [EstateAction; 9],
    pub idempotency: [WorthQueryApplicationIdempotencyBinding; 9],
}

pub struct EstateLifecycleInputs {
    pub requested: WorthQueryRequestedElevation,
    pub approved: WorthQueryApprovedElevation,
    pub mandatory_review: WorthQueryMandatoryReview,
    pub approval_action: EstateAction,
    pub close_action: EstateAction,
    pub review_action: EstateAction,
    pub idempotency: [WorthQueryApplicationIdempotencyBinding; 3],
}

pub fn exercise_estate_commands(
    runtime: &BankIdentityRuntime,
    principal: &BankAuthenticatedPrincipal,
    request: &WorthQueryRequestScope,
    inputs: EstateCommandInputs,
) -> Vec<Result<(), BankEstateProgressionDenial>> {
    let [notify, freeze, open, recognize, delegate, revoke, request_elevation, release, disburse] =
        inputs.actions;
    let [notify_id, freeze_id, open_id, recognize_id, delegate_id, revoke_id, request_id, release_id, disburse_id] =
        inputs.idempotency;
    vec![
        runtime
            .notify_estate_death(principal, notify, notify_id, request)
            .map(|_| ()),
        runtime
            .freeze_estate_account(principal, freeze, freeze_id, request)
            .map(|_| ()),
        runtime
            .open_estate_case(principal, open, open_id, request)
            .map(|_| ()),
        runtime
            .recognize_estate_executor(principal, recognize, recognize_id, request)
            .map(|_| ()),
        runtime
            .delegate_estate_capability(principal, delegate, delegate_id, request)
            .map(|_| ()),
        runtime
            .revoke_estate_capability(principal, revoke, revoke_id, request)
            .map(|_| ()),
        runtime
            .request_estate_emergency_access(principal, request_elevation, request_id, request)
            .map(|_| ()),
        runtime
            .release_estate(principal, release, release_id, request)
            .map(|_| ()),
        runtime
            .disburse_estate(principal, disburse, disburse_id, request)
            .map(|_| ()),
    ]
}

pub fn exercise_estate_lifecycle(
    runtime: &BankIdentityRuntime,
    principal: &BankAuthenticatedPrincipal,
    request: &WorthQueryRequestScope,
    inputs: EstateLifecycleInputs,
) -> Vec<Result<(), BankEstateProgressionDenial>> {
    let [approval_id, close_id, review_id] = inputs.idempotency;
    vec![
        runtime
            .approve_estate_emergency_access(
                principal,
                inputs.requested,
                inputs.approval_action,
                approval_id,
                request,
            )
            .map(|_| ()),
        runtime
            .revoke_estate_emergency_access(
                principal,
                inputs.approved,
                inputs.close_action,
                close_id,
                request,
            )
            .map(|_| ()),
        runtime
            .complete_estate_mandatory_review(
                principal,
                inputs.mandatory_review,
                inputs.review_action,
                review_id,
                request,
            )
            .map(|_| ()),
    ]
}
