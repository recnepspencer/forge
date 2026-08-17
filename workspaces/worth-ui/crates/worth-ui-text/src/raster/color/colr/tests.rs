use kurbo::{Point, Rect, Shape};
use skrifa::color::{Brush, ColorPainter, CompositeMode};

use super::super::transform::ColorTransform;
use super::{
    sample_clipped_brush, ClippedBrushSample, ColorPainterImpl, ColorPainterInput,
    UiColorRasterGeometry,
};

#[test]
fn transformed_clip_uses_the_untransformed_device_sample_frame() {
    let transform = ColorTransform::from(skrifa::color::Transform {
        xx: 1.0,
        yx: 0.0,
        xy: 0.0,
        yy: 1.0,
        dx: 10.0,
        dy: 0.0,
    });
    let clips = [Rect::new(10.0, 0.0, 20.0, 10.0).to_path(0.1)];
    let palette = [skrifa::color::Color {
        blue: 0,
        green: 0,
        red: 255,
        alpha: 255,
    }];
    let brush = Brush::Solid {
        palette_index: 0,
        alpha: 1.0,
    };

    assert!(sample_clipped_brush(ClippedBrushSample {
        clips: &clips,
        brush: &brush,
        point: Point::new(15.0, 5.0),
        transform,
        palette: &palette,
    })
    .unwrap()
    .is_some());
    assert!(sample_clipped_brush(ClippedBrushSample {
        clips: &clips,
        brush: &brush,
        point: Point::new(5.0, 5.0),
        transform,
        palette: &palette,
    })
    .unwrap()
    .is_none());
}

#[test]
fn bounded_composite_can_consume_an_unbounded_source_intermediate() {
    let inputs = crate::font_collection::profile_inputs_from_repository();
    let font = harfrust::FontRef::from_index(&inputs[0].bytes, 0).unwrap();
    let palette = vec![
        skrifa::color::Color {
            blue: 0,
            green: 0,
            red: 255,
            alpha: 255,
        },
        skrifa::color::Color {
            blue: 255,
            green: 0,
            red: 0,
            alpha: 255,
        },
    ];
    let mut painter = ColorPainterImpl::new(ColorPainterInput {
        font: &font,
        coords: &[],
        palette,
        geometry: UiColorRasterGeometry {
            width: 1,
            height: 1,
        },
        scale: 1.0,
        pixels_per_em: 16.0,
        base_x: 0,
        top: 1,
    })
    .unwrap();
    painter.push_clip_box(skrifa::raw::types::BoundingBox {
        x_min: 0.0,
        y_min: 0.0,
        x_max: 1.0,
        y_max: 1.0,
    });
    painter.fill(Brush::Solid {
        palette_index: 1,
        alpha: 1.0,
    });
    painter.pop_clip();
    painter.push_layer(CompositeMode::SrcIn);
    painter.fill(Brush::Solid {
        palette_index: 0,
        alpha: 1.0,
    });
    painter.pop_layer();

    let pixels = painter.finish().unwrap();
    assert_eq!(pixels, [255, 0, 0, 255]);
}
