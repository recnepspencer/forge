use worth_query::facade::foundation::{
    discover_basis_lifecycle_support, BasisFamily, ResultShapeFamily, WorthQueryCapabilityFamily,
    WorthQuerySupportReport,
};
use worth_query::facade::runtime::{QuerySubscriptionFamily, ViewShapeDescriptor};

use crate::capability::{
    FrozenViewBindingEntry, QueryBasisPostureReference, QueryDenialPresentation,
    QueryLiveCompatibility, QueryResultShapeReference, QueryViewBindingKey,
    QueryViewCapabilityReference, ViewBindingDescriptor, ViewBindingFamily, ViewBindingId,
};

pub(super) fn standard_query_owned_view_binding_descriptor() -> ViewBindingDescriptor {
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

    query_owned_view_binding_descriptor(
        QueryViewCapabilityReference::from_query_capability_descriptor(query_capability),
        query_composition,
        QueryBasisPostureReference::from_basis_support_discovery(&basis_support),
        Some(QueryLiveCompatibility::declaration_only(
            QuerySubscriptionFamily::CollectionMembership,
        )),
    )
}

pub(super) fn query_owned_view_binding_without_live_compatibility_descriptor(
) -> ViewBindingDescriptor {
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

    query_owned_view_binding_descriptor(
        QueryViewCapabilityReference::from_query_capability_descriptor(query_capability),
        query_composition,
        QueryBasisPostureReference::from_basis_support_discovery(&basis_support),
        None,
    )
}

pub(super) fn frozen_view_binding_entry_for_descriptor(
    descriptor: ViewBindingDescriptor,
) -> FrozenViewBindingEntry {
    FrozenViewBindingEntry::new(
        descriptor,
        QueryViewBindingKey::from_digest_basis("phase6-test-binding-key"),
    )
}

fn query_owned_view_binding_descriptor(
    query_capability: QueryViewCapabilityReference,
    query_composition: &worth_query::facade::foundation::QueryCompositionSupportProfile,
    basis_posture: QueryBasisPostureReference,
    live_compatibility: Option<QueryLiveCompatibility>,
) -> ViewBindingDescriptor {
    let descriptor = ViewBindingDescriptor::query_owned(
        ViewBindingId::new("workspace.view_binding.selection").unwrap(),
        ViewBindingFamily::collection(),
    )
    .with_query_capability_posture(query_capability)
    .with_query_composition_support(query_composition)
    .with_view_shape(ViewShapeDescriptor::table())
    .with_result_shape(QueryResultShapeReference::from_result_shape_family(
        ResultShapeFamily::Collection,
    ))
    .with_basis_posture(basis_posture)
    .with_denial_presentation(QueryDenialPresentation::structured_status());
    match live_compatibility {
        Some(live_compatibility) => descriptor.with_live_compatibility(live_compatibility),
        None => descriptor,
    }
}
