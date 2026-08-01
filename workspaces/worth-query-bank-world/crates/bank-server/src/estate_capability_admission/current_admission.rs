use std::num::NonZeroUsize;

use bank_domain::{
    estate::{
        CapabilityGrantStatus, EstateAction, EstateCapabilityPurpose, EstateDecision,
        EstateWorkflowStage, RestrictedBankField,
    },
    schema::{ViewEstateIdentityVerificationCapability, ViewRestrictedEstateOperation},
};
use worth_query_host::facade::{
    domain::TypedApplicationValue,
    primary_graph::{
        WorthQueryApplicationQueryAdmissionDenialKind, WorthQueryApplicationQueryControls,
        WorthQueryOperationAuthorizationDenialKind,
    },
};

use super::fixture::{capability_world, request_scope, GrantSpec, ESTATE};
use crate::{
    application_query::execute_estate_customer_disclosure_with, BankApplicationQueryDenial,
};

#[test]
fn active_bank_grant_mints_authority_only_when_the_operation_session_consumes_it() {
    let fixture = capability_world(
        "active-capability",
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
    let prepared = application
        .prepare_capability_access(
            principal.query(),
            &capability,
            identity_disclosure_action(),
            &request,
        )
        .unwrap();

    assert_eq!(
        fixture.oracle_decision(identity_disclosure_action()),
        EstateDecision::Allowed
    );
    assert_eq!(prepared.operation(), "ViewRestrictedEstateOperation");
    assert_eq!(
        prepared.projected_request().field_value(),
        Some(&RestrictedBankField::CustomerIdentity.into_foundational_value())
    );
    assert_zero_canonical_work(prepared.admission_canonical_work());
    assert_eq!(capability.lookup_evidence().basis_preparations(), 0);
    assert_eq!(capability.lookup_evidence().digest_derivations(), 0);
    assert_eq!(
        capability.lookup_evidence().digest_text_materializations(),
        0
    );

    drop(prepared);
    let (_, evidence) = execute_estate_customer_disclosure_with(
        &fixture.runtime,
        &principal,
        ESTATE,
        query_controls(&request),
        |plan| {
            (
                plan.authorization_decision_fact_count(),
                plan.capability_time_sample()
                    .expect("governed consumption retains the trusted session sample")
                    .semantic_byte_width(),
                plan.authorization_work(),
                plan.canonical_work(),
            )
        },
    )
    .unwrap();

    assert_eq!(evidence.0, 4);
    assert_eq!(evidence.1, 9);
    assert_eq!(evidence.2.requirement_count(), 2);
    assert!(evidence.2.paths_evaluated() > 0);
    assert!(evidence.2.signal_dependency_count() > 0);
    assert!(evidence.3.admission().basis_preparations() > 0);
    assert!(evidence.3.admission().digest_derivations() > 0);
    for warm_phase in [
        evidence.3.execution(),
        evidence.3.provider_commit(),
        evidence.3.projection(),
        evidence.3.live_delivery(),
        evidence.3.retry_resolution(),
        evidence.3.recovery_inspection(),
        evidence.3.publication(),
    ] {
        assert_zero_canonical_work(warm_phase);
    }
}

#[test]
fn revoked_expired_future_and_workflow_drifted_grants_fail_current_admission() {
    let mut revoked = GrantSpec::identity_verification();
    revoked.status = CapabilityGrantStatus::Revoked;
    assert_eq!(
        disclosure_denial("revoked", revoked, EstateWorkflowStage::Administration),
        WorthQueryOperationAuthorizationDenialKind::PermissionDenied
    );

    let mut expired = GrantSpec::identity_verification();
    expired.not_after = 1;
    assert_eq!(
        disclosure_denial("expired", expired, EstateWorkflowStage::Administration),
        WorthQueryOperationAuthorizationDenialKind::PermissionDenied
    );

    let mut future = GrantSpec::identity_verification();
    future.not_before = u64::MAX;
    assert_eq!(
        disclosure_denial("future", future, EstateWorkflowStage::Administration),
        WorthQueryOperationAuthorizationDenialKind::PermissionDenied
    );

    let mut workflow = GrantSpec::identity_verification();
    workflow.workflow = EstateWorkflowStage::AuthorityReview;
    assert_eq!(
        disclosure_denial(
            "workflow-drift",
            workflow,
            EstateWorkflowStage::Administration
        ),
        WorthQueryOperationAuthorizationDenialKind::PermissionDenied
    );
}

fn disclosure_denial(
    scenario: &str,
    spec: GrantSpec,
    stage: EstateWorkflowStage,
) -> WorthQueryOperationAuthorizationDenialKind {
    let fixture = capability_world(scenario, spec, stage, false, 0);
    assert!(matches!(
        fixture.oracle_decision(identity_disclosure_action()),
        EstateDecision::Denied(_)
    ));
    let principal = fixture.authenticate();
    let request = request_scope();
    let denial = execute_estate_customer_disclosure_with(
        &fixture.runtime,
        &principal,
        ESTATE,
        query_controls(&request),
        |_| (),
    )
    .err()
    .expect("the hostile grant should deny only inside its query session");
    let BankApplicationQueryDenial::Admission(denial) = denial else {
        panic!("current capability policy must deny during governed query admission")
    };
    let WorthQueryApplicationQueryAdmissionDenialKind::Authorization(kind) = denial.kind() else {
        panic!("current capability policy must preserve its authorization denial")
    };
    kind
}

pub(super) fn identity_disclosure_action() -> EstateAction {
    EstateAction::ViewRestrictedEstate {
        estate: ESTATE,
        field: RestrictedBankField::CustomerIdentity,
        purpose: EstateCapabilityPurpose::IdentityVerification,
    }
}

pub(super) fn view_action() -> EstateAction {
    EstateAction::ViewRestrictedEstate {
        estate: ESTATE,
        field: RestrictedBankField::CustomerIdentity,
        purpose: EstateCapabilityPurpose::EstateAdministration,
    }
}

pub(super) fn query_controls<'a>(
    request: &'a worth_query_host::facade::admission::authenticated_principal::WorthQueryRequestScope,
) -> WorthQueryApplicationQueryControls<'a, bank_domain::schema::BankSchema> {
    WorthQueryApplicationQueryControls::current_one_shot(
        NonZeroUsize::new(1).unwrap(),
        NonZeroUsize::new(256).unwrap(),
        request,
    )
}

fn assert_zero_canonical_work(
    work: worth_query_host::facade::domain::WorthQueryCanonicalWorkEvidence,
) {
    assert_eq!(work.basis_preparations(), 0);
    assert_eq!(work.digest_derivations(), 0);
    assert_eq!(work.canonical_entries(), 0);
    assert_eq!(work.canonical_encoded_bytes(), 0);
    assert_eq!(work.canonical_material_allocation_bytes(), 0);
    assert_eq!(work.sha256_input_bytes(), 0);
    assert_eq!(work.sha256_compression_blocks(), 0);
    assert_eq!(work.digest_text_materializations(), 0);
}
