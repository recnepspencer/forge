use worth_ui::facade::inspection::{
    UiClientPhysicalRect, UiGeometryOnly, UiVisualRegionAdjudication, UiVisualRegionCompleteness,
    UiVisualSnapshotReceipt,
};

pub(super) fn assert_frontmost_inset_region_adjudication(
    receipt: &UiVisualSnapshotReceipt<UiGeometryOnly>,
) {
    receipt.with_coordinate_scope(|scope| {
        let full = UiClientPhysicalRect::new(0, 0, 200, 120).unwrap();
        let result = scope.adjudicate_region(scope.client_region(full));
        assert_eq!(result.completeness(), UiVisualRegionCompleteness::Complete);
        assert_eq!(result.intersections().len(), 5);
        assert_intersection(&result, 0, rect(20, 15, 180, 105), 2);
        assert_intersection(&result, 1, rect(0, 0, 200, 15), 0);
        assert_intersection(&result, 2, rect(0, 105, 200, 120), 0);
        assert_intersection(&result, 3, rect(0, 15, 20, 105), 0);
        assert_intersection(&result, 4, rect(180, 15, 200, 105), 0);
        assert_eq!(result.cost().candidates_considered(), 2);
        assert!(result.cost().spatial_index_probes() > 0);
        assert!(result.cost().trace_index_probes() > 0);

        let outside = UiClientPhysicalRect::new(201, 121, 202, 122).unwrap();
        let empty = scope.adjudicate_region(scope.client_region(outside));
        assert_eq!(
            empty.completeness(),
            UiVisualRegionCompleteness::EmptyAndComplete
        );
        assert!(empty.intersections().is_empty());
        assert_eq!(empty.cost().candidates_considered(), 0);
    });
}

pub(super) fn assert_region_candidate_truncation(
    receipt: &UiVisualSnapshotReceipt<UiGeometryOnly>,
) {
    receipt.with_coordinate_scope(|scope| {
        let result = full_region(&scope);
        assert_eq!(result.completeness(), UiVisualRegionCompleteness::Truncated);
        assert!(result.intersections().is_empty());
        assert_eq!(result.budget().maximum_candidates(), 1);
        assert_eq!(result.cost().candidates_considered(), 1);
    });
}

pub(super) fn assert_region_result_truncation(receipt: &UiVisualSnapshotReceipt<UiGeometryOnly>) {
    receipt.with_coordinate_scope(|scope| {
        let result = full_region(&scope);
        assert_eq!(result.completeness(), UiVisualRegionCompleteness::Truncated);
        assert_eq!(result.intersections().len(), 1);
        assert_intersection(&result, 0, rect(20, 15, 180, 105), 2);
        assert_eq!(result.budget().maximum_results(), 1);
        assert_eq!(result.cost().candidates_considered(), 2);
    });
}

fn full_region(
    scope: &worth_ui::facade::inspection::UiVisualCoordinateScope<'_>,
) -> UiVisualRegionAdjudication {
    let full = UiClientPhysicalRect::new(0, 0, 200, 120).unwrap();
    scope.adjudicate_region(scope.client_region(full))
}

fn assert_intersection(
    result: &UiVisualRegionAdjudication,
    index: usize,
    expected_region: UiClientPhysicalRect,
    declaration_index: usize,
) {
    let intersection = &result.intersections()[index];
    assert_eq!(intersection.region(), expected_region);
    assert_eq!(
        intersection
            .identity_trace()
            .authored_provenance()
            .declaration_index(),
        declaration_index
    );
}

fn rect(left: u32, top: u32, right: u32, bottom: u32) -> UiClientPhysicalRect {
    UiClientPhysicalRect::new(left, top, right, bottom).unwrap()
}
