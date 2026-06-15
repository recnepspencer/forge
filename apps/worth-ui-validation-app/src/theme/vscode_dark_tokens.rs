use egui::{Color32, Visuals};
use worth_ui::facade::{ThemeTokenDescriptor, ThemeTokenValue};
use worth_ui_harness::facade::HarnessVisualTokenRole;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationWorkbenchTheme {
    editor_canvas: Color32,
    activity_bar: Color32,
    sidebar: Color32,
    panel: Color32,
    panel_raised: Color32,
    border_subtle: Color32,
    text_primary: Color32,
    text_muted: Color32,
    accent: Color32,
}

impl ValidationWorkbenchTheme {
    pub fn from_theme_tokens(
        descriptors: &[ThemeTokenDescriptor],
    ) -> Result<Self, ValidationWorkbenchThemeError> {
        Ok(Self {
            editor_canvas: color_for_role(descriptors, HarnessVisualTokenRole::EditorCanvas)?,
            activity_bar: color_for_role(descriptors, HarnessVisualTokenRole::ActivityBar)?,
            sidebar: color_for_role(descriptors, HarnessVisualTokenRole::Sidebar)?,
            panel: color_for_role(descriptors, HarnessVisualTokenRole::Panel)?,
            panel_raised: color_for_role(descriptors, HarnessVisualTokenRole::PanelRaised)?,
            border_subtle: color_for_role(descriptors, HarnessVisualTokenRole::BorderSubtle)?,
            text_primary: color_for_role(descriptors, HarnessVisualTokenRole::TextPrimary)?,
            text_muted: color_for_role(descriptors, HarnessVisualTokenRole::TextMuted)?,
            accent: color_for_role(descriptors, HarnessVisualTokenRole::Accent)?,
        })
    }

    pub fn editor_canvas(&self) -> Color32 {
        self.editor_canvas
    }

    pub fn activity_bar(&self) -> Color32 {
        self.activity_bar
    }

    pub fn sidebar(&self) -> Color32 {
        self.sidebar
    }

    pub fn panel(&self) -> Color32 {
        self.panel
    }

    pub fn panel_raised(&self) -> Color32 {
        self.panel_raised
    }

    pub fn border_subtle(&self) -> Color32 {
        self.border_subtle
    }

    pub fn text_primary(&self) -> Color32 {
        self.text_primary
    }

    pub fn text_muted(&self) -> Color32 {
        self.text_muted
    }

    pub fn accent(&self) -> Color32 {
        self.accent
    }

    pub fn visuals(&self) -> Visuals {
        let mut visuals = Visuals::dark();
        visuals.panel_fill = self.panel;
        visuals.window_fill = self.panel_raised;
        visuals.widgets.noninteractive.fg_stroke.color = self.text_primary;
        visuals.widgets.inactive.fg_stroke.color = self.text_primary;
        visuals.widgets.hovered.bg_fill = self.accent;
        visuals.widgets.active.bg_fill = self.accent;
        visuals
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationWorkbenchThemeError {
    MissingRole(HarnessVisualTokenRole),
    NonColorRole(HarnessVisualTokenRole),
    InvalidColor(HarnessVisualTokenRole),
}

fn color_for_role(
    descriptors: &[ThemeTokenDescriptor],
    role: HarnessVisualTokenRole,
) -> Result<Color32, ValidationWorkbenchThemeError> {
    let descriptor = descriptors
        .iter()
        .find(|descriptor| descriptor.id().as_str() == role.token_id_text())
        .ok_or(ValidationWorkbenchThemeError::MissingRole(role))?;
    let Some(ThemeTokenValue::Color(color)) = descriptor.value() else {
        return Err(ValidationWorkbenchThemeError::NonColorRole(role));
    };
    parse_hex_color(role, color.as_str())
}

fn parse_hex_color(
    role: HarnessVisualTokenRole,
    value: &str,
) -> Result<Color32, ValidationWorkbenchThemeError> {
    let hex = value
        .strip_prefix('#')
        .ok_or(ValidationWorkbenchThemeError::InvalidColor(role))?;
    let rgba = u32::from_str_radix(hex, 16)
        .map_err(|_| ValidationWorkbenchThemeError::InvalidColor(role))?;
    match hex.len() {
        6 => Ok(Color32::from_rgb(
            ((rgba >> 16) & 0xFF) as u8,
            ((rgba >> 8) & 0xFF) as u8,
            (rgba & 0xFF) as u8,
        )),
        8 => Ok(Color32::from_rgba_premultiplied(
            ((rgba >> 24) & 0xFF) as u8,
            ((rgba >> 16) & 0xFF) as u8,
            ((rgba >> 8) & 0xFF) as u8,
            (rgba & 0xFF) as u8,
        )),
        _ => Err(ValidationWorkbenchThemeError::InvalidColor(role)),
    }
}
