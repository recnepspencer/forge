use worth_ui::facade::{
    ThemeColorValue, ThemeTokenDescriptor, ThemeTokenId, ThemeTokenSource, ThemeTokenValue,
};

use super::{HarnessThemeTokenBinding, HarnessThemeTokenCatalog, HarnessVisualTokenRole};

pub fn vscode_like_dark_theme_catalog() -> HarnessThemeTokenCatalog {
    let bindings = HarnessVisualTokenRole::REQUIRED
        .into_iter()
        .map(|role| HarnessThemeTokenBinding::new(role, token_id(role.token_id_text())))
        .collect::<Vec<_>>();
    let descriptors = HarnessVisualTokenRole::REQUIRED
        .into_iter()
        .map(|role| theme_token(role, color_for_role(role)))
        .collect::<Vec<_>>();
    HarnessThemeTokenCatalog::new(bindings, descriptors)
}

fn theme_token(role: HarnessVisualTokenRole, hex: &'static str) -> ThemeTokenDescriptor {
    ThemeTokenDescriptor::define(
        token_id(role.token_id_text()),
        role.theme_family(),
        ThemeTokenSource::application(),
        ThemeTokenValue::color(ThemeColorValue::hex(hex).expect("valid harness color token")),
    )
}

fn token_id(raw_text: &str) -> ThemeTokenId {
    ThemeTokenId::new(raw_text).expect("valid harness theme token id")
}

fn color_for_role(role: HarnessVisualTokenRole) -> &'static str {
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
