//! Qualified CBDT/CBLC and sbix bitmap selection and decoding.

mod composite;
mod decode;

use read_fonts::TableProvider;
use skrifa::{
    bitmap::{BitmapData, Origin},
    FontRef, GlyphId,
};
use swash::zeno::Placement;
use worth_ui_host_contract::UiGlyphRasterKey;

use self::composite::expand_cbdt_composite;
use self::decode::{decode_png, validate_bgra, validate_bitmap, UiBitmapPixels};
use super::super::denial::UiGlyphRasterizationDenial;
use super::image::UiColorRasterGeometry;
use super::pixels::{
    bilinear_resample, canonicalize_pixels, finish_image, pixels_per_em, scaled_dimension,
    UiCanonicalColorImage, UiColorPixelEncoding, UiResampleSize,
};
use crate::font_collection::color_glyph::bitmap_selection::{
    select as select_bitmap_source, UiBitmapSelection, UiBitmapSelectionError,
};
use crate::layout_artifact::UiQualifiedTextFaceResource;
use crate::UiQualifiedTextLayout;

struct UiBitmapGlyph<'a> {
    pixels: UiBitmapPixels<'a>,
    width: u32,
    height: u32,
    ppem_x: f32,
    ppem_y: f32,
    bearing_x: f32,
    bearing_y: f32,
    inner_bearing_x: f32,
    inner_bearing_y: f32,
    placement_origin: Origin,
}

pub(super) fn bitmap_geometry(
    layout: &UiQualifiedTextLayout,
    key: UiGlyphRasterKey,
) -> Result<UiColorRasterGeometry, UiGlyphRasterizationDenial> {
    let resource = layout
        .artifact()
        .face_resource(key.face())
        .ok_or(UiGlyphRasterizationDenial::InvalidFaceResource)?;
    let face = face_for_key(resource, key)?;
    let glyph = select_bitmap(&face, key)?;
    Ok(UiColorRasterGeometry {
        width: scaled_dimension(glyph.width, pixels_per_em(key) / glyph.ppem_x)?,
        height: scaled_dimension(glyph.height, pixels_per_em(key) / glyph.ppem_y)?,
    })
}

pub(super) fn render_color_bitmap(
    resource: &UiQualifiedTextFaceResource,
    key: UiGlyphRasterKey,
) -> Result<UiCanonicalColorImage, UiGlyphRasterizationDenial> {
    let face = face_for_key(resource, key)?;
    let glyph = select_bitmap(&face, key)?;
    let target_width = scaled_dimension(glyph.width, pixels_per_em(key) / glyph.ppem_x)?;
    let target_height = scaled_dimension(glyph.height, pixels_per_em(key) / glyph.ppem_y)?;
    let (source_pixels, encoding) = match &glyph.pixels {
        UiBitmapPixels::Png(bytes) => (
            decode_png(bytes, glyph.width, glyph.height)?,
            UiColorPixelEncoding::StraightRgba,
        ),
        UiBitmapPixels::Bgra(bytes) => {
            validate_bgra(bytes, glyph.width, glyph.height)?;
            (bytes.to_vec(), UiColorPixelEncoding::PremultipliedBgra)
        }
        UiBitmapPixels::LinearPremultipliedRgba(bytes) => {
            validate_bgra(bytes, glyph.width, glyph.height)?;
            (bytes.clone(), UiColorPixelEncoding::LinearPremultipliedRgba)
        }
    };
    let canonical = canonicalize_pixels(&source_pixels, encoding)?;
    let pixels = bilinear_resample(
        &canonical,
        UiResampleSize {
            source_width: glyph.width,
            source_height: glyph.height,
            target_width,
            target_height,
        },
    )?;
    let placement = bitmap_placement(
        &face,
        &glyph,
        key,
        UiColorRasterGeometry {
            width: target_width,
            height: target_height,
        },
    )?;
    finish_image(placement, pixels)
}

fn face_for_key<'a>(
    resource: &'a UiQualifiedTextFaceResource,
    key: UiGlyphRasterKey,
) -> Result<FontRef<'a>, UiGlyphRasterizationDenial> {
    FontRef::from_index(resource.bytes(), key.face().face_index())
        .map_err(|_| UiGlyphRasterizationDenial::InvalidFaceResource)
}

fn select_bitmap<'a>(
    face: &FontRef<'a>,
    key: UiGlyphRasterKey,
) -> Result<UiBitmapGlyph<'a>, UiGlyphRasterizationDenial> {
    let selection = select_bitmap_source(face, GlyphId::new(key.glyph_id()), pixels_per_em(key))
        .map_err(map_bitmap_selection_error)?;
    match selection.ok_or(UiGlyphRasterizationDenial::BitmapUnavailable)? {
        UiBitmapSelection::Direct(glyph) => bitmap_from_skrifa(glyph),
        UiBitmapSelection::CbdtComposite(selection) => {
            let composite = expand_cbdt_composite(selection)?
                .ok_or(UiGlyphRasterizationDenial::BitmapUnavailable)?;
            Ok(UiBitmapGlyph {
                pixels: UiBitmapPixels::LinearPremultipliedRgba(composite.pixels),
                width: composite.width,
                height: composite.height,
                ppem_x: composite.ppem_x,
                ppem_y: composite.ppem_y,
                bearing_x: 0.0,
                bearing_y: 0.0,
                inner_bearing_x: composite.bearing_x,
                inner_bearing_y: composite.bearing_y,
                placement_origin: Origin::TopLeft,
            })
        }
    }
}

fn map_bitmap_selection_error(error: UiBitmapSelectionError) -> UiGlyphRasterizationDenial {
    match error {
        UiBitmapSelectionError::Malformed => UiGlyphRasterizationDenial::BitmapUnavailable,
        UiBitmapSelectionError::Unsupported => UiGlyphRasterizationDenial::UnsupportedBitmapFormat,
    }
}

fn bitmap_from_skrifa<'a>(
    glyph: skrifa::bitmap::BitmapGlyph<'a>,
) -> Result<UiBitmapGlyph<'a>, UiGlyphRasterizationDenial> {
    let pixels = match glyph.data {
        BitmapData::Png(bytes) => UiBitmapPixels::Png(bytes),
        BitmapData::Bgra(bytes) => UiBitmapPixels::Bgra(bytes),
        BitmapData::Mask(_) => return Err(UiGlyphRasterizationDenial::UnsupportedBitmapFormat),
    };
    validate_bitmap(&pixels, glyph.width, glyph.height)?;
    Ok(UiBitmapGlyph {
        pixels,
        width: glyph.width,
        height: glyph.height,
        ppem_x: glyph.ppem_x,
        ppem_y: glyph.ppem_y,
        bearing_x: glyph.bearing_x,
        bearing_y: glyph.bearing_y,
        inner_bearing_x: glyph.inner_bearing_x,
        inner_bearing_y: glyph.inner_bearing_y,
        placement_origin: glyph.placement_origin,
    })
}

fn bitmap_placement(
    face: &FontRef<'_>,
    glyph: &UiBitmapGlyph<'_>,
    key: UiGlyphRasterKey,
    geometry: UiColorRasterGeometry,
) -> Result<Placement, UiGlyphRasterizationDenial> {
    let units_per_em = f32::from(
        face.head()
            .map_err(|_| UiGlyphRasterizationDenial::InvalidFaceResource)?
            .units_per_em(),
    );
    if units_per_em == 0.0 || !units_per_em.is_finite() {
        return Err(UiGlyphRasterizationDenial::InvalidFaceResource);
    }
    let scale_x = units_per_em / glyph.ppem_x;
    let scale_y = units_per_em / glyph.ppem_y;
    let image_left = glyph.bearing_x + glyph.inner_bearing_x * scale_x;
    let image_top = match glyph.placement_origin {
        Origin::TopLeft => glyph.bearing_y + glyph.inner_bearing_y * scale_y,
        Origin::BottomLeft => {
            glyph.bearing_y + glyph.inner_bearing_y * scale_y + glyph.height as f32 * scale_y
        }
    };
    let target_scale = pixels_per_em(key) / units_per_em;
    if !scale_x.is_finite()
        || !scale_y.is_finite()
        || !image_left.is_finite()
        || !image_top.is_finite()
        || !target_scale.is_finite()
    {
        return Err(UiGlyphRasterizationDenial::ExtentExceeded);
    }
    let fractional = key.fractional_origin();
    let left = (image_left * target_scale + fractional_pixel(fractional.x_over_64())).floor();
    let top = (image_top * target_scale + fractional_pixel(fractional.y_over_64())).ceil();
    Ok(Placement {
        left: left as i32,
        top: top as i32,
        width: geometry.width,
        height: geometry.height,
    })
}

fn fractional_pixel(value_over_64: i16) -> f32 {
    f32::from(value_over_64) / 64.0
}
