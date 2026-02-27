//! Viewport organism — 3D camera, sky/ground rendering, wireframe cube.

use eframe::egui;
use egui::{Color32, Frame, Key, Pos2, Rect, Stroke};
use forge_ui_theme::ForgeTheme;
use glam::{Mat4, Vec3, Vec4};

// ── Camera ────────────────────────────────────────────────────────────────────

pub struct Camera {
    pub pos: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub fov_y: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            pos: Vec3::new(0.0, 2.0, 5.0),
            yaw: 0.0,
            pitch: -0.3,
            fov_y: 60.0_f32.to_radians(),
        }
    }

    pub fn view_proj(&self, aspect: f32) -> Mat4 {
        let rotation = glam::Quat::from_euler(glam::EulerRot::YXZ, self.yaw, self.pitch, 0.0);
        let forward = rotation * Vec3::NEG_Z;
        let up = rotation * Vec3::Y;
        let view = Mat4::look_at_rh(self.pos, self.pos + forward, up);
        let proj = Mat4::perspective_rh(self.fov_y, aspect, 0.1, 100.0);
        proj * view
    }

    pub fn forward(&self) -> Vec3 {
        let rotation = glam::Quat::from_euler(glam::EulerRot::YXZ, self.yaw, 0.0, 0.0);
        rotation * Vec3::NEG_Z
    }

    pub fn right(&self) -> Vec3 {
        let rotation = glam::Quat::from_euler(glam::EulerRot::YXZ, self.yaw, 0.0, 0.0);
        rotation * Vec3::X
    }
}

// ── ViewportState ─────────────────────────────────────────────────────────────

pub struct ViewportState {
    pub camera: Camera,
    pub cube_center: Vec3,
    pub mouse_captured: bool,
    pub time_of_day: f32,
}

impl ViewportState {
    pub fn new() -> Self {
        Self {
            camera: Camera::new(),
            cube_center: Vec3::ZERO,
            mouse_captured: false,
            time_of_day: 0.25,
        }
    }
}

// ── Drawing ───────────────────────────────────────────────────────────────────

/// Draw the 3D viewport in the central panel.
pub fn draw_viewport(ctx: &egui::Context, vp: &mut ViewportState, theme: &ForgeTheme) {
    egui::CentralPanel::default()
        .frame(Frame::new().fill(theme.bg_base))
        .show(ctx, |ui| {
            let t = theme;
            let rect = ui.available_rect_before_wrap();

            // ── Day / night cycle ────────────────────────────────────
            vp.time_of_day = (vp.time_of_day + ctx.input(|i| i.stable_dt) * (1.0 / 90.0)).fract();
            ctx.request_repaint();

            let t_day = vp.time_of_day;
            let sun_angle = t_day * std::f32::consts::TAU;
            let sun_y = (sun_angle - std::f32::consts::PI * 0.5).sin();
            let daytime = ((sun_y + 1.0) * 0.5).powf(0.4);

            let mid_top = Color32::from_rgb(3, 5, 20);
            let mid_hor = Color32::from_rgb(5, 10, 30);
            let ss_top = Color32::from_rgb(40, 30, 100);
            let ss_hor = Color32::from_rgb(255, 110, 30);
            let noon_top = Color32::from_rgb(18, 90, 195);
            let noon_hor = Color32::from_rgb(130, 200, 255);

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

            let sky_top = lerp_col(
                lerp_col(mid_top, ss_top, rise_t),
                noon_top,
                daytime * (1.0 - rise_t),
            );
            let sky_hor = lerp_col(
                lerp_col(mid_hor, ss_hor, rise_t),
                noon_hor,
                daytime * (1.0 - rise_t),
            );

            // ── Sky gradient mesh ─────────────────────────────────────
            draw_sky_mesh(ui, rect, sky_top, sky_hor, daytime, &lerp_col);

            // ── Stars ─────────────────────────────────────────────────
            draw_stars(ui, rect, daytime, t_day);

            // ── Horizon glow ──────────────────────────────────────────
            draw_horizon_glow(ui, rect, rise_t);

            // ── Camera controls ───────────────────────────────────────
            handle_camera_controls(ctx, ui, vp, rect);

            // ── 3D Projection & Drawing ───────────────────────────────
            let aspect = rect.width() / rect.height().max(1.0);
            let vp_mat = vp.camera.view_proj(aspect);

            let project = |pos: Vec3| -> Option<Pos2> {
                let mut p = vp_mat * Vec4::new(pos.x, pos.y, pos.z, 1.0);
                if p.w <= 0.0 {
                    return None;
                }
                p /= p.w;
                let x = rect.min.x + (p.x * 0.5 + 0.5) * rect.width();
                let y = rect.min.y + (0.5 - p.y * 0.5) * rect.height();
                Some(Pos2::new(x, y))
            };

            // Wireframe cube
            draw_wireframe_cube(ui, &project, vp.cube_center, t.accent_primary);

            // Sun / Moon
            draw_sun_moon(ui, &project, rect, sun_angle, sun_y, daytime, &lerp_col);

            // Ground grid
            draw_ground_grid(ui, &project, daytime, &lerp_col);

            // HUD hint
            let hint = if vp.mouse_captured {
                "● LOCKED  ·  WASD move  ·  Space/Shift up/down  ·  scroll zoom  ·  Esc to release"
            } else {
                "Click viewport to look  ·  scroll to zoom"
            };
            let hg = ui.fonts(|f| {
                f.layout_no_wrap(
                    hint.to_string(),
                    egui::FontId::proportional(10.5),
                    t.text_muted,
                )
            });
            ui.painter().galley(
                Pos2::new(rect.min.x + 12.0, rect.max.y - 20.0),
                hg,
                t.text_muted,
            );
        });
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn draw_sky_mesh(
    ui: &mut egui::Ui,
    rect: Rect,
    sky_top: Color32,
    sky_hor: Color32,
    daytime: f32,
    lerp_col: &dyn Fn(Color32, Color32, f32) -> Color32,
) {
    use egui::epaint::{Mesh, Vertex, WHITE_UV};
    let tl = rect.left_top();
    let tr = rect.right_top();
    let bl = rect.left_bottom();
    let br = rect.right_bottom();
    let mut mesh = Mesh::default();
    let horizon_y = rect.min.y + rect.height() * 0.55;
    let hl = Pos2::new(rect.min.x, horizon_y);
    let hr = Pos2::new(rect.max.x, horizon_y);

    let add = |m: &mut Mesh, p: Pos2, c: Color32| {
        m.vertices.push(Vertex {
            pos: p,
            uv: WHITE_UV,
            color: c,
        });
    };
    add(&mut mesh, tl, sky_top);
    add(&mut mesh, tr, sky_top);
    add(&mut mesh, hr, sky_hor);
    add(&mut mesh, hl, sky_hor);
    mesh.indices.extend_from_slice(&[0, 1, 2, 0, 2, 3]);

    let ground_top = lerp_col(
        Color32::from_rgb(40, 50, 35),
        Color32::from_rgb(60, 70, 50),
        daytime,
    );
    let ground_bot = lerp_col(
        Color32::from_rgb(20, 25, 18),
        Color32::from_rgb(35, 45, 28),
        daytime,
    );
    let base = mesh.vertices.len() as u32;
    add(&mut mesh, hl, ground_top);
    add(&mut mesh, hr, ground_top);
    add(&mut mesh, br, ground_bot);
    add(&mut mesh, bl, ground_bot);
    mesh.indices
        .extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);

    ui.painter()
        .add(egui::Shape::Mesh(std::sync::Arc::new(mesh)));
}

fn draw_stars(ui: &mut egui::Ui, rect: Rect, daytime: f32, t_day: f32) {
    let star_alpha = ((1.0 - daytime) * 255.0) as u8;
    if star_alpha <= 10 {
        return;
    }

    let stars: &[(f32, f32, f32)] = &[
        (0.06, 0.05, 1.5),
        (0.18, 0.09, 1.0),
        (0.32, 0.03, 2.0),
        (0.45, 0.12, 1.2),
        (0.57, 0.04, 1.8),
        (0.71, 0.07, 1.0),
        (0.84, 0.02, 2.2),
        (0.92, 0.10, 1.3),
        (0.13, 0.20, 1.6),
        (0.29, 0.18, 0.9),
        (0.50, 0.22, 1.4),
        (0.65, 0.16, 2.0),
        (0.78, 0.25, 1.1),
        (0.88, 0.19, 1.7),
        (0.07, 0.32, 1.3),
        (0.40, 0.35, 2.1),
        (0.60, 0.30, 0.8),
        (0.75, 0.40, 1.5),
        (0.20, 0.45, 1.0),
        (0.90, 0.38, 2.0),
        (0.35, 0.50, 1.2),
    ];
    let h_frac = 0.55;
    for &(fx, fy, r) in stars {
        let px = rect.min.x + fx * rect.width();
        let py = rect.min.y + fy * rect.height() * h_frac;
        let twinkle = ((t_day * 500.0 + fx * 137.0).sin() * 0.3 + 0.7).clamp(0.0, 1.0);
        let a = (star_alpha as f32 * twinkle) as u8;
        let col = Color32::from_rgba_unmultiplied(255, 252, 220, a);
        ui.painter().circle_filled(Pos2::new(px, py), r, col);
    }
}

fn draw_horizon_glow(ui: &mut egui::Ui, rect: Rect, rise_t: f32) {
    if rise_t <= 0.05 {
        return;
    }
    let alpha = (rise_t * 180.0) as u8;
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
}

fn handle_camera_controls(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    vp: &mut ViewportState,
    rect: Rect,
) {
    let dt = ctx.input(|i| i.stable_dt).min(0.05);
    let speed = 5.0 * dt;
    let input = ctx.input(|i| i.clone());

    let canvas_resp = ui.interact(rect, egui::Id::new("viewport_bg"), egui::Sense::click());

    if vp.mouse_captured {
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
        ctx.send_viewport_cmd(egui::ViewportCommand::CursorVisible(false));
        let delta = input.pointer.delta();
        if delta.x != 0.0 || delta.y != 0.0 {
            vp.camera.yaw -= delta.x * 0.005;
            vp.camera.pitch -= delta.y * 0.005;
            vp.camera.pitch = vp.camera.pitch.clamp(-1.5, 1.5);
            ctx.request_repaint();
        }
    } else {
        ctx.send_viewport_cmd(egui::ViewportCommand::CursorVisible(true));
    }

    if canvas_resp.hovered() {
        let scroll = ctx.input(|i| i.smooth_scroll_delta.y);
        if scroll != 0.0 {
            vp.camera.fov_y -= scroll * 0.001;
            vp.camera.fov_y = vp
                .camera
                .fov_y
                .clamp(10.0_f32.to_radians(), 120.0_f32.to_radians());
            ctx.request_repaint();
        }
    }

    let mut move_dir = Vec3::ZERO;
    if input.key_down(Key::W) || input.key_down(Key::ArrowUp) {
        move_dir += vp.camera.forward();
    }
    if input.key_down(Key::S) || input.key_down(Key::ArrowDown) {
        move_dir -= vp.camera.forward();
    }
    if input.key_down(Key::A) || input.key_down(Key::ArrowLeft) {
        move_dir -= vp.camera.right();
    }
    if input.key_down(Key::D) || input.key_down(Key::ArrowRight) {
        move_dir += vp.camera.right();
    }
    if input.key_down(Key::Space) {
        move_dir.y += 1.0;
    }
    if input.modifiers.shift {
        move_dir.y -= 1.0;
    }

    if move_dir.length_squared() > 0.01 {
        vp.camera.pos += move_dir.normalize() * speed;
        ctx.request_repaint();
    }
}

fn draw_wireframe_cube(
    ui: &mut egui::Ui,
    project: &dyn Fn(Vec3) -> Option<Pos2>,
    center: Vec3,
    color: Color32,
) {
    let d = 0.5;
    let c = center;
    let vertices = [
        c + Vec3::new(-d, -d, -d),
        c + Vec3::new(d, -d, -d),
        c + Vec3::new(d, d, -d),
        c + Vec3::new(-d, d, -d),
        c + Vec3::new(-d, -d, d),
        c + Vec3::new(d, -d, d),
        c + Vec3::new(d, d, d),
        c + Vec3::new(-d, d, d),
    ];
    let edges = [
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 0),
        (4, 5),
        (5, 6),
        (6, 7),
        (7, 4),
        (0, 4),
        (1, 5),
        (2, 6),
        (3, 7),
    ];
    let stroke = Stroke::new(2.0, color);
    for &(i, j) in &edges {
        if let (Some(p1), Some(p2)) = (project(vertices[i]), project(vertices[j])) {
            ui.painter().line_segment([p1, p2], stroke);
        }
    }
}

fn draw_sun_moon(
    ui: &mut egui::Ui,
    project: &dyn Fn(Vec3) -> Option<Pos2>,
    rect: Rect,
    sun_angle: f32,
    sun_y: f32,
    daytime: f32,
    lerp_col: &dyn Fn(Color32, Color32, f32) -> Color32,
) {
    let sx = sun_angle.cos() * 40.0;
    let sy = (sun_angle - std::f32::consts::PI * 0.5).sin() * 40.0;
    let is_day = sun_y > -0.05;
    let body_pos = Vec3::new(sx, sy, 0.0);

    if let Some(sp) = project(body_pos) {
        if rect.contains(sp) {
            if is_day {
                let sun_col = lerp_col(
                    Color32::from_rgb(255, 180, 50),
                    Color32::from_rgb(255, 252, 200),
                    daytime,
                );
                for i in (0..5u8).rev() {
                    let r = 18.0 + i as f32 * 8.0;
                    let a = (60u8).saturating_sub(i * 14);
                    let halo =
                        Color32::from_rgba_unmultiplied(sun_col.r(), sun_col.g(), sun_col.b(), a);
                    ui.painter().circle_filled(sp, r, halo);
                }
                ui.painter().circle_filled(sp, 18.0, sun_col);
                ui.painter()
                    .circle_filled(sp, 10.0, Color32::from_rgb(255, 255, 240));
            } else {
                let moon_col = Color32::from_rgb(210, 215, 230);
                ui.painter().circle_filled(
                    sp,
                    12.0,
                    Color32::from_rgba_unmultiplied(200, 210, 230, 60),
                );
                ui.painter().circle_filled(
                    sp,
                    10.0,
                    Color32::from_rgba_unmultiplied(200, 210, 230, 80),
                );
                ui.painter().circle_filled(sp, 8.0, moon_col);
            }
        }
    }
}

fn draw_ground_grid(
    ui: &mut egui::Ui,
    project: &dyn Fn(Vec3) -> Option<Pos2>,
    daytime: f32,
    lerp_col: &dyn Fn(Color32, Color32, f32) -> Color32,
) {
    let grid_extent = 30i32;
    let step = 1i32;
    let grid_col_base = lerp_col(
        Color32::from_rgba_unmultiplied(60, 120, 80, 60),
        Color32::from_rgba_unmultiplied(100, 180, 110, 100),
        daytime,
    );

    let mut z = -grid_extent;
    while z <= grid_extent {
        let p1 = project(Vec3::new(-grid_extent as f32, 0.0, z as f32));
        let p2 = project(Vec3::new(grid_extent as f32, 0.0, z as f32));
        if let (Some(p1), Some(p2)) = (p1, p2) {
            let dist_fade = 1.0 - (z.abs() as f32 / grid_extent as f32).powi(2);
            let a = (dist_fade * grid_col_base.a() as f32) as u8;
            let c = Color32::from_rgba_unmultiplied(
                grid_col_base.r(),
                grid_col_base.g(),
                grid_col_base.b(),
                a,
            );
            let w = if z == 0 { 1.5 } else { 0.8 };
            ui.painter().line_segment([p1, p2], Stroke::new(w, c));
        }
        z += step;
    }
    let mut x = -grid_extent;
    while x <= grid_extent {
        let p1 = project(Vec3::new(x as f32, 0.0, -grid_extent as f32));
        let p2 = project(Vec3::new(x as f32, 0.0, grid_extent as f32));
        if let (Some(p1), Some(p2)) = (p1, p2) {
            let dist_fade = 1.0 - (x.abs() as f32 / grid_extent as f32).powi(2);
            let a = (dist_fade * grid_col_base.a() as f32) as u8;
            let c = Color32::from_rgba_unmultiplied(
                grid_col_base.r(),
                grid_col_base.g(),
                grid_col_base.b(),
                a,
            );
            let w = if x == 0 { 1.5 } else { 0.8 };
            ui.painter().line_segment([p1, p2], Stroke::new(w, c));
        }
        x += step;
    }
}
