use super::*;

#[test]
fn existing_truth_probe_intent_common_path_executes_through_canonical_handoff() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("intent-admission-probe-dx")
        .expect("workspace should open");
    let binding = seeded_probe_binding(&mut workspace);
    let request = ForgeQueryExistingTruthProbeRequest::new(binding, ["identity.id", "title.value"])
        .expect("probe request should build");
    let runtime = workspace.into_runtime();

    let result = runtime
        .probe_existing_intent(request)
        .execute()
        .expect("probe common-path helper should execute");

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
        Some(ForgeQueryIntentAdmissionFamily::LowerRuntimeCapabilityRoutingIntent)
    );
}

#[test]
fn existing_truth_probe_intent_advanced_path_exposes_request_eligibility_decision_and_handoff() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("intent-admission-probe-advanced")
        .expect("workspace should open");
    let binding = seeded_probe_binding(&mut workspace);
    let request = ForgeQueryExistingTruthProbeRequest::new(binding, ["identity.id", "title.value"])
        .expect("probe request should build");
    let runtime = workspace.into_runtime();

    let review = runtime
        .probe_existing_intent(request)
        .review()
        .expect("advanced probe path should review");
    let handoff = review
        .admitted_handoff()
        .expect("admitted probe review should expose a handoff");

    assert_eq!(
        review.request().entrypoint(),
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteExistingTruthProbeRouting
    );
    match review.decision() {
        ForgeQueryIntentAdmissionDecision::Admitted(plan) => {
            assert_eq!(plan.decision_digest(), handoff.decision_digest());
        }
        other => panic!("expected admitted probe review, got {other:?}"),
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
fn workspace_existing_truth_probe_intent_common_path_executes_through_canonical_handoff() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("intent-admission-workspace-probe-dx")
        .expect("workspace should open");
    let binding = seeded_probe_binding(&mut workspace);
    let request = ForgeQueryExistingTruthProbeRequest::new(binding, ["identity.id", "title.value"])
        .expect("probe request should build");

    let result = workspace
        .probe_existing_intent(request)
        .execute()
        .expect("workspace probe common-path helper should execute");

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
        Some(ForgeQueryIntentAdmissionFamily::LowerRuntimeCapabilityRoutingIntent)
    );
}

fn seeded_probe_binding(
    workspace: &mut ForgeQueryWorkspace,
) -> ForgeQueryExistingTruthTargetBinding {
    let seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-1")
                .aspect("title.value", "Seed title")
        })
        .expect("seed insert should execute");
    workspace
        .bind_existing_entity(
            ForgeQueryExistingEntityTarget::new(
                "authority:task-1",
                seed.deltas()[0].entity_identity.clone(),
            )
            .expect("existing entity target should build")
            .in_target_collection("Task")
            .expect("existing entity target collection should build"),
        )
        .expect("binding should build")
}
