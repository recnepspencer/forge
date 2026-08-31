#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMountedOutlineAppearanceMechanic {
    node_receipt: crate::UiMountedNodeReceiptIdentity,
    clip: super::UiAppearanceClip,
    visual_bounds: super::UiAppearanceDamageRegion,
    color: super::UiMountedAppearanceColor,
    width: u32,
    offset: u32,
    radii: super::UiAppearancePhysicalRadii,
    opacity: super::UiMountedAppearanceOpacity,
    projection: super::UiAppearanceProjectionAttribution,
}

#[doc(hidden)]
pub struct UiMountedOutlineAppearanceCompletionInput {
    pub issuer: crate::UiMountedNodeReceiptIssuer,
    pub node_receipt: crate::UiMountedNodeReceiptIdentity,
    pub clip: super::UiAppearanceClip,
    pub visual_bounds: super::UiAppearanceDamageRegion,
    pub color: super::UiMountedAppearanceColor,
    pub width: u32,
    pub offset: u32,
    pub radii: super::UiAppearancePhysicalRadii,
    pub opacity: super::UiMountedAppearanceOpacity,
    pub projection: super::UiAppearanceProjectionAttribution,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMountedOutlineAppearanceCompletionDenial {
    NodeReceiptFrameMismatch,
    ProjectionIssuerMismatch,
}

impl UiMountedOutlineAppearanceMechanic {
    #[doc(hidden)]
    pub fn complete_from_runtime_mounting(
        input: UiMountedOutlineAppearanceCompletionInput,
    ) -> Result<Self, UiMountedOutlineAppearanceCompletionDenial> {
        if input.node_receipt.frame() != input.issuer.frame_identity() {
            return Err(UiMountedOutlineAppearanceCompletionDenial::NodeReceiptFrameMismatch);
        }
        if !input.projection.matches_issuer(input.issuer) {
            return Err(UiMountedOutlineAppearanceCompletionDenial::ProjectionIssuerMismatch);
        }
        Ok(Self {
            node_receipt: input.node_receipt,
            clip: input.clip,
            visual_bounds: input.visual_bounds,
            color: input.color,
            width: input.width,
            offset: input.offset,
            radii: input.radii,
            opacity: input.opacity,
            projection: input.projection,
        })
    }
    pub const fn node_receipt(&self) -> crate::UiMountedNodeReceiptIdentity {
        self.node_receipt
    }
    pub const fn clip(&self) -> super::UiAppearanceClip {
        self.clip
    }
    pub const fn visual_bounds(&self) -> super::UiAppearanceDamageRegion {
        self.visual_bounds
    }
    pub const fn color(&self) -> super::UiMountedAppearanceColor {
        self.color
    }
    pub const fn width(&self) -> u32 {
        self.width
    }
    pub const fn offset(&self) -> u32 {
        self.offset
    }
    pub const fn radii(&self) -> super::UiAppearancePhysicalRadii {
        self.radii
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
