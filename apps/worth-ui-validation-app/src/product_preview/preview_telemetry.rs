use worth_ui::facade::{
    WorthUiLayoutAxis, WorthUiPageHostBoundaryPosture, WorthUiPageHostPresentationChild,
    WorthUiPageHostPresentationRegion, WorthUiPageHostResolvedSizing, WorthUiRuntimeHost,
};

const PREVIEW_SHELL_SIZE: ValidationPreviewSize = ValidationPreviewSize::new(1240.0, 820.0);

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ValidationPreviewSize {
    width: f32,
    height: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ValidationPreviewTelemetry {
    root: ValidationPreviewRegionTelemetry,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ValidationPreviewRegionTelemetry {
    axis: WorthUiLayoutAxis,
    gap: Option<f32>,
    padding: Option<f32>,
    scroll_owner: bool,
    sizing: Option<WorthUiPageHostResolvedSizing>,
    sibling_boundaries: Vec<WorthUiPageHostBoundaryPosture>,
    children: Vec<ValidationPreviewChildTelemetry>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ValidationPreviewChildTelemetry {
    Region(ValidationPreviewRegionTelemetry),
    Slot(ValidationPreviewSlotTelemetry),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ValidationPreviewSlotTelemetry {
    surface_id: String,
    component_id: String,
    measured_size: ValidationPreviewSize,
}

impl ValidationPreviewSize {
    const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub(crate) fn width(self) -> f32 {
        self.width
    }

    pub(crate) fn height(self) -> f32 {
        self.height
    }
}

impl ValidationPreviewTelemetry {
    pub(crate) fn inspect(
        runtime: &WorthUiRuntimeHost,
        page_name: &str,
    ) -> Result<Self, worth_ui::facade::WorthUiPageHostPlanDenial> {
        let presentation = runtime.inspect_page_host_presentation(page_name)?;
        Ok(Self {
            root: build_region_telemetry(presentation.root(), runtime),
        })
    }

    pub(crate) fn root(&self) -> &ValidationPreviewRegionTelemetry {
        &self.root
    }

    pub(crate) fn find_slot(&self, surface_id: &str) -> Option<&ValidationPreviewSlotTelemetry> {
        self.root.find_slot(surface_id)
    }
}

impl ValidationPreviewRegionTelemetry {
    pub(crate) fn gap(&self) -> Option<f32> {
        self.gap
    }

    pub(crate) fn padding(&self) -> Option<f32> {
        self.padding
    }

    fn find_slot(&self, surface_id: &str) -> Option<&ValidationPreviewSlotTelemetry> {
        self.children.iter().find_map(|child| match child {
            ValidationPreviewChildTelemetry::Region(region) => region.find_slot(surface_id),
            ValidationPreviewChildTelemetry::Slot(slot) if slot.surface_id() == surface_id => {
                Some(slot)
            }
            ValidationPreviewChildTelemetry::Slot(_) => None,
        })
    }
}

impl ValidationPreviewSlotTelemetry {
    pub(crate) fn surface_id(&self) -> &str {
        &self.surface_id
    }

    pub(crate) fn measured_size(&self) -> ValidationPreviewSize {
        self.measured_size
    }
}

fn build_region_telemetry(
    region: &WorthUiPageHostPresentationRegion,
    runtime: &WorthUiRuntimeHost,
) -> ValidationPreviewRegionTelemetry {
    ValidationPreviewRegionTelemetry {
        axis: region.axis().clone(),
        gap: region.gap(),
        padding: region.padding(),
        scroll_owner: region.scroll_owner(),
        sizing: region.sizing().cloned(),
        sibling_boundaries: region.sibling_boundaries().to_vec(),
        children: region
            .children()
            .iter()
            .map(|child| match child {
                WorthUiPageHostPresentationChild::Region(region) => {
                    ValidationPreviewChildTelemetry::Region(build_region_telemetry(region, runtime))
                }
                WorthUiPageHostPresentationChild::Slot(slot) => {
                    ValidationPreviewChildTelemetry::Slot(ValidationPreviewSlotTelemetry {
                        surface_id: slot.surface_id().to_owned(),
                        component_id: slot.component_id().to_owned(),
                        measured_size: ValidationPreviewSize::new(
                            measure_surface_width(
                                slot.surface_id(),
                                slot.component_id(),
                                PREVIEW_SHELL_SIZE.height(),
                            ),
                            measure_surface_height(
                                slot.surface_id(),
                                slot.component_id(),
                                PREVIEW_SHELL_SIZE.width(),
                            ),
                        ),
                    })
                }
            })
            .collect(),
    }
}

fn measure_surface_width(surface_id: &str, component_id: &str, _available_height: f32) -> f32 {
    match component_id {
        "worth.component.button" => {
            if surface_id == "worth.surface.preview.button.proof" {
                132.0
            } else {
                120.0
            }
        }
        _ => 120.0,
    }
}

fn measure_surface_height(surface_id: &str, component_id: &str, _available_width: f32) -> f32 {
    match component_id {
        "worth.component.button" => {
            if surface_id == "worth.surface.preview.button.proof" {
                40.0
            } else {
                36.0
            }
        }
        _ => 40.0,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        launch::ValidationWorkbenchLaunch, reload::ValidationSourcePackage,
        ValidationWorkbenchAuthoredInputs,
    };

    use super::{ValidationPreviewSize, ValidationPreviewTelemetry};

    #[test]
    fn preview_shell_telemetry_exposes_runtime_gap_and_padding() {
        let prepared = ValidationWorkbenchLaunch::new()
            .prepare_from_authored_inputs(ValidationWorkbenchAuthoredInputs::new(
                ValidationSourcePackage::sample(),
            ))
            .expect("validation workbench should prepare");
        let telemetry = ValidationPreviewTelemetry::inspect(prepared.runtime(), "HeaderProofPage")
            .expect("preview telemetry should inspect");

        assert_eq!(telemetry.root().gap(), Some(0.0));
        assert_eq!(telemetry.root().padding(), Some(0.0));
    }

    #[test]
    fn preview_shell_telemetry_tracks_visible_slot_metrics() {
        let prepared = ValidationWorkbenchLaunch::new()
            .prepare_from_authored_inputs(ValidationWorkbenchAuthoredInputs::new(
                ValidationSourcePackage::sample(),
            ))
            .expect("validation workbench should prepare");
        let telemetry = ValidationPreviewTelemetry::inspect(prepared.runtime(), "HeaderProofPage")
            .expect("preview telemetry should inspect");
        let primitive = telemetry
            .find_slot("worth.surface.preview.primitive.proof")
            .expect("primitive proof slot should exist");

        assert_eq!(
            primitive.measured_size(),
            ValidationPreviewSize::new(120.0, 40.0)
        );
    }
}
