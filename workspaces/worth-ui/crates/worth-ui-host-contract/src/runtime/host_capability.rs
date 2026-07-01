#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthUiHostCapability {
    TextInput,
    Ime,
    Accessibility,
    FontMetrics,
    VisualCapture,
}

impl WorthUiHostCapability {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TextInput => "text-input",
            Self::Ime => "ime",
            Self::Accessibility => "accessibility",
            Self::FontMetrics => "font-metrics",
            Self::VisualCapture => "visual-capture",
        }
    }
}
