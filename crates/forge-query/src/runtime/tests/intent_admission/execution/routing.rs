use super::*;

#[test]
fn workspace_probe_existing_delegates_to_routing_intent_execution() {
    let mut delegated_workspace = stateful_bridge_task_runtime()
        .workspace("intent-admission-probe-delegated")
        .expect("workspace should open");
    let delegated_binding = seeded_probe_binding(&mut delegated_workspace);
    let delegated = delegated_workspace
        .probe_existing(
            delegated_binding,
            test_aspect_touches(["identity.id", "title.value"]),
        )
        .expect("legacy workspace probe should execute");

    let mut canonical_workspace = stateful_bridge_task_runtime()
        .workspace("intent-admission-probe-canonical")
        .expect("workspace should open");
    let canonical_binding = seeded_probe_binding(&mut canonical_workspace);
    let request = ForgeQueryExistingTruthProbeRequest::new(
        canonical_binding,
        test_aspect_touches(["identity.id", "title.value"]),
    )
    .expect("probe request should build");
    let canonical = canonical_workspace
        .probe_existing_intent(request)
        .execute()
        .expect("canonical probe intent should execute");

    assert_eq!(delegated, *canonical.probe());
    assert_eq!(
        canonical
            .receipt()
            .execution_provenance()
            .map(|provenance| provenance.entrypoint()),
        Some(ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteExistingTruthProbeRouting)
    );
    assert_eq!(
        canonical
            .receipt()
            .decision_trace_envelope()
            .map(trace_stages),
        Some(vec![
            ForgeQueryIntentDecisionTraceStage::RawIntent,
            ForgeQueryIntentDecisionTraceStage::Eligibility,
            ForgeQueryIntentDecisionTraceStage::AdmittedDecision,
            ForgeQueryIntentDecisionTraceStage::ExecutionHandoff,
            ForgeQueryIntentDecisionTraceStage::ExecutionOutcome,
        ])
    );
}

#[test]
fn runtime_probe_existing_delegates_to_canonical_routing_intent_execution() {
    let mut delegated_workspace = stateful_bridge_task_runtime()
        .workspace("intent-admission-probe-runtime-delegated")
        .expect("workspace should open");
    let delegated_binding = seeded_probe_binding(&mut delegated_workspace);
    let delegated_request = ForgeQueryExistingTruthProbeRequest::new(
        delegated_binding,
        test_aspect_touches(["identity.id", "title.value"]),
    )
    .expect("probe request should build");
    let delegated_runtime = delegated_workspace.into_runtime();
    let delegated = delegated_runtime
        .probe_existing(delegated_request)
        .expect("legacy runtime probe should execute");

    let mut canonical_workspace = stateful_bridge_task_runtime()
        .workspace("intent-admission-probe-runtime-canonical")
        .expect("workspace should open");
    let canonical_binding = seeded_probe_binding(&mut canonical_workspace);
    let canonical_request = ForgeQueryExistingTruthProbeRequest::new(
        canonical_binding,
        test_aspect_touches(["identity.id", "title.value"]),
    )
    .expect("probe request should build");
    let canonical_runtime = canonical_workspace.into_runtime();
    let canonical = canonical_runtime
        .probe_existing_intent(canonical_request)
        .execute()
        .expect("canonical probe intent should execute");

    assert_eq!(delegated, *canonical.probe());
    assert_eq!(
        canonical.receipt().probe_digest(),
        canonical.probe().probe_digest()
    );
}

fn seeded_probe_binding(
    workspace: &mut ForgeQueryWorkspace,
) -> ForgeQueryExistingTruthTargetBinding {
    let seed = workspace
        .insert("Task", |task| {
            task.aspect(
                test_aspect_touch("identity.id"),
                test_string_aspect_value("task-1"),
            )
            .aspect(
                test_aspect_touch("title.value"),
                test_string_aspect_value("Seed title"),
            )
        })
        .expect("seed insert should execute");
    workspace
        .bind_existing_entity(
            ForgeQueryExistingEntityTarget::new(
                crate::runtime::ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::ForgeQueryExistingTruthBindingAuthorityLabel::new("authority:task-1").expect("existing-truth authority label")).expect("existing-truth authority identity"),
                seed.deltas()[0].entity_identity.clone(),
            )
            .expect("existing entity target should build")
            .in_target_collection("Task")
            .expect("existing entity target collection should build"),
        )
        .expect("binding should build")
}
