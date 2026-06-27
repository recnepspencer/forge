#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ThemeTokenFamily {
    Surface,
    ElevatedSurface,
    Text,
    MutedText,
    Border,
    Accent,
    Selection,
    Focus,
    Danger,
    Warning,
    Success,
    Advisory,
    Disabled,
    Overlay,
    Shadow,
    ChartSeries,
    RuntimeState,
    Unknown(String),
}

impl ThemeTokenFamily {
    pub fn surface() -> Self {
        Self::Surface
    }

    pub fn elevated_surface() -> Self {
        Self::ElevatedSurface
    }

    pub fn text() -> Self {
        Self::Text
    }

    pub fn muted_text() -> Self {
        Self::MutedText
    }

    pub fn border() -> Self {
        Self::Border
    }

    pub fn accent() -> Self {
        Self::Accent
    }

    pub fn selection() -> Self {
        Self::Selection
    }

    pub fn focus() -> Self {
        Self::Focus
    }

    pub fn danger() -> Self {
        Self::Danger
    }

    pub fn warning() -> Self {
        Self::Warning
    }

    pub fn success() -> Self {
        Self::Success
    }

    pub fn advisory() -> Self {
        Self::Advisory
    }

    pub fn disabled() -> Self {
        Self::Disabled
    }

    pub fn overlay() -> Self {
        Self::Overlay
    }

    pub fn shadow() -> Self {
        Self::Shadow
    }

    pub fn chart_series() -> Self {
        Self::ChartSeries
    }

    pub fn runtime_state() -> Self {
        Self::RuntimeState
    }

    pub fn unknown_for_diagnostics(name: impl Into<String>) -> Self {
        Self::Unknown(name.into())
    }

    pub(crate) fn is_known(&self) -> bool {
        !matches!(self, Self::Unknown(_))
    }

    pub(crate) fn digest_basis(&self) -> String {
        match self {
            Self::Surface => "surface".to_string(),
            Self::ElevatedSurface => "elevated_surface".to_string(),
            Self::Text => "text".to_string(),
            Self::MutedText => "muted_text".to_string(),
            Self::Border => "border".to_string(),
            Self::Accent => "accent".to_string(),
            Self::Selection => "selection".to_string(),
            Self::Focus => "focus".to_string(),
            Self::Danger => "danger".to_string(),
            Self::Warning => "warning".to_string(),
            Self::Success => "success".to_string(),
            Self::Advisory => "advisory".to_string(),
            Self::Disabled => "disabled".to_string(),
            Self::Overlay => "overlay".to_string(),
            Self::Shadow => "shadow".to_string(),
            Self::ChartSeries => "chart_series".to_string(),
            Self::RuntimeState => "runtime_state".to_string(),
            Self::Unknown(name) => format!("unknown:{}", length_prefixed(name)),
        }
    }
}

fn length_prefixed(value: &str) -> String {
    format!("{}:{value}", value.len())
}
