use worth_ui::facade::app::WorthUiNativeApplicationShell;
use worth_ui::facade::app::{UiMountedInspectedFrame, UiMountedVisualTargetDenial};
use worth_ui::facade::declaration::{
    ComponentAllocationMeasurementContract, ComponentChildPolicy, ComponentDescriptor,
    ComponentHitTestContract, ComponentHitTestOrder, ComponentId, ComponentPropSchema,
    ComponentStateOwnership,
};
use worth_ui::facade::inspection::{
    UiClientPhysicalPixel, UiClientPhysicalRect, UiClientRegionVisualTarget,
    UiCurrentPresentedSurfaceTarget, UiGeometryOnly, UiHitTestRegionIndexIdentity,
    UiMountedNodeVisualTarget, UiPixelsRequired, UiVisibleRegionIndexIdentity,
    UiUnbudgetedVisualSnapshotComparisonRequest, UiVisualComparisonPixelPolicy,
    UiVisualOverlayDenial, UiVisualOverlayGrant, UiVisualOverlayTarget, UiVisualQueryBudget,
    UiVisualSnapshotComparisonBudget, UiVisualSnapshotComparisonOutcome, UiVisualSnapshotDenial,
    UiVisualSnapshotReceipt, UiVisualSnapshotRequest,
};
use worth_ui::facade::rebind::UiRebindReceipt;

struct GovernedRequestInputs {
    geometry_target: UiCurrentPresentedSurfaceTarget,
    pixel_target: UiCurrentPresentedSurfaceTarget,
}

fn governed_requests_typecheck(
    shell: &mut WorthUiNativeApplicationShell,
    inputs: GovernedRequestInputs,
) -> Result<(), UiVisualSnapshotDenial> {
    let authority = shell.visual_inspection_authority();
    let _ = authority.policy();
    let geometry_grant = authority.issue_geometry_grant();
    let pixel_grant = authority.issue_pixel_grant();
    let _overlay_grant = authority.issue_overlay_grant();

    let geometry = shell.begin_visual_geometry_snapshot(
        &geometry_grant,
        UiVisualSnapshotRequest::for_local_development_unredacted_frame(inputs.geometry_target),
    )?;
    let _ = shell.poll_visual_snapshot(geometry, 0);

    let pixels = shell.begin_visual_pixel_snapshot(
        &pixel_grant,
        UiVisualSnapshotRequest::for_local_development_unredacted_frame(inputs.pixel_target)
            .artifacts(UiPixelsRequired::policy()),
    )?;
    let _ = shell.cancel_visual_snapshot(pixels);
    Ok(())
}

fn required_pixels_are_total(receipt: &UiVisualSnapshotReceipt<UiPixelsRequired>) {
    let _ = receipt.pixel_artifact().bytes();
}

fn compare_rebound_snapshots(
    shell: &mut WorthUiNativeApplicationShell,
    predecessor: &UiVisualSnapshotReceipt<UiPixelsRequired>,
    successor: &UiVisualSnapshotReceipt<UiPixelsRequired>,
    rebind: &UiRebindReceipt,
) -> UiVisualSnapshotComparisonOutcome {
    let grant = shell
        .visual_inspection_authority()
        .issue_comparison_grant();
    let request =
        UiUnbudgetedVisualSnapshotComparisonRequest::between(predecessor, successor, rebind)
            .with_pixel_observation(UiVisualComparisonPixelPolicy::IfAlreadyRetained)
            .with_budget(
                UiVisualSnapshotComparisonBudget::bounded(128)
                    .expect("comparison budget is nonzero"),
            );
    shell.compare_visual_snapshots(&grant, request)
}

fn coordinate_brand_is_usable(receipt: &UiVisualSnapshotReceipt<UiGeometryOnly>) {
    receipt.with_coordinate_scope(|scope| {
        let point = scope.client_pixel(
            UiClientPhysicalPixel::new(80, 48).expect("canonical point is nonnegative"),
        ).expect("canonical point lies inside the captured extent");
        let _ = scope.adjudicate_point(point);
        let region = scope.client_region(
            UiClientPhysicalRect::new(0, 0, 160, 96).expect("canonical region is nonempty"),
        );
        let _ = scope.adjudicate_region(region);
    });
}

fn shell_disposes_owned_snapshot(
    shell: &mut WorthUiNativeApplicationShell,
    receipt: UiVisualSnapshotReceipt<UiGeometryOnly>,
) {
    let _ = shell.dispose_visual_snapshot(receipt);
}

fn live_selected_node_seals_target(
    frame: UiMountedInspectedFrame,
) -> Result<UiMountedNodeVisualTarget, UiMountedVisualTargetDenial> {
    frame.node_visual_target()
}

fn snapshot_scope_seals_region_target(
    receipt: UiVisualSnapshotReceipt<UiGeometryOnly>,
) -> Result<UiClientRegionVisualTarget, UiVisualSnapshotDenial> {
    receipt.into_client_region_target(|scope| {
        scope.client_region(
            UiClientPhysicalRect::new(0, 0, 8, 8).expect("compile-contract crop is nonempty"),
        )
    })
}

fn authored_hit_contract_carries_distinct_total_order() {
    let allocation = ComponentAllocationMeasurementContract::fill_viewport();
    let contract = ComponentHitTestContract::allocation_bounds(
        ComponentHitTestOrder::front_to_back(0),
        allocation,
    );
    let component = ComponentDescriptor::new(
        ComponentId::new("compile.visual.hit").unwrap(),
        ComponentPropSchema::named("compile.visual.hit.props"),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    )
    .with_hit_test(contract);
    let _ = component;
}

fn index_identity_types_remain_distinct(
    visible: UiVisibleRegionIndexIdentity,
    hit_test: UiHitTestRegionIndexIdentity,
) {
    fn accepts_visible(_: UiVisibleRegionIndexIdentity) {}
    fn accepts_hit_test(_: UiHitTestRegionIndexIdentity) {}
    accepts_visible(visible);
    accepts_hit_test(hit_test);
}

fn query_budget_receipt_is_interpretable(budget: UiVisualQueryBudget) {
    let _ = (budget.maximum_results(), budget.maximum_candidates());
}

fn governed_overlay_lifecycle_typechecks(
    shell: &mut WorthUiNativeApplicationShell,
    grant: &UiVisualOverlayGrant,
    target: UiVisualOverlayTarget,
) -> Result<(), UiVisualOverlayDenial> {
    let pending = shell.show_identity_overlay(grant, target)?;
    let published = shell
        .present_visual_overlay(pending, 2, 1)
        .map_err(|failure| failure.denial())?;
    let _cleared = shell
        .clear_visual_overlay(published, 3, 2)
        .map_err(|failure| failure.denial())?;
    Ok(())
}

fn main() {
    let _ = (
        governed_requests_typecheck,
        required_pixels_are_total,
        compare_rebound_snapshots,
        coordinate_brand_is_usable,
        shell_disposes_owned_snapshot,
        live_selected_node_seals_target,
        snapshot_scope_seals_region_target,
        authored_hit_contract_carries_distinct_total_order,
        index_identity_types_remain_distinct,
        query_budget_receipt_is_interpretable,
        governed_overlay_lifecycle_typechecks,
    );
}
