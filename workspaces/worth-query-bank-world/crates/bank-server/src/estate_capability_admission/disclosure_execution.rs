use bank_domain::estate::{BankDisclosure, EstateWorkflowStage, RestrictedBankField};
use bank_domain::schema::{
    ViewEstateIdentityVerificationCapability, ViewRestrictedEstateOperation,
};
use worth_query_host::facade::domain::TypedApplicationValue;
use worth_query_host::facade::installed::domain_computation::WorthQueryApplicationQueryOmissionPosture;
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationDisclosureOutcome, WorthQueryApplicationDisclosureReceipt,
    WorthQueryApplicationDisclosureReceiptPosture, WorthQueryOperationAuthorizationDenialKind,
};
use worth_query_host::facade::publication::domain_computation::{
    publish_application_field_omission, WorthQueryApplicationAuthorizationPublicationProfile,
    WorthQueryApplicationQueryPublicationInspection,
};

use super::fixture::{
    capability_world, governed_disclosure_world, request_scope, CapabilityFixture, GrantSpec,
    APPROVER, DECEASED, ESTATE, EXECUTOR,
};
use super::publication_evidence::{
    assert_authorization_denial_publication, assert_field_omission_publication,
    assert_omission_noninterference, publication_profile, ExpectedAuthorizationDenialPublication,
};
use crate::{queries, BankApplicationQueryDenial, BankReadControls};

#[test]
fn present_protected_beneficiary_value_cannot_influence_public_omission_material() {
    let first = governed_disclosure_world("governed-disclosure-hidden-twin", APPROVER);
    let second = governed_disclosure_world("governed-disclosure-hidden-twin", EXECUTOR);
    assert_eq!(first.source_beneficiaries(), vec![APPROVER]);
    assert_eq!(second.source_beneficiaries(), vec![EXECUTOR]);
    assert_ne!(first.source_beneficiaries(), second.source_beneficiaries());

    let first_principal = first.authenticate();
    let second_principal = second.authenticate();
    let first_result = first
        .runtime
        .query(queries::estate_customer_identity(ESTATE))
        .as_principal(&first_principal)
        .controls(BankReadControls::current(request_scope(), 1, 512).unwrap())
        .execute()
        .unwrap();
    let second_result = second
        .runtime
        .query(queries::estate_customer_identity(ESTATE))
        .as_principal(&second_principal)
        .controls(BankReadControls::current(request_scope(), 1, 512).unwrap())
        .execute()
        .unwrap();

    assert_eq!(first_result.rows(), second_result.rows());
    let first_disclosure = first_result.receipt().disclosure();
    let second_disclosure = second_result.receipt().disclosure();
    assert_eq!(first_disclosure.disclosed(), second_disclosure.disclosed());
    assert_eq!(first_disclosure.omitted(), second_disclosure.omitted());
    assert_eq!(first_disclosure.decisions(), second_disclosure.decisions());
    assert_ne!(
        first_disclosure.outcome_identity(),
        second_disclosure.outcome_identity()
    );

    let profile = publication_profile();
    let first_publication = publish_application_field_omission(
        first_disclosure,
        WorthQueryApplicationAuthorizationPublicationProfile::exact(profile),
    )
    .unwrap();
    let second_publication = publish_application_field_omission(
        second_disclosure,
        WorthQueryApplicationAuthorizationPublicationProfile::exact(profile),
    )
    .unwrap();
    assert_omission_noninterference(&first_publication, &second_publication, profile);
}

#[test]
fn authenticated_bank_consumer_receives_only_the_governed_published_shape() {
    let fixture = capability_world(
        "governed-disclosure",
        GrantSpec::identity_verification(),
        EstateWorkflowStage::Administration,
    );
    let principal = fixture.authenticate();
    let controls = BankReadControls::current(request_scope(), 1, 512).unwrap();

    let published = fixture
        .runtime
        .query(queries::estate_customer_identity(ESTATE))
        .as_principal(&principal)
        .controls(controls)
        .execute()
        .unwrap();

    assert_eq!(published.rows().len(), 1);
    assert_eq!(
        published.rows()[0].customer(),
        BankDisclosure::Disclosed(DECEASED)
    );
    assert_eq!(
        published.rows()[0].beneficiaries(),
        &BankDisclosure::Omitted(RestrictedBankField::BeneficiaryIdentity.classification())
    );
    assert_governed_disclosure(published.receipt().disclosure(), &fixture);
    assert_query_publication(published.receipt().inspect(), published.rows().len());
    assert_field_omission_publication(published.receipt().disclosure());
}

fn assert_governed_disclosure(
    disclosure: &WorthQueryApplicationDisclosureReceipt,
    fixture: &CapabilityFixture,
) {
    assert_eq!(
        disclosure.posture(),
        WorthQueryApplicationDisclosureReceiptPosture::Governed
    );
    assert_eq!(
        disclosure.classification(),
        Some("estate-customer-identity")
    );
    assert_disclosure_decisions(disclosure);
    assert_disclosure_authority(disclosure, fixture);
}

fn assert_disclosure_decisions(disclosure: &WorthQueryApplicationDisclosureReceipt) {
    assert_eq!(
        disclosure.disclosed(),
        &[
            RestrictedBankField::CustomerIdentity.into_foundational_value(),
            RestrictedBankField::CustomerIdentity.into_foundational_value(),
        ]
    );
    assert_eq!(
        disclosure.omitted(),
        &[
            RestrictedBankField::BeneficiaryIdentity.into_foundational_value(),
            RestrictedBankField::BeneficiaryIdentity.into_foundational_value(),
        ]
    );
    let decisions = disclosure.decisions();
    assert_eq!(decisions.len(), 4);
    assert_eq!(disclosure.disclosure_decision_count(), decisions.len());
    assert!(decisions[0].slot() < decisions[1].slot());
    assert_eq!(
        decisions
            .iter()
            .filter(|decision| {
                decision.outcome() == WorthQueryApplicationDisclosureOutcome::Disclosed
            })
            .count(),
        2
    );
    assert_eq!(
        decisions
            .iter()
            .filter(|decision| {
                decision.outcome() == WorthQueryApplicationDisclosureOutcome::Omitted
            })
            .count(),
        2
    );
}

fn assert_disclosure_authority(
    disclosure: &WorthQueryApplicationDisclosureReceipt,
    fixture: &CapabilityFixture,
) {
    let installed_capability = fixture
        .runtime
        .application_runtime()
        .installed_schema()
        .capability(
            ViewEstateIdentityVerificationCapability::reference(),
            ViewRestrictedEstateOperation::reference(),
        )
        .unwrap();
    assert_eq!(
        disclosure.capability_authority_identity(),
        Some(installed_capability.authority_identity())
    );
    assert!(disclosure.decision_identity().is_some());
    assert!(disclosure.authorization_decision_fact_count() > 0);
}

fn assert_query_publication(
    publication: WorthQueryApplicationQueryPublicationInspection<'_>,
    result_count: usize,
) {
    assert_eq!(
        publication.terminal().omission_posture(),
        WorthQueryApplicationQueryOmissionPosture::GovernedOmission
    );
    assert!(publication.session_identity() > 0);
    assert!(publication.managed_run_identity() > 0);
    assert!(publication.admitted_plan_identity() > 0);
    assert_eq!(publication.result_count(), result_count);
    assert_eq!(
        publication.relational_branch(),
        &publication.terminal().basis_identity().branch_id().0
    );
    assert!(publication.terminal_resources_released());
    assert_eq!(publication.publication_canonical_entries(), 0);
    assert_eq!(publication.publication_sha256_compression_blocks(), 0);
    assert_eq!(publication.publication_identity_text_materializations(), 0);
}

#[test]
fn bank_consumer_cannot_repurpose_an_administration_grant_for_identity_disclosure() {
    let fixture = capability_world(
        "wrong-purpose-disclosure",
        GrantSpec::view(),
        EstateWorkflowStage::Administration,
    );
    let principal = fixture.authenticate();
    let controls = BankReadControls::current(request_scope(), 1, 256).unwrap();

    let denial = match fixture
        .runtime
        .query(queries::estate_customer_identity(ESTATE))
        .as_principal(&principal)
        .controls(controls)
        .execute()
    {
        Ok(_) => panic!("wrong-purpose grant must not publish identity disclosure"),
        Err(denial) => denial,
    };

    let BankApplicationQueryDenial::CapabilityAdmission(denial) = denial else {
        panic!("wrong-purpose grant must fail at capability admission")
    };
    assert_eq!(
        denial.kind(),
        WorthQueryOperationAuthorizationDenialKind::CapabilityAuthorizationMissing
    );
    assert_authorization_denial_publication(
        &denial,
        ExpectedAuthorizationDenialPublication {
            cause: worth_query_host::facade::primary_graph::WorthQueryApplicationAuthorizationExplanationCause::MissingCapability,
            code: "worth.query.authorization.missing-capability",
        },
    );
}
