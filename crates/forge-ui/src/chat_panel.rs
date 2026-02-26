//! Chat panel — message list + input area.

use eframe::egui;
use egui::{Color32, CornerRadius, Frame, Pos2, Stroke, Vec2};
use forge_ui_state::AppState;

/// Draw the chat panel content.
pub fn draw_chat_panel(ui: &mut egui::Ui, state: &mut AppState) {
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
