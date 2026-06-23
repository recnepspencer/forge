use crate::capability::{
    AppearanceTokenId, DensityTokenId, ThemeTokenId, ThemeTokenValue, WorthUiAppearanceValue,
    WorthUiDensityValue,
};
use crate::runtime::{
    WorthUiAppearanceStateTokenDenialReason, WorthUiPrimitiveColor, WorthUiRuntimeHost,
};

pub(super) fn resolve_theme_color(
    runtime: &WorthUiRuntimeHost,
    token_text: &str,
) -> Result<WorthUiPrimitiveColor, WorthUiAppearanceStateTokenDenialReason> {
    let token = ThemeTokenId::new(token_text)
        .map_err(|_| WorthUiAppearanceStateTokenDenialReason::InvalidTokenSyntax)?;
    let descriptor = runtime
        .inspect_active_theme_token_descriptor(&token)
        .ok_or(WorthUiAppearanceStateTokenDenialReason::MissingThemeToken)?;
    match descriptor
        .value()
        .ok_or(WorthUiAppearanceStateTokenDenialReason::MissingThemeToken)?
    {
        ThemeTokenValue::Color(color) => parse_theme_color(color.as_str())
            .ok_or(WorthUiAppearanceStateTokenDenialReason::WrongAppearanceTokenKind),
    }
}

pub(super) fn resolve_font_size_points(
    runtime: &WorthUiRuntimeHost,
    token_text: &str,
) -> Result<f32, WorthUiAppearanceStateTokenDenialReason> {
    let token = AppearanceTokenId::new(token_text)
        .map_err(|_| WorthUiAppearanceStateTokenDenialReason::InvalidTokenSyntax)?;
    let descriptor = runtime
        .inspect_active_appearance_token_descriptor(&token)
        .ok_or(WorthUiAppearanceStateTokenDenialReason::MissingAppearanceToken)?;
    match descriptor.value() {
        WorthUiAppearanceValue::FontSize(value) => Ok(value.points()),
        _ => Err(WorthUiAppearanceStateTokenDenialReason::WrongAppearanceTokenKind),
    }
}

pub(super) fn resolve_density_points(
    runtime: &WorthUiRuntimeHost,
    token_text: &str,
) -> Result<f32, WorthUiAppearanceStateTokenDenialReason> {
    let token = DensityTokenId::new(token_text)
        .map_err(|_| WorthUiAppearanceStateTokenDenialReason::InvalidTokenSyntax)?;
    let descriptor = runtime
        .inspect_active_density_token_descriptor(&token)
        .ok_or(WorthUiAppearanceStateTokenDenialReason::MissingDensityToken)?;
    match descriptor.value() {
        WorthUiDensityValue::Padding(value) => Ok(value.horizontal_points()),
        WorthUiDensityValue::Spacing(value) => Ok(value.points()),
        WorthUiDensityValue::HitTargetMinimum(value) => Ok(value.points()),
        WorthUiDensityValue::Posture(_) => {
            Err(WorthUiAppearanceStateTokenDenialReason::WrongDensityTokenKind)
        }
    }
}

fn parse_theme_color(value: &str) -> Option<WorthUiPrimitiveColor> {
    let hex = value.strip_prefix('#')?;
    if !matches!(hex.len(), 6 | 8) || !hex.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return None;
    }
    Some(WorthUiPrimitiveColor::new(
        u8::from_str_radix(&hex[0..2], 16).ok()?,
        u8::from_str_radix(&hex[2..4], 16).ok()?,
        u8::from_str_radix(&hex[4..6], 16).ok()?,
    ))
}
