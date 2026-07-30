use worth_ui::facade::inspection::{
    UiClientPhysicalPixel, UiGeometryOnly, UiVisualCapturePoll, UiVisualHitTestOutcome,
    UiVisualSnapshotOutcome, UiVisualSnapshotRequest,
};
use worth_ui_runtime::facade::mounted::{UiMountedInspectionReceipt, UiMountedInspectionRequest};

pub(super) fn assert_published_shapes(
    output: &egui::FullOutput,
    target: worth_ui::facade::inspection::UiClientPhysicalRect,
    pixels_per_point: f32,
) {
    let rects = magenta_rects(output);
    assert_eq!(rects.len(), 4, "one mechanic emits exactly four strips");
    let border = 2.0 / pixels_per_point;
    let target_width = (target.right() - target.left()) as f32 / pixels_per_point;
    let target_height = (target.bottom() - target.top()) as f32 / pixels_per_point;
    let horizontal = rects
        .iter()
        .filter(|rect| near(rect.width(), target_width) && near(rect.height(), border))
        .count();
    let vertical = rects
        .iter()
        .filter(|rect| near(rect.width(), border) && near(rect.height(), target_height))
        .count();
    assert_eq!((horizontal, vertical), (2, 2));
    let bounds = union(&rects);
    assert!(near(bounds.left(), target.left() as f32 / pixels_per_point));
    assert!(near(bounds.top(), target.top() as f32 / pixels_per_point));
    assert!(near(
        bounds.right(),
        target.right() as f32 / pixels_per_point
    ));
    assert!(near(
        bounds.bottom(),
        target.bottom() as f32 / pixels_per_point
    ));
    assert!(!rects.iter().any(|rect| rect.contains(bounds.center())));
}

pub(super) fn assert_no_overlay_shapes(output: &egui::FullOutput) {
    assert!(
        magenta_rects(output).is_empty(),
        "cleared presentation must emit no identity-overlay pixels"
    );
}

pub(super) fn assert_retained_overlay_repaint(
    context: &egui::Context,
    host: &worth_ui_host_egui::WorthUiHostEgui,
    target: worth_ui::facade::inspection::UiClientPhysicalRect,
) {
    let output = context.run_ui(super::super::raw_input(), |_| {
        host.repaint_retained_surfaces();
    });
    assert_published_shapes(&output, target, context.pixels_per_point());
}

pub(super) fn assert_retained_clear_repaint(
    context: &egui::Context,
    host: &worth_ui_host_egui::WorthUiHostEgui,
) {
    let output = context.run_ui(super::super::raw_input(), |_| {
        host.repaint_retained_surfaces();
    });
    assert_no_overlay_shapes(&output);
}

pub(super) fn assert_overlay_is_not_indexed(
    context: &egui::Context,
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
    selected_instance: u64,
) {
    let receipt = capture_current_geometry(context, session);
    assert_eq!(receipt.visible_region_count(), 2);
    assert_eq!(receipt.hit_test_region_count(), 2);
    receipt.with_coordinate_scope(|scope| {
        let point = scope
            .client_pixel(UiClientPhysicalPixel::new(100, 50).unwrap())
            .unwrap();
        let adjudication = scope.adjudicate_point(point).unwrap();
        let UiVisualHitTestOutcome::Target(target) = adjudication.hit_test() else {
            panic!("the overlay cannot replace the selected canonical hit target");
        };
        assert_eq!(
            target.identity_trace().mounted_node().mounted_instance(),
            selected_instance
        );
    });
    let disposed = session.dispose_visual_snapshot(receipt);
    assert!(disposed.released_registered_resource());
}

fn capture_current_geometry(
    context: &egui::Context,
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
) -> worth_ui::facade::inspection::UiVisualSnapshotReceipt<UiGeometryOnly> {
    let target = match session.inspect_mounted_frame(UiMountedInspectionRequest::current()) {
        UiMountedInspectionReceipt::Available(frame) => frame
            .current_visual_target()
            .expect("the overlay successor has one presented surface"),
        other => panic!("the overlay successor remains inspectable, got {other:?}"),
    };
    let grant = session.visual_inspection_authority().issue_geometry_grant();
    let pending = session
        .begin_visual_geometry_snapshot(
            &grant,
            UiVisualSnapshotRequest::for_local_development_unredacted_frame(target)
                .artifacts(UiGeometryOnly::policy()),
        )
        .expect("the overlay successor admits a geometry capture");
    let mut pending = Some(pending);
    let mut poll = None;
    let _ = context.run_ui(super::super::raw_input(), |_| {
        poll =
            Some(session.poll_visual_snapshot(pending.take().expect("one capture is consumed"), 3));
    });
    match poll.expect("egui settles the overlay geometry capture") {
        UiVisualCapturePoll::Completed(UiVisualSnapshotOutcome::Captured(receipt)) => receipt,
        _ => panic!("overlay geometry capture must complete with a captured receipt"),
    }
}

fn magenta_rects(output: &egui::FullOutput) -> Vec<egui::Rect> {
    output
        .shapes
        .iter()
        .filter_map(|clipped| {
            let egui::epaint::Shape::Rect(rect) = &clipped.shape else {
                return None;
            };
            (rect.fill == egui::Color32::from_rgba_unmultiplied(255, 0, 255, 255))
                .then_some(rect.rect)
        })
        .collect()
}

fn union(rects: &[egui::Rect]) -> egui::Rect {
    rects
        .iter()
        .copied()
        .reduce(egui::Rect::union)
        .expect("four overlay strips have a union")
}

fn near(left: f32, right: f32) -> bool {
    (left - right).abs() < 0.001
}
