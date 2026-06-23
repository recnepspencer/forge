#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthUiAppearanceFamily {
    Typography,
    Layout,
    Border,
    Elevation,
    Spacing,
    Color,
}

impl WorthUiAppearanceFamily {
    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::Typography => "typography",
            Self::Layout => "layout",
            Self::Border => "border",
            Self::Elevation => "elevation",
            Self::Spacing => "spacing",
            Self::Color => "color",
        }
    }
}
