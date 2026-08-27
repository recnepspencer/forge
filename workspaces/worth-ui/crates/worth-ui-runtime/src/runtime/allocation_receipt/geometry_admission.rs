#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime) enum UiAllocationGeometryAdmissionDenial {
    NonFinite,
    NegativeExtent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiAllocationGeometrySurface {
    semantic: worth_ui_host_contract::UiSemanticSurfaceIdentity,
    host: worth_ui_host_contract::UiHostSurfaceIdentity,
    binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::runtime) struct UiAdmittedAllocationGeometry {
    components: [f32; 4],
    surface: UiAllocationGeometrySurface,
}

impl UiAdmittedAllocationGeometry {
    pub(in crate::runtime) fn from_allocation_admission(
        bounds: crate::runtime::UiAllocationAxisAlignedBounds,
        mounted_surface: crate::mounting::UiSurfaceBindingIdentityView,
    ) -> Result<Self, UiAllocationGeometryAdmissionDenial> {
        let components = [bounds.x(), bounds.y(), bounds.width(), bounds.height()];
        if components.iter().any(|value| !value.is_finite()) {
            return Err(UiAllocationGeometryAdmissionDenial::NonFinite);
        }
        if components[2] < 0.0 || components[3] < 0.0 {
            return Err(UiAllocationGeometryAdmissionDenial::NegativeExtent);
        }
        Ok(Self {
            components,
            surface: UiAllocationGeometrySurface::from_mounted_binding(mounted_surface),
        })
    }

    pub(in crate::runtime) const fn components(self) -> [f32; 4] {
        self.components
    }

    pub(crate) const fn surface(self) -> UiAllocationGeometrySurface {
        self.surface
    }
}

impl UiAllocationGeometrySurface {
    fn from_mounted_binding(binding: crate::mounting::UiSurfaceBindingIdentityView) -> Self {
        Self {
            semantic: binding.semantic_surface_identity(),
            host: binding.host_surface_identity(),
            binding: binding.binding_generation(),
        }
    }

    #[cfg(test)]
    pub(crate) const fn for_test(
        semantic: worth_ui_host_contract::UiSemanticSurfaceIdentity,
        host: worth_ui_host_contract::UiHostSurfaceIdentity,
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    ) -> Self {
        Self {
            semantic,
            host,
            binding,
        }
    }

    pub(crate) const fn host_surface(self) -> worth_ui_host_contract::UiHostSurfaceIdentity {
        self.host
    }

    pub(crate) const fn binding(self) -> worth_ui_host_contract::UiSurfaceBindingGeneration {
        self.binding
    }
}
