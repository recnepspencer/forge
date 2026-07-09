use worth_query::facade::{
    discover_basis_lifecycle_support, BasisFamily, QuerySubscriptionFamily,
    QuerySubscriptionSupportPosture, ResultShapeFamily, ViewShapeDescriptor,
    WorthQueryApplicationFacade, WorthQueryCapabilityFamily,
};

use super::replacement_impact_test_support::{
    artifact_from_modules, impact_test_app, token_module,
};
use crate::capability::{
    ComponentChildPolicy, ComponentDescriptor, ComponentId, ComponentPropSchema,
    ComponentStateOwnership, QueryBasisPostureReference, QueryDenialPresentation,
    QueryLiveCompatibility, QueryResultShapeReference, QueryViewCapabilityReference,
    SurfaceDescriptor, SurfaceId, SurfaceKind, SurfacePlacementClass, SurfaceStateClass,
    ThemeColorValue, ThemeTokenDescriptor, ThemeTokenFamily, ThemeTokenId, ThemeTokenSource,
    ThemeTokenValue, ViewBindingDescriptor, ViewBindingFamily, ViewBindingId,
};
use crate::facade::{WorthUi, WorthUiApp};
use crate::runtime::{
    WorthUiRuntimeHost, WorthUiRuntimeLaunch, WorthUiSourceProvider, WorthUiWatchedArtifactInput,
};
use crate::source::{WorthUiArtifact, WorthUiRustAuthoredArtifactInputModule};

pub(super) fn storm_app() -> WorthUiApp {
    impact_test_app()
}

pub(super) fn rich_storm_app() -> WorthUiApp {
    WorthUi::app()
        .register_component(component("workspace.component.dashboard"))
        .register_surface(surface("workspace.surface.main"))
        .register_view_binding(query_binding("workspace.view_binding.selection"))
        .register_theme_token(theme_token("theme.text.primary", "#101820"))
        .register_theme_token(theme_token("theme.text.secondary", "#C7492A"))
        .freeze()
}

pub(super) fn token_artifact(app: &WorthUiApp, token_id: &str) -> WorthUiArtifact {
    artifact_from_modules(app, [token_module(token_id)])
}

pub(super) fn rich_artifact(app: &WorthUiApp, token_id: &str) -> WorthUiArtifact {
    artifact_from_modules(
        app,
        [WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
            .with_component("workspace.component.dashboard")
            .with_surface("workspace.surface.main")
            .with_binding("workspace.view_binding.selection")
            .with_token(token_id, token_id)],
    )
}

pub(super) fn runtime_with_token(app: &WorthUiApp, token_id: &str) -> WorthUiRuntimeHost {
    app.launch_runtime(WorthUiRuntimeLaunch::from_canonical_artifact(
        token_artifact(app, token_id),
    ))
    .expect("runtime launches")
}

pub(super) fn runtime_with_rich_artifact(app: &WorthUiApp, token_id: &str) -> WorthUiRuntimeHost {
    app.launch_runtime(WorthUiRuntimeLaunch::from_canonical_artifact(
        rich_artifact(app, token_id),
    ))
    .expect("runtime launches")
}

pub(super) fn file_token_provider(token_id: &str) -> WorthUiSourceProvider {
    WorthUiSourceProvider::filesystem_root(r"C:\workspace").with_file(
        "app/main.wui",
        format!(r#"token {token_id} = "{token_id}";"#),
    )
}

pub(super) fn rust_token_provider(app: &WorthUiApp, token_id: &str) -> WorthUiSourceProvider {
    WorthUiSourceProvider::rust_authored_artifact(format!("rust-authored-{token_id}"))
        .with_artifact_input(WorthUiWatchedArtifactInput::from_rust_authored_artifact(
            format!("rust-token-{token_id}"),
            token_artifact(app, token_id),
        ))
}

pub(super) fn rich_file_provider(token_id: &str) -> WorthUiSourceProvider {
    WorthUiSourceProvider::filesystem_root(r"C:\workspace").with_file(
        "app/main.wui",
        format!(
            r#"
            component workspace.component.dashboard {{}}
            surface workspace.surface.main {{}}
            binding workspace.view_binding.selection {{}}
            token {token_id} = "{token_id}";
            "#
        ),
    )
}

pub(super) fn rich_rust_provider(app: &WorthUiApp, token_id: &str) -> WorthUiSourceProvider {
    WorthUiSourceProvider::rust_authored_artifact(format!("rust-authored-rich-{token_id}"))
        .with_artifact_input(WorthUiWatchedArtifactInput::from_rust_authored_artifact(
            format!("rust-rich-{token_id}"),
            rich_artifact(app, token_id),
        ))
}

pub(super) fn invalid_file_provider(label: &str) -> WorthUiSourceProvider {
    WorthUiSourceProvider::filesystem_root(r"C:\workspace").with_file("app/main.wui", label)
}

fn component(id: &str) -> ComponentDescriptor {
    ComponentDescriptor::new(
        ComponentId::new(id).expect("valid component id"),
        ComponentPropSchema::named("workspace.props"),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    )
}

fn surface(id: &str) -> SurfaceDescriptor {
    SurfaceDescriptor::new(
        SurfaceId::new(id).expect("valid surface id"),
        SurfaceKind::primary_content(),
        ComponentId::new("workspace.component.dashboard").expect("valid component id"),
        SurfacePlacementClass::primary_region(),
        SurfaceStateClass::restorable(),
    )
}

fn theme_token(id: &str, color: &str) -> ThemeTokenDescriptor {
    ThemeTokenDescriptor::define(
        ThemeTokenId::new(id).expect("valid token id"),
        ThemeTokenFamily::text(),
        ThemeTokenSource::application(),
        ThemeTokenValue::color(ThemeColorValue::hex(color).expect("valid color")),
    )
}

fn query_binding(id: &str) -> ViewBindingDescriptor {
    let support_report = WorthQueryApplicationFacade::runtime_backed_default().support_report();
    let query_capability = support_report
        .support_matrix()
        .descriptor(WorthQueryCapabilityFamily::QueryComposition)
        .expect("query composition support posture");
    let query_composition = support_report
        .query_composition_support_profile()
        .expect("query composition profile");
    let basis_support =
        discover_basis_lifecycle_support(BasisFamily::CurrentHead, "subscription_declaration");

    ViewBindingDescriptor::query_owned(
        ViewBindingId::new(id).expect("valid view binding id"),
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
    .with_live_compatibility(QueryLiveCompatibility::from_subscription_posture(
        QuerySubscriptionFamily::CollectionMembership,
        QuerySubscriptionSupportPosture::RuntimeBackedCertified,
    ))
    .with_denial_presentation(QueryDenialPresentation::structured_status())
}
