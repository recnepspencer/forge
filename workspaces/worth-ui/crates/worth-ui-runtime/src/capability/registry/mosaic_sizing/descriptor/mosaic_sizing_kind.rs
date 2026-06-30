#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MosaicSizingKind {
    Fixed,
    Fill,
    Ratio,
    Bounded,
    Hug,
    MinMax,
    GrowThenScroll,
    ContentMeasured,
    PersistedUserSize,
    ViewportRelative,
}

impl MosaicSizingKind {
    pub fn fixed() -> Self {
        Self::Fixed
    }

    pub fn fill() -> Self {
        Self::Fill
    }

    pub fn ratio() -> Self {
        Self::Ratio
    }

    pub fn bounded() -> Self {
        Self::Bounded
    }

    pub fn hug() -> Self {
        Self::Hug
    }

    pub fn min_max() -> Self {
        Self::MinMax
    }

    pub fn grow_then_scroll() -> Self {
        Self::GrowThenScroll
    }

    pub fn content_measured() -> Self {
        Self::ContentMeasured
    }

    pub fn persisted_user_size() -> Self {
        Self::PersistedUserSize
    }

    pub fn viewport_relative() -> Self {
        Self::ViewportRelative
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::Fixed => "fixed",
            Self::Fill => "fill",
            Self::Ratio => "ratio",
            Self::Bounded => "bounded",
            Self::Hug => "hug",
            Self::MinMax => "min_max",
            Self::GrowThenScroll => "grow_then_scroll",
            Self::ContentMeasured => "content_measured",
            Self::PersistedUserSize => "persisted_user_size",
            Self::ViewportRelative => "viewport_relative",
        }
    }
}
