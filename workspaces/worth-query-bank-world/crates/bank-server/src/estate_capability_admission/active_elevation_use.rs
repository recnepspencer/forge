use bank_domain::{
    estate::{
        BankDisclosure, EmergencyAccessId, EmergencyAccessReason, EmergencyAccessStatus,
        EstateAction, EstateDisbursement, EstatePosting, EstateWorkflowStage, MandatoryReviewId,
        MandatoryReviewStatus, RestrictedBankField,
    },
    model::{Money, SignedMoney},
    queries::EstateGovernanceQuery,
    reads::EstateGovernanceContext,
    schema::{
        AccountStatus, DisburseEstateCapability, DisburseEstateOperation,
        ViewEstateEmergencyProtectionCapability, ViewRestrictedEstateOperation,
    },
};
use worth_query_host::facade::{
    domain::TypedApplicationValue,
    primary_graph::{
        WorthQueryApplicationIdempotencyBinding, WorthQueryApplicationOneShotResult,
        WorthQueryElevationCloseOutcome, WorthQueryMandatoryReviewOutcome,
        WorthQueryOperationAuthorizationDenialKind,
    },
};

use super::{
    fixture::{
        emergency_request_world, emergency_request_world_with_alternate_bound, request_scope,
        GrantSpec, ACCOUNT, ALTERNATE_EMERGENCY_BOUND_GRANT, APPROVER, ESTATE, GRANT,
        OTHER_ACCOUNT, REVIEWER, SPECIALIST,
    },
    lifecycle_journey::{approve_elevation, request_elevation},
};
use crate::{queries, BankApplicationQueryDenial, BankReadControls};

type GovernanceResult =
    WorthQueryApplicationOneShotResult<EstateGovernanceQuery, EstateGovernanceContext>;

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
    let access = EmergencyAccessId::new(361).unwrap();
    let review = MandatoryReviewId::new(362).unwrap();
    let requested = request_elevation(
        &fixture,
        &requester,
        GRANT,
        361,
        362,
        91,
        RestrictedBankField::AccountDetails,
    );
    let approved = approve_elevation(&fixture, &approver, requested, 361, 93);

    let published = fixture
        .runtime
        .query(queries::estate_emergency_account_details(ESTATE, access))
        .as_principal(&requester)
        .controls(controls())
        .execute_with_approved_elevation(&approved)
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

    let close = fixture
        .runtime
        .revoke_estate_emergency_access(
            &approver,
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
    let closed = governance_readback(&fixture, &requester);
    let closed_emergency = emergency(&closed, access);
    assert_eq!(closed_emergency.status(), EmergencyAccessStatus::Revoked);
    assert_eq!(closed_emergency.grant(), GRANT);
    assert_eq!(closed_emergency.requester(), SPECIALIST);
    assert_eq!(
        closed_emergency.reason(),
        EmergencyAccessReason::PreventImmediateLoss
    );
    assert_eq!(closed_emergency.approver(), Some(approver.principal_id()));
    assert_eq!(closed_emergency.reviewer(), None);
    assert_eq!(closed_emergency.mandatory_review().id, review);
    assert_eq!(
        closed_emergency.mandatory_review().status,
        MandatoryReviewStatus::Required
    );
    assert_eq!(closed_emergency.mandatory_review().reviewer, None);

    let reviewed = fixture
        .runtime
        .complete_estate_mandatory_review(
            &reviewer,
            mandatory,
            EstateAction::CompleteMandatoryReview {
                estate: ESTATE,
                access,
                review,
            },
            WorthQueryApplicationIdempotencyBinding::new([97; 32], [98; 32]),
            &request_scope(),
        )
        .expect("the exact mandatory review should commit after readback");
    let WorthQueryMandatoryReviewOutcome::Reviewed(_) = reviewed else {
        panic!("the exact review must be fresh: {reviewed:?}");
    };
    let completed = governance_readback(&fixture, &requester);
    let completed_emergency = emergency(&completed, access);
    assert_eq!(completed_emergency.grant(), GRANT);
    assert_eq!(completed_emergency.requester(), SPECIALIST);
    assert_eq!(completed_emergency.review(), review);
    assert_eq!(completed_emergency.reviewer(), Some(REVIEWER));
    assert_eq!(
        completed_emergency.mandatory_review().status,
        MandatoryReviewStatus::Completed
    );
    assert_eq!(
        completed_emergency.mandatory_review().reviewer,
        Some(REVIEWER)
    );
}

#[test]
fn approved_different_field_cannot_open_account_details() {
    let fixture = emergency_request_world_with_alternate_bound(
        "estate-emergency-field-substitution",
        GrantSpec::emergency_view_for(RestrictedBankField::BeneficiaryIdentity),
        GrantSpec::emergency_view(),
        EstateWorkflowStage::Administration,
    );
    let requester = fixture.authenticate();
    let approver = fixture.authenticate_approver();
    let wrong_requested = request_elevation(
        &fixture,
        &requester,
        GRANT,
        371,
        372,
        101,
        RestrictedBankField::BeneficiaryIdentity,
    );
    let wrong_approved = approve_elevation(&fixture, &approver, wrong_requested, 371, 103);
    let account_access = EmergencyAccessId::new(373).unwrap();
    let account_requested = request_elevation(
        &fixture,
        &requester,
        ALTERNATE_EMERGENCY_BOUND_GRANT,
        373,
        374,
        105,
        RestrictedBankField::AccountDetails,
    );
    let _account_approved = approve_elevation(&fixture, &approver, account_requested, 373, 107);

    let denial = match fixture
        .runtime
        .query(queries::estate_emergency_account_details(
            ESTATE,
            account_access,
        ))
        .as_principal(&requester)
        .controls(controls())
        .execute_with_approved_elevation(&wrong_approved)
    {
        Ok(_) => panic!("a different approved field must not disclose account details"),
        Err(denial) => denial,
    };
    let BankApplicationQueryDenial::CapabilityAdmission(denial) = denial else {
        panic!("field substitution must fail during capability admission: {denial:?}");
    };
    assert_eq!(
        denial.kind(),
        WorthQueryOperationAuthorizationDenialKind::ElevationApprovalRejected
    );
}

#[test]
fn real_approved_elevation_cannot_enter_the_bank_disbursement_operation() {
    let fixture = emergency_request_world(
        "estate-emergency-disbursement-escalation",
        GrantSpec::emergency_view(),
        EstateWorkflowStage::Administration,
    );
    let requester = fixture.authenticate();
    let approver = fixture.authenticate_approver();
    let requested = request_elevation(
        &fixture,
        &requester,
        GRANT,
        381,
        382,
        111,
        RestrictedBankField::AccountDetails,
    );
    let approved = approve_elevation(&fixture, &approver, requested, 381, 113);
    let capability = fixture
        .runtime
        .application_runtime()
        .installed_schema()
        .capability(
            DisburseEstateCapability::reference(),
            DisburseEstateOperation::reference(),
        )
        .unwrap();
    let action = disbursement_action();

    let denial = fixture
        .runtime
        .application_runtime()
        .admit_approved_elevation_access(
            &approved,
            requester.query(),
            &capability,
            action,
            &request_scope(),
        )
        .err()
        .expect("DisburseEstate does not admit approved-elevation authority");
    assert_eq!(
        denial.kind(),
        WorthQueryOperationAuthorizationDenialKind::ElevationNotApplicable
    );
    let ordinary = fixture.runtime.disburse_estate(
        &requester,
        action,
        WorthQueryApplicationIdempotencyBinding::new([115; 32], [116; 32]),
        &request_scope(),
    );
    assert!(matches!(
        ordinary,
        Err(crate::BankEstateProgressionDenial::Authorization(_))
    ));
}

fn disbursement_action() -> EstateAction {
    EstateAction::DisburseEstate(EstateDisbursement {
        estate: ESTATE,
        source_account: ACCOUNT,
        destination_account: OTHER_ACCOUNT,
        beneficiary: APPROVER,
        amount: Money::from_minor(250).unwrap(),
        postings: [
            EstatePosting {
                account: ACCOUNT,
                amount: SignedMoney::from_minor(-250),
            },
            EstatePosting {
                account: OTHER_ACCOUNT,
                amount: SignedMoney::from_minor(250),
            },
        ],
    })
}

fn governance_readback(
    fixture: &super::fixture::CapabilityFixture,
    observer: &crate::BankAuthenticatedPrincipal,
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
