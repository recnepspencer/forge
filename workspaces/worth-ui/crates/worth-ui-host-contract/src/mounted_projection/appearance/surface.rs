#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiMountedSurfacePaint {
    Fill(super::UiMountedAppearanceColor),
    Border {
        color: super::UiMountedAppearanceColor,
        inward_width: super::UiAppearanceLogicalLength,
    },
    FillAndBorder {
        fill: super::UiMountedAppearanceColor,
        border: super::UiMountedAppearanceColor,
        inward_width: super::UiAppearanceLogicalLength,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMountedSurfaceAppearanceMechanic {
    node_receipt: crate::UiMountedNodeReceiptIdentity,
    bounds: super::UiAppearanceAllocationBounds,
    clip: super::UiAppearanceClip,
    layer: crate::UiMountedLayerProjection,
    visual_bounds: super::UiAppearanceVisualBounds,
    radii: super::UiAppearanceNormalizedLogicalRadii,
    paint: UiMountedSurfacePaint,
    opacity: super::UiMountedAppearanceOpacity,
    projection: super::UiMountedNodeAppearanceAttribution,
}

#[doc(hidden)]
pub struct UiMountedSurfaceAppearanceCompletionInput {
    pub issuer: crate::UiMountedNodeReceiptIssuer,
    pub node_receipt: crate::UiMountedNodeReceiptIdentity,
    pub bounds: super::UiAppearanceAllocationBounds,
    pub clip: super::UiAppearanceClip,
    pub layer: crate::UiMountedLayerProjection,
    pub radii: super::UiAppearanceNormalizedLogicalRadii,
    pub paint: UiMountedSurfacePaint,
    pub opacity: super::UiMountedAppearanceOpacity,
    pub projection: super::UiMountedNodeAppearanceAttribution,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMountedSurfaceAppearanceCompletionDenial {
    NodeReceiptFrameMismatch,
    ProjectionIssuerMismatch,
    RadiiAllocationMismatch,
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
        if !input.radii.matches_allocation(input.bounds) {
            return Err(UiMountedSurfaceAppearanceCompletionDenial::RadiiAllocationMismatch);
        }
        let inward_width = match &input.paint {
            UiMountedSurfacePaint::Fill(_) => super::UiAppearanceLogicalLength::ZERO,
            UiMountedSurfacePaint::Border { inward_width, .. }
            | UiMountedSurfacePaint::FillAndBorder { inward_width, .. } => *inward_width,
        };
        if inward_width.subpixels() > input.bounds.width().min(input.bounds.height()) / 2 {
            return Err(
                UiMountedSurfaceAppearanceCompletionDenial::BorderWidthExceedsHalfMinimumDimension,
            );
        }
        Ok(Self {
            node_receipt: input.node_receipt,
            bounds: input.bounds,
            clip: input.clip,
            layer: input.layer,
            visual_bounds: super::UiAppearanceVisualBounds::from_surface_allocation(input.bounds),
            radii: input.radii,
            paint: input.paint,
            opacity: input.opacity,
            projection: input.projection,
        })
    }

    pub const fn node_receipt(&self) -> crate::UiMountedNodeReceiptIdentity {
        self.node_receipt
    }
    pub const fn bounds(&self) -> super::UiAppearanceAllocationBounds {
        self.bounds
    }
    pub const fn clip(&self) -> super::UiAppearanceClip {
        self.clip
    }
    pub const fn layer(&self) -> crate::UiMountedLayerProjection {
        self.layer
    }
    pub const fn visual_bounds(&self) -> super::UiAppearanceVisualBounds {
        self.visual_bounds
    }
    pub const fn radii(&self) -> super::UiAppearanceNormalizedLogicalRadii {
        self.radii
    }
    pub const fn paint(&self) -> &UiMountedSurfacePaint {
        &self.paint
    }
    pub const fn opacity(&self) -> super::UiMountedAppearanceOpacity {
        self.opacity
    }
    pub const fn projection(&self) -> super::UiMountedNodeAppearanceAttribution {
        self.projection
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn length(value: i32) -> super::super::UiAppearanceLogicalLength {
        super::super::UiAppearanceLogicalLength::new(value).unwrap()
    }

    fn input(
        width: u32,
        height: u32,
        inward_width: i32,
    ) -> UiMountedSurfaceAppearanceCompletionInput {
        let frame = crate::UiMountedFrameIdentity::mint_unbound().unwrap();
        let issuer = crate::UiMountedNodeReceiptIssuer::mint_for(frame).unwrap();
        let instance = crate::UiMountedInstanceIdentity::mint_unbound().unwrap();
        let bounds = super::super::UiAppearanceAllocationBounds::new(0, 0, width, height).unwrap();
        UiMountedSurfaceAppearanceCompletionInput {
            issuer,
            node_receipt: issuer.receipt_for(instance),
            bounds,
            clip: super::super::UiAppearanceClip::new(0, 0, width, height).unwrap(),
            layer: crate::UiMountedLayerProjection::Layer(crate::UiMountedLayerReference::new(0)),
            radii: super::super::UiAppearanceNormalizedLogicalRadii::normalize(
                bounds,
                [super::super::UiAppearanceLogicalLength::ZERO; 4],
            ),
            paint: UiMountedSurfacePaint::Border {
                color: super::super::UiMountedAppearanceColor::from_straight_srgba([0; 4]),
                inward_width: length(inward_width),
            },
            opacity: super::super::UiMountedAppearanceOpacity::ONE,
            projection: super::super::UiMountedNodeAppearanceAttribution::from_runtime_mounting(
                issuer, 1, 1,
            )
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

    #[test]
    fn surface_visual_bounds_are_derived_from_allocation() {
        let mechanic =
            UiMountedSurfaceAppearanceMechanic::complete_from_runtime_mounting(input(5, 9, 0))
                .unwrap();
        assert_eq!(
            mechanic.visual_bounds(),
            super::super::UiAppearanceVisualBounds::from_surface_allocation(mechanic.bounds())
        );
    }

    #[test]
    fn surface_completion_denies_radii_normalized_for_another_allocation() {
        let mut mismatched = input(5, 9, 0);
        let other = super::super::UiAppearanceAllocationBounds::new(0, 0, 50, 90).unwrap();
        mismatched.radii = super::super::UiAppearanceNormalizedLogicalRadii::normalize(
            other,
            [super::super::UiAppearanceLogicalLength::ZERO; 4],
        );
        assert_eq!(
            UiMountedSurfaceAppearanceMechanic::complete_from_runtime_mounting(mismatched),
            Err(UiMountedSurfaceAppearanceCompletionDenial::RadiiAllocationMismatch)
        );
    }
}
