pub(crate) fn native_paint_completion(
    epoch: u64,
) -> worth_ui_host_contract::UiMountedSurfacePresentationCompletion {
    worth_ui_host_contract::UiMountedSurfacePresentationCompletion::new(
        worth_ui_host_contract::UiHostSurfacePresentationMode::NativeDisplay,
        worth_ui_host_contract::UiHostPresentationEpoch::issued_by_host(epoch),
        worth_ui_host_contract::UiMountedCompletedEffects::new(vec![
            worth_ui_host_contract::UiMountedEffectFamily::NativePaint,
        ]),
        worth_ui_host_contract::UiHostPresentationCostReport::default(),
    )
}
