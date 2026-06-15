use worth_ui::facade::{ThemeTokenFamily, ThemeTokenSource, ThemeTokenValue, WorthUi};
use worth_ui_harness::facade::{
    HarnessRuntimeOutcomeVisualRole, HarnessVisualFoundationBundle,
    HarnessVisualFoundationRegistration, HarnessVisualTokenRole,
};

#[test]
fn vscode_like_dark_theme_token_palette_is_complete_and_inspectable() {
    let prepared = HarnessVisualFoundationBundle::vscode_like_dark()
        .prepare()
        .expect("default visual foundation should prepare");
    let receipt = prepared.receipt().clone();
    let app = WorthUi::app()
        .install_harness_visual_foundation(prepared)
        .freeze();

    for role in HarnessVisualTokenRole::REQUIRED {
        assert!(
            receipt.covers_token_role(role),
            "missing token role {role:?}"
        );
        let token = app
            .capabilities()
            .theme_tokens()
            .get(&worth_ui::facade::ThemeTokenId::new(role.token_id_text()).unwrap())
            .unwrap_or_else(|| panic!("role {role:?} was not registered"));
        assert_eq!(token.family(), &expected_theme_family(role));
        assert_eq!(token.source(), &ThemeTokenSource::application());
        assert_eq!(theme_color(token), expected_theme_color(role));
    }

    assert_eq!(
        app.capabilities().theme_tokens().len(),
        HarnessVisualTokenRole::REQUIRED.len()
    );
}

#[test]
fn harness_theme_tokens_cover_focus_selection_overlay_and_runtime_states() {
    let prepared = HarnessVisualFoundationBundle::vscode_like_dark()
        .prepare()
        .expect("default visual foundation should prepare");
    let receipt = prepared.receipt();

    for role in [
        HarnessVisualTokenRole::FocusRing,
        HarnessVisualTokenRole::Selection,
        HarnessVisualTokenRole::OverlayElevated,
        HarnessVisualTokenRole::OverlayScrim,
        HarnessVisualTokenRole::RuntimeSuccess,
        HarnessVisualTokenRole::RuntimeWarning,
        HarnessVisualTokenRole::RuntimeDanger,
        HarnessVisualTokenRole::RuntimeDisabled,
        HarnessVisualTokenRole::RuntimeActive,
    ] {
        assert!(
            receipt.covers_token_role(role),
            "missing token role {role:?}"
        );
    }

    for role in HarnessRuntimeOutcomeVisualRole::REQUIRED {
        assert!(
            receipt.covers_runtime_outcome_role(role),
            "missing runtime outcome role {role:?}"
        );
    }
}

fn theme_color(descriptor: &worth_ui::facade::ThemeTokenDescriptor) -> &str {
    match descriptor.value().expect("harness theme token value") {
        ThemeTokenValue::Color(color) => color.as_str(),
    }
}

fn expected_theme_family(role: HarnessVisualTokenRole) -> ThemeTokenFamily {
    match role {
        HarnessVisualTokenRole::EditorCanvas
        | HarnessVisualTokenRole::ActivityBar
        | HarnessVisualTokenRole::Sidebar
        | HarnessVisualTokenRole::Panel => ThemeTokenFamily::surface(),
        HarnessVisualTokenRole::PanelRaised => ThemeTokenFamily::elevated_surface(),
        HarnessVisualTokenRole::OverlayElevated | HarnessVisualTokenRole::OverlayScrim => {
            ThemeTokenFamily::overlay()
        }
        HarnessVisualTokenRole::BorderSubtle => ThemeTokenFamily::border(),
        HarnessVisualTokenRole::TextPrimary => ThemeTokenFamily::text(),
        HarnessVisualTokenRole::TextMuted => ThemeTokenFamily::muted_text(),
        HarnessVisualTokenRole::Accent | HarnessVisualTokenRole::CommandHighlight => {
            ThemeTokenFamily::accent()
        }
        HarnessVisualTokenRole::FocusRing => ThemeTokenFamily::focus(),
        HarnessVisualTokenRole::Selection => ThemeTokenFamily::selection(),
        HarnessVisualTokenRole::RuntimeSuccess => ThemeTokenFamily::success(),
        HarnessVisualTokenRole::RuntimeWarning => ThemeTokenFamily::warning(),
        HarnessVisualTokenRole::RuntimeDanger => ThemeTokenFamily::danger(),
        HarnessVisualTokenRole::RuntimeDisabled => ThemeTokenFamily::disabled(),
        HarnessVisualTokenRole::RuntimeActive => ThemeTokenFamily::runtime_state(),
        HarnessVisualTokenRole::DiagnosticInfo => ThemeTokenFamily::advisory(),
    }
}

fn expected_theme_color(role: HarnessVisualTokenRole) -> &'static str {
    match role {
        HarnessVisualTokenRole::EditorCanvas => "#1E1E1E",
        HarnessVisualTokenRole::ActivityBar => "#181818",
        HarnessVisualTokenRole::Sidebar => "#252526",
        HarnessVisualTokenRole::Panel => "#2D2D30",
        HarnessVisualTokenRole::PanelRaised => "#333337",
        HarnessVisualTokenRole::OverlayElevated => "#3C3C3C",
        HarnessVisualTokenRole::OverlayScrim => "#00000099",
        HarnessVisualTokenRole::BorderSubtle => "#3C3C3C",
        HarnessVisualTokenRole::TextPrimary => "#CCCCCC",
        HarnessVisualTokenRole::TextMuted => "#8C8C8C",
        HarnessVisualTokenRole::Accent => "#007ACC",
        HarnessVisualTokenRole::FocusRing => "#3794FF",
        HarnessVisualTokenRole::Selection => "#264F78",
        HarnessVisualTokenRole::CommandHighlight => "#094771",
        HarnessVisualTokenRole::RuntimeSuccess => "#89D185",
        HarnessVisualTokenRole::RuntimeWarning => "#CCA700",
        HarnessVisualTokenRole::RuntimeDanger => "#F48771",
        HarnessVisualTokenRole::RuntimeDisabled => "#6A6A6A",
        HarnessVisualTokenRole::RuntimeActive => "#3794FF",
        HarnessVisualTokenRole::DiagnosticInfo => "#75BEFF",
    }
}
