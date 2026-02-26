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
    /// True when the viewport has captured the mouse (click to enter, Escape to exit).
    mouse_captured: bool,
    /// 0.0 = midnight, 0.25 = sunrise, 0.5 = noon, 0.75 = sunset
    time_of_day: f32,
}

impl ViewportState {
    fn new() -> Self {
        Self {
            camera: Camera::new(),
            cube_center: Vec3::ZERO,
            mouse_captured: false,
            time_of_day: 0.25, // start at sunrise
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

                        // ── ⌘K search (takes remaining center space) ────
                        {
                            let search_w = 280.0_f32;
                            let search_h = 30.0_f32;
                            let (rect, resp) = ui.allocate_exact_size(Vec2::new(search_w, search_h), egui::Sense::click());
                            if ui.is_rect_visible(rect) {
                                let hovered = resp.hovered() || palette_open;
                                let bg = if hovered { bg_raised } else { bg_base };
                                let border = if hovered { border_subtle } else { border_def };
                                ui.painter().rect(rect, CornerRadius::same(radius_md as u8), bg, Stroke::new(1.0, border), egui::StrokeKind::Outside);

                                // Search icon on the left
                                let icon_y = rect.center().y - 6.0;
                                let search_galley = ui.fonts(|f| f.layout_no_wrap("🔍".to_string(), egui::FontId::proportional(11.0), _text_muted));
                                ui.painter().galley(Pos2::new(rect.min.x + 10.0, icon_y), search_galley, _text_muted);

                                // Placeholder text
                                let galley_text = ui.fonts(|f| f.layout_no_wrap("Search operations…".to_string(), egui::FontId::proportional(fsz_sm), _text_muted));
                                ui.painter().galley(Pos2::new(rect.min.x + 28.0, rect.center().y - galley_text.size().y / 2.0), galley_text, _text_muted);

                                // ⌘K badge on the right
                                let badge_text = "⌘K";
                                let badge_galley = ui.fonts(|f| f.layout_no_wrap(badge_text.to_string(), egui::FontId::proportional(10.0), _text_muted));
                                let badge_w = badge_galley.size().x + 8.0;
                                let badge_h = badge_galley.size().y + 4.0;
                                let badge_rect = Rect::from_min_size(
                                    Pos2::new(rect.max.x - badge_w - 8.0, rect.center().y - badge_h / 2.0),
                                    Vec2::new(badge_w, badge_h),
                                );
                                ui.painter().rect(badge_rect, CornerRadius::same(3), border_def, Stroke::NONE, egui::StrokeKind::Outside);
                                ui.painter().galley(badge_rect.min + Vec2::new(4.0, 2.0), badge_galley, _text_muted);
                            }
                            if resp.clicked() { self.state.palette.toggle(); }
                        }

                        // ── Spacer → push theme toggle to the right ────
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.add_space(16.0);
                            // Theme toggle using SVG icon
                            let theme_fg_icon = match self.state.theme_kind {
                                forge_ui_state::ThemeKind::Dark  => FgIcon::Sun,
                                forge_ui_state::ThemeKind::Light => FgIcon::Moon,
                            };
                            let (rect, resp) = ui.allocate_exact_size(Vec2::new(28.0, 28.0), egui::Sense::click());
                            if ui.is_rect_visible(rect) {
                                if resp.hovered() {
                                    ui.painter().rect_filled(rect, CornerRadius::same(radius_sm as u8), bg_raised);
                                }
                                // Draw centered SVG icon
                                if let Some(tex) = self.icons.textures.get(&theme_fg_icon) {
                                    let icon_size = 16.0;
                                    let icon_pos = rect.center() - Vec2::splat(icon_size / 2.0);
                                    let sized = egui::load::SizedTexture::new(tex.id(), [icon_size, icon_size]);
                                    let img = egui::Image::from_texture(sized).tint(text_sec);
                                    img.paint_at(ui, Rect::from_min_size(icon_pos, Vec2::splat(icon_size)));
                                }
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

                // ── Day / night cycle ────────────────────────────────────
                // Advance time (full cycle in ~90 seconds real time).
                vp.time_of_day = (vp.time_of_day + ctx.input(|i| i.stable_dt) * (1.0 / 90.0)).fract();
                ctx.request_repaint(); // always animate

                let t_day = vp.time_of_day; // 0.0=midnight  0.5=noon
                // Map to a 0-1 "brightness" with smooth sunrise/sunset peaks
                let sun_angle = t_day * std::f32::consts::TAU; // radians
                // y component of sun: +1 = zenith, -1 = nadir
                let sun_y = (sun_angle - std::f32::consts::PI * 0.5).sin(); // -1..+1
                let daytime = ((sun_y + 1.0) * 0.5).powf(0.4); // 0=night, 1=full day

                // Sky color key-frames (top / horizon)
                // midnight
                let mid_top  = Color32::from_rgb(3,  5, 20);
                let mid_hor  = Color32::from_rgb(5, 10, 30);
                // sunrise/sunset
                let ss_top   = Color32::from_rgb(40, 30, 100);
                let ss_hor   = Color32::from_rgb(255, 110, 30);
                // noon
                let noon_top = Color32::from_rgb(18, 90, 195);
                let noon_hor = Color32::from_rgb(130, 200, 255);

                // sunrise factor: peaks when sun is near horizon going up
                let rise_t = {
                    let h = (sun_y.abs() - 0.0).max(0.0);
                    (1.0 - (h / 0.35).min(1.0)).powi(2) * if sun_y > -0.1 { 1.0 } else { 0.0 }
                };

                let lerp_col = |a: Color32, b: Color32, t: f32| -> Color32 {
                    Color32::from_rgb(
                        (a.r() as f32 + (b.r() as f32 - a.r() as f32) * t) as u8,
                        (a.g() as f32 + (b.g() as f32 - a.g() as f32) * t) as u8,
                        (a.b() as f32 + (b.b() as f32 - a.b() as f32) * t) as u8,
                    )
                };

                // Blend: night → sunrise → noon → sunset → night
                let sky_top = lerp_col(
                    lerp_col(mid_top, ss_top, rise_t),
                    noon_top, daytime * (1.0 - rise_t),
                );
                let sky_hor = lerp_col(
                    lerp_col(mid_hor, ss_hor, rise_t),
                    noon_hor, daytime * (1.0 - rise_t),
                );

                // ── Sky gradient mesh ─────────────────────────────────────
                {
                    use egui::epaint::{Mesh, Vertex, WHITE_UV};
                    let tl = rect.left_top();
                    let tr = rect.right_top();
                    let bl = rect.left_bottom();
                    let br = rect.right_bottom();
                    let mut mesh = Mesh::default();
                    let horizon_y = rect.min.y + rect.height() * 0.55; // horizon sits slightly below center

                    // Top strip (sky): tl → tr → horizon_l → horizon_r
                    let hl = Pos2::new(rect.min.x, horizon_y);
                    let hr = Pos2::new(rect.max.x, horizon_y);

                    let add = |m: &mut Mesh, p: Pos2, c: Color32| {
                        m.vertices.push(Vertex { pos: p, uv: WHITE_UV, color: c }); };
                    add(&mut mesh, tl, sky_top); // 0
                    add(&mut mesh, tr, sky_top); // 1
                    add(&mut mesh, hr, sky_hor); // 2
                    add(&mut mesh, hl, sky_hor); // 3
                    mesh.indices.extend_from_slice(&[0,1,2, 0,2,3]);

                    // Ground strip (below horizon): dark earth tone
                    let ground_top = lerp_col(Color32::from_rgb(40, 50, 35), Color32::from_rgb(60, 70, 50), daytime);
                    let ground_bot = lerp_col(Color32::from_rgb(20, 25, 18), Color32::from_rgb(35, 45, 28), daytime);
                    let base = mesh.vertices.len() as u32;
                    add(&mut mesh, hl,  ground_top); // base+0
                    add(&mut mesh, hr,  ground_top); // base+1
                    add(&mut mesh, br,  ground_bot); // base+2
                    add(&mut mesh, bl,  ground_bot); // base+3
                    mesh.indices.extend_from_slice(&[base,base+1,base+2, base,base+2,base+3]);

                    ui.painter().add(egui::Shape::Mesh(std::sync::Arc::new(mesh)));
                }

                // ── Stars (visible at night, fade out at day) ─────────────
                let star_alpha = ((1.0 - daytime) * 255.0) as u8;
                if star_alpha > 10 {
                    // Fixed pseudo-random star positions
                    let stars: &[(f32, f32, f32)] = &[
                        (0.06, 0.05, 1.5), (0.18, 0.09, 1.0), (0.32, 0.03, 2.0),
                        (0.45, 0.12, 1.2), (0.57, 0.04, 1.8), (0.71, 0.07, 1.0),
                        (0.84, 0.02, 2.2), (0.92, 0.10, 1.3), (0.13, 0.20, 1.6),
                        (0.29, 0.18, 0.9), (0.50, 0.22, 1.4), (0.65, 0.16, 2.0),
                        (0.78, 0.25, 1.1), (0.88, 0.19, 1.7), (0.07, 0.32, 1.3),
                        (0.40, 0.35, 2.1), (0.60, 0.30, 0.8), (0.75, 0.40, 1.5),
                        (0.20, 0.45, 1.0), (0.90, 0.38, 2.0), (0.35, 0.50, 1.2),
                    ];
                    let h_frac = 0.55; // match horizon
                    for &(fx, fy, r) in stars {
                        let px = rect.min.x + fx * rect.width();
                        let py = rect.min.y + fy * rect.height() * h_frac;
                        let twinkle = ((t_day * 500.0 + fx * 137.0).sin() * 0.3 + 0.7).clamp(0.0, 1.0);
                        let a = (star_alpha as f32 * twinkle) as u8;
                        let col = Color32::from_rgba_unmultiplied(255, 252, 220, a);
                        ui.painter().circle_filled(Pos2::new(px, py), r, col);
                    }
                }

                // ── Horizon glow (sunrise/sunset rim) ────────────────────
                if rise_t > 0.05 {
                    let alpha = (rise_t * 180.0) as u8;
                    let glow_col = Color32::from_rgba_unmultiplied(255, 90, 20, alpha);
                    let horizon_y = rect.min.y + rect.height() * 0.55;
                    for i in 0..6u8 {
                        let spread = i as f32 * 3.0;
                        let a = (alpha as f32 * (1.0 - i as f32 / 6.0)) as u8;
                        let c = Color32::from_rgba_unmultiplied(255, 90, 20, a);
                        ui.painter().hline(
                            rect.min.x..=rect.max.x,
                            horizon_y - spread,
                            Stroke::new(1.5, c),
                        );
                        ui.painter().hline(
                            rect.min.x..=rect.max.x,
                            horizon_y + spread,
                            Stroke::new(1.5, c),
                        );
                    }
                    let _ = glow_col;
                }

                // ── WASD / Mouse Aim Navigation ───────────────────────────
                let dt = ctx.input(|i| i.stable_dt).min(0.05);
                let speed = 5.0 * dt;
                let input = ctx.input(|i| i.clone());

                // ── Click-to-lock mouse capture ───────────────────────────
                // Click viewport → lock + hide cursor; Click again or Esc → release.
                let canvas_resp = ui.interact(rect, egui::Id::new("viewport_bg"), egui::Sense::click());

                if vp.mouse_captured {
                    // Any click anywhere releases the lock (cursor is hidden so normal
                    // hit-testing on canvas_resp doesn't fire).
                    if input.pointer.any_click() {
                        vp.mouse_captured = false;
                    }
                    if input.key_pressed(Key::Escape) {
                        vp.mouse_captured = false;
                    }
                } else if canvas_resp.clicked() {
                    vp.mouse_captured = true;
                }

                if vp.mouse_captured {
                    // Hide cursor and continuously read delta (no button needed).
                    ctx.send_viewport_cmd(egui::ViewportCommand::CursorVisible(false));
                    let delta = input.pointer.delta();
                    if delta.x != 0.0 || delta.y != 0.0 {
                        vp.camera.yaw   -= delta.x * 0.005;
                        vp.camera.pitch -= delta.y * 0.005;
                        vp.camera.pitch  = vp.camera.pitch.clamp(-1.5, 1.5);
                        ctx.request_repaint();
                    }
                } else {
                    ctx.send_viewport_cmd(egui::ViewportCommand::CursorVisible(true));
                }

                // ── Scroll to zoom ─────────────────────────────────────────
                if canvas_resp.hovered() {
                    let scroll = ctx.input(|i| i.smooth_scroll_delta.y);
                    if scroll != 0.0 {
                        vp.camera.fov_y -= scroll * 0.001;
                        vp.camera.fov_y = vp.camera.fov_y.clamp(10.0_f32.to_radians(), 120.0_f32.to_radians());
                        ctx.request_repaint();
                    }
                }

                // ── WASD movement ─────────────────────────────────────────
                let mut move_dir = Vec3::ZERO;
                if input.key_down(Key::W) || input.key_down(Key::ArrowUp)    { move_dir += vp.camera.forward(); }
                if input.key_down(Key::S) || input.key_down(Key::ArrowDown)  { move_dir -= vp.camera.forward(); }
                if input.key_down(Key::A) || input.key_down(Key::ArrowLeft)  { move_dir -= vp.camera.right(); }
                if input.key_down(Key::D) || input.key_down(Key::ArrowRight) { move_dir += vp.camera.right(); }
                if input.key_down(Key::Space)   { move_dir.y += 1.0; }
                if input.modifiers.shift        { move_dir.y -= 1.0; }

                if move_dir.length_squared() > 0.01 {
                    vp.camera.pos += move_dir.normalize() * speed;
                    ctx.request_repaint();
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

                // ── Sun / Moon ────────────────────────────────────────────
                {
                    // Sun orbits in XZ plane at Y=40, X-component tracks time
                    let sx = (sun_angle).cos() * 40.0;
                    let sy = (sun_angle - std::f32::consts::PI * 0.5).sin() * 40.0;
                    let sz = 0.0_f32;

                    let is_day = sun_y > -0.05;
                    let body_pos = Vec3::new(sx, sy, sz);

                    if let Some(sp) = project(body_pos) {
                        if rect.contains(sp) {
                            if is_day {
                                // Sun: bright disc with atmospheric halo layers
                                let sun_col = lerp_col(Color32::from_rgb(255, 180, 50), Color32::from_rgb(255, 252, 200), daytime);
                                for i in (0..5u8).rev() {
                                    let r = 18.0 + i as f32 * 8.0;
                                    let a = (60u8).saturating_sub(i * 14);
                                    let halo = Color32::from_rgba_unmultiplied(sun_col.r(), sun_col.g(), sun_col.b(), a);
                                    ui.painter().circle_filled(sp, r, halo);
                                }
                                ui.painter().circle_filled(sp, 18.0, sun_col);
                                // Bright core
                                ui.painter().circle_filled(sp, 10.0, Color32::from_rgb(255, 255, 240));
                            } else {
                                // Moon: cool grey disc with subtle glow
                                let moon_col = Color32::from_rgb(210, 215, 230);
                                ui.painter().circle_filled(sp, 12.0, Color32::from_rgba_unmultiplied(200, 210, 230, 60));
                                ui.painter().circle_filled(sp, 10.0, Color32::from_rgba_unmultiplied(200, 210, 230, 80));
                                ui.painter().circle_filled(sp,  8.0, moon_col);
                            }
                        }
                    }
                }

                // ── Ground grid (XZ plane at Y=0) ─────────────────────────
                {
                    let grid_extent = 30i32;
                    let step = 1i32;
                    let grid_col_base = lerp_col(
                        Color32::from_rgba_unmultiplied(60, 120, 80, 60),
                        Color32::from_rgba_unmultiplied(100, 180, 110, 100),
                        daytime,
                    );

                    // Draw lines parallel to X axis (varying Z)
                    let mut z = -grid_extent;
                    while z <= grid_extent {
                        let p1 = project(Vec3::new(-grid_extent as f32, 0.0, z as f32));
                        let p2 = project(Vec3::new( grid_extent as f32, 0.0, z as f32));
                        if let (Some(p1), Some(p2)) = (p1, p2) {
                            let dist_fade = 1.0 - (z.abs() as f32 / grid_extent as f32).powi(2);
                            let a = (dist_fade * grid_col_base.a() as f32) as u8;
                            let c = Color32::from_rgba_unmultiplied(grid_col_base.r(), grid_col_base.g(), grid_col_base.b(), a);
                            let w = if z == 0 { 1.5 } else { 0.8 };
                            ui.painter().line_segment([p1, p2], Stroke::new(w, c));
                        }
                        z += step;
                    }
                    // Lines parallel to Z axis (varying X)
                    let mut x = -grid_extent;
                    while x <= grid_extent {
                        let p1 = project(Vec3::new(x as f32, 0.0, -grid_extent as f32));
                        let p2 = project(Vec3::new(x as f32, 0.0,  grid_extent as f32));
                        if let (Some(p1), Some(p2)) = (p1, p2) {
                            let dist_fade = 1.0 - (x.abs() as f32 / grid_extent as f32).powi(2);
                            let a = (dist_fade * grid_col_base.a() as f32) as u8;
                            let c = Color32::from_rgba_unmultiplied(grid_col_base.r(), grid_col_base.g(), grid_col_base.b(), a);
                            let w = if x == 0 { 1.5 } else { 0.8 };
                            ui.painter().line_segment([p1, p2], Stroke::new(w, c));
                        }
                        x += step;
                    }
                }


                // Mini key hint at bottom-left
                let hint = if vp.mouse_captured {
                    "● LOCKED  ·  WASD move  ·  Space/Shift up/down  ·  scroll zoom  ·  Esc to release"
                } else {
                    "Click viewport to look  ·  scroll to zoom"
                };
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
