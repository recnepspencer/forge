use bank_domain::{
    estate::{
        EstateAction, EstateCapabilityPurpose, EstateDisbursement, EstatePosting,
        EstateWorkflowStage, RestrictedBankField,
    },
    model::{Money, SignedMoney, USD},
    schema::{
        DisburseEstateCapability, DisburseEstateOperation, FreezeEstateAccountCapability,
        FreezeEstateAccountOperation, RecognizeEstateExecutorCapability,
        RecognizeEstateExecutorOperation, ViewEstateAdministrationCapability,
        ViewEstateIdentityVerificationCapability, ViewRestrictedEstateOperation,
    },
};
use worth_query_host::facade::domain::TypedApplicationValue;
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationQueryAdmissionDenialKind, WorthQueryOperationAuthorizationDenialKind,
};

use super::{
    current_admission::{query_controls, view_action},
    fixture::{
        capability_world, request_scope, GrantSpec, ACCOUNT, AUTHORITY, ESTATE, EXECUTOR,
        OTHER_ACCOUNT,
    },
};
use crate::{
    application_query::execute_estate_customer_disclosure_action_with, BankApplicationQueryDenial,
};

#[test]
fn static_and_graph_bound_values_cannot_understate_identity_disclosure() {
    let fixture = capability_world(
        "view-binding",
        GrantSpec::identity_verification(),
        EstateWorkflowStage::Administration,
        false,
        0,
    );
    let principal = fixture.authenticate();
    let application = fixture.runtime.application_runtime();
    let capability = application
        .installed_schema()
        .capability(
            ViewEstateIdentityVerificationCapability::reference(),
            ViewRestrictedEstateOperation::reference(),
        )
        .unwrap();
    let request = request_scope();

    let wrong_purpose = EstateAction::ViewRestrictedEstate {
        estate: ESTATE,
        field: RestrictedBankField::CustomerIdentity,
        purpose: EstateCapabilityPurpose::LegalCompliance,
    };
    assert_eq!(
        application
            .prepare_capability_access(principal.query(), &capability, wrong_purpose, &request)
            .err()
            .expect("the wrong purpose must deny")
            .kind(),
        WorthQueryOperationAuthorizationDenialKind::CapabilityProjectionRejected
    );
    let wrong_field = EstateAction::ViewRestrictedEstate {
        estate: ESTATE,
        field: RestrictedBankField::AccountDetails,
        purpose: EstateCapabilityPurpose::IdentityVerification,
    };
    application
        .prepare_capability_access(principal.query(), &capability, wrong_field, &request)
        .expect("field value is graph policy, not static request shape");
    assert_governed_action_denial(
        &fixture,
        &principal,
        &request,
        wrong_field,
        WorthQueryOperationAuthorizationDenialKind::PermissionDenied,
    );
    let missing_resource = EstateAction::ViewRestrictedEstate {
        estate: bank_domain::estate::EstateCaseId::new(99_999).unwrap(),
        field: RestrictedBankField::CustomerIdentity,
        purpose: EstateCapabilityPurpose::IdentityVerification,
    };
    application
        .prepare_capability_access(principal.query(), &capability, missing_resource, &request)
        .expect("entity existence is graph truth, not static request shape");
    assert_governed_action_denial(
        &fixture,
        &principal,
        &request,
        missing_resource,
        WorthQueryOperationAuthorizationDenialKind::CapabilityProjectionRejected,
    );
    assert_eq!(
        application
            .prepare_capability_access(
                principal.query(),
                &capability,
                EstateAction::FreezeAccount {
                    estate: ESTATE,
                    account: ACCOUNT,
                },
                &request,
            )
            .err()
            .expect("the wrong action variant must deny")
            .kind(),
        WorthQueryOperationAuthorizationDenialKind::CapabilityProjectionRejected
    );
}

fn assert_governed_action_denial(
    fixture: &super::fixture::CapabilityFixture,
    principal: &crate::BankAuthenticatedPrincipal,
    request: &worth_query_host::facade::admission::authenticated_principal::WorthQueryRequestScope,
    action: EstateAction,
    expected: WorthQueryOperationAuthorizationDenialKind,
) {
    let denial = execute_estate_customer_disclosure_action_with(
        &fixture.runtime,
        principal,
        ESTATE,
        action,
        query_controls(request),
        |_| (),
    )
    .err()
    .expect("hostile disclosure input must deny inside the query session");
    let BankApplicationQueryDenial::Admission(denial) = denial else {
        panic!("hostile disclosure input must fail during governed admission")
    };
    assert_eq!(
        denial.kind(),
        WorthQueryApplicationQueryAdmissionDenialKind::Authorization(expected)
    );
}

#[test]
fn relation_amount_and_context_are_retained_without_pre_session_authority() {
    assert_related_account_retained();
    assert_amount_retained();
    assert_legal_authority_context_retained();
}

#[test]
fn installed_capability_authority_is_runtime_affine() {
    let source = capability_world(
        "foreign-capability-source",
        GrantSpec::view(),
        EstateWorkflowStage::Administration,
        false,
        0,
    );
    let target = capability_world(
        "foreign-capability-target",
        GrantSpec::view(),
        EstateWorkflowStage::Administration,
        false,
        0,
    );
    let capability = source
        .runtime
        .application_runtime()
        .installed_schema()
        .capability(
            ViewEstateAdministrationCapability::reference(),
            ViewRestrictedEstateOperation::reference(),
        )
        .unwrap();
    let principal = target.authenticate();
    let denial = target
        .runtime
        .application_runtime()
        .prepare_capability_access(
            principal.query(),
            &capability,
            view_action(),
            &request_scope(),
        )
        .err()
        .expect("a foreign installed capability must deny");
    assert!(matches!(
        denial.kind(),
        WorthQueryOperationAuthorizationDenialKind::StaleInstalledSchema
            | WorthQueryOperationAuthorizationDenialKind::StaleInstalledOperation
    ));
}

fn assert_related_account_retained() {
    let fixture = capability_world(
        "wrong-account",
        GrantSpec::freeze(),
        EstateWorkflowStage::Administration,
        false,
        0,
    );
    let principal = fixture.authenticate();
    let application = fixture.runtime.application_runtime();
    let capability = application
        .installed_schema()
        .capability(
            FreezeEstateAccountCapability::reference(),
            FreezeEstateAccountOperation::reference(),
        )
        .unwrap();
    let action = EstateAction::FreezeAccount {
        estate: ESTATE,
        account: OTHER_ACCOUNT,
    };
    let prepared = application
        .prepare_capability_access(principal.query(), &capability, action, &request_scope())
        .expect("preparation retains relation input without evaluating the grant");
    assert_eq!(
        prepared
            .projected_request()
            .related()
            .expect("freeze input carries its related account")
            .selector()
            .value(),
        &OTHER_ACCOUNT.into_foundational_value()
    );
    assert_zero_preparation_hashing(prepared.admission_canonical_work());
}

fn assert_amount_retained() {
    let fixture = capability_world(
        "amount-ceiling",
        GrantSpec::disburse(1_000),
        EstateWorkflowStage::Administration,
        false,
        0,
    );
    let principal = fixture.authenticate();
    let application = fixture.runtime.application_runtime();
    let capability = application
        .installed_schema()
        .capability(
            DisburseEstateCapability::reference(),
            DisburseEstateOperation::reference(),
        )
        .unwrap();
    let amount = Money::<USD>::from_minor(1_001).unwrap();
    let action = EstateAction::DisburseEstate(EstateDisbursement {
        estate: ESTATE,
        source_account: ACCOUNT,
        destination_account: OTHER_ACCOUNT,
        beneficiary: EXECUTOR,
        amount,
        postings: [
            EstatePosting {
                account: ACCOUNT,
                amount: SignedMoney::from_minor(-1_001),
            },
            EstatePosting {
                account: OTHER_ACCOUNT,
                amount: SignedMoney::from_minor(1_001),
            },
        ],
    });
    let prepared = application
        .prepare_capability_access(principal.query(), &capability, action, &request_scope())
        .expect("preparation retains amount input without evaluating the grant ceiling");
    assert_eq!(
        prepared.projected_request().amount_value(),
        Some(&amount.into_foundational_value())
    );
    assert_zero_preparation_hashing(prepared.admission_canonical_work());
}

fn assert_legal_authority_context_retained() {
    let fixture = capability_world(
        "context-anchor",
        GrantSpec::recognize(),
        EstateWorkflowStage::Administration,
        true,
        0,
    );
    let principal = fixture.authenticate();
    let application = fixture.runtime.application_runtime();
    let capability = application
        .installed_schema()
        .capability(
            RecognizeEstateExecutorCapability::reference(),
            RecognizeEstateExecutorOperation::reference(),
        )
        .unwrap();
    let action = EstateAction::RecognizeExecutor {
        estate: ESTATE,
        executor: EXECUTOR,
        authority: AUTHORITY,
    };
    let prepared = application
        .prepare_capability_access(principal.query(), &capability, action, &request_scope())
        .expect("preparation retains context input without evaluating conflict policy");
    let context = prepared.projected_request().context_value().entities();
    assert_eq!(context.len(), 1);
    assert_eq!(
        context[0].selector().value(),
        &AUTHORITY.into_foundational_value()
    );
    assert_zero_preparation_hashing(prepared.admission_canonical_work());
}

fn assert_zero_preparation_hashing(
    work: worth_query_host::facade::domain::WorthQueryCanonicalWorkEvidence,
) {
    assert_eq!(work.basis_preparations(), 0);
    assert_eq!(work.digest_derivations(), 0);
    assert_eq!(work.canonical_encoded_bytes(), 0);
    assert_eq!(work.sha256_input_bytes(), 0);
}
