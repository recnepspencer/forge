use std::io::Cursor;

use read_fonts::TableProvider;
use skrifa::{
    bitmap::{BitmapData, BitmapStrikes, Origin},
    instance::Size,
    GlyphId,
};

use crate::font_collection::UiFontGlyphInkBounds;

pub(super) fn bounds(
    font: &harfrust::FontRef<'_>,
    glyph_id: GlyphId,
) -> Option<Option<UiFontGlyphInkBounds>> {
    let glyph = BitmapStrikes::new(font).glyph_for_size(Size::unscaled(), glyph_id)?;
    let pixels = alpha_bounds(&glyph.data, glyph.width, glyph.height)?;
    let Some(pixels) = pixels else {
        return Some(None);
    };
    let units_per_em = f32::from(font.head().ok()?.units_per_em());
    let scale_x = units_per_em / glyph.ppem_x;
    let scale_y = units_per_em / glyph.ppem_y;
    if !scale_x.is_finite() || !scale_y.is_finite() {
        return Some(None);
    }
    let image_left = glyph.bearing_x + glyph.inner_bearing_x * scale_x;
    let image_top = match glyph.placement_origin {
        Origin::TopLeft => glyph.bearing_y + glyph.inner_bearing_y * scale_y,
        Origin::BottomLeft => {
            glyph.bearing_y + glyph.inner_bearing_y * scale_y + glyph.height as f32 * scale_y
        }
    };
    Some(Some(UiFontGlyphInkBounds {
        x_min: (image_left + pixels.left as f32 * scale_x).floor() as i32,
        y_min: (image_top - pixels.bottom as f32 * scale_y).floor() as i32,
        x_max: (image_left + pixels.right as f32 * scale_x).ceil() as i32,
        y_max: (image_top - pixels.top as f32 * scale_y).ceil() as i32,
    }))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct PixelBounds {
    left: u32,
    top: u32,
    right: u32,
    bottom: u32,
}

fn alpha_bounds(data: &BitmapData<'_>, width: u32, height: u32) -> Option<Option<PixelBounds>> {
    let alpha = match data {
        BitmapData::Bgra(bytes) => bgra_alpha(bytes, width, height)?,
        BitmapData::Mask(mask) => mask.decode(width, height).ok()?,
        BitmapData::Png(bytes) => png_alpha(bytes, width, height)?,
    };
    Some(nonzero_bounds(&alpha, width, height))
}

fn bgra_alpha(bytes: &[u8], width: u32, height: u32) -> Option<Vec<u8>> {
    let pixels = usize::try_from(width)
        .ok()?
        .checked_mul(usize::try_from(height).ok()?)?;
    let expected = pixels.checked_mul(4)?;
    (bytes.len() >= expected).then(|| {
        bytes[..expected]
            .chunks_exact(4)
            .map(|pixel| pixel[3])
            .collect()
    })
}

fn png_alpha(bytes: &[u8], width: u32, height: u32) -> Option<Vec<u8>> {
    let mut decoder = png::Decoder::new_with_limits(
        Cursor::new(bytes),
        png::Limits {
            bytes: 2 * 1024 * 1024,
        },
    );
    decoder.set_transformations(png::Transformations::EXPAND | png::Transformations::STRIP_16);
    let mut reader = decoder.read_info().ok()?;
    let output_size = reader.output_buffer_size()?;
    let mut output = vec![0; output_size];
    let info = reader.next_frame(&mut output).ok()?;
    if info.width != width || info.height != height {
        return None;
    }
    let samples = info.color_type.samples();
    let alpha_index = match info.color_type {
        png::ColorType::GrayscaleAlpha => Some(1),
        png::ColorType::Rgba => Some(3),
        png::ColorType::Grayscale | png::ColorType::Rgb => None,
        png::ColorType::Indexed => return None,
    };
    let pixels = usize::try_from(width)
        .ok()?
        .checked_mul(usize::try_from(height).ok()?)?;
    match alpha_index {
        Some(alpha_index) => Some(
            output[..info.buffer_size()]
                .chunks_exact(samples)
                .take(pixels)
                .map(|pixel| pixel[alpha_index])
                .collect(),
        ),
        None => Some(vec![255; pixels]),
    }
}

fn nonzero_bounds(alpha: &[u8], width: u32, height: u32) -> Option<PixelBounds> {
    let mut bounds: Option<PixelBounds> = None;
    for (index, value) in alpha.iter().copied().enumerate() {
        if value == 0 {
            continue;
        }
        let index = u32::try_from(index).ok()?;
        let x = index % width;
        let y = index / width;
        if y >= height {
            break;
        }
        bounds = Some(match bounds {
            Some(bounds) => PixelBounds {
                left: bounds.left.min(x),
                top: bounds.top.min(y),
                right: bounds.right.max(x + 1),
                bottom: bounds.bottom.max(y + 1),
            },
            None => PixelBounds {
                left: x,
                top: y,
                right: x + 1,
                bottom: y + 1,
            },
        });
    }
    bounds
}

#[cfg(test)]
pub(crate) fn transparent_and_bordered_bitmap_alpha_has_exact_support() {
    let mut alpha = [0; 16];
    alpha[5] = 1;
    alpha[6] = 255;
    alpha[9] = 127;
    alpha[10] = 1;
    let expected = Some(PixelBounds {
        left: 1,
        top: 1,
        right: 3,
        bottom: 3,
    });
    let transparent = rgba_png(&[0; 16]);
    assert_eq!(
        alpha_bounds(&BitmapData::Png(&transparent), 4, 4),
        Some(None)
    );
    let bordered = rgba_png(&alpha);
    assert_eq!(
        alpha_bounds(&BitmapData::Png(&bordered), 4, 4),
        Some(expected)
    );
    let bgra = alpha
        .iter()
        .flat_map(|alpha| [0, 0, 0, *alpha])
        .collect::<Vec<_>>();
    assert_eq!(alpha_bounds(&BitmapData::Bgra(&bgra), 4, 4), Some(expected));
}

#[cfg(test)]
fn rgba_png(alpha: &[u8; 16]) -> Vec<u8> {
    let mut output = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut output, 4, 4);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        let pixels = alpha
            .iter()
            .flat_map(|alpha| [255, 255, 255, *alpha])
            .collect::<Vec<_>>();
        writer.write_image_data(&pixels).unwrap();
    }
    output
}
