use super::*;

#[test]
fn read_family_intent_common_path_helper_executes_through_canonical_handoff() {
    let runtime = read_runtime();
    let mut workspace = ForgeQueryWorkspace::new("intent-admission-read-dx", runtime)
        .expect("workspace should build");
    let family = identity_read_family(&mut workspace, "tasks");

    let result = workspace
        .read_family_intent(&family)
        .execute()
        .expect("read common-path helper should execute");

    assert_eq!(
        result.receipt().decision_trace_envelope().map(trace_stages),
        Some(vec![
            ForgeQueryIntentDecisionTraceStage::RawIntent,
            ForgeQueryIntentDecisionTraceStage::Eligibility,
            ForgeQueryIntentDecisionTraceStage::AdmittedDecision,
            ForgeQueryIntentDecisionTraceStage::ExecutionHandoff,
            ForgeQueryIntentDecisionTraceStage::ExecutionOutcome,
        ])
    );
    assert_eq!(
        result
            .receipt()
            .execution_provenance()
            .map(|provenance| provenance.family()),
        Some(ForgeQueryIntentAdmissionFamily::ReadExecutionIntent)
    );
}

#[test]
fn read_family_intent_advanced_path_helper_exposes_request_eligibility_decision_and_handoff() {
    let runtime = read_runtime();
    let mut workspace = ForgeQueryWorkspace::new("intent-admission-read-advanced", runtime)
        .expect("workspace should build");
    let family = identity_read_family(&mut workspace, "tasks");

    let review = workspace
        .read_family_intent(&family)
        .review()
        .expect("advanced read path should review");
    let handoff = review
        .admitted_handoff()
        .expect("admitted read review should expose a handoff");

    assert_eq!(
        review.request().entrypoint(),
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteReadFamily
    );
    match review.decision() {
        ForgeQueryIntentAdmissionDecision::Admitted(plan) => {
            assert_eq!(plan.decision_digest(), handoff.decision_digest());
        }
        other => panic!("expected admitted read review, got {other:?}"),
    }
    let consumer = review.consumer_inspection();
    assert_eq!(
        consumer.outcome_class(),
        ForgeQueryIntentConsumerOutcomeClass::Admitted
    );
    assert_eq!(consumer.admission_family(), Some(review.request().family()));
    assert_eq!(
        consumer.covered_entrypoint(),
        Some(review.request().entrypoint())
    );
}

#[test]
fn live_read_intent_common_path_helper_executes_through_canonical_handoff() {
    let runtime = read_runtime();
    let mut workspace = ForgeQueryWorkspace::new("intent-admission-live-read-dx", runtime)
        .expect("workspace should build");
    let live_view: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("tasks.table", |q| {
            q.from("Task")
                .select([
                    crate::authoring::AspectFieldKey::from_authoring_parts("identity", "id")
                        .unwrap(),
                    crate::authoring::AspectFieldKey::from_authoring_parts("title", "value")
                        .unwrap(),
                ])
                .order_by(
                    crate::authoring::AspectFieldKey::from_authoring_parts("title", "value")
                        .unwrap(),
                )
                .schema_basis("intent-admission-live-read")
        })
        .expect("live view should declare");

    let result = workspace
        .read_live_intent(&live_view)
        .execute()
        .expect("live read common-path helper should execute");

    assert_eq!(
        result.receipt().decision_trace_envelope().map(trace_stages),
        Some(vec![
            ForgeQueryIntentDecisionTraceStage::RawIntent,
            ForgeQueryIntentDecisionTraceStage::Eligibility,
            ForgeQueryIntentDecisionTraceStage::AdmittedDecision,
            ForgeQueryIntentDecisionTraceStage::ExecutionHandoff,
            ForgeQueryIntentDecisionTraceStage::ExecutionOutcome,
        ])
    );
    assert_eq!(
        result
            .receipt()
            .execution_provenance()
            .map(|provenance| provenance.entrypoint()),
        Some(ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteLiveRead)
    );
}

#[test]
fn live_read_intent_advanced_path_helper_exposes_request_eligibility_decision_and_handoff() {
    let runtime = read_runtime();
    let mut workspace = ForgeQueryWorkspace::new("intent-admission-live-read-advanced", runtime)
        .expect("workspace should build");
    let live_view: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("tasks.table", |q| {
            q.from("Task")
                .select([
                    crate::authoring::AspectFieldKey::from_authoring_parts("identity", "id")
                        .unwrap(),
                    crate::authoring::AspectFieldKey::from_authoring_parts("title", "value")
                        .unwrap(),
                ])
                .order_by(
                    crate::authoring::AspectFieldKey::from_authoring_parts("title", "value")
                        .unwrap(),
                )
                .schema_basis("intent-admission-live-read")
        })
        .expect("live view should declare");

    let review = workspace
        .read_live_intent(&live_view)
        .review()
        .expect("advanced live read path should review");
    let handoff = review
        .admitted_handoff()
        .expect("admitted live read review should expose a handoff");

    assert_eq!(
        review.request().entrypoint(),
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteLiveRead
    );
    match review.decision() {
        ForgeQueryIntentAdmissionDecision::Admitted(plan) => {
            assert_eq!(plan.decision_digest(), handoff.decision_digest());
        }
        other => panic!("expected admitted live read review, got {other:?}"),
    }
    let consumer = review.consumer_inspection();
    assert_eq!(
        consumer.outcome_class(),
        ForgeQueryIntentConsumerOutcomeClass::Admitted
    );
    assert_eq!(consumer.admission_family(), Some(review.request().family()));
    assert_eq!(
        consumer.covered_entrypoint(),
        Some(review.request().entrypoint())
    );
}
