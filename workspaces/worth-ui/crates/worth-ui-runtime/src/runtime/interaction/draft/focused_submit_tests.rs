use worth_ui_host_contract::{
    UiHostObservationPresentationBasis, UiHostPresentationEpoch, UiMountedFrameIdentity,
    UiMountedInstanceIdentity, UiMountedNodeReceiptIdentity, UiSemanticSurfaceIdentity,
    UiSurfaceBindingGeneration,
};

use super::{
    UiDraftRuntimeState, UiLocalInputRecipientBindingContext,
    UiLocalInputRecipientBindingStopReason, UiLocalInputRecipientFamily, UiLocalInputStopReason,
};

#[test]
fn focused_submit_is_not_active_until_host_affinity_is_installed() {
    let session =
        crate::runtime::tests::active_application_session_test_support::source_backed_component_session();
    let generation = session.active_generation_identity();
    let mounted = crate::mounting::WorthUiMountedSessionState::new(
        session.host_session_identity(),
        Default::default(),
        None,
    )
    .expect("the test host session owns mounted state");
    let target = target_view();
    let context = UiLocalInputRecipientBindingContext::new(
        91,
        worth_ui_host_contract::UiHostApplicationGeneration::new(1).unwrap(),
        &generation,
        &mounted,
    );
    let mut state = UiDraftRuntimeState::new();

    let denial = state
        .bind_focused_submit(target, context, |_| false)
        .expect_err("a rejected host affinity cannot create a keyboard recipient");

    assert_eq!(
        denial,
        UiLocalInputRecipientBindingStopReason::HostAffinityInstallationDenied
    );
    assert_eq!(state.snapshot().active_recipients, 0);
    assert!(state.active_input_binding().is_none());
}

#[test]
fn focused_submit_carries_exact_host_affinity_and_closes_with_its_instance() {
    let session =
        crate::runtime::tests::active_application_session_test_support::source_backed_component_session();
    let generation = session.active_generation_identity();
    let mounted = crate::mounting::WorthUiMountedSessionState::new(
        session.host_session_identity(),
        Default::default(),
        None,
    )
    .expect("the test host session owns mounted state");
    let target = target_view();
    let context = UiLocalInputRecipientBindingContext::new(
        92,
        worth_ui_host_contract::UiHostApplicationGeneration::new(1).unwrap(),
        &generation,
        &mounted,
    );
    let mut installed = None;
    let mut state = UiDraftRuntimeState::new();

    let receipt = state
        .bind_focused_submit(target, context, |binding| {
            installed = Some(binding);
            true
        })
        .expect("the host accepts the exact focused recipient");
    let binding = installed.expect("installation receives exact host affinity");

    assert_eq!(receipt.family(), UiLocalInputRecipientFamily::Submit);
    assert_eq!(binding.family(), UiLocalInputRecipientFamily::Submit);
    assert_eq!(binding.mounted_instance(), target.mounted_instance());
    assert_eq!(binding.node_receipt(), target.node_receipt());
    assert_eq!(state.active_input_binding(), Some(binding));

    let stops = state.cancel_instance(
        target.mounted_instance(),
        UiLocalInputStopReason::RecipientReplaced,
    );
    assert_eq!(stops.len(), 1);
    assert!(stops[0].settled_recipient());
    assert!(state.active_input_binding().is_none());
}

fn target_view() -> crate::runtime::interaction::UiPresentedInteractionTargetView {
    let binding = UiSurfaceBindingGeneration::mint_unbound().expect("binding identity capacity");
    let presentation = UiHostObservationPresentationBasis::new(
        worth_ui_host_contract::UiHostSurfaceIdentity::mint_unbound()
            .expect("host surface identity capacity"),
        UiMountedFrameIdentity::mint_unbound().expect("frame identity capacity"),
        binding,
        UiHostPresentationEpoch::issued_by_host(1),
    );
    crate::runtime::interaction::targeting::interaction_target_view_for_test(
        presentation,
        crate::mounting::UiMountedInteractionAffinityInput {
            surface: UiSemanticSurfaceIdentity::mint_unbound().expect("surface identity capacity"),
            binding,
            mounted_instance: UiMountedInstanceIdentity::mint_unbound()
                .expect("instance identity capacity"),
            node_receipt: UiMountedNodeReceiptIdentity::mint_unbound()
                .expect("node receipt identity capacity"),
        },
    )
}
