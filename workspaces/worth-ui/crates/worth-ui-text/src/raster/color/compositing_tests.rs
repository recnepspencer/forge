use skrifa::color::CompositeMode;

use super::compositing::{compose, source_over_bytes, UiLinearColorPixel};

#[test]
fn source_over_compositing_preserves_premultiplied_alpha() {
    let mut destination = [0, 0, 128, 128];
    source_over_bytes(&mut destination, &[128, 0, 0, 128]);
    assert_eq!(destination, [128, 0, 64, 192]);
}

#[test]
fn layer_reordering_changes_the_composited_intrinsic_color() {
    let red = [128, 0, 0, 128];
    let blue = [0, 0, 128, 128];
    let mut red_over_blue = blue;
    source_over_bytes(&mut red_over_blue, &red);
    let mut blue_over_red = red;
    source_over_bytes(&mut blue_over_red, &blue);
    assert_ne!(red_over_blue, blue_over_red);
}

#[test]
fn transparent_source_preserves_destination_alpha_and_color() {
    let mut destination = [32, 64, 96, 192];
    source_over_bytes(&mut destination, &[255, 0, 0, 0]);
    assert_eq!(destination, [32, 64, 96, 192]);
}

#[test]
fn atop_modes_preserve_the_declared_operand_alpha() {
    let source = UiLinearColorPixel {
        r: 0.3,
        g: 0.0,
        b: 0.0,
        a: 0.5,
    };
    let destination = UiLinearColorPixel {
        r: 0.0,
        g: 0.3,
        b: 0.0,
        a: 0.75,
    };

    assert_pixel(
        compose(source, destination, CompositeMode::SrcAtop).unwrap(),
        [0.225, 0.15, 0.0, 0.75],
    );
    assert_pixel(
        compose(source, destination, CompositeMode::DestAtop).unwrap(),
        [0.075, 0.15, 0.0, 0.5],
    );
}

#[test]
fn w3c_nonseparable_modes_use_weighted_luminosity_and_channel_saturation() {
    let source = UiLinearColorPixel {
        r: 0.4,
        g: 0.1,
        b: 0.2,
        a: 0.5,
    };
    let destination = UiLinearColorPixel {
        r: 0.075,
        g: 0.525,
        b: 0.225,
        a: 0.75,
    };
    let vectors = [
        (CompositeMode::HslHue, [0.46525, 0.39025, 0.34025]),
        (CompositeMode::HslSaturation, [0.175, 0.55, 0.275]),
        (CompositeMode::HslColor, [0.46525, 0.39025, 0.34025]),
        (CompositeMode::HslLuminosity, [0.14725, 0.52225, 0.24725]),
    ];
    for (mode, expected) in vectors {
        let actual = compose(source, destination, mode).unwrap();
        assert_pixel(actual, [expected[0], expected[1], expected[2], 0.875]);
    }
}

fn assert_pixel(actual: UiLinearColorPixel, expected: [f64; 4]) {
    for (actual, expected) in [actual.r, actual.g, actual.b, actual.a]
        .into_iter()
        .zip(expected)
    {
        assert!((actual - expected).abs() < 1e-9, "{actual} != {expected}");
    }
}
