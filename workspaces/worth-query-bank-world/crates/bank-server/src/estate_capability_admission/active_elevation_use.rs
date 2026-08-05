use bank_domain::{
    estate::{
        EmergencyAccessId, EstateAction, EstateDisbursement, EstatePosting, EstateWorkflowStage,
        RestrictedBankField,
    },
    model::{Money, SignedMoney},
    schema::{DisburseEstateCapability, DisburseEstateOperation},
};
use worth_foundational::facade::{
    FoundationalBoundaryEvidenceExecutionPosture, FoundationalDiagnosticOutcomeKind,
};
use worth_query_host::facade::{
    primary_graph::{
        WorthQueryApplicationAuthorizationExplanationCause,
        WorthQueryApplicationIdempotencyBinding, WorthQueryOperationAuthorizationDenialKind,
    },
    publication::domain_computation::{
        publish_application_authorization_denial,
        WorthQueryApplicationAuthorizationPublicationProfile,
    },
};

use super::{
    fixture::{
        emergency_request_world, emergency_request_world_with_alternate_bound, request_scope,
        GrantSpec, ACCOUNT, ALTERNATE_EMERGENCY_BOUND_GRANT, APPROVER, ESTATE, GRANT,
        OTHER_ACCOUNT,
    },
    lifecycle_journey::{approve_elevation, request_elevation},
    publication_evidence::publication_profile,
};
use crate::{queries, BankApplicationQueryDenial, BankReadControls};

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
    assert_denial_publication(
        &denial,
        WorthQueryApplicationAuthorizationExplanationCause::ElevationDenied,
        "worth.query.authorization.elevation-denied",
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

fn controls() -> BankReadControls {
    BankReadControls::current(request_scope(), 1, 20_000).unwrap()
}

fn assert_denial_publication(
    denial: &worth_query_host::facade::primary_graph::WorthQueryOperationAuthorizationDenial,
    expected_cause: WorthQueryApplicationAuthorizationExplanationCause,
    expected_code: &str,
) {
    let published = publish_application_authorization_denial(
        denial,
        WorthQueryApplicationAuthorizationPublicationProfile::exact(publication_profile()),
    )
    .unwrap();

    assert_eq!(published.artifact().denial(), denial);
    assert_eq!(published.artifact().cause(), expected_cause);
    assert_eq!(
        published.explanation().outcome_kind(),
        FoundationalDiagnosticOutcomeKind::Denied
    );
    assert_eq!(
        published.explanation().rows()[0].code().as_str(),
        expected_code
    );
    assert_eq!(
        published.denied_closeout_receipt().execution_posture(),
        FoundationalBoundaryEvidenceExecutionPosture::NotExecuted
    );
    assert_eq!(
        published.publication_receipt().execution_posture(),
        FoundationalBoundaryEvidenceExecutionPosture::Executed
    );
}
