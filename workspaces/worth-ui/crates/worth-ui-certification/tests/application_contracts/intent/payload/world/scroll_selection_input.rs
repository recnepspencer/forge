use worth_ui::facade::intent::{UiIntent, UiIntentDeclaration};
use worth_ui_dsl::{
    WorthUiArtifactInputBodyAtom, WorthUiIntentInteractionFamily, WorthUiIntentInteractionRoute,
    WorthUiRustAuthoredArtifactInput, WorthUiRustAuthoredArtifactInputModule,
};

pub(in crate::intent) fn routed_scroll_selection_input<I: UiIntent>(
    declaration: UiIntentDeclaration<I>,
    replacement: bool,
) -> WorthUiRustAuthoredArtifactInput {
    let mut scroll_target_body = vec![
        WorthUiArtifactInputBodyAtom::Identifier("interaction".to_owned()),
        WorthUiArtifactInputBodyAtom::Identifier("selection-commit".to_owned()),
        WorthUiArtifactInputBodyAtom::Identifier("routes".to_owned()),
        WorthUiArtifactInputBodyAtom::Identifier(super::DECLARATION.to_owned()),
    ];
    scroll_target_body.extend(scroll_region_body(true));
    let replacement_body = scroll_region_body(replacement);
    let module = WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
        .with_component(super::PAINT_ONLY)
        .with_control_routes(
            super::HIT_ONLY,
            [WorthUiIntentInteractionRoute::product(
                WorthUiIntentInteractionFamily::SelectionCommit,
                super::DECLARATION,
            )],
        )
        .with_component_body_atoms(super::PAINT_AND_HIT, scroll_target_body)
        .with_component_body_atoms_and_authored_identity(
            super::NEITHER,
            "phase315.selection.replacement_sentinel",
            replacement_body,
        )
        .with_surface(super::SURFACE)
        .with_token(super::PAINT_ONLY_TOKEN, "theme.visual_identity.red")
        .with_token(super::PAINT_AND_HIT_TOKEN, "theme.visual_identity.purple")
        .with_intent_declaration(super::bind_portal_operability(declaration));
    WorthUiRustAuthoredArtifactInput::from_modules([module])
}

fn scroll_region_body(enabled: bool) -> Vec<WorthUiArtifactInputBodyAtom> {
    if !enabled {
        return Vec::new();
    }
    vec![
        WorthUiArtifactInputBodyAtom::Identifier("region".to_owned()),
        WorthUiArtifactInputBodyAtom::Identifier(super::SCROLL_REGION.to_owned()),
        WorthUiArtifactInputBodyAtom::LeftBrace,
        WorthUiArtifactInputBodyAtom::Identifier("sizing".to_owned()),
        WorthUiArtifactInputBodyAtom::Identifier(super::SCROLL_SIZING.to_owned()),
        WorthUiArtifactInputBodyAtom::Semicolon,
        WorthUiArtifactInputBodyAtom::RightBrace,
    ]
}
