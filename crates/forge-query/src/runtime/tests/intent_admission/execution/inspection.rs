use super::*;

fn derived_titles_view(
    runtime: &mut ForgeQueryRuntime,
    view_name: &str,
) -> ForgeQueryDerivedViewHandle<ForgeQueryNativeRow> {
    let live = runtime
        .declare_live_view::<ForgeQueryNativeRow>("tasks.table", task_live_request(), task_schema())
        .expect("live view should declare");
    runtime
        .declare_maintained_derived_view::<ForgeQueryNativeRow>(
            ForgeQueryDerivedView::new(view_name, test_aspect_touches(["title"]))
                .depends_on_live(&live)
                .produces(test_aspect_touches(["title.summary"])),
            TitleListMaintainer,
        )
        .expect("derived view should declare")
}

#[test]
fn workspace_materialize_result_delegates_to_derived_materialization_intent_execution() {
    let mut runtime = read_runtime();
    let derived = derived_titles_view(&mut runtime, "computed.intent.materialize");
    runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", test_string_aspect_value("materialize-1")),
                ("title.value", test_string_aspect_value("Materialize me")),
            ],
        ))
        .expect("write should materialize derived output");
    let mut workspace = ForgeQueryWorkspace::new("materialize-delegation", runtime)
        .expect("workspace should build");

    let delegated = workspace
        .materialize_result(&derived)
        .expect("derived materialization should execute");
    let canonical = workspace
        .materialize_intent(&derived)
        .execute()
        .expect("canonical materialization should execute");

    assert_eq!(delegated, canonical);
    assert_eq!(canonical.receipt().view_name(), derived.name());
    assert_eq!(
        canonical
            .receipt()
            .execution_provenance()
            .map(|provenance| provenance.entrypoint()),
        Some(ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteDerivedMaterialization)
    );
}

#[test]
fn runtime_read_derived_result_delegates_to_derived_materialization_intent_execution() {
    let mut runtime = read_runtime();
    let derived = derived_titles_view(&mut runtime, "computed.intent.runtime-materialize");
    runtime
        .write(insert_command(
            "Task",
            [
                (
                    "identity.id",
                    test_string_aspect_value("runtime-materialize-1"),
                ),
                (
                    "title.value",
                    test_string_aspect_value("Runtime materialize me"),
                ),
            ],
        ))
        .expect("write should materialize derived output");

    let delegated = runtime
        .read_derived_result(&derived)
        .expect("derived materialization should execute");
    let canonical = runtime
        .review_runtime_derived_materialization(derived.name().to_string())
        .and_then(|review| {
            runtime.resolve_reviewed_admitted_derived_materialization_handoff(review)
        })
        .map(|handoff| runtime.prepare_derived_materialization_execution_binding(handoff))
        .and_then(|binding| runtime.execute_derived_materialization_execution_binding(binding))
        .expect("canonical derived materialization should execute");

    assert_eq!(delegated, canonical);
    assert_eq!(canonical.receipt().view_name(), derived.name());
    assert_eq!(
        canonical
            .receipt()
            .execution_provenance()
            .map(|provenance| provenance.entrypoint()),
        Some(ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteDerivedMaterialization)
    );
}

#[test]
fn workspace_inspect_derived_view_delegates_to_derived_inspection_intent_execution() {
    let mut runtime = read_runtime();
    let derived = derived_titles_view(&mut runtime, "computed.intent.inspect");
    runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", test_string_aspect_value("inspect-1")),
                ("title.value", test_string_aspect_value("Inspect me")),
            ],
        ))
        .expect("write should materialize derived output");
    let mut workspace =
        ForgeQueryWorkspace::new("inspect-delegation", runtime).expect("workspace should build");

    let delegated = workspace
        .inspect(&derived)
        .expect("legacy derived inspection should succeed");
    let canonical = workspace
        .inspect_derived_intent(&derived)
        .execute()
        .expect("canonical derived inspection should execute");

    match delegated {
        ForgeQueryInspection::DerivedView(evidence) => {
            assert_eq!(&evidence, canonical.evidence());
        }
        other => panic!("expected derived inspection, got {other:?}"),
    }
    assert_eq!(
        canonical
            .receipt()
            .execution_provenance()
            .map(|provenance| provenance.entrypoint()),
        Some(ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteDerivedInspection)
    );
}

#[test]
fn runtime_inspect_derived_view_delegates_to_derived_inspection_intent_execution() {
    let mut runtime = read_runtime();
    let derived = derived_titles_view(&mut runtime, "computed.intent.runtime-inspect");
    runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", test_string_aspect_value("runtime-inspect-1")),
                (
                    "title.value",
                    test_string_aspect_value("Runtime inspect me"),
                ),
            ],
        ))
        .expect("write should materialize derived output");

    let delegated = runtime
        .inspect_derived_view(&derived)
        .expect("legacy runtime derived inspection should succeed");
    let canonical = runtime
        .review_runtime_derived_inspection(derived.name().to_string())
        .and_then(|review| runtime.resolve_reviewed_admitted_derived_inspection_handoff(review))
        .map(|handoff| runtime.prepare_derived_inspection_execution_binding(handoff))
        .and_then(|binding| runtime.execute_derived_inspection_execution_binding(binding))
        .expect("canonical derived inspection should execute");

    assert_eq!(&delegated, canonical.evidence());
    assert_eq!(
        canonical
            .receipt()
            .execution_provenance()
            .map(|provenance| provenance.entrypoint()),
        Some(ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteDerivedInspection)
    );
}

#[test]
fn workspace_inspect_live_view_delegates_to_unified_inspection_intent_execution() {
    let runtime = read_runtime();
    let mut workspace =
        ForgeQueryWorkspace::new("generic-inspect-delegation", runtime).expect("workspace build");
    let live: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
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
                .schema_basis("intent-admission-generic-inspection-delegation")
        })
        .expect("live view should declare");

    let delegated = workspace
        .inspect(&live)
        .expect("legacy generic inspection should succeed");
    let canonical = workspace
        .inspect_intent(&live)
        .execute()
        .expect("canonical generic inspection should execute");

    assert_eq!(delegated, canonical.inspection().clone());
    assert_eq!(
        canonical
            .receipt()
            .execution_provenance()
            .map(|provenance| provenance.entrypoint()),
        Some(ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteUnifiedInspection)
    );
}

#[test]
fn runtime_inspect_live_view_delegates_to_unified_inspection_intent_execution() {
    let mut runtime = read_runtime();
    let live: ForgeQueryLiveView<ForgeQueryNativeRow> = runtime
        .declare_live_view("tasks.table", task_live_request(), task_schema())
        .expect("live view should declare");

    let delegated = runtime
        .inspect(&live)
        .expect("legacy runtime generic inspection should succeed");
    let canonical = runtime
        .inspect_intent(&live)
        .execute()
        .expect("canonical generic inspection should execute");

    assert_eq!(delegated, canonical.inspection().clone());
    assert_eq!(
        canonical
            .receipt()
            .execution_provenance()
            .map(|provenance| provenance.entrypoint()),
        Some(ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteUnifiedInspection)
    );
}

#[test]
fn runtime_specific_intent_inspection_wrappers_delegate_to_unified_inspection_execution() {
    let mut runtime = intent_runtime_with_authority(TestIntentAuthority);
    let receipt = runtime
        .execute_intent(ForgeQueryIntentDeclaration::strategy_commit(
            "delegated-intent-receipt",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            test_intent_input([("entity", "task-1"), ("title", "Delegated intent title")]),
        ))
        .expect("intent should execute");

    let delegated = runtime
        .inspect_intent_receipt(&receipt)
        .expect("direct wrapper should inspect");
    let canonical = runtime
        .inspect_intent(&receipt)
        .execute()
        .expect("canonical inspection should execute");

    match canonical.inspection() {
        ForgeQueryInspection::IntentReceipt(inspection) => assert_eq!(delegated, *inspection),
        other => panic!("expected intent receipt inspection, got {other:?}"),
    }
    assert_eq!(
        canonical
            .receipt()
            .execution_provenance()
            .map(|provenance| provenance.entrypoint()),
        Some(ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteUnifiedInspection)
    );
}

#[test]
fn runtime_specific_intent_denial_wrapper_delegates_to_unified_inspection_execution() {
    let mut runtime = intent_runtime_with_authority(InvariantViolationIntentAuthority);
    let error = runtime
        .execute_intent(ForgeQueryIntentDeclaration::strategy_commit(
            "delegated-intent-denial",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            test_intent_input([("entity", "task-1"), ("dependency", "cycle")]),
        ))
        .expect_err("invariant violation must deny");
    let evidence = match error {
        ForgeQueryRuntimeError::IntentCommitDenied { evidence, .. } => evidence,
        other => panic!("expected intent denial, got {other:?}"),
    };

    let delegated = runtime
        .inspect_intent_denial(&evidence)
        .expect("direct denial wrapper should inspect");
    let canonical = runtime
        .inspect_intent(&evidence)
        .execute()
        .expect("canonical inspection should execute");

    match canonical.inspection() {
        ForgeQueryInspection::IntentDenial(inspection) => assert_eq!(delegated, *inspection),
        other => panic!("expected intent denial inspection, got {other:?}"),
    }
}

#[test]
fn runtime_specific_effect_and_preview_wrappers_delegate_to_unified_inspection_execution() {
    let mut runtime = stateful_bridge_task_runtime();
    let live = runtime
        .declare_live_view::<ForgeQueryNativeRow>(
            "tasks.wrapper-delegation",
            task_live_request(),
            task_schema(),
        )
        .expect("live should declare");
    let effect = runtime
        .declare_effect::<ForgeQueryNativeRow>(ForgeQueryEffectDeclaration::deliver(
            "ui.wrapper-delegation",
            ForgeQueryEffectTrigger::live_view(&live, test_aspect_touches(["title"])),
            "ui.preview",
        ))
        .expect("effect should declare");

    let effect_delegated = runtime
        .inspect_effect(&effect)
        .expect("effect wrapper should inspect");
    let effect_canonical = runtime
        .inspect_intent(&effect)
        .execute()
        .expect("canonical effect inspection should execute");
    match effect_canonical.inspection() {
        ForgeQueryInspection::Effect(inspection) => assert_eq!(effect_delegated, *inspection),
        other => panic!("expected effect inspection, got {other:?}"),
    }

    let outcome = {
        let mut preview = runtime
            .preview(test_session_label("wrapper delegation preview"))
            .expect("preview session should be admitted");
        preview.use_view(&live);
        preview.discard()
    };

    let preview_delegated = runtime
        .inspect_preview_outcome(&outcome)
        .expect("preview outcome wrapper should inspect");
    let preview_canonical = runtime
        .inspect_intent(&outcome)
        .execute()
        .expect("canonical preview inspection should execute");
    match preview_canonical.inspection() {
        ForgeQueryInspection::PreviewOutcome(inspection) => {
            assert_eq!(preview_delegated, *inspection)
        }
        other => panic!("expected preview outcome inspection, got {other:?}"),
    }
}
