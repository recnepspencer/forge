mod capability_reload;
mod interaction_application;
mod rebind_execution;

use worth_ui::facade::{
    WorthUiApp, WorthUiHeaderFramePlan, WorthUiHeaderFrameRebindDenial,
    WorthUiHeaderFrameRebindReceipt, WorthUiHeaderFrameRebindRequest, WorthUiPageHostPlan,
    WorthUiPageHostRequest, WorthUiRuntimeHost, WorthUiSemanticChangedSliceSet,
    WorthUiSemanticCompileBoundary, WorthUiSemanticSliceInventory,
};

use crate::app_capabilities::validation_header_frame_rebind_request;
use crate::reload::{
    ValidationAuthoredStructuralReloadEvidence, ValidationAuthoredStructuralSlotEvidence,
    ValidationObservedAuthoredBatch, ValidationPreparedReload, ValidationReloadEvidence,
    ValidationReloadInput, ValidationReloadRequest, ValidationReloadStage, ValidationReloadTick,
    ValidationRuntimeChangeEvidence, ValidationRuntimeReloadTickOutcome, ValidationSourcePackage,
};
use capability_reload::merge_source_reload_with_theme_reload;
pub use interaction_application::{
    ValidationComponentInteractionApplicationDenial, ValidationDropdownSelectionApplicationDenial,
};

pub struct ValidationRuntimeWorkbench {
    app: WorthUiApp,
    runtime: WorthUiRuntimeHost,
    header_frame_plan: WorthUiHeaderFramePlan,
    page_host_plan: WorthUiPageHostPlan,
}

impl ValidationRuntimeWorkbench {
    pub(crate) fn new(
        app: WorthUiApp,
        runtime: WorthUiRuntimeHost,
        header_frame_plan: WorthUiHeaderFramePlan,
        page_host_plan: WorthUiPageHostPlan,
    ) -> Self {
        Self {
            app,
            runtime,
            header_frame_plan,
            page_host_plan,
        }
    }

    pub fn app(&self) -> &WorthUiApp {
        &self.app
    }

    pub fn runtime(&self) -> &WorthUiRuntimeHost {
        &self.runtime
    }

    pub fn runtime_mut(&mut self) -> &mut WorthUiRuntimeHost {
        &mut self.runtime
    }

    pub fn header_frame_plan(&self) -> &WorthUiHeaderFramePlan {
        &self.header_frame_plan
    }

    pub fn page_host_plan(&self) -> &WorthUiPageHostPlan {
        &self.page_host_plan
    }

    pub fn validation_header_frame_rebind_request(&self) -> WorthUiHeaderFrameRebindRequest {
        validation_header_frame_rebind_request()
    }

    pub fn validation_page_host_request(&self) -> WorthUiPageHostRequest {
        WorthUiPageHostRequest::new(self.page_host_plan.page_name())
    }

    pub fn select_page_host_page(
        &mut self,
        page_name: &str,
    ) -> Result<(), worth_ui::facade::WorthUiPageHostPlanDenial> {
        self.page_host_plan = WorthUiPageHostPlan::from_runtime(
            &self.runtime,
            WorthUiPageHostRequest::new(page_name),
        )?;
        Ok(())
    }

    pub(crate) fn into_launch_parts(
        self,
    ) -> (
        WorthUiApp,
        WorthUiRuntimeHost,
        WorthUiHeaderFramePlan,
        WorthUiPageHostPlan,
    ) {
        (
            self.app,
            self.runtime,
            self.header_frame_plan,
            self.page_host_plan,
        )
    }

    pub fn prepare_reload(&self, request: ValidationReloadRequest) -> ValidationPreparedReload {
        self.runtime
            .prepare_validation_reload(self.runtime.active_capability_snapshot(), request)
    }

    pub fn activate_reload(
        &mut self,
        prepared: ValidationPreparedReload,
    ) -> Result<ValidationReloadEvidence, ValidationReloadStage> {
        prepared.activate(&mut self.runtime)
    }

    pub fn rebind_header_after_reload(
        &mut self,
        evidence: &ValidationReloadEvidence,
    ) -> Result<WorthUiHeaderFrameRebindReceipt, WorthUiHeaderFrameRebindDenial> {
        let (next_plan, receipt) = self.runtime.rebind_header_frame_after_reload(
            self.app.capabilities(),
            &self.header_frame_plan,
            validation_header_frame_rebind_request(),
            evidence,
        )?;
        self.header_frame_plan = next_plan;
        Ok(receipt)
    }

    pub fn apply_reload_tick(
        &mut self,
        tick: ValidationReloadTick,
    ) -> ValidationRuntimeReloadTickOutcome {
        match tick {
            ValidationReloadTick::Unchanged(observation) => {
                ValidationRuntimeReloadTickOutcome::Unchanged(observation)
            }
            ValidationReloadTick::Unreadable(denial) => {
                ValidationRuntimeReloadTickOutcome::InputUnreadable(denial)
            }
            ValidationReloadTick::Changed(input) => self.apply_reload_input(input),
        }
    }

    fn apply_reload_input(
        &mut self,
        input: ValidationReloadInput,
    ) -> ValidationRuntimeReloadTickOutcome {
        match input {
            ValidationReloadInput::ObservedAuthoredBatch(batch) => {
                self.apply_observed_authored_batch(batch)
            }
            ValidationReloadInput::SourcePackage(source) => self.apply_source_reload(source),
            ValidationReloadInput::HeaderTheme(theme) => self.apply_theme_reload(theme),
            ValidationReloadInput::HeaderCommands(commands) => self.apply_command_reload(commands),
            ValidationReloadInput::HeaderCommandProjections(command_projections) => {
                self.apply_command_projection_reload(command_projections)
            }
            ValidationReloadInput::HeaderComponents(component) => {
                self.apply_component_reload(component)
            }
            ValidationReloadInput::HeaderAppearance(appearance) => {
                self.apply_appearance_reload(appearance)
            }
            ValidationReloadInput::HeaderDensity(density) => self.apply_density_reload(density),
            ValidationReloadInput::LiveViewSource(_) => {
                unreachable!("live-view reload input must be applied by ValidationWorkbenchApp")
            }
            ValidationReloadInput::HeaderAppearanceAndDensity {
                appearance,
                density,
            } => self.apply_appearance_and_density_reload_input(appearance, density),
            ValidationReloadInput::SourcePackageAndHeaderTheme { source, theme } => {
                let source_outcome = self.apply_source_reload(source);
                let theme_outcome = self.apply_theme_reload(theme);
                merge_source_reload_with_theme_reload(source_outcome, theme_outcome)
            }
        }
    }

    fn apply_observed_authored_batch(
        &mut self,
        batch: ValidationObservedAuthoredBatch,
    ) -> ValidationRuntimeReloadTickOutcome {
        let (source, theme, command, command_projection, component, appearance, density) =
            batch.into_parts();
        let previous_slots = self.current_page_slot_structure();
        let prepared = self.prepare_reload(ValidationReloadRequest::from_source_module(
            source.module_path(),
            source.source_text(),
        ));
        let authored_structural_receipt = prepared.changed_fact_mapping_receipt().cloned();
        let source_evidence = if prepared.is_ready() {
            self.activate_reload(prepared)
                .expect("observed authored batch should activate source immediately")
        } else {
            prepared.evidence().clone()
        };
        let authored_structural = authored_structural_receipt.map(|receipt| {
            ValidationAuthoredStructuralReloadEvidence::from_mapping_receipt(
                &receipt,
                previous_slots,
                self.current_page_slot_structure(),
            )
        });
        let capability_evidence = self.apply_authored_batch_capability_reload(
            theme,
            command,
            command_projection,
            component,
            appearance,
            density,
        );
        let admitted_change = self
            .runtime
            .admit_authored_runtime_change(&source_evidence, Some(&capability_evidence))
            .expect("source and capability evidence should admit a common authored batch change");
        let runtime_change =
            ValidationRuntimeChangeEvidence::from_admitted_change(&admitted_change);
        let compile_boundary = worth_ui::facade::WorthUiCompileBoundaryCertification::certify(
            &WorthUiSemanticCompileBoundary::current(),
            &WorthUiSemanticChangedSliceSet::lower_runtime_change(
                &WorthUiSemanticSliceInventory::current(),
                &admitted_change,
            ),
        );
        let phase_execution = self.runtime_change_rebind_receipts(&admitted_change);
        ValidationRuntimeReloadTickOutcome::AuthoredBatchReloaded {
            source_evidence,
            capability_evidence,
            runtime_change,
            compile_boundary,
            phase_execution,
            authored_structural,
        }
    }

    fn apply_authored_batch_capability_reload(
        &mut self,
        theme: Option<crate::reload::ValidationThemeSource>,
        command: Option<crate::reload::ValidationCommandSource>,
        command_projection: Option<crate::reload::ValidationCommandProjectionSource>,
        component: Option<crate::reload::ValidationComponentSource>,
        appearance: Option<crate::reload::ValidationAppearanceSource>,
        density: Option<crate::reload::ValidationDensitySource>,
    ) -> worth_ui::facade::WorthUiCapabilityReloadEvidence {
        let prepared = self.prepare_authored_batch_capability_reload(
            theme.as_ref(),
            command.as_ref(),
            command_projection.as_ref(),
            component.as_ref(),
            appearance.as_ref(),
            density.as_ref(),
        );
        if prepared.is_ready() {
            self.activate_capability_reload(prepared)
                .expect("observed authored batch should activate capability batch immediately")
        } else {
            prepared.evidence().clone()
        }
    }

    fn apply_source_reload(
        &mut self,
        source: ValidationSourcePackage,
    ) -> ValidationRuntimeReloadTickOutcome {
        let prepared = self.prepare_reload(ValidationReloadRequest::from_source_module(
            source.module_path(),
            source.source_text(),
        ));
        let previous_slots = self.current_page_slot_structure();
        let authored_structural = prepared
            .changed_fact_mapping_receipt()
            .map(|receipt| receipt.clone());
        if prepared.is_ready() {
            return match self.activate_reload(prepared) {
                Ok(evidence) => {
                    let admitted_change = self
                        .runtime
                        .admit_validation_runtime_change(&evidence)
                        .expect("activated validation evidence should admit runtime change");
                    let phase_execution = self.runtime_change_rebind_receipts(&admitted_change);
                    ValidationRuntimeReloadTickOutcome::SourceReloaded {
                        evidence,
                        phase_execution,
                        authored_structural: authored_structural.map(|receipt| {
                            ValidationAuthoredStructuralReloadEvidence::from_mapping_receipt(
                                &receipt,
                                previous_slots,
                                self.current_page_slot_structure(),
                            )
                        }),
                    }
                }
                Err(stage) => ValidationRuntimeReloadTickOutcome::SourceActivationDenied(stage),
            };
        }

        let evidence = prepared.evidence().clone();
        let admitted_change = self
            .runtime
            .admit_validation_runtime_change(&evidence)
            .expect("non-activated validation evidence should still admit common runtime change");
        let phase_execution = self.runtime_change_rebind_receipts(&admitted_change);
        ValidationRuntimeReloadTickOutcome::SourceReloaded {
            evidence,
            phase_execution,
            authored_structural: authored_structural.map(|receipt| {
                ValidationAuthoredStructuralReloadEvidence::from_mapping_receipt(
                    &receipt,
                    previous_slots,
                    self.current_page_slot_structure(),
                )
            }),
        }
    }

    fn apply_appearance_and_density_reload_input(
        &mut self,
        appearance: crate::reload::ValidationAppearanceSource,
        density: crate::reload::ValidationDensitySource,
    ) -> ValidationRuntimeReloadTickOutcome {
        match self.apply_appearance_and_density_capability_reload(&appearance, &density) {
            Ok((evidence, phase_execution)) => {
                ValidationRuntimeReloadTickOutcome::AppearanceAndDensityReloaded {
                    evidence,
                    phase_execution,
                }
            }
            Err(stage) => ValidationRuntimeReloadTickOutcome::AppearanceActivationDenied(stage),
        }
    }

    fn current_page_slot_structure(&self) -> Vec<ValidationAuthoredStructuralSlotEvidence> {
        let runtime = self.runtime();
        self.page_host_plan()
            .execute_frame()
            .slots()
            .iter()
            .filter_map(|slot| {
                let surface_id = worth_ui::facade::SurfaceId::new(slot.surface_id()).ok()?;
                let surface = runtime.inspect_active_surface_descriptor(&surface_id)?;
                let component_id = runtime
                    .inspect_active_authored_surface_component_id(&surface_id)
                    .unwrap_or_else(|| surface.component_id().as_str());
                Some(ValidationAuthoredStructuralSlotEvidence::new(
                    slot.slot_name().to_owned(),
                    slot.surface_id().to_owned(),
                    component_id.to_owned(),
                ))
            })
            .collect()
    }
}
