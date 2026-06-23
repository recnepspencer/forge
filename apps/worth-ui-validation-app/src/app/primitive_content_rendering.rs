use egui::{Align2, FontId, Stroke};
use worth_ui::facade::{
    WorthUiPrimitiveContentIconPaintCommand, WorthUiPrimitiveContentItem,
    WorthUiPrimitiveContentReceipt, WorthUiPrimitiveFlowItemFrame,
    WorthUiResolvedAppearanceStateReceipt,
};

use super::primitive_paint_colors::color_from_primitive_with_opacity;

pub(crate) fn render_primitive_content(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    content: &WorthUiPrimitiveContentReceipt,
    item_frames: &[WorthUiPrimitiveFlowItemFrame],
    appearance: &WorthUiResolvedAppearanceStateReceipt,
) {
    for item_frame in item_frames {
        let Some(item) = content.items().get(item_frame.item_index()) else {
            continue;
        };
        let frame = item_frame.frame();
        let item_rect = egui::Rect::from_min_size(
            egui::pos2(rect.left() + frame.x(), rect.top() + frame.y()),
            egui::vec2(frame.width(), frame.height()),
        );
        render_content_item(ui, item_rect, item, appearance);
    }
}

fn render_content_item(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    item: &WorthUiPrimitiveContentItem,
    appearance: &WorthUiResolvedAppearanceStateReceipt,
) {
    match item {
        WorthUiPrimitiveContentItem::Text(text) => {
            render_text(ui, rect, text.text(), text.size_points(), appearance)
        }
        WorthUiPrimitiveContentItem::Icon(icon) => render_icon(
            ui,
            rect,
            icon.paint_command(),
            icon.stroke_width_points(),
            appearance,
        ),
        WorthUiPrimitiveContentItem::Spacer(_) => {}
        WorthUiPrimitiveContentItem::Badge(badge) => {
            render_badge(ui, rect, badge.text(), badge.size_points(), appearance)
        }
        WorthUiPrimitiveContentItem::Divider(divider) => {
            render_divider(ui, rect, divider.thickness_points(), appearance)
        }
    }
}

fn render_text(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    text: &str,
    size_points: f32,
    appearance: &WorthUiResolvedAppearanceStateReceipt,
) {
    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        text,
        FontId::proportional(size_points),
        color_from_primitive_with_opacity(appearance.text_color(), appearance.opacity()),
    );
}

fn render_icon(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    command: WorthUiPrimitiveContentIconPaintCommand,
    stroke_width: f32,
    appearance: &WorthUiResolvedAppearanceStateReceipt,
) {
    let stroke = Stroke::new(
        stroke_width,
        color_from_primitive_with_opacity(appearance.icon_color(), appearance.opacity()),
    );
    match command {
        WorthUiPrimitiveContentIconPaintCommand::Plus => render_plus_icon(ui, rect, stroke),
        WorthUiPrimitiveContentIconPaintCommand::Check => render_check_icon(ui, rect, stroke),
        WorthUiPrimitiveContentIconPaintCommand::Search => render_search_icon(ui, rect, stroke),
        WorthUiPrimitiveContentIconPaintCommand::Info => {
            render_info_icon(ui, rect, stroke, appearance)
        }
        WorthUiPrimitiveContentIconPaintCommand::Warning => render_warning_icon(ui, rect, stroke),
        WorthUiPrimitiveContentIconPaintCommand::NamedSymbol => {
            render_named_symbol(ui, rect, stroke)
        }
    }
}

fn render_plus_icon(ui: &mut egui::Ui, rect: egui::Rect, stroke: Stroke) {
    let arm = rect.width().min(rect.height()) * 0.58;
    let center = rect.center();
    ui.painter().line_segment(
        [
            egui::pos2(center.x - arm * 0.5, center.y),
            egui::pos2(center.x + arm * 0.5, center.y),
        ],
        stroke,
    );
    ui.painter().line_segment(
        [
            egui::pos2(center.x, center.y - arm * 0.5),
            egui::pos2(center.x, center.y + arm * 0.5),
        ],
        stroke,
    );
}

fn render_check_icon(ui: &mut egui::Ui, rect: egui::Rect, stroke: Stroke) {
    let center = rect.center();
    let size = rect.width().min(rect.height());
    ui.painter().line_segment(
        [
            egui::pos2(center.x - size * 0.28, center.y + size * 0.02),
            egui::pos2(center.x - size * 0.07, center.y + size * 0.22),
        ],
        stroke,
    );
    ui.painter().line_segment(
        [
            egui::pos2(center.x - size * 0.07, center.y + size * 0.22),
            egui::pos2(center.x + size * 0.31, center.y - size * 0.24),
        ],
        stroke,
    );
}

fn render_search_icon(ui: &mut egui::Ui, rect: egui::Rect, stroke: Stroke) {
    let center = rect.center();
    let radius = rect.width().min(rect.height()) * 0.24;
    ui.painter().circle_stroke(
        center + egui::vec2(-radius * 0.25, -radius * 0.25),
        radius,
        stroke,
    );
    ui.painter().line_segment(
        [
            center + egui::vec2(radius * 0.42, radius * 0.42),
            center + egui::vec2(radius * 1.1, radius * 1.1),
        ],
        stroke,
    );
}

fn render_info_icon(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    stroke: Stroke,
    appearance: &WorthUiResolvedAppearanceStateReceipt,
) {
    let center = rect.center();
    let radius = rect.width().min(rect.height()) * 0.32;
    ui.painter().circle_stroke(center, radius, stroke);
    render_text(ui, rect, "i", radius * 1.4, appearance);
}

fn render_warning_icon(ui: &mut egui::Ui, rect: egui::Rect, stroke: Stroke) {
    let center = rect.center();
    let size = rect.width().min(rect.height()) * 0.62;
    let top = egui::pos2(center.x, center.y - size * 0.5);
    let left = egui::pos2(center.x - size * 0.55, center.y + size * 0.45);
    let right = egui::pos2(center.x + size * 0.55, center.y + size * 0.45);
    ui.painter().line_segment([top, left], stroke);
    ui.painter().line_segment([left, right], stroke);
    ui.painter().line_segment([right, top], stroke);
    ui.painter().line_segment(
        [
            center + egui::vec2(0.0, -size * 0.18),
            center + egui::vec2(0.0, size * 0.18),
        ],
        stroke,
    );
}

fn render_named_symbol(ui: &mut egui::Ui, rect: egui::Rect, stroke: Stroke) {
    ui.painter().circle_stroke(
        rect.center(),
        rect.width().min(rect.height()) * 0.28,
        stroke,
    );
}

fn render_badge(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    text: &str,
    size_points: f32,
    appearance: &WorthUiResolvedAppearanceStateReceipt,
) {
    let radius = (rect.height() * 0.5).min(appearance.radius_points());
    ui.painter().rect_filled(
        rect,
        radius,
        color_from_primitive_with_opacity(appearance.border_color(), appearance.opacity()),
    );
    render_text(ui, rect, text, size_points, appearance);
}

fn render_divider(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    thickness_points: f32,
    appearance: &WorthUiResolvedAppearanceStateReceipt,
) {
    let center_y = rect.center().y;
    ui.painter().line_segment(
        [
            egui::pos2(rect.left(), center_y),
            egui::pos2(rect.right(), center_y),
        ],
        Stroke::new(
            thickness_points,
            color_from_primitive_with_opacity(appearance.border_color(), appearance.opacity()),
        ),
    );
}
