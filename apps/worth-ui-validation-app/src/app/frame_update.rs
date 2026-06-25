use super::live_view::render_live_view_state_proof;
use super::ValidationWorkbenchApp;
use crate::header::render_header_only;
use eframe::{App, Frame};
use egui::{CentralPanel, Context};
use std::time::Duration;

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

        let live_view_receipt = self.live_view_projection_proof();
        let mut live_view_intents = Default::default();
        CentralPanel::default().show(ctx, |ui| {
            live_view_intents = render_live_view_state_proof(
                ui,
                self.workbench.runtime(),
                live_view_receipt.as_ref().map_err(String::as_str),
                self.last_live_view_edit.as_ref(),
                self.last_live_view_edit_denial.as_ref(),
                self.last_live_view_submission.as_ref(),
                self.last_live_view_submission_denial.as_ref(),
                self.last_live_view_source_denial.as_deref(),
            );
        });
        for intent in live_view_intents.state_edits {
            match self
                .workbench
                .runtime_mut()
                .apply_live_view_state_edit(intent)
            {
                Ok(receipt) => {
                    let _ = self
                        .workbench
                        .runtime()
                        .admit_live_view_state_runtime_change(&receipt);
                    self.last_live_view_edit = Some(receipt);
                    self.last_live_view_edit_denial = None;
                }
                Err(denial) => {
                    self.last_live_view_edit = None;
                    self.last_live_view_edit_denial = Some(denial);
                }
            }
        }
        for interaction in live_view_intents.submissions {
            match self
                .workbench
                .runtime()
                .activate_mounted_live_view_interaction(&interaction)
            {
                Ok(eligible) => {
                    let receipt = self
                        .workbench
                        .runtime()
                        .submit_live_view_interaction(eligible);
                    self.last_live_view_submission = Some(receipt);
                    self.last_live_view_submission_denial = None;
                }
                Err(denial) => {
                    self.last_live_view_submission = None;
                    self.last_live_view_submission_denial = Some(denial);
                }
            }
        }
    }
}
