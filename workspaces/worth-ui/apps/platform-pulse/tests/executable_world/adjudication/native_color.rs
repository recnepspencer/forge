use std::fmt;

use crate::external_observation::NativeClientPixelCapture;

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
    Ok(NativeColorVerdict {
        expected,
        matching_samples,
        sampled_pixels,
    })
}

fn samples(
    pixels: &NativeClientPixelCapture,
    expected: ExpectedNativeColor,
) -> Vec<NativePixelSampleObservation> {
    let width = pixels.width() as usize;
    let height = pixels.height() as usize;
    let xs = [width / 4, width / 2, width.saturating_mul(3) / 4];
    let ys = [height / 4, height / 2, height.saturating_mul(3) / 4];
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
        }
    }
}
