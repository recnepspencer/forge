use egui::{Color32, Stroke};
use worth_ui::facade::{
    SurfaceId, WorthUiComponentInteractionReceipt, WorthUiPrimitiveEventDispatchPlan,
    WorthUiPrimitiveEventHitTestPoint, WorthUiPrimitiveEventRegionOrder,
    WorthUiPrimitiveEventRegionReceipt, WorthUiPrimitiveObservedPostureReceipt,
    WorthUiPrimitiveProofDenial, WorthUiPrimitiveProofReceipt,
    WorthUiPrimitiveResolvedCursorPosture,
};

use super::primitive_content_rendering::render_primitive_content;
use super::primitive_denial_rendering::render_primitive_denial;
use super::primitive_paint_colors::color_from_primitive_with_opacity;

pub(crate) fn render_centered_primitive_proof(
    ui: &mut egui::Ui,
    primitive: Result<&WorthUiPrimitiveProofReceipt, &WorthUiPrimitiveProofDenial>,
    inner_primitive: Result<&WorthUiPrimitiveProofReceipt, &WorthUiPrimitiveProofDenial>,
    last_interaction: Option<&WorthUiComponentInteractionReceipt>,
    last_interaction_denial: Option<&str>,
) -> Vec<SurfaceId> {
    let available = ui.available_rect_before_wrap();
    let mut clicked_surface_ids = Vec::new();
    ui.scope_builder(egui::UiBuilder::new().max_rect(available), |ui| {
        ui.painter()
            .rect_filled(available, 0.0, Color32::from_rgb(14, 15, 17));
        let primitive = match primitive {
            Ok(primitive) => primitive,
            Err(denial) => {
                render_primitive_denial(ui, denial);
                return;
            }
        };
        let inner_primitive = match inner_primitive {
            Ok(inner_primitive) => Some(inner_primitive),
            Err(denial) => {
                render_primitive_denial(ui, denial);
                return;
            }
        };

        let rest_plan = primitive.draw_plan(available.width(), available.height());
        let rect = primitive_rect(available, &rest_plan);
        let inner_plan = inner_primitive.map(|inner| inner.draw_plan(rect.width(), rect.height()));
        let inner_rect = inner_plan.as_ref().map(|plan| primitive_rect(rect, plan));
        let event_plan = primitive_event_plan(
            primitive,
            &rest_plan,
            inner_primitive,
            inner_plan.as_ref(),
            available,
            rect,
        );
        let pointer = ui.input(|input| input.pointer.hover_pos().or(input.pointer.interact_pos()));
        if let Some(pointer) = pointer {
            let point = WorthUiPrimitiveEventHitTestPoint::new(pointer.x, pointer.y);
            apply_cursor(ui, event_plan.cursor_at(point));
            if ui.input(|input| input.pointer.button_released(egui::PointerButton::Primary)) {
                let dispatch = event_plan.dispatch_primary_click(point);
                clicked_surface_ids = dispatch
                    .emitted_surface_ids()
                    .iter()
                    .filter_map(|surface_id| SurfaceId::new(surface_id).ok())
                    .collect();
            }
        }
        let outer_observed = observed_posture_for_surface(
            ui,
            &event_plan,
            primitive.surface_id(),
            primitive.interaction().selected(),
            primitive.interaction().affordance().disabled_posture(),
        );
        let paint_plan =
            primitive.paint_plan(available.width(), available.height(), outer_observed);
        paint_primitive(ui, rect, primitive, &paint_plan);
        if let (Some(inner), Some(_plan), Some(inner_rect)) =
            (inner_primitive, inner_plan, inner_rect)
        {
            let inner_observed = observed_posture_for_surface(
                ui,
                &event_plan,
                inner.surface_id(),
                inner.interaction().selected(),
                inner.interaction().affordance().disabled_posture(),
            );
            let inner_paint_plan = inner.paint_plan(rect.width(), rect.height(), inner_observed);
            paint_primitive(ui, inner_rect, inner, &inner_paint_plan);
        }
        render_proof_text(
            ui,
            rect,
            primitive,
            last_interaction,
            last_interaction_denial,
        );
    });
    clicked_surface_ids
}

fn primitive_rect(
    available: egui::Rect,
    draw_plan: &worth_ui::facade::WorthUiPrimitiveDrawPlan,
) -> egui::Rect {
    let frame = draw_plan.frame();
    egui::Rect::from_min_size(
        egui::pos2(available.left() + frame.x(), available.top() + frame.y()),
        egui::vec2(frame.width(), frame.height()),
    )
}

fn primitive_event_plan(
    primitive: &WorthUiPrimitiveProofReceipt,
    rest_plan: &worth_ui::facade::WorthUiPrimitiveDrawPlan,
    inner_primitive: Option<&WorthUiPrimitiveProofReceipt>,
    inner_plan: Option<&worth_ui::facade::WorthUiPrimitiveDrawPlan>,
    available: egui::Rect,
    outer_rect: egui::Rect,
) -> WorthUiPrimitiveEventDispatchPlan {
    let mut regions = Vec::new();
    if let Some(region) = WorthUiPrimitiveEventRegionReceipt::from_primitive_draw_plan_at(
        primitive,
        rest_plan,
        WorthUiPrimitiveEventRegionOrder::new(0, 0),
        available.left(),
        available.top(),
    ) {
        regions.push(region);
    }
    if let (Some(inner), Some(plan)) = (inner_primitive, inner_plan) {
        if let Some(region) = WorthUiPrimitiveEventRegionReceipt::from_child_primitive_draw_plan_at(
            inner,
            plan,
            WorthUiPrimitiveEventRegionOrder::new(1, 0),
            primitive.surface_id(),
            outer_rect.left(),
            outer_rect.top(),
        ) {
            regions.push(region);
        }
    }
    WorthUiPrimitiveEventDispatchPlan::from_regions(regions)
}

fn observed_posture_for_surface(
    ui: &egui::Ui,
    event_plan: &WorthUiPrimitiveEventDispatchPlan,
    surface_id: &str,
    selected: bool,
    disabled: bool,
) -> WorthUiPrimitiveObservedPostureReceipt {
    let pointer = ui.input(|input| input.pointer.hover_pos().or(input.pointer.interact_pos()));
    let hovered = pointer
        .and_then(|pointer| {
            event_plan.hit_test(WorthUiPrimitiveEventHitTestPoint::new(pointer.x, pointer.y))
        })
        .is_some_and(|region| region.surface_id() == surface_id);
    let pressed = hovered && ui.input(|input| input.pointer.primary_down());
    WorthUiPrimitiveObservedPostureReceipt::from_renderer_observation(
        hovered, pressed, false, disabled, selected,
    )
}

fn paint_primitive(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    primitive: &WorthUiPrimitiveProofReceipt,
    paint_plan: &worth_ui::facade::WorthUiPrimitivePaintPlan,
) {
    let active_appearance = paint_plan.active_appearance();
    if !active_appearance.background_color().is_transparent() {
        ui.painter().rect_filled(
            rect,
            active_appearance.radius_points(),
            color_from_primitive_with_opacity(
                active_appearance.background_color(),
                active_appearance.opacity(),
            ),
        );
    }
    ui.painter().rect(
        rect,
        active_appearance.radius_points(),
        Color32::TRANSPARENT,
        Stroke::new(
            active_appearance.border_width_points(),
            color_from_primitive_with_opacity(
                active_appearance.border_color(),
                active_appearance.opacity(),
            ),
        ),
        egui::StrokeKind::Outside,
    );
    if active_appearance.focus_ring_width_points() > 0.0 {
        ui.painter().rect_stroke(
            rect.expand(active_appearance.focus_ring_width_points() + 1.0),
            active_appearance.radius_points(),
            Stroke::new(
                active_appearance.focus_ring_width_points(),
                color_from_primitive_with_opacity(
                    active_appearance.focus_ring_color(),
                    active_appearance.opacity(),
                ),
            ),
            egui::StrokeKind::Outside,
        );
    }
    render_primitive_content(
        ui,
        rect,
        primitive.content(),
        paint_plan.draw_plan().item_frames(),
        active_appearance,
    );
}

fn apply_cursor(ui: &egui::Ui, cursor: WorthUiPrimitiveResolvedCursorPosture) {
    let Some(cursor) = egui_cursor(cursor) else {
        return;
    };
    ui.ctx().set_cursor_icon(cursor);
}

fn egui_cursor(cursor: WorthUiPrimitiveResolvedCursorPosture) -> Option<egui::CursorIcon> {
    match cursor {
        WorthUiPrimitiveResolvedCursorPosture::Default => None,
        WorthUiPrimitiveResolvedCursorPosture::Pointer => Some(egui::CursorIcon::PointingHand),
        WorthUiPrimitiveResolvedCursorPosture::Text => Some(egui::CursorIcon::Text),
        WorthUiPrimitiveResolvedCursorPosture::Grab => Some(egui::CursorIcon::Grab),
        WorthUiPrimitiveResolvedCursorPosture::Grabbing => Some(egui::CursorIcon::Grabbing),
        WorthUiPrimitiveResolvedCursorPosture::Resize => Some(egui::CursorIcon::ResizeHorizontal),
        WorthUiPrimitiveResolvedCursorPosture::NotAllowed => Some(egui::CursorIcon::NotAllowed),
    }
}

fn render_proof_text(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    primitive: &WorthUiPrimitiveProofReceipt,
    last_interaction: Option<&WorthUiComponentInteractionReceipt>,
    last_interaction_denial: Option<&str>,
) {
    let text_origin = egui::pos2(rect.left(), rect.bottom() + 18.0);
    let text_rect = egui::Rect::from_min_size(text_origin, egui::vec2(680.0, 118.0));
    ui.scope_builder(egui::UiBuilder::new().max_rect(text_rect), |ui| {
        ui.label(
            egui::RichText::new(format!(
                "surface={} dependency={}:{}",
                primitive.surface_id(),
                primitive.dependency_fact().family().token(),
                primitive.dependency_fact().identity()
            ))
            .color(Color32::from_rgb(146, 150, 156))
            .size(12.0),
        );
        ui.label(
            egui::RichText::new(format!(
                "primitive receipt digest={}",
                primitive.receipt_digest()
            ))
            .color(Color32::from_rgb(195, 204, 216))
            .size(12.0),
        );
        ui.label(
            egui::RichText::new(format!(
                "interaction={} payload={} motion={:?}/{}",
                primitive.interaction().interaction_id(),
                primitive.interaction().submit_payload().digest(),
                primitive.motion().kind(),
                primitive.motion().duration().token()
            ))
            .color(Color32::from_rgb(195, 204, 216))
            .size(12.0),
        );
        ui.label(
            egui::RichText::new(format!(
                "interaction readiness={:?} operability={:?}/{:?}",
                primitive.interaction().readiness(),
                primitive.interaction().operability().posture(),
                primitive.interaction().operability().basis()
            ))
            .color(Color32::from_rgb(195, 204, 216))
            .size(12.0),
        );
        if let Some(denial) = last_interaction_denial {
            ui.label(
                egui::RichText::new(format!("interaction denied={denial}"))
                    .color(Color32::from_rgb(255, 173, 96))
                    .size(12.0),
            );
        } else if let Some(receipt) = last_interaction {
            ui.label(
                egui::RichText::new(format!(
                    "submitted={} kind={} target={} digest={}",
                    receipt.interaction_id(),
                    receipt.kind().token(),
                    interaction_target_text(receipt),
                    receipt.receipt_digest()
                ))
                .color(Color32::from_rgb(195, 204, 216))
                .size(12.0),
            );
            ui.label(
                egui::RichText::new(format!(
                    "submitted payload={}",
                    interaction_payload_text(receipt)
                ))
                .color(Color32::from_rgb(195, 204, 216))
                .size(12.0),
            );
        } else {
            ui.label(
                egui::RichText::new("no submit receipt yet")
                    .color(Color32::from_rgb(108, 112, 118))
                    .size(12.0),
            );
        }
    });
}

fn interaction_target_text(receipt: &WorthUiComponentInteractionReceipt) -> String {
    format!("{:?}", receipt.target())
}

fn interaction_payload_text(receipt: &WorthUiComponentInteractionReceipt) -> String {
    let fields = receipt.payload().fields();
    if fields.is_empty() {
        return "<empty>".to_owned();
    }
    fields
        .iter()
        .map(|field| format!("{}={}", field.name(), field.value().as_text()))
        .collect::<Vec<_>>()
        .join(", ")
}
