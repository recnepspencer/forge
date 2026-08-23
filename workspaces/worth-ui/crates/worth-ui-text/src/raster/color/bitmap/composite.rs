//! Qualified CBDT composite-image expansion through the shared color-glyph table owner.

use super::super::super::denial::UiGlyphRasterizationDenial;
use super::super::compositing::source_over_bytes;
use super::super::pixels::{canonicalize_pixels, UiColorPixelEncoding};
use super::decode::{decode_png, validate_bitmap_dimensions};
use crate::font_collection::color_glyph::bitmap_selection::UiCbdtCompositeSelection;
use read_fonts::tables::bitmap::{BitmapContent, BitmapData, BitmapDataFormat, BitmapMetrics};
use skrifa::GlyphId;

pub(super) struct UiCbdtCompositeImage {
    pub(super) pixels: Vec<u8>,
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) ppem_x: f32,
    pub(super) ppem_y: f32,
    pub(super) bearing_x: f32,
    pub(super) bearing_y: f32,
}

struct CbdtContext<'font, 'borrow> {
    cblc: &'borrow read_fonts::tables::cblc::Cblc<'font>,
    cbdt: &'borrow read_fonts::tables::cbdt::Cbdt<'font>,
    size: &'borrow read_fonts::tables::bitmap::BitmapSize,
}

#[derive(Clone, Copy)]
struct BitmapShape {
    width: u32,
    height: u32,
}

#[derive(Clone, Copy)]
struct BitmapPlacement {
    width: u32,
    height: u32,
    offset_x: i32,
    offset_y: i32,
}

pub(super) fn expand_cbdt_composite(
    selection: UiCbdtCompositeSelection<'_>,
) -> Result<Option<UiCbdtCompositeImage>, UiGlyphRasterizationDenial> {
    let context = CbdtContext {
        cblc: &selection.cblc,
        cbdt: &selection.cbdt,
        size: &selection.size,
    };
    flatten_composite(&context, selection.glyph, &selection.data, &mut Vec::new()).map(Some)
}

fn flatten_composite<'font, 'borrow>(
    context: &CbdtContext<'font, 'borrow>,
    glyph: GlyphId,
    data: &BitmapData<'font>,
    stack: &mut Vec<u32>,
) -> Result<UiCbdtCompositeImage, UiGlyphRasterizationDenial> {
    let (shape, bearing_x, bearing_y) = metrics(data)?;
    validate_bitmap_dimensions(shape.width, shape.height)?;
    let pixels = match &data.content {
        BitmapContent::Data(format, bytes) => {
            decode_data(*format, *bytes, context.size.bit_depth(), shape)?
        }
        BitmapContent::Composite(components) => {
            if components.is_empty() || stack.len() >= 64 || stack.contains(&glyph.to_u32()) {
                return Err(UiGlyphRasterizationDenial::InvalidColorPixels);
            }
            stack.push(glyph.to_u32());
            let mut pixels = transparent_pixels(shape)?;
            for component in *components {
                let child_id = GlyphId::from(component.glyph_id());
                let location = context
                    .size
                    .location(context.cblc.offset_data(), child_id)
                    .map_err(|_| UiGlyphRasterizationDenial::BitmapUnavailable)?;
                if location.is_empty() {
                    return Err(UiGlyphRasterizationDenial::BitmapUnavailable);
                }
                let child = context
                    .cbdt
                    .data(&location)
                    .map_err(|_| UiGlyphRasterizationDenial::BitmapUnavailable)?;
                let child_image = flatten_composite(context, child_id, &child, stack)?;
                place_child(
                    &mut pixels,
                    &child_image,
                    BitmapPlacement {
                        width: shape.width,
                        height: shape.height,
                        offset_x: i32::from(component.x_offset()),
                        offset_y: i32::from(component.y_offset()),
                    },
                )?;
            }
            stack.pop();
            pixels
        }
    };
    Ok(UiCbdtCompositeImage {
        pixels,
        width: shape.width,
        height: shape.height,
        ppem_x: f32::from(context.size.ppem_x()),
        ppem_y: f32::from(context.size.ppem_y()),
        bearing_x,
        bearing_y,
    })
}

fn decode_data(
    format: BitmapDataFormat,
    bytes: &[u8],
    bit_depth: u8,
    shape: BitmapShape,
) -> Result<Vec<u8>, UiGlyphRasterizationDenial> {
    match format {
        BitmapDataFormat::Png => canonicalize_pixels(
            &decode_png(bytes, shape.width, shape.height)?,
            UiColorPixelEncoding::StraightRgba,
        ),
        BitmapDataFormat::ByteAligned if bit_depth == 32 => {
            let expected = usize::try_from(shape.width)
                .ok()
                .and_then(|width| {
                    usize::try_from(shape.height)
                        .ok()
                        .and_then(|height| width.checked_mul(height))
                })
                .and_then(|pixels| pixels.checked_mul(4))
                .ok_or(UiGlyphRasterizationDenial::ExtentExceeded)?;
            if bytes.len() != expected {
                return Err(UiGlyphRasterizationDenial::InvalidColorPixels);
            }
            canonicalize_pixels(bytes, UiColorPixelEncoding::PremultipliedBgra)
        }
        _ => Err(UiGlyphRasterizationDenial::UnsupportedBitmapFormat),
    }
}

fn metrics(data: &BitmapData<'_>) -> Result<(BitmapShape, f32, f32), UiGlyphRasterizationDenial> {
    let (shape, bearing_x, bearing_y) = match &data.metrics {
        BitmapMetrics::Small(metrics) => (
            BitmapShape {
                width: u32::from(metrics.width()),
                height: u32::from(metrics.height()),
            },
            f32::from(metrics.bearing_x()),
            f32::from(metrics.bearing_y()),
        ),
        BitmapMetrics::Big(metrics) => (
            BitmapShape {
                width: u32::from(metrics.width()),
                height: u32::from(metrics.height()),
            },
            f32::from(metrics.hori_bearing_x()),
            f32::from(metrics.hori_bearing_y()),
        ),
    };
    if shape.width == 0 || shape.height == 0 {
        return Err(UiGlyphRasterizationDenial::EmptyRaster);
    }
    Ok((shape, bearing_x, bearing_y))
}

fn transparent_pixels(shape: BitmapShape) -> Result<Vec<u8>, UiGlyphRasterizationDenial> {
    let count = usize::try_from(shape.width)
        .ok()
        .and_then(|width| {
            usize::try_from(shape.height)
                .ok()
                .and_then(|height| width.checked_mul(height))
        })
        .and_then(|pixels| pixels.checked_mul(4))
        .ok_or(UiGlyphRasterizationDenial::ExtentExceeded)?;
    Ok(vec![0; count])
}

fn place_child(
    destination: &mut [u8],
    child: &UiCbdtCompositeImage,
    placement: BitmapPlacement,
) -> Result<(), UiGlyphRasterizationDenial> {
    let right = placement
        .offset_x
        .checked_add(
            i32::try_from(child.width).map_err(|_| UiGlyphRasterizationDenial::ExtentExceeded)?,
        )
        .ok_or(UiGlyphRasterizationDenial::ExtentExceeded)?;
    let bottom = placement
        .offset_y
        .checked_add(
            i32::try_from(child.height).map_err(|_| UiGlyphRasterizationDenial::ExtentExceeded)?,
        )
        .ok_or(UiGlyphRasterizationDenial::ExtentExceeded)?;
    let canvas_width =
        i32::try_from(placement.width).map_err(|_| UiGlyphRasterizationDenial::ExtentExceeded)?;
    let canvas_height =
        i32::try_from(placement.height).map_err(|_| UiGlyphRasterizationDenial::ExtentExceeded)?;
    let left = placement.offset_x.max(0);
    let top = placement.offset_y.max(0);
    let right = right.min(canvas_width);
    let bottom = bottom.min(canvas_height);
    if left >= right || top >= bottom {
        return Ok(());
    }
    for destination_y in top..bottom {
        for destination_x in left..right {
            let source_x = u32::try_from(destination_x - placement.offset_x)
                .map_err(|_| UiGlyphRasterizationDenial::InvalidColorPixels)?;
            let source_y = u32::try_from(destination_y - placement.offset_y)
                .map_err(|_| UiGlyphRasterizationDenial::InvalidColorPixels)?;
            let destination_index = usize::try_from(
                (u32::try_from(destination_y).unwrap() * placement.width
                    + u32::try_from(destination_x).unwrap())
                    * 4,
            )
            .unwrap();
            let source_index = usize::try_from((source_y * child.width + source_x) * 4).unwrap();
            source_over_bytes(
                &mut destination[destination_index..destination_index + 4],
                &child.pixels[source_index..source_index + 4],
            );
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signed_component_offsets_clip_to_the_parent_bitmap() {
        let child = UiCbdtCompositeImage {
            pixels: [255, 0, 0, 255].repeat(4),
            width: 2,
            height: 2,
            ppem_x: 16.0,
            ppem_y: 16.0,
            bearing_x: 0.0,
            bearing_y: 0.0,
        };
        let mut destination = vec![0; 2 * 2 * 4];
        place_child(
            &mut destination,
            &child,
            BitmapPlacement {
                width: 2,
                height: 2,
                offset_x: -1,
                offset_y: -1,
            },
        )
        .unwrap();
        assert_eq!(&destination[..4], &[255, 0, 0, 255]);
        assert!(destination[4..].iter().all(|byte| *byte == 0));
    }
}
