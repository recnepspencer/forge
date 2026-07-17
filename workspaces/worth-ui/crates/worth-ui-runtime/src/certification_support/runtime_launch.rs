use crate::facade::entry::WorthUiApp;
use crate::runtime::{WorthUiRuntime, WorthUiRuntimeLaunch};
use crate::source::{
    WorthUiArtifactInputResolver, WorthUiBindingSemanticsLowerer,
    WorthUiCanonicalArtifactAssembler, WorthUiIdentitySeedLowerer,
    WorthUiRustAuthoredArtifactInput, WorthUiRustAuthoredArtifactInputModule,
    WorthUiRustAuthoredToArtifactInputLowerer, WorthUiStructuralLegalityLowerer,
};

/// Launch an app through an empty canonical artifact for external certification.
/// Production callers must arrive with a source-lowered candidate instead.
pub fn launch_empty_runtime_for_certification(app: &WorthUiApp) -> WorthUiRuntime {
    let input = WorthUiRustAuthoredToArtifactInputLowerer::lower(
        &WorthUiRustAuthoredArtifactInput::from_modules([
            WorthUiRustAuthoredArtifactInputModule::new("certification.empty"),
        ]),
    );
    let snapshot = app.capabilities();
    let resolved = WorthUiArtifactInputResolver::resolve(&input, snapshot)
        .expect("empty certification artifact input resolves");
    let structured = WorthUiStructuralLegalityLowerer::lower(&resolved, snapshot)
        .expect("empty certification artifact is structurally legal");
    let bound = WorthUiBindingSemanticsLowerer::lower(&structured, snapshot)
        .expect("empty certification artifact binds");
    let seeded = WorthUiIdentitySeedLowerer::lower(&bound)
        .expect("empty certification artifact receives identity seeds")
        .0;
    let artifact = WorthUiCanonicalArtifactAssembler::assemble(&seeded)
        .expect("empty certification artifact assembles");
    app.launch_runtime(WorthUiRuntimeLaunch::from_canonical_artifact(artifact))
        .expect("certification runtime launches")
}
