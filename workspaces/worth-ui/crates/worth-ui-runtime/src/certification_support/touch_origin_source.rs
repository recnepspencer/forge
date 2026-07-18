use crate::capability::{
    ComponentChildPolicy, ComponentDescriptor, ComponentId, ComponentPropSchema,
};
use crate::facade::entry::{WorthUi, WorthUiApp};
use crate::facade::registry::{
    ComponentStateOwnership, SurfaceDescriptor, SurfaceId, SurfaceKind, SurfacePlacementClass,
    SurfaceStateClass,
};
use crate::runtime::{WorthUiRuntimeLaunch, WorthUiSourceProvider, WorthUiWatcherEvent};
use crate::source::{
    WorthUiArtifact, WorthUiBindingSemanticsLowerer, WorthUiCanonicalArtifactAssembler,
    WorthUiIdentitySeedLowerer, WorthUiRustAuthoredArtifactInput,
    WorthUiRustAuthoredArtifactInputModule, WorthUiRustAuthoredToArtifactInputLowerer,
    WorthUiStructuralLegalityLowerer,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiTouchOriginFixtureVariant {
    Baseline,
    OverlayArtifact,
    SameArtifactExtraPlanHook,
}

pub(super) fn touch_runtime_app() -> WorthUiApp {
    let support_app = touch_runtime_support_app();
    let submission = launch_runtime(&support_app, empty_runtime_artifact(&support_app))
        .source_ingress(touch_runtime_graph_source_provider())
        .start()
        .ingest([WorthUiWatcherEvent::provider_revision(
            touch_runtime_graph_provider_revision(),
        )])
        .expect("touch-origin graph provider should debounce")
        .lower_to_candidate_submission(support_app.capabilities())
        .expect("touch-origin graph provider should lower to a composition");

    touch_runtime_builder()
        .with_candidate_submission(submission)
        .freeze()
        .expect("application preparation should succeed")
}

fn touch_runtime_builder() -> crate::facade::entry::WorthUiBuilder {
    WorthUi::app()
        .register_component(ComponentDescriptor::new(
            ComponentId::new("workspace.component.dashboard").expect("valid component id"),
            ComponentPropSchema::named("workspace.props"),
            ComponentChildPolicy::no_children(),
            ComponentStateOwnership::runtime_owned(),
        ))
        .register_surface(SurfaceDescriptor::new(
            SurfaceId::new("workspace.surface.command_save").expect("valid surface id"),
            SurfaceKind::primary_content(),
            ComponentId::new("workspace.component.dashboard").expect("valid component id"),
            SurfacePlacementClass::primary_region(),
            SurfaceStateClass::restorable(),
        ))
}

pub(super) fn active_runtime_artifact(
    app: &WorthUiApp,
    variant: WorthUiTouchOriginFixtureVariant,
) -> WorthUiArtifact {
    replacement_candidate(app, variant)
        .artifact_bundle()
        .artifact()
        .clone()
}

pub(super) fn replacement_candidate(
    app: &WorthUiApp,
    variant: WorthUiTouchOriginFixtureVariant,
) -> crate::runtime::WorthUiReplacementCandidate {
    crate::runtime::candidate::rust_authored_replacement_candidate(
        runtime_origin_artifact(app, variant),
        app.capabilities().digest(),
        crate::runtime::WorthUiReplacementCause::rust_authored_input_change(
            runtime_origin_provider_revision(variant)
                .as_bytes()
                .iter()
                .fold(0_u64, |digest, byte| {
                    digest.rotate_left(5) ^ u64::from(*byte)
                }),
        ),
    )
    .expect("certification replacement candidate should lower")
}

pub(super) fn launch_runtime(
    app: &WorthUiApp,
    artifact: WorthUiArtifact,
) -> crate::runtime::WorthUiRuntime {
    app.launch_runtime(WorthUiRuntimeLaunch::from_canonical_artifact(artifact))
        .expect("runtime launches")
}

fn touch_runtime_support_app() -> WorthUiApp {
    touch_runtime_builder()
        .freeze()
        .expect("application preparation should succeed")
}

fn runtime_origin_artifact(
    app: &WorthUiApp,
    variant: WorthUiTouchOriginFixtureVariant,
) -> WorthUiArtifact {
    let module = match variant {
        WorthUiTouchOriginFixtureVariant::Baseline
        | WorthUiTouchOriginFixtureVariant::SameArtifactExtraPlanHook => {
            WorthUiRustAuthoredArtifactInputModule::new("app/graph_touch_origin_runtime.wui")
                .with_surface("workspace.surface.command_save")
        }
        WorthUiTouchOriginFixtureVariant::OverlayArtifact => {
            WorthUiRustAuthoredArtifactInputModule::new("app/graph_touch_origin_runtime.wui")
                .with_surface("workspace.surface.command_save")
                .with_component("workspace.component.dashboard")
        }
    };
    artifact_from_modules(app, [module])
}

fn empty_runtime_artifact(app: &WorthUiApp) -> WorthUiArtifact {
    artifact_from_modules(
        app,
        [WorthUiRustAuthoredArtifactInputModule::new(
            "touch-origin.empty",
        )],
    )
}

fn artifact_from_modules<const N: usize>(
    app: &WorthUiApp,
    modules: [WorthUiRustAuthoredArtifactInputModule; N],
) -> WorthUiArtifact {
    let artifact_input = WorthUiRustAuthoredToArtifactInputLowerer::lower(
        &WorthUiRustAuthoredArtifactInput::from_modules(modules),
    );
    let snapshot = app.capabilities();
    let resolved = crate::source::WorthUiArtifactInputResolver::resolve(&artifact_input, snapshot)
        .expect("artifact input resolves");
    let structured =
        WorthUiStructuralLegalityLowerer::lower(&resolved, snapshot).expect("structure lowers");
    let bound =
        WorthUiBindingSemanticsLowerer::lower(&structured, snapshot).expect("semantics lower");
    let identity_seeded = WorthUiIdentitySeedLowerer::lower(&bound)
        .expect("identity seeds lower")
        .0;
    WorthUiCanonicalArtifactAssembler::assemble(&identity_seeded)
        .expect("canonical artifact assembles")
}

fn touch_runtime_graph_source_provider() -> WorthUiSourceProvider {
    WorthUiSourceProvider::in_memory(touch_runtime_graph_provider_revision()).with_file(
        "app/graph_touch_origin_runtime.wui",
        "surface workspace.surface.command_save {}\ncomponent workspace.component.dashboard {}",
    )
}

fn touch_runtime_graph_provider_revision() -> &'static str {
    "touch-origin-graph"
}

fn runtime_origin_provider_revision(variant: WorthUiTouchOriginFixtureVariant) -> &'static str {
    match variant {
        WorthUiTouchOriginFixtureVariant::Baseline => "touch-origin-baseline",
        WorthUiTouchOriginFixtureVariant::OverlayArtifact => "touch-origin-overlay",
        WorthUiTouchOriginFixtureVariant::SameArtifactExtraPlanHook => {
            "touch-origin-same-artifact-extra-plan-hook"
        }
    }
}
