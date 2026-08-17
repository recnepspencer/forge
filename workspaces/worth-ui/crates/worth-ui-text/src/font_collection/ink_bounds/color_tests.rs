use harfrust::FontRef;
use skrifa::color::{Brush, ColorPainter, CompositeMode};

use super::{color::InkPainter, color::Rect};
use crate::font_collection::{
    application_color_fixtures::{cpal_colors, with_tables},
    UiFontGlyphInkBounds,
};

pub(crate) fn transparent_and_porter_duff_layers_have_exact_nonzero_bounds() {
    let source = crate::font_collection::profile_inputs_from_repository()
        .into_vec()
        .into_iter()
        .next()
        .expect("qualified profile has a font");
    let bytes = with_tables(
        &source.bytes,
        &[(b"CPAL", cpal_colors(&[[255, 0, 0, 255]]))],
    );
    let font = FontRef::from_index(&bytes, 0).unwrap();
    let palette_index = 0;
    let rect = Rect::new(0.0, 0.0, 20.0, 20.0);
    let palette_color = Brush::Solid {
        palette_index,
        alpha: 1.0,
    };

    let mut transparent = InkPainter::new(&font, &[]);
    transparent.push_clip_rect(rect);
    transparent.fill(Brush::Solid {
        palette_index,
        alpha: 0.0,
    });
    assert!(transparent.finish().is_none());

    for mode in [
        CompositeMode::Clear,
        CompositeMode::Xor,
        CompositeMode::SrcOut,
    ] {
        let mut painter = InkPainter::new(&font, &[]);
        painter.push_clip_rect(rect);
        painter.fill(palette_color.clone());
        painter.push_layer(mode);
        painter.fill(palette_color.clone());
        painter.pop_layer();
        assert!(
            painter.finish().is_none(),
            "{mode:?} must remove coincident nonzero coverage"
        );
    }

    for mode in [CompositeMode::Xor, CompositeMode::SrcOut] {
        let mut painter = InkPainter::new(&font, &[]);
        painter.push_clip_rect(rect);
        painter.fill(Brush::Solid {
            palette_index,
            alpha: 0.5,
        });
        painter.push_layer(mode);
        painter.fill(Brush::Solid {
            palette_index,
            alpha: 0.5,
        });
        painter.pop_layer();
        assert_eq!(
            painter.finish(),
            Some(UiFontGlyphInkBounds {
                x_min: 0,
                y_min: 0,
                x_max: 20,
                y_max: 20,
            }),
            "{mode:?} retains nonzero overlap when both alphas are partial"
        );
    }

    let alternating = crate::font_collection::color_glyph::path::rectangles([
        Rect::new(0.0, 0.0, 8.0, 8.0),
        Rect::new(12.0, 12.0, 20.0, 20.0),
    ]);
    let opposite = crate::font_collection::color_glyph::path::rectangles([
        Rect::new(12.0, 0.0, 20.0, 8.0),
        Rect::new(0.0, 12.0, 8.0, 20.0),
    ]);
    let mut same_bounds_distinct_contours = InkPainter::new(&font, &[]);
    same_bounds_distinct_contours.push_clip_path(alternating.clone());
    same_bounds_distinct_contours.fill(palette_color.clone());
    same_bounds_distinct_contours.push_layer(CompositeMode::Xor);
    same_bounds_distinct_contours.push_clip_path(opposite.clone());
    same_bounds_distinct_contours.fill(palette_color.clone());
    same_bounds_distinct_contours.pop_layer();
    assert_eq!(
        same_bounds_distinct_contours.finish(),
        Some(UiFontGlyphInkBounds {
            x_min: 0,
            y_min: 0,
            x_max: 20,
            y_max: 20,
        }),
        "distinct contours sharing one AABB must not cancel as identical coverage"
    );

    for mode in [CompositeMode::SrcIn, CompositeMode::DestIn] {
        let mut painter = InkPainter::new(&font, &[]);
        painter.push_clip_path(alternating.clone());
        painter.fill(palette_color.clone());
        painter.push_layer(mode);
        painter.push_clip_path(opposite.clone());
        painter.fill(palette_color.clone());
        painter.pop_layer();
        assert!(
            painter.finish().is_none(),
            "{mode:?} must use contour coverage rather than overlapping AABBs"
        );
    }

    let mut source_replaces_destination = InkPainter::new(&font, &[]);
    source_replaces_destination.push_clip_rect(rect);
    source_replaces_destination.fill(palette_color.clone());
    source_replaces_destination.push_layer(CompositeMode::Src);
    source_replaces_destination.push_clip_rect(Rect::new(5.0, 6.0, 9.0, 12.0));
    source_replaces_destination.fill(palette_color);
    source_replaces_destination.pop_layer();
    assert_eq!(
        source_replaces_destination.finish(),
        Some(UiFontGlyphInkBounds {
            x_min: 5,
            y_min: 6,
            x_max: 9,
            y_max: 12,
        })
    );
}
