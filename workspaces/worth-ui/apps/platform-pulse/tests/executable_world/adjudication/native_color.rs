use std::fmt;

use crate::external_observation::NativeClientPixelCapture;
use worth_ui_platform_pulse::visual_identity_pulse::{
    PLATFORM_PULSE_BACKGROUND_LOGICAL_POINT, PLATFORM_PULSE_CANONICAL_LOGICAL_EXTENT,
    PLATFORM_PULSE_TARGET_LOGICAL_POINT, PLATFORM_PULSE_TARGET_RGB,
};

const EXPECTED_BLUE: [u8; 3] = [0x2f, 0x81, 0xf7];
const EXPECTED_GREEN: [u8; 3] = [0x3f, 0xb9, 0x50];
const CHANNEL_TOLERANCE: u8 = 12;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ExpectedNativeColor {
    Blue,
    Green,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct NativeColorVerdict {
    expected: ExpectedNativeColor,
    matching_samples: usize,
    sampled_pixels: usize,
}

#[derive(Debug)]
pub(crate) enum NativeColorFailure {
    InsufficientPixelSamples,
    ExpectedColorNotVisible {
        expected: ExpectedNativeColor,
        matching: usize,
        samples: Box<[NativePixelSampleObservation]>,
    },
    ExpectedTargetNotVisible {
        point: [u32; 2],
        rgba: Option<[u8; 4]>,
        capture_extent: [u32; 2],
        matching_target_pixels: usize,
        target_pixel_bounds: Option<([u32; 2], [u32; 2])>,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct NativePixelSampleObservation {
    x: usize,
    y: usize,
    rgba: [u8; 4],
    matches_expected_color: bool,
}

pub(crate) fn adjudicate_native_color(
    pixels: &NativeClientPixelCapture,
    expected: ExpectedNativeColor,
) -> Result<NativeColorVerdict, NativeColorFailure> {
    let samples = samples(pixels, expected);
    let sampled_pixels = samples.len();
    let matching_samples = samples
        .iter()
        .filter(|sample| sample.matches_expected_color)
        .count();
    if sampled_pixels < 9 {
        return Err(NativeColorFailure::InsufficientPixelSamples);
    }
    if matching_samples * 4 < sampled_pixels * 3 {
        return Err(NativeColorFailure::ExpectedColorNotVisible {
            expected,
            matching: matching_samples,
            samples: samples.into_boxed_slice(),
        });
    }
    let target_point = scaled_point(pixels, PLATFORM_PULSE_TARGET_LOGICAL_POINT);
    let target_rgba = pixel_at(pixels, target_point);
    let target_point_visible = target_rgba.is_some_and(|rgba| {
        rgba[..3]
            .iter()
            .zip(PLATFORM_PULSE_TARGET_RGB)
            .all(|(&observed, expected)| observed.abs_diff(expected) <= CHANNEL_TOLERANCE)
    });
    if !target_point_visible {
        let target_summary = target_pixel_summary(pixels);
        return Err(NativeColorFailure::ExpectedTargetNotVisible {
            point: target_point,
            rgba: target_rgba,
            capture_extent: [pixels.width(), pixels.height()],
            matching_target_pixels: target_summary.matching_pixels,
            target_pixel_bounds: target_summary.bounds,
        });
    }
    Ok(NativeColorVerdict {
        expected,
        matching_samples,
        sampled_pixels,
    })
}

struct TargetPixelSummary {
    matching_pixels: usize,
    bounds: Option<([u32; 2], [u32; 2])>,
}

fn target_pixel_summary(pixels: &NativeClientPixelCapture) -> TargetPixelSummary {
    let mut matching_pixels = 0;
    let mut minimum = [u32::MAX, u32::MAX];
    let mut maximum = [0, 0];
    for (index, rgba) in pixels.rgba().chunks_exact(4).enumerate() {
        let matches = rgba[..3]
            .iter()
            .zip(PLATFORM_PULSE_TARGET_RGB)
            .all(|(&observed, expected)| observed.abs_diff(expected) <= CHANNEL_TOLERANCE);
        if !matches {
            continue;
        }
        let index = u32::try_from(index).expect("capture index fits its u32 extent");
        let point = [index % pixels.width(), index / pixels.width()];
        matching_pixels += 1;
        minimum = [minimum[0].min(point[0]), minimum[1].min(point[1])];
        maximum = [maximum[0].max(point[0]), maximum[1].max(point[1])];
    }
    TargetPixelSummary {
        matching_pixels,
        bounds: (matching_pixels != 0).then_some((minimum, maximum)),
    }
}

fn samples(
    pixels: &NativeClientPixelCapture,
    expected: ExpectedNativeColor,
) -> Vec<NativePixelSampleObservation> {
    let width = pixels.width() as usize;
    let height = pixels.height() as usize;
    let xs = [
        scaled_point(pixels, PLATFORM_PULSE_BACKGROUND_LOGICAL_POINT)[0] as usize,
        width / 5,
        width.saturating_mul(9) / 10,
    ];
    let ys = [
        scaled_point(pixels, PLATFORM_PULSE_BACKGROUND_LOGICAL_POINT)[1] as usize,
        height.saturating_mul(5) / 6,
        height.saturating_mul(11) / 12,
    ];
    let expected_rgb = match expected {
        ExpectedNativeColor::Blue => EXPECTED_BLUE,
        ExpectedNativeColor::Green => EXPECTED_GREEN,
    };
    let mut samples = Vec::with_capacity(9);
    for y in ys {
        for x in xs {
            let offset = (y * width + x) * 4;
            if let Some(pixel) = pixels.rgba().get(offset..offset + 4) {
                let rgba = [pixel[0], pixel[1], pixel[2], pixel[3]];
                let matches_expected_color = rgba[..3]
                    .iter()
                    .zip(expected_rgb)
                    .all(|(&observed, expected)| observed.abs_diff(expected) <= CHANNEL_TOLERANCE);
                samples.push(NativePixelSampleObservation {
                    x,
                    y,
                    rgba,
                    matches_expected_color,
                });
            }
        }
    }
    samples
}

fn scaled_point(pixels: &NativeClientPixelCapture, logical: [u32; 2]) -> [u32; 2] {
    [
        ((u64::from(logical[0]) * u64::from(pixels.width())
            + u64::from(PLATFORM_PULSE_CANONICAL_LOGICAL_EXTENT[0] / 2))
            / u64::from(PLATFORM_PULSE_CANONICAL_LOGICAL_EXTENT[0])) as u32,
        ((u64::from(logical[1]) * u64::from(pixels.height())
            + u64::from(PLATFORM_PULSE_CANONICAL_LOGICAL_EXTENT[1] / 2))
            / u64::from(PLATFORM_PULSE_CANONICAL_LOGICAL_EXTENT[1])) as u32,
    ]
}

fn pixel_at(pixels: &NativeClientPixelCapture, point: [u32; 2]) -> Option<[u8; 4]> {
    let width = usize::try_from(pixels.width()).ok()?;
    let x = usize::try_from(point[0]).ok()?;
    let y = usize::try_from(point[1]).ok()?;
    let offset = y.checked_mul(width)?.checked_add(x)?.checked_mul(4)?;
    let pixel = pixels.rgba().get(offset..offset + 4)?;
    Some([pixel[0], pixel[1], pixel[2], pixel[3]])
}

impl NativeColorVerdict {
    pub(crate) fn expected(self) -> ExpectedNativeColor {
        self.expected
    }

    pub(crate) fn matching_samples(self) -> usize {
        self.matching_samples
    }

    pub(crate) fn sampled_pixels(self) -> usize {
        self.sampled_pixels
    }
}

impl fmt::Display for NativeColorFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InsufficientPixelSamples => {
                formatter.write_str("native client capture yielded fewer than nine samples")
            }
            Self::ExpectedColorNotVisible {
                expected,
                matching,
                samples,
            } => {
                write!(
                    formatter,
                    "expected {expected:?} was visible in only {matching}/{} samples",
                    samples.len()
                )?;
                for sample in samples {
                    write!(
                        formatter,
                        "; ({}, {}) rgba={:?} match={}",
                        sample.x, sample.y, sample.rgba, sample.matches_expected_color
                    )?;
                }
                Ok(())
            }
            Self::ExpectedTargetNotVisible {
                point,
                rgba,
                capture_extent,
                matching_target_pixels,
                target_pixel_bounds,
            } => {
                write!(
                    formatter,
                    "canonical target point ({}, {}) did not show target color; rgba={rgba:?}; \
                     capture={}x{}; target-colored pixels={matching_target_pixels}; \
                     target bounds={target_pixel_bounds:?}",
                    point[0], point[1], capture_extent[0], capture_extent[1]
                )
            }
        }
    }
}
