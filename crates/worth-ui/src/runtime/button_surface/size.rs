#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiButtonSize {
    Small,
    Medium,
    Large,
}

impl WorthUiButtonSize {
    pub(super) fn default_width(self) -> f32 {
        match self {
            Self::Small => 96.0,
            Self::Medium => 132.0,
            Self::Large => 172.0,
        }
    }

    pub(super) fn default_height(self) -> f32 {
        match self {
            Self::Small => 32.0,
            Self::Medium => 40.0,
            Self::Large => 48.0,
        }
    }

    pub(super) fn padding_x(self) -> f32 {
        match self {
            Self::Small => 12.0,
            Self::Medium => 16.0,
            Self::Large => 22.0,
        }
    }

    pub(super) fn padding_y(self) -> f32 {
        match self {
            Self::Small => 6.0,
            Self::Medium => 9.0,
            Self::Large => 12.0,
        }
    }

    pub(super) fn text_size(self) -> f32 {
        match self {
            Self::Small => 13.0,
            Self::Medium => 14.0,
            Self::Large => 15.0,
        }
    }

    pub(super) fn icon_size(self) -> f32 {
        match self {
            Self::Small => 18.0,
            Self::Medium => 22.0,
            Self::Large => 28.0,
        }
    }

    pub(super) fn content_gap(self) -> f32 {
        match self {
            Self::Small => 4.0,
            Self::Medium => 5.0,
            Self::Large => 5.0,
        }
    }

    pub(super) fn icon_stroke_width(self) -> f32 {
        match self {
            Self::Small => 1.75,
            Self::Medium => 2.25,
            Self::Large => 2.75,
        }
    }
}
