//! COLRv1 brush sampling in the selected CPAL palette.

use kurbo::Point;
use skrifa::color::{Brush, ColorStop, Extend};

use super::super::{
    compositing::UiLinearColorPixel, pixels::srgb_channel_to_linear, transform::ColorTransform,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ColorBrushSampleDenial {
    EmptyColorLine,
    InvalidPalette,
    NonInvertibleTransform,
    UnknownExtend,
}

#[derive(Clone, Copy)]
struct BrushSample<'a> {
    point: Point,
    transform: ColorTransform,
    palette: &'a [skrifa::color::Color],
}

struct LinearGradientSample<'a> {
    p0: Point,
    p1: Point,
    color_stops: &'a [ColorStop],
    extend: Extend,
}

struct RadialGradientSample<'a> {
    c0: Point,
    r0: f32,
    c1: Point,
    r1: f32,
    color_stops: &'a [ColorStop],
    extend: Extend,
}

struct SweepGradientSample<'a> {
    center: Point,
    start_angle: f32,
    end_angle: f32,
    color_stops: &'a [ColorStop],
    extend: Extend,
}

pub(super) fn sample(
    brush: &Brush<'_>,
    point: Point,
    transform: ColorTransform,
    palette: &[skrifa::color::Color],
) -> Result<Option<UiLinearColorPixel>, ColorBrushSampleDenial> {
    let sample = BrushSample {
        point,
        transform,
        palette,
    };
    match brush {
        Brush::Solid {
            palette_index,
            alpha,
        } => solid_color(*palette_index, *alpha, sample.palette).map(Some),
        Brush::LinearGradient { .. } => linear_brush(brush, sample),
        Brush::RadialGradient { .. } => radial_brush(brush, sample),
        Brush::SweepGradient { .. } => sweep_brush(brush, sample),
    }
}

fn linear_brush(
    brush: &Brush<'_>,
    sample: BrushSample<'_>,
) -> Result<Option<UiLinearColorPixel>, ColorBrushSampleDenial> {
    let Brush::LinearGradient {
        p0,
        p1,
        color_stops,
        extend,
    } = brush
    else {
        unreachable!("linear brush dispatch preserves its variant");
    };
    linear_color(
        LinearGradientSample {
            p0: Point::new(f64::from(p0.x), f64::from(p0.y)),
            p1: Point::new(f64::from(p1.x), f64::from(p1.y)),
            color_stops,
            extend: *extend,
        },
        sample,
    )
}

fn radial_brush(
    brush: &Brush<'_>,
    sample: BrushSample<'_>,
) -> Result<Option<UiLinearColorPixel>, ColorBrushSampleDenial> {
    let Brush::RadialGradient {
        c0,
        r0,
        c1,
        r1,
        color_stops,
        extend,
    } = brush
    else {
        unreachable!("radial brush dispatch preserves its variant");
    };
    radial_color(
        RadialGradientSample {
            c0: Point::new(f64::from(c0.x), f64::from(c0.y)),
            r0: *r0,
            c1: Point::new(f64::from(c1.x), f64::from(c1.y)),
            r1: *r1,
            color_stops,
            extend: *extend,
        },
        sample,
    )
}

fn sweep_brush(
    brush: &Brush<'_>,
    sample: BrushSample<'_>,
) -> Result<Option<UiLinearColorPixel>, ColorBrushSampleDenial> {
    let Brush::SweepGradient {
        c0,
        start_angle,
        end_angle,
        color_stops,
        extend,
    } = brush
    else {
        unreachable!("sweep brush dispatch preserves its variant");
    };
    sweep_color(
        SweepGradientSample {
            center: Point::new(f64::from(c0.x), f64::from(c0.y)),
            start_angle: *start_angle,
            end_angle: *end_angle,
            color_stops,
            extend: *extend,
        },
        sample,
    )
}

fn solid_color(
    palette_index: u16,
    alpha: f32,
    palette: &[skrifa::color::Color],
) -> Result<UiLinearColorPixel, ColorBrushSampleDenial> {
    palette_color(palette, palette_index, f64::from(alpha))
}

fn linear_color(
    gradient: LinearGradientSample<'_>,
    sample: BrushSample<'_>,
) -> Result<Option<UiLinearColorPixel>, ColorBrushSampleDenial> {
    let point = sample
        .transform
        .inverse_apply(sample.point)
        .ok_or(ColorBrushSampleDenial::NonInvertibleTransform)?;
    let p0 = gradient.p0;
    let p1 = gradient.p1;
    let delta = p1 - p0;
    let denominator = delta.x * delta.x + delta.y * delta.y;
    let position = if denominator <= f64::EPSILON {
        0.0
    } else {
        (point - p0).dot(delta) / denominator
    };
    gradient_color(
        gradient.color_stops,
        position,
        gradient.extend,
        sample.palette,
    )
    .map(Some)
}

fn radial_color(
    gradient: RadialGradientSample<'_>,
    sample: BrushSample<'_>,
) -> Result<Option<UiLinearColorPixel>, ColorBrushSampleDenial> {
    let point = sample
        .transform
        .inverse_apply(sample.point)
        .ok_or(ColorBrushSampleDenial::NonInvertibleTransform)?;
    let c0 = gradient.c0;
    let c1 = gradient.c1;
    let Some(position) = radial_position(RadialPosition {
        point,
        c0,
        r0: f64::from(gradient.r0),
        c1,
        r1: f64::from(gradient.r1),
    }) else {
        return Ok(None);
    };
    gradient_color(
        gradient.color_stops,
        position,
        gradient.extend,
        sample.palette,
    )
    .map(Some)
}

fn sweep_color(
    gradient: SweepGradientSample<'_>,
    sample: BrushSample<'_>,
) -> Result<Option<UiLinearColorPixel>, ColorBrushSampleDenial> {
    let point = sample
        .transform
        .inverse_apply(sample.point)
        .ok_or(ColorBrushSampleDenial::NonInvertibleTransform)?;
    let center = gradient.center;
    let angle = (center.y - point.y)
        .atan2(point.x - center.x)
        .to_degrees()
        .rem_euclid(360.0);
    let start_angle = f64::from(gradient.start_angle);
    let span = f64::from(gradient.end_angle) - start_angle;
    let position = if span.abs() <= f64::EPSILON {
        if gradient.extend != Extend::Pad {
            return Ok(None);
        }
        if angle < start_angle {
            f64::NEG_INFINITY
        } else {
            f64::INFINITY
        }
    } else {
        (angle - start_angle) / span
    };
    gradient_color(
        gradient.color_stops,
        position,
        gradient.extend,
        sample.palette,
    )
    .map(Some)
}

struct RadialPosition {
    point: Point,
    c0: Point,
    r0: f64,
    c1: Point,
    r1: f64,
}

fn radial_position(position: RadialPosition) -> Option<f64> {
    let centers = position.c1 - position.c0;
    let relative = position.point - position.c0;
    let radius_delta = position.r1 - position.r0;
    if centers.x == 0.0 && centers.y == 0.0 && radius_delta == 0.0 {
        return None;
    }
    let center_square = centers.dot(centers);
    let radius_square = radius_delta * radius_delta;
    let a = center_square - radius_square;
    let relative_projection = relative.dot(centers);
    let radius_projection = position.r0 * radius_delta;
    let b = -2.0 * (relative_projection + radius_projection);
    let c = relative.dot(relative) - position.r0 * position.r0;
    let a_scale = center_square.abs().max(radius_square.abs()).max(1.0);
    if a.abs() <= f64::EPSILON * a_scale * 16.0 {
        let b_scale = relative_projection
            .abs()
            .max(radius_projection.abs())
            .max(1.0);
        if b.abs() <= f64::EPSILON * b_scale * 32.0 {
            return None;
        }
        return valid_radial_root(-c / b, position.r0, radius_delta);
    }
    let discriminant = b * b - 4.0 * a * c;
    let discriminant_scale = (b * b).abs().max((4.0 * a * c).abs()).max(1.0);
    if discriminant < -f64::EPSILON * discriminant_scale * 32.0 {
        return None;
    }
    let root = discriminant.max(0.0).sqrt();
    let signed_root = if b >= 0.0 { root } else { -root };
    let stable_numerator = -0.5 * (b + signed_root);
    let roots = if stable_numerator.abs() <= f64::MIN_POSITIVE {
        [-b / (2.0 * a); 2]
    } else {
        [stable_numerator / a, c / stable_numerator]
    };
    roots
        .into_iter()
        .filter_map(|candidate| valid_radial_root(candidate, position.r0, radius_delta))
        .max_by(f64::total_cmp)
}

fn valid_radial_root(candidate: f64, r0: f64, radius_delta: f64) -> Option<f64> {
    (candidate.is_finite() && r0 + radius_delta * candidate > 0.0).then_some(candidate)
}

fn palette_color(
    palette: &[skrifa::color::Color],
    index: u16,
    paint_alpha: f64,
) -> Result<UiLinearColorPixel, ColorBrushSampleDenial> {
    let color = *palette
        .get(usize::from(index))
        .ok_or(ColorBrushSampleDenial::InvalidPalette)?;
    let alpha = f64::from(color.alpha()) / 255.0 * paint_alpha.clamp(0.0, 1.0);
    Ok(UiLinearColorPixel {
        r: srgb_channel_to_linear(color.red()) * alpha,
        g: srgb_channel_to_linear(color.green()) * alpha,
        b: srgb_channel_to_linear(color.blue()) * alpha,
        a: alpha,
    })
}

fn gradient_color(
    stops: &[ColorStop],
    position: f64,
    extend: Extend,
    palette: &[skrifa::color::Color],
) -> Result<UiLinearColorPixel, ColorBrushSampleDenial> {
    let position = extend_position(position, extend)?;
    let first = stops
        .first()
        .ok_or(ColorBrushSampleDenial::EmptyColorLine)?;
    let last = stops.last().ok_or(ColorBrushSampleDenial::EmptyColorLine)?;
    if position < f64::from(first.offset) {
        return stop_color(*first, palette);
    }
    if position >= f64::from(last.offset) {
        return stop_color(*last, palette);
    }
    let right_index = stops.partition_point(|stop| f64::from(stop.offset) <= position);
    let left = stops[right_index - 1];
    let right = stops[right_index];
    let factor = (position - f64::from(left.offset)) / f64::from(right.offset - left.offset);
    Ok(interpolate_color(
        stop_color(left, palette)?,
        stop_color(right, palette)?,
        factor,
    ))
}

fn stop_color(
    stop: ColorStop,
    palette: &[skrifa::color::Color],
) -> Result<UiLinearColorPixel, ColorBrushSampleDenial> {
    palette_color(palette, stop.palette_index, f64::from(stop.alpha))
}

fn interpolate_color(
    left: UiLinearColorPixel,
    right: UiLinearColorPixel,
    factor: f64,
) -> UiLinearColorPixel {
    UiLinearColorPixel {
        r: left.r + (right.r - left.r) * factor,
        g: left.g + (right.g - left.g) * factor,
        b: left.b + (right.b - left.b) * factor,
        a: left.a + (right.a - left.a) * factor,
    }
}

fn extend_position(position: f64, extend: Extend) -> Result<f64, ColorBrushSampleDenial> {
    match extend {
        Extend::Pad => Ok(position.clamp(0.0, 1.0)),
        Extend::Repeat => Ok(position.rem_euclid(1.0)),
        Extend::Reflect => {
            let position = position.rem_euclid(2.0);
            Ok(if position <= 1.0 {
                position
            } else {
                2.0 - position
            })
        }
        Extend::Unknown => Err(ColorBrushSampleDenial::UnknownExtend),
    }
}
