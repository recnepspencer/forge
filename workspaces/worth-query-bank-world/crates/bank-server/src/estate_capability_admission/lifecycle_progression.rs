use std::time::Duration;

use bank_domain::estate::{
    EmergencyAccessId, EmergencyAccessReason, EstateAction, EstateWorkflowStage, MandatoryReviewId,
    RestrictedBankField,
};
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationIdempotencyBinding, WorthQueryElevationRequestOutcome,
};

use super::fixture::{
    capability_world, emergency_request_world, request_scope, GrantSpec, ESTATE, GRANT,
};
use crate::estate_progression::BankEstateProgressionDenial;

#[test]
fn public_bank_runtime_commits_exact_emergency_request_through_query() {
    let fixture = emergency_request_world(
        "estate-emergency-request",
        GrantSpec::emergency_view(),
        EstateWorkflowStage::Administration,
    );
    let principal = fixture.authenticate();
    let action = EstateAction::RequestEmergencyAccess {
        estate: ESTATE,
        access: EmergencyAccessId::new(301).unwrap(),
        review: MandatoryReviewId::new(302).unwrap(),
        grant: GRANT,
        reason: EmergencyAccessReason::PreventImmediateLoss,
        field: RestrictedBankField::AccountDetails,
        duration: Duration::from_secs(300),
    };

    let outcome = fixture
        .runtime
        .request_estate_emergency_access(
            &principal,
            action,
            WorthQueryApplicationIdempotencyBinding::new([31; 32], [32; 32]),
            &request_scope(),
        )
        .expect("the public Bank runtime must reach Query's request progression");
    let WorthQueryElevationRequestOutcome::Requested(requested) = outcome else {
        panic!("the exact Bank request must commit once: {outcome:?}");
    };

    assert_eq!(requested.elevation_key(), "estate-emergency-access-301");
    assert_eq!(requested.review_key(), "estate-mandatory-review-302");
    assert_eq!(requested.commit_receipt().changed_record_count(), 7);
}

#[test]
fn governed_view_grant_cannot_substitute_for_request_command_authority() {
    let fixture = capability_world(
        "estate-emergency-command-substitution",
        GrantSpec::emergency_view(),
        EstateWorkflowStage::Administration,
        false,
        0,
    );
    let principal = fixture.authenticate();
    let action = EstateAction::RequestEmergencyAccess {
        estate: ESTATE,
        access: EmergencyAccessId::new(311).unwrap(),
        review: MandatoryReviewId::new(312).unwrap(),
        grant: GRANT,
        reason: EmergencyAccessReason::PreventImmediateLoss,
        field: RestrictedBankField::AccountDetails,
        duration: Duration::from_secs(300),
    };

    let denial = fixture
        .runtime
        .request_estate_emergency_access(
            &principal,
            action,
            WorthQueryApplicationIdempotencyBinding::new([41; 32], [42; 32]),
            &request_scope(),
        )
        .expect_err("a governed view grant must not authorize the request command");
    let BankEstateProgressionDenial::Authorization(denial) = denial else {
        panic!("command substitution must fail at authorization: {denial:?}");
    };
    assert_eq!(
        denial.kind(),
        worth_query_host::facade::primary_graph::WorthQueryOperationAuthorizationDenialKind::PermissionDenied
    );
}

#[test]
fn request_command_grant_cannot_substitute_for_governed_upper_bound_authority() {
    let fixture = capability_world(
        "estate-emergency-upper-bound-substitution",
        GrantSpec::emergency_request(),
        EstateWorkflowStage::Administration,
        false,
        0,
    );
    let principal = fixture.authenticate();
    let action = EstateAction::RequestEmergencyAccess {
        estate: ESTATE,
        access: EmergencyAccessId::new(321).unwrap(),
        review: MandatoryReviewId::new(322).unwrap(),
        grant: GRANT,
        reason: EmergencyAccessReason::PreventImmediateLoss,
        field: RestrictedBankField::AccountDetails,
        duration: Duration::from_secs(300),
    };

    let denial = fixture
        .runtime
        .request_estate_emergency_access(
            &principal,
            action,
            WorthQueryApplicationIdempotencyBinding::new([51; 32], [52; 32]),
            &request_scope(),
        )
        .expect_err("a request command grant must not become governed view authority");
    let BankEstateProgressionDenial::Authorization(denial) = denial else {
        panic!("upper-bound substitution must fail at authorization: {denial:?}");
    };
    assert_eq!(
        denial.kind(),
        worth_query_host::facade::primary_graph::WorthQueryOperationAuthorizationDenialKind::PermissionDenied
    );
}
