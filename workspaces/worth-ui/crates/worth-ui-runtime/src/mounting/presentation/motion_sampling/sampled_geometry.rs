#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiPresentationGeometrySamplingDenial {
    NonFinite,
    NegativeExtent,
    MissingSemanticBasis,
    PresentationBindingChanged,
    PresentationSurfaceChanged,
}

/// Geometry derived from a committed Motion receipt at one exact presented
/// host basis. This is deliberately not allocation or layout geometry.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct UiPresentationSampledGeometry {
    components: [f32; 4],
    target: crate::runtime::motion::UiMotionTargetIdentity,
    owner_revision: u64,
    presentation_basis: worth_ui_host_contract::UiHostObservationPresentationBasis,
}

impl UiPresentationSampledGeometry {
    pub(crate) fn from_motion_sample(
        target: crate::runtime::motion::UiMotionTargetIdentity,
        owner_revision: u64,
        expected_presentation: worth_ui_host_contract::UiHostObservationPresentationBasis,
        _semantic_basis: crate::runtime::motion::UiMotionSemanticGeometry,
        components: [f32; 4],
        presentation_basis: worth_ui_host_contract::UiHostObservationPresentationBasis,
    ) -> Result<Self, UiPresentationGeometrySamplingDenial> {
        if presentation_basis.host_surface() != expected_presentation.host_surface() {
            return Err(UiPresentationGeometrySamplingDenial::PresentationSurfaceChanged);
        }
        if presentation_basis.binding() != expected_presentation.binding() {
            return Err(UiPresentationGeometrySamplingDenial::PresentationBindingChanged);
        }
        if components.iter().any(|value| !value.is_finite()) {
            return Err(UiPresentationGeometrySamplingDenial::NonFinite);
        }
        if components[2] < 0.0 || components[3] < 0.0 {
            return Err(UiPresentationGeometrySamplingDenial::NegativeExtent);
        }
        Ok(Self {
            components,
            target,
            owner_revision,
            presentation_basis,
        })
    }

    pub(crate) const fn components(self) -> [f32; 4] {
        self.components
    }

    pub(crate) const fn target(self) -> crate::runtime::motion::UiMotionTargetIdentity {
        self.target
    }

    pub(crate) const fn owner_revision(self) -> u64 {
        self.owner_revision
    }

    pub(crate) const fn presentation_basis(
        self,
    ) -> worth_ui_host_contract::UiHostObservationPresentationBasis {
        self.presentation_basis
    }

    pub(super) fn with_presentation_basis(
        mut self,
        presentation_basis: worth_ui_host_contract::UiHostObservationPresentationBasis,
    ) -> Result<Self, UiPresentationGeometrySamplingDenial> {
        if presentation_basis.host_surface() != self.presentation_basis.host_surface() {
            return Err(UiPresentationGeometrySamplingDenial::PresentationSurfaceChanged);
        }
        if presentation_basis.binding() != self.presentation_basis.binding() {
            return Err(UiPresentationGeometrySamplingDenial::PresentationBindingChanged);
        }
        self.presentation_basis = presentation_basis;
        Ok(self)
    }

    pub(crate) fn project_to_host_surface(
        self,
    ) -> Result<
        worth_ui_host_contract::UiHostSurfaceLogicalGeometry,
        worth_ui_host_contract::UiHostServiceGeometryDenial,
    > {
        worth_ui_host_contract::UiHostSurfaceLogicalGeometry::from_sampled_logical_projection(
            self.presentation_basis,
            self.components,
        )
    }
}
