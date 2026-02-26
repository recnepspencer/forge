//! Forge UI — application entry point.
//!
//! M0 scaffold: themed window with interactive 2D viewport.
//! WASD / arrow keys pan the canvas; click + drag moves the selected shape.

use eframe::egui;
use egui::{Color32, CornerRadius, Frame, Key, Pos2, Rect, Stroke, Vec2};
use forge_ui_components::{FgIcon, IconStore};
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

// ── Viewport state ────────────────────────────────────────────────────────────

use glam::{Mat4, Vec3, Vec4};

// ── Viewport state ────────────────────────────────────────────────────────────

struct Camera {
    pos: Vec3,
    yaw: f32,
    pitch: f32,
    fov_y: f32,
}

impl Camera {
    fn new() -> Self {
        Self {
            pos: Vec3::new(0.0, 2.0, 5.0),
            yaw: 0.0,
            pitch: -0.3, // look slightly down
            fov_y: 60.0_f32.to_radians(),
        }
    }

    fn view_proj(&self, aspect: f32) -> Mat4 {
        let rotation = glam::Quat::from_euler(glam::EulerRot::YXZ, self.yaw, self.pitch, 0.0);
        let forward = rotation * Vec3::NEG_Z;
        let up = rotation * Vec3::Y;
        let view = Mat4::look_at_rh(self.pos, self.pos + forward, up);
        let proj = Mat4::perspective_rh(self.fov_y, aspect, 0.1, 100.0);
        proj * view
    }

    fn forward(&self) -> Vec3 {
        let rotation = glam::Quat::from_euler(glam::EulerRot::YXZ, self.yaw, 0.0, 0.0);
        rotation * Vec3::NEG_Z
    }

    fn right(&self) -> Vec3 {
        let rotation = glam::Quat::from_euler(glam::EulerRot::YXZ, self.yaw, 0.0, 0.0);
        rotation * Vec3::X
    }
}

struct ViewportState {
    camera: Camera,
    cube_center: Vec3,
}

impl ViewportState {
    fn new() -> Self {
        Self {
            camera: Camera::new(),
            cube_center: Vec3::ZERO,
        }
    }
}

// ── App ───────────────────────────────────────────────────────────────────────

struct ForgeApp {
    state:    AppState,
    icons:    IconStore,
    viewport: ViewportState,
}

impl ForgeApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let state = AppState::new();
        state.theme.apply_to_egui(&cc.egui_ctx);

        // Install the SVG loader so egui_extras can load textures from bytes.
        egui_extras::install_image_loaders(&cc.egui_ctx);

        Self {
            icons:    IconStore::load(&cc.egui_ctx),
            viewport: ViewportState::new(),
            state,
        }
    }
}

impl eframe::App for ForgeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.state.theme.apply_to_egui(ctx);

        // ── Global shortcuts ──────────────────────────────────────────────
        let input = ctx.input(|i| i.clone());
        if input.modifiers.command && input.key_pressed(Key::K) { self.state.palette.toggle(); }
        if input.key_pressed(Key::Escape) { self.state.palette.close(); }

        // ── Titlebar ──────────────────────────────────────────────────────
        {
            let t = &self.state.theme;
            let accent     = t.accent_primary;
            let bg_raised  = t.bg_raised;
            let bg_base    = t.bg_base;
            let border_def = t.border_default;
            let border_subtle = t.border_subtle;
            let _text_muted = t.text_muted;
            let text_sec   = t.text_secondary;
            let radius_sm  = t.radius_sm;
            let radius_md  = t.radius_md;
            let fsz_md = t.font_size_md;
            let fsz_sm = t.font_size_sm;
            let palette_open = self.state.palette.open;
            let theme_icon = match self.state.theme_kind {
                forge_ui_state::ThemeKind::Dark  => "☀",
                forge_ui_state::ThemeKind::Light => "☾",
            };
            
            let top_bg = t.bg_surface;
            let top_stroke = t.border_subtle;

            egui::TopBottomPanel::top("titlebar")
                .exact_height(44.0)
                .frame(Frame::new()
                    .fill(top_bg)
                    .stroke(Stroke::new(1.0, top_stroke)))
                .show(ctx, |ui| {
                    ui.horizontal_centered(|ui| {
                        ui.add_space(12.0);

                        // ── Logo pill ─────────────────────────────────────
                        let logo_text = "◆ Forge";
                        let galley = ui.fonts(|f|
                            f.layout_no_wrap(logo_text.to_string(), egui::FontId::proportional(fsz_md), Color32::WHITE));
                        let pad = Vec2::new(10.0, 5.0);
                        let (logo_rect, _) = ui.allocate_exact_size(galley.size() + pad * 2.0, egui::Sense::hover());
                        if ui.is_rect_visible(logo_rect) {
                            ui.painter().rect_filled(logo_rect, CornerRadius::same(radius_sm as u8), accent);
                            ui.painter().galley(logo_rect.min + pad, galley, Color32::WHITE);
                        }
                        ui.add_space(8.0);

                        // ── Filename chip ─────────────────────────────────
                        Frame::new()
                            .fill(bg_raised)
                            .stroke(Stroke::new(1.0, border_def))
                            .corner_radius(CornerRadius::same(radius_sm as u8))
                            .inner_margin(egui::Margin { left: 10, right: 10, top: 3, bottom: 3 })
                            .show(ui, |ui| {
                                ui.label(egui::RichText::new("my_model.fg ●").color(text_sec).size(fsz_sm));
                            });

                        // ── ⌘K search (centered) ─────────────────────────
                        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                            let (rect, resp) = ui.allocate_exact_size(Vec2::new(260.0, 30.0), egui::Sense::click());
                            if ui.is_rect_visible(rect) {
                                let hovered = resp.hovered() || palette_open;
                                let bg = if hovered { bg_raised } else { bg_base };
                                let border = if hovered { border_subtle } else { border_def };
                                ui.painter().rect(rect, CornerRadius::same(radius_md as u8), bg, Stroke::new(1.0, border), egui::StrokeKind::Outside);
                                
                                // Draw ⌘K on the right
                                let galley_cmd = ui.fonts(|f| f.layout_no_wrap("⌘K".to_string(), egui::FontId::proportional(fsz_sm), _text_muted));
                                ui.painter().galley(Pos2::new(rect.max.x - 10.0 - galley_cmd.size().x, rect.center().y - galley_cmd.size().y / 2.0), galley_cmd, _text_muted);
                                
                                // Draw text
                                let search_text = if palette_open { "Search operations…" } else { "Search operations…" };
                                let galley_text = ui.fonts(|f| f.layout_no_wrap(search_text.to_string(), egui::FontId::proportional(fsz_sm), _text_muted));
                                ui.painter().galley(Pos2::new(rect.min.x + 12.0, rect.center().y - galley_text.size().y / 2.0), galley_text, _text_muted);
                            }
                            if resp.clicked() { self.state.palette.toggle(); }
                        });

                        // ── Theme toggle (right-aligned) ──────────────────
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.add_space(16.0);
                            let (rect, resp) = ui.allocate_exact_size(Vec2::new(28.0, 28.0), egui::Sense::click());
                            if ui.is_rect_visible(rect) {
                                let hovered = resp.hovered();
                                if hovered {
                                    ui.painter().rect_filled(rect, CornerRadius::same(radius_sm as u8), bg_raised);
                                }
                                let galley = ui.fonts(|f| f.layout_no_wrap(theme_icon.to_string(), egui::FontId::proportional(14.0), text_sec));
                                ui.painter().galley(rect.center() - galley.size() / 2.0, galley, text_sec);
                            }
                            if resp.clicked() {
                                self.state.toggle_theme();
                            }
                        });
                    });
                });
        }

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
                    ui.add_space(12.0);
                    ui.label(egui::RichText::new(format!(
                        "{} faces · {} verts · {} edges", tel.face_count, tel.vertex_count, tel.edge_count,
                    )).color(t.text_muted).size(t.font_size_xs));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(12.0);
                        ui.label(egui::RichText::new(format!("Exact · {:.1}ms · v0.1", tel.last_op_ms))
                            .color(t.text_muted).size(t.font_size_xs));
                    });
                });
            });

        // ── Left sidebar — Feature tree ────────────────────────────────────
        {
            let t_sidebar = self.state.theme.bg_sidebar;
            let t_border  = self.state.theme.border_subtle;
            egui::SidePanel::left("sidebar")
                .default_width(220.0)
                .min_width(160.0)
                .frame(Frame::new().fill(t_sidebar).stroke(Stroke::new(1.0, t_border)))
                .show(ctx, |ui| {
                    let t = &self.state.theme;
                    ui.add_space(10.0);

                    // Section label
                    ui.horizontal(|ui| {
                        ui.add_space(14.0);
                        ui.label(egui::RichText::new("FEATURE TREE")
                            .color(t.text_muted).size(t.font_size_xs).strong());
                    });
                    ui.add_space(6.0);
                    ui.separator();
                    ui.add_space(4.0);

                    let features = self.state.model.features().to_vec();
                    let row_h = 30.0;

                    for feature in &features {
                        let is_selected = self.state.model.selected() == Some(feature.id);

                        let status_color = match &feature.status {
                            forge_ui_types::FeatureStatus::Exact        => t.success,
                            forge_ui_types::FeatureStatus::NearBoundary => t.warning,
                            forge_ui_types::FeatureStatus::Error(_)     => t.danger,
                            forge_ui_types::FeatureStatus::Pending      => t.text_muted,
                        };

                        // Allocate full-width row, then place content with left offset.
                        let avail_w = ui.available_width();
                        let (row_rect, row_resp) = ui.allocate_exact_size(
                            Vec2::new(avail_w, row_h), egui::Sense::click());

                        if ui.is_rect_visible(row_rect) {
                            let painter = ui.painter();
                            let rr = CornerRadius::same(t.radius_sm as u8);
                            // Inset the visual background from the panel edges
                            let bg_rect = Rect::from_min_max(
                                Pos2::new(row_rect.min.x + 6.0, row_rect.min.y + 1.0),
                                Pos2::new(row_rect.max.x - 6.0, row_rect.max.y - 1.0),
                            );

                            if is_selected {
                                painter.rect_filled(bg_rect, rr, t.accent_subtle);
                                // Left accent stripe
                                painter.rect_filled(
                                    Rect::from_min_size(bg_rect.min, Vec2::new(3.0, bg_rect.height())),
                                    CornerRadius::same(2), t.accent_primary);
                            } else if row_resp.hovered() {
                                painter.rect_filled(bg_rect, rr, Color32::from_white_alpha(7));
                            }

                            // Feature icon and name
                            let label_color = if is_selected { t.text_primary } else { t.text_secondary };
                            let mut child_ui = ui.new_child(
                                egui::UiBuilder::new()
                                    .max_rect(bg_rect)
                                    .layout(egui::Layout::left_to_right(egui::Align::Center))
                            );
                            child_ui.add_space(8.0);
                            self.icons.draw(&mut child_ui, FgIcon::Box, 14.0, status_color);
                            child_ui.add_space(6.0);
                            child_ui.label(egui::RichText::new(&feature.name).color(label_color).size(t.font_size_sm));
                        }
                        if row_resp.clicked() { self.state.model.select(feature.id); }
                    }
                });
        }

        // ── Right drawer ──────────────────────────────────────────────────
        {
            let t_surface = self.state.theme.bg_surface;
            let t_border  = self.state.theme.border_subtle;
            egui::SidePanel::right("right_drawer")
                .default_width(308.0)
                .frame(Frame::new().fill(t_surface).stroke(Stroke::new(1.0, t_border)))
                .show(ctx, |ui| {
                    let t = &self.state.theme;

                    // ── Tab bar ───────────────────────────────────────────
                    let tab_h = 38.0;
                    let avail_w = ui.available_width();
                    let (tab_bar_rect, _) = ui.allocate_exact_size(Vec2::new(avail_w, tab_h), egui::Sense::hover());

                    // Underline
                    ui.painter().rect_filled(
                        Rect::from_min_size(Pos2::new(tab_bar_rect.min.x, tab_bar_rect.max.y - 1.0),
                            Vec2::new(avail_w, 1.0)), 0.0, t.border_subtle);

                    let tabs = [
                        (forge_ui_state::DrawerTab::Properties, "Properties"),
                        (forge_ui_state::DrawerTab::Chat, "Chat"),
                    ];
                    let tab_w = avail_w / tabs.len() as f32;

                    for (idx, (tab, label)) in tabs.iter().enumerate() {
                        let is_active = self.state.drawer.active_tab == *tab;
                        let tab_x = tab_bar_rect.min.x + idx as f32 * tab_w;
                        let this_rect = Rect::from_min_size(Pos2::new(tab_x, tab_bar_rect.min.y), Vec2::new(tab_w, tab_h));

                        let text_color = if is_active { t.text_primary } else { t.text_muted };
                        let galley = ui.fonts(|f| f.layout_no_wrap(label.to_string(), egui::FontId::proportional(t.font_size_sm), text_color));
                        let text_pos = this_rect.center() - Vec2::new(galley.size().x / 2.0, galley.size().y / 2.0);
                        ui.painter().galley(text_pos, galley, text_color);

                        if is_active {
                            ui.painter().rect_filled(
                                Rect::from_min_size(
                                    Pos2::new(this_rect.min.x + 16.0, this_rect.max.y - 2.0),
                                    Vec2::new(this_rect.width() - 32.0, 2.0),
                                ), CornerRadius::same(1), t.accent_primary);
                        }

                        let resp = ui.interact(this_rect, egui::Id::new(format!("tab_{idx}")), egui::Sense::click());
                        if resp.clicked() { self.state.drawer.active_tab = *tab; }
                    }

                    ui.add_space(8.0);

                    match self.state.drawer.active_tab {
                        forge_ui_state::DrawerTab::Properties => draw_properties_panel(ui, &mut self.state),
                        forge_ui_state::DrawerTab::Chat       => draw_chat_panel(ui, &mut self.state),
                    }
                });
        }

        // ── Central viewport — interactive 3D canvas ─────────────────────
        egui::CentralPanel::default()
            .frame(Frame::new().fill(self.state.theme.bg_base))
            .show(ctx, |ui| {
                let vp = &mut self.viewport;
                let t  = &self.state.theme;
                let rect = ui.available_rect_before_wrap();

                // ── Background ────────────────────────────────────────────
                ui.painter().rect_filled(rect, 0.0, t.bg_base);

                // ── WASD / Mouse Aim Navigation ───────────────────────────
                let dt = ctx.input(|i| i.stable_dt).min(0.05);
                let speed = 5.0 * dt;
                let input = ctx.input(|i| i.clone());
                
                // Mouse aim via drag on the background
                let canvas_resp = ui.interact(rect, egui::Id::new("viewport_bg"), egui::Sense::drag());
                if canvas_resp.dragged() {
                    let delta = canvas_resp.drag_delta();
                    vp.camera.yaw -= delta.x * 0.005;
                    vp.camera.pitch -= delta.y * 0.005;
                    vp.camera.pitch = vp.camera.pitch.clamp(-1.5, 1.5);
                }
                
                // Scroll to zoom
                if canvas_resp.hovered() {
                    let scroll = ctx.input(|i| i.smooth_scroll_delta.y);
                    if scroll != 0.0 {
                        vp.camera.fov_y -= scroll * 0.001;
                        vp.camera.fov_y = vp.camera.fov_y.clamp(10.0_f32.to_radians(), 120.0_f32.to_radians());
                    }
                }

                let mut move_dir = Vec3::ZERO;
                if input.key_down(Key::W) || input.key_down(Key::ArrowUp)    { move_dir += vp.camera.forward(); }
                if input.key_down(Key::S) || input.key_down(Key::ArrowDown)  { move_dir -= vp.camera.forward(); }
                if input.key_down(Key::A) || input.key_down(Key::ArrowLeft)  { move_dir -= vp.camera.right(); }
                if input.key_down(Key::D) || input.key_down(Key::ArrowRight) { move_dir += vp.camera.right(); }
                
                // Vertical move with Q/E
                if input.key_down(Key::Q) { move_dir.y -= 1.0; }
                if input.key_down(Key::E) { move_dir.y += 1.0; }

                if move_dir.length_squared() > 0.01 {
                    vp.camera.pos += move_dir.normalize() * speed;
                    ctx.request_repaint(); // Keep repainting while moving
                }

                // ── 3D Projection & Drawing ───────────────────────────────
                let aspect = rect.width() / rect.height().max(1.0);
                let vp_mat = vp.camera.view_proj(aspect);
                
                let project = |pos: Vec3| -> Option<Pos2> {
                    let mut p = vp_mat * Vec4::new(pos.x, pos.y, pos.z, 1.0);
                    if p.w <= 0.0 { return None; } // behind camera
                    p /= p.w;
                    // NDC to screen
                    let x = rect.min.x + (p.x * 0.5 + 0.5) * rect.width();
                    let y = rect.min.y + (0.5 - p.y * 0.5) * rect.height();
                    Some(Pos2::new(x, y))
                };

                let size = 1.0;
                let c = vp.cube_center;
                let d = size / 2.0;
                let vertices = [
                    c + Vec3::new(-d, -d, -d), c + Vec3::new( d, -d, -d),
                    c + Vec3::new( d,  d, -d), c + Vec3::new(-d,  d, -d),
                    c + Vec3::new(-d, -d,  d), c + Vec3::new( d, -d,  d),
                    c + Vec3::new( d,  d,  d), c + Vec3::new(-d,  d,  d),
                ];
                let edges = [
                    (0,1), (1,2), (2,3), (3,0), // back face
                    (4,5), (5,6), (6,7), (7,4), // front face
                    (0,4), (1,5), (2,6), (3,7), // connecting edges
                ];

                let stroke = Stroke::new(2.0, t.accent_primary);
                for &(i, j) in &edges {
                    if let (Some(p1), Some(p2)) = (project(vertices[i]), project(vertices[j])) {
                        ui.painter().line_segment([p1, p2], stroke);
                    }
                }


                // ── HUD overlay ───────────────────────────────────────────
                // Mini key hint at bottom-left
                let hint = "WASD/↑↓←→ move  ·  QE up/down  ·  mouse drag aim  ·  scroll zoom";
                let hg = ui.fonts(|f| f.layout_no_wrap(hint.to_string(), egui::FontId::proportional(10.5), t.text_muted));
                ui.painter().galley(Pos2::new(rect.min.x + 12.0, rect.max.y - 20.0), hg, t.text_muted);
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
                        .shadow(egui::Shadow { offset: [0, 8], blur: 32, spread: 0, color: Color32::from_black_alpha(120) })
                        .show(ui, |ui| {
                            ui.set_width(520.0);
                            ui.add_space(4.0);
                            ui.horizontal(|ui| {
                                ui.add_space(12.0);
                                ui.label(egui::RichText::new("⌘K").color(t.text_muted).size(t.font_size_sm));
                                ui.add_space(6.0);
                                let resp = ui.add(
                                    egui::TextEdit::singleline(&mut self.state.palette.query)
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
    }
}

// ── Properties panel ─────────────────────────────────────────────────────────

fn draw_properties_panel(ui: &mut egui::Ui, state: &mut AppState) {
    let t = &state.theme;
    ui.add_space(4.0);

    if let Some(_id) = state.model.selected() {
        if let Some(plane) = state.model.planes().iter().next() {
            Frame::new()
                .fill(t.accent_subtle)
                .corner_radius(CornerRadius::same(t.radius_sm as u8))
                .inner_margin(egui::Margin { left: 8, right: 8, top: 3, bottom: 3 })
                .show(ui, |ui| {
                    ui.label(egui::RichText::new("Selected Plane")
                        .color(t.accent_primary).size(t.font_size_xs).strong());
                });
            ui.add_space(10.0);

            for (label, value) in [
                ("Normal X", format!("{:.4}", plane.normal[0])),
                ("Normal Y", format!("{:.4}", plane.normal[1])),
                ("Normal Z", format!("{:.4}", plane.normal[2])),
                ("Offset D", format!("{:.4}", plane.offset)),
            ] {
                ui.horizontal(|ui| {
                    ui.add_space(4.0);
                    ui.label(egui::RichText::new(label).color(t.text_muted).size(t.font_size_sm));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(4.0);
                        ui.label(egui::RichText::new(value).color(t.text_primary).size(t.font_size_sm).monospace());
                    });
                });
            }
        }
    } else {
        ui.add_space(24.0);
        ui.vertical_centered(|ui| {
            ui.label(egui::RichText::new("No selection").color(t.text_secondary).size(t.font_size_md).strong());
            ui.add_space(6.0);
            ui.label(egui::RichText::new("Click a feature in the tree\nor a shape in the viewport.")
                .color(t.text_muted).size(t.font_size_sm));
        });
    }
}

// ── Chat panel ───────────────────────────────────────────────────────────────

fn draw_chat_panel(ui: &mut egui::Ui, state: &mut AppState) {
    let t = &state.theme;
    let available_height = ui.available_height();
    let input_area_h = 84.0;
    let footer_h     = 28.0;

    Frame::new()
        .inner_margin(egui::Margin { left: 12, right: 12, top: 0, bottom: 0 })
        .show(ui, |ui| {

        // ── Message list ─────────────────────────────────────────────────
        egui::ScrollArea::vertical()
            .max_height(available_height - input_area_h - footer_h - 20.0)
            .stick_to_bottom(true)
            .show(ui, |ui| {
                let messages = state.chat.messages().to_vec();
                for msg in &messages {
                    let is_user = matches!(msg.role, forge_ui_types::MessageRole::User);
                    let text = match &msg.content {
                        forge_ui_types::MessageContent::Text(s)                   => s.clone(),
                        forge_ui_types::MessageContent::CodeBlock { source, .. }  => source.clone(),
                        forge_ui_types::MessageContent::KernelEvent(s)            => format!("[event] {s}"),
                    };

                    ui.add_space(8.0);
                    if is_user {
                        // User — plain with accent sender chip
                        ui.horizontal(|ui| {
                            Frame::new()
                                .fill(t.accent_subtle)
                                .corner_radius(CornerRadius::same(t.radius_sm as u8))
                                .inner_margin(egui::Margin { left: 6, right: 6, top: 2, bottom: 2 })
                                .show(ui, |ui| {
                                    ui.label(egui::RichText::new("You").color(t.accent_primary).size(t.font_size_xs).strong());
                                });
                        });
                        ui.add_space(3.0);
                        ui.label(egui::RichText::new(&text).color(t.text_primary).size(t.font_size_sm));
                    } else {
                        // Agent/System — card bubble
                        Frame::new()
                            .fill(t.chat_agent_bg)
                            .stroke(Stroke::new(1.0, t.border_subtle))
                            .corner_radius(CornerRadius::same(t.radius_md as u8))
                            .inner_margin(egui::Margin { left: 10, right: 10, top: 8, bottom: 8 })
                            .show(ui, |ui| {
                                let (sender, color) = match msg.role {
                                    forge_ui_types::MessageRole::Agent  => ("Forge",  t.success),
                                    forge_ui_types::MessageRole::System => ("System", t.text_muted),
                                    forge_ui_types::MessageRole::User   => ("You",    t.accent_primary),
                                };
                                ui.label(egui::RichText::new(sender).color(color).size(t.font_size_xs).strong());
                                ui.add_space(3.0);
                                ui.label(egui::RichText::new(&text).color(t.text_secondary).size(t.font_size_sm));
                                ui.add_space(6.0);
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new("Good 👍").color(t.text_muted).size(t.font_size_xs));
                                    ui.add_space(4.0);
                                    ui.label(egui::RichText::new("Bad 👎").color(t.text_muted).size(t.font_size_xs));
                                });
                            });
                    }
                }
                ui.add_space(8.0);
            });

        ui.separator();
        ui.add_space(4.0);

        // ── Input area ───────────────────────────────────────────────────
        Frame::new()
            .fill(t.bg_raised)
            .stroke(Stroke::new(1.0, t.border_default))
            .corner_radius(CornerRadius::same(t.radius_md as u8))
            .inner_margin(egui::Margin { left: 10, right: 10, top: 8, bottom: 8 })
            .show(ui, |ui| {
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
                    ui.label(egui::RichText::new("Forge · v0.1").color(t.text_muted).size(t.font_size_xs));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Painted circle send button
                        let send_ready = !state.chat.input_draft.trim().is_empty();
                        let (btn_rect, btn_resp) = ui.allocate_exact_size(Vec2::splat(26.0), egui::Sense::click());
                        if ui.is_rect_visible(btn_rect) {
                            let c = btn_rect.center();
                            let bg  = if send_ready { t.accent_primary } else { t.bg_raised };
                            let bdr = if send_ready { t.accent_primary } else { t.border_default };
                            ui.painter().circle_filled(c, 11.0, bg);
                            ui.painter().circle_stroke(c, 11.0, Stroke::new(1.0, bdr));
                            let arrow_col = if send_ready { Color32::WHITE } else { t.text_muted };
                            ui.painter().add(egui::Shape::convex_polygon(
                                vec![Pos2::new(c.x+4.5, c.y), Pos2::new(c.x-2.5, c.y-4.0), Pos2::new(c.x-2.5, c.y+4.0)],
                                arrow_col, Stroke::NONE,
                            ));
                        }
                        if btn_resp.clicked() && send_ready { state.chat.submit_draft(); }
                    });
                });
            });

    }); // end padding frame

    // Footer
    ui.add_space(4.0);
    ui.horizontal(|ui| {
        ui.add_space(12.0);
        ui.label(egui::RichText::new("▲ Planning").color(t.text_muted).size(t.font_size_xs));
        ui.add_space(4.0);
        ui.label(egui::RichText::new("·").color(t.border_default).size(t.font_size_xs));
        ui.add_space(4.0);
        ui.label(egui::RichText::new("Forge Kernel v0.1").color(t.text_muted).size(t.font_size_xs));
    });
}
