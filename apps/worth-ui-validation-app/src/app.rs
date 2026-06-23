mod primitive_content_rendering;
mod primitive_denial_rendering;
mod primitive_event_testing;
mod primitive_interaction;
mod primitive_paint_colors;
mod primitive_proof;
mod reload_loop_config;
mod reset;
mod support;

pub use primitive_event_testing::{
    ValidationMountedPrimitiveEventFrameDenial, ValidationMountedPrimitiveEventFrameReceipt,
    ValidationMountedPrimitiveEventViewport,
};
pub use primitive_interaction::ValidationMountedPrimitiveInteractionDenial;
pub use reload_loop_config::{
    default_reload_loop_config, default_reload_loop_config_from_authored_inputs,
};

use crate::app_proof_snapshot::ValidationAppProofSnapshot;
use crate::header::render_header_only;
use crate::header::ValidationHeaderSelectionAction;
use crate::launch::{PreparedValidationWorkbenchLaunch, ValidationObservedStartupEvidence};
use crate::manual_flow::{actions_for_flow, ValidationManualAppAction, ValidationManualFlowId};
use crate::native_window::validation_native_options;
use crate::pages::manual_flow_matrix::ValidationManualFlowMatrixProjection;
use crate::reload::{
    ValidationAuthoredReloadEdit, ValidationAuthoredReloadEditDenial,
    ValidationCapturedAuthoredBatch, ValidationManualReloadEdit, ValidationReloadEvidenceLog,
    ValidationReloadInput, ValidationReloadLoop, ValidationReloadLoopConfig, ValidationReloadTick,
};
use crate::runtime_workbench::ValidationRuntimeWorkbench;
use crate::{
    ValidationHeaderAppliedStyleReceipt, ValidationManualFlowMatrixSnapshot,
    ValidationWorkbenchAuthoredInputs,
};
use eframe::{App, Frame};
use egui::{CentralPanel, Context};
use primitive_proof::render_centered_primitive_proof;
use std::fs;
use std::time::Duration;
use support::{
    default_header_appearance_path, default_header_command_path,
    default_header_command_projection_path, default_header_component_path,
    default_header_density_path, default_validation_source_path, write_manual_reload_edit,
};
use worth_ui::facade::{
    CommandId, CommandProjectionId, SurfaceId, WorthUiComponentInteractionReceipt,
    WorthUiHeaderMenuPlan, WorthUiHeaderThemePlan, WorthUiPageHostPlan,
};

pub struct ValidationWorkbenchApp {
    workbench: ValidationRuntimeWorkbench,
    reload_loop: ValidationReloadLoop,
    evidence_log: ValidationReloadEvidenceLog,
    baseline_authored_inputs: ValidationWorkbenchAuthoredInputs,
    observed_startup: Option<ValidationObservedStartupEvidence>,
    reload_loop_config: ValidationReloadLoopConfig,
    last_executed_flow: Option<ValidationManualFlowId>,
    last_primitive_interaction: Option<WorthUiComponentInteractionReceipt>,
    last_primitive_interaction_denial: Option<String>,
    staged_manual_reload_edit: Option<ValidationManualReloadEdit>,
}
impl ValidationWorkbenchApp {
    pub fn new(launch: PreparedValidationWorkbenchLaunch) -> Self {
        let reload_loop_config =
            default_reload_loop_config_from_authored_inputs(Some(launch.authored_inputs()));
        Self::new_with_reload_loop_config(launch, reload_loop_config)
    }

    pub fn new_with_reload_loop_config(
        launch: PreparedValidationWorkbenchLaunch,
        reload_loop_config: ValidationReloadLoopConfig,
    ) -> Self {
        let baseline_authored_inputs = ValidationWorkbenchAuthoredInputs::sample();
        let observed_startup = launch.observed_startup().cloned();
        Self {
            workbench: launch.into_runtime_workbench(),
            reload_loop: ValidationReloadLoop::start(reload_loop_config.clone())
                .expect("validation reload loop should observe the header theme file"),
            evidence_log: ValidationReloadEvidenceLog::default(),
            baseline_authored_inputs,
            observed_startup,
            reload_loop_config,
            last_executed_flow: None,
            last_primitive_interaction: None,
            last_primitive_interaction_denial: None,
            staged_manual_reload_edit: None,
        }
    }

    pub fn run_native(launch: PreparedValidationWorkbenchLaunch) -> eframe::Result<()> {
        eframe::run_native(
            "Worth UI Validation App",
            validation_native_options(),
            Box::new(|_| Ok(Box::new(Self::new(launch)))),
        )
    }

    pub fn header_plan(&self) -> &WorthUiHeaderMenuPlan {
        self.workbench.header_frame_plan().menu_plan()
    }

    pub fn header_theme_plan(&self) -> &WorthUiHeaderThemePlan {
        self.workbench.header_frame_plan().theme_plan()
    }

    pub fn page_host_plan(&self) -> &WorthUiPageHostPlan {
        self.workbench.page_host_plan()
    }

    pub fn dispatch_manual_action(&mut self, action: ValidationManualAppAction) {
        match action {
            ValidationManualAppAction::ExecuteFlow(flow_id) => {
                for step in actions_for_flow(flow_id) {
                    self.dispatch_manual_action(step);
                }
                self.last_executed_flow = Some(flow_id);
            }
            ValidationManualAppAction::ResetToBaseline => self.reset_to_baseline(),
            ValidationManualAppAction::StageReloadEdit(edit) => {
                self.staged_manual_reload_edit = Some(edit);
            }
            ValidationManualAppAction::SubmitStagedReloadEdit => {
                if let Some(edit) = self.staged_manual_reload_edit.take() {
                    write_manual_reload_edit(&self.reload_loop_config, edit)
                        .expect("manual reload edit should write observed files");
                    self.apply_next_reload_tick();
                }
            }
            ValidationManualAppAction::SelectDropdownCommand {
                projection_id,
                command_id,
            } => self.apply_dropdown_command_selection(&projection_id, &command_id),
            ValidationManualAppAction::AdvanceReloadCycle => self.apply_next_reload_tick(),
        }
    }

    pub fn apply_manual_reload_edit(&mut self, edit: ValidationManualReloadEdit) {
        self.dispatch_manual_action(ValidationManualAppAction::StageReloadEdit(edit));
        self.dispatch_manual_action(ValidationManualAppAction::SubmitStagedReloadEdit);
    }

    pub fn apply_manual_source_text(&mut self, source_text: impl Into<String>) {
        self.apply_manual_reload_edit(ValidationManualReloadEdit::source_file(
            self.reload_loop_config
                .source_path()
                .cloned()
                .unwrap_or_else(default_validation_source_path),
            source_text.into(),
        ));
    }

    pub fn apply_authored_reload_edit(
        &mut self,
        edit: ValidationAuthoredReloadEdit,
    ) -> Result<(), ValidationAuthoredReloadEditDenial> {
        let source_path = self
            .reload_loop_config
            .source_path()
            .cloned()
            .unwrap_or_else(default_validation_source_path);
        let source_text = fs::read_to_string(&source_path)
            .expect("authored reload edits require a readable observed source file");
        let next_source = edit.apply_to_source_text(&source_text)?;
        self.apply_manual_source_text(next_source);
        Ok(())
    }

    pub fn apply_manual_theme_text(&mut self, source_text: impl Into<String>) {
        self.apply_manual_reload_edit(ValidationManualReloadEdit::theme_file(
            self.reload_loop_config.theme_path().clone(),
            source_text.into(),
        ));
    }

    pub fn apply_manual_command_text(&mut self, source_text: impl Into<String>) {
        self.apply_manual_reload_edit(ValidationManualReloadEdit::command_file(
            self.reload_loop_config
                .command_path()
                .cloned()
                .unwrap_or_else(default_header_command_path),
            source_text.into(),
        ));
    }

    pub fn apply_manual_command_projection_text(&mut self, source_text: impl Into<String>) {
        self.apply_manual_reload_edit(ValidationManualReloadEdit::command_projection_file(
            self.reload_loop_config
                .command_projection_path()
                .cloned()
                .unwrap_or_else(default_header_command_projection_path),
            source_text.into(),
        ));
    }

    pub fn apply_manual_component_text(&mut self, source_text: impl Into<String>) {
        self.apply_manual_reload_edit(ValidationManualReloadEdit::component_file(
            self.reload_loop_config
                .component_path()
                .cloned()
                .unwrap_or_else(default_header_component_path),
            source_text.into(),
        ));
    }

    pub fn apply_manual_appearance_text(&mut self, source_text: impl Into<String>) {
        self.apply_manual_reload_edit(ValidationManualReloadEdit::appearance_file(
            self.reload_loop_config
                .appearance_path()
                .cloned()
                .unwrap_or_else(default_header_appearance_path),
            source_text.into(),
        ));
    }

    pub fn apply_manual_density_text(&mut self, source_text: impl Into<String>) {
        self.apply_manual_reload_edit(ValidationManualReloadEdit::density_file(
            self.reload_loop_config
                .density_path()
                .cloned()
                .unwrap_or_else(default_header_density_path),
            source_text.into(),
        ));
    }

    pub fn apply_manual_appearance_and_density_text(
        &mut self,
        appearance_text: impl Into<String>,
        density_text: impl Into<String>,
    ) {
        self.apply_manual_reload_edit(ValidationManualReloadEdit::appearance_and_density_files(
            self.reload_loop_config
                .appearance_path()
                .cloned()
                .unwrap_or_else(default_header_appearance_path),
            appearance_text.into(),
            self.reload_loop_config
                .density_path()
                .cloned()
                .unwrap_or_else(default_header_density_path),
            density_text.into(),
        ));
    }

    pub fn seed_dropdown_selection(&mut self, projection_id: &str, command_id: &str) {
        self.select_dropdown_command(projection_id, command_id);
    }

    pub fn select_dropdown_command(&mut self, projection_id: &str, command_id: &str) {
        self.dispatch_manual_action(ValidationManualAppAction::select_dropdown_command(
            projection_id,
            command_id,
        ));
    }

    pub fn run_manual_flow(&mut self, flow_id: ValidationManualFlowId) {
        self.dispatch_manual_action(ValidationManualAppAction::ExecuteFlow(flow_id));
    }

    pub fn run_one_update_cycle(&mut self) {
        self.run_one_reload_observation_cycle();
    }
    pub fn run_one_reload_observation_cycle(&mut self) {
        self.apply_next_reload_tick();
    }
    pub fn run_one_reload_observation_cycle_with_capture(
        &mut self,
    ) -> Option<ValidationCapturedAuthoredBatch> {
        self.apply_next_reload_tick_with_capture()
    }

    pub fn replay_captured_authored_batch(&mut self, captured: &ValidationCapturedAuthoredBatch) {
        let outcome = self
            .workbench
            .apply_reload_tick(ValidationReloadTick::Changed(
                ValidationReloadInput::ObservedAuthoredBatch(captured.observed_batch().clone()),
            ));
        self.evidence_log
            .record_runtime_reload_tick_outcome(outcome);
    }
    pub fn proof_snapshot(&self) -> ValidationAppProofSnapshot {
        ValidationAppProofSnapshot::from_workbench(
            &self.workbench,
            &self.evidence_log,
            self.observed_startup.as_ref(),
        )
    }

    pub fn manual_flow_matrix_snapshot(&self) -> ValidationManualFlowMatrixSnapshot {
        ValidationManualFlowMatrixSnapshot::from_proof(
            &self.proof_snapshot(),
            self.last_executed_flow,
        )
    }

    pub fn manual_flow_matrix_render_plan(&self) -> crate::ValidationManualFlowMatrixRenderPlan {
        let proof = self.proof_snapshot();
        ValidationManualFlowMatrixProjection::new(
            manual_matrix_style(&proof),
            ValidationManualFlowMatrixSnapshot::from_proof(&proof, self.last_executed_flow),
        )
        .into_render_plan()
    }
}
impl App for ValidationWorkbenchApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        ctx.request_repaint_after(Duration::from_millis(250));
        self.apply_next_reload_tick();
        let frame = self.workbench.header_frame_plan().execute_frame();
        let header_actions =
            render_header_only(ctx, frame.menu(), frame.theme(), frame.appearance());
        for action in header_actions {
            self.apply_header_selection_action(action);
        }

        let primitive_surface_id =
            SurfaceId::new("worth.surface.preview.primitive.proof").expect("valid surface id");
        let inner_primitive_surface_id =
            SurfaceId::new("worth.surface.preview.primitive.inner").expect("valid surface id");
        let primitive_receipt = self
            .workbench
            .runtime()
            .resolve_primitive_proof(&primitive_surface_id);
        let inner_primitive_receipt = self
            .workbench
            .runtime()
            .resolve_primitive_proof(&inner_primitive_surface_id);
        let mut primitive_clicked_surface_ids = Vec::new();
        CentralPanel::default().show(ctx, |ui| {
            primitive_clicked_surface_ids = render_centered_primitive_proof(
                ui,
                primitive_receipt.as_ref(),
                inner_primitive_receipt.as_ref(),
                self.last_primitive_interaction.as_ref(),
                self.last_primitive_interaction_denial.as_deref(),
            );
        });
        for surface_id in primitive_clicked_surface_ids {
            self.apply_mounted_primitive_primary_click(&surface_id);
        }
    }
}
impl ValidationWorkbenchApp {
    fn apply_header_selection_action(&mut self, action: ValidationHeaderSelectionAction) {
        self.apply_dropdown_command_selection(action.projection_id(), action.command_id());
    }

    fn apply_dropdown_command_selection(&mut self, projection_id: &str, command_id: &str) {
        let projection_id = CommandProjectionId::new(projection_id).expect("valid projection id");
        let command_id = CommandId::new(command_id).expect("valid command id");
        let _ = self
            .workbench
            .select_dropdown_command(&projection_id, &command_id);
    }

    fn apply_next_reload_tick(&mut self) {
        let outcome = self
            .workbench
            .apply_reload_tick(self.reload_loop.poll_inputs());
        self.evidence_log
            .record_runtime_reload_tick_outcome(outcome);
    }

    fn apply_next_reload_tick_with_capture(&mut self) -> Option<ValidationCapturedAuthoredBatch> {
        let tick = self.reload_loop.poll_inputs();
        let captured = match &tick {
            ValidationReloadTick::Changed(ValidationReloadInput::ObservedAuthoredBatch(batch)) => {
                Some(ValidationCapturedAuthoredBatch::new(batch.clone()))
            }
            _ => None,
        };
        let outcome = self.workbench.apply_reload_tick(tick);
        self.evidence_log
            .record_runtime_reload_tick_outcome(outcome);
        captured
    }
}

fn manual_matrix_style(proof: &ValidationAppProofSnapshot) -> ValidationHeaderAppliedStyleReceipt {
    proof.header().applied_style().clone()
}
