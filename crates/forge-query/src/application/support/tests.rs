use crate::application::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryCapabilityStatus,
    ForgeQueryConfig, ForgeQueryQueryConfig, ForgeQueryRelationalConfig,
    ForgeQueryRuntimeBridgeConfig, ForgeQuerySignalConfig,
};
use crate::composition::runtime_backed_query_composition_support_profile;

fn query_disabled_application_facade() -> ForgeQueryApplicationFacade {
    ForgeQueryApplicationFacade::new(
        ForgeQueryConfig::runtime_backed_default()
            .with_query(ForgeQueryQueryConfig::disabled())
            .with_signal(ForgeQuerySignalConfig::disabled())
            .with_runtime_bridge(ForgeQueryRuntimeBridgeConfig::disabled())
            .with_relational(ForgeQueryRelationalConfig::disabled()),
    )
    .expect("query-disabled facade config should remain valid when dependents are disabled")
}

#[test]
fn support_report_publishes_verified_query_composition_profile_and_deferred_neighbors() {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let support = facade.support_matrix();
    let report = facade.support_report();
    let expected_profile = runtime_backed_query_composition_support_profile();

    assert_eq!(
        support
            .descriptor(ForgeQueryCapabilityFamily::QueryComposition)
            .expect("query composition descriptor should exist")
            .status(),
        ForgeQueryCapabilityStatus::Admitted
    );
    assert!(report
        .admitted_capability_families()
        .contains(&ForgeQueryCapabilityFamily::QueryComposition));

    let profile = report
        .query_composition_support_profile()
        .expect("query composition profile should be present");

    assert_eq!(
        profile, &expected_profile,
        "support report should publish the complete runtime-backed composition profile without drift"
    );
    assert_eq!(
        profile.profile_digest(),
        expected_profile.profile_digest(),
        "published support digest must match the runtime-backed authority profile digest"
    );
    assert_eq!(
        profile
            .admitted_scope_families()
            .iter()
            .map(|family| family.as_str())
            .collect::<Vec<_>>(),
        vec![
            "predicate_scope",
            "ordering_scope",
            "projection_scope",
            "traversal_bound_scope",
            "basis_aware_scope",
        ]
    );
    assert_eq!(
        profile
            .admitted_template_families()
            .iter()
            .map(|family| family.as_str())
            .collect::<Vec<_>>(),
        vec![
            "detail_template",
            "collection_template",
            "grouped_collection_template",
        ]
    );
    assert_eq!(
        profile
            .admitted_view_families()
            .iter()
            .map(|family| family.as_str())
            .collect::<Vec<_>>(),
        vec![
            "table",
            "detail",
            "inspector_detail_observed",
            "inspector_detail_focused",
            "kanban_grouped",
        ]
    );
    assert!(profile.deferred_view_families().is_empty());

    assert_eq!(
        profile
            .composition_statuses()
            .iter()
            .map(|(family, status)| format!("{}:{}", family.as_str(), status.as_str()))
            .collect::<Vec<_>>(),
        vec![
            "named_scope_expansion:verified".to_string(),
            "template_instantiation:verified".to_string(),
        ]
    );
    assert_eq!(
        profile
            .scope_temporal_async_postures()
            .iter()
            .map(|(family, posture)| format!("{}:{}", family.as_str(), posture.as_str()))
            .collect::<Vec<_>>(),
        vec![
            "predicate_scope:ordinary_only".to_string(),
            "ordering_scope:ordinary_only".to_string(),
            "projection_scope:ordinary_only".to_string(),
            "traversal_bound_scope:ordinary_only".to_string(),
            "basis_aware_scope:future_preserving".to_string(),
        ]
    );
    assert_eq!(
        profile
            .deferred_scope_markers()
            .iter()
            .map(|marker| marker.as_str())
            .collect::<Vec<_>>(),
        vec![
            "observed_inspector_detail_template",
            "focused_inspector_detail_template",
        ]
    );
    assert_eq!(
        profile
            .template_temporal_async_postures()
            .iter()
            .map(|(family, posture)| format!("{}:{}", family.as_str(), posture.as_str()))
            .collect::<Vec<_>>(),
        vec![
            "detail_template:ordinary_only".to_string(),
            "collection_template:ordinary_only".to_string(),
            "observed_inspector_detail_template:visible_but_deferred".to_string(),
            "focused_inspector_detail_template:visible_but_deferred".to_string(),
            "grouped_collection_template:future_preserving".to_string(),
        ]
    );
    assert_eq!(
        profile
            .view_shape_statuses()
            .iter()
            .map(|(family, status)| format!("{}:{}", family.as_str(), status.as_str()))
            .collect::<Vec<_>>(),
        vec![
            "table:verified".to_string(),
            "detail:verified".to_string(),
            "inspector_detail_observed:verified".to_string(),
            "inspector_detail_focused:verified".to_string(),
            "kanban_grouped:verified".to_string(),
        ]
    );
    assert_eq!(
        profile
            .view_shape_temporal_async_postures()
            .iter()
            .map(|(family, posture)| format!("{}:{}", family.as_str(), posture.as_str()))
            .collect::<Vec<_>>(),
        vec![
            "table:future_preserving".to_string(),
            "detail:future_preserving".to_string(),
            "inspector_detail_observed:visible_but_deferred".to_string(),
            "inspector_detail_focused:visible_but_deferred".to_string(),
            "kanban_grouped:future_preserving".to_string(),
        ]
    );
}

#[test]
fn support_report_hides_query_composition_profile_when_query_capability_is_disabled() {
    let facade = query_disabled_application_facade();
    let support = facade.support_matrix();
    let report = facade.support_report();

    assert_eq!(
        support
            .descriptor(ForgeQueryCapabilityFamily::QueryComposition)
            .expect("query composition descriptor should exist")
            .status(),
        ForgeQueryCapabilityStatus::Unsupported
    );
    assert!(report
        .unsupported_capability_families()
        .contains(&ForgeQueryCapabilityFamily::QueryComposition));
    assert_eq!(report.query_composition_support_profile(), None);
}

#[test]
fn support_report_digest_tracks_query_composition_profile_publication_posture() {
    let admitted_report = ForgeQueryApplicationFacade::runtime_backed_default().support_report();
    let denied_report = query_disabled_application_facade().support_report();

    let admitted_profile = admitted_report
        .query_composition_support_profile()
        .expect("admitted query composition profile should be published");

    assert_ne!(
        admitted_report.report_digest(),
        denied_report.report_digest(),
        "support report digest should change when query composition publication changes"
    );
    assert_ne!(
        admitted_profile.profile_digest(),
        "none",
        "published composition profile should carry a real support digest"
    );
}
