use bank_domain::estate::{
    CapabilityGrantStatus, EstateCapabilityOperation, EstateCapabilityPurpose, EstateWorkflowStage,
    RestrictedBankField,
};
use bank_domain::queries::EstateGovernanceQuery;
use bank_domain::reads::EstateGovernanceContext;
use worth_query_host::facade::declaration::application_schema::TypedApplicationValue;
use worth_query_host::facade::installed::domain_computation::WorthQueryApplicationQueryOmissionPosture;
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationDisclosureOutcome, WorthQueryApplicationDisclosureReceiptPosture,
    WorthQueryApplicationOneShotResult, WorthQueryOperationAuthorizationDenialKind,
};

use super::fixture::{
    capability_world, request_scope, GrantSpec, DECEASED, ESTATE, GRANT, SPECIALIST,
};
use crate::{queries, BankApplicationQueryDenial, BankReadControls};

type GovernanceResult =
    WorthQueryApplicationOneShotResult<EstateGovernanceQuery, EstateGovernanceContext>;

#[test]
fn public_estate_governance_query_consumes_the_exact_administration_capability() {
    let fixture = capability_world(
        "governed-estate-query",
        GrantSpec::governance_view(),
        EstateWorkflowStage::Administration,
        false,
        0,
    );
    let principal = fixture.authenticate();
    let result = fixture
        .runtime
        .query(queries::estate_governance_context(ESTATE))
        .as_principal(&principal)
        .controls(controls())
        .execute()
        .expect("the exact administration capability should govern the estate query");

    assert_governance_context(&result);
    assert_governance_receipt(&result);
}

fn assert_governance_context(result: &GovernanceResult) {
    assert_eq!(result.rows().len(), 1);
    let context = &result.rows()[0];
    assert_eq!(context.estate(), ESTATE);
    assert_eq!(context.stage(), EstateWorkflowStage::Administration);
    assert!(context.beneficiaries().is_empty());
    assert_eq!(context.assignments().len(), 3);
    assert_eq!(context.capabilities().len(), 1);
    let capability = &context.capabilities()[0];
    assert_eq!(capability.id(), GRANT);
    assert_eq!(
        capability.operation(),
        EstateCapabilityOperation::ViewRestrictedEstate
    );
    assert_eq!(
        capability.purpose(),
        EstateCapabilityPurpose::EstateAdministration
    );
    assert_eq!(
        capability.workflow_stage(),
        EstateWorkflowStage::Administration
    );
    assert_eq!(capability.status(), CapabilityGrantStatus::Active);
    assert_eq!(capability.grantee(), SPECIALIST);
    assert_eq!(capability.grantor(), DECEASED);
    assert!(capability.emergencies().is_empty());
}

fn assert_governance_receipt(result: &GovernanceResult) {
    let disclosure = result.receipt().disclosure();
    assert_eq!(
        disclosure.posture(),
        WorthQueryApplicationDisclosureReceiptPosture::Governed
    );
    assert_eq!(
        disclosure.classification(),
        Some("estate-governance-context")
    );
    assert_eq!(disclosure.decisions().len(), 30);
    assert!(disclosure.omitted().is_empty());
    for decision in disclosure.decisions() {
        assert_eq!(
            decision.required_disclosure(),
            &RestrictedBankField::GovernanceMetadata.into_foundational_value()
        );
        assert_eq!(
            decision.outcome(),
            WorthQueryApplicationDisclosureOutcome::Disclosed
        );
    }
    assert!(disclosure.authorization_decision_fact_count() > 0);
    assert_eq!(
        result.receipt().omission_posture(),
        WorthQueryApplicationQueryOmissionPosture::NoOmission
    );
    assert_eq!(result.receipt().fallback_count(), 0);
}

#[test]
fn public_estate_governance_query_rejects_one_axis_purpose_and_field_substitution() {
    let mut wrong_purpose = GrantSpec::governance_view();
    wrong_purpose.purpose = EstateCapabilityPurpose::IdentityVerification;
    assert_governance_denied("governance-wrong-purpose", wrong_purpose);

    let mut wrong_field = GrantSpec::governance_view();
    wrong_field.field = Some(RestrictedBankField::CustomerIdentity);
    assert_governance_denied("governance-wrong-field", wrong_field);
}

fn assert_governance_denied(scenario: &str, grant: GrantSpec) {
    let fixture = capability_world(
        scenario,
        grant,
        EstateWorkflowStage::Administration,
        false,
        0,
    );
    let principal = fixture.authenticate();
    let denial = fixture
        .runtime
        .query(queries::estate_governance_context(ESTATE))
        .as_principal(&principal)
        .controls(controls())
        .execute()
        .err()
        .expect("a substituted grant dimension must deny the governed query");

    let BankApplicationQueryDenial::CapabilityAdmission(denial) = denial else {
        panic!("the substituted grant must fail at capability admission: {denial:#?}")
    };
    assert_eq!(
        denial.kind(),
        WorthQueryOperationAuthorizationDenialKind::PermissionDenied
    );
}

fn controls() -> BankReadControls {
    BankReadControls::current(request_scope(), 1, 20_000).unwrap()
}
