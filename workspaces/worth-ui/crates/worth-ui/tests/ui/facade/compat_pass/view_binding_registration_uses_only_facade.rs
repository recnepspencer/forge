use worth_query::facade::foundation::{
    discover_basis_lifecycle_support, BasisFamily, ResultShapeFamily,
    WorthQueryCapabilityFamily, WorthQuerySupportReport,
};
use worth_query::facade::runtime::{
    QuerySubscriptionFamily, ViewShapeDescriptor,
};
use worth_ui::facade::{
    QueryBasisPostureReference, QueryDenialPresentation, QueryLiveCompatibility,
    QueryResultShapeReference, QueryViewCapabilityReference, ViewBindingDescriptor,
    ViewBindingFamily, ViewBindingId, WorthUi,
};

fn main() {
    let query_support = WorthQuerySupportReport::runtime_backed_default();
    let query_capability = query_support
        .support_matrix()
        .descriptor(WorthQueryCapabilityFamily::QueryComposition)
        .expect("query composition support posture");
    let query_composition = query_support
        .query_composition_support_profile()
        .expect("query composition profile");
    let basis_support =
        discover_basis_lifecycle_support(BasisFamily::CurrentHead, "subscription_declaration");

    let _app = WorthUi::app()
        .register_view_binding(
            ViewBindingDescriptor::query_owned(
                ViewBindingId::new("workspace.view_binding.tasks").unwrap(),
                ViewBindingFamily::collection(),
            )
            .with_query_capability_posture(
                QueryViewCapabilityReference::from_query_capability_descriptor(query_capability),
            )
            .with_query_composition_support(query_composition)
            .with_view_shape(ViewShapeDescriptor::table())
            .with_result_shape(QueryResultShapeReference::from_result_shape_family(
                ResultShapeFamily::Collection,
            ))
            .with_basis_posture(QueryBasisPostureReference::from_basis_support_discovery(
                &basis_support,
            ))
            .with_live_compatibility(QueryLiveCompatibility::declaration_only(
                QuerySubscriptionFamily::CollectionMembership,
            ))
            .with_denial_presentation(QueryDenialPresentation::structured_status()),
        )
        .freeze();
}
