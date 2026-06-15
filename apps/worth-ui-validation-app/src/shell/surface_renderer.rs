use egui::{CentralPanel, Context, Frame, SidePanel, TopBottomPanel};

use crate::commands::ShellCommandRegistry;
use crate::pages::surface_atlas::SurfaceAtlasPage;
use crate::pages::{workbench, ValidationPageRegistry};
use crate::shell::{StableShellSurfaceId, StableShellSurfaceManifest, ValidationRunSummary};
use crate::theme::ValidationWorkbenchTheme;

pub struct ShellSurfaceRenderer<'a> {
    theme: &'a ValidationWorkbenchTheme,
    surface_atlas: &'a SurfaceAtlasPage,
    pages: ValidationPageRegistry,
    commands: ShellCommandRegistry,
}

impl<'a> ShellSurfaceRenderer<'a> {
    pub fn new(
        theme: &'a ValidationWorkbenchTheme,
        surface_atlas: &'a SurfaceAtlasPage,
        pages: ValidationPageRegistry,
        commands: ShellCommandRegistry,
    ) -> Self {
        Self {
            theme,
            surface_atlas,
            pages,
            commands,
        }
    }

    pub fn render_manifest_surfaces(&self, ctx: &Context, summary: &ValidationRunSummary) {
        for surface in StableShellSurfaceManifest::REQUIRED.surfaces() {
            self.render_surface(ctx, summary, surface.id());
        }
    }

    fn render_surface(
        &self,
        ctx: &Context,
        summary: &ValidationRunSummary,
        surface_id: StableShellSurfaceId,
    ) {
        match surface_id {
            StableShellSurfaceId::MENU_BAR => self.render_menu_bar(ctx),
            StableShellSurfaceId::TOOLBAR => self.render_toolbar(ctx),
            StableShellSurfaceId::ACTIVITY_RAIL => self.render_activity_rail(ctx),
            StableShellSurfaceId::SCENARIO_NAV => self.render_scenario_nav(ctx, summary),
            StableShellSurfaceId::COMMAND_PALETTE => {}
            StableShellSurfaceId::INSPECTOR => self.render_inspector(ctx, summary),
            StableShellSurfaceId::BOTTOM_TIMELINE => self.render_bottom_timeline(ctx),
            StableShellSurfaceId::STATUS_BAR => self.render_status_bar(ctx, summary),
            StableShellSurfaceId::PAGE_HOST => self.render_page_host(ctx, summary),
            StableShellSurfaceId::EDITOR_TABS => {}
            StableShellSurfaceId::OVERLAY_LAYER => self.render_overlay_layer(ctx),
            _ => {}
        }
    }

    fn render_menu_bar(&self, ctx: &Context) {
        TopBottomPanel::top(StableShellSurfaceId::MENU_BAR.as_str()).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.strong("Worth UI");
                for command in self.commands.commands() {
                    ui.label(command.label());
                }
            });
        });
    }

    fn render_toolbar(&self, ctx: &Context) {
        TopBottomPanel::top(StableShellSurfaceId::TOOLBAR.as_str()).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.push_id(StableShellSurfaceId::COMMAND_PALETTE.as_str(), |ui| {
                    let _ = ui.button("Palette");
                });
                let _ = ui.button("Run");
                let _ = ui.button("Replay");
                let _ = ui.button("Inspect");
            });
        });
    }

    fn render_activity_rail(&self, ctx: &Context) {
        SidePanel::left(StableShellSurfaceId::ACTIVITY_RAIL.as_str())
            .resizable(false)
            .exact_width(48.0)
            .frame(Frame::NONE.fill(self.theme.activity_bar()))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label("W");
                    ui.label("R");
                    ui.label("E");
                });
            });
    }

    fn render_scenario_nav(&self, ctx: &Context, summary: &ValidationRunSummary) {
        SidePanel::left(StableShellSurfaceId::SCENARIO_NAV.as_str())
            .resizable(true)
            .default_width(220.0)
            .frame(Frame::NONE.fill(self.theme.sidebar()))
            .show(ctx, |ui| {
                ui.heading("Scenarios");
                ui.monospace(summary.selected_scenario());
                for page in self.pages.pages() {
                    ui.label(page.label());
                }
            });
    }

    fn render_inspector(&self, ctx: &Context, summary: &ValidationRunSummary) {
        SidePanel::right(StableShellSurfaceId::INSPECTOR.as_str())
            .resizable(true)
            .default_width(280.0)
            .frame(Frame::NONE.fill(self.theme.panel()))
            .show(ctx, |ui| {
                ui.heading("Runtime");
                ui.label("Active plan");
                ui.monospace(
                    summary
                        .runtime_observation()
                        .active_plan_digest()
                        .to_string(),
                );
            });
    }

    fn render_bottom_timeline(&self, ctx: &Context) {
        TopBottomPanel::bottom(StableShellSurfaceId::BOTTOM_TIMELINE.as_str())
            .resizable(true)
            .default_height(118.0)
            .frame(Frame::NONE.fill(self.theme.panel_raised()))
            .show(ctx, |ui| {
                ui.heading("Timeline");
                ui.label("No live scenario run yet");
            });
    }

    fn render_status_bar(&self, ctx: &Context, summary: &ValidationRunSummary) {
        TopBottomPanel::bottom(StableShellSurfaceId::STATUS_BAR.as_str()).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Native");
                ui.separator();
                ui.label("Plan");
                ui.monospace(
                    summary
                        .runtime_observation()
                        .active_plan_digest()
                        .to_string(),
                );
            });
        });
    }

    fn render_page_host(&self, ctx: &Context, summary: &ValidationRunSummary) {
        CentralPanel::default()
            .frame(Frame::NONE.fill(self.theme.editor_canvas()))
            .show(ctx, |ui| {
                ui.push_id(StableShellSurfaceId::EDITOR_TABS.as_str(), |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Surface atlas");
                        ui.separator();
                        ui.label("Scenario runs");
                        ui.separator();
                        ui.label("Evidence");
                    });
                });
                ui.separator();
                ui.push_id(StableShellSurfaceId::PAGE_HOST.as_str(), |ui| {
                    workbench::render_workbench_page(ui, self.theme, self.surface_atlas, summary);
                });
            });
    }

    fn render_overlay_layer(&self, ctx: &Context) {
        egui::Area::new(StableShellSurfaceId::OVERLAY_LAYER.as_str().into())
            .anchor(egui::Align2::RIGHT_TOP, [-24.0, 64.0])
            .show(ctx, |ui| {
                ui.visuals_mut().widgets.noninteractive.bg_fill = self.theme.panel_raised();
                ui.push_id(StableShellSurfaceId::COMMAND_PALETTE.as_str(), |ui| {
                    ui.label("Surface atlas");
                    ui.separator();
                    ui.label("Command palette affordance");
                });
            });
    }
}
