use super::super::support::*;
#[test]
fn graph_batch_dispatch_survives_handoff_binding_receipt_and_trace() {
    let mut runtime = runtime_with_relation_obligation("TaskEdge");
    let (commands, breadth, program) = graph_batch_program();

    let review = runtime
        .review_authoritative_runtime_write_batch_with_graph_artifacts(commands, breadth, program)
        .expect("graph batch review should admit");
    let handoff = runtime
        .resolve_reviewed_admitted_authoritative_write_batch_handoff(review)
        .expect("admitted graph batch should resolve to handoff");
    let handoff_dispatch = handoff
        .obligation_dispatch()
        .expect("graph handoff should include selected obligation dispatch");
    let envelope_digest = handoff_dispatch
        .envelope_digest()
        .expect("matching graph obligation should materialize dispatch envelope")
        .to_string();

    assert_eq!(handoff_dispatch.selection().matched_obligation_count(), 1);
    assert_eq!(
        handoff_dispatch
            .selection()
            .counters()
            .registration_full_scan_count(),
        0
    );
    assert_eq!(
        handoff_dispatch
            .envelope()
            .expect("matching graph obligation should have envelope")
            .context()
            .kind(),
        WorthQueryGraphObligationDispatchContextKind::GraphComposition
    );

    let binding = runtime.prepare_authoritative_mutation_batch_execution_binding(handoff);
    assert_eq!(
        binding
            .obligation_dispatch()
            .and_then(|dispatch| dispatch.envelope_digest()),
        Some(envelope_digest.as_str())
    );

    let receipt = runtime
        .execute_authoritative_mutation_batch_execution_binding(binding)
        .expect("binding should execute");
    assert_eq!(
        receipt
            .obligation_dispatch()
            .and_then(|dispatch| dispatch.envelope_digest()),
        Some(envelope_digest.as_str())
    );
    assert_eq!(
        receipt.graph_obligation_envelope_digest(),
        Some(envelope_digest.as_str())
    );
    assert_eq!(
        receipt
            .graph_obligation_evidence()
            .and_then(|evidence| evidence.envelope_digest().map(str::to_string)),
        Some(envelope_digest.clone())
    );
    assert_eq!(
        receipt
            .graph_obligation_evidence()
            .and_then(|evidence| evidence.execution_point()),
        Some(WorthQueryGraphObligationDispatchContextKind::GraphComposition)
    );
    assert_eq!(
        receipt
            .decision_trace_envelope()
            .and_then(|trace| trace.graph_obligation_envelope_digest()),
        Some(envelope_digest.as_str())
    );
    match runtime
        .inspect(&receipt)
        .expect("batch receipt should inspect")
    {
        WorthQueryInspection::BatchWriteReceipt(inspection) => {
            assert_eq!(
                inspection.graph_obligation_envelope_digest(),
                Some(envelope_digest.as_str())
            );
            assert_eq!(
                inspection
                    .graph_obligation_evidence()
                    .map(|evidence| evidence.evidence_digest()),
                receipt
                    .graph_obligation_evidence()
                    .as_ref()
                    .map(|evidence| evidence.evidence_digest())
            );
        }
        other => panic!("expected batch receipt inspection, got {other:?}"),
    }
}

#[test]
fn ordinary_batch_dispatch_uses_authoritative_command_batch_context() {
    let runtime = runtime_with_collection_obligation("Task");
    let review = runtime
        .review_authoritative_runtime_write_batch(vec![task_insert_command("ordinary-context")])
        .expect("ordinary batch review should admit");
    let handoff = runtime
        .resolve_reviewed_admitted_authoritative_write_batch_handoff(review)
        .expect("admitted ordinary batch should resolve to handoff");
    let dispatch = handoff
        .obligation_dispatch()
        .expect("ordinary command batch should still select graph obligations");

    assert_eq!(dispatch.selection().matched_obligation_count(), 1);
    assert_eq!(
        dispatch
            .envelope()
            .expect("matching ordinary obligation should have envelope")
            .context()
            .kind(),
        WorthQueryGraphObligationDispatchContextKind::AuthoritativeCommandBatch
    );
}

#[test]
fn no_match_batch_keeps_selection_counters_without_fake_envelope() {
    let runtime = stateful_bridge_task_runtime();
    let review = runtime
        .review_authoritative_runtime_write_batch(vec![task_insert_command("no-match")])
        .expect("ordinary batch review should admit");
    let handoff = runtime
        .resolve_reviewed_admitted_authoritative_write_batch_handoff(review)
        .expect("admitted ordinary batch should resolve to handoff");
    let dispatch = handoff
        .obligation_dispatch()
        .expect("descriptor-backed batch should keep selection evidence");

    assert_eq!(dispatch.selection().matched_obligation_count(), 0);
    assert!(dispatch.envelope().is_none());
    assert_eq!(dispatch.envelope_digest(), None);
    let evidence = dispatch.attachment_evidence();
    assert_eq!(evidence.selected_obligation_count(), 0);
    assert_eq!(evidence.envelope_digest(), None);
    assert_eq!(evidence.denial_projection(), None);
    assert!(!evidence.selection_digest().is_empty());
    assert!(!evidence.dispatch_digest().is_empty());
    assert!(!evidence.evidence_digest().is_empty());
    assert!(
        dispatch
            .selection()
            .counters()
            .attempted_bucket_lookup_count()
            > 0
    );
    assert_eq!(
        dispatch
            .selection()
            .counters()
            .registration_full_scan_count(),
        0
    );
}

#[test]
fn scalar_write_dispatch_survives_binding_receipt_and_trace() {
    let mut runtime = runtime_with_scalar_collection_obligation("Task");
    let receipt = runtime
        .write(task_insert_command("scalar-dispatch"))
        .expect("scalar write should execute");
    let dispatch = receipt
        .obligation_dispatch()
        .expect("scalar write should carry obligation dispatch");
    let envelope_digest = dispatch
        .envelope_digest()
        .expect("matching scalar obligation should materialize envelope");

    assert_eq!(
        dispatch.envelope().unwrap().context().kind(),
        WorthQueryGraphObligationDispatchContextKind::ScalarMutation
    );
    assert_eq!(dispatch.execution_inputs().len(), 1);
    assert_eq!(
        dispatch.execution_inputs()[0]
            .executor_contract()
            .state_access_policy(),
        WorthQueryGraphObligationStateAccessPolicy::DeclaredBudgetOnly
    );
    assert_eq!(
        dispatch.execution_inputs()[0]
            .executor_contract()
            .support_lane(),
        WorthQueryGraphObligationSupportLane::ScalarMutation
    );
    assert_eq!(
        receipt.graph_obligation_envelope_digest(),
        Some(envelope_digest)
    );
    assert_eq!(
        receipt
            .graph_obligation_evidence()
            .and_then(|evidence| evidence.envelope_digest().map(str::to_string))
            .as_deref(),
        Some(envelope_digest)
    );
    assert_eq!(
        receipt
            .graph_obligation_evidence()
            .and_then(|evidence| evidence.execution_point()),
        Some(WorthQueryGraphObligationDispatchContextKind::ScalarMutation)
    );
    assert_eq!(
        receipt
            .decision_trace_envelope()
            .and_then(|trace| trace.graph_obligation_envelope_digest()),
        Some(envelope_digest)
    );
}

#[test]
fn unsupported_obligation_support_posture_denies_before_batch_execution() {
    let runtime = runtime_with_blocking_collection_obligation("Task");
    let review = runtime
        .review_authoritative_runtime_write_batch(vec![task_insert_command("blocked")])
        .expect("ordinary batch review should admit before dispatch selection");
    let error = runtime
        .resolve_reviewed_admitted_authoritative_write_batch_handoff(review)
        .expect_err("blocking selected obligation should deny handoff resolution");

    match error {
        WorthQueryRuntimeError::GraphObligationDenied(denial) => {
            assert_eq!(denial.blocking_count(), 1);
        }
        other => panic!("unexpected blocking obligation error: {other:?}"),
    }
}

#[test]
fn malformed_graph_touch_descriptor_denies_instead_of_bypassing_dispatch() {
    let runtime = runtime_with_collection_obligation("Task");
    let (mut commands, breadth, program) = graph_batch_program();
    commands.pop();
    let review = runtime
        .review_authoritative_runtime_write_batch_with_graph_artifacts(commands, breadth, program)
        .expect("runtime review records graph artifacts before descriptor derivation");
    let error = runtime
        .resolve_reviewed_admitted_authoritative_write_batch_handoff(review)
        .expect_err("malformed touch descriptor must deny handoff resolution");

    match error {
        WorthQueryRuntimeError::GraphObligationTouchDescriptorDenied(denial) => {
            assert_eq!(
                denial.kind(),
                WorthQueryGraphTouchDescriptorDenialKind::ProgramComponentCountMismatch
            );
        }
        other => panic!("unexpected malformed descriptor error: {other:?}"),
    }
}

fn runtime_with_relation_obligation(relation_kind: &str) -> WorthQueryRuntime {
    complete_backend_from_parts_builder()
        .graph_obligation(
            WorthQueryGraphObligationRegistration::schema_contract_validator(
                WorthQueryGraphObligationRuleIdentity::new(
                    "test.graph-obligation-dispatch",
                    relation_kind,
                    "v1",
                )
                .unwrap(),
                WorthQueryGraphTouchSelector::relation_kind(relation_kind).unwrap(),
                WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
            ),
        )
        .build_backend_from_parts()
        .build()
        .expect("runtime should build with relation graph obligation")
}

fn runtime_with_collection_obligation(collection: &str) -> WorthQueryRuntime {
    runtime_with_collection_registration(collection_registration(
        collection,
        WorthQueryGraphObligationSupportPosture::supported(
            WorthQueryGraphObligationSupportLane::AuthoritativeCommandBatch,
        ),
    ))
}

fn runtime_with_blocking_collection_obligation(collection: &str) -> WorthQueryRuntime {
    runtime_with_collection_registration(collection_registration(
        collection,
        WorthQueryGraphObligationSupportPosture::unsupported(
            WorthQueryGraphObligationSupportLane::AuthoritativeCommandBatch,
        ),
    ))
}

fn runtime_with_scalar_collection_obligation(collection: &str) -> WorthQueryRuntime {
    runtime_with_collection_registration(collection_registration(
        collection,
        WorthQueryGraphObligationSupportPosture::supported(
            WorthQueryGraphObligationSupportLane::ScalarMutation,
        ),
    ))
}

fn runtime_with_collection_registration(
    registration: WorthQueryGraphObligationRegistration,
) -> WorthQueryRuntime {
    complete_backend_from_parts_builder()
        .graph_obligation(registration)
        .build_backend_from_parts()
        .build()
        .expect("runtime should build with collection graph obligation")
}

fn collection_registration(
    collection: &str,
    support_posture: WorthQueryGraphObligationSupportPosture,
) -> WorthQueryGraphObligationRegistration {
    WorthQueryGraphObligationRegistration::schema_contract_validator(
        WorthQueryGraphObligationRuleIdentity::new(
            "test.graph-obligation-dispatch",
            collection,
            "v1",
        )
        .unwrap(),
        WorthQueryGraphTouchSelector::collection(collection).unwrap(),
        WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    )
    .with_support_posture(support_posture)
}

fn graph_batch_program() -> (
    Vec<WorthQueryWriteCommand>,
    WorthQueryGraphCompositionBreadth,
    WorthQueryGraphCompositionProgram,
) {
    let mut graph = WorthQueryGraphCompositionBuilder::new();
    let task = graph
        .insert_entity("task", "Task", |entity| {
            entity
                .set_aspect(
                    test_aspect_touch("identity.id"),
                    test_authored_string_aspect_value("task-dispatch"),
                )
                .set_aspect(
                    test_aspect_touch("title.value"),
                    test_authored_string_aspect_value("Draft"),
                )
        })
        .unwrap();
    let edge = graph
        .insert_symbolic_relation("edge", "TaskEdge", |relation| {
            relation
                .set_aspect(
                    test_aspect_touch("edge.kind"),
                    test_authored_string_aspect_value("depends_on"),
                )
                .symbolic_entity_identity(test_aspect_touch("edge.source_identity"), &task)
                .existing_entity_identity(
                    test_aspect_touch("edge.target_identity"),
                    test_entity_identity("task-existing"),
                )
        })
        .unwrap();
    graph
        .delete_relation(&edge, |delete| {
            delete.touches(test_aspect_touches([
                "edge.kind",
                "edge.source_identity",
                "edge.target_identity",
            ]))
        })
        .unwrap();
    graph.finish().unwrap()
}

fn task_insert_command(id: &str) -> WorthQueryWriteCommand {
    WorthQueryWriteCommand::InsertAspects {
        collection: crate::runtime::WorthQueryMutationTargetCollectionIdentity::new(
            "write-command-declared",
            "Task",
        ),
        aspects: vec![
            WorthQueryAdmittedAspectValue::new(
                test_aspect_touch("identity.id"),
                test_string_aspect_value(id),
            )
            .unwrap(),
            WorthQueryAdmittedAspectValue::new(
                test_aspect_touch("title.value"),
                test_string_aspect_value("Ordinary task"),
            )
            .unwrap(),
        ],
        symbolic_aspect_references: Vec::new(),
        metadata: WorthQueryMutationMetadata::new(),
        naming_intent: None,
        continuity_intent: None,
        symbolic_target_reference: None,
    }
}
