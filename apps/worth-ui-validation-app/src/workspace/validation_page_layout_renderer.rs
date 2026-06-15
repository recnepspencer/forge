use egui::{CentralPanel, Color32, Frame, ScrollArea, SidePanel, TopBottomPanel, Ui};
use worth_ui::facade::{WorthUiLayoutAxis, WorthUiLayoutTopologyChild, WorthUiLayoutTopologyNode};

use crate::{
    runtime::{PreparedValidationWorkbenchLaunch, ValidationWorkbenchSnapshot},
    workspace::{validation_page_slot_renderer, ValidationWorkspaceState},
};

use super::{
    validation_page_layout_sizing::{
        central_child_index, child_region_sizing, child_resizable, total_flex_weight,
    },
    ValidationResolvedPage,
};

pub(crate) fn render_page_host(
    ui: &mut Ui,
    launch: &PreparedValidationWorkbenchLaunch,
    snapshot: ValidationWorkbenchSnapshot,
    state: &mut ValidationWorkspaceState,
) {
    let page = ValidationResolvedPage::from_state(state);
    let page_name = page.authoring_page_name(state);
    let layout = launch
        .layout_topology()
        .page(page_name)
        .expect("validation authoring sample should contain a layout for every reachable page");

    Frame::new().fill(ui.visuals().panel_fill).show(ui, |ui| {
        ui.set_min_size(ui.available_size());
        render_region(
            ui,
            launch,
            page_name,
            layout.root(),
            &page,
            snapshot,
            state,
            "root",
            0,
        );
    });
}

fn render_region(
    ui: &mut Ui,
    launch: &PreparedValidationWorkbenchLaunch,
    page_name: &str,
    node: &WorthUiLayoutTopologyNode,
    page: &ValidationResolvedPage,
    snapshot: ValidationWorkbenchSnapshot,
    state: &mut ValidationWorkspaceState,
    route: &str,
    depth: usize,
) {
    let frame = region_frame(ui, depth);
    frame.show(ui, |ui| {
        ui.set_min_size(ui.available_size());
        if node.scroll_owner() {
            ScrollArea::both()
                .id_salt(format!("{page_name}.{route}.scroll"))
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.set_min_size(ui.available_size());
                    render_axis_region(
                        ui, launch, page_name, node, page, snapshot, state, route, depth,
                    );
                });
        } else {
            render_axis_region(
                ui, launch, page_name, node, page, snapshot, state, route, depth,
            );
        }
    });
}

fn render_axis_region(
    ui: &mut Ui,
    launch: &PreparedValidationWorkbenchLaunch,
    page_name: &str,
    node: &WorthUiLayoutTopologyNode,
    page: &ValidationResolvedPage,
    snapshot: ValidationWorkbenchSnapshot,
    state: &mut ValidationWorkspaceState,
    route: &str,
    depth: usize,
) {
    let children = node.children();
    if children.is_empty() {
        ui.label("Empty layout region");
        return;
    }

    let pivot = central_child_index(children);
    let total_flex = total_flex_weight(children).max(1);

    match node.axis() {
        WorthUiLayoutAxis::Row => {
            for index in 0..pivot {
                let child = &children[index];
                let panel_id = format!("{page_name}.{route}.left.{index}");
                let panel = configure_side_panel(
                    SidePanel::left(panel_id),
                    child,
                    ui.available_width(),
                    total_flex,
                    launch,
                );
                panel.show_inside(ui, |ui| {
                    render_child(
                        ui,
                        launch,
                        page_name,
                        child,
                        page,
                        snapshot,
                        state,
                        format!("{route}/{index}").as_str(),
                        depth + 1,
                    );
                });
            }

            for index in (pivot + 1..children.len()).rev() {
                let child = &children[index];
                let panel_id = format!("{page_name}.{route}.right.{index}");
                let panel = configure_side_panel(
                    SidePanel::right(panel_id),
                    child,
                    ui.available_width(),
                    total_flex,
                    launch,
                );
                panel.show_inside(ui, |ui| {
                    render_child(
                        ui,
                        launch,
                        page_name,
                        child,
                        page,
                        snapshot,
                        state,
                        format!("{route}/{index}").as_str(),
                        depth + 1,
                    );
                });
            }
        }
        WorthUiLayoutAxis::Column => {
            for index in 0..pivot {
                let child = &children[index];
                let panel_id = format!("{page_name}.{route}.top.{index}");
                let panel = configure_top_panel(
                    TopBottomPanel::top(panel_id),
                    child,
                    ui.available_height(),
                    total_flex,
                    launch,
                );
                panel.show_inside(ui, |ui| {
                    render_child(
                        ui,
                        launch,
                        page_name,
                        child,
                        page,
                        snapshot,
                        state,
                        format!("{route}/{index}").as_str(),
                        depth + 1,
                    );
                });
            }

            for index in (pivot + 1..children.len()).rev() {
                let child = &children[index];
                let panel_id = format!("{page_name}.{route}.bottom.{index}");
                let panel = configure_top_panel(
                    TopBottomPanel::bottom(panel_id),
                    child,
                    ui.available_height(),
                    total_flex,
                    launch,
                );
                panel.show_inside(ui, |ui| {
                    render_child(
                        ui,
                        launch,
                        page_name,
                        child,
                        page,
                        snapshot,
                        state,
                        format!("{route}/{index}").as_str(),
                        depth + 1,
                    );
                });
            }
        }
    }

    CentralPanel::default()
        .frame(region_frame(ui, depth + 1))
        .show_inside(ui, |ui| {
            render_child(
                ui,
                launch,
                page_name,
                &children[pivot],
                page,
                snapshot,
                state,
                format!("{route}/{pivot}").as_str(),
                depth + 1,
            );
        });
}

fn render_child(
    ui: &mut Ui,
    launch: &PreparedValidationWorkbenchLaunch,
    page_name: &str,
    child: &WorthUiLayoutTopologyChild,
    page: &ValidationResolvedPage,
    snapshot: ValidationWorkbenchSnapshot,
    state: &mut ValidationWorkspaceState,
    route: &str,
    depth: usize,
) {
    match child {
        WorthUiLayoutTopologyChild::Region(node) => {
            render_region(
                ui, launch, page_name, node, page, snapshot, state, route, depth,
            );
        }
        WorthUiLayoutTopologyChild::Slot(slot) => {
            validation_page_slot_renderer::render_slot(ui, page, slot.slot_name(), snapshot, state);
        }
    }
}

fn configure_side_panel(
    panel: SidePanel,
    child: &WorthUiLayoutTopologyChild,
    available: f32,
    total_flex: u32,
    launch: &PreparedValidationWorkbenchLaunch,
) -> SidePanel {
    let sizing = child_region_sizing(child, available, total_flex, launch.layout_measurements());
    let min = sizing
        .as_ref()
        .and_then(|sizing| sizing.min_size)
        .unwrap_or(160.0);
    let max = sizing
        .as_ref()
        .and_then(|sizing| sizing.max_size)
        .unwrap_or(available.max(min));
    let default = sizing
        .as_ref()
        .and_then(|sizing| sizing.default_size)
        .unwrap_or((available * 0.33).clamp(min, max));
    let panel = panel
        .frame(Frame::new().fill(Color32::from_gray(28)))
        .show_separator_line(true)
        .min_width(min)
        .max_width(max)
        .default_width(default)
        .resizable(child_resizable(child));

    match sizing.and_then(|sizing| sizing.exact_size) {
        Some(exact) if !child_resizable(child) => panel.exact_width(exact),
        _ => panel,
    }
}

fn configure_top_panel(
    panel: TopBottomPanel,
    child: &WorthUiLayoutTopologyChild,
    available: f32,
    total_flex: u32,
    launch: &PreparedValidationWorkbenchLaunch,
) -> TopBottomPanel {
    let sizing = child_region_sizing(child, available, total_flex, launch.layout_measurements());
    let min = sizing
        .as_ref()
        .and_then(|sizing| sizing.min_size)
        .unwrap_or(72.0);
    let max = sizing
        .as_ref()
        .and_then(|sizing| sizing.max_size)
        .unwrap_or(available.max(min));
    let default = sizing
        .as_ref()
        .and_then(|sizing| sizing.default_size)
        .unwrap_or((available * 0.25).clamp(min, max));
    let panel = panel
        .frame(Frame::new().fill(Color32::from_gray(26)))
        .show_separator_line(true)
        .min_height(min)
        .max_height(max)
        .default_height(default)
        .resizable(child_resizable(child));

    match sizing.and_then(|sizing| sizing.exact_size) {
        Some(exact) if !child_resizable(child) => panel.exact_height(exact),
        _ => panel,
    }
}
fn region_frame(ui: &Ui, depth: usize) -> Frame {
    let depth_tint = (depth as u8).saturating_mul(4);
    Frame::new()
        .fill(Color32::from_gray(22u8.saturating_add(depth_tint)))
        .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
        .inner_margin(8.0)
}
