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
        ViewRestrictedEstateOperation,
    },
};
use worth_query_host::facade::primary_graph::WorthQueryOperationAuthorizationDenialKind;

use super::{
    current_admission::view_action,
    fixture::{
        capability_world, request_scope, GrantSpec, ACCOUNT, AUTHORITY, ESTATE, EXECUTOR,
        OTHER_ACCOUNT,
    },
};

#[test]
fn purpose_field_resource_and_input_variant_cannot_understate_the_view_request() {
    let fixture = capability_world(
        "view-binding",
        GrantSpec::view(),
        EstateWorkflowStage::Administration,
        false,
        0,
    );
    let principal = fixture.authenticate();
    let application = fixture.runtime.application_runtime();
    let capability = application
        .installed_schema()
        .capability(
            ViewEstateAdministrationCapability::reference(),
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
            .admit_capability_access(principal.query(), &capability, wrong_purpose, &request)
            .err()
            .expect("the wrong purpose must deny")
            .kind(),
        WorthQueryOperationAuthorizationDenialKind::CapabilityProjectionRejected
    );
    let wrong_field = EstateAction::ViewRestrictedEstate {
        estate: ESTATE,
        field: RestrictedBankField::AccountDetails,
        purpose: EstateCapabilityPurpose::EstateAdministration,
    };
    assert_eq!(
        application
            .admit_capability_access(principal.query(), &capability, wrong_field, &request)
            .err()
            .expect("the wrong field must deny")
            .kind(),
        WorthQueryOperationAuthorizationDenialKind::PermissionDenied
    );
    let missing_resource = EstateAction::ViewRestrictedEstate {
        estate: bank_domain::estate::EstateCaseId::new(99_999).unwrap(),
        field: RestrictedBankField::CustomerIdentity,
        purpose: EstateCapabilityPurpose::EstateAdministration,
    };
    assert_eq!(
        application
            .admit_capability_access(principal.query(), &capability, missing_resource, &request)
            .err()
            .expect("the missing resource must deny")
            .kind(),
        WorthQueryOperationAuthorizationDenialKind::CapabilityProjectionRejected
    );
    assert_eq!(
        application
            .admit_capability_access(
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

#[test]
fn relation_amount_and_context_are_derived_from_the_exact_bank_action() {
    assert_wrong_account_denied();
    assert_amount_over_ceiling_denied();
    assert_self_recognition_context_denied();
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
        .admit_capability_access(
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

fn assert_wrong_account_denied() {
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
    assert_eq!(
        application
            .admit_capability_access(principal.query(), &capability, action, &request_scope())
            .err()
            .expect("the wrong related account must deny")
            .kind(),
        WorthQueryOperationAuthorizationDenialKind::PermissionDenied
    );
}

fn assert_amount_over_ceiling_denied() {
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
    assert_eq!(
        application
            .admit_capability_access(principal.query(), &capability, action, &request_scope())
            .err()
            .expect("an amount over the grant ceiling must deny")
            .kind(),
        WorthQueryOperationAuthorizationDenialKind::PermissionDenied
    );
}

fn assert_self_recognition_context_denied() {
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
    assert_eq!(
        application
            .admit_capability_access(principal.query(), &capability, action, &request_scope())
            .err()
            .expect("self-recognition must deny")
            .kind(),
        WorthQueryOperationAuthorizationDenialKind::PermissionDenied
    );
}
