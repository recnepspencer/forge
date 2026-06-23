use crate::capability::{
    ThemeColorValue, WorthUiBorderWidthValue, WorthUiCornerRadiusValue, WorthUiFontSizeValue,
    WorthUiLengthValue, WorthUiPaddingValue, WorthUiShadowValue, WorthUiSpacingValue,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiAppearanceValue {
    Color(ThemeColorValue),
    Length(WorthUiLengthValue),
    FontSize(WorthUiFontSizeValue),
    Padding(WorthUiPaddingValue),
    Spacing(WorthUiSpacingValue),
    BorderWidth(WorthUiBorderWidthValue),
    CornerRadius(WorthUiCornerRadiusValue),
    Shadow(WorthUiShadowValue),
}

impl WorthUiAppearanceValue {
    pub fn color(value: ThemeColorValue) -> Self {
        Self::Color(value)
    }

    pub fn length(value: WorthUiLengthValue) -> Self {
        Self::Length(value)
    }

    pub fn font_size(value: WorthUiFontSizeValue) -> Self {
        Self::FontSize(value)
    }

    pub fn padding(value: WorthUiPaddingValue) -> Self {
        Self::Padding(value)
    }

    pub fn spacing(value: WorthUiSpacingValue) -> Self {
        Self::Spacing(value)
    }

    pub fn border_width(value: WorthUiBorderWidthValue) -> Self {
        Self::BorderWidth(value)
    }

    pub fn corner_radius(value: WorthUiCornerRadiusValue) -> Self {
        Self::CornerRadius(value)
    }

    pub fn shadow(value: WorthUiShadowValue) -> Self {
        Self::Shadow(value)
    }

    pub(crate) fn digest_basis(&self) -> String {
        match self {
            Self::Color(value) => format!("color:{}", value.digest_basis()),
            Self::Length(value) => format!("length:{}", value.digest_basis()),
            Self::FontSize(value) => value.digest_basis(),
            Self::Padding(value) => value.digest_basis(),
            Self::Spacing(value) => value.digest_basis(),
            Self::BorderWidth(value) => value.digest_basis(),
            Self::CornerRadius(value) => value.digest_basis(),
            Self::Shadow(value) => value.digest_basis(),
        }
    }
}
