use worth_ui::facade::inspection::{
    UiClientPhysicalPixel, UiPixelsRequired, UiVisualCoordinateOrientation,
    UiVisualCoordinateRounding, UiVisualHitTestOutcome, UiVisualHitTestTarget,
    UiVisualPointAdjudication, UiVisualSnapshotReceipt, UiVisualVisibleOutcome,
};
use worth_ui_platform_pulse::visual_identity_pulse::{
    PLATFORM_PULSE_BACKGROUND_LOGICAL_POINT, PLATFORM_PULSE_IDENTITY_TARGET_AUTHORED_NAME,
    PLATFORM_PULSE_TARGET_LOGICAL_POINT,
};

use crate::visual_identity_execution::PlatformPulseVisualExecutionDenial;

pub(super) struct PlatformPulseAdjudicatedPoints {
    pub(super) target_point: UiClientPhysicalPixel,
    pub(super) target: UiVisualPointAdjudication,
    pub(super) background_point: UiClientPhysicalPixel,
    pub(super) background: UiVisualPointAdjudication,
    pub(super) selected_target: UiVisualHitTestTarget,
}

pub(super) fn adjudicate_points(
    receipt: &UiVisualSnapshotReceipt<UiPixelsRequired>,
) -> Result<PlatformPulseAdjudicatedPoints, PlatformPulseVisualExecutionDenial> {
    receipt.with_coordinate_scope(|scope| {
        let target_point = scenario_point(receipt, PLATFORM_PULSE_TARGET_LOGICAL_POINT)?;
        let target = scope
            .client_pixel(target_point)
            .map_err(|_| PlatformPulseVisualExecutionDenial::PointCoordinate)
            .and_then(|point| {
                scope
                    .adjudicate_point(point)
                    .map_err(|_| PlatformPulseVisualExecutionDenial::PointOmitted)
            })?;
        let background_point = scenario_point(receipt, PLATFORM_PULSE_BACKGROUND_LOGICAL_POINT)?;
        let background = scope
            .client_pixel(background_point)
            .map_err(|_| PlatformPulseVisualExecutionDenial::PointCoordinate)
            .and_then(|point| {
                scope
                    .adjudicate_point(point)
                    .map_err(|_| PlatformPulseVisualExecutionDenial::PointOmitted)
            })?;
        let selected_target = validate_point_identity(&target, &background)?;
        Ok(PlatformPulseAdjudicatedPoints {
            target_point,
            target,
            background_point,
            background,
            selected_target,
        })
    })
}

fn validate_point_identity(
    target: &UiVisualPointAdjudication,
    background: &UiVisualPointAdjudication,
) -> Result<UiVisualHitTestTarget, PlatformPulseVisualExecutionDenial> {
    let target_hit = validate_target_identity(target)?;
    let UiVisualVisibleOutcome::Contributors(background_visible) = background.visible() else {
        return Err(PlatformPulseVisualExecutionDenial::PointUnsupported);
    };
    let background_visible = background_visible
        .frontmost()
        .ok_or(PlatformPulseVisualExecutionDenial::PointUnsupported)?;
    let UiVisualHitTestOutcome::Target(background_hit) = background.hit_test() else {
        return Err(PlatformPulseVisualExecutionDenial::PointUnsupported);
    };
    let target_node = target_hit.identity_trace().mounted_node();
    if background_visible.identity_trace().mounted_node()
        != background_hit.identity_trace().mounted_node()
        || background_hit.identity_trace().mounted_node() == target_node
    {
        return Err(PlatformPulseVisualExecutionDenial::PointIdentityMismatch);
    }
    Ok(target_hit)
}

fn validate_target_identity(
    target: &UiVisualPointAdjudication,
) -> Result<UiVisualHitTestTarget, PlatformPulseVisualExecutionDenial> {
    let UiVisualVisibleOutcome::Contributors(target_visible) = target.visible() else {
        return Err(PlatformPulseVisualExecutionDenial::PointUnsupported);
    };
    let target_visible = target_visible
        .frontmost()
        .ok_or(PlatformPulseVisualExecutionDenial::PointUnsupported)?;
    let UiVisualHitTestOutcome::Target(target_hit) = target.hit_test() else {
        return Err(PlatformPulseVisualExecutionDenial::PointUnsupported);
    };
    if target_visible.identity_trace().mounted_node() != target_hit.identity_trace().mounted_node()
    {
        return Err(PlatformPulseVisualExecutionDenial::PointIdentityMismatch);
    }
    validate_target_authored_name(target_hit)?;
    Ok(target_hit.clone())
}

fn validate_target_authored_name(
    target: &UiVisualHitTestTarget,
) -> Result<(), PlatformPulseVisualExecutionDenial> {
    if target
        .identity_trace()
        .declaration()
        .authored_semantic_name()
        == PLATFORM_PULSE_IDENTITY_TARGET_AUTHORED_NAME
    {
        Ok(())
    } else {
        Err(PlatformPulseVisualExecutionDenial::AuthoredNameMismatch)
    }
}

fn scenario_point(
    receipt: &UiVisualSnapshotReceipt<UiPixelsRequired>,
    logical: [u32; 2],
) -> Result<UiClientPhysicalPixel, PlatformPulseVisualExecutionDenial> {
    let coordinates = receipt.coordinates();
    if coordinates.orientation() != UiVisualCoordinateOrientation::TopLeftOrigin
        || coordinates.rounding() != UiVisualCoordinateRounding::PixelCenterNearest
    {
        return Err(PlatformPulseVisualExecutionDenial::PointCoordinate);
    }
    let scale = coordinates.scale();
    let translation = coordinates.translation();
    let x = f64::from(logical[0]) * f64::from(scale[0]) + f64::from(translation[0]);
    let y = f64::from(logical[1]) * f64::from(scale[1]) + f64::from(translation[1]);
    if !x.is_finite() || !y.is_finite() {
        return Err(PlatformPulseVisualExecutionDenial::PointCoordinate);
    }
    UiClientPhysicalPixel::new(x.round() as i64, y.round() as i64)
        .map_err(|_| PlatformPulseVisualExecutionDenial::PointCoordinate)
}
