use crate::capability::{
    CommandDescriptor, CommandId, ComponentId, SurfaceId, WorthUiQueryViewRegistration,
};
use crate::facade::{WorthUi, WorthUiApp};
use crate::runtime::tests::replacement_impact_test_support::impact_test_app;
use crate::runtime::{
    WorthUiCandidateAdmission, WorthUiCandidateArtifactBundle, WorthUiCandidateAuthoringLane,
    WorthUiCandidateDependencyMetadata, WorthUiCandidateLoweringBasis, WorthUiQuerySupportReceipt,
    WorthUiQuerySupportStatus, WorthUiReplacementCandidate, WorthUiReplacementCause,
};
use crate::source::{
    WorthUiArtifact, WorthUiArtifactDigestor, WorthUiArtifactEquivalenceBasis,
    WorthUiBindingSemanticsLowerer, WorthUiCanonicalArtifactAssembler, WorthUiIdentitySeedLowerer,
    WorthUiRustAuthoredArtifactInput, WorthUiRustAuthoredArtifactInputModule,
    WorthUiRustAuthoredToArtifactInputLowerer, WorthUiStructuralLegalityLowerer,
};

pub(super) fn query_bound_app() -> WorthUiApp {
    let installed = worth_ui_query_binding::certification::worth_ui_installed_test_domain(
        "dependency-impact-query-app",
    );
    let builder = WorthUi::app()
        .register_query_view(query_view(&installed, "workspace.view_binding.selection"))
        .expect("installed selection view registers");
    builder
        .register_query_view(query_view(&installed, "workspace.view_binding.detail"))
        .expect("installed detail view registers")
        .freeze()
        .expect("application preparation should succeed")
}

pub(super) fn query_bound_surface_app() -> WorthUiApp {
    let base = impact_test_app();
    let installed = worth_ui_query_binding::certification::worth_ui_installed_test_domain(
        "dependency-impact-surface-query-app",
    );
    WorthUi::app()
        .register_command(CommandDescriptor::new(
            command_id("workspace.command.save"),
            "Save",
        ))
        .register_command(CommandDescriptor::new(
            command_id("workspace.command.open"),
            "Open",
        ))
        .register_component(component_descriptor_from_base_app(
            &base,
            "workspace.component.dashboard",
        ))
        .register_surface(surface_descriptor_from_base_app(
            &base,
            "workspace.surface.command_save",
        ))
        .register_surface(surface_descriptor_from_base_app(
            &base,
            "workspace.surface.command_open",
        ))
        .register_query_view(query_view(&installed, "workspace.view_binding.selection"))
        .expect("installed selection view registers")
        .freeze()
        .expect("application preparation should succeed")
}

pub(super) fn query_bound_artifact(app: &WorthUiApp, binding_id: &str) -> WorthUiArtifact {
    lower_rust_authored_artifact(
        app,
        [WorthUiRustAuthoredArtifactInputModule::new("app/main.wui").with_binding(binding_id)],
    )
}

pub(super) fn surface_and_query_binding_module(
    surface_id: &str,
    binding_id: &str,
) -> WorthUiRustAuthoredArtifactInputModule {
    WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
        .with_surface(surface_id)
        .with_binding(binding_id)
}

pub(super) fn lower_rust_authored_artifact<const N: usize>(
    app: &WorthUiApp,
    modules: [WorthUiRustAuthoredArtifactInputModule; N],
) -> WorthUiArtifact {
    let artifact_input = WorthUiRustAuthoredToArtifactInputLowerer::lower(
        &WorthUiRustAuthoredArtifactInput::from_modules(modules),
    );
    let resolved =
        crate::source::WorthUiArtifactInputResolver::resolve(&artifact_input, app.capabilities())
            .expect("artifact input resolves");
    let structured = WorthUiStructuralLegalityLowerer::lower(&resolved, app.capabilities())
        .expect("artifact structure lowers");
    let bound = WorthUiBindingSemanticsLowerer::lower(&structured, app.capabilities())
        .expect("artifact semantics lower");
    let identity_seeded = WorthUiIdentitySeedLowerer::lower(&bound)
        .expect("identity seeds lower")
        .0;
    WorthUiCanonicalArtifactAssembler::assemble(&identity_seeded)
        .expect("canonical artifact assembles")
}

pub(super) fn candidate_with_forged_query_support(
    runtime: &crate::runtime::WorthUiRuntime,
    artifact: WorthUiArtifact,
) -> crate::runtime::WorthUiAdmittedReplacementCandidate {
    candidate_with_forged_query_support_hook_count(runtime, artifact, 1)
}

pub(super) fn candidate_with_forged_query_support_hook_count(
    runtime: &crate::runtime::WorthUiRuntime,
    artifact: WorthUiArtifact,
    runtime_hook_count: usize,
) -> crate::runtime::WorthUiAdmittedReplacementCandidate {
    let artifact_digest =
        WorthUiArtifactDigestor::digest(&artifact, WorthUiArtifactEquivalenceBasis::semantic());
    let dependency_metadata = WorthUiCandidateDependencyMetadata::derive_for_artifact(&artifact);
    let lowering_basis = WorthUiCandidateLoweringBasis::from_raw_parts_for_test(
        runtime.replacement_admission_basis().snapshot_digest(),
        WorthUiQuerySupportReceipt::with_runtime_hook_count_for_test(
            WorthUiQuerySupportStatus::Supported,
            runtime_hook_count,
            "dependency-impact-narrowing",
        ),
    );
    let bundle = WorthUiCandidateArtifactBundle::from_optional_parts_for_test(
        artifact,
        Some(artifact_digest),
        Some(dependency_metadata),
        Some(lowering_basis),
    )
    .expect("hostile candidate bundle seals");
    let candidate = WorthUiReplacementCandidate::from_artifact_bundle(
        bundle,
        WorthUiReplacementCause::manual_refresh(77),
        WorthUiCandidateAuthoringLane::rust_authored(),
    )
    .expect("hostile candidate seals");
    WorthUiCandidateAdmission::for_active_basis(runtime.replacement_admission_basis())
        .admit(candidate)
        .expect("hostile candidate admits through forged query support receipt")
}

fn query_view(
    installed: &worth_ui_query_binding::WorthUiInstalledQueryDomain,
    id: &str,
) -> WorthUiQueryViewRegistration {
    WorthUiQueryViewRegistration::new(
        installed
            .measurement_view(id)
            .expect("installed Query view admits"),
    )
}

fn command_id(id: &str) -> CommandId {
    CommandId::new(id).expect("valid command id")
}

fn component_descriptor_from_base_app(
    base: &WorthUiApp,
    id: &str,
) -> crate::capability::ComponentDescriptor {
    let id = ComponentId::new(id).expect("valid component id");
    base.capabilities()
        .components()
        .descriptors()
        .iter()
        .find(|descriptor| descriptor.id() == &id)
        .expect("base app component exists")
        .clone()
}

fn surface_descriptor_from_base_app(
    base: &WorthUiApp,
    id: &str,
) -> crate::capability::SurfaceDescriptor {
    let id = SurfaceId::new(id).expect("valid surface id");
    base.capabilities()
        .surfaces()
        .descriptors()
        .iter()
        .find(|descriptor| descriptor.id() == &id)
        .expect("base app surface exists")
        .clone()
}
