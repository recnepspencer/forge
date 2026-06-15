use worth_ui::facade::ThemeTokenFamily;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum HarnessVisualTokenRole {
    EditorCanvas,
    ActivityBar,
    Sidebar,
    Panel,
    PanelRaised,
    OverlayElevated,
    OverlayScrim,
    BorderSubtle,
    TextPrimary,
    TextMuted,
    Accent,
    FocusRing,
    Selection,
    CommandHighlight,
    RuntimeSuccess,
    RuntimeWarning,
    RuntimeDanger,
    RuntimeDisabled,
    RuntimeActive,
    DiagnosticInfo,
}

impl HarnessVisualTokenRole {
    pub const REQUIRED: [Self; 20] = [
        Self::EditorCanvas,
        Self::ActivityBar,
        Self::Sidebar,
        Self::Panel,
        Self::PanelRaised,
        Self::OverlayElevated,
        Self::OverlayScrim,
        Self::BorderSubtle,
        Self::TextPrimary,
        Self::TextMuted,
        Self::Accent,
        Self::FocusRing,
        Self::Selection,
        Self::CommandHighlight,
        Self::RuntimeSuccess,
        Self::RuntimeWarning,
        Self::RuntimeDanger,
        Self::RuntimeDisabled,
        Self::RuntimeActive,
        Self::DiagnosticInfo,
    ];

    pub fn token_id_text(self) -> &'static str {
        match self {
            Self::EditorCanvas => "harness.theme.editor.canvas",
            Self::ActivityBar => "harness.theme.activity_bar.background",
            Self::Sidebar => "harness.theme.sidebar.background",
            Self::Panel => "harness.theme.panel.background",
            Self::PanelRaised => "harness.theme.panel.raised",
            Self::OverlayElevated => "harness.theme.overlay.elevated",
            Self::OverlayScrim => "harness.theme.overlay.scrim",
            Self::BorderSubtle => "harness.theme.border.subtle",
            Self::TextPrimary => "harness.theme.text.primary",
            Self::TextMuted => "harness.theme.text.muted",
            Self::Accent => "harness.theme.accent.primary",
            Self::FocusRing => "harness.theme.focus.ring",
            Self::Selection => "harness.theme.selection.background",
            Self::CommandHighlight => "harness.theme.command.highlight",
            Self::RuntimeSuccess => "harness.theme.runtime.success",
            Self::RuntimeWarning => "harness.theme.runtime.warning",
            Self::RuntimeDanger => "harness.theme.runtime.danger",
            Self::RuntimeDisabled => "harness.theme.runtime.disabled",
            Self::RuntimeActive => "harness.theme.runtime.active",
            Self::DiagnosticInfo => "harness.theme.diagnostic.info",
        }
    }

    pub(crate) fn theme_family(self) -> ThemeTokenFamily {
        match self {
            Self::EditorCanvas | Self::ActivityBar | Self::Sidebar | Self::Panel => {
                ThemeTokenFamily::surface()
            }
            Self::PanelRaised => ThemeTokenFamily::elevated_surface(),
            Self::OverlayElevated | Self::OverlayScrim => ThemeTokenFamily::overlay(),
            Self::BorderSubtle => ThemeTokenFamily::border(),
            Self::TextPrimary => ThemeTokenFamily::text(),
            Self::TextMuted => ThemeTokenFamily::muted_text(),
            Self::Accent | Self::CommandHighlight => ThemeTokenFamily::accent(),
            Self::FocusRing => ThemeTokenFamily::focus(),
            Self::Selection => ThemeTokenFamily::selection(),
            Self::RuntimeSuccess => ThemeTokenFamily::success(),
            Self::RuntimeWarning => ThemeTokenFamily::warning(),
            Self::RuntimeDanger => ThemeTokenFamily::danger(),
            Self::RuntimeDisabled => ThemeTokenFamily::disabled(),
            Self::RuntimeActive => ThemeTokenFamily::runtime_state(),
            Self::DiagnosticInfo => ThemeTokenFamily::advisory(),
        }
    }
}
