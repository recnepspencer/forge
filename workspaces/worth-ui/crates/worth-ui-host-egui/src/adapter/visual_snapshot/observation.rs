use std::sync::Arc;

use super::state::{UiEguiPresentedSurface, UiEguiScreenshotCorrelation};

pub(super) struct UiEguiScreenshotEvent {
    viewport: egui::ViewportId,
    image: Arc<egui::ColorImage>,
}

pub(super) fn matching_screenshot(
    context: &egui::Context,
    request: worth_ui_host_contract::UiHostCaptureRequestIdentity,
) -> Option<UiEguiScreenshotEvent> {
    context.input(|input| {
        input.events.iter().find_map(|event| {
            let egui::Event::Screenshot {
                viewport_id,
                user_data,
                image,
            } = event
            else {
                return None;
            };
            let correlation = user_data
                .data
                .as_ref()
                .and_then(|data| data.downcast_ref::<UiEguiScreenshotCorrelation>())?;
            (correlation.request() == request).then(|| UiEguiScreenshotEvent {
                viewport: *viewport_id,
                image: Arc::clone(image),
            })
        })
    })
}

pub(super) fn geometry_observation(
    context: &egui::Context,
    request: worth_ui_host_contract::UiHostVisualCaptureRequest,
    presented: UiEguiPresentedSurface,
) -> worth_ui_host_contract::UiHostCaptureObservationOutcome {
    let Some(transform) = coordinate_transform(context, None) else {
        return worth_ui_host_contract::UiHostCaptureObservationOutcome::CaptureAffinityIndeterminate;
    };
    captured(request, presented, transform, None)
}

pub(super) fn pixel_observation(
    context: &egui::Context,
    request: worth_ui_host_contract::UiHostVisualCaptureRequest,
    presented: UiEguiPresentedSurface,
    event: UiEguiScreenshotEvent,
) -> worth_ui_host_contract::UiHostCaptureObservationOutcome {
    if event.viewport != context.viewport_id() {
        return worth_ui_host_contract::UiHostCaptureObservationOutcome::CaptureAffinityIndeterminate;
    }
    let Some(dimensions) = image_dimensions(&event.image) else {
        return worth_ui_host_contract::UiHostCaptureObservationOutcome::CaptureAffinityIndeterminate;
    };
    let Some(transform) = coordinate_transform(context, Some(dimensions)) else {
        return worth_ui_host_contract::UiHostCaptureObservationOutcome::CaptureAffinityIndeterminate;
    };
    let Some(byte_count) = u64::from(dimensions[0])
        .checked_mul(u64::from(dimensions[1]))
        .and_then(|pixels| pixels.checked_mul(4))
    else {
        return worth_ui_host_contract::UiHostCaptureObservationOutcome::CapacityExceeded;
    };
    if byte_count > request.maximum_pixel_bytes() {
        return worth_ui_host_contract::UiHostCaptureObservationOutcome::CapacityExceeded;
    }
    let bytes = copy_rgba(&event.image);
    let stride = match dimensions[0].checked_mul(4) {
        Some(stride) => stride,
        None => return worth_ui_host_contract::UiHostCaptureObservationOutcome::CapacityExceeded,
    };
    let pixels = worth_ui_host_contract::UiHostPixelArtifact::copied_by_host(
        dimensions,
        stride,
        bytes.into_boxed_slice(),
        worth_ui_host_contract::UiHostPixelColorSpace::Srgb,
    );
    captured(request, presented, transform, Some(pixels))
}

fn captured(
    request: worth_ui_host_contract::UiHostVisualCaptureRequest,
    presented: UiEguiPresentedSurface,
    transform: worth_ui_host_contract::UiHostCoordinateTransform,
    pixels: Option<worth_ui_host_contract::UiHostPixelArtifact>,
) -> worth_ui_host_contract::UiHostCaptureObservationOutcome {
    worth_ui_host_contract::UiHostCaptureObservationOutcome::Captured(
        worth_ui_host_contract::UiHostCaptureObservation::observed_by_host(
            worth_ui_host_contract::UiHostCaptureAffinity::observed_by_host(
                request.identity(),
                request.expected_epoch(),
            ),
            transform,
            presented.regions(),
            pixels,
        ),
    )
}

fn coordinate_transform(
    context: &egui::Context,
    physical_dimensions: Option<[u32; 2]>,
) -> Option<worth_ui_host_contract::UiHostCoordinateTransform> {
    context.input(|input| {
        let screen = input.screen_rect();
        let viewport = input.viewport();
        let native_scale = viewport.native_pixels_per_point?;
        let logical = [screen.width(), screen.height()];
        if !native_scale.is_finite()
            || native_scale <= 0.0
            || logical
                .iter()
                .any(|value| !value.is_finite() || *value <= 0.0)
        {
            return None;
        }
        let physical = physical_dimensions.unwrap_or([
            dimension(logical[0] * input.pixels_per_point())?,
            dimension(logical[1] * input.pixels_per_point())?,
        ]);
        let scale = [
            physical[0] as f32 / logical[0],
            physical[1] as f32 / logical[1],
        ];
        let inner = viewport.inner_rect?;
        let origin = [
            signed_coordinate(inner.min.x * native_scale)?,
            signed_coordinate(inner.min.y * native_scale)?,
        ];
        Some(
            worth_ui_host_contract::UiHostCoordinateTransform::observed_by_host(
                worth_ui_host_contract::UiHostClientAreaObservation::observed_by_host(
                    origin, physical,
                ),
                worth_ui_host_contract::UiHostViewportTransformObservation::observed_by_host(
                    logical,
                    scale,
                    [screen.min.x, screen.min.y],
                ),
                worth_ui_host_contract::UiHostCoordinatePosture::observed_by_host(
                    worth_ui_host_contract::UiHostCoordinateOrientation::TopLeftOrigin,
                    worth_ui_host_contract::UiHostCoordinateRounding::PixelCenterNearest,
                ),
            ),
        )
    })
}

fn image_dimensions(image: &egui::ColorImage) -> Option<[u32; 2]> {
    Some([
        u32::try_from(image.size[0]).ok()?,
        u32::try_from(image.size[1]).ok()?,
    ])
}

fn copy_rgba(image: &egui::ColorImage) -> Vec<u8> {
    image
        .pixels
        .iter()
        .flat_map(|pixel| pixel.to_srgba_unmultiplied())
        .collect()
}

fn dimension(value: f32) -> Option<u32> {
    (value.is_finite() && value > 0.0 && value <= u32::MAX as f32).then_some(value.round() as u32)
}

fn signed_coordinate(value: f32) -> Option<i32> {
    (value.is_finite() && value >= i32::MIN as f32 && value <= i32::MAX as f32)
        .then_some(value.round() as i32)
}
