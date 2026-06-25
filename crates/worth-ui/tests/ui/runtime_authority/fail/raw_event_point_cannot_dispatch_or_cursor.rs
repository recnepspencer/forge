use worth_ui::facade::{
    WorthUiPrimitiveEventDispatchPlan, WorthUiPrimitiveEventHitTestPoint,
};

fn main() {}

fn raw_event_point_cannot_dispatch_or_cursor(
    plan: &WorthUiPrimitiveEventDispatchPlan,
    point: WorthUiPrimitiveEventHitTestPoint,
) {
    let _ = plan.dispatch_primary_click(point);
    let _ = plan.cursor_receipt_at(point);
}
