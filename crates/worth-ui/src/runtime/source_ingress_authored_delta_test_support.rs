use std::collections::BTreeSet;

use forge_query::facade::{
    discover_basis_lifecycle_support, BasisFamily, ForgeQueryApplicationFacade,
    ForgeQueryCapabilityFamily, QuerySubscriptionFamily, ResultShapeFamily, ViewShapeDescriptor,
};

pub(crate) use super::source_ingress_authored_delta_source_fixtures::{
    appearance_recipe_renamed_source_text, layout_changed_source_text,
    layout_gap_changed_source_text, layout_padding_changed_source_text,
    mixed_content_and_appearance_source_text, page_added_source_text, reordered_source_text,
    runtime_binding_added_source_text, shell_reassigned_source_text, source_text,
};
use crate::capability::{
    QueryBasisPostureReference, QueryDenialPresentation, QueryLiveCompatibility,
    QueryResultShapeReference, QueryViewCapabilityReference, ViewBindingDescriptor,
    ViewBindingFamily, ViewBindingId,
};
use crate::facade::{
    ComponentChildPolicy, ComponentDescriptor, ComponentId, ComponentPropSchema,
    ComponentStateOwnership, MosaicChildRule, MosaicClippingPosture, MosaicFocusScopeKind,
    MosaicHitTestPosture, MosaicRegionKindDescriptor, MosaicRegionKindId, MosaicRegionPersistence,
    MosaicRegionRole, MosaicScrollOwnership, MosaicSizingBehavior, SurfaceDescriptor, SurfaceId,
    SurfaceKind, SurfacePlacementClass, SurfaceStateClass, WorthUi, WorthUiApp,
    WorthUiRuntimeSourceModule,
};
use crate::runtime::{
    WorthUiAuthoredDeclarationKind, WorthUiAuthoredDeltaChangePosture,
    WorthUiAuthoredSemanticSubject, WorthUiObservedAuthoredEdit, WorthUiRuntimeHost,
    WorthUiSemanticSliceId, WorthUiValidationReloadRequest,
};

pub(crate) fn prepare_validation_reload(
    runtime: &WorthUiRuntimeHost,
    source: impl Into<String>,
) -> crate::runtime::WorthUiValidationPreparedReload {
    runtime.prepare_validation_reload(
        runtime.active_capability_snapshot(),
        WorthUiValidationReloadRequest::from_source_module("app/main.wui", source),
    )
}

pub(crate) fn validation_reload_request(
    source: impl Into<String>,
) -> crate::runtime::WorthUiValidationReloadRequest {
    WorthUiValidationReloadRequest::from_source_module("app/main.wui", source)
}

pub(crate) fn observed_authored_edit(
    source: impl Into<String>,
) -> crate::runtime::WorthUiObservedAuthoredEdit {
    WorthUiObservedAuthoredEdit::from_source_provider(
        crate::runtime::WorthUiSourceProvider::in_memory("validation-app-reload")
            .with_file("app/main.wui", source),
    )
    .expect("validation-app source module should lower into a real observed edit")
}

pub(crate) fn runtime_for_source(
    app: &WorthUiApp,
    source: impl Into<String>,
) -> WorthUiRuntimeHost {
    let prepared = WorthUi::runtime_launch()
        .from_source_module(WorthUiRuntimeSourceModule::new("app/main.wui", source))
        .prepare_authoring_for(app)
        .expect("source-authored runtime prepares");
    app.launch_runtime(prepared.into_runtime_launch())
        .expect("runtime launches")
}
pub(crate) fn authored_delta_test_app() -> WorthUiApp {
    WorthUi::app()
        .register_view_binding(query_bound_view_binding("workspace.view_binding.selection"))
        .register_component(ComponentDescriptor::new(
            ComponentId::new("workspace.component.product_list").unwrap(),
            ComponentPropSchema::named("workspace.props"),
            ComponentChildPolicy::no_children(),
            ComponentStateOwnership::runtime_owned(),
        ))
        .register_surface(validation_surface("validation.surface.products.collection"))
        .register_surface(validation_surface("validation.surface.orders.collection"))
        .register_mosaic_region_kind(layout_region(
            "worth.ui.layout.column",
            MosaicRegionRole::stack(),
            MosaicChildRule::accepts_regions(),
        ))
        .register_mosaic_region_kind(layout_region(
            "worth.ui.layout.row",
            MosaicRegionRole::split(),
            MosaicChildRule::accepts_regions(),
        ))
        .register_mosaic_region_kind(layout_region(
            "worth.ui.layout.slot",
            MosaicRegionRole::primary(),
            MosaicChildRule::accepts_surfaces(),
        ))
        .freeze()
}

pub(crate) fn changed_source_provider() -> crate::runtime::WorthUiSourceProvider {
    crate::runtime::WorthUiSourceProvider::in_memory("editor-buffer").with_file(
        "app/main.wui",
        source_text("validation.surface.orders.collection"),
    )
}

pub(crate) fn declaration_rows(
    authored_delta: &crate::runtime::WorthUiAuthoredDeltaSummary,
) -> BTreeSet<(
    WorthUiAuthoredDeclarationKind,
    String,
    WorthUiAuthoredDeltaChangePosture,
)> {
    authored_delta
        .touched_declaration_rows()
        .iter()
        .map(|row| {
            (
                row.kind(),
                row.declaration_name().to_owned(),
                row.change_posture(),
            )
        })
        .collect()
}

pub(crate) fn semantic_rows(
    authored_delta: &crate::runtime::WorthUiAuthoredDeltaSummary,
) -> BTreeSet<(
    WorthUiSemanticSliceId,
    String,
    WorthUiAuthoredDeltaChangePosture,
)> {
    authored_delta
        .semantic_slice_rows()
        .iter()
        .map(|row| {
            (
                row.slice_id(),
                subject_label(row.subject()),
                row.change_posture(),
            )
        })
        .collect()
}

pub(crate) fn semantic_fact_rows(
    receipt: &crate::runtime::WorthUiValidationChangedFactMappingReceipt,
) -> BTreeSet<(
    WorthUiSemanticSliceId,
    String,
    WorthUiAuthoredDeltaChangePosture,
    usize,
)> {
    receipt
        .rows()
        .iter()
        .map(|row| {
            (
                row.semantic_row().slice_id(),
                subject_label(row.semantic_row().subject()),
                row.semantic_row().change_posture(),
                row.changed_fact_count(),
            )
        })
        .collect()
}

pub(crate) fn semantic_fact_family_rows(
    receipt: &crate::runtime::WorthUiValidationChangedFactMappingReceipt,
) -> BTreeSet<(
    WorthUiSemanticSliceId,
    String,
    WorthUiAuthoredDeltaChangePosture,
    Vec<crate::runtime::WorthUiRuntimeFactFamily>,
)> {
    receipt
        .rows()
        .iter()
        .map(|row| {
            (
                row.semantic_row().slice_id(),
                subject_label(row.semantic_row().subject()),
                row.semantic_row().change_posture(),
                row.changed_fact_families().to_vec(),
            )
        })
        .collect()
}

pub(crate) fn subject_label(subject: &WorthUiAuthoredSemanticSubject) -> String {
    match subject {
        WorthUiAuthoredSemanticSubject::Workspace { workspace_name } => {
            format!("workspace:{workspace_name}")
        }
        WorthUiAuthoredSemanticSubject::Page { page_name } => format!("page:{page_name}"),
        WorthUiAuthoredSemanticSubject::PageSlot {
            page_name,
            slot_name,
        } => format!("page-slot:{page_name}:{slot_name}"),
        WorthUiAuthoredSemanticSubject::Surface { surface_id } => {
            format!("surface:{surface_id}")
        }
        WorthUiAuthoredSemanticSubject::AppearanceRecipe { recipe_name } => {
            format!("appearance:{recipe_name}")
        }
        WorthUiAuthoredSemanticSubject::RuntimeBinding { binding_name } => {
            format!("binding:{binding_name}")
        }
    }
}

fn validation_surface(id: &str) -> SurfaceDescriptor {
    SurfaceDescriptor::new(
        SurfaceId::new(id).unwrap(),
        SurfaceKind::primary_content(),
        ComponentId::new("workspace.component.product_list").unwrap(),
        SurfacePlacementClass::primary_region(),
        SurfaceStateClass::restorable(),
    )
}

fn layout_region(
    id: &str,
    role: MosaicRegionRole,
    child_rule: MosaicChildRule,
) -> MosaicRegionKindDescriptor {
    MosaicRegionKindDescriptor::new(MosaicRegionKindId::new(id).unwrap(), role)
        .with_persistence(MosaicRegionPersistence::restorable())
        .with_sizing_behavior(MosaicSizingBehavior::fills_available_space())
        .with_focus_scope(MosaicFocusScopeKind::active_surface_scope())
        .with_child_rule(child_rule)
        .with_allowed_surface_class(SurfacePlacementClass::primary_region())
        .with_scroll_ownership(MosaicScrollOwnership::region_owned())
        .with_clipping(MosaicClippingPosture::clip_to_region())
        .with_hit_test(MosaicHitTestPosture::participates())
}

fn query_bound_view_binding(binding_id: &str) -> ViewBindingDescriptor {
    let query_support = ForgeQueryApplicationFacade::runtime_backed_default().support_report();
    let query_capability = query_support
        .support_matrix()
        .descriptor(ForgeQueryCapabilityFamily::QueryComposition)
        .expect("query composition support posture");
    let query_composition = query_support
        .query_composition_support_profile()
        .expect("query composition profile");
    let basis_support =
        discover_basis_lifecycle_support(BasisFamily::CurrentHead, "subscription_declaration");

    ViewBindingDescriptor::query_owned(
        ViewBindingId::new(binding_id).unwrap(),
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
    .with_denial_presentation(QueryDenialPresentation::structured_status())
}
