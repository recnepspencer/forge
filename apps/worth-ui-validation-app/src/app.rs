use eframe::{App, Frame, NativeOptions};
use egui::{CentralPanel, Context};
use std::path::PathBuf;
use std::time::Duration;
use worth_ui::facade::{WorthUiHeaderMenuPlan, WorthUiHeaderThemePlan};

use crate::header::render_header_only;
use crate::launch::PreparedValidationWorkbenchLaunch;
use crate::reload::{
    ValidationReloadEvidenceEntry, ValidationReloadEvidenceLog, ValidationReloadLoop,
    ValidationReloadLoopConfig,
};
use crate::runtime_workbench::ValidationRuntimeWorkbench;

pub struct ValidationWorkbenchApp {
    workbench: ValidationRuntimeWorkbench,
    reload_loop: ValidationReloadLoop,
    evidence_log: ValidationReloadEvidenceLog,
}

impl ValidationWorkbenchApp {
    pub fn new(launch: PreparedValidationWorkbenchLaunch) -> Self {
        Self {
            workbench: launch.into_runtime_workbench(),
            reload_loop: ValidationReloadLoop::start(
                ValidationReloadLoopConfig::new(default_header_theme_path())
                    .with_source_path(default_validation_source_path())
                    .with_command_path(default_header_command_path())
                    .with_command_projection_path(default_header_command_projection_path()),
            )
            .expect("validation reload loop should observe the header theme file"),
            evidence_log: ValidationReloadEvidenceLog::default(),
        }
    }

    pub fn run_native(launch: PreparedValidationWorkbenchLaunch) -> eframe::Result<()> {
        eframe::run_native(
            "Worth UI Validation App",
            NativeOptions::default(),
            Box::new(|_| Ok(Box::new(Self::new(launch)))),
        )
    }

    pub fn header_plan(&self) -> &WorthUiHeaderMenuPlan {
        self.workbench.header_frame_plan().menu_plan()
    }

    pub fn header_theme_plan(&self) -> &WorthUiHeaderThemePlan {
        self.workbench.header_frame_plan().theme_plan()
    }
}

impl App for ValidationWorkbenchApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        ctx.request_repaint_after(Duration::from_millis(250));
        self.apply_next_reload_tick();
        let frame = self.workbench.header_frame_plan().execute_frame();
        render_header_only(ctx, frame.menu(), frame.theme());
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Worth UI reload evidence");
            render_reload_evidence_log(ui, &self.evidence_log);
        });
    }
}

impl ValidationWorkbenchApp {
    fn apply_next_reload_tick(&mut self) {
        let outcome = self
            .workbench
            .apply_reload_tick(self.reload_loop.poll_inputs());
        self.evidence_log
            .record_runtime_reload_tick_outcome(outcome);
    }
}

fn render_reload_evidence_log(ui: &mut egui::Ui, evidence_log: &ValidationReloadEvidenceLog) {
    if evidence_log.entries().is_empty() {
        ui.label("No reload evidence yet.");
        return;
    }

    for entry in evidence_log.entries().iter().rev() {
        render_reload_evidence_entry(ui, entry);
        ui.separator();
    }
}

fn render_reload_evidence_entry(ui: &mut egui::Ui, entry: &ValidationReloadEvidenceEntry) {
    match entry {
        ValidationReloadEvidenceEntry::RuntimeReload {
            status,
            active_artifact_digest,
            active_plan_digest,
            header_rebind_status,
        } => {
            ui.label(format!("Reload status: {status:?}"));
            ui.label(format!("Active artifact: {active_artifact_digest}"));
            ui.label(format!("Active plan: {active_plan_digest}"));
            ui.label(format!("Header rebind: {header_rebind_status:?}"));
        }
        ValidationReloadEvidenceEntry::ThemeDenied(denial) => {
            ui.label("Theme reload denied");
            ui.label(format!(
                "Theme source digest: {}",
                denial.theme_source_digest()
            ));
            ui.label(format!("Reason: {:?}", denial.reason()));
        }
        ValidationReloadEvidenceEntry::ThemeReload {
            status,
            active_snapshot_digest,
            touched_theme_token_count,
            header_rebind_status,
        } => {
            ui.label(format!("Theme reload status: {status:?}"));
            ui.label(format!("Active snapshot: {active_snapshot_digest}"));
            ui.label(format!("Touched tokens: {touched_theme_token_count}"));
            ui.label(format!("Header rebind: {header_rebind_status:?}"));
        }
        ValidationReloadEvidenceEntry::CommandReload {
            status,
            active_snapshot_digest,
            touched_command_count,
            header_rebind_status,
        } => {
            ui.label(format!("Command reload status: {status:?}"));
            ui.label(format!("Active snapshot: {active_snapshot_digest}"));
            ui.label(format!("Touched commands: {touched_command_count}"));
            ui.label(format!("Header rebind: {header_rebind_status:?}"));
        }
        ValidationReloadEvidenceEntry::CommandProjectionReload {
            status,
            active_snapshot_digest,
            touched_projection_count,
            header_rebind_status,
        } => {
            ui.label(format!("Command projection reload status: {status:?}"));
            ui.label(format!("Active snapshot: {active_snapshot_digest}"));
            ui.label(format!("Touched projections: {touched_projection_count}"));
            ui.label(format!("Header rebind: {header_rebind_status:?}"));
        }
        ValidationReloadEvidenceEntry::SourceActivationDenied(stage) => {
            ui.label("Source activation denied");
            ui.label(format!("Denied stage: {stage:?}"));
        }
        ValidationReloadEvidenceEntry::ThemeActivationDenied(stage) => {
            ui.label("Theme activation denied");
            ui.label(format!("Denied stage: {stage:?}"));
        }
        ValidationReloadEvidenceEntry::CommandActivationDenied(stage) => {
            ui.label("Command activation denied");
            ui.label(format!("Denied stage: {stage:?}"));
        }
        ValidationReloadEvidenceEntry::CommandProjectionActivationDenied(stage) => {
            ui.label("Command projection activation denied");
            ui.label(format!("Denied stage: {stage:?}"));
        }
        ValidationReloadEvidenceEntry::InputUnreadable(denial) => {
            ui.label("Reload input unreadable");
            ui.label(format!("Path: {}", denial.path().display()));
            ui.label(format!("Reason: {}", denial.reason()));
        }
    }
}

fn default_header_theme_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("theme/header.theme")
}

fn default_header_command_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("theme/header.commands")
}

fn default_header_command_projection_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("theme/header.projections")
}

fn default_validation_source_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("source/header.wui")
}
