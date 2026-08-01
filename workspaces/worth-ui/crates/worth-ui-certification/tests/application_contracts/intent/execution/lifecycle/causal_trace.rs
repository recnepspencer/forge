use worth_ui::facade::inspection::{
    UiIntentCausalTraceAttemptPosture, UiIntentCausalTraceEvidence, UiIntentEvidenceLookup,
    UiIntentEvidenceReference,
};
use worth_ui::facade::intent::{
    UiIntentExecutionDispatchOutcome, UiIntentExecutionTransitionPosture,
};
use worth_ui_runtime::certification_support::WorthUiIntentResourceCensusCertificationExt;

use super::provider::{AttemptStep, ExecutionScript, ScriptedProvider};
use super::{advance, only_transition};
use crate::intent::admission::phase3::world::AdmissionWorld;

#[test]
fn carried_reference_correlates_admission_attempt_and_terminal_outcome() {
    let (provider, _) = ScriptedProvider::new([ExecutionScript::running([AttemptStep::Completed])]);
    let mut world = AdmissionWorld::launch_with_provider(1, provider);
    let admitted = world.admit_exact(0);
    let admission = admitted.slot_identity();
    let UiIntentExecutionDispatchOutcome::AttemptPrepared(dispatch) = world
        .session
        .dispatch_admitted_intent(admitted, super::super::execution_deadline(20))
    else {
        panic!("a current admitted intent must prepare one attempt")
    };
    let reference = dispatch
        .evidence_reference()
        .expect("production dispatch carries its exact evidence reference");
    assert_trace_census(&world, 1);

    let prepared = found(&world, reference);
    assert_eq!(prepared.reference(), reference);
    assert!(prepared.interaction().source_sequence() > 0);
    assert!(prepared.route().is_some());
    assert!(prepared.payload().is_some());
    assert!(prepared.operability().is_some());
    let retained_admission = prepared.admission().expect("admission stage is retained");
    assert_eq!(retained_admission.slot(), admission.slot());
    assert_eq!(retained_admission.generation(), admission.generation());
    let attempt = prepared.attempt().expect("prepared attempt is retained");
    assert_eq!(attempt.slot(), dispatch.attempt().slot());
    assert_eq!(attempt.generation(), dispatch.attempt().generation());
    assert_eq!(
        attempt.posture(),
        UiIntentCausalTraceAttemptPosture::Prepared
    );
    assert!(prepared.completion().is_none());

    assert_eq!(
        only_transition(advance(&mut world, 1)).posture(),
        UiIntentExecutionTransitionPosture::Started
    );
    assert_eq!(
        found(&world, reference)
            .attempt()
            .expect("started attempt remains correlated")
            .posture(),
        UiIntentCausalTraceAttemptPosture::Started
    );

    assert!(matches!(
        only_transition(advance(&mut world, 2)).posture(),
        UiIntentExecutionTransitionPosture::Completed { .. }
    ));
    let completed = found(&world, reference);
    assert!(completed.is_complete_through_product_outcome());
    assert!(completed
        .completion()
        .expect("terminal outcome is retained")
        .consequence_pending_at_completion());
    let _ = world.session.shutdown();
}

#[test]
fn lookup_distinguishes_foreign_session_and_expired_ring_entries() {
    let (provider, _) = ScriptedProvider::new([ExecutionScript::running([AttemptStep::Completed])]);
    let mut world = AdmissionWorld::launch_with_provider(3, provider);
    let admitted = world.admit_exact(0);
    let UiIntentExecutionDispatchOutcome::AttemptPrepared(dispatch) = world
        .session
        .dispatch_admitted_intent(admitted, super::super::execution_deadline(20))
    else {
        panic!("a current admitted intent must prepare one attempt")
    };
    let reference = dispatch
        .evidence_reference()
        .expect("reference is retained");
    let foreign = UiIntentEvidenceReference::from_diagnostic_parts(
        reference.session_diagnostic_value().wrapping_add(1),
        reference.slot(),
        reference.generation(),
    );
    assert_eq!(
        world.session.lookup_intent_causal_trace(foreign),
        UiIntentEvidenceLookup::ForeignSession
    );
    let wrong_generation = UiIntentEvidenceReference::from_diagnostic_parts(
        reference.session_diagnostic_value(),
        reference.slot(),
        reference.generation().wrapping_add(1),
    );
    assert_eq!(
        world.session.lookup_intent_causal_trace(wrong_generation),
        UiIntentEvidenceLookup::Expired
    );

    for index in 0..worth_ui::facade::inspection::UI_INTENT_INTERACTION_EVIDENCE_ENTRY_CAPACITY {
        let _ = world.evaluate(index % 3);
    }
    assert_eq!(
        world.session.lookup_intent_causal_trace(reference),
        UiIntentEvidenceLookup::Expired
    );
    assert_trace_census(
        &world,
        worth_ui::facade::inspection::UI_INTENT_INTERACTION_EVIDENCE_ENTRY_CAPACITY,
    );
    let _ = world.session.shutdown();
}

fn assert_trace_census(world: &AdmissionWorld, expected: usize) {
    let census = world.session.intent_resource_census_for_certification();
    assert_eq!(census.retained_evidence_references(), expected);
    assert_eq!(
        census.retained_evidence_bytes(),
        expected
            * core::mem::size_of::<worth_ui::facade::inspection::UiIntentCausalTraceEvidence>()
    );
}

fn found(
    world: &AdmissionWorld,
    reference: UiIntentEvidenceReference,
) -> UiIntentCausalTraceEvidence {
    match world.session.lookup_intent_causal_trace(reference) {
        UiIntentEvidenceLookup::Found(trace) => trace,
        posture => panic!("current evidence reference must resolve: {posture:?}"),
    }
}
