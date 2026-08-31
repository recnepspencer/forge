#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMountedTextForegroundAppearanceMechanic {
    paint_span: crate::UiMountedTextPaintSpanIdentity,
    foreground: super::UiMountedAppearanceColor,
    opacity: super::UiMountedAppearanceOpacity,
    projection: super::UiAppearanceProjectionAttribution,
}

#[doc(hidden)]
pub struct UiMountedTextForegroundAppearanceCompletionInput {
    pub issuer: crate::UiMountedNodeReceiptIssuer,
    pub paint_span: crate::UiMountedTextPaintSpanIdentity,
    pub foreground: super::UiMountedAppearanceColor,
    pub opacity: super::UiMountedAppearanceOpacity,
    pub projection: super::UiAppearanceProjectionAttribution,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMountedTextForegroundAppearanceCompletionDenial {
    ProjectionIssuerMismatch,
}

impl UiMountedTextForegroundAppearanceMechanic {
    #[doc(hidden)]
    pub fn complete_from_runtime_mounting(
        input: UiMountedTextForegroundAppearanceCompletionInput,
    ) -> Result<Self, UiMountedTextForegroundAppearanceCompletionDenial> {
        if !input.projection.matches_issuer(input.issuer) {
            return Err(
                UiMountedTextForegroundAppearanceCompletionDenial::ProjectionIssuerMismatch,
            );
        }
        Ok(Self {
            paint_span: input.paint_span,
            foreground: input.foreground,
            opacity: input.opacity,
            projection: input.projection,
        })
    }
    pub const fn paint_span(&self) -> crate::UiMountedTextPaintSpanIdentity {
        self.paint_span
    }
    pub const fn foreground(&self) -> super::UiMountedAppearanceColor {
        self.foreground
    }
    pub const fn opacity(&self) -> super::UiMountedAppearanceOpacity {
        self.opacity
    }
    pub const fn projection(&self) -> super::UiAppearanceProjectionAttribution {
        self.projection
    }
}
