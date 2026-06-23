use worth_ui::facade::{
    AppearanceTokenId, DensityTokenId, SurfaceId, WorthUiAppearanceValue, WorthUiDensityValue,
};

use super::render_plan::{
    ValidationPageSlotAppearanceDependencyProof, ValidationPageSlotDensityDependencyProof,
    ValidationPageSlotInteractionRenderPlan, ValidationPageSlotInteractionSlotRow,
};
use crate::reload::{ValidationAuthoredStructuralReloadEvidence, ValidationPageHostRebindEvidence};
use crate::runtime_workbench::ValidationRuntimeWorkbench;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationPageSlotInteractionProjection {
    page_name: String,
    slots: Vec<ValidationPageSlotInteractionSlotRow>,
    previous_slots: Vec<ValidationPageSlotInteractionSlotRow>,
    shadow_dependency: ValidationPageSlotAppearanceDependencyProof,
    padding_dependency: ValidationPageSlotDensityDependencyProof,
    authored_structural: Option<ValidationAuthoredStructuralReloadEvidence>,
    latest_rebind: Option<ValidationPageHostRebindEvidence>,
}

impl ValidationPageSlotInteractionProjection {
    pub fn from_workbench(
        workbench: &ValidationRuntimeWorkbench,
        latest_rebind: Option<&ValidationPageHostRebindEvidence>,
        authored_structural: Option<&ValidationAuthoredStructuralReloadEvidence>,
    ) -> Self {
        let runtime = workbench.runtime();
        let slots = workbench
            .page_host_plan()
            .execute_frame()
            .slots()
            .iter()
            .filter_map(|slot| {
                let surface_id = SurfaceId::new(slot.surface_id()).ok()?;
                let surface = runtime.inspect_active_surface_descriptor(&surface_id)?;
                let component_id = runtime
                    .inspect_active_authored_surface_component_id(&surface_id)
                    .unwrap_or_else(|| surface.component_id().as_str());
                Some(ValidationPageSlotInteractionSlotRow::new(
                    slot.slot_name().to_owned(),
                    slot.surface_id().to_owned(),
                    component_id.to_owned(),
                ))
            })
            .collect();

        Self {
            page_name: workbench.page_host_plan().page_name().to_owned(),
            slots,
            previous_slots: authored_structural
                .map(|evidence| {
                    evidence
                        .previous_slots()
                        .iter()
                        .map(|slot| {
                            ValidationPageSlotInteractionSlotRow::new(
                                slot.slot_name().to_owned(),
                                slot.surface_id().to_owned(),
                                slot.component_id().to_owned(),
                            )
                        })
                        .collect()
                })
                .unwrap_or_default(),
            shadow_dependency: shadow_dependency(runtime),
            padding_dependency: padding_dependency(runtime),
            authored_structural: authored_structural.cloned(),
            latest_rebind: latest_rebind.cloned(),
        }
    }

    pub fn into_render_plan(self) -> ValidationPageSlotInteractionRenderPlan {
        ValidationPageSlotInteractionRenderPlan::new(
            self.page_name,
            self.slots,
            self.previous_slots,
            self.shadow_dependency,
            self.padding_dependency,
            self.authored_structural
                .map(|evidence| evidence.rows().to_vec())
                .unwrap_or_default(),
            self.latest_rebind,
        )
    }
}

fn shadow_dependency(
    runtime: &worth_ui::facade::WorthUiRuntimeHost,
) -> ValidationPageSlotAppearanceDependencyProof {
    let Some(token) = AppearanceTokenId::new("validation.appearance.header.panel_shadow").ok()
    else {
        return ValidationPageSlotAppearanceDependencyProof::new(
            "validation.appearance.header.panel_shadow".to_owned(),
            0,
            0,
            0,
            0,
        );
    };
    let Some(descriptor) = runtime.inspect_active_appearance_token_descriptor(&token) else {
        return ValidationPageSlotAppearanceDependencyProof::new(
            token.as_str().to_owned(),
            0,
            0,
            0,
            0,
        );
    };
    if let WorthUiAppearanceValue::Shadow(value) = descriptor.value() {
        return ValidationPageSlotAppearanceDependencyProof::new(
            token.as_str().to_owned(),
            value.offset_x_points().into(),
            value.offset_y_points().into(),
            value.blur_points().into(),
            value.spread_points().into(),
        );
    }
    ValidationPageSlotAppearanceDependencyProof::new(token.as_str().to_owned(), 0, 0, 0, 0)
}

fn padding_dependency(
    runtime: &worth_ui::facade::WorthUiRuntimeHost,
) -> ValidationPageSlotDensityDependencyProof {
    let Some(token) = DensityTokenId::new("validation.density.header.container_padding").ok()
    else {
        return ValidationPageSlotDensityDependencyProof::new(
            "validation.density.header.container_padding".to_owned(),
            0,
            0,
            0,
            0,
        );
    };
    let Some(descriptor) = runtime.inspect_active_density_token_descriptor(&token) else {
        return ValidationPageSlotDensityDependencyProof::new(
            token.as_str().to_owned(),
            0,
            0,
            0,
            0,
        );
    };
    if let WorthUiDensityValue::Padding(value) = descriptor.value() {
        return ValidationPageSlotDensityDependencyProof::new(
            token.as_str().to_owned(),
            value.top().points() as i32,
            value.right().points() as i32,
            value.bottom().points() as i32,
            value.left().points() as i32,
        );
    }
    ValidationPageSlotDensityDependencyProof::new(token.as_str().to_owned(), 0, 0, 0, 0)
}
