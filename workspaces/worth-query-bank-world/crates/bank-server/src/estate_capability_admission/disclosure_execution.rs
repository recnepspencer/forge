use bank_domain::estate::{BankDisclosure, EstateWorkflowStage, RestrictedBankField};
use bank_domain::schema::{
    ViewEstateIdentityVerificationCapability, ViewRestrictedEstateOperation,
};
use worth_foundational::facade::{
    FoundationalBoundaryArtifactCategory, FoundationalBoundaryEvidenceCloseoutDisposition,
    FoundationalBoundaryEvidenceExecutionPosture, FoundationalBoundaryEvidenceReceiptKind,
    FoundationalDiagnosticDenialClass, FoundationalDiagnosticOutcomeKind,
};
use worth_query_host::facade::domain::TypedApplicationValue;
use worth_query_host::facade::installed::domain_computation::WorthQueryApplicationQueryOmissionPosture;
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationAuthorizationExplanationCause, WorthQueryApplicationDisclosureOutcome,
    WorthQueryApplicationDisclosureReceipt, WorthQueryApplicationDisclosureReceiptPosture,
    WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind,
};
use worth_query_host::facade::publication::domain_computation::{
    publish_application_authorization_denial, publish_application_field_omission,
    WorthQueryApplicationAuthorizationPublicationProfile,
    WorthQueryApplicationQueryPublicationInspection,
};

use super::fixture::{
    capability_world, request_scope, CapabilityFixture, GrantSpec, DECEASED, ESTATE,
};
use super::publication_evidence::publication_profile;
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

fn assert_field_omission_publication(disclosure: &WorthQueryApplicationDisclosureReceipt) {
    let omission = publish_application_field_omission(
        disclosure,
        WorthQueryApplicationAuthorizationPublicationProfile::exact(publication_profile()),
    )
    .unwrap();
    assert_eq!(
        omission.boundary_category(),
        FoundationalBoundaryArtifactCategory::Artifact
    );
    assert_eq!(omission.artifact().disclosure(), disclosure);
    let locator = omission
        .provenance()
        .source_basis()
        .boundary_artifact_locator()
        .unwrap();
    assert_eq!(
        locator.artifact_id().get(),
        disclosure.outcome_identity().unwrap().get()
    );
    assert_eq!(
        omission.explanation().outcome_kind(),
        FoundationalDiagnosticOutcomeKind::Partial
    );
    assert_eq!(
        omission.publication_receipt().receipt_kind(),
        FoundationalBoundaryEvidenceReceiptKind::Publication
    );
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
        WorthQueryOperationAuthorizationDenialKind::CapabilityAuthorizationMissing
    );
    assert_authorization_denial_publication(&denial);
}

fn assert_authorization_denial_publication(denial: &WorthQueryOperationAuthorizationDenial) {
    let published = publish_application_authorization_denial(
        denial,
        WorthQueryApplicationAuthorizationPublicationProfile::exact(publication_profile()),
    )
    .unwrap();
    assert_eq!(published.artifact().denial(), denial);
    assert_eq!(
        published.artifact().cause(),
        WorthQueryApplicationAuthorizationExplanationCause::MissingCapability
    );
    assert_eq!(
        published.boundary_category(),
        FoundationalBoundaryArtifactCategory::Artifact
    );
    assert_eq!(
        published.explanation().outcome_kind(),
        FoundationalDiagnosticOutcomeKind::Denied
    );
    assert_eq!(
        published.explanation().rows()[0].code().as_str(),
        "worth.query.authorization.missing-capability"
    );
    let worth_foundational::facade::FoundationalDiagnosticRow::Decision(row) =
        &published.explanation().rows()[0]
    else {
        panic!("authorization denial must publish one decision row");
    };
    assert_eq!(
        row.denial_class(),
        Some(FoundationalDiagnosticDenialClass::PolicyDenied)
    );
    assert_eq!(
        published.denied_closeout_receipt().closeout_disposition(),
        Some(FoundationalBoundaryEvidenceCloseoutDisposition::Denied)
    );
    assert_eq!(
        published.denied_closeout_receipt().execution_posture(),
        FoundationalBoundaryEvidenceExecutionPosture::NotExecuted
    );
    assert!(!published.denied_closeout_receipt().did_execute());
    assert_eq!(
        published.publication_receipt().execution_posture(),
        FoundationalBoundaryEvidenceExecutionPosture::Executed
    );
}
