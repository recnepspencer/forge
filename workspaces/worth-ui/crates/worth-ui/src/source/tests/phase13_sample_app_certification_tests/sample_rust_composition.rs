use crate::source::{
    WorthUiArtifactInputBodyAtom, WorthUiRustCompositionInput, WorthUiRustCompositionModule,
};

pub(super) fn sample_rust_composition() -> WorthUiRustCompositionInput {
    WorthUiRustCompositionInput::from_modules(sample_rust_modules())
}

pub(super) fn reordered_sample_rust_composition() -> WorthUiRustCompositionInput {
    let [main, inspector, theme] = sample_rust_modules();
    WorthUiRustCompositionInput::from_modules([theme, inspector, main])
}

pub(super) fn rust_composition_with_duplicate_identity() -> WorthUiRustCompositionInput {
    WorthUiRustCompositionInput::from_modules([WorthUiRustCompositionModule::new("app/main.wui")
        .component_authored_identity("workspace.component.dashboard", "sample.duplicate")
        .component_authored_identity("workspace.component.inspector_panel", "sample.duplicate")])
}

fn sample_rust_modules() -> [WorthUiRustCompositionModule; 3] {
    [
        WorthUiRustCompositionModule::new("app/main.wui")
            .import("app/panels/inspector.wui")
            .import("app/theme/tokens.wui")
            .component_body_atoms("workspace.component.dashboard", dashboard_body_atoms())
            .surface("workspace.surface.main")
            .surface("workspace.surface.overlay")
            .surface("workspace.surface.inspector")
            .binding("workspace.view_binding.selection"),
        WorthUiRustCompositionModule::new("app/panels/inspector.wui")
            .component("workspace.component.inspector_panel"),
        WorthUiRustCompositionModule::new("app/theme/tokens.wui")
            .token("theme.text.default", "theme.text.primary"),
    ]
}

fn dashboard_body_atoms() -> Vec<WorthUiArtifactInputBodyAtom> {
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

fn ident(text: &str) -> WorthUiArtifactInputBodyAtom {
    WorthUiArtifactInputBodyAtom::Identifier(text.to_owned())
}
