use bank_domain::estate::EstateAction;
use bank_server::{
    BankApprovedEstateElevation, BankAuthenticatedPrincipal, BankEstateElevationApprovalOutcome,
    BankEstateElevationCloseOutcome, BankEstateElevationRequestOutcome, BankEstateMandatoryReview,
    BankEstateMandatoryReviewOutcome, BankEstateProgressionDenial, BankIdentityRuntime,
    BankRequestedEstateElevation, BankReviewedEstateElevation,
};
use worth_query_host::facade::{
    admission::authenticated_principal::WorthQueryRequestScope,
    primary_graph::WorthQueryApplicationIdempotencyBinding,
};

pub struct EstateCommandInputs {
    pub actions: [EstateAction; 9],
    pub idempotency: [WorthQueryApplicationIdempotencyBinding; 9],
}

pub struct EstateLifecyclePrincipals<'a> {
    pub requester: &'a BankAuthenticatedPrincipal,
    pub approver: &'a BankAuthenticatedPrincipal,
    pub closer: &'a BankAuthenticatedPrincipal,
    pub reviewer: &'a BankAuthenticatedPrincipal,
}

pub struct EstateLifecycleInputs {
    pub request_action: EstateAction,
    pub approval_action: EstateAction,
    pub close_action: EstateAction,
    pub review_action: EstateAction,
    pub idempotency: [WorthQueryApplicationIdempotencyBinding; 4],
}

#[derive(Debug)]
pub enum EstateLifecycleProgressionOutcome {
    Reviewed(Box<BankReviewedEstateElevation>),
    AlreadyReviewed(Box<BankReviewedEstateElevation>),
    RequestStopped(Box<BankEstateElevationRequestOutcome>),
    ApprovalStopped(Box<BankEstateElevationApprovalOutcome>),
    CloseStopped(Box<BankEstateElevationCloseOutcome>),
    ReviewStopped(Box<BankEstateMandatoryReviewOutcome>),
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
    principals: EstateLifecyclePrincipals<'_>,
    request: &WorthQueryRequestScope,
    inputs: EstateLifecycleInputs,
) -> Result<EstateLifecycleProgressionOutcome, BankEstateProgressionDenial> {
    let [request_id, approval_id, close_id, review_id] = inputs.idempotency;
    let requested = match requested_receipt(runtime.request_estate_emergency_access(
        principals.requester,
        inputs.request_action,
        request_id,
        request,
    )?) {
        Ok(receipt) => receipt,
        Err(stopped) => return Ok(EstateLifecycleProgressionOutcome::RequestStopped(stopped)),
    };
    let approved = match approved_receipt(runtime.approve_estate_emergency_access(
        principals.approver,
        requested,
        inputs.approval_action,
        approval_id,
        request,
    )?) {
        Ok(receipt) => receipt,
        Err(stopped) => return Ok(EstateLifecycleProgressionOutcome::ApprovalStopped(stopped)),
    };
    let mandatory_review = match mandatory_review_receipt(runtime.revoke_estate_emergency_access(
        principals.closer,
        approved,
        inputs.close_action,
        close_id,
        request,
    )?) {
        Ok(receipt) => receipt,
        Err(stopped) => return Ok(EstateLifecycleProgressionOutcome::CloseStopped(stopped)),
    };
    Ok(reviewed_outcome(runtime.complete_estate_mandatory_review(
        principals.reviewer,
        mandatory_review,
        inputs.review_action,
        review_id,
        request,
    )?))
}

fn requested_receipt(
    outcome: BankEstateElevationRequestOutcome,
) -> Result<BankRequestedEstateElevation, Box<BankEstateElevationRequestOutcome>> {
    match outcome {
        BankEstateElevationRequestOutcome::Requested(receipt)
        | BankEstateElevationRequestOutcome::AlreadyRequested(receipt) => Ok(receipt),
        stopped => Err(Box::new(stopped)),
    }
}

fn approved_receipt(
    outcome: BankEstateElevationApprovalOutcome,
) -> Result<BankApprovedEstateElevation, Box<BankEstateElevationApprovalOutcome>> {
    match outcome {
        BankEstateElevationApprovalOutcome::Approved(receipt)
        | BankEstateElevationApprovalOutcome::AlreadyApproved(receipt) => Ok(receipt),
        stopped => Err(Box::new(stopped)),
    }
}

fn mandatory_review_receipt(
    outcome: BankEstateElevationCloseOutcome,
) -> Result<BankEstateMandatoryReview, Box<BankEstateElevationCloseOutcome>> {
    match outcome {
        BankEstateElevationCloseOutcome::Closed(receipt)
        | BankEstateElevationCloseOutcome::AlreadyClosed(receipt) => Ok(receipt),
        stopped => Err(Box::new(stopped)),
    }
}

fn reviewed_outcome(
    outcome: BankEstateMandatoryReviewOutcome,
) -> EstateLifecycleProgressionOutcome {
    match outcome {
        BankEstateMandatoryReviewOutcome::Reviewed(receipt) => {
            EstateLifecycleProgressionOutcome::Reviewed(Box::new(receipt))
        }
        BankEstateMandatoryReviewOutcome::AlreadyReviewed(receipt) => {
            EstateLifecycleProgressionOutcome::AlreadyReviewed(Box::new(receipt))
        }
        stopped => EstateLifecycleProgressionOutcome::ReviewStopped(Box::new(stopped)),
    }
}
