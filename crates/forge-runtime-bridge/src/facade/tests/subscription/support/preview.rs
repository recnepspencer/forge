use super::*;

pub(crate) fn preview_active_detail_subscription(
    suffix: &str,
) -> (
    crate::facade::RuntimeBridge,
    crate::facade::BridgePreviewActiveSubscription,
) {
    preview_active_subscription_from_ready(activation_ready_detail_subscription(), suffix)
}

pub(crate) fn preview_promotion_detail_subscription(
    suffix: &str,
) -> (
    crate::facade::RuntimeBridge,
    crate::facade::BridgePreviewActiveSubscription,
    crate::facade::BridgePreviewPromotionRecord,
    crate::facade::BridgeSubscriptionActivationReady,
) {
    let (runtime, ready) = activation_ready_detail_subscription();
    let admitted_preview = runtime
        .admit_preview_session(
            crate::facade::BridgePreviewSessionIdentity::new(format!("preview-session:{suffix}")),
            subscription_preview_declaration(suffix),
        )
        .expect("preview session should admit");
    let (active_preview_session, execution_record) =
        runtime.activate_preview_session(admitted_preview, 3, 1, 2);
    let preview_basis = runtime
        .admit_subscription_preview_basis(&active_preview_session, &execution_record)
        .expect("preview basis should admit");
    let cost_profile = runtime
        .admit_subscription_delivery_cost_profile(
            BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery,
            4,
            4,
            1,
        )
        .expect("cost profile should admit");
    let consumer = canonical_consumer_contract(&runtime);
    let preview_active = runtime.activate_preview_subscription_delivery(
        ready,
        preview_basis,
        cost_profile,
        consumer,
    );
    let proof = active_preview_session.promotion_admissibility_proof();
    let (_promoted_session, promotion_record) = runtime
        .promote_preview_session(
            active_preview_session,
            &execution_record,
            &proof,
            format!("commit-boundary:{suffix}"),
            format!("authoritative-artifact:{suffix}"),
        )
        .expect("speculation promotion should succeed");
    let promoted_ready = activation_ready_detail_subscription_in_runtime(&runtime);

    (runtime, preview_active, promotion_record, promoted_ready)
}

pub(crate) fn preview_work_trace(
    runtime: &crate::facade::RuntimeBridge,
    preview_active: &crate::facade::BridgePreviewActiveSubscription,
    suffix: &str,
) -> BridgeSubscriptionPreviewWorkTrace {
    runtime
        .record_preview_subscription_work(
            preview_active,
            vec![
                BridgeSubscriptionPreviewWorkInput::routing(format!("preview-routing:{suffix}")),
                BridgeSubscriptionPreviewWorkInput::delivery(format!("preview-delivery:{suffix}")),
                BridgeSubscriptionPreviewWorkInput::diagnostics(format!(
                    "preview-diagnostics:{suffix}"
                )),
                BridgeSubscriptionPreviewWorkInput::continuation(format!(
                    "preview-continuation:{suffix}"
                )),
            ],
        )
        .expect("preview work trace should bind all preview work categories")
}

pub(crate) fn preview_active_collection_subscription(
    suffix: &str,
) -> (
    crate::facade::RuntimeBridge,
    crate::facade::BridgePreviewActiveSubscription,
) {
    preview_active_subscription_from_ready(activation_ready_collection_subscription(), suffix)
}

pub(crate) fn preview_active_subscription_from_ready(
    (runtime, ready): (
        crate::facade::RuntimeBridge,
        crate::facade::BridgeSubscriptionActivationReady,
    ),
    suffix: &str,
) -> (
    crate::facade::RuntimeBridge,
    crate::facade::BridgePreviewActiveSubscription,
) {
    let admitted_preview = runtime
        .admit_preview_session(
            crate::facade::BridgePreviewSessionIdentity::new(format!("preview-session:{suffix}")),
            subscription_preview_declaration(suffix),
        )
        .expect("preview session should admit");
    let (active_preview, execution_record) =
        runtime.activate_preview_session(admitted_preview, 3, 1, 2);
    let preview_basis = runtime
        .admit_subscription_preview_basis(&active_preview, &execution_record)
        .expect("preview basis should admit");
    let cost_profile = runtime
        .admit_subscription_delivery_cost_profile(
            BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery,
            4,
            4,
            1,
        )
        .expect("cost profile should admit");
    let consumer = canonical_consumer_contract(&runtime);
    let preview_active = runtime.activate_preview_subscription_delivery(
        ready,
        preview_basis,
        cost_profile,
        consumer,
    );
    (runtime, preview_active)
}

pub(crate) fn subscription_preview_declaration(
    suffix: &str,
) -> crate::facade::BridgePreviewSessionDeclaration {
    crate::facade::BridgePreviewSessionDeclaration::new(
        crate::facade::BridgePreviewSessionDeclarationIdentity::new(format!(
            "preview-declaration:{suffix}"
        )),
        crate::facade::BridgeRequestKind::Preview,
        crate::facade::BridgeSpeculativeBranchBinding::new(
            crate::facade::BridgeSpeculativeBranchBindingIdentity::new(format!(
                "preview-binding:{suffix}"
            )),
            TruthBranchIdentity::new(format!("truth-branch:{suffix}")),
            crate::facade::BridgeSignalBranchIdentity::new(format!("signal-branch:{suffix}")),
        ),
        format!("truth-view:{suffix}"),
        format!("source-capability:{suffix}"),
        format!("request-shape:{suffix}"),
        format!("artifact-schema:{suffix}"),
    )
}

pub(crate) fn zero_preview_residue_inputs(
    suffix: &str,
) -> Vec<crate::facade::BridgeSubscriptionPreviewResidueArtifactInput> {
    [
        crate::facade::BridgeSubscriptionPreviewResidueCategory::AuthoritativeTruthSubscription,
        crate::facade::BridgeSubscriptionPreviewResidueCategory::BridgeSubscriptionRegistry,
        crate::facade::BridgeSubscriptionPreviewResidueCategory::ActiveDelivery,
        crate::facade::BridgeSubscriptionPreviewResidueCategory::FanoutConsumerContract,
        crate::facade::BridgeSubscriptionPreviewResidueCategory::Continuation,
        crate::facade::BridgeSubscriptionPreviewResidueCategory::CheckpointReplay,
        crate::facade::BridgeSubscriptionPreviewResidueCategory::SignalVisible,
    ]
    .into_iter()
    .map(|category| {
        crate::facade::BridgeSubscriptionPreviewResidueArtifactInput::zero(
            category,
            format!("preview-residue-evidence:{suffix}:{}", category.as_str()),
        )
    })
    .collect()
}

pub(crate) fn preview_residue_inputs_with_count(
    suffix: &str,
    nonzero_category: crate::facade::BridgeSubscriptionPreviewResidueCategory,
    residue_count: usize,
) -> Vec<crate::facade::BridgeSubscriptionPreviewResidueArtifactInput> {
    zero_preview_residue_inputs(suffix)
        .into_iter()
        .map(|input| {
            if input.category() == nonzero_category {
                crate::facade::BridgeSubscriptionPreviewResidueArtifactInput::new(
                    nonzero_category,
                    residue_count,
                    format!(
                        "preview-residue-evidence:{suffix}:{}:nonzero",
                        nonzero_category.as_str()
                    ),
                )
            } else {
                input
            }
        })
        .collect()
}
