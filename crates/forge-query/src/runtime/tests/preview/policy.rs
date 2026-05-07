use super::super::support::*;

#[test]
fn runtime_surfaces_authority_lanes_on_public_handles_and_receipts() {
    let mut runtime = stateful_bridge_task_runtime();
    let live = runtime
        .declare_live_view::<Value>("tasks.authority", task_live_request(), task_schema())
        .expect("live view should declare");
    let derived = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("task_titles.authority", ["title".to_string()]),
            TitleListMaintainer,
        )
        .expect("derived view should declare");

    let receipt = runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", json!("")),
                ("title.value", json!("Authority lane task")),
            ],
        ))
        .expect("insert should write");
    let patches = runtime.drain_derived_patches(derived.name());
    let inspector = runtime.inspect_receipt(&receipt);

    assert_eq!(
        live.authority_lane(),
        ForgeQueryAuthorityLane::AuthoritativeTruth
    );
    assert_eq!(
        derived.authority_lane(),
        ForgeQueryAuthorityLane::DerivedRuntimeState
    );
    assert_eq!(
        receipt.authority_lane(),
        ForgeQueryAuthorityLane::AuthoritativeTruth
    );
    assert_eq!(
        inspector.authority_lane(),
        ForgeQueryAuthorityLane::AuthoritativeTruth
    );
    assert_eq!(
        patches.derived_patches[0].authority_lane(),
        ForgeQueryAuthorityLane::DerivedRuntimeState
    );
}

#[test]
fn preview_defaults_to_derive_only_effect_policy_but_keeps_explicit_writes_preview_local() {
    let mut runtime = stateful_bridge_task_runtime();
    let mut preview = runtime
        .preview("default policy")
        .expect("preview session should be admitted");

    assert_eq!(preview.effect_policy(), ForgeQueryEffectPolicy::DeriveOnly);
    assert!(preview
        .admit_effect_action(
            ForgeQueryEffectAction::Derive,
            ForgeQueryAuthorityLane::DerivedRuntimeState
        )
        .is_ok());

    let delivery_denial = preview
        .admit_effect_action(
            ForgeQueryEffectAction::Deliver,
            ForgeQueryAuthorityLane::EffectDeliveryState,
        )
        .expect_err("derive-only preview should deny effect delivery");
    assert!(matches!(
        delivery_denial,
        ForgeQueryRuntimeError::EffectPolicyDenied(_)
    ));

    let write_denial = preview
        .admit_effect_action(
            ForgeQueryEffectAction::WriteIntent,
            ForgeQueryAuthorityLane::AuthoritativeTruth,
        )
        .expect_err("derive-only preview should deny authoritative write intent");
    assert!(matches!(
        write_denial,
        ForgeQueryRuntimeError::EffectPolicyDenied(_)
    ));

    let preview_receipt = preview
        .write(insert_command(
            "Task",
            [
                ("identity.id", json!("")),
                ("title.value", json!("Preview-local task")),
            ],
        ))
        .expect("explicit preview write should stage");
    assert_eq!(
        preview_receipt.authority_lane(),
        ForgeQueryAuthorityLane::PreviewTruth
    );

    let outcome = preview.discard();
    assert_eq!(outcome.effect_policy(), ForgeQueryEffectPolicy::DeriveOnly);
    assert_eq!(outcome.source_lane(), ForgeQueryAuthorityLane::PreviewTruth);
    assert_eq!(outcome.target_lane(), ForgeQueryAuthorityLane::PreviewTruth);
}

#[test]
fn sandboxed_preview_policy_admits_only_sandboxed_write_intents() {
    let mut runtime = stateful_bridge_task_runtime();
    let preview = runtime
        .preview_with_options(
            "sandboxed writes",
            ForgeQueryPreviewOptions::sandboxed_write_intent(),
        )
        .expect("preview session should be admitted");

    let admission = preview
        .admit_effect_action(
            ForgeQueryEffectAction::WriteIntent,
            ForgeQueryAuthorityLane::PreviewTruth,
        )
        .expect("sandboxed write intent should be admitted to preview truth");
    assert_eq!(
        admission.policy(),
        ForgeQueryEffectPolicy::SandboxedWriteIntent
    );
    assert_eq!(admission.action(), ForgeQueryEffectAction::WriteIntent);
    assert_eq!(
        admission.target_lane(),
        ForgeQueryAuthorityLane::PreviewTruth
    );

    let denial = preview
        .admit_effect_action(
            ForgeQueryEffectAction::WriteIntent,
            ForgeQueryAuthorityLane::AuthoritativeTruth,
        )
        .expect_err("sandboxed write intent must not target authoritative truth");
    assert!(matches!(
        denial,
        ForgeQueryRuntimeError::EffectPolicyDenied(_)
    ));
}

#[test]
fn derive_only_preview_denies_operation_write_effects() {
    let mut runtime = stateful_bridge_task_runtime();
    runtime
        .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
        .expect("live view should declare before preview-safe operation runs");
    let program = preview_safe_program();
    let installed = runtime
        .install_program(program)
        .expect("program should install");
    let operation = installed
        .operation("create_task")
        .expect("operation ref should build");

    let mut preview = runtime
        .preview("derive-only operation")
        .expect("preview session should be admitted");
    let error = preview
        .run_operation(
            operation,
            vec![ForgeQueryOperationInput::new(
                "title",
                Value::String("Should not stage".to_string()),
            )],
        )
        .expect_err("derive-only preview should deny write-effect operations");

    assert!(matches!(
        error,
        ForgeQueryRuntimeError::EffectPolicyDenied(_)
    ));
    assert_eq!(preview.compare_to_authoritative().write_count(), 0);
}
