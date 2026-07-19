use super::*;

#[test]
fn runtime_bridge_section_controls_preview_capability_without_query_section_drift() {
    let facade = WorthQueryApplicationFacade::new(
        WorthQueryConfig::runtime_backed_default()
            .with_runtime_bridge(WorthQueryRuntimeBridgeConfig::disabled()),
    )
    .expect("disabling runtime bridge alone should preserve a valid facade config");

    let error = facade
        .preview_query_capability()
        .expect_err("disabled runtime bridge section should deny preview capability admission");
    assert_eq!(
        error.failure_class(),
        WorthQueryFacadeFailureClass::MissingOwningSection
    );
    assert_eq!(
        error.capability_family(),
        Some(WorthQueryCapabilityFamily::PreviewSession)
    );
    assert_eq!(error.counters().unsupported_composition_denial_count(), 1);
    assert_eq!(error.counters().deferred_capability_denial_count(), 0);
}
