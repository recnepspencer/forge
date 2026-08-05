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
        OTHER_ACCOUNT, OTHER_AUTHORITY,
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
    let wrong_purpose_denial = application
        .admit_capability_access(principal.query(), &capability, wrong_purpose, &request)
        .err()
        .expect("the wrong purpose must deny");
    assert_eq!(
        wrong_purpose_denial.kind(),
        WorthQueryOperationAuthorizationDenialKind::PurposeMismatch
    );
    assert!(wrong_purpose_denial.identity().is_some());
    assert_eq!(
        wrong_purpose_denial.causes(),
        [WorthQueryOperationAuthorizationDenialKind::PurposeMismatch]
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
        WorthQueryOperationAuthorizationDenialKind::CapabilityAuthorizationMissing
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
fn relation_and_amount_are_derived_from_the_exact_bank_action() {
    assert_wrong_account_denied();
    assert_amount_over_ceiling_denied();
}

#[test]
fn separation_of_duty_is_anchored_to_the_exact_action_authority() {
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

    let self_recognition = EstateAction::RecognizeExecutor {
        estate: ESTATE,
        executor: EXECUTOR,
        authority: AUTHORITY,
    };
    let self_recognition_denial = application
        .admit_capability_access(
            principal.query(),
            &capability,
            self_recognition,
            &request_scope(),
        )
        .err()
        .expect("the selected self-held authority must deny");
    assert_eq!(
        self_recognition_denial.kind(),
        WorthQueryOperationAuthorizationDenialKind::SeparationOfDutyRuleMatched
    );
    assert!(self_recognition_denial.identity().is_some());
    assert_eq!(
        self_recognition_denial.causes(),
        [WorthQueryOperationAuthorizationDenialKind::SeparationOfDutyRuleMatched]
    );

    let other_authority = EstateAction::RecognizeExecutor {
        estate: ESTATE,
        executor: EXECUTOR,
        authority: OTHER_AUTHORITY,
    };
    application
        .admit_capability_access(
            principal.query(),
            &capability,
            other_authority,
            &request_scope(),
        )
        .expect("an unrelated self-held authority must not poison the selected action");
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
        WorthQueryOperationAuthorizationDenialKind::CapabilityAuthorizationMissing
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
        WorthQueryOperationAuthorizationDenialKind::CapabilityAuthorizationMissing
    );
}
