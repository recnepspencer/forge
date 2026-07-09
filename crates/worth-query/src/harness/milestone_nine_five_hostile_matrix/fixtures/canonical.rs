use crate::authoring::{
    AspectFieldSelector, AuthoredResultShapeField, OrderingSelector, RootEntityKey,
};
use crate::composition::{
    GuidedCompositionPath, QueryScopeDescriptor, QueryTemplateDescriptor, TemplateBindingSet,
    TemplateParameterSlot,
};
use crate::harness::fixtures::execution_preflights;
use crate::query_context::{
    admit_query_basis_context, bind_query_basis_context, QueryBasisContextRequest,
    QueryContextBindingSource,
};

pub fn direct_detail() -> crate::canonicalization::CanonicalQueryBundle {
    crate::authoring::GuidedAuthoringPath::canonicalize_detail(
        crate::authoring::RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .project(AspectFieldSelector::new("profile", "display_name").unwrap())
            .build()
            .unwrap(),
        detail_shape(),
    )
    .unwrap()
}

pub fn direct_collection() -> crate::canonicalization::CanonicalQueryBundle {
    crate::authoring::GuidedAuthoringPath::canonicalize_collection(
        crate::authoring::RawAuthoredQuery::collection_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .project(AspectFieldSelector::new("profile", "display_name").unwrap())
            .project(AspectFieldSelector::new("status", "lane").unwrap())
            .order_by(OrderingSelector::ascending("profile", "display_name").unwrap())
            .build()
            .unwrap(),
        collection_shape(),
    )
    .unwrap()
}

pub fn named_scope_collection() -> crate::composition::ComposedCanonicalQueryBundle {
    let base_query =
        crate::authoring::RawAuthoredQuery::collection_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .build()
            .unwrap();
    let (_artifact, expanded) = GuidedCompositionPath::expand_collection_scopes(
        base_query,
        collection_shape(),
        [
            QueryScopeDescriptor::projection(
                "profile_projection",
                [AspectFieldSelector::new("profile", "display_name").unwrap()],
            ),
            QueryScopeDescriptor::projection(
                "status_projection",
                [AspectFieldSelector::new("status", "lane").unwrap()],
            ),
            QueryScopeDescriptor::ordering(
                "display_name_ordering",
                [OrderingSelector::ascending("profile", "display_name").unwrap()],
            ),
        ],
    )
    .unwrap();
    GuidedCompositionPath::canonicalize_expanded(expanded).unwrap()
}

pub fn template_detail() -> crate::composition::ComposedCanonicalQueryBundle {
    let slot = TemplateParameterSlot::projection("profile_projection");
    let template = QueryTemplateDescriptor::detail(
        crate::authoring::RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .build()
            .unwrap(),
        detail_shape(),
    )
    .with_slot(slot.clone());
    let bindings = TemplateBindingSet::new().bind_projection(
        &slot,
        AspectFieldSelector::new("profile", "display_name").unwrap(),
    );
    let (_artifact, expanded) =
        GuidedCompositionPath::instantiate_detail_template(template, bindings).unwrap();
    GuidedCompositionPath::canonicalize_expanded(expanded).unwrap()
}

pub fn template_collection() -> crate::composition::ComposedCanonicalQueryBundle {
    let profile_slot = TemplateParameterSlot::projection("profile_projection");
    let status_slot = TemplateParameterSlot::projection("status_projection");
    let ordering_slot = TemplateParameterSlot::ordering("display_name_ordering");
    let template = QueryTemplateDescriptor::collection(
        crate::authoring::RawAuthoredQuery::collection_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .build()
            .unwrap(),
        collection_shape(),
    )
    .with_slot(profile_slot.clone())
    .with_slot(status_slot.clone())
    .with_slot(ordering_slot.clone());
    let bindings = TemplateBindingSet::new()
        .bind_projection(
            &profile_slot,
            AspectFieldSelector::new("profile", "display_name").unwrap(),
        )
        .bind_projection(
            &status_slot,
            AspectFieldSelector::new("status", "lane").unwrap(),
        )
        .bind_ordering(
            &ordering_slot,
            OrderingSelector::ascending("profile", "display_name").unwrap(),
        );
    let (_artifact, expanded) =
        GuidedCompositionPath::instantiate_collection_template(template, bindings).unwrap();
    GuidedCompositionPath::canonicalize_expanded(expanded).unwrap()
}

pub fn basis_aware_composed_collection() -> crate::composition::ComposedCanonicalQueryBundle {
    let direct = direct_collection();
    let evidence =
        crate::composition::BasisScopeEvidence::from_admitted_context_for_canonical_query(
            &admitted_current_head_context(),
            direct.query().digest(),
        );
    let (_artifact, expanded) = GuidedCompositionPath::expand_collection_scopes(
        crate::authoring::RawAuthoredQuery::collection_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .project(AspectFieldSelector::new("profile", "display_name").unwrap())
            .project(AspectFieldSelector::new("status", "lane").unwrap())
            .order_by(OrderingSelector::ascending("profile", "display_name").unwrap())
            .build()
            .unwrap(),
        collection_shape(),
        [QueryScopeDescriptor::basis_aware("current_basis", evidence)],
    )
    .unwrap();
    GuidedCompositionPath::canonicalize_expanded(expanded).unwrap()
}

pub fn basis_aware_composed_detail() -> crate::composition::ComposedCanonicalQueryBundle {
    let direct = direct_detail();
    let evidence =
        crate::composition::BasisScopeEvidence::from_admitted_context_for_canonical_query(
            &admitted_current_head_context(),
            direct.query().digest(),
        );
    let (_artifact, expanded) = GuidedCompositionPath::expand_detail_scopes(
        crate::authoring::RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .project(AspectFieldSelector::new("profile", "display_name").unwrap())
            .build()
            .unwrap(),
        detail_shape(),
        [QueryScopeDescriptor::basis_aware("current_basis", evidence)],
    )
    .unwrap();
    GuidedCompositionPath::canonicalize_expanded(expanded).unwrap()
}

fn admitted_current_head_context() -> crate::query_context::AdmittedQueryBasisContext {
    let preflight = execution_preflights::direct_runtime_preflight();
    let binding = bind_query_basis_context(
        QueryBasisContextRequest::current_branch_head(),
        QueryContextBindingSource::RuntimeCurrent(&preflight),
    )
    .unwrap();
    admit_query_basis_context(binding).unwrap()
}

fn detail_shape() -> crate::authoring::DetailAuthoredResultShape {
    crate::authoring::RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .field(AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap())
        .build()
        .unwrap()
}

fn collection_shape() -> crate::authoring::CollectionAuthoredResultShape {
    crate::authoring::RawAuthoredResultShape::collection_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .field(AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap())
        .field(AuthoredResultShapeField::new("status", "lane", "lane").unwrap())
        .build()
        .unwrap()
}
