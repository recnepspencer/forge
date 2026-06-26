use forge_query::facade::{
    discover_basis_lifecycle_support, BasisFamily, ForgeQueryApplicationFacade,
    ForgeQueryCapabilityFamily, ForgeQueryConfig, ForgeQueryQueryConfig,
    ForgeQueryRelationalConfig, ForgeQueryRuntimeBridgeConfig, ForgeQuerySignalConfig,
    QuerySubscriptionFamily, QuerySubscriptionSupportPosture, ResultShapeFamily,
    ViewShapeDescriptor,
};
use worth_ui::facade::{
    QueryBasisPostureReference, QueryDenialPresentation, QueryLiveCompatibility,
    QueryResultShapeReference, QueryViewCapabilityReference, ViewBindingDescriptor,
    ViewBindingFamily, ViewBindingId, VisibleStateBindingDeclaration,
};

pub(crate) fn table_view_binding(id: &str) -> ViewBindingDescriptor {
    complete_view_binding(
        id,
        ViewBindingFamily::collection(),
        ViewShapeDescriptor::table(),
    )
    .with_result_shape(QueryResultShapeReference::from_result_shape_family(
        ResultShapeFamily::Collection,
    ))
}

pub(crate) fn detail_view_binding(id: &str) -> ViewBindingDescriptor {
    complete_view_binding(
        id,
        ViewBindingFamily::detail(),
        ViewShapeDescriptor::detail(),
    )
    .with_result_shape(QueryResultShapeReference::from_result_shape_family(
        ResultShapeFamily::Detail,
    ))
}

pub(crate) fn complete_view_binding(
    id: &str,
    family: ViewBindingFamily,
    view_shape: ViewShapeDescriptor,
) -> ViewBindingDescriptor {
    with_query_support_and_composition(
        ViewBindingDescriptor::query_owned(view_binding_id(id), family)
            .with_view_shape(view_shape)
            .with_basis_posture(admitted_basis_posture())
            .with_live_compatibility(query_live_compatibility())
            .with_visible_state_binding(VisibleStateBindingDeclaration::new("loading_posture"))
            .with_denial_presentation(QueryDenialPresentation::structured_status()),
    )
}

pub(crate) fn query_live_compatibility() -> QueryLiveCompatibility {
    QueryLiveCompatibility::declaration_only(QuerySubscriptionFamily::CollectionMembership)
}

pub(crate) fn denied_query_live_compatibility() -> QueryLiveCompatibility {
    QueryLiveCompatibility::from_subscription_posture(
        QuerySubscriptionFamily::CollectionMembership,
        QuerySubscriptionSupportPosture::RuntimeBackedDenied,
    )
}

pub(crate) fn with_query_support_and_composition(
    descriptor: ViewBindingDescriptor,
) -> ViewBindingDescriptor {
    let support_report = ForgeQueryApplicationFacade::runtime_backed_default().support_report();
    let query_capability = support_report
        .support_matrix()
        .descriptor(ForgeQueryCapabilityFamily::QueryComposition)
        .expect("runtime-backed query composition support row");
    let query_composition = support_report
        .query_composition_support_profile()
        .expect("runtime-backed query composition profile");

    descriptor
        .with_query_capability_posture(
            QueryViewCapabilityReference::from_query_capability_descriptor(query_capability),
        )
        .with_query_composition_support(query_composition)
}

pub(crate) fn admitted_basis_posture() -> QueryBasisPostureReference {
    let basis_support =
        discover_basis_lifecycle_support(BasisFamily::CurrentHead, "subscription_declaration");
    QueryBasisPostureReference::from_basis_support_discovery(&basis_support)
}

pub(crate) fn unsupported_query_capability_binding(id: &str) -> ViewBindingDescriptor {
    let support_report = ForgeQueryApplicationFacade::new(
        ForgeQueryConfig::runtime_backed_default()
            .with_query(ForgeQueryQueryConfig::disabled())
            .with_signal(ForgeQuerySignalConfig::disabled())
            .with_runtime_bridge(ForgeQueryRuntimeBridgeConfig::disabled())
            .with_relational(ForgeQueryRelationalConfig::disabled()),
    )
    .expect("disabled query config still produces a facade")
    .support_report();
    let query_capability = support_report
        .support_matrix()
        .descriptor(ForgeQueryCapabilityFamily::QueryComposition)
        .expect("disabled query config still reports query composition posture");
    let basis_support =
        discover_basis_lifecycle_support(BasisFamily::CurrentHead, "subscription_declaration");

    ViewBindingDescriptor::query_owned(view_binding_id(id), ViewBindingFamily::collection())
        .with_query_capability_posture(
            QueryViewCapabilityReference::from_query_capability_descriptor(query_capability),
        )
        .with_view_shape(ViewShapeDescriptor::table())
        .with_result_shape(QueryResultShapeReference::from_result_shape_family(
            ResultShapeFamily::Collection,
        ))
        .with_basis_posture(QueryBasisPostureReference::from_basis_support_discovery(
            &basis_support,
        ))
        .with_denial_presentation(QueryDenialPresentation::structured_status())
}

pub(crate) fn deferred_basis_view_binding(id: &str) -> ViewBindingDescriptor {
    let basis_support = discover_basis_lifecycle_support(BasisFamily::StoreBacked, "observation");
    table_view_binding(id).with_basis_posture(
        QueryBasisPostureReference::from_basis_support_discovery(&basis_support),
    )
}

pub(crate) fn pseudo_query_view_binding(id: &str) -> ViewBindingDescriptor {
    ViewBindingDescriptor::local_pseudo_query_for_diagnostics(
        view_binding_id(id),
        ViewBindingFamily::collection(),
        "ui_cache.tasks",
    )
}

pub(crate) fn view_binding_id(raw_text: &str) -> ViewBindingId {
    ViewBindingId::new(raw_text).expect("valid view binding id")
}
