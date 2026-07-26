use crate::capability::CapabilitySnapshot;
use crate::source::{WorthUiArtifactInputResolver, WorthUiResolvedArtifactInput};
use worth_ui_dsl::{
    WorthUiArtifactInputBodyAtom, WorthUiRustAuthoredArtifactInput,
    WorthUiRustAuthoredArtifactInputModule,
};

pub(super) fn resolved_artifact_input_from_modules<const N: usize>(
    modules: [WorthUiRustAuthoredArtifactInputModule; N],
    snapshot: &CapabilitySnapshot,
) -> WorthUiResolvedArtifactInput {
    let artifact_input = crate::source::test_compilation::compile_rust_authored(
        &WorthUiRustAuthoredArtifactInput::from_modules(modules),
    );
    WorthUiArtifactInputResolver::resolve(&artifact_input, snapshot).expect("phase 4 resolution")
}

pub(super) fn standard_component_module() -> WorthUiRustAuthoredArtifactInputModule {
    WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
        .with_component_body_atoms("workspace.component.dashboard", standard_body_atoms())
}

pub(super) fn standard_body_atoms() -> Vec<WorthUiArtifactInputBodyAtom> {
    vec![
        ident("region"),
        ident("workspace.region.primary"),
        WorthUiArtifactInputBodyAtom::LeftBrace,
        ident("sizing"),
        ident("workspace.sizing.fill"),
        WorthUiArtifactInputBodyAtom::Semicolon,
        ident("state"),
        ident("workspace.state.region_scroll"),
        WorthUiArtifactInputBodyAtom::Semicolon,
        ident("mount"),
        ident("workspace.surface.main"),
        ident("placement"),
        ident("workspace.placement.primary"),
        ident("state"),
        ident("workspace.state.primary_surface"),
        WorthUiArtifactInputBodyAtom::Semicolon,
        WorthUiArtifactInputBodyAtom::RightBrace,
        ident("region"),
        ident("workspace.region.overlay"),
        WorthUiArtifactInputBodyAtom::LeftBrace,
        ident("sizing"),
        ident("workspace.sizing.overlay"),
        WorthUiArtifactInputBodyAtom::Semicolon,
        ident("state"),
        ident("workspace.state.overlay_pinned"),
        WorthUiArtifactInputBodyAtom::Semicolon,
        ident("mount"),
        ident("workspace.surface.overlay"),
        ident("placement"),
        ident("workspace.placement.overlay"),
        WorthUiArtifactInputBodyAtom::Semicolon,
        WorthUiArtifactInputBodyAtom::RightBrace,
    ]
}

pub(super) fn invalid_sizing_body_atoms() -> Vec<WorthUiArtifactInputBodyAtom> {
    vec![
        ident("region"),
        ident("workspace.region.primary"),
        WorthUiArtifactInputBodyAtom::LeftBrace,
        ident("sizing"),
        ident("workspace.sizing.overlay"),
        WorthUiArtifactInputBodyAtom::Semicolon,
        ident("state"),
        ident("workspace.state.primary_pinned"),
        WorthUiArtifactInputBodyAtom::Semicolon,
        WorthUiArtifactInputBodyAtom::RightBrace,
    ]
}

pub(super) fn illegal_region_child_mix_body_atoms() -> Vec<WorthUiArtifactInputBodyAtom> {
    vec![
        ident("region"),
        ident("workspace.region.primary"),
        WorthUiArtifactInputBodyAtom::LeftBrace,
        ident("region"),
        ident("workspace.region.overlay"),
        WorthUiArtifactInputBodyAtom::LeftBrace,
        ident("sizing"),
        ident("workspace.sizing.overlay"),
        WorthUiArtifactInputBodyAtom::Semicolon,
        ident("state"),
        ident("workspace.state.overlay_pinned"),
        WorthUiArtifactInputBodyAtom::Semicolon,
        ident("mount"),
        ident("workspace.surface.overlay"),
        ident("placement"),
        ident("workspace.placement.overlay"),
        WorthUiArtifactInputBodyAtom::Semicolon,
        WorthUiArtifactInputBodyAtom::RightBrace,
        WorthUiArtifactInputBodyAtom::RightBrace,
    ]
}

fn ident(text: &str) -> WorthUiArtifactInputBodyAtom {
    WorthUiArtifactInputBodyAtom::Identifier(text.to_owned())
}
