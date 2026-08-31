#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiMountedBackdropIdentity(Box<str>);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiOverlayPlacementReceipt {
    overlay_revision: u64,
    ordinal: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMountedBackdropMechanic {
    identity: UiMountedBackdropIdentity,
    semantic_surface: crate::UiSemanticSurfaceIdentity,
    placement: UiOverlayPlacementReceipt,
    bounds: super::UiAppearanceDamageRegion,
    clip: super::UiAppearanceClip,
    background: super::UiMountedAppearanceColor,
    opacity: super::UiMountedAppearanceOpacity,
    projection: super::UiAppearanceProjectionAttribution,
}

#[doc(hidden)]
pub struct UiMountedBackdropCompletionInput {
    pub issuer: crate::UiMountedNodeReceiptIssuer,
    pub identity: UiMountedBackdropIdentity,
    pub semantic_surface: crate::UiSemanticSurfaceIdentity,
    pub placement: UiOverlayPlacementReceipt,
    pub bounds: super::UiAppearanceDamageRegion,
    pub clip: super::UiAppearanceClip,
    pub background: super::UiMountedAppearanceColor,
    pub opacity: super::UiMountedAppearanceOpacity,
    pub projection: super::UiAppearanceProjectionAttribution,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMountedBackdropCompletionDenial {
    ProjectionIssuerMismatch,
}

impl UiMountedBackdropMechanic {
    #[doc(hidden)]
    pub fn complete_from_runtime_mounting(
        input: UiMountedBackdropCompletionInput,
    ) -> Result<Self, UiMountedBackdropCompletionDenial> {
        if !input.projection.matches_issuer(input.issuer) {
            return Err(UiMountedBackdropCompletionDenial::ProjectionIssuerMismatch);
        }
        Ok(Self {
            identity: input.identity,
            semantic_surface: input.semantic_surface,
            placement: input.placement,
            bounds: input.bounds,
            clip: input.clip,
            background: input.background,
            opacity: input.opacity,
            projection: input.projection,
        })
    }

    pub const fn identity(&self) -> &UiMountedBackdropIdentity {
        &self.identity
    }
    pub const fn semantic_surface(&self) -> crate::UiSemanticSurfaceIdentity {
        self.semantic_surface
    }
    pub const fn placement(&self) -> UiOverlayPlacementReceipt {
        self.placement
    }
    pub const fn bounds(&self) -> super::UiAppearanceDamageRegion {
        self.bounds
    }
    pub const fn clip(&self) -> super::UiAppearanceClip {
        self.clip
    }
    pub const fn background(&self) -> super::UiMountedAppearanceColor {
        self.background
    }
    pub const fn opacity(&self) -> super::UiMountedAppearanceOpacity {
        self.opacity
    }
    pub const fn projection(&self) -> super::UiAppearanceProjectionAttribution {
        self.projection
    }
    pub const fn participates_in_hit_testing(&self) -> bool {
        false
    }
}

impl UiMountedBackdropIdentity {
    #[doc(hidden)]
    pub fn from_runtime_mounting(value: impl Into<Box<str>>) -> Option<Self> {
        let value = value.into();
        (!value.is_empty()).then_some(Self(value))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl UiOverlayPlacementReceipt {
    #[doc(hidden)]
    pub const fn from_runtime_overlay_order(overlay_revision: u64, ordinal: u32) -> Option<Self> {
        if overlay_revision == 0 {
            None
        } else {
            Some(Self {
                overlay_revision,
                ordinal,
            })
        }
    }
    pub const fn overlay_revision(self) -> u64 {
        self.overlay_revision
    }
    pub const fn ordinal(self) -> u32 {
        self.ordinal
    }
}
