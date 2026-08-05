use std::time::Duration;

use bank_domain::estate::{
    CapabilityGrantId, EmergencyAccessId, EmergencyAccessReason, EstateAction, MandatoryReviewId,
    RestrictedBankField,
};
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationIdempotencyBinding, WorthQueryApprovedElevation,
    WorthQueryElevationApprovalOutcome, WorthQueryElevationRequestOutcome,
    WorthQueryRequestedElevation,
};

use super::fixture::{request_scope, CapabilityFixture, ESTATE};
use crate::BankAuthenticatedPrincipal;

pub(super) fn request_elevation(
    fixture: &CapabilityFixture,
    requester: &BankAuthenticatedPrincipal,
    grant: CapabilityGrantId,
    access: u64,
    review: u64,
    idempotency: u8,
    field: RestrictedBankField,
) -> WorthQueryRequestedElevation {
    let outcome = fixture
        .runtime
        .request_estate_emergency_access(
            requester,
            EstateAction::RequestEmergencyAccess {
                estate: ESTATE,
                access: EmergencyAccessId::new(access).unwrap(),
                review: MandatoryReviewId::new(review).unwrap(),
                grant,
                reason: EmergencyAccessReason::PreventImmediateLoss,
                field,
                duration: Duration::from_secs(300),
            },
            WorthQueryApplicationIdempotencyBinding::new([idempotency; 32], [idempotency + 1; 32]),
            &request_scope(),
        )
        .expect("the approval prerequisite request should commit");
    let WorthQueryElevationRequestOutcome::Requested(requested) = outcome else {
        panic!("the approval prerequisite must be fresh: {outcome:?}");
    };
    requested
}

pub(super) fn approve_elevation(
    fixture: &CapabilityFixture,
    approver: &BankAuthenticatedPrincipal,
    requested: WorthQueryRequestedElevation,
    access: u64,
    idempotency: u8,
) -> WorthQueryApprovedElevation {
    let outcome = fixture
        .runtime
        .approve_estate_emergency_access(
            approver,
            requested,
            EstateAction::ApproveEmergencyAccess {
                estate: ESTATE,
                access: EmergencyAccessId::new(access).unwrap(),
            },
            WorthQueryApplicationIdempotencyBinding::new([idempotency; 32], [idempotency + 1; 32]),
            &request_scope(),
        )
        .expect("the terminal lifecycle prerequisite approval should commit");
    let WorthQueryElevationApprovalOutcome::Approved(approved) = outcome else {
        panic!("the terminal prerequisite approval must be fresh: {outcome:?}");
    };
    approved
}
