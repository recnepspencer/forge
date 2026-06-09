use super::*;

#[test]
fn runtime_backed_default_bootstraps_all_lane_local_sections_as_enabled() {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let report = facade.support_report();

    let enabled_sections = report
        .section_postures()
        .iter()
        .filter(|posture| posture.enabled())
        .map(|posture| posture.section())
        .collect::<Vec<_>>();

    assert_eq!(
        enabled_sections,
        vec![
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::Relational,
            ForgeQueryConfigSectionFamily::Signal,
            ForgeQueryConfigSectionFamily::RuntimeBridge,
        ]
    );
}

#[test]
fn preview_section_disable_does_not_rewrite_query_context_bootstrap_posture() {
    let facade = ForgeQueryApplicationFacade::new(
        ForgeQueryConfig::runtime_backed_default()
            .with_runtime_bridge(ForgeQueryRuntimeBridgeConfig::disabled()),
    )
    .expect("runtime bridge disable should preserve bootstrap of the remaining sections");
    let report = facade.support_report();

    let query_context = report
        .query_context_support_profile()
        .expect("query context support should remain published without preview admission");

    assert!(query_context
        .admitted_basis_families()
        .iter()
        .any(|family| family.as_str() == "preview_derived_historical"));
    assert_eq!(
        query_context
            .deferred_scope_markers()
            .iter()
            .map(|marker| marker.as_str())
            .collect::<Vec<_>>(),
        vec![
            "store_backed_historical",
            "store_backed_diff",
            "broad_collection_diff"
        ]
    );
    assert!(report
        .unsupported_capability_families()
        .contains(&ForgeQueryCapabilityFamily::PreviewSession));
}
