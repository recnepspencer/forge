use super::super::support::*;

#[test]
fn preview_effect_policy_bindings_distinguish_delivery_and_write_intent() {
    let mut runtime = stateful_bridge_task_runtime();
    let live = runtime
        .declare_live_view::<Value>("tasks.preview-policy", task_live_request(), task_schema())
        .expect("live should declare");
    let delivery_effect = runtime
        .declare_effect::<Value>(ForgeQueryEffectDeclaration::deliver(
            "ui.preview-policy",
            ForgeQueryEffectTrigger::live_view(&live, ["title"]),
            "ui.preview",
        ))
        .expect("delivery effect should declare");
    let intent_effect = runtime
        .declare_effect::<Value>(ForgeQueryEffectDeclaration::write_intent(
            "intent.preview-policy",
            ForgeQueryEffectTrigger::live_view(&live, ["title"]),
            "preview-intent",
        ))
        .expect("write-intent effect should declare");

    let muted = {
        let mut preview = runtime
            .preview_with_options(
                "muted effect",
                ForgeQueryPreviewOptions::derive_only()
                    .with_effect_policy(ForgeQueryEffectPolicy::Muted),
            )
            .expect("preview session should be admitted");
        preview
            .use_effect(&delivery_effect)
            .expect("muted policy should bind but not activate")
    };
    assert_eq!(
        muted.effect_disposition(),
        Some(ForgeQueryPreviewEffectBindingDisposition::Muted)
    );
    assert!(!muted.effect_delivery_admitted());

    let redirected = {
        let mut preview = runtime
            .preview_with_options(
                "redirected effect",
                ForgeQueryPreviewOptions::derive_only()
                    .with_effect_policy(ForgeQueryEffectPolicy::Redirected),
            )
            .expect("preview session should be admitted");
        preview
            .use_effect(&delivery_effect)
            .expect("redirected policy should admit preview delivery")
    };
    assert_eq!(
        redirected.effect_disposition(),
        Some(ForgeQueryPreviewEffectBindingDisposition::RedirectedDelivery)
    );
    assert!(redirected.effect_delivery_admitted());

    let sandboxed = {
        let mut preview = runtime
            .preview_with_options(
                "sandboxed effect",
                ForgeQueryPreviewOptions::derive_only()
                    .with_effect_policy(ForgeQueryEffectPolicy::SandboxedWriteIntent),
            )
            .expect("preview session should be admitted");
        preview
            .use_effect(&intent_effect)
            .expect("sandboxed policy should admit preview write intent")
    };
    assert_eq!(
        sandboxed.effect_disposition(),
        Some(ForgeQueryPreviewEffectBindingDisposition::SandboxedWriteIntent)
    );
    assert!(sandboxed.pending_write_intent_admitted());

    let denied = {
        let mut preview = runtime
            .preview_with_options(
                "sandboxed delivery denial",
                ForgeQueryPreviewOptions::derive_only()
                    .with_effect_policy(ForgeQueryEffectPolicy::SandboxedWriteIntent),
            )
            .expect("preview session should be admitted");
        preview
            .use_effect(&delivery_effect)
            .expect_err("sandboxed write intent policy should not admit delivery effects")
    };
    assert!(matches!(
        denied,
        ForgeQueryRuntimeError::EffectPolicyDenied(_)
    ));
}
