use crate::capability::{
    AppearanceTokenId, CapabilitySnapshot, ComponentDescriptor, ComponentId, DensityTokenId,
    MeasurementValue, SurfaceDescriptor, SurfaceId, ThemeTokenDescriptor, ThemeTokenId,
    WorthUiAppearanceTokenDescriptor, WorthUiDensityTokenDescriptor,
};
use crate::runtime::{
    WorthUiActiveRuntimeObservation, WorthUiAuthoredSurfacePropEntry,
    WorthUiRuntimeAuthoringSnapshot, WorthUiRuntimeFrameEpoch, WorthUiRuntimeHost,
    WorthUiRuntimeLifecycle,
};

impl WorthUiRuntimeHost {
    pub fn lifecycle(&self) -> WorthUiRuntimeLifecycle {
        self.active_state_for_read().lifecycle()
    }

    pub fn frame_epoch(&self) -> WorthUiRuntimeFrameEpoch {
        self.active_state_for_read().frame_epoch()
    }

    pub fn inspect_active(&self) -> WorthUiActiveRuntimeObservation {
        self.active_state_for_read().observation()
    }

    pub fn active_capability_snapshot(&self) -> &CapabilitySnapshot {
        self.active_state_for_read().capability_snapshot()
    }

    pub fn inspect_active_component_descriptor(
        &self,
        component_id: &ComponentId,
    ) -> Option<&ComponentDescriptor> {
        self.active_state_for_read()
            .capability_snapshot()
            .components()
            .get(component_id)
    }

    pub fn inspect_active_surface_descriptor(
        &self,
        surface_id: &SurfaceId,
    ) -> Option<&SurfaceDescriptor> {
        self.active_state_for_read()
            .capability_snapshot()
            .surfaces()
            .get(surface_id)
    }

    pub fn inspect_active_authored_surface_component_id(
        &self,
        surface_id: &SurfaceId,
    ) -> Option<&str> {
        self.active_state_for_read()
            .authoring_snapshot()
            .and_then(|snapshot| {
                snapshot
                    .authored_surfaces()
                    .component_id_for_surface(surface_id.as_str())
            })
    }

    pub fn inspect_active_authored_surface_prop(
        &self,
        surface_id: &SurfaceId,
        key: &str,
    ) -> Option<&str> {
        self.active_state_for_read()
            .authoring_snapshot()
            .and_then(|snapshot| {
                snapshot
                    .authored_surface_props()
                    .string_prop(surface_id.as_str(), key)
            })
    }

    pub fn inspect_active_authored_surface_props<'a>(
        &'a self,
        surface_id: &'a SurfaceId,
    ) -> impl Iterator<Item = &'a WorthUiAuthoredSurfacePropEntry> + 'a {
        self.active_state_for_read()
            .authoring_snapshot()
            .into_iter()
            .flat_map(move |snapshot| {
                snapshot
                    .authored_surface_props()
                    .entries_for_surface(surface_id.as_str())
            })
    }

    pub fn inspect_active_appearance_token_descriptor(
        &self,
        appearance_token_id: &AppearanceTokenId,
    ) -> Option<&WorthUiAppearanceTokenDescriptor> {
        self.active_state_for_read()
            .capability_snapshot()
            .appearance_tokens()
            .get(appearance_token_id)
    }

    pub fn inspect_active_theme_token_descriptor(
        &self,
        theme_token_id: &ThemeTokenId,
    ) -> Option<&ThemeTokenDescriptor> {
        self.active_state_for_read()
            .capability_snapshot()
            .theme_tokens()
            .get(theme_token_id)
    }

    pub fn inspect_active_density_token_descriptor(
        &self,
        density_token_id: &DensityTokenId,
    ) -> Option<&WorthUiDensityTokenDescriptor> {
        self.active_state_for_read()
            .capability_snapshot()
            .density_tokens()
            .get(density_token_id)
    }

    pub fn active_authoring_snapshot(&self) -> Option<&WorthUiRuntimeAuthoringSnapshot> {
        self.active_state_for_read().authoring_snapshot()
    }

    pub fn inspect_active_named_measurement_pixels(&self, token: &str) -> Option<f32> {
        self.active_state_for_read()
            .capability_snapshot()
            .mosaic_sizing_contracts()
            .descriptors()
            .iter()
            .find_map(|descriptor| {
                let measurement = descriptor.named_measurement()?;
                if measurement.token().as_str() != token {
                    return None;
                }
                match measurement.value() {
                    MeasurementValue::LogicalPixels(value)
                    | MeasurementValue::BreakpointLogicalPixels(value) => Some(*value as f32),
                    _ => None,
                }
            })
    }
}
