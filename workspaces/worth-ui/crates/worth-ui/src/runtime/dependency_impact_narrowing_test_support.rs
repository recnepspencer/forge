use forge_query::facade::{
    discover_basis_lifecycle_support, BasisFamily, ForgeQueryApplicationFacade,
    ForgeQueryCapabilityFamily, QuerySubscriptionFamily, QuerySubscriptionSupportPosture,
    ResultShapeFamily, ViewShapeDescriptor,
};

use crate::capability::{
    CommandDescriptor, CommandId, ComponentId, QueryBasisPostureReference, QueryDenialPresentation,
    QueryLiveCompatibility, QueryResultShapeReference, QueryViewCapabilityReference, SurfaceId,
    ViewBindingDescriptor, ViewBindingFamily, ViewBindingId,
};
use crate::facade::{WorthUi, WorthUiApp};
use crate::runtime::{
    replacement_impact_test_support::impact_test_app, WorthUiCandidateAdmission,
    WorthUiCandidateArtifactBundle, WorthUiCandidateAuthoringLane,
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
    WorthUi::app()
        .register_view_binding(query_bound_view_binding("workspace.view_binding.selection"))
        .register_view_binding(query_bound_view_binding("workspace.view_binding.detail"))
        .freeze()
}

pub(super) fn query_bound_surface_app() -> WorthUiApp {
    let base = impact_test_app();
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
        .register_view_binding(query_bound_view_binding("workspace.view_binding.selection"))
        .freeze()
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
    runtime: &crate::runtime::WorthUiRuntimeHost,
    artifact: WorthUiArtifact,
) -> crate::runtime::WorthUiAdmittedReplacementCandidate {
    candidate_with_forged_query_support_hook_count(runtime, artifact, 1)
}

pub(super) fn candidate_with_forged_query_support_hook_count(
    runtime: &crate::runtime::WorthUiRuntimeHost,
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
            99,
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

fn query_bound_view_binding(id: &str) -> ViewBindingDescriptor {
    let support_report = ForgeQueryApplicationFacade::runtime_backed_default().support_report();
    let query_capability = support_report
        .support_matrix()
        .descriptor(ForgeQueryCapabilityFamily::QueryComposition)
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
