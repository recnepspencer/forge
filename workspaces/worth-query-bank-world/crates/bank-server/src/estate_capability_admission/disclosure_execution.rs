use bank_domain::estate::{BankDisclosure, EstateWorkflowStage, RestrictedBankField};
use bank_domain::schema::{
    ViewEstateIdentityVerificationCapability, ViewRestrictedEstateOperation,
};
use worth_query_host::facade::domain::TypedApplicationValue;
use worth_query_host::facade::installed::domain_computation::WorthQueryApplicationQueryOmissionPosture;
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationDisclosureReceiptPosture, WorthQueryOperationAuthorizationDenialKind,
};

use super::fixture::{capability_world, request_scope, GrantSpec, DECEASED, ESTATE};
use crate::{queries, BankApplicationQueryDenial, BankReadControls};

#[test]
fn authenticated_bank_consumer_receives_only_the_governed_published_shape() {
    let fixture = capability_world(
        "governed-disclosure",
        GrantSpec::identity_verification(),
        EstateWorkflowStage::Administration,
        false,
        0,
    );
    let principal = fixture.authenticate();
    let controls = BankReadControls::current(request_scope(), 1, 256).unwrap();

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
        published.receipt().omission_posture(),
        WorthQueryApplicationQueryOmissionPosture::NoOmission
    );
    let disclosure = published.receipt().disclosure();
    assert_eq!(
        disclosure.posture(),
        WorthQueryApplicationDisclosureReceiptPosture::Governed
    );
    assert_eq!(
        disclosure.classification(),
        Some("estate-customer-identity")
    );
    assert_eq!(
        disclosure.disclosed(),
        &[RestrictedBankField::CustomerIdentity.into_foundational_value()]
    );
    assert!(disclosure.omitted().is_empty());
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

    let publication = published.receipt().inspect();
    assert!(publication.session_identity() > 0);
    assert!(publication.managed_run_identity() > 0);
    assert!(publication.admitted_plan_identity() > 0);
    assert_eq!(publication.result_count(), published.rows().len());
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
        false,
        0,
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
        WorthQueryOperationAuthorizationDenialKind::PermissionDenied
    );
}
