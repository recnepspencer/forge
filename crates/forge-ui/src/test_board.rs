//! Component Test Board — a Figma-style canvas for previewing all UI elements.
//!
//! Navigate here via the "Test Board" tab. Press Escape to return to the editor.

use eframe::egui;
use egui::{Color32, CornerRadius, Frame, Stroke, Vec2};
use forge_ui_components::{
    fg_button, fg_card, fg_form, fg_input, fg_modal, fg_textarea, fg_dropdown, fg_alert,
    FgButton, FgButtonVariant, FgButtonSize, FgCard, FgIcon, IconStore,
    FgInput, FgTextArea, FgDropdown, DropdownItem, DropdownState, FgAlert, FgAlertVariant,
};
use forge_ui_state::AppState;

/// Per-session state for the test board demos.
/// Using a thread-local so we don't pollute AppState.
#[derive(Default)]
struct TestBoardState {
    show_modal: bool,
    modal_form_name: String,
    modal_form_desc: String,

    alert_visible: bool,
    alert_variant: FgAlertVariant,

    input_demo: String,
    textarea_demo: String,

    dropdown_state: DropdownState,
    dropdown_selected: Option<String>,

    form_name: String,
    form_email: String,
    form_notes: String,
    form_role_state: DropdownState,
    form_role_selected: Option<String>,
}

std::thread_local! {
    static DEMO: std::cell::RefCell<TestBoardState> = std::cell::RefCell::new(TestBoardState::default());
}

/// Draw the full-page component test board.
pub fn draw_test_board(ctx: &egui::Context, state: &mut AppState, icons: &IconStore) {
    let t = state.theme.clone();

    DEMO.with(|cell| {
        let demo = &mut *cell.borrow_mut();

        egui::CentralPanel::default()
            .frame(Frame::new().fill(t.bg_base))
            .show(ctx, |ui| {
                egui::ScrollArea::both().show(ui, |ui| {
                    ui.add_space(24.0);

                    // ── Page header ──────────────────────────────────────
                    ui.horizontal(|ui| {
                        ui.add_space(32.0);
                        ui.label(egui::RichText::new("Component Test Board")
                            .color(t.text_primary).size(t.font_size_xl).strong());
                    });
                    ui.add_space(16.0);

                    let margin = egui::Margin { left: 32, right: 32, top: 0, bottom: 0 };

                    // ══════════════════════════════════════════════════════
                    // §1 — BUTTONS (with click feedback)
                    // ══════════════════════════════════════════════════════
                    section_header(ui, &t, "Buttons — Click to see pressed state");
                    Frame::new()
                        .fill(t.bg_surface).corner_radius(CornerRadius::same(t.radius_md as u8))
                        .inner_margin(egui::Margin::same(16)).outer_margin(margin)
                        .stroke(Stroke::new(1.0, t.border_subtle))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                fg_button(ui, &t, icons, FgButton::new("Primary"));
                                ui.add_space(8.0);
                                fg_button(ui, &t, icons, FgButton::new("Secondary").variant(FgButtonVariant::Secondary));
                                ui.add_space(8.0);
                                fg_button(ui, &t, icons, FgButton::new("Danger").variant(FgButtonVariant::Danger));
                                ui.add_space(8.0);
                                fg_button(ui, &t, icons, FgButton::new("Ghost").variant(FgButtonVariant::Ghost));
                                ui.add_space(8.0);
                                fg_button(ui, &t, icons, FgButton::new("Link").variant(FgButtonVariant::Link));
                            });
                            ui.add_space(8.0);
                            ui.horizontal(|ui| {
                                fg_button(ui, &t, icons, FgButton::new("Small").size(FgButtonSize::Sm));
                                ui.add_space(8.0);
                                fg_button(ui, &t, icons, FgButton::new("Medium").size(FgButtonSize::Md));
                                ui.add_space(8.0);
                                fg_button(ui, &t, icons, FgButton::new("Large").size(FgButtonSize::Lg));
                                ui.add_space(8.0);
                                fg_button(ui, &t, icons, FgButton::new("Disabled").disabled(true));
                                ui.add_space(8.0);
                                fg_button(ui, &t, icons, FgButton::new("Loading").disabled(true));
                            });
                        });
                    ui.add_space(24.0);

                    // ══════════════════════════════════════════════════════
                    // §2 — MODAL DEMO
                    // ══════════════════════════════════════════════════════
                    section_header(ui, &t, "Modal (screen overlay)");
                    Frame::new()
                        .fill(t.bg_surface).corner_radius(CornerRadius::same(t.radius_md as u8))
                        .inner_margin(egui::Margin::same(16)).outer_margin(margin)
                        .stroke(Stroke::new(1.0, t.border_subtle))
                        .show(ui, |ui| {
                            ui.label(egui::RichText::new("Click to open a modal with a form inside:")
                                .color(t.text_secondary).size(t.font_size_sm));
                            ui.add_space(8.0);
                            let resp = fg_button(ui, &t, icons, FgButton::new("Open Modal"));
                            if resp.clicked() { demo.show_modal = true; }
                        });
                    ui.add_space(24.0);

                    // ══════════════════════════════════════════════════════
                    // §3 — ALERTS DEMO
                    // ══════════════════════════════════════════════════════
                    section_header(ui, &t, "Alerts");
                    Frame::new()
                        .fill(t.bg_surface).corner_radius(CornerRadius::same(t.radius_md as u8))
                        .inner_margin(egui::Margin::same(16)).outer_margin(margin)
                        .stroke(Stroke::new(1.0, t.border_subtle))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                for (label, variant) in [
                                    ("Show Success", FgAlertVariant::Success),
                                    ("Show Warning", FgAlertVariant::Warning),
                                    ("Show Error", FgAlertVariant::Error),
                                    ("Show Info", FgAlertVariant::Info),
                                ] {
                                    let v = match variant {
                                        FgAlertVariant::Success => FgButtonVariant::Primary,
                                        FgAlertVariant::Warning => FgButtonVariant::Secondary,
                                        FgAlertVariant::Error   => FgButtonVariant::Danger,
                                        FgAlertVariant::Info    => FgButtonVariant::Ghost,
                                    };
                                    let resp = fg_button(ui, &t, icons, FgButton::new(label).variant(v));
                                    if resp.clicked() {
                                        demo.alert_visible = true;
                                        demo.alert_variant = variant;
                                    }
                                    ui.add_space(4.0);
                                }
                            });
                            ui.add_space(8.0);
                            if demo.alert_visible {
                                let msg = match demo.alert_variant {
                                    FgAlertVariant::Success => "Operation completed successfully!",
                                    FgAlertVariant::Warning => "This action may have side effects.",
                                    FgAlertVariant::Error   => "Something went wrong. Please try again.",
                                    FgAlertVariant::Info    => "Here is some helpful information.",
                                };
                                fg_alert(ui, &t, FgAlert::new(msg).variant(demo.alert_variant));
                                ui.add_space(4.0);
                                let dismiss = fg_button(ui, &t, icons,
                                    FgButton::new("Dismiss").variant(FgButtonVariant::Ghost).size(FgButtonSize::Sm));
                                if dismiss.clicked() { demo.alert_visible = false; }
                            }
                        });
                    ui.add_space(24.0);

                    // ══════════════════════════════════════════════════════
                    // §4 — FORM INPUTS
                    // ══════════════════════════════════════════════════════
                    section_header(ui, &t, "Form Inputs");
                    Frame::new()
                        .fill(t.bg_surface).corner_radius(CornerRadius::same(t.radius_md as u8))
                        .inner_margin(egui::Margin::same(16)).outer_margin(margin)
                        .stroke(Stroke::new(1.0, t.border_subtle))
                        .show(ui, |ui| {
                            fg_input(ui, &t, FgInput::new(&mut demo.input_demo)
                                .label("Text Input")
                                .placeholder("Type something…"));
                            ui.add_space(12.0);
                            fg_textarea(ui, &t, FgTextArea::new(&mut demo.textarea_demo)
                                .label("Text Area")
                                .placeholder("Enter multiple lines…")
                                .rows(3));
                        });
                    ui.add_space(24.0);

                    // ══════════════════════════════════════════════════════
                    // §5 — SEARCHABLE DROPDOWN
                    // ══════════════════════════════════════════════════════
                    section_header(ui, &t, "Searchable Dropdown");
                    Frame::new()
                        .fill(t.bg_surface).corner_radius(CornerRadius::same(t.radius_md as u8))
                        .inner_margin(egui::Margin::same(16)).outer_margin(margin)
                        .stroke(Stroke::new(1.0, t.border_subtle))
                        .show(ui, |ui| {
                            let items = vec![
                                DropdownItem::new("cube", "Cube").group("Primitives"),
                                DropdownItem::new("sphere", "Sphere").group("Primitives"),
                                DropdownItem::new("cylinder", "Cylinder").group("Primitives"),
                                DropdownItem::new("union", "Boolean Union").group("Operations"),
                                DropdownItem::new("subtract", "Boolean Subtract").group("Operations"),
                                DropdownItem::new("intersect", "Boolean Intersect").group("Operations"),
                                DropdownItem::new("fillet", "Fillet").group("Features"),
                                DropdownItem::new("chamfer", "Chamfer").group("Features"),
                                DropdownItem::new("extrude", "Extrude").group("Features"),
                            ];
                            let selection = fg_dropdown(
                                ui, &t, "demo_dropdown",
                                FgDropdown::new(&items, demo.dropdown_selected.as_deref(), &mut demo.dropdown_state)
                                    .label("Select an operation")
                                    .placeholder("Choose…"),
                            );
                            if let Some(id) = selection {
                                demo.dropdown_selected = Some(id);
                            }
                            ui.add_space(8.0);
                            if let Some(ref sel) = demo.dropdown_selected {
                                ui.label(egui::RichText::new(format!("Selected: {sel}"))
                                    .color(t.text_primary).size(t.font_size_sm));
                            }
                        });
                    ui.add_space(24.0);

                    // ══════════════════════════════════════════════════════
                    // §6 — FORM COMPONENT
                    // ══════════════════════════════════════════════════════
                    section_header(ui, &t, "Form Component (with built-in padding + button footer)");
                    Frame::new()
                        .outer_margin(margin)
                        .show(ui, |ui| {
                            let role_items = vec![
                                DropdownItem::new("admin", "Admin"),
                                DropdownItem::new("editor", "Editor"),
                                DropdownItem::new("viewer", "Viewer"),
                            ];
                            fg_form(ui, &t,
                                |ui| {
                                    fg_input(ui, &t, FgInput::new(&mut demo.form_name)
                                        .label("Full Name").placeholder("Jane Doe"));
                                    ui.add_space(12.0);
                                    fg_input(ui, &t, FgInput::new(&mut demo.form_email)
                                        .label("Email").placeholder("jane@forge.dev"));
                                    ui.add_space(12.0);
                                    let sel = fg_dropdown(ui, &t, "form_role",
                                        FgDropdown::new(&role_items, demo.form_role_selected.as_deref(), &mut demo.form_role_state)
                                            .label("Role").placeholder("Select role…"));
                                    if let Some(id) = sel { demo.form_role_selected = Some(id); }
                                    ui.add_space(12.0);
                                    fg_textarea(ui, &t, FgTextArea::new(&mut demo.form_notes)
                                        .label("Notes").placeholder("Additional notes…").rows(2));
                                },
                                |ui| {
                                    fg_button(ui, &t, icons, FgButton::new("Cancel").variant(FgButtonVariant::Ghost));
                                    ui.add_space(8.0);
                                    fg_button(ui, &t, icons, FgButton::new("Save"));
                                },
                            );
                        });
                    ui.add_space(24.0);

                    // ══════════════════════════════════════════════════════
                    // §7 — COLORS
                    // ══════════════════════════════════════════════════════
                    section_header(ui, &t, "Colors");
                    Frame::new()
                        .fill(t.bg_surface).corner_radius(CornerRadius::same(t.radius_md as u8))
                        .inner_margin(egui::Margin::same(16)).outer_margin(margin)
                        .stroke(Stroke::new(1.0, t.border_subtle))
                        .show(ui, |ui| {
                            ui.horizontal_wrapped(|ui| {
                                color_swatch(ui, "bg_base", t.bg_base, &t);
                                color_swatch(ui, "bg_surface", t.bg_surface, &t);
                                color_swatch(ui, "bg_raised", t.bg_raised, &t);
                                color_swatch(ui, "accent", t.accent_primary, &t);
                                color_swatch(ui, "success", t.success, &t);
                                color_swatch(ui, "warning", t.warning, &t);
                                color_swatch(ui, "danger", t.danger, &t);
                                color_swatch(ui, "info", t.info, &t);
                            });
                        });
                    ui.add_space(24.0);

                    // ══════════════════════════════════════════════════════
                    // §8 — TYPOGRAPHY
                    // ══════════════════════════════════════════════════════
                    section_header(ui, &t, "Typography");
                    Frame::new()
                        .fill(t.bg_surface).corner_radius(CornerRadius::same(t.radius_md as u8))
                        .inner_margin(egui::Margin::same(16)).outer_margin(margin)
                        .stroke(Stroke::new(1.0, t.border_subtle))
                        .show(ui, |ui| {
                            ui.label(egui::RichText::new("XL — 20px Heading").size(t.font_size_xl).color(t.text_primary).strong());
                            ui.label(egui::RichText::new("LG — 16px Subheading").size(t.font_size_lg).color(t.text_primary));
                            ui.label(egui::RichText::new("MD — 14px Body").size(t.font_size_md).color(t.text_primary));
                            ui.label(egui::RichText::new("SM — 12px Caption").size(t.font_size_sm).color(t.text_secondary));
                            ui.label(egui::RichText::new("XS — 11px Fine print").size(t.font_size_xs).color(t.text_muted));
                        });
                    ui.add_space(24.0);

                    // ══════════════════════════════════════════════════════
                    // §9 — ICONS
                    // ══════════════════════════════════════════════════════
                    section_header(ui, &t, "Icons (Lucide SVGs)");
                    Frame::new()
                        .fill(t.bg_surface).corner_radius(CornerRadius::same(t.radius_md as u8))
                        .inner_margin(egui::Margin::same(16)).outer_margin(margin)
                        .stroke(Stroke::new(1.0, t.border_subtle))
                        .show(ui, |ui| {
                            let all_icons = [
                                (FgIcon::Plus, "Plus"), (FgIcon::Minus, "Minus"),
                                (FgIcon::Check, "Check"), (FgIcon::X, "X"),
                                (FgIcon::Search, "Search"), (FgIcon::Eye, "Eye"),
                                (FgIcon::PenLine, "PenLine"), (FgIcon::Trash2, "Trash2"),
                                (FgIcon::Box, "Box"), (FgIcon::Layers3, "Layers3"),
                                (FgIcon::Grid2x2, "Grid2x2"), (FgIcon::Ruler, "Ruler"),
                                (FgIcon::Move3d, "Move3d"), (FgIcon::Sun, "Sun"),
                                (FgIcon::Moon, "Moon"), (FgIcon::ChevronRight, "ChevronRight"),
                                (FgIcon::ChevronDown, "ChevronDown"), (FgIcon::MessageSquare, "MsgSq"),
                            ];
                            ui.horizontal_wrapped(|ui| {
                                for (icon, name) in &all_icons {
                                    ui.vertical(|ui| {
                                        ui.set_min_width(56.0);
                                        ui.set_max_width(56.0);
                                        ui.horizontal(|ui| {
                                            ui.add_space(14.0);
                                            icons.draw(ui, *icon, 20.0, t.text_primary);
                                        });
                                        ui.label(egui::RichText::new(*name).color(t.text_muted).size(9.0));
                                    });
                                }
                            });
                        });
                    ui.add_space(24.0);

                    // ══════════════════════════════════════════════════════
                    // §10 — CARDS
                    // ══════════════════════════════════════════════════════
                    section_header(ui, &t, "Cards");
                    ui.horizontal(|ui| {
                        ui.add_space(32.0);
                        fg_card(ui, &t, FgCard::flat(), |ui| {
                            ui.set_min_width(180.0);
                            ui.label(egui::RichText::new("Flat Card").color(t.text_primary).size(t.font_size_md).strong());
                            ui.label(egui::RichText::new("bg_surface").color(t.text_muted).size(t.font_size_xs));
                        });
                        ui.add_space(12.0);
                        fg_card(ui, &t, FgCard::raised(), |ui| {
                            ui.set_min_width(180.0);
                            ui.label(egui::RichText::new("Raised Card").color(t.text_primary).size(t.font_size_md).strong());
                            ui.label(egui::RichText::new("bg_raised").color(t.text_muted).size(t.font_size_xs));
                        });
                        ui.add_space(12.0);
                        fg_card(ui, &t, FgCard::accent(), |ui| {
                            ui.set_min_width(180.0);
                            ui.label(egui::RichText::new("Accent Card").color(t.accent_primary).size(t.font_size_md).strong());
                            ui.label(egui::RichText::new("accent_subtle").color(t.text_muted).size(t.font_size_xs));
                        });
                    });

                    ui.add_space(48.0);
                });
            });

        // ── Modal overlay (rendered outside ScrollArea) ──────────────────
        if demo.show_modal {
            fg_modal(ctx, &t, "demo_modal", 400.0, |ui| {
                ui.label(egui::RichText::new("Create Feature")
                    .color(t.text_primary).size(t.font_size_lg).strong());
                ui.add_space(16.0);
                fg_input(ui, &t, FgInput::new(&mut demo.modal_form_name)
                    .label("Name").placeholder("New Cube…"));
                ui.add_space(12.0);
                fg_textarea(ui, &t, FgTextArea::new(&mut demo.modal_form_desc)
                    .label("Description").placeholder("Optional notes…").rows(2));
                ui.add_space(16.0);
                ui.separator();
                ui.add_space(8.0);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let save = fg_button(ui, &t, icons, FgButton::new("Create"));
                    ui.add_space(8.0);
                    let cancel = fg_button(ui, &t, icons, FgButton::new("Cancel").variant(FgButtonVariant::Ghost));
                    if save.clicked() || cancel.clicked() {
                        demo.show_modal = false;
                    }
                });
            });
        }
    });
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn section_header(ui: &mut egui::Ui, t: &forge_ui_theme::ForgeTheme, label: &str) {
    ui.horizontal(|ui| {
        ui.add_space(32.0);
        ui.label(egui::RichText::new(label).color(t.text_primary).size(t.font_size_lg).strong());
    });
    ui.add_space(8.0);
}

fn color_swatch(ui: &mut egui::Ui, name: &str, color: Color32, t: &forge_ui_theme::ForgeTheme) {
    ui.vertical(|ui| {
        ui.set_min_width(72.0);
        let (r, _) = ui.allocate_exact_size(Vec2::new(56.0, 28.0), egui::Sense::hover());
        ui.painter().rect_filled(r, CornerRadius::same(4), Color32::from_rgb(40, 40, 40));
        ui.painter().rect_filled(r, CornerRadius::same(4), color);
        ui.painter().rect_stroke(r, CornerRadius::same(4), Stroke::new(1.0, t.border_default), egui::StrokeKind::Outside);
        ui.label(egui::RichText::new(name).color(t.text_muted).size(9.0));
    });
}
