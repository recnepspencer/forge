//! Forge UI — application entry point.
//!
//! Composed from organism modules. Each panel/section lives in its own file.
//! Atom and molecule components live in forge-ui-components.

mod chat_panel;
mod properties_panel;
mod right_drawer;
mod sidebar;
mod status_bar;
mod test_board;
mod titlebar;
mod viewport;

use eframe::egui;
use egui::{Color32, CornerRadius, Frame, Key, Stroke};
use forge_ui_components::IconStore;
use forge_ui_state::AppState;

/// Application page routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Editor,
    TestBoard,
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Forge")
            .with_inner_size([1400.0, 900.0])
            .with_min_inner_size([900.0, 600.0])
            .with_decorations(true),
        ..Default::default()
    };
    eframe::run_native(
        "Forge",
        native_options,
        Box::new(|cc| Ok(Box::new(ForgeApp::new(cc)))),
    )
}

// ── App ───────────────────────────────────────────────────────────────────────

struct ForgeApp {
    state:    AppState,
    icons:    IconStore,
    viewport: viewport::ViewportState,
    page:     Page,
}

impl ForgeApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let state = AppState::new();
        state.theme.apply_to_egui(&cc.egui_ctx);
        egui_extras::install_image_loaders(&cc.egui_ctx);

        Self {
            icons:    IconStore::load(&cc.egui_ctx),
            viewport: viewport::ViewportState::new(),
            state,
            page:     Page::Editor,
        }
    }
}

impl eframe::App for ForgeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.state.theme.apply_to_egui(ctx);

        // ── Global shortcuts ──────────────────────────────────────────
        let input = ctx.input(|i| i.clone());
        if input.key_pressed(Key::F1) {
            self.page = match self.page {
                Page::Editor    => Page::TestBoard,
                Page::TestBoard => Page::Editor,
            };
        }
        if input.modifiers.command && input.key_pressed(Key::K) {
            self.state.palette.toggle();
        }
        if input.key_pressed(Key::Escape) {
            if self.page == Page::TestBoard {
                self.page = Page::Editor;
            } else {
                self.state.palette.close();
            }
        }

        // ── Titlebar (always visible) ─────────────────────────────────
        titlebar::draw_titlebar(ctx, &mut self.state, &self.icons, &mut self.page);

        // ── Page routing ──────────────────────────────────────────────
        if self.page == Page::TestBoard {
            test_board::draw_test_board(ctx, &mut self.state, &self.icons);
            return;
        }

        // ── Editor layout ─────────────────────────────────────────────
        status_bar::draw_status_bar(ctx, &self.state);
        sidebar::draw_sidebar(ctx, &mut self.state, &self.icons);
        right_drawer::draw_right_drawer(ctx, &mut self.state);
        viewport::draw_viewport(ctx, &mut self.viewport, &self.state.theme);

        // ── Command palette overlay ───────────────────────────────────
        if self.state.palette.open {
            draw_command_palette(ctx, &mut self.state);
        }
    }
}

// ── Command palette (will extract to its own module next) ─────────────────────

fn draw_command_palette(ctx: &egui::Context, state: &mut AppState) {
    egui::Area::new(egui::Id::new("command_palette"))
        .anchor(egui::Align2::CENTER_TOP, egui::vec2(0.0, 56.0))
        .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            let t = &state.theme;
            Frame::new()
                .fill(t.bg_raised)
                .stroke(Stroke::new(1.0, t.border_default))
                .corner_radius(CornerRadius::same(t.radius_lg as u8))
                .shadow(egui::Shadow { offset: [0, 8], blur: 32, spread: 0, color: Color32::from_black_alpha(120) })
                .show(ui, |ui| {
                    ui.set_width(520.0);
                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        ui.add_space(12.0);
                        ui.label(egui::RichText::new("⌘K").color(t.text_muted).size(t.font_size_sm));
                        ui.add_space(6.0);
                        let resp = ui.add(
                            egui::TextEdit::singleline(&mut state.palette.query)
                                .hint_text("Search operations, features, geometry…")
                                .desired_width(f32::INFINITY)
                                .font(egui::FontId::proportional(t.font_size_md))
                                .frame(false),
                        );
                        resp.request_focus();
                    });
                    ui.add_space(4.0);
                    ui.separator();
                    for (icon, name, desc) in [
                        ("⊕", "Boolean Union",     "Merge two solids"),
                        ("⊖", "Boolean Subtract",  "Subtract one solid from another"),
                        ("⊗", "Boolean Intersect", "Keep the overlapping region"),
                    ] {
                        ui.add_space(2.0);
                        ui.horizontal(|ui| {
                            ui.add_space(14.0);
                            ui.label(egui::RichText::new(icon).color(t.accent_primary).size(t.font_size_md));
                            ui.add_space(8.0);
                            ui.vertical(|ui| {
                                ui.label(egui::RichText::new(name).color(t.text_primary).size(t.font_size_sm).strong());
                                ui.label(egui::RichText::new(desc).color(t.text_muted).size(t.font_size_xs));
                            });
                        });
                    }
                    ui.add_space(4.0);
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.add_space(14.0);
                        ui.label(egui::RichText::new("↑↓ navigate  ↵ run  Esc close").color(t.text_muted).size(t.font_size_xs));
                    });
                    ui.add_space(4.0);
                });
        });
}
