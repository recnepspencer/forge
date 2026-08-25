use std::fmt;

use super::platform_pulse_control_points::{checked_in, PlatformPulseControlPointManifestFailure};
use crate::external_observation::{NativeClientPixelCapture, NativeClientPixelPoint};

const MINIMUM_INTERIOR_PIXELS: usize = 9;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PlatformPulseActionControlPoint {
    point: NativeClientPixelPoint,
    region: NativeControlPixelRegion,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PlatformPulseConfirmationControlPoint {
    point: NativeClientPixelPoint,
    region: NativeControlPixelRegion,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct NativeControlPixelRegion {
    minimum: [u32; 2],
    maximum: [u32; 2],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct VisibleControlPixelChange {
    differing_pixels: usize,
}

#[derive(Debug)]
pub(crate) enum IntentControlPointFailure {
    ControlPointManifest(PlatformPulseControlPointManifestFailure),
    InsufficientInteriorPixels {
        control: &'static str,
        matching_pixels: usize,
        interior_pixels: usize,
    },
    InvalidSelectedPixel,
    IndistinguishableControls,
    CaptureMismatch,
    VisibleChangeMissing {
        differing_pixels: usize,
        compared_pixels: usize,
    },
}

pub(crate) fn adjudicate_action_control_point(
    capture: &NativeClientPixelCapture,
) -> Result<PlatformPulseActionControlPoint, IntentControlPointFailure> {
    let manifest = checked_in().map_err(IntentControlPointFailure::ControlPointManifest)?;
    let (point, region) = select_interior_pixel(
        capture,
        "action",
        manifest.target_rgba()[..3].try_into().expect("RGB prefix"),
        manifest.action_control_logical_point(),
        manifest.logical_client_extent(),
        manifest.channel_tolerance(),
    )?;
    Ok(PlatformPulseActionControlPoint { point, region })
}

pub(crate) fn adjudicate_confirmation_control_point(
    capture: &NativeClientPixelCapture,
) -> Result<PlatformPulseConfirmationControlPoint, IntentControlPointFailure> {
    let manifest = checked_in().map_err(IntentControlPointFailure::ControlPointManifest)?;
    let (point, region) = select_interior_pixel(
        capture,
        "confirmation",
        manifest.confirmation_rgba()[..3]
            .try_into()
            .expect("RGB prefix"),
        manifest.confirmation_control_logical_point(),
        manifest.logical_client_extent(),
        manifest.channel_tolerance(),
    )?;
    Ok(PlatformPulseConfirmationControlPoint { point, region })
}

pub(crate) fn require_distinct_control_points(
    action: PlatformPulseActionControlPoint,
    confirmation: PlatformPulseConfirmationControlPoint,
) -> Result<(), IntentControlPointFailure> {
    if action.point == confirmation.point {
        return Err(IntentControlPointFailure::IndistinguishableControls);
    }
    Ok(())
}

pub(crate) fn adjudicate_visible_control_change(
    baseline: &NativeClientPixelCapture,
    current: &NativeClientPixelCapture,
    region: NativeControlPixelRegion,
) -> Result<VisibleControlPixelChange, IntentControlPointFailure> {
    if baseline.process_id() != current.process_id()
        || baseline.width() != current.width()
        || baseline.height() != current.height()
    {
        return Err(IntentControlPointFailure::CaptureMismatch);
    }
    let mut differing_pixels = 0;
    let mut compared_pixels = 0;
    for y in region.minimum[1]..=region.maximum[1] {
        for x in region.minimum[0]..=region.maximum[0] {
            let before = rgba_at(baseline, x, y);
            let after = rgba_at(current, x, y);
            if before.is_some() && after.is_some() {
                compared_pixels += 1;
                differing_pixels += usize::from(before != after);
            }
        }
    }
    if differing_pixels < MINIMUM_INTERIOR_PIXELS {
        return Err(IntentControlPointFailure::VisibleChangeMissing {
            differing_pixels,
            compared_pixels,
        });
    }
    Ok(VisibleControlPixelChange { differing_pixels })
}

fn select_interior_pixel(
    capture: &NativeClientPixelCapture,
    control: &'static str,
    expected: [u8; 3],
    preferred_logical_point: [u32; 2],
    logical_extent: [u32; 2],
    channel_tolerance: u8,
) -> Result<(NativeClientPixelPoint, NativeControlPixelRegion), IntentControlPointFailure> {
    let mut matching_pixels = 0;
    let mut interior = Vec::new();
    let mut minimum = [u32::MAX, u32::MAX];
    let mut maximum = [0, 0];
    for y in 0..capture.height() {
        for x in 0..capture.width() {
            if !matches_at(capture, x, y, expected, channel_tolerance) {
                continue;
            }
            matching_pixels += 1;
            minimum = [minimum[0].min(x), minimum[1].min(y)];
            maximum = [maximum[0].max(x), maximum[1].max(y)];
            if x > 0
                && y > 0
                && x + 1 < capture.width()
                && y + 1 < capture.height()
                && neighborhood_matches(capture, x, y, expected, channel_tolerance)
            {
                interior.push((x, y));
            }
        }
    }
    if interior.len() < MINIMUM_INTERIOR_PIXELS {
        return Err(IntentControlPointFailure::InsufficientInteriorPixels {
            control,
            matching_pixels,
            interior_pixels: interior.len(),
        });
    }
    let preferred = [
        scale_axis(
            preferred_logical_point[0],
            capture.width(),
            logical_extent[0],
        ),
        scale_axis(
            preferred_logical_point[1],
            capture.height(),
            logical_extent[1],
        ),
    ];
    let (x, y) = interior
        .into_iter()
        .min_by_key(|(x, y)| x.abs_diff(preferred[0]) + y.abs_diff(preferred[1]))
        .ok_or(IntentControlPointFailure::InvalidSelectedPixel)?;
    let point = NativeClientPixelPoint::interior(capture, x, y, 1)
        .ok_or(IntentControlPointFailure::InvalidSelectedPixel)?;
    Ok((point, NativeControlPixelRegion { minimum, maximum }))
}

fn neighborhood_matches(
    capture: &NativeClientPixelCapture,
    x: u32,
    y: u32,
    expected: [u8; 3],
    channel_tolerance: u8,
) -> bool {
    ((y - 1)..=(y + 1)).all(|sample_y| {
        ((x - 1)..=(x + 1))
            .all(|sample_x| matches_at(capture, sample_x, sample_y, expected, channel_tolerance))
    })
}

fn matches_at(
    capture: &NativeClientPixelCapture,
    x: u32,
    y: u32,
    expected: [u8; 3],
    channel_tolerance: u8,
) -> bool {
    let Some(offset) = usize::try_from(y)
        .ok()
        .and_then(|y| y.checked_mul(capture.width() as usize))
        .and_then(|row| row.checked_add(x as usize))
        .and_then(|pixel| pixel.checked_mul(4))
    else {
        return false;
    };
    capture
        .rgba()
        .get(offset..offset + 3)
        .is_some_and(|observed| {
            observed
                .iter()
                .zip(expected)
                .all(|(&channel, expected)| channel.abs_diff(expected) <= channel_tolerance)
        })
}

fn scale_axis(logical: u32, physical: u32, logical_extent: u32) -> u32 {
    ((u64::from(logical) * u64::from(physical) + u64::from(logical_extent / 2))
        / u64::from(logical_extent)) as u32
}

fn rgba_at(capture: &NativeClientPixelCapture, x: u32, y: u32) -> Option<[u8; 4]> {
    let offset = usize::try_from(y)
        .ok()?
        .checked_mul(capture.width() as usize)?
        .checked_add(x as usize)?
        .checked_mul(4)?;
    let rgba = capture.rgba().get(offset..offset + 4)?;
    Some([rgba[0], rgba[1], rgba[2], rgba[3]])
}

impl PlatformPulseActionControlPoint {
    pub(crate) fn point(self) -> NativeClientPixelPoint {
        self.point
    }

    pub(crate) fn region(self) -> NativeControlPixelRegion {
        self.region
    }
}

impl PlatformPulseConfirmationControlPoint {
    pub(crate) fn point(self) -> NativeClientPixelPoint {
        self.point
    }

    pub(crate) fn region(self) -> NativeControlPixelRegion {
        self.region
    }
}

impl VisibleControlPixelChange {
    pub(crate) fn differing_pixels(self) -> usize {
        self.differing_pixels
    }
}

impl fmt::Display for IntentControlPointFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ControlPointManifest(failure) => {
                write!(formatter, "test-owned control-point manifest: {failure:?}")
            }
            Self::InsufficientInteriorPixels {
                control,
                matching_pixels,
                interior_pixels,
            } => write!(
                formatter,
                "{control} control had {matching_pixels} matching pixels but only \
                 {interior_pixels} independently clickable interior pixels"
            ),
            Self::InvalidSelectedPixel => {
                formatter.write_str("selected control pixel was outside its source capture")
            }
            Self::IndistinguishableControls => {
                formatter.write_str("action and confirmation resolved to the same native pixel")
            }
            Self::CaptureMismatch => {
                formatter.write_str("visible posture captures came from different client images")
            }
            Self::VisibleChangeMissing {
                differing_pixels,
                compared_pixels,
            } => write!(
                formatter,
                "visible posture changed only {differing_pixels}/{compared_pixels} control pixels"
            ),
        }
    }
}
