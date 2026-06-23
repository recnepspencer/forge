#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiCapabilityReloadDenialCode {
    AppearanceShadow(WorthUiAppearanceShadowParseDenialCode),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiAppearanceShadowParseDenialCode {
    InvalidArity,
    InvalidColor,
    InvalidOffsetX,
    InvalidOffsetY,
    InvalidBlur,
    InvalidSpread,
}
