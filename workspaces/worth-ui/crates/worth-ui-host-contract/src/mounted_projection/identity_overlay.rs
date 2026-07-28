#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMountedClientPhysicalRect {
    left: u32,
    top: u32,
    right: u32,
    bottom: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMountedClientCoordinateBasis {
    client_physical_dimensions: [u32; 2],
    viewport_logical_dimension_bits: [u32; 2],
    scale_bits: [u32; 2],
    translation_bits: [u32; 2],
    orientation: crate::UiHostCoordinateOrientation,
    rounding: crate::UiHostCoordinateRounding,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMountedIdentityOverlayMechanic {
    overlay_identity: u64,
    base_snapshot: u64,
    base_frame: crate::UiMountedFrameIdentity,
    target_receipt: crate::UiMountedNodeReceiptIdentity,
    successor_frame: crate::UiMountedFrameIdentity,
    surface: crate::UiSemanticSurfaceIdentity,
    binding: crate::UiSurfaceBindingGeneration,
    coordinate_basis: UiMountedClientCoordinateBasis,
    target_region: UiMountedClientPhysicalRect,
    border_width: u8,
    color: super::UiMountedRgba8,
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMountedIdentityOverlayMechanicInput {
    pub overlay_identity: u64,
    pub base_snapshot: u64,
    pub base_frame: crate::UiMountedFrameIdentity,
    pub target_receipt: crate::UiMountedNodeReceiptIdentity,
    pub successor_frame: crate::UiMountedFrameIdentity,
    pub surface: crate::UiSemanticSurfaceIdentity,
    pub binding: crate::UiSurfaceBindingGeneration,
    pub coordinate_basis: UiMountedClientCoordinateBasis,
    pub target_region: UiMountedClientPhysicalRect,
}

impl UiMountedClientPhysicalRect {
    #[doc(hidden)]
    pub const fn from_runtime_mounting(
        left: u32,
        top: u32,
        right: u32,
        bottom: u32,
    ) -> Option<Self> {
        if left >= right || top >= bottom {
            return None;
        }
        Some(Self {
            left,
            top,
            right,
            bottom,
        })
    }

    pub const fn left(self) -> u32 {
        self.left
    }

    pub const fn top(self) -> u32 {
        self.top
    }

    pub const fn right(self) -> u32 {
        self.right
    }

    pub const fn bottom(self) -> u32 {
        self.bottom
    }
}

impl UiMountedClientCoordinateBasis {
    #[doc(hidden)]
    pub fn from_runtime_mounting(transform: crate::UiHostCoordinateTransform) -> Option<Self> {
        let physical = transform.client_physical_dimensions();
        let logical = transform.viewport_logical_dimensions();
        let scale = transform.scale();
        let translation = transform.translation();
        if physical.contains(&0)
            || logical
                .iter()
                .any(|value| !value.is_finite() || *value <= 0.0)
            || scale
                .iter()
                .any(|value| !value.is_finite() || *value <= 0.0)
            || translation.iter().any(|value| !value.is_finite())
            || physical_dimension(logical[0], scale[0])? != physical[0]
            || physical_dimension(logical[1], scale[1])? != physical[1]
        {
            return None;
        }
        Some(Self {
            client_physical_dimensions: physical,
            viewport_logical_dimension_bits: logical.map(f32::to_bits),
            scale_bits: scale.map(f32::to_bits),
            translation_bits: translation.map(f32::to_bits),
            orientation: transform.orientation(),
            rounding: transform.rounding(),
        })
    }

    pub const fn client_physical_dimensions(self) -> [u32; 2] {
        self.client_physical_dimensions
    }

    pub fn viewport_logical_dimensions(self) -> [f32; 2] {
        self.viewport_logical_dimension_bits.map(f32::from_bits)
    }

    pub fn scale(self) -> [f32; 2] {
        self.scale_bits.map(f32::from_bits)
    }

    pub fn translation(self) -> [f32; 2] {
        self.translation_bits.map(f32::from_bits)
    }

    pub const fn orientation(self) -> crate::UiHostCoordinateOrientation {
        self.orientation
    }

    pub const fn rounding(self) -> crate::UiHostCoordinateRounding {
        self.rounding
    }
}

impl UiMountedIdentityOverlayMechanic {
    pub const BORDER_WIDTH_PHYSICAL_PIXELS: u8 = 2;
    pub const COLOR: super::UiMountedRgba8 = super::UiMountedRgba8::new(255, 0, 255, 255);

    #[doc(hidden)]
    pub fn from_runtime_mounting(input: UiMountedIdentityOverlayMechanicInput) -> Option<Self> {
        (input.overlay_identity != 0
            && input.base_snapshot != 0
            && input.target_receipt.frame() == input.base_frame
            && input.successor_frame != input.base_frame)
            .then_some(Self {
                overlay_identity: input.overlay_identity,
                base_snapshot: input.base_snapshot,
                base_frame: input.base_frame,
                target_receipt: input.target_receipt,
                successor_frame: input.successor_frame,
                surface: input.surface,
                binding: input.binding,
                coordinate_basis: input.coordinate_basis,
                target_region: input.target_region,
                border_width: Self::BORDER_WIDTH_PHYSICAL_PIXELS,
                color: Self::COLOR,
            })
    }

    pub const fn overlay_identity(self) -> u64 {
        self.overlay_identity
    }

    pub const fn base_snapshot(self) -> u64 {
        self.base_snapshot
    }

    pub const fn base_frame(self) -> crate::UiMountedFrameIdentity {
        self.base_frame
    }

    pub const fn target_receipt(self) -> crate::UiMountedNodeReceiptIdentity {
        self.target_receipt
    }

    pub const fn successor_frame(self) -> crate::UiMountedFrameIdentity {
        self.successor_frame
    }

    pub const fn surface(self) -> crate::UiSemanticSurfaceIdentity {
        self.surface
    }

    pub const fn binding(self) -> crate::UiSurfaceBindingGeneration {
        self.binding
    }

    pub const fn coordinate_basis(self) -> UiMountedClientCoordinateBasis {
        self.coordinate_basis
    }

    pub const fn target_region(self) -> UiMountedClientPhysicalRect {
        self.target_region
    }

    pub const fn border_width(self) -> u8 {
        self.border_width
    }

    pub const fn color(self) -> super::UiMountedRgba8 {
        self.color
    }
}

fn physical_dimension(logical: f32, scale: f32) -> Option<u32> {
    let physical = logical * scale;
    (physical.is_finite() && physical > 0.0 && physical <= u32::MAX as f32)
        .then_some(physical.round() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mechanic_requires_distinct_successor_and_preserves_exact_base_affinity() {
        let base_frame = crate::UiMountedFrameIdentity::mint_unbound().unwrap();
        let successor_frame = crate::UiMountedFrameIdentity::mint_unbound().unwrap();
        let instance = crate::UiMountedInstanceIdentity::mint_unbound().unwrap();
        let target_receipt = crate::UiMountedNodeReceiptIssuer::mint_for(base_frame)
            .unwrap()
            .receipt_for(instance);
        let surface = crate::UiSemanticSurfaceIdentity::mint_unbound().unwrap();
        let binding = crate::UiSurfaceBindingGeneration::mint_unbound().unwrap();
        let target_region =
            UiMountedClientPhysicalRect::from_runtime_mounting(32, 20, 128, 76).unwrap();
        let coordinate_basis = coordinate_basis();
        let input = UiMountedIdentityOverlayMechanicInput {
            overlay_identity: 7,
            base_snapshot: 11,
            base_frame,
            target_receipt,
            successor_frame,
            surface,
            binding,
            coordinate_basis,
            target_region,
        };

        let mechanic = UiMountedIdentityOverlayMechanic::from_runtime_mounting(input).unwrap();
        assert_eq!(mechanic.overlay_identity(), 7);
        assert_eq!(mechanic.base_snapshot(), 11);
        assert_eq!(mechanic.base_frame(), base_frame);
        assert_eq!(mechanic.target_receipt(), target_receipt);
        assert_eq!(mechanic.successor_frame(), successor_frame);
        assert_eq!(mechanic.surface(), surface);
        assert_eq!(mechanic.binding(), binding);
        assert_eq!(mechanic.coordinate_basis(), coordinate_basis);
        assert_eq!(mechanic.target_region(), target_region);
        assert_eq!(mechanic.border_width(), 2);
        assert_eq!(mechanic.color().channels(), [255, 0, 255, 255]);

        assert!(UiMountedIdentityOverlayMechanic::from_runtime_mounting(
            UiMountedIdentityOverlayMechanicInput {
                successor_frame: base_frame,
                ..input
            }
        )
        .is_none());
    }

    #[test]
    fn physical_target_rect_rejects_empty_edges() {
        assert!(UiMountedClientPhysicalRect::from_runtime_mounting(4, 5, 4, 8).is_none());
        assert!(UiMountedClientPhysicalRect::from_runtime_mounting(4, 5, 8, 5).is_none());
        assert!(UiMountedClientPhysicalRect::from_runtime_mounting(4, 5, 8, 9).is_some());
    }

    fn coordinate_basis() -> UiMountedClientCoordinateBasis {
        UiMountedClientCoordinateBasis::from_runtime_mounting(
            crate::UiHostCoordinateTransform::observed_by_host(
                crate::UiHostClientAreaObservation::observed_by_host([40, 24], [160, 96]),
                crate::UiHostViewportTransformObservation::observed_by_host(
                    [160.0, 96.0],
                    [1.0, 1.0],
                    [0.0, 0.0],
                ),
                crate::UiHostCoordinatePosture::observed_by_host(
                    crate::UiHostCoordinateOrientation::TopLeftOrigin,
                    crate::UiHostCoordinateRounding::PixelCenterNearest,
                ),
            ),
        )
        .unwrap()
    }
}
