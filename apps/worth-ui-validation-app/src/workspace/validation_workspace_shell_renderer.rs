use eframe::egui;
use egui::{Align, Area, Context, Frame, Layout, Order, RichText, SidePanel, TopBottomPanel};

use crate::runtime::ValidationWorkbenchSnapshot;

use super::{
    validation_page_catalog::static_pages, ValidationDynamicPageRequest, ValidationWorkspaceShell,
};

pub(crate) fn render(shell: &mut ValidationWorkspaceShell, ctx: &Context) {
    ctx.set_visuals(egui::Visuals::dark());
    let snapshot = shell.snapshot();

    let topbar = render_topbar(shell, ctx);
    let rail = render_rail(shell, ctx);
    let inspector = render_inspector(shell, ctx, snapshot);
    let status = render_status(shell, ctx, snapshot);

    render_page_panel(shell, ctx, snapshot);
    render_command_palette(shell, ctx);
    render_toasts(shell, ctx);

    shell.state_mut().set_rail_width(rail);
    shell.state_mut().set_inspector_width(inspector);
    shell.state_mut().set_status_height(status);
    let _ = topbar;
}

fn render_topbar(shell: &mut ValidationWorkspaceShell, ctx: &Context) -> f32 {
    let sample = shell.launch().sample();
    TopBottomPanel::top("validation.workspace.topbar")
        .resizable(false)
        .show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.heading(sample.workspace_title());
                ui.label(RichText::new(sample.shell().topbar()).monospace());
                ui.separator();
                for page in static_pages() {
                    let selected = shell.state().navigation().active_page()
                        == super::ValidationPageHandle::Static(page.id());
                    if ui.selectable_label(selected, page.title()).clicked() {
                        shell.select_static_page(page.id());
                    }
                }
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui.button("Palette").clicked() {
                        shell.state_mut().toggle_command_palette();
                    }
                });
            });
        })
        .response
        .rect
        .height()
}

fn render_rail(shell: &mut ValidationWorkspaceShell, ctx: &Context) -> f32 {
    let sample = shell.launch().sample();
    SidePanel::left("validation.workspace.rail")
        .default_width(shell.state().rail_width())
        .resizable(true)
        .show(ctx, |ui| {
            ui.label(RichText::new(sample.shell().rail()).strong());
            ui.add_space(8.0);
            for page in static_pages() {
                let selected = shell.state().navigation().active_page()
                    == super::ValidationPageHandle::Static(page.id());
                if ui.selectable_label(selected, page.title()).clicked() {
                    shell.select_static_page(page.id());
                }
                ui.small(page.summary());
                ui.add_space(8.0);
            }
        })
        .response
        .rect
        .width()
}

fn render_inspector(
    shell: &mut ValidationWorkspaceShell,
    ctx: &Context,
    snapshot: ValidationWorkbenchSnapshot,
) -> f32 {
    let sample = shell.launch().sample();
    SidePanel::right("validation.workspace.inspector")
        .default_width(shell.state().inspector_width())
        .resizable(true)
        .show(ctx, |ui| {
            ui.label(RichText::new(sample.shell().inspector()).strong());
            ui.add_space(8.0);
            ui.label(format!(
                "Active page: {}",
                shell.state().navigation().active_page_title()
            ));
            if let Some(parameter_badge) = shell.state().navigation().active_parameter_badge() {
                ui.monospace(parameter_badge);
            }
            ui.separator();
            ui.label(format!("Artifact: {}", snapshot.artifact_digest()));
            ui.label(format!("Plan: {}", snapshot.active_plan_digest()));
            ui.label(format!("Theme: {}", sample.theme_name()));
            ui.separator();
            ui.label(RichText::new("Open dynamic pages").strong());
            let open_pages = shell.state().navigation().open_dynamic_pages().to_vec();
            for page in open_pages {
                ui.horizontal(|ui| {
                    if ui.selectable_label(false, page.title()).clicked() {
                        shell
                            .state_mut()
                            .navigation_mut()
                            .select_dynamic_page(page.handle());
                    }
                    if ui.small_button("x").clicked() {
                        shell
                            .state_mut()
                            .navigation_mut()
                            .close_dynamic_page(page.handle());
                    }
                });
            }
        })
        .response
        .rect
        .width()
}

fn render_status(
    shell: &mut ValidationWorkspaceShell,
    ctx: &Context,
    snapshot: ValidationWorkbenchSnapshot,
) -> f32 {
    let sample = shell.launch().sample();
    TopBottomPanel::bottom("validation.workspace.status")
        .default_height(shell.state().status_height())
        .min_height(88.0)
        .max_height(240.0)
        .resizable(true)
        .show(ctx, |ui| {
            let frame_fill = ui.visuals().faint_bg_color;
            Frame::new().fill(frame_fill).show(ui, |ui| {
                let available = ui.available_size();
                ui.set_min_size(available);
                ui.vertical(|ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new(sample.shell().status()).strong());
                        ui.separator();
                        ui.label(format!("App: {}", snapshot.app_name()));
                        ui.label(format!("Workspace: {}", snapshot.workspace_name()));
                        ui.label(format!("Commands: {}", sample.commands().commands().len()));
                    });
                    ui.separator();
                    ui.horizontal_wrapped(|ui| {
                        if ui.button("Open Product P-1001").clicked() {
                            let request = ValidationDynamicPageRequest::product_detail("P-1001")
                                .expect("sample product detail request should be valid");
                            shell
                                .state_mut()
                                .navigation_mut()
                                .open_dynamic_page(request)
                                .expect("sample product detail request should open");
                        }
                        if ui.button("Open Order O-4902").clicked() {
                            let request = ValidationDynamicPageRequest::order_detail("O-4902")
                                .expect("sample order detail request should be valid");
                            shell
                                .state_mut()
                                .navigation_mut()
                                .open_dynamic_page(request)
                                .expect("sample order detail request should open");
                        }
                        ui.label(format!(
                            "Open detail tabs: {}",
                            shell.state().navigation().open_dynamic_pages().len()
                        ));
                    });
                    ui.add_space(8.0);
                    ui.label("Workspace-owned status regions should grow with their shell allocation, not collapse back to intrinsic content height.");
                });
            });
        })
        .response
        .rect
        .height()
}

fn render_page_panel(
    shell: &mut ValidationWorkspaceShell,
    ctx: &Context,
    snapshot: ValidationWorkbenchSnapshot,
) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.label(RichText::new(shell.launch().sample().shell().page_host()).strong());
        ui.separator();
        shell.render_page_host(ui, snapshot);
    });
}

fn render_command_palette(shell: &mut ValidationWorkspaceShell, ctx: &Context) {
    if !shell.state().command_palette_open() {
        return;
    }

    Area::new("validation.workspace.command-palette".into())
        .order(Order::Foreground)
        .fixed_pos((220.0, 72.0))
        .show(ctx, |ui| {
            Frame::popup(ui.style()).show(ui, |ui| {
                ui.label(RichText::new("CommandPaletteOverlay").strong());
                ui.add_space(6.0);
                for command in shell.launch().sample().commands().commands() {
                    ui.monospace(format!("{} -> {}", command.id(), command.label()));
                }
                if ui.button("Close").clicked() {
                    shell.state_mut().close_command_palette();
                }
            });
        });
}

fn render_toasts(shell: &mut ValidationWorkspaceShell, ctx: &Context) {
    let sample = shell.launch().sample();
    let messages: Vec<String> = shell
        .state()
        .toasts()
        .iter()
        .map(|toast| toast.message().to_owned())
        .collect();
    if messages.is_empty() {
        return;
    }

    Area::new("validation.workspace.toasts".into())
        .order(Order::Foreground)
        .anchor(egui::Align2::RIGHT_BOTTOM, (-16.0, -16.0))
        .show(ctx, |ui| {
            Frame::popup(ui.style()).show(ui, |ui| {
                ui.label(RichText::new(sample.shell().toasts()).strong());
                for message in messages {
                    ui.label(message);
                }
            });
        });
}
