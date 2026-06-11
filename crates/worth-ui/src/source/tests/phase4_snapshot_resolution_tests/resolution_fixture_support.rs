use forge_query::facade::{
    discover_basis_lifecycle_support, BasisFamily, ForgeQueryApplicationFacade,
    ForgeQueryCapabilityFamily, QuerySubscriptionFamily, ResultShapeFamily, ViewShapeDescriptor,
};

use crate::capability::{
    CapabilitySnapshot, CapabilitySupportCatalog, ComponentChildPolicy, ComponentDescriptor,
    ComponentId, ComponentPropSchema, ComponentStateOwnership, QueryBasisPostureReference,
    QueryDenialPresentation, QueryLiveCompatibility, QueryResultShapeReference,
    QueryViewCapabilityReference, RegisteredCapabilitySet, RegistrationCandidate,
    SurfaceDescriptor, SurfaceId, SurfaceKind, SurfacePlacementClass, SurfaceStateClass,
    ThemeColorValue, ThemeTokenAlias, ThemeTokenDescriptor, ThemeTokenFamily, ThemeTokenId,
    ThemeTokenSource, ThemeTokenValue, ViewBindingDescriptor, ViewBindingFamily, ViewBindingId,
    COMPONENT_FAMILY_NAME, SURFACE_FAMILY_NAME, THEME_TOKEN_FAMILY_NAME, VIEW_BINDING_FAMILY_NAME,
};
use crate::facade::{WorthUi, WorthUiApp};
use crate::source::{
    WorthUiArtifactInput, WorthUiResolutionDiagnosticCode, WorthUiResolutionReport,
    WorthUiRustAuthoredArtifactInput, WorthUiRustAuthoredArtifactInputModule,
    WorthUiRustAuthoredToArtifactInputLowerer,
};

pub(super) fn standard_artifact_input() -> WorthUiArtifactInput {
    WorthUiRustAuthoredToArtifactInputLowerer::lower(
        &WorthUiRustAuthoredArtifactInput::from_modules([
            WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
                .with_import("app/panels/inspector.wui")
                .with_component("workspace.component.dashboard")
                .with_surface("workspace.surface.inspector")
                .with_binding("workspace.view_binding.selection")
                .with_token("theme.text.default", "theme.text.primary"),
            WorthUiRustAuthoredArtifactInputModule::new("app/panels/inspector.wui")
                .with_component("workspace.component.inspector_panel"),
        ]),
    )
}

pub(super) fn admitted_app() -> WorthUiApp {
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

    WorthUi::app()
        .register_component(component_descriptor("workspace.component.dashboard"))
        .register_component(component_descriptor("workspace.component.inspector_panel"))
        .register_view_binding(
            ViewBindingDescriptor::query_owned(
                ViewBindingId::new("workspace.view_binding.selection").unwrap(),
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
        .register_surface(
            SurfaceDescriptor::new(
                SurfaceId::new("workspace.surface.inspector").unwrap(),
                SurfaceKind::primary_content(),
                ComponentId::new("workspace.component.dashboard").unwrap(),
                SurfacePlacementClass::primary_region(),
                SurfaceStateClass::restorable(),
            )
            .with_view_binding(ViewBindingId::new("workspace.view_binding.selection").unwrap()),
        )
        .register_theme_token(ThemeTokenDescriptor::define(
            ThemeTokenId::new("theme.text.primary").unwrap(),
            ThemeTokenFamily::text(),
            ThemeTokenSource::application(),
            ThemeTokenValue::color(ThemeColorValue::hex("#101820").unwrap()),
        ))
        .register_theme_token(ThemeTokenDescriptor::alias(
            ThemeTokenId::new("theme.text.default").unwrap(),
            ThemeTokenFamily::text(),
            ThemeTokenSource::application(),
            ThemeTokenAlias::to(ThemeTokenId::new("theme.text.primary").unwrap()),
        ))
        .freeze()
}

pub(super) fn empty_snapshot() -> WorthUiApp {
    WorthUi::app().freeze()
}

pub(super) fn component_descriptor(identity: &str) -> ComponentDescriptor {
    ComponentDescriptor::new(
        ComponentId::new(identity).unwrap(),
        ComponentPropSchema::named("workspace.props"),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    )
}

pub(super) fn snapshot_with_support_catalog(
    base: &CapabilitySnapshot,
    support_catalog: CapabilitySupportCatalog,
) -> CapabilitySnapshot {
    CapabilitySnapshot::from_registered_capabilities_commands_command_projections_components_icons_surfaces_mosaic_regions_mosaic_placements_mosaic_sizing_mosaic_state_native_capabilities_plugin_slots_view_bindings_runtime_outcome_projections_settings_task_presentations_and_theme_tokens(
        clone_registered_capabilities(base),
        base.commands().clone(),
        base.command_projections().clone(),
        base.components().clone(),
        base.icons().clone(),
        base.surfaces().clone(),
        base.mosaic_regions().clone(),
        base.mosaic_placement_policies().clone(),
        base.mosaic_sizing_contracts().clone(),
        base.mosaic_state_slots().clone(),
        base.native_capabilities().clone(),
        base.plugin_slots().clone(),
        base.view_bindings().clone(),
        base.runtime_outcome_projections().clone(),
        base.settings().clone(),
        base.task_presentations().clone(),
        base.theme_tokens().clone(),
        support_catalog,
    )
}

pub(super) fn support_catalog_with_extra<const N: usize>(
    extra: [RegistrationCandidate; N],
) -> CapabilitySupportCatalog {
    let mut candidates = vec![
        RegistrationCandidate::admitted(COMPONENT_FAMILY_NAME, "workspace.component.dashboard"),
        RegistrationCandidate::admitted(
            COMPONENT_FAMILY_NAME,
            "workspace.component.inspector_panel",
        ),
        RegistrationCandidate::admitted(SURFACE_FAMILY_NAME, "workspace.surface.inspector"),
        RegistrationCandidate::admitted(
            VIEW_BINDING_FAMILY_NAME,
            "workspace.view_binding.selection",
        ),
        RegistrationCandidate::admitted(THEME_TOKEN_FAMILY_NAME, "theme.text.primary"),
        RegistrationCandidate::admitted(THEME_TOKEN_FAMILY_NAME, "theme.text.default"),
    ];
    candidates.extend(extra);
    CapabilitySupportCatalog::from_registration_candidates(&candidates)
}

pub(super) fn diagnostic_codes(
    report: &WorthUiResolutionReport,
) -> Vec<WorthUiResolutionDiagnosticCode> {
    report
        .diagnostics()
        .iter()
        .map(|diagnostic| diagnostic.code())
        .collect()
}

fn clone_registered_capabilities(snapshot: &CapabilitySnapshot) -> RegisteredCapabilitySet {
    let registered = snapshot.registered_capabilities();
    RegisteredCapabilitySet::from_counts(
        registered.registered_family_count(),
        registered.total_width(),
    )
}
