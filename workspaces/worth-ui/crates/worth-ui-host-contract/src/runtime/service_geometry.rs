#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostServiceGeometryDenial {
    NonFinite,
    NegativeExtent,
    EmptyPhysicalExtent,
    PhysicalEdgeOrder,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiHostSurfaceLogicalGeometry {
    surface: crate::UiHostSurfaceIdentity,
    binding: crate::UiSurfaceBindingGeneration,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostPhysicalPixelGeometry {
    surface: crate::UiHostSurfaceIdentity,
    binding: crate::UiSurfaceBindingGeneration,
    left: u32,
    top: u32,
    right: u32,
    bottom: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostPhysicalPixelGeometryInput {
    pub surface: crate::UiHostSurfaceIdentity,
    pub binding: crate::UiSurfaceBindingGeneration,
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
}

impl UiHostSurfaceLogicalGeometry {
    #[doc(hidden)]
    pub fn from_sampled_logical_projection(
        presentation: crate::UiHostObservationPresentationBasis,
        components: [f32; 4],
    ) -> Result<Self, UiHostServiceGeometryDenial> {
        if components.iter().any(|value| !value.is_finite()) {
            return Err(UiHostServiceGeometryDenial::NonFinite);
        }
        if components[2] < 0.0 || components[3] < 0.0 {
            return Err(UiHostServiceGeometryDenial::NegativeExtent);
        }
        Ok(Self {
            surface: presentation.host_surface(),
            binding: presentation.binding(),
            x: components[0],
            y: components[1],
            width: components[2],
            height: components[3],
        })
    }

    pub const fn components(self) -> [f32; 4] {
        [self.x, self.y, self.width, self.height]
    }

    pub const fn surface(self) -> crate::UiHostSurfaceIdentity {
        self.surface
    }

    pub const fn binding(self) -> crate::UiSurfaceBindingGeneration {
        self.binding
    }
}

impl UiHostPhysicalPixelGeometry {
    pub fn observed_by_host(
        input: UiHostPhysicalPixelGeometryInput,
    ) -> Result<Self, UiHostServiceGeometryDenial> {
        if input.left == input.right || input.top == input.bottom {
            return Err(UiHostServiceGeometryDenial::EmptyPhysicalExtent);
        }
        if input.left > input.right || input.top > input.bottom {
            return Err(UiHostServiceGeometryDenial::PhysicalEdgeOrder);
        }
        Ok(Self {
            surface: input.surface,
            binding: input.binding,
            left: input.left,
            top: input.top,
            right: input.right,
            bottom: input.bottom,
        })
    }

    pub const fn edges(self) -> [u32; 4] {
        [self.left, self.top, self.right, self.bottom]
    }

    pub const fn surface(self) -> crate::UiHostSurfaceIdentity {
        self.surface
    }

    pub const fn binding(self) -> crate::UiSurfaceBindingGeneration {
        self.binding
    }
}

#[cfg(test)]
mod tests {
    use super::{
        UiHostPhysicalPixelGeometry, UiHostPhysicalPixelGeometryInput, UiHostServiceGeometryDenial,
        UiHostSurfaceLogicalGeometry,
    };

    #[test]
    fn host_logical_and_physical_geometry_are_not_interchangeable() {
        let surface = crate::UiHostSurfaceIdentity::mint_unbound().unwrap();
        let binding = crate::UiSurfaceBindingGeneration::mint_unbound().unwrap();
        let frame = crate::UiMountedFrameIdentity::mint_unbound().unwrap();
        let presentation = crate::UiHostObservationPresentationBasis::new(
            surface,
            frame,
            binding,
            crate::UiHostPresentationEpoch::issued_by_host(1),
        );
        let logical = UiHostSurfaceLogicalGeometry::from_sampled_logical_projection(
            presentation,
            [2.5, 3.0, 10.0, 4.0],
        )
        .expect("finite logical geometry");
        let physical =
            UiHostPhysicalPixelGeometry::observed_by_host(UiHostPhysicalPixelGeometryInput {
                surface,
                binding,
                left: 5,
                top: 6,
                right: 25,
                bottom: 14,
            })
            .expect("ordered physical geometry");

        assert_eq!(logical.components(), [2.5, 3.0, 10.0, 4.0]);
        assert_eq!(physical.edges(), [5, 6, 25, 14]);
        assert_eq!(logical.surface(), surface);
        assert_eq!(physical.binding(), binding);
        assert_ne!(
            core::any::TypeId::of::<UiHostSurfaceLogicalGeometry>(),
            core::any::TypeId::of::<UiHostPhysicalPixelGeometry>()
        );
    }

    #[test]
    fn host_geometry_rejects_invalid_values_before_transport() {
        let surface = crate::UiHostSurfaceIdentity::mint_unbound().unwrap();
        let binding = crate::UiSurfaceBindingGeneration::mint_unbound().unwrap();
        assert_eq!(
            UiHostSurfaceLogicalGeometry::from_sampled_logical_projection(
                crate::UiHostObservationPresentationBasis::new(
                    surface,
                    crate::UiMountedFrameIdentity::mint_unbound().unwrap(),
                    binding,
                    crate::UiHostPresentationEpoch::issued_by_host(1),
                ),
                [0.0, 0.0, -1.0, 1.0],
            ),
            Err(UiHostServiceGeometryDenial::NegativeExtent)
        );
        assert_eq!(
            UiHostPhysicalPixelGeometry::observed_by_host(UiHostPhysicalPixelGeometryInput {
                surface,
                binding,
                left: 8,
                top: 2,
                right: 4,
                bottom: 6,
            }),
            Err(UiHostServiceGeometryDenial::PhysicalEdgeOrder)
        );
    }
}
