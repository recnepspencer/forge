use super::super::support::*;

#[test]
fn derive_only_preview_binds_handles_and_mutes_effects_without_residue() {
    let mut runtime = stateful_bridge_task_runtime();
    let live = runtime
        .declare_live_view::<Value>("tasks.preview-bind", task_live_request(), task_schema())
        .expect("live should declare");
    let computed = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.preview-bind", ["title".to_string()])
                .depends_on_live(&live)
                .produces(["title.summary".to_string()]),
            TitleListMaintainer,
        )
        .expect("computed should declare");
    let delivery_effect = runtime
        .declare_effect::<Value>(ForgeQueryEffectDeclaration::deliver(
            "ui.preview-bind",
            ForgeQueryEffectTrigger::live_view(&live, ["title"]),
            "ui.preview",
        ))
        .expect("delivery effect should declare");
    let intent_effect = runtime
        .declare_effect::<Value>(ForgeQueryEffectDeclaration::write_intent(
            "intent.preview-bind",
            ForgeQueryEffectTrigger::live_view(&live, ["title"]),
            "preview-intent",
        ))
        .expect("write-intent effect should declare");

    let (live_binding, computed_binding, delivery_binding, intent_binding, outcome) = {
        let mut preview = runtime
            .preview(test_session_label("derive-only bindings"))
            .expect("preview session should be admitted");
        let live_binding = preview.use_view(&live);
        let computed_binding = preview.use_computed(&computed);
        let delivery_binding = preview
            .use_effect(&delivery_effect)
            .expect("derive-only preview should bind delivery effect muted");
        let intent_binding = preview
            .use_effect(&intent_effect)
            .expect("derive-only preview should bind write-intent effect muted");

        assert_eq!(
            live_binding.family(),
            ForgeQueryPreviewHandleBindingFamily::LiveView
        );
        assert_eq!(
            live_binding.preview_lane(),
            ForgeQueryAuthorityLane::PreviewTruth
        );
        assert_eq!(
            computed_binding.source_lane(),
            ForgeQueryAuthorityLane::DerivedRuntimeState
        );
        assert_eq!(
            delivery_binding.effect_disposition(),
            Some(ForgeQueryPreviewEffectBindingDisposition::MutedByDeriveOnly)
        );
        assert!(!delivery_binding.effect_delivery_admitted());
        assert_eq!(
            intent_binding.effect_disposition(),
            Some(ForgeQueryPreviewEffectBindingDisposition::MutedByDeriveOnly)
        );
        assert!(!intent_binding.pending_write_intent_admitted());

        (
            live_binding,
            computed_binding,
            delivery_binding,
            intent_binding,
            preview.discard(),
        )
    };

    assert_eq!(outcome.preview_binding_count(), 4);
    assert_eq!(outcome.effect_binding_count(), 2);
    assert_eq!(outcome.effect_delivery_residue_count(), 0);
    assert_eq!(outcome.pending_write_intent_residue_count(), 0);
    assert_eq!(outcome.authoritative_residue_count(), 0);
    assert!(runtime
        .drain_effect_deliveries(&delivery_effect)
        .expect("authoritative delivery queue should still exist")
        .is_empty());
    assert!(runtime
        .drain_effect_deliveries(&intent_effect)
        .expect("authoritative intent queue should still exist")
        .is_empty());

    let live_inspection = runtime
        .inspect_preview_binding(&live_binding)
        .expect("preview binding inspection should succeed");
    assert_eq!(
        live_inspection.family(),
        ForgeQueryPreviewHandleBindingFamily::LiveView
    );
    assert_eq!(
        live_inspection.source_lane(),
        ForgeQueryAuthorityLane::AuthoritativeTruth
    );
    assert_eq!(
        live_inspection.preview_lane(),
        ForgeQueryAuthorityLane::PreviewTruth
    );
    assert!(!live_inspection.basis_digest().is_empty());
    assert!(!live_inspection.admission_digest().is_empty());

    let computed_inspection = runtime
        .inspect_preview_binding(&computed_binding)
        .expect("computed preview binding inspection should succeed");
    assert_eq!(
        computed_inspection.family(),
        ForgeQueryPreviewHandleBindingFamily::ComputedView
    );
    assert_eq!(
        computed_inspection.source_lane(),
        ForgeQueryAuthorityLane::DerivedRuntimeState
    );

    let delivery_inspection = runtime
        .inspect_preview_binding(&delivery_binding)
        .expect("delivery preview binding inspection should succeed");
    assert_eq!(
        delivery_inspection.effect_disposition(),
        Some("muted-by-derive-only")
    );
    assert!(!delivery_inspection.effect_delivery_admitted());

    let intent_inspection = runtime
        .inspect_preview_binding(&intent_binding)
        .expect("intent preview binding inspection should succeed");
    assert_eq!(
        intent_inspection.effect_disposition(),
        Some("muted-by-derive-only")
    );
    assert!(!intent_inspection.pending_write_intent_admitted());

    let outcome_inspection = runtime
        .inspect_preview_outcome(&outcome)
        .expect("preview outcome inspection should succeed");
    assert_eq!(
        outcome_inspection.closeout_kind(),
        ForgeQueryPreviewCloseoutKind::Discarded
    );
    assert!(outcome_inspection.discarded());
    assert!(!outcome_inspection.promoted());
    assert_eq!(
        outcome_inspection.source_lane(),
        ForgeQueryAuthorityLane::PreviewTruth
    );
    assert_eq!(
        outcome_inspection.target_lane(),
        ForgeQueryAuthorityLane::PreviewTruth
    );
    assert_eq!(outcome_inspection.effect_binding_count(), 2);
    assert_eq!(outcome_inspection.subscription_residue_count(), 0);
    assert_eq!(outcome_inspection.derived_runtime_residue_count(), 0);
    assert_eq!(outcome_inspection.preview_write_staging_count(), 0);
    assert_eq!(outcome_inspection.promoted_write_count(), 0);
    assert_eq!(outcome_inspection.authoritative_residue_count(), 0);
    assert!(!outcome_inspection.closeout_digest().is_empty());
    assert!(!outcome_inspection.residue_digest().is_empty());
    assert!(!outcome_inspection.inspection_digest().is_empty());
}

#[test]
fn preview_write_routes_bound_live_computed_and_redirected_effect_without_authoritative_residue() {
    let mut runtime = stateful_bridge_task_runtime();
    let live = runtime
        .declare_live_view::<Value>(
            "tasks.preview-execution",
            task_live_request(),
            task_schema(),
        )
        .expect("live should declare");
    let computed = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.preview-execution", ["title".to_string()])
                .depends_on_live(&live)
                .produces(["title.summary".to_string()]),
            TitleListMaintainer,
        )
        .expect("computed should declare");
    let delivery_effect = runtime
        .declare_effect::<Value>(ForgeQueryEffectDeclaration::deliver(
            "ui.preview-execution",
            ForgeQueryEffectTrigger::computed_view(&computed, ["title.summary"]),
            "ui.preview",
        ))
        .expect("delivery effect should declare");

    let (execution_evidence, outcome) = {
        let mut preview = runtime
            .preview_with_options(
                test_session_label("preview execution"),
                ForgeQueryPreviewOptions::redirected_delivery(),
            )
            .expect("preview session should be admitted");
        preview.use_view(&live);
        preview.use_computed(&computed);
        preview
            .use_effect(&delivery_effect)
            .expect("redirected preview should admit delivery effect");
        preview
            .write(insert_command(
                "Task",
                [
                    ("identity.id", json!("preview-execution-task")),
                    ("title.value", json!("Preview execution task")),
                ],
            ))
            .expect("preview write should stage and route");
        (
            preview.preview_execution_evidence().to_vec(),
            preview.discard(),
        )
    };

    assert!(execution_evidence.iter().any(|evidence| {
        evidence.kind() == ForgeQueryPreviewExecutionKind::LivePatch
            && evidence.handle_name() == "tasks.preview-execution"
            && evidence.preview_lane() == ForgeQueryAuthorityLane::PreviewTruth
            && !evidence.execution_digest().is_empty()
    }));
    assert!(execution_evidence.iter().any(|evidence| {
        evidence.kind() == ForgeQueryPreviewExecutionKind::ComputedPatch
            && evidence.handle_name() == "computed.preview-execution"
            && evidence.aspect_paths() == ["title.summary"]
    }));
    assert!(execution_evidence.iter().any(|evidence| {
        evidence.kind() == ForgeQueryPreviewExecutionKind::EffectDelivery
            && evidence.handle_name() == "ui.preview-execution"
    }));

    let closeout = outcome.closeout_evidence();
    assert_eq!(
        closeout.class_count(ForgeQueryPreviewResidueClass::SubscriptionState),
        1
    );
    assert_eq!(
        closeout.class_count(ForgeQueryPreviewResidueClass::DerivedRuntimeState),
        1
    );
    assert_eq!(closeout.effect_delivery_residue_count(), 1);
    assert_eq!(closeout.pending_write_intent_residue_count(), 0);
    assert_eq!(closeout.authoritative_residue_count(), 0);
    assert!(runtime
        .drain_patches(&live)
        .query_delivery_batches
        .is_empty());
    assert!(runtime.read_derived(&computed).is_empty());
    assert!(runtime
        .drain_effect_deliveries(&delivery_effect)
        .expect("authoritative effect queue should exist")
        .is_empty());
}

#[test]
fn preview_sandboxed_write_intent_execution_stays_separate_from_delivery_residue() {
    let mut runtime = stateful_bridge_task_runtime();
    let live = runtime
        .declare_live_view::<Value>(
            "tasks.preview-intent-exec",
            task_live_request(),
            task_schema(),
        )
        .expect("live should declare");
    let intent_effect = runtime
        .declare_effect::<Value>(ForgeQueryEffectDeclaration::write_intent(
            "intent.preview-execution",
            ForgeQueryEffectTrigger::live_view(&live, ["title"]),
            "preview-intent",
        ))
        .expect("write-intent effect should declare");

    let (execution_evidence, outcome) = {
        let mut preview = runtime
            .preview_with_options(
                test_session_label("preview intent execution"),
                ForgeQueryPreviewOptions::sandboxed_write_intent(),
            )
            .expect("preview session should be admitted");
        preview.use_view(&live);
        preview
            .use_effect(&intent_effect)
            .expect("sandboxed preview should admit write-intent effect");
        preview
            .write(insert_command(
                "Task",
                [
                    ("identity.id", json!("preview-intent-task")),
                    ("title.value", json!("Preview intent task")),
                ],
            ))
            .expect("preview write should route pending intent");
        (
            preview.preview_execution_evidence().to_vec(),
            preview.discard(),
        )
    };

    assert!(execution_evidence.iter().any(|evidence| {
        evidence.kind() == ForgeQueryPreviewExecutionKind::PendingWriteIntent
            && evidence.handle_name() == "intent.preview-execution"
    }));
    let closeout = outcome.closeout_evidence();
    assert_eq!(closeout.effect_delivery_residue_count(), 0);
    assert_eq!(closeout.pending_write_intent_residue_count(), 1);
    assert_eq!(closeout.authoritative_residue_count(), 0);
    assert!(runtime
        .drain_effect_deliveries(&intent_effect)
        .expect("authoritative pending intent queue should exist")
        .is_empty());
}
