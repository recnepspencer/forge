#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiMountedSurfacePaint {
    Fill(super::UiMountedAppearanceColor),
    Border {
        color: super::UiMountedAppearanceColor,
        inward_width: u32,
    },
    FillAndBorder {
        fill: super::UiMountedAppearanceColor,
        border: super::UiMountedAppearanceColor,
        inward_width: u32,
    },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UiAppearanceProjectionAttribution {
    pub(super) frame: crate::UiMountedFrameIdentity,
    pub(super) issuer_nonce: u64,
    identity: u64,
    revision: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMountedSurfaceAppearanceMechanic {
    node_receipt: crate::UiMountedNodeReceiptIdentity,
    bounds: super::UiAppearanceDamageRegion,
    clip: super::UiAppearanceClip,
    layer: crate::UiMountedLayerProjection,
    visual_bounds: super::UiAppearanceDamageRegion,
    radii: super::UiAppearancePhysicalRadii,
    paint: UiMountedSurfacePaint,
    opacity: super::UiMountedAppearanceOpacity,
    projection: UiAppearanceProjectionAttribution,
}

#[doc(hidden)]
pub struct UiMountedSurfaceAppearanceCompletionInput {
    pub issuer: crate::UiMountedNodeReceiptIssuer,
    pub node_receipt: crate::UiMountedNodeReceiptIdentity,
    pub bounds: super::UiAppearanceDamageRegion,
    pub clip: super::UiAppearanceClip,
    pub layer: crate::UiMountedLayerProjection,
    pub visual_bounds: super::UiAppearanceDamageRegion,
    pub radii: super::UiAppearancePhysicalRadii,
    pub paint: UiMountedSurfacePaint,
    pub opacity: super::UiMountedAppearanceOpacity,
    pub projection: UiAppearanceProjectionAttribution,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMountedSurfaceAppearanceCompletionDenial {
    NodeReceiptFrameMismatch,
    ProjectionIssuerMismatch,
    BorderWidthExceedsHalfMinimumDimension,
}

impl UiMountedSurfaceAppearanceMechanic {
    #[doc(hidden)]
    pub fn complete_from_runtime_mounting(
        input: UiMountedSurfaceAppearanceCompletionInput,
    ) -> Result<Self, UiMountedSurfaceAppearanceCompletionDenial> {
        if input.node_receipt.frame() != input.issuer.frame_identity() {
            return Err(UiMountedSurfaceAppearanceCompletionDenial::NodeReceiptFrameMismatch);
        }
        if !input.projection.matches_issuer(input.issuer) {
            return Err(UiMountedSurfaceAppearanceCompletionDenial::ProjectionIssuerMismatch);
        }
        let inward_width = match &input.paint {
            UiMountedSurfacePaint::Fill(_) => 0,
            UiMountedSurfacePaint::Border { inward_width, .. }
            | UiMountedSurfacePaint::FillAndBorder { inward_width, .. } => *inward_width,
        };
        if inward_width > input.bounds.width().min(input.bounds.height()) / 2 {
            return Err(
                UiMountedSurfaceAppearanceCompletionDenial::BorderWidthExceedsHalfMinimumDimension,
            );
        }
        Ok(Self {
            node_receipt: input.node_receipt,
            bounds: input.bounds,
            clip: input.clip,
            layer: input.layer,
            visual_bounds: input.visual_bounds,
            radii: input.radii,
            paint: input.paint,
            opacity: input.opacity,
            projection: input.projection,
        })
    }

    pub const fn node_receipt(&self) -> crate::UiMountedNodeReceiptIdentity {
        self.node_receipt
    }
    pub const fn bounds(&self) -> super::UiAppearanceDamageRegion {
        self.bounds
    }
    pub const fn clip(&self) -> super::UiAppearanceClip {
        self.clip
    }
    pub const fn layer(&self) -> crate::UiMountedLayerProjection {
        self.layer
    }
    pub const fn visual_bounds(&self) -> super::UiAppearanceDamageRegion {
        self.visual_bounds
    }
    pub const fn radii(&self) -> super::UiAppearancePhysicalRadii {
        self.radii
    }
    pub const fn paint(&self) -> &UiMountedSurfacePaint {
        &self.paint
    }
    pub const fn opacity(&self) -> super::UiMountedAppearanceOpacity {
        self.opacity
    }
    pub const fn projection(&self) -> UiAppearanceProjectionAttribution {
        self.projection
    }
}

impl UiAppearanceProjectionAttribution {
    #[doc(hidden)]
    pub const fn from_runtime_mounting(
        issuer: crate::UiMountedNodeReceiptIssuer,
        identity: u64,
        revision: u64,
    ) -> Option<Self> {
        if identity == 0 || revision == 0 {
            None
        } else {
            Some(Self {
                frame: issuer.frame_identity(),
                issuer_nonce: issuer.issuer_nonce(),
                identity,
                revision,
            })
        }
    }
    pub(super) fn matches_issuer(self, issuer: crate::UiMountedNodeReceiptIssuer) -> bool {
        self.frame == issuer.frame_identity() && self.issuer_nonce == issuer.issuer_nonce()
    }
    pub const fn identity(self) -> u64 {
        self.identity
    }
    pub const fn revision(self) -> u64 {
        self.revision
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input(
        width: u32,
        height: u32,
        inward_width: u32,
    ) -> UiMountedSurfaceAppearanceCompletionInput {
        let frame = crate::UiMountedFrameIdentity::mint_unbound().unwrap();
        let issuer = crate::UiMountedNodeReceiptIssuer::mint_for(frame).unwrap();
        let instance = crate::UiMountedInstanceIdentity::mint_unbound().unwrap();
        let bounds = super::super::UiAppearanceDamageRegion::new(0, 0, width, height).unwrap();
        UiMountedSurfaceAppearanceCompletionInput {
            issuer,
            node_receipt: issuer.receipt_for(instance),
            bounds,
            clip: super::super::UiAppearanceClip::new(bounds),
            layer: crate::UiMountedLayerProjection::Layer(crate::UiMountedLayerReference::new(0)),
            visual_bounds: bounds,
            radii: super::super::UiAppearancePhysicalRadii::normalize(bounds, [0; 4]),
            paint: UiMountedSurfacePaint::Border {
                color: super::super::UiMountedAppearanceColor::from_straight_srgba([0; 4]),
                inward_width,
            },
            opacity: super::super::UiMountedAppearanceOpacity::ONE,
            projection: UiAppearanceProjectionAttribution::from_runtime_mounting(issuer, 1, 1)
                .unwrap(),
        }
    }

    #[test]
    fn surface_completion_denies_border_wider_than_half_minimum_dimension() {
        assert!(
            UiMountedSurfaceAppearanceMechanic::complete_from_runtime_mounting(input(5, 9, 2))
                .is_ok()
        );
        assert_eq!(
            UiMountedSurfaceAppearanceMechanic::complete_from_runtime_mounting(input(5, 9, 3)),
            Err(UiMountedSurfaceAppearanceCompletionDenial::BorderWidthExceedsHalfMinimumDimension)
        );
    }
}
