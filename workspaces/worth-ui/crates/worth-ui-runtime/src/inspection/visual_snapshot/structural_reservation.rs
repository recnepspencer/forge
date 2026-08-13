pub(crate) fn host_structural_reservation(
    scope: super::UiVisualGrantScope,
    snapshot_lease: &crate::mounting::UiMountedVisualSnapshotLease,
    regions: &crate::mounting::UiMountedVisualRegionBasis,
    trace: &crate::mounting::UiMountedIdentityTraceBasis,
) -> Result<u64, worth_ui_inspection::UiVisualSnapshotDenial> {
    enforce_region_capacities(scope, regions)?;
    let common = checked_add(
        Some(snapshot_lease.structural_bytes()),
        trace.retained_structural_bytes(),
    )?;
    let pending = checked_add(Some(common), regions.retained_structural_bytes())?;
    let completed = checked_add(Some(common), estimated_index_bytes(regions))?;
    let reservation = u64::try_from(pending.max(completed))
        .map_err(|_| worth_ui_inspection::UiVisualSnapshotDenial::CapacityExceeded)?;
    if reservation > scope.maximum_retained_structural_bytes_per_receipt() {
        return Err(
            worth_ui_inspection::UiVisualSnapshotDenial::RetainedStructurePerReceiptCapacityExceeded,
        );
    }
    Ok(reservation)
}

pub(crate) fn retained_snapshot_structure(
    snapshot_lease: &crate::mounting::UiMountedVisualSnapshotLease,
    trace: &crate::mounting::UiMountedIdentityTraceBasis,
    visible: &super::UiVisibleRegionIndex,
    hit_test: &super::UiHitTestRegionIndex,
) -> Result<u64, worth_ui_inspection::UiVisualSnapshotDenial> {
    let mounted_and_trace = checked_add(
        Some(snapshot_lease.structural_bytes()),
        trace.retained_structural_bytes(),
    )?;
    let with_visible = checked_add(Some(mounted_and_trace), visible.retained_structural_bytes())?;
    let total = checked_add(Some(with_visible), hit_test.retained_structural_bytes())?;
    u64::try_from(total).map_err(|_| worth_ui_inspection::UiVisualSnapshotDenial::CapacityExceeded)
}

fn enforce_region_capacities(
    scope: super::UiVisualGrantScope,
    regions: &crate::mounting::UiMountedVisualRegionBasis,
) -> Result<(), worth_ui_inspection::UiVisualSnapshotDenial> {
    let paint = regions.paint();
    let hit_test = regions.hit_test();
    if paint.len() > usize::try_from(scope.maximum_visible_region_records()).unwrap_or(usize::MAX) {
        return Err(worth_ui_inspection::UiVisualSnapshotDenial::VisibleRegionCapacityExceeded);
    }
    if hit_test.len()
        > usize::try_from(scope.maximum_hit_test_region_records()).unwrap_or(usize::MAX)
    {
        return Err(worth_ui_inspection::UiVisualSnapshotDenial::HitTestRegionCapacityExceeded);
    }
    Ok(())
}

fn estimated_index_bytes(regions: &crate::mounting::UiMountedVisualRegionBasis) -> Option<usize> {
    let paint = regions.paint();
    let hit_test = regions.hit_test();
    super::UiVisibleRegionIndex::estimated_retained_structural_bytes(paint.len())?.checked_add(
        super::UiHitTestRegionIndex::estimated_retained_structural_bytes(hit_test.len())?,
    )
}

fn checked_add(
    left: Option<usize>,
    right: Option<usize>,
) -> Result<usize, worth_ui_inspection::UiVisualSnapshotDenial> {
    left.and_then(|left| left.checked_add(right?))
        .ok_or(worth_ui_inspection::UiVisualSnapshotDenial::CapacityExceeded)
}
