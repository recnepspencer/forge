//! Successful Bank emergency-use lifecycle and authoritative readback evidence.

use std::time::Duration;

use bank_domain::{
    estate::{
        BankDisclosure, EmergencyAccessId, EmergencyAccessReason, EmergencyAccessStatus,
        EstateAction, EstateWorkflowStage, MandatoryReviewId, MandatoryReviewStatus,
        RestrictedBankField,
    },
    queries::EstateGovernanceQuery,
    reads::EstateGovernanceContext,
    schema::{
        AccountStatus, ViewEstateEmergencyProtectionCapability, ViewRestrictedEstateOperation,
    },
};
use worth_query_host::facade::{
    domain::TypedApplicationValue,
    primary_graph::{
        WorthQueryApplicationIdempotencyBinding, WorthQueryApplicationOneShotResult,
        WorthQueryElevationCloseOutcome, WorthQueryMandatoryReview,
        WorthQueryMandatoryReviewOutcome,
    },
};

use super::{
    fixture::{
        emergency_request_world, request_scope, CapabilityFixture, GrantSpec, ACCOUNT, ESTATE,
        GRANT, REVIEWER, SPECIALIST,
    },
    lifecycle_journey::{
        approve_elevation, request_elevation, ElevationApprovalSpec, ElevationRequestSpec,
    },
};
use crate::{queries, BankAuthenticatedPrincipal, BankReadControls};

type GovernanceResult =
    WorthQueryApplicationOneShotResult<EstateGovernanceQuery, EstateGovernanceContext>;

#[derive(Clone, Copy)]
struct EmergencyJourneyIdentity {
    access: EmergencyAccessId,
    review: MandatoryReviewId,
}

#[test]
fn approved_emergency_discloses_account_details_and_terminal_state_reads_back() {
    let fixture = emergency_request_world(
        "estate-emergency-active-use",
        GrantSpec::emergency_view(),
        EstateWorkflowStage::Administration,
    );
    let requester = fixture.authenticate();
    let approver = fixture.authenticate_approver();
    let reviewer = fixture.authenticate_reviewer();
    let identity = EmergencyJourneyIdentity {
        access: EmergencyAccessId::new(361).unwrap(),
        review: MandatoryReviewId::new(362).unwrap(),
    };
    let requested = request_elevation(
        &fixture,
        &requester,
        ElevationRequestSpec {
            grant: GRANT,
            access: 361,
            review: 362,
            idempotency: 91,
            field: RestrictedBankField::AccountDetails,
            duration: Duration::from_secs(300),
        },
    );
    let approved = approve_elevation(
        &fixture,
        &approver,
        requested,
        ElevationApprovalSpec {
            access: 361,
            idempotency: 93,
        },
    );

    assert_approved_account_disclosure(&fixture, &requester, identity.access, &approved);
    let mandatory = close_elevation(&fixture, &approver, approved, identity.access);
    assert_closed_readback(&fixture, &requester, &approver, identity);
    complete_review(&fixture, &reviewer, mandatory, identity);
    assert_completed_readback(&fixture, &requester, identity);
}

/// Q8.26-C6: a commit retains a pre-image exactly when its installed contract
/// declares one — and the negative twin holds in the same journey.
///
/// `ApproveEmergencyAccess` declares `RecordedInverse` over
/// `EmergencyAccessStatusField`, so its commit must carry the demanded slice.
/// `RequestEmergencyAccess` is a create lane declared `not_correctable`: there
/// is no prior truth, so it must carry nothing. Asserting only the positive
/// would pass against a runtime that retained indiscriminately, which would
/// make every receipt claim an inverse it cannot honour.
#[test]
fn retention_follows_the_declared_mechanism_on_both_lifecycle_commits() {
    let fixture = emergency_request_world(
        "estate-emergency-retention-by-mechanism",
        GrantSpec::emergency_view(),
        EstateWorkflowStage::Administration,
    );
    let requester = fixture.authenticate();
    let approver = fixture.authenticate_approver();
    let requested = request_elevation(
        &fixture,
        &requester,
        ElevationRequestSpec {
            grant: GRANT,
            access: 371,
            review: 372,
            idempotency: 95,
            field: RestrictedBankField::AccountDetails,
            duration: Duration::from_secs(300),
        },
    );
    let approved = approve_elevation(
        &fixture,
        &approver,
        requested,
        ElevationApprovalSpec {
            access: 371,
            idempotency: 97,
        },
    );

    assert!(
        approved
            .approval_commit_receipt()
            .retained_preimage()
            .is_some(),
        "ApproveEmergencyAccess declares RecordedInverse/ExactPriorTruth, so its \
         commit must carry the pre-image its installed contract demands"
    );
    assert!(
        approved
            .request_commit_receipt()
            .retained_preimage()
            .is_none(),
        "RequestEmergencyAccess is a not_correctable create lane — it has no \
         prior truth, so its commit must retain nothing"
    );
}

fn assert_approved_account_disclosure(
    fixture: &CapabilityFixture,
    requester: &BankAuthenticatedPrincipal,
    access: EmergencyAccessId,
    approved: &worth_query_host::facade::primary_graph::WorthQueryApprovedElevation,
) {
    let published = fixture
        .runtime
        .query(queries::estate_emergency_account_details(ESTATE, access))
        .as_principal(requester)
        .controls(controls())
        .execute_with_approved_elevation(approved)
        .expect("the exact approved field should reach the public Bank query");
    assert_eq!(published.rows().len(), 1);
    let BankDisclosure::Disclosed(account) = published.rows()[0].account() else {
        panic!("the exact approved account details must be disclosed");
    };
    assert_eq!(account.id(), ACCOUNT);
    assert_eq!(account.display_name().as_str(), "Estate Operating");
    assert_eq!(account.status(), AccountStatus::Frozen);
    let disclosure = published.receipt().disclosure();
    assert_eq!(
        disclosure.classification(),
        Some("estate-emergency-account-details")
    );
    assert_eq!(disclosure.decisions().len(), 4);
    assert!(disclosure.decisions().iter().all(|decision| {
        decision.required_disclosure()
            == &RestrictedBankField::AccountDetails.into_foundational_value()
    }));
    let capability = fixture
        .runtime
        .application_runtime()
        .installed_schema()
        .capability(
            ViewEstateEmergencyProtectionCapability::reference(),
            ViewRestrictedEstateOperation::reference(),
        )
        .unwrap();
    assert_eq!(
        disclosure.capability_authority_identity(),
        Some(capability.authority_identity())
    );
}

fn close_elevation(
    fixture: &CapabilityFixture,
    approver: &BankAuthenticatedPrincipal,
    approved: worth_query_host::facade::primary_graph::WorthQueryApprovedElevation,
    access: EmergencyAccessId,
) -> WorthQueryMandatoryReview {
    let close = fixture
        .runtime
        .revoke_estate_emergency_access(
            approver,
            approved,
            EstateAction::RevokeEmergencyAccess {
                estate: ESTATE,
                access,
            },
            WorthQueryApplicationIdempotencyBinding::new([95; 32], [96; 32]),
            &request_scope(),
        )
        .expect("the used elevation should remain closable through its exact command");
    let WorthQueryElevationCloseOutcome::Closed(mandatory) = close else {
        panic!("the close must commit before readback: {close:?}");
    };
    mandatory
}

fn assert_closed_readback(
    fixture: &CapabilityFixture,
    requester: &BankAuthenticatedPrincipal,
    approver: &BankAuthenticatedPrincipal,
    identity: EmergencyJourneyIdentity,
) {
    let closed = governance_readback(fixture, requester);
    let emergency = emergency(&closed, identity.access);
    assert_eq!(emergency.status(), EmergencyAccessStatus::Revoked);
    assert_eq!(emergency.grant(), GRANT);
    assert_eq!(emergency.requester(), SPECIALIST);
    assert_eq!(
        emergency.reason(),
        EmergencyAccessReason::PreventImmediateLoss
    );
    assert_eq!(emergency.approver(), Some(approver.principal_id()));
    assert_eq!(emergency.reviewer(), None);
    assert_eq!(emergency.mandatory_review().id, identity.review);
    assert_eq!(
        emergency.mandatory_review().status,
        MandatoryReviewStatus::Required
    );
    assert_eq!(emergency.mandatory_review().reviewer, None);
}

fn complete_review(
    fixture: &CapabilityFixture,
    reviewer: &BankAuthenticatedPrincipal,
    mandatory: WorthQueryMandatoryReview,
    identity: EmergencyJourneyIdentity,
) {
    let outcome = fixture
        .runtime
        .complete_estate_mandatory_review(
            reviewer,
            mandatory,
            EstateAction::CompleteMandatoryReview {
                estate: ESTATE,
                access: identity.access,
                review: identity.review,
            },
            WorthQueryApplicationIdempotencyBinding::new([97; 32], [98; 32]),
            &request_scope(),
        )
        .expect("the exact mandatory review should commit after readback");
    let WorthQueryMandatoryReviewOutcome::Reviewed(_) = outcome else {
        panic!("the exact review must be fresh: {outcome:?}");
    };
}

fn assert_completed_readback(
    fixture: &CapabilityFixture,
    requester: &BankAuthenticatedPrincipal,
    identity: EmergencyJourneyIdentity,
) {
    let completed = governance_readback(fixture, requester);
    let emergency = emergency(&completed, identity.access);
    assert_eq!(emergency.grant(), GRANT);
    assert_eq!(emergency.requester(), SPECIALIST);
    assert_eq!(emergency.review(), identity.review);
    assert_eq!(emergency.reviewer(), Some(REVIEWER));
    assert_eq!(
        emergency.mandatory_review().status,
        MandatoryReviewStatus::Completed
    );
    assert_eq!(emergency.mandatory_review().reviewer, Some(REVIEWER));
}

fn governance_readback(
    fixture: &CapabilityFixture,
    observer: &BankAuthenticatedPrincipal,
) -> GovernanceResult {
    fixture
        .runtime
        .query(queries::estate_governance_context(ESTATE))
        .as_principal(observer)
        .controls(controls())
        .execute()
        .expect("the governance observer should read authoritative lifecycle state")
}

fn emergency(
    result: &GovernanceResult,
    access: EmergencyAccessId,
) -> &bank_domain::reads::EstateEmergencyContext {
    result.rows()[0]
        .capabilities()
        .iter()
        .find(|capability| capability.id() == GRANT)
        .expect("the original governed grant should remain observable")
        .emergencies()
        .iter()
        .find(|emergency| emergency.id() == access)
        .expect("the exact lifecycle record should be projected from current graph truth")
}

fn controls() -> BankReadControls {
    BankReadControls::current(request_scope(), 1, 20_000).unwrap()
}
