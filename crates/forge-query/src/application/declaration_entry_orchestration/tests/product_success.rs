use crate::application::{
    ForgeQueryDeclarationEntryOrchestrationProduct, ForgeQueryDeclarationEntryOrchestrationStage,
    ForgeQueryDeclarationRouteIntent,
};
use crate::target_binding::{ForgeQueryBindingTargetKind, ForgeQueryBindingTargetWitness};

use super::domain::{admitted_handle, AdmittedFamily, Input};

#[test]
fn route_product_orchestration_matches_explicit_route_plan_digest() {
    let handle = admitted_handle("collaborative");
    let progressed = handle
        .declare_review_and_progress(Input::<AdmittedFamily>::new("edge:42"))
        .unwrap_or_else(|_| panic!("progression should admit"));

    let explicit = handle
        .plan_routes_from_progressed(progressed.clone())
        .unwrap_or_else(|_| panic!("explicit route plan should succeed"));
    let orchestrated = handle
        .orchestrate_routes_from_progressed(progressed.clone())
        .unwrap_or_else(|_| panic!("orchestrated route plan should succeed"));
    let proof = handle.orchestrate_routes_from_progressed_proof(progressed);

    assert_eq!(
        explicit.route_plan_digest(),
        orchestrated.route_plan_digest()
    );
    assert_eq!(
        proof.plan().product(),
        ForgeQueryDeclarationEntryOrchestrationProduct::RoutePlan
    );
    assert_eq!(
        proof.plan().starting_artifact_stage(),
        ForgeQueryDeclarationEntryOrchestrationStage::ProgressionAdmitted
    );
    assert_eq!(
        proof
            .step_records()
            .last()
            .expect("route proof should stop at route")
            .stage(),
        ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned
    );
}

#[test]
fn receipt_product_orchestration_matches_explicit_receipt_digest() {
    let handle = admitted_handle("collaborative");
    let progressed = handle
        .declare_review_and_progress(Input::<AdmittedFamily>::new("edge:42"))
        .unwrap_or_else(|_| panic!("progression should admit"));

    let explicit = handle
        .receipt_routes_from_progressed(progressed.clone())
        .unwrap_or_else(|_| panic!("explicit receipt should succeed"));
    let orchestrated = handle
        .orchestrate_receipt_from_progressed(progressed.clone())
        .unwrap_or_else(|_| panic!("orchestrated receipt should succeed"));
    let proof = handle.orchestrate_receipt_from_progressed_proof(progressed);

    assert_eq!(explicit.receipt_digest(), orchestrated.receipt_digest());
    assert_eq!(
        proof.plan().product(),
        ForgeQueryDeclarationEntryOrchestrationProduct::Receipt
    );
    assert_eq!(
        proof
            .step_records()
            .last()
            .expect("receipt proof should stop at receipt")
            .stage(),
        ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued
    );
}

#[test]
fn envelope_product_orchestration_matches_explicit_envelope_digest_and_intent() {
    let handle = admitted_handle("collaborative");
    let progressed = handle
        .declare_review_and_progress(Input::<AdmittedFamily>::new("edge:42"))
        .unwrap_or_else(|_| panic!("progression should admit"));

    let explicit = handle
        .envelope_routes_from_progressed_with_intent(
            progressed.clone(),
            ForgeQueryDeclarationRouteIntent::RelationalOnly,
        )
        .unwrap_or_else(|_| panic!("explicit envelope should succeed"));
    let orchestrated = handle
        .orchestrate_envelope_from_progressed_with_intent(
            progressed.clone(),
            ForgeQueryDeclarationRouteIntent::RelationalOnly,
        )
        .unwrap_or_else(|_| panic!("orchestrated envelope should succeed"));
    let proof = handle.orchestrate_envelope_from_progressed_proof_with_intent(
        progressed,
        ForgeQueryDeclarationRouteIntent::RelationalOnly,
    );

    assert_eq!(explicit.envelope_digest(), orchestrated.envelope_digest());
    assert_eq!(
        proof.plan().product(),
        ForgeQueryDeclarationEntryOrchestrationProduct::Envelope
    );
    assert_eq!(
        proof.plan().requested_route_intent(),
        Some(ForgeQueryDeclarationRouteIntent::RelationalOnly)
    );
    assert_eq!(
        proof
            .step_records()
            .last()
            .expect("envelope proof should stop at envelope")
            .stage(),
        ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed
    );
}

#[test]
fn declaration_entry_artifacts_expose_stable_binding_targets() {
    let handle = admitted_handle("collaborative");
    let progressed = handle
        .declare_review_and_progress(Input::<AdmittedFamily>::new("edge:42"))
        .unwrap_or_else(|_| panic!("progression should admit"));
    let route = handle
        .plan_routes_from_progressed(progressed.clone())
        .unwrap_or_else(|_| panic!("route plan should succeed"));
    let receipt = handle
        .receipt_routes_from_progressed(progressed.clone())
        .unwrap_or_else(|_| panic!("receipt should succeed"));
    let envelope = handle
        .envelope_routes_from_progressed(progressed.clone())
        .unwrap_or_else(|_| panic!("envelope should succeed"));

    let progressed_target = progressed.binding_target();
    assert_eq!(
        progressed_target.kind(),
        ForgeQueryBindingTargetKind::AdmittedDeclarationProgression
    );
    assert_eq!(
        progressed_target.target_digest(),
        progressed.progression_digest()
    );

    let route_target = route.binding_target();
    assert_eq!(
        route_target.kind(),
        ForgeQueryBindingTargetKind::DeclarationRoutePlan
    );
    assert_eq!(route_target.target_digest(), route.route_plan_digest());

    let receipt_target = receipt.binding_target();
    assert_eq!(
        receipt_target.kind(),
        ForgeQueryBindingTargetKind::DeclarationReceipt
    );
    assert_eq!(
        receipt_target.target_digest(),
        format!("{:?}", receipt.receipt_digest())
    );

    let envelope_target = envelope.binding_target();
    assert_eq!(
        envelope_target.kind(),
        ForgeQueryBindingTargetKind::DeclarationEnvelope
    );
    assert_eq!(
        envelope_target.target_digest(),
        format!("{:?}", envelope.envelope_digest())
    );
}
