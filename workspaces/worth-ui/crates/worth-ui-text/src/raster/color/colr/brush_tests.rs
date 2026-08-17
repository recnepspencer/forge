use kurbo::Point;
use skrifa::{
    color::{Brush, Color, ColorStop, Extend},
    raw::types::Point as FontPoint,
};

use super::super::{compositing::UiLinearColorPixel, transform::ColorTransform};
use super::brush::sample;

const RED: Color = Color {
    blue: 0,
    green: 0,
    red: 255,
    alpha: 255,
};
const BLUE: Color = Color {
    blue: 255,
    green: 0,
    red: 0,
    alpha: 255,
};
const BLACK: Color = Color {
    blue: 0,
    green: 0,
    red: 0,
    alpha: 255,
};
const WHITE: Color = Color {
    blue: 255,
    green: 255,
    red: 255,
    alpha: 255,
};
const OPAQUE_STOPS: [ColorStop; 2] = [
    ColorStop {
        offset: 0.0,
        palette_index: 0,
        alpha: 1.0,
    },
    ColorStop {
        offset: 1.0,
        palette_index: 1,
        alpha: 1.0,
    },
];

#[test]
fn palette_reordering_changes_the_selected_intrinsic_color() {
    let brush = Brush::Solid {
        palette_index: 0,
        alpha: 1.0,
    };
    let original = sampled(&brush, Point::ZERO, ColorTransform::IDENTITY, &[RED, BLUE]);
    let reordered = sampled(&brush, Point::ZERO, ColorTransform::IDENTITY, &[BLUE, RED]);
    assert_pixel(original, [1.0, 0.0, 0.0, 1.0]);
    assert_pixel(reordered, [0.0, 0.0, 1.0, 1.0]);
}

#[test]
fn opaque_gradient_midpoint_interpolates_in_linear_light() {
    let brush = linear_brush(&OPAQUE_STOPS);
    let midpoint = sampled(
        &brush,
        Point::new(1.0, 0.0),
        ColorTransform::IDENTITY,
        &[BLACK, WHITE],
    );
    assert_pixel(midpoint, [0.5, 0.5, 0.5, 1.0]);
}

#[test]
fn unequal_alpha_gradient_interpolates_premultiplied_channels() {
    let stops = [
        ColorStop {
            alpha: 0.25,
            ..OPAQUE_STOPS[0]
        },
        ColorStop {
            alpha: 0.75,
            ..OPAQUE_STOPS[1]
        },
    ];
    let brush = linear_brush(&stops);
    let midpoint = sampled(
        &brush,
        Point::new(1.0, 0.0),
        ColorTransform::IDENTITY,
        &[RED, BLUE],
    );
    assert_pixel(midpoint, [0.125, 0.0, 0.375, 0.5]);
}

#[test]
fn sweep_uses_skrifa_clockwise_degrees_in_font_space() {
    let brush = Brush::SweepGradient {
        c0: FontPoint { x: 0.0, y: 0.0 },
        start_angle: 0.0,
        end_angle: 180.0,
        color_stops: &OPAQUE_STOPS,
        extend: Extend::Pad,
    };
    let palette = [RED, BLUE];
    assert_pixel(
        sampled(
            &brush,
            Point::new(1.0, 0.0),
            ColorTransform::IDENTITY,
            &palette,
        ),
        [1.0, 0.0, 0.0, 1.0],
    );
    assert_pixel(
        sampled(
            &brush,
            Point::new(0.0, -1.0),
            ColorTransform::IDENTITY,
            &palette,
        ),
        [0.5, 0.0, 0.5, 1.0],
    );
    assert_pixel(
        sampled(
            &brush,
            Point::new(-1.0, 0.0),
            ColorTransform::IDENTITY,
            &palette,
        ),
        [0.0, 0.0, 1.0, 1.0],
    );
}

#[test]
fn equal_angle_repeating_sweep_paints_nothing() {
    let brush = Brush::SweepGradient {
        c0: FontPoint { x: 0.0, y: 0.0 },
        start_angle: 45.0,
        end_angle: 45.0,
        color_stops: &OPAQUE_STOPS,
        extend: Extend::Repeat,
    };
    assert!(sample(
        &brush,
        Point::new(1.0, 0.0),
        ColorTransform::IDENTITY,
        &[RED, BLUE],
    )
    .unwrap()
    .is_none());
}

#[test]
fn radial_gradient_transform_preserves_gradient_radius_space() {
    let brush = radial_brush(
        FontPoint { x: 0.0, y: 0.0 },
        0.0,
        FontPoint { x: 0.0, y: 0.0 },
        10.0,
    );
    let scale = ColorTransform::from(skrifa::color::Transform {
        xx: 2.0,
        yx: 0.0,
        xy: 0.0,
        yy: 2.0,
        dx: 0.0,
        dy: 0.0,
    });
    assert_pixel(
        sampled(&brush, Point::new(10.0, 0.0), scale, &[RED, BLUE]),
        [0.5, 0.0, 0.5, 1.0],
    );
}

#[test]
fn identical_radial_circles_and_outside_cone_paint_nothing() {
    let identical = radial_brush(
        FontPoint { x: 0.0, y: 0.0 },
        2.0,
        FontPoint { x: 0.0, y: 0.0 },
        2.0,
    );
    assert!(sample(
        &identical,
        Point::new(2.0, 0.0),
        ColorTransform::IDENTITY,
        &[RED, BLUE]
    )
    .unwrap()
    .is_none());

    let cone = radial_brush(
        FontPoint { x: 0.0, y: 0.0 },
        1.0,
        FontPoint { x: 4.0, y: 0.0 },
        2.0,
    );
    assert!(sample(
        &cone,
        Point::new(0.0, 10.0),
        ColorTransform::IDENTITY,
        &[RED, BLUE]
    )
    .unwrap()
    .is_none());
}

#[test]
fn radial_overlap_uses_greatest_positive_radius_root() {
    let cylinder = radial_brush(
        FontPoint { x: 0.0, y: 0.0 },
        1.0,
        FontPoint { x: 4.0, y: 0.0 },
        1.0,
    );
    let color = sampled(
        &cylinder,
        Point::new(2.0, 0.0),
        ColorTransform::IDENTITY,
        &[RED, BLUE],
    );
    assert_pixel(color, [0.25, 0.0, 0.75, 1.0]);
}

#[test]
fn radial_negative_radii_keep_only_the_positive_radius_branch() {
    let brush = radial_brush(
        FontPoint { x: 0.0, y: 0.0 },
        -1.0,
        FontPoint { x: 0.0, y: 0.0 },
        -2.0,
    );
    assert_pixel(
        sampled(
            &brush,
            Point::new(1.0, 0.0),
            ColorTransform::IDENTITY,
            &[RED, BLUE],
        ),
        [1.0, 0.0, 0.0, 1.0],
    );
}

fn linear_brush(stops: &[ColorStop]) -> Brush<'_> {
    Brush::LinearGradient {
        p0: FontPoint { x: 0.0, y: 0.0 },
        p1: FontPoint { x: 2.0, y: 0.0 },
        color_stops: stops,
        extend: Extend::Pad,
    }
}

fn radial_brush(c0: FontPoint<f32>, r0: f32, c1: FontPoint<f32>, r1: f32) -> Brush<'static> {
    Brush::RadialGradient {
        c0,
        r0,
        c1,
        r1,
        color_stops: &OPAQUE_STOPS,
        extend: Extend::Pad,
    }
}

fn sampled(
    brush: &Brush<'_>,
    point: Point,
    transform: ColorTransform,
    palette: &[Color],
) -> UiLinearColorPixel {
    sample(brush, point, transform, palette)
        .unwrap()
        .expect("sample point is painted")
}

fn assert_pixel(actual: UiLinearColorPixel, expected: [f64; 4]) {
    for (actual, expected) in [actual.r, actual.g, actual.b, actual.a]
        .into_iter()
        .zip(expected)
    {
        assert!((actual - expected).abs() < 1e-9, "{actual} != {expected}");
    }
}
