//! Forge UI — application entry point.
//!
//! M0 scaffold: opens a blank themed window using eframe.
//! Subsequent milestones will add the wgpu viewport and all panels.

use eframe::egui;
use egui::{Color32, CornerRadius, Frame, Pos2, Rect, Stroke, Vec2};
use forge_ui_state::AppState;

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

struct ForgeApp {
    state: AppState,
}

impl ForgeApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let state = AppState::new();
        state.theme.apply_to_egui(&cc.egui_ctx);
        Self { state }
    }
}

impl eframe::App for ForgeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.state.theme.apply_to_egui(ctx);

        // ── Global keyboard shortcuts ─────────────────────────────────────
        let input = ctx.input(|i| i.clone());
        if input.modifiers.command && input.key_pressed(egui::Key::K) {
            self.state.palette.toggle();
        }
        if input.key_pressed(egui::Key::Escape) {
            self.state.palette.close();
        }

        // ── Titlebar ──────────────────────────────────────────────────────
        egui::TopBottomPanel::top("titlebar")
            .exact_height(44.0)
            .frame(Frame::new().fill(self.state.theme.bg_surface).stroke(
                Stroke::new(1.0, self.state.theme.border_subtle),
            ))
            .show(ctx, |ui| {
                // Pull out primitive values before any closures to satisfy borrow checker.
                let accent     = self.state.theme.accent_primary;
                let bg_raised  = self.state.theme.bg_raised;
                let border_def = self.state.theme.border_default;
                let text_muted = self.state.theme.text_muted;
                let text_sec   = self.state.theme.text_secondary;
                let radius_sm  = self.state.theme.radius_sm;
                let radius_md  = self.state.theme.radius_md;
                let sp2 = self.state.theme.sp(2);
                let sp3 = self.state.theme.sp(3);
                let sp1 = self.state.theme.sp(1);
                let sp0 = self.state.theme.sp(0);
                let fsz_md = self.state.theme.font_size_md;
                let fsz_sm = self.state.theme.font_size_sm;
                let palette_open = self.state.palette.open;
                let theme_icon = match self.state.theme_kind {
                    forge_ui_state::ThemeKind::Dark => "☀",
                    forge_ui_state::ThemeKind::Light => "☾",
                };

                ui.horizontal_centered(|ui| {
                    ui.add_space(sp3);

                    // Logo pill
                    let logo_text = "◆ Forge";
                    let galley = ui.fonts(|f| {
                        f.layout_no_wrap(
                            logo_text.to_string(),
                            egui::FontId::proportional(fsz_md),
                            Color32::WHITE,
                        )
                    });
                    let pad = Vec2::new(sp2, sp1);
                    let logo_size = galley.size() + pad * 2.0;
                    let (logo_rect, _) = ui.allocate_exact_size(logo_size, egui::Sense::hover());
                    if ui.is_rect_visible(logo_rect) {
                        ui.painter().rect_filled(
                            logo_rect,
                            CornerRadius::same(radius_sm as u8),
                            accent,
                        );
                        ui.painter().galley(logo_rect.min + pad, galley, Color32::WHITE);
                    }

                    ui.add_space(sp2);

                    // Filename chip
                    Frame::new()
                        .fill(bg_raised)
                        .stroke(Stroke::new(1.0, border_def))
                        .corner_radius(CornerRadius::same(radius_sm as u8))
                        .inner_margin(egui::Margin::symmetric(sp2 as i8, sp0 as i8))
                        .show(ui, |ui| {
                            ui.label(
                                egui::RichText::new("my_model.fg ●")
                                    .color(text_sec)
                                    .size(fsz_sm),
                            );
                        });

                    // ⌘K bar (center)
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        ui.horizontal(|ui| {
                            let label = if palette_open {
                                "  ⌘K  Searching…"
                            } else {
                                "  ⌘K  Search operations…"
                            };
                            let btn = ui.add(
                                egui::Button::new(
                                    egui::RichText::new(label)
                                        .color(text_muted)
                                        .size(fsz_sm),
                                )
                                .min_size(egui::vec2(280.0, 32.0))
                                .corner_radius(CornerRadius::same(radius_md as u8))
                                .stroke(Stroke::new(1.0, border_def))
                                .fill(bg_raised),
                            );
                            if btn.clicked() {
                                self.state.palette.toggle();
                            }
                        });
                    });

                    // Theme toggle
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(sp3);
                        if ui
                            .button(egui::RichText::new(theme_icon).color(text_sec))
                            .clicked()
                        {
                            self.state.toggle_theme();
                        }
                    });
                });
            });

        // ── Status bar ────────────────────────────────────────────────────
        egui::TopBottomPanel::bottom("statusbar")
            .exact_height(24.0)
            .frame(Frame::new()
                .fill(self.state.theme.bg_surface)
                .stroke(Stroke::new(1.0, self.state.theme.border_subtle)))
            .show(ctx, |ui| {
                let t = &self.state.theme;
                let tel = &self.state.telemetry;
                ui.horizontal_centered(|ui| {
                    ui.add_space(t.sp(2));
                    ui.label(
                        egui::RichText::new(format!(
                            "{} faces · {} verts · {} edges",
                            tel.face_count, tel.vertex_count, tel.edge_count,
                        ))
                        .color(t.text_muted)
                        .size(t.font_size_xs),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(t.sp(2));
                        ui.label(
                            egui::RichText::new(format!(
                                "Exact · {:.1}ms · v0.1",
                                tel.last_op_ms,
                            ))
                            .color(t.text_muted)
                            .size(t.font_size_xs),
                        );
                    });
                });
            });

        // ── Left sidebar — Feature tree ────────────────────────────────────
        egui::SidePanel::left("sidebar")
            .default_width(220.0)
            .min_width(160.0)
            .frame(Frame::new()
                .fill(self.state.theme.bg_sidebar)
                .stroke(Stroke::new(1.0, self.state.theme.border_subtle)))
            .show(ctx, |ui| {
                let t = &self.state.theme;
                ui.add_space(t.sp(2));

                // Section header — uppercase label with inline rule
                ui.horizontal(|ui| {
                    ui.add_space(t.sp(2));
                    ui.label(
                        egui::RichText::new("FEATURE TREE")
                            .color(t.text_muted)
                            .size(t.font_size_xs)
                            .strong(),
                    );
                });
                ui.add_space(t.sp(1));
                ui.separator();
                ui.add_space(t.sp(1));

                let features = self.state.model.features().to_vec();
                let row_h = 28.0;
                let left_pad = 10.0;

                for feature in &features {
                    let is_selected = self.state.model.selected() == Some(feature.id);

                    let (status_color, _) = match &feature.status {
                        forge_ui_types::FeatureStatus::Exact        => (t.success, ""),
                        forge_ui_types::FeatureStatus::NearBoundary => (t.warning, ""),
                        forge_ui_types::FeatureStatus::Error(_)     => (t.danger, ""),
                        forge_ui_types::FeatureStatus::Pending      => (t.text_muted, ""),
                    };

                    // Allocate the row — constrained width with horizontal margin.
                    let avail_w = ui.available_width() - left_pad;
                    let (row_rect, row_resp) = ui.allocate_exact_size(
                        Vec2::new(avail_w, row_h),
                        egui::Sense::click(),
                    );

                    if ui.is_rect_visible(row_rect) {
                        let painter = ui.painter();
                        let rr = CornerRadius::same(t.radius_sm as u8);

                        // Selection / hover background
                        if is_selected {
                            painter.rect_filled(row_rect, rr, t.accent_subtle);
                            // Left accent stripe
                            painter.rect_filled(
                                Rect::from_min_size(
                                    row_rect.min,
                                    Vec2::new(3.0, row_h),
                                ),
                                CornerRadius::same(2),
                                t.accent_primary,
                            );
                        } else if row_resp.hovered() {
                            painter.rect_filled(
                                row_rect,
                                rr,
                                Color32::from_white_alpha(6),
                            );
                        }

                        // Status dot
                        let dot_x = row_rect.min.x + left_pad + 4.0;
                        let dot_y = row_rect.center().y;
                        painter.circle_filled(Pos2::new(dot_x, dot_y), 3.5, status_color);

                        // Feature name
                        let label_color = if is_selected { t.text_primary } else { t.text_secondary };
                        let font = egui::FontId::proportional(t.font_size_sm);
                        let galley = ui.fonts(|f| {
                            f.layout_no_wrap(feature.name.clone(), font, label_color)
                        });
                        let text_pos = Pos2::new(
                            dot_x + 12.0,
                            row_rect.center().y - galley.size().y / 2.0,
                        );
                        painter.galley(text_pos, galley, label_color);
                    }

                    if row_resp.clicked() {
                        self.state.model.select(feature.id);
                    }
                }
            });

        // ── Right drawer ──────────────────────────────────────────────────
        egui::SidePanel::right("right_drawer")
            .default_width(308.0)
            .frame(Frame::new()
                .fill(self.state.theme.bg_surface)
                .stroke(Stroke::new(1.0, self.state.theme.border_subtle)))
            .show(ctx, |ui| {
                let t = &self.state.theme;

                // Custom tab bar
                let tab_height = 40.0;
                let (tab_rect, _) =
                    ui.allocate_exact_size(Vec2::new(ui.available_width(), tab_height), egui::Sense::hover());

                // Tab bar background
                ui.painter().rect_filled(tab_rect, 0.0, t.bg_surface);
                // Bottom border of tab bar
                ui.painter().rect_filled(
                    Rect::from_min_size(
                        Pos2::new(tab_rect.min.x, tab_rect.max.y - 1.0),
                        Vec2::new(tab_rect.width(), 1.0),
                    ),
                    0.0,
                    t.border_subtle,
                );

                let tabs = [
                    (forge_ui_state::DrawerTab::Properties, "Properties"),
                    (forge_ui_state::DrawerTab::Chat, "Chat"),
                ];
                let tab_w = tab_rect.width() / tabs.len() as f32;

                for (idx, (tab, label)) in tabs.iter().enumerate() {
                    let is_active = self.state.drawer.active_tab == *tab;
                    let tab_x = tab_rect.min.x + idx as f32 * tab_w;
                    let this_tab_rect = Rect::from_min_size(
                        Pos2::new(tab_x, tab_rect.min.y),
                        Vec2::new(tab_w, tab_height),
                    );

                    let text_color = if is_active { t.text_primary } else { t.text_muted };
                    let font = egui::FontId::proportional(t.font_size_sm);
                    let galley = ui.fonts(|f| {
                        f.layout_no_wrap(label.to_string(), font, text_color)
                    });
                    let text_pos = this_tab_rect.center()
                        - Vec2::new(galley.size().x / 2.0, galley.size().y / 2.0);
                    ui.painter().galley(text_pos, galley, text_color);

                    // Active bottom border
                    if is_active {
                        ui.painter().rect_filled(
                            Rect::from_min_size(
                                Pos2::new(this_tab_rect.min.x + 8.0, this_tab_rect.max.y - 2.0),
                                Vec2::new(this_tab_rect.width() - 16.0, 2.0),
                            ),
                            CornerRadius::same(1),
                            t.accent_primary,
                        );
                    }

                    // Click detection
                    let tab_resp = ui.interact(this_tab_rect, egui::Id::new(format!("tab_{idx}")), egui::Sense::click());
                    if tab_resp.clicked() {
                        self.state.drawer.active_tab = *tab;
                    }
                }

                ui.add_space(t.sp(2));

                match self.state.drawer.active_tab {
                    forge_ui_state::DrawerTab::Properties => {
                        draw_properties_panel(ui, &mut self.state);
                    }
                    forge_ui_state::DrawerTab::Chat => {
                        draw_chat_panel(ui, &mut self.state);
                    }
                }
            });

        // ── Central viewport (placeholder for wgpu in M5) ─────────────────
        egui::CentralPanel::default()
            .frame(Frame::new().fill(self.state.theme.bg_base))
            .show(ctx, |ui| {
                let t = &self.state.theme;
                let rect = ui.available_rect_before_wrap();

                // Background
                ui.painter().rect_filled(rect, 0.0, t.bg_base);

                // Dot grid overlay
                let grid_spacing = 28.0;
                let dot_r = 1.0;
                let start_x = rect.min.x + (rect.min.x % grid_spacing);
                let start_y = rect.min.y + (rect.min.y % grid_spacing);
                let mut y = start_y;
                while y < rect.max.y {
                    let mut x = start_x;
                    while x < rect.max.x {
                        ui.painter()
                            .circle_filled(Pos2::new(x, y), dot_r, t.viewport_grid);
                        x += grid_spacing;
                    }
                    y += grid_spacing;
                }

                // Centered placeholder card
                let card_w = 320.0;
                let card_h = 60.0;
                let card_rect = Rect::from_center_size(rect.center(), Vec2::new(card_w, card_h));
                ui.painter().rect(
                    card_rect,
                    CornerRadius::same(t.radius_lg as u8),
                    t.bg_surface,
                    Stroke::new(1.0, t.border_default),
                    egui::StrokeKind::Outside,
                );
                // Info dot
                ui.painter().circle_filled(
                    Pos2::new(card_rect.min.x + 20.0, card_rect.center().y),
                    4.0,
                    t.info,
                );
                UI_PAINTER_TEXT(ui, t, card_rect);
            });

        // ── Command palette overlay ───────────────────────────────────────
        if self.state.palette.open {
            egui::Area::new(egui::Id::new("command_palette"))
                .anchor(egui::Align2::CENTER_TOP, egui::vec2(0.0, 56.0))
                .order(egui::Order::Foreground)
                .show(ctx, |ui| {
                    let t = &self.state.theme;
                    Frame::new()
                        .fill(t.bg_raised)
                        .stroke(Stroke::new(1.0, t.border_default))
                        .corner_radius(CornerRadius::same(t.radius_lg as u8))
                        .shadow(egui::Shadow {
                            offset: [0, 8],
                            blur: 32,
                            spread: 0,
                            color: Color32::from_black_alpha(120),
                        })
                        .show(ui, |ui| {
                            ui.set_width(520.0);
                            ui.add_space(t.sp(1));

                            // Search row
                            ui.horizontal(|ui| {
                                ui.add_space(t.sp(2));
                                ui.label(egui::RichText::new("⌘K").color(t.text_muted).size(t.font_size_sm));
                                ui.add_space(t.sp(1));
                                let response = ui.add(
                                    egui::TextEdit::singleline(&mut self.state.palette.query)
                                        .hint_text("Search operations, features, geometry…")
                                        .desired_width(f32::INFINITY)
                                        .font(egui::FontId::proportional(t.font_size_md))
                                        .frame(false),
                                );
                                response.request_focus();
                            });

                            ui.add_space(t.sp(1));
                            ui.separator();

                            // Stub result rows
                            for (icon, name, desc) in [
                                ("⊕", "Boolean Union", "Merge two solids"),
                                ("⊖", "Boolean Subtract", "Subtract one solid from another"),
                                ("⊗", "Boolean Intersect", "Keep the overlapping region"),
                            ] {
                                ui.horizontal(|ui| {
                                    ui.add_space(t.sp(2));
                                    ui.label(egui::RichText::new(icon).color(t.accent_primary).size(t.font_size_md));
                                    ui.add_space(t.sp(1));
                                    ui.vertical(|ui| {
                                        ui.label(egui::RichText::new(name).color(t.text_primary).size(t.font_size_sm).strong());
                                        ui.label(egui::RichText::new(desc).color(t.text_muted).size(t.font_size_xs));
                                    });
                                });
                                ui.add_space(t.sp(1));
                            }

                            ui.separator();
                            ui.horizontal(|ui| {
                                ui.add_space(t.sp(2));
                                ui.label(
                                    egui::RichText::new("↑↓ navigate  ↵ run  Esc close")
                                        .color(t.text_muted)
                                        .size(t.font_size_xs),
                                );
                            });
                            ui.add_space(t.sp(1));
                        });
                });
        }
    }
}

/// Paint the viewport placeholder text — factored out to avoid borrow issues.
fn UI_PAINTER_TEXT(ui: &egui::Ui, t: &forge_ui_theme::ForgeTheme, card_rect: Rect) {
    let text = "3D Viewport  —  wgpu SDF raymarcher arriving in M5";
    let font = egui::FontId::proportional(t.font_size_sm);
    let galley = ui.fonts(|f| f.layout_no_wrap(text.to_string(), font, t.text_secondary));
    let text_pos = Pos2::new(
        card_rect.min.x + 36.0,
        card_rect.center().y - galley.size().y / 2.0,
    );
    ui.painter().galley(text_pos, galley, t.text_secondary);
}

fn draw_properties_panel(ui: &mut egui::Ui, state: &mut AppState) {
    let t = &state.theme;
    ui.add_space(t.sp(1));

    if let Some(_id) = state.model.selected() {
        if let Some(plane) = state.model.planes().iter().next() {
            // Section chip
            Frame::new()
                .fill(t.accent_subtle)
                .corner_radius(CornerRadius::same(t.radius_sm as u8))
                .inner_margin(egui::Margin::symmetric(t.sp(1) as i8, 2))
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new("Selected Plane")
                            .color(t.accent_primary)
                            .size(t.font_size_xs)
                            .strong(),
                    );
                });
            ui.add_space(t.sp(2));

            // Property rows
            for (label, value) in [
                ("Normal X", format!("{:.4}", plane.normal[0])),
                ("Normal Y", format!("{:.4}", plane.normal[1])),
                ("Normal Z", format!("{:.4}", plane.normal[2])),
                ("Offset D", format!("{:.4}", plane.offset)),
            ] {
                ui.horizontal(|ui| {
                    ui.add_space(t.sp(1));
                    ui.label(
                        egui::RichText::new(label)
                            .color(t.text_muted)
                            .size(t.font_size_sm),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(t.sp(1));
                        ui.label(
                            egui::RichText::new(value)
                                .color(t.text_primary)
                                .size(t.font_size_sm)
                                .monospace(),
                        );
                    });
                });
            }
        }
    } else {
        ui.add_space(t.sp(3));
        ui.horizontal(|ui| {
            ui.add_space(t.sp(4));
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("No selection").color(t.text_secondary).size(t.font_size_md).strong());
                ui.add_space(t.sp(1));
                ui.label(
                    egui::RichText::new("Click a feature in the tree or a face in the viewport to inspect it.")
                        .color(t.text_muted)
                        .size(t.font_size_sm),
                );
            });
        });
    }
}

fn draw_chat_panel(ui: &mut egui::Ui, state: &mut AppState) {
    let t = &state.theme;
    let available_height = ui.available_height();
    let input_area_h = 80.0;
    let footer_h = 28.0;

    let h_pad = 12.0_f32;

    // Wrap everything in a horizontal-padded frame so messages never touch the panel edges.
    Frame::new()
        .inner_margin(egui::Margin { left: h_pad as i8, right: h_pad as i8, top: 0, bottom: 0 })
        .show(ui, |ui| {

    // ── Message list ─────────────────────────────────────────────────────
    egui::ScrollArea::vertical()
        .max_height(available_height - input_area_h - footer_h - 16.0)
        .stick_to_bottom(true)
        .show(ui, |ui| {
            let messages = state.chat.messages().to_vec();
            for msg in &messages {
                let is_user = matches!(msg.role, forge_ui_types::MessageRole::User);

                let text = match &msg.content {
                    forge_ui_types::MessageContent::Text(s) => s.clone(),
                    forge_ui_types::MessageContent::CodeBlock { source, .. } => source.clone(),
                    forge_ui_types::MessageContent::KernelEvent(s) => format!("[event] {s}"),
                };

                if is_user {
                    // User message — plain, right-aligned sender chip
                    ui.add_space(t.sp(2));
                    ui.horizontal(|ui| {
                        ui.add_space(t.sp(2));
                        Frame::new()
                            .fill(t.accent_subtle)
                            .corner_radius(CornerRadius::same(t.radius_sm as u8))
                            .inner_margin(egui::Margin::symmetric(6, 2))
                            .show(ui, |ui| {
                                ui.label(
                                    egui::RichText::new("You")
                                        .color(t.accent_primary)
                                        .size(t.font_size_xs)
                                        .strong(),
                                );
                            });
                    });
                    ui.add_space(2.0);
                    ui.horizontal(|ui| {
                        ui.add_space(t.sp(2));
                        ui.label(egui::RichText::new(&text).color(t.text_primary).size(t.font_size_sm));
                    });
                } else {
                    // Agent/System message — card bubble
                    ui.add_space(t.sp(2));
                    Frame::new()
                        .fill(t.chat_agent_bg)
                        .stroke(Stroke::new(1.0, t.border_subtle))
                        .corner_radius(CornerRadius::same(t.radius_md as u8))
                        .inner_margin(egui::Margin::symmetric(
                            t.sp(2) as i8,
                            t.sp(1) as i8,
                        ))
                        .show(ui, |ui| {
                            // Sender row
                            ui.horizontal(|ui| {
                                let (sender, color) = match msg.role {
                                    forge_ui_types::MessageRole::Agent => ("Forge", t.success),
                                    forge_ui_types::MessageRole::System => ("System", t.text_muted),
                                    forge_ui_types::MessageRole::User => ("You", t.accent_primary),
                                };
                                ui.label(egui::RichText::new(sender).color(color).size(t.font_size_xs).strong());
                            });
                            ui.add_space(2.0);
                            ui.label(egui::RichText::new(&text).color(t.text_secondary).size(t.font_size_sm));
                            ui.add_space(t.sp(1));

                            // Feedback row
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("Good 👍").color(t.text_muted).size(t.font_size_xs));
                                ui.add_space(t.sp(1));
                                ui.label(egui::RichText::new("Bad 👎").color(t.text_muted).size(t.font_size_xs));
                            });
                        });
                }
            }
            ui.add_space(t.sp(2));
        });

    ui.separator();
    ui.add_space(4.0);

    // ── Input area ───────────────────────────────────────────────────────
    Frame::new()
        .fill(t.bg_raised)
        .stroke(Stroke::new(1.0, t.border_default))
        .corner_radius(CornerRadius::same(t.radius_md as u8))
        .inner_margin(egui::Margin::symmetric(t.sp(2) as i8, t.sp(1) as i8))
        .show(ui, |ui| {
            // Hint row inside the box
            ui.add(
                egui::TextEdit::multiline(&mut state.chat.input_draft)
                    .hint_text("Ask Forge anything…  @ to mention,  / for workflows")
                    .desired_width(f32::INFINITY)
                    .desired_rows(2)
                    .font(egui::FontId::proportional(t.font_size_sm))
                    .frame(false),
            );

            ui.separator();

            ui.horizontal(|ui| {
                // Model label
                ui.label(
                    egui::RichText::new("Forge · v0.1")
                        .color(t.text_muted)
                        .size(t.font_size_xs),
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Send button — painted circle with arrow, no default button frame.
                    let send_ready = !state.chat.input_draft.trim().is_empty();
                    let btn_size = Vec2::splat(26.0);
                    let (btn_rect, btn_resp) =
                        ui.allocate_exact_size(btn_size, egui::Sense::click());

                    if ui.is_rect_visible(btn_rect) {
                        let c = btn_rect.center();
                        let circle_color = if send_ready { t.accent_primary } else { t.bg_raised };
                        let border_color = if send_ready { t.accent_primary } else { t.border_default };
                        ui.painter().circle_filled(c, 11.0, circle_color);
                        ui.painter().circle_stroke(c, 11.0, Stroke::new(1.0, border_color));

                        // Painted right-pointing triangle arrow
                        let arrow_color = if send_ready { Color32::WHITE } else { t.text_muted };
                        let tip   = Pos2::new(c.x + 4.5, c.y);
                        let top   = Pos2::new(c.x - 2.5, c.y - 4.0);
                        let bot   = Pos2::new(c.x - 2.5, c.y + 4.0);
                        ui.painter().add(egui::Shape::convex_polygon(
                            vec![tip, top, bot],
                            arrow_color,
                            Stroke::NONE,
                        ));
                    }

                    if btn_resp.clicked() && send_ready {
                        state.chat.submit_draft();
                    }
                });
            });
        }); // ── end input Frame

    }); // ── end horizontal-padding Frame

    // ── Footer model label (outside padding frame, flush to panel edges) ─────
    ui.add_space(4.0);
    ui.horizontal(|ui| {
        ui.add_space(t.sp(2));
        ui.label(egui::RichText::new("▲ Planning").color(t.text_muted).size(t.font_size_xs));
        ui.add_space(t.sp(1));
        ui.label(egui::RichText::new("·").color(t.border_default).size(t.font_size_xs));
        ui.add_space(t.sp(1));
        ui.label(egui::RichText::new("Forge Kernel v0.1").color(t.text_muted).size(t.font_size_xs));
    });
}
