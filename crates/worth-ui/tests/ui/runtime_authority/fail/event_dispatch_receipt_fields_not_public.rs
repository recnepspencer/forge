use worth_ui::facade::{
    WorthUiPrimitiveEventContainment, WorthUiPrimitiveEventDispatchCounters,
    WorthUiPrimitiveEventDispatchOutcome, WorthUiPrimitiveEventDispatchReceipt,
    WorthUiPrimitiveResolvedCursorPosture,
};

fn main() {
    let _forged = WorthUiPrimitiveEventDispatchReceipt {
        primary_surface_id: Some("worth.surface.preview.primitive.inner".to_owned()),
        emitted_surface_ids: vec!["worth.surface.preview.primitive.inner".to_owned()],
        cursor: WorthUiPrimitiveResolvedCursorPosture::Pointer,
        containment: Some(WorthUiPrimitiveEventContainment::Contain),
        outcome: WorthUiPrimitiveEventDispatchOutcome::Emitted,
        candidates: Vec::new(),
        counters: counters(),
        dispatch_digest: 1,
    };
}

fn counters() -> WorthUiPrimitiveEventDispatchCounters {
    panic!("fixture only checks event dispatch field privacy")
}
