//! Deterministic linear-premultiplied compositing for COLRv1 layers.

use skrifa::color::CompositeMode;

#[derive(Clone, Copy, Debug, Default)]
pub(super) struct UiLinearColorPixel {
    pub(super) r: f64,
    pub(super) g: f64,
    pub(super) b: f64,
    pub(super) a: f64,
}

impl UiLinearColorPixel {
    pub(super) const TRANSPARENT: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };

    pub(super) fn clamp(self) -> Self {
        let alpha = self.a.clamp(0.0, 1.0);
        Self {
            r: self.r.clamp(0.0, alpha),
            g: self.g.clamp(0.0, alpha),
            b: self.b.clamp(0.0, alpha),
            a: alpha,
        }
    }
}

pub(super) fn compose(
    source: UiLinearColorPixel,
    destination: UiLinearColorPixel,
    mode: CompositeMode,
) -> Option<UiLinearColorPixel> {
    use CompositeMode::*;
    let (source_factor, destination_factor) = match mode {
        Clear => (0.0, 0.0),
        Src => (1.0, 0.0),
        Dest => (0.0, 1.0),
        SrcOver => (1.0, 1.0 - source.a),
        DestOver => (1.0 - destination.a, 1.0),
        SrcIn => (destination.a, 0.0),
        DestIn => (0.0, source.a),
        SrcOut => (1.0 - destination.a, 0.0),
        DestOut => (0.0, 1.0 - source.a),
        SrcAtop => (destination.a, 1.0 - source.a),
        DestAtop => (1.0 - destination.a, source.a),
        Xor => (1.0 - destination.a, 1.0 - source.a),
        Plus => {
            return Some(
                UiLinearColorPixel {
                    r: source.r + destination.r,
                    g: source.g + destination.g,
                    b: source.b + destination.b,
                    a: source.a + destination.a,
                }
                .clamp(),
            );
        }
        Screen | Overlay | Darken | Lighten | ColorDodge | ColorBurn | HardLight | SoftLight
        | Difference | Exclusion | Multiply | HslHue | HslSaturation | HslColor | HslLuminosity => {
            return Some(advanced(source, destination, mode).clamp());
        }
        Unknown => return None,
    };
    Some(
        UiLinearColorPixel {
            r: source.r * source_factor + destination.r * destination_factor,
            g: source.g * source_factor + destination.g * destination_factor,
            b: source.b * source_factor + destination.b * destination_factor,
            a: source.a * source_factor + destination.a * destination_factor,
        }
        .clamp(),
    )
}

pub(super) fn source_over_bytes(destination: &mut [u8], source: &[u8]) {
    let destination_pixel = UiLinearColorPixel::from_bytes(destination);
    let source_pixel = UiLinearColorPixel::from_bytes(source);
    let composed = compose(source_pixel, destination_pixel, CompositeMode::SrcOver)
        .unwrap_or(UiLinearColorPixel::TRANSPARENT);
    composed.write_bytes(destination);
}

impl UiLinearColorPixel {
    fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            r: f64::from(bytes[0]) / 255.0,
            g: f64::from(bytes[1]) / 255.0,
            b: f64::from(bytes[2]) / 255.0,
            a: f64::from(bytes[3]) / 255.0,
        }
        .clamp()
    }

    fn write_bytes(self, bytes: &mut [u8]) {
        let pixel = self.clamp();
        bytes[0] = (pixel.r * 255.0).round() as u8;
        bytes[1] = (pixel.g * 255.0).round() as u8;
        bytes[2] = (pixel.b * 255.0).round() as u8;
        bytes[3] = (pixel.a * 255.0).round() as u8;
    }
}

fn advanced(
    source: UiLinearColorPixel,
    destination: UiLinearColorPixel,
    mode: CompositeMode,
) -> UiLinearColorPixel {
    let source_rgb = straight(source);
    let destination_rgb = straight(destination);
    let blended = blend_rgb(source_rgb, destination_rgb, mode);
    let alpha = source.a + destination.a - source.a * destination.a;
    UiLinearColorPixel {
        r: (1.0 - source.a) * destination.r
            + (1.0 - destination.a) * source.r
            + source.a * destination.a * blended[0],
        g: (1.0 - source.a) * destination.g
            + (1.0 - destination.a) * source.g
            + source.a * destination.a * blended[1],
        b: (1.0 - source.a) * destination.b
            + (1.0 - destination.a) * source.b
            + source.a * destination.a * blended[2],
        a: alpha,
    }
}

fn straight(pixel: UiLinearColorPixel) -> [f64; 3] {
    if pixel.a <= f64::EPSILON {
        [0.0; 3]
    } else {
        [pixel.r / pixel.a, pixel.g / pixel.a, pixel.b / pixel.a]
    }
}

fn blend_rgb(source: [f64; 3], destination: [f64; 3], mode: CompositeMode) -> [f64; 3] {
    use CompositeMode::*;
    if matches!(mode, HslHue | HslSaturation | HslColor | HslLuminosity) {
        return hsl_blend(source, destination, mode);
    }
    core::array::from_fn(|index| {
        let s = source[index];
        let d = destination[index];
        match mode {
            Screen => s + d - s * d,
            Overlay => overlay(s, d),
            Darken => s.min(d),
            Lighten => s.max(d),
            ColorDodge => {
                if s >= 1.0 {
                    1.0
                } else {
                    (d / (1.0 - s)).min(1.0)
                }
            }
            ColorBurn => {
                if s <= 0.0 {
                    0.0
                } else {
                    1.0 - ((1.0 - d) / s).min(1.0)
                }
            }
            HardLight => overlay(d, s),
            SoftLight => soft_light(s, d),
            Difference => (d - s).abs(),
            Exclusion => s + d - 2.0 * s * d,
            Multiply => s * d,
            _ => s,
        }
    })
}

fn overlay(source: f64, destination: f64) -> f64 {
    if destination <= 0.5 {
        2.0 * source * destination
    } else {
        1.0 - 2.0 * (1.0 - source) * (1.0 - destination)
    }
}

fn soft_light(source: f64, destination: f64) -> f64 {
    if source <= 0.5 {
        destination - (1.0 - 2.0 * source) * destination * (1.0 - destination)
    } else {
        let d = if destination <= 0.25 {
            ((16.0 * destination - 12.0) * destination + 4.0) * destination
        } else {
            destination.sqrt()
        };
        destination + (2.0 * source - 1.0) * (d - destination)
    }
}

fn hsl_blend(source: [f64; 3], destination: [f64; 3], mode: CompositeMode) -> [f64; 3] {
    use CompositeMode::*;
    match mode {
        HslHue => set_lum(
            set_saturation(source, saturation(destination)),
            luminosity(destination),
        ),
        HslSaturation => set_lum(
            set_saturation(destination, saturation(source)),
            luminosity(destination),
        ),
        HslColor => set_lum(source, luminosity(destination)),
        HslLuminosity => set_lum(destination, luminosity(source)),
        _ => destination,
    }
}

fn luminosity(color: [f64; 3]) -> f64 {
    0.3 * color[0] + 0.59 * color[1] + 0.11 * color[2]
}

fn saturation(color: [f64; 3]) -> f64 {
    color[0].max(color[1]).max(color[2]) - color[0].min(color[1]).min(color[2])
}

fn set_lum(color: [f64; 3], target: f64) -> [f64; 3] {
    let delta = target - luminosity(color);
    clip_color(color.map(|channel| channel + delta))
}

fn clip_color(mut color: [f64; 3]) -> [f64; 3] {
    let lum = luminosity(color);
    let minimum = color[0].min(color[1]).min(color[2]);
    let maximum = color[0].max(color[1]).max(color[2]);
    if minimum < 0.0 {
        let denominator = lum - minimum;
        if denominator > f64::EPSILON {
            color = color.map(|channel| lum + (channel - lum) * lum / denominator);
        }
    }
    if maximum > 1.0 {
        let denominator = maximum - lum;
        if denominator > f64::EPSILON {
            color = color.map(|channel| lum + (channel - lum) * (1.0 - lum) / denominator);
        }
    }
    color
}

fn set_saturation(color: [f64; 3], target: f64) -> [f64; 3] {
    let mut order = [0, 1, 2];
    order.sort_by(|left, right| color[*left].total_cmp(&color[*right]));
    let [minimum, middle, maximum] = order;
    let mut result = color;
    if color[maximum] > color[minimum] {
        result[middle] =
            (color[middle] - color[minimum]) * target / (color[maximum] - color[minimum]);
        result[maximum] = target;
    } else {
        result[middle] = 0.0;
        result[maximum] = 0.0;
    }
    result[minimum] = 0.0;
    result
}
