//! Deterministic COLR paint traversal and pixel production.

mod brush;

#[cfg(test)]
mod brush_tests;
#[cfg(test)]
mod tests;

use kurbo::{BezPath, Point, Shape};
use skrifa::{
    color::{
        Brush, ColorGlyphFormat, ColorPainter, CompositeMode, PaintCachedColorGlyph, PaintError,
    },
    instance::{LocationRef, NormalizedCoord, Size},
    outline::{Engine, HintingInstance},
    raw::types::BoundingBox,
    GlyphId, MetadataProvider,
};

use super::super::demand_candidate::UiGlyphRasterCandidate;
use super::super::denial::UiGlyphRasterizationDenial;
use super::compositing::{compose, UiLinearColorPixel};
use super::image::UiColorRasterGeometry;
use super::pixels::{finish_linear_image, pixels_per_em, UiLinearImagePlacement};
use super::transform::ColorTransform;
use crate::font_collection::color_glyph::path;
use crate::layout_artifact::UiQualifiedTextFaceResource;
use worth_ui_host_contract::UiGlyphRasterKey;

const SAMPLES_PER_AXIS: usize = 4;
const SAMPLE_OFFSETS: [f64; SAMPLES_PER_AXIS] = [0.125, 0.375, 0.625, 0.875];

pub(super) fn render_colr(
    resource: &UiQualifiedTextFaceResource,
    candidate: &UiGlyphRasterCandidate,
    key: UiGlyphRasterKey,
    geometry: UiColorRasterGeometry,
) -> Result<super::pixels::UiCanonicalColorImage, UiGlyphRasterizationDenial> {
    let face = harfrust::FontRef::from_index(resource.bytes(), key.face().face_index())
        .map_err(|_| UiGlyphRasterizationDenial::InvalidFaceResource)?;
    let glyph_id = GlyphId::new(key.glyph_id());
    let color_glyphs = face.color_glyphs();
    let glyph = color_glyphs
        .get_with_format(glyph_id, ColorGlyphFormat::ColrV1)
        .or_else(|| color_glyphs.get_with_format(glyph_id, ColorGlyphFormat::ColrV0))
        .ok_or(UiGlyphRasterizationDenial::OutlineUnavailable)?;
    let palette = palette_for(&face, key)?;
    let location = variation_location(&face, key);
    let scale = f64::from(pixels_per_em(key)) / f64::from(candidate.units_per_em);
    if !scale.is_finite() || scale <= 0.0 {
        return Err(UiGlyphRasterizationDenial::ExtentExceeded);
    }
    let placement = color_placement(candidate, key, geometry, scale)?;
    let pixels_per_em = pixels_per_em(key);
    let mut painter = ColorPainterImpl::new(ColorPainterInput {
        font: &face,
        coords: location.coords(),
        palette,
        geometry,
        scale,
        pixels_per_em,
        base_x: placement.left,
        top: placement.top,
    })?;
    glyph
        .paint(&location, &mut painter)
        .map_err(|_| UiGlyphRasterizationDenial::InvalidColorPixels)?;
    let pixels = painter.finish()?;
    finish_linear_image(
        UiLinearImagePlacement {
            width: geometry.width,
            height: geometry.height,
            left: placement.left,
            top: placement.top,
        },
        pixels,
    )
}

fn palette_for(
    face: &harfrust::FontRef<'_>,
    key: UiGlyphRasterKey,
) -> Result<Vec<skrifa::color::Color>, UiGlyphRasterizationDenial> {
    face.color_palettes()
        .get(key.palette().index())
        .map(|palette| palette.colors().to_vec())
        .ok_or(UiGlyphRasterizationDenial::InvalidColorPalette)
}

fn variation_location(
    face: &harfrust::FontRef<'_>,
    key: UiGlyphRasterKey,
) -> skrifa::instance::Location {
    face.axes()
        .location(key.variations().records().map(|variation| {
            (
                skrifa::Tag::from_be_bytes(variation.axis()),
                variation.value_milli() as f32 / 1_000.0,
            )
        }))
}

struct ColorPlacement {
    left: i32,
    top: i32,
}

fn color_placement(
    candidate: &UiGlyphRasterCandidate,
    key: UiGlyphRasterKey,
    geometry: UiColorRasterGeometry,
    scale: f64,
) -> Result<ColorPlacement, UiGlyphRasterizationDenial> {
    let origin_x = f64::from(key.fractional_origin().x_over_64()) / 64.0;
    let origin_y = f64::from(key.fractional_origin().y_over_64()) / 64.0;
    let left = (f64::from(candidate.ink_bounds.x_min()) * scale + origin_x).floor() as i32;
    let base_y = (f64::from(candidate.ink_bounds.y_min()) * scale + origin_y).ceil() as i32;
    let top = i32::try_from(geometry.height)
        .ok()
        .and_then(|height| height.checked_add(base_y))
        .ok_or(UiGlyphRasterizationDenial::ExtentExceeded)?;
    Ok(ColorPlacement { left, top })
}

struct ColorPainterImpl<'font, 'data> {
    font: &'font harfrust::FontRef<'data>,
    palette: Vec<skrifa::color::Color>,
    geometry: UiColorRasterGeometry,
    scale: f64,
    hinting: HintingInstance,
    base_x: i32,
    top: i32,
    transform: ColorTransform,
    transforms: Vec<ColorTransform>,
    clips: Vec<BezPath>,
    canvas: Vec<UiLinearColorPixel>,
    layers: Vec<ColorLayer>,
    painted: bool,
    failed: bool,
}

struct ColorPainterInput<'font, 'data, 'coords> {
    font: &'font harfrust::FontRef<'data>,
    coords: &'coords [NormalizedCoord],
    palette: Vec<skrifa::color::Color>,
    geometry: UiColorRasterGeometry,
    scale: f64,
    pixels_per_em: f32,
    base_x: i32,
    top: i32,
}

struct ColorLayer {
    mode: CompositeMode,
    destination: Vec<UiLinearColorPixel>,
}

impl<'font, 'data> ColorPainterImpl<'font, 'data> {
    fn new<'coords>(
        input: ColorPainterInput<'font, 'data, 'coords>,
    ) -> Result<Self, UiGlyphRasterizationDenial> {
        let pixels = usize::try_from(input.geometry.width)
            .ok()
            .and_then(|width| {
                usize::try_from(input.geometry.height)
                    .ok()
                    .and_then(|height| width.checked_mul(height))
            })
            .unwrap_or(0);
        let outlines = input.font.outline_glyphs();
        let hinting = HintingInstance::new(
            &outlines,
            Size::new(input.pixels_per_em),
            LocationRef::new(input.coords),
            Engine::Interpreter,
        )
        .map_err(|_| UiGlyphRasterizationDenial::OutlineUnavailable)?;
        Ok(Self {
            font: input.font,
            palette: input.palette,
            geometry: input.geometry,
            scale: input.scale,
            hinting,
            base_x: input.base_x,
            top: input.top,
            transform: ColorTransform::IDENTITY,
            transforms: Vec::new(),
            clips: Vec::new(),
            canvas: vec![UiLinearColorPixel::TRANSPARENT; pixels],
            layers: Vec::new(),
            painted: false,
            failed: false,
        })
    }

    fn finish(self) -> Result<Vec<u8>, UiGlyphRasterizationDenial> {
        if self.failed {
            return Err(UiGlyphRasterizationDenial::InvalidColorPixels);
        }
        if !self.painted {
            return Err(UiGlyphRasterizationDenial::EmptyRaster);
        }
        Ok(self
            .canvas
            .into_iter()
            .flat_map(|pixel| {
                [
                    to_byte(pixel.r),
                    to_byte(pixel.g),
                    to_byte(pixel.b),
                    to_byte(pixel.a),
                ]
            })
            .collect())
    }

    fn sample_point(&self, x: u32, y: u32, x_offset: f64, y_offset: f64) -> Point {
        Point::new(
            (f64::from(x) + x_offset + f64::from(self.base_x)) / self.scale,
            (f64::from(self.top) - f64::from(y) - y_offset) / self.scale,
        )
    }

    fn raster_fill(&mut self, brush: &Brush<'_>) {
        for y in 0..self.geometry.height {
            for x in 0..self.geometry.width {
                let mut source = UiLinearColorPixel::TRANSPARENT;
                for y_offset in SAMPLE_OFFSETS {
                    for x_offset in SAMPLE_OFFSETS {
                        // Clip paths already carry the active COLRv1 transform. The
                        // device sample must remain in the common font-space frame;
                        // transforming both would cancel translation/scale coverage.
                        let point = self.sample_point(x, y, x_offset, y_offset);
                        let color = match sample_clipped_brush(ClippedBrushSample {
                            clips: &self.clips,
                            brush,
                            point,
                            transform: self.transform,
                            palette: &self.palette,
                        }) {
                            Ok(Some(color)) => color,
                            Ok(None) => continue,
                            Err(()) => {
                                self.failed = true;
                                return;
                            }
                        };
                        let sample_weight =
                            1.0 / f64::from((SAMPLES_PER_AXIS * SAMPLES_PER_AXIS) as u32);
                        source.r += color.r * sample_weight;
                        source.g += color.g * sample_weight;
                        source.b += color.b * sample_weight;
                        source.a += color.a * sample_weight;
                    }
                }
                if source.a <= f64::EPSILON {
                    continue;
                }
                let index = usize::try_from(y * self.geometry.width + x).unwrap();
                self.canvas[index] =
                    match compose(source, self.canvas[index], CompositeMode::SrcOver) {
                        Some(pixel) => pixel,
                        None => {
                            self.failed = true;
                            return;
                        }
                    };
                self.painted = true;
            }
        }
    }
}

pub(super) struct ClippedBrushSample<'sample, 'brush> {
    pub(super) clips: &'sample [BezPath],
    pub(super) brush: &'sample Brush<'brush>,
    pub(super) point: Point,
    pub(super) transform: ColorTransform,
    pub(super) palette: &'sample [skrifa::color::Color],
}

pub(super) fn sample_clipped_brush(
    sample: ClippedBrushSample<'_, '_>,
) -> Result<Option<UiLinearColorPixel>, ()> {
    let ClippedBrushSample {
        clips,
        brush,
        point,
        transform,
        palette,
    } = sample;
    if !clips.iter().all(|clip| clip.contains(point)) {
        return Ok(None);
    }
    brush::sample(brush, point, transform, palette).map_err(|_| ())
}

impl ColorPainter for ColorPainterImpl<'_, '_> {
    fn push_transform(&mut self, transform: skrifa::color::Transform) {
        self.transforms.push(self.transform);
        self.transform = self.transform.concat(ColorTransform::from(transform));
    }

    fn pop_transform(&mut self) {
        self.transform = self.transforms.pop().unwrap_or(ColorTransform::IDENTITY);
    }

    fn push_clip_glyph(&mut self, glyph_id: GlyphId) {
        let path = path::hinted_glyph(
            self.font,
            glyph_id,
            &self.hinting,
            self.scale,
            self.transform.to_kurbo(),
        );
        match path {
            Some(path) => self.clips.push(path),
            None => self.failed = true,
        }
    }

    fn push_clip_box(&mut self, clip_box: BoundingBox<f32>) {
        self.clips.push(path::rectangle(
            kurbo::Rect::new(
                f64::from(clip_box.x_min),
                f64::from(clip_box.y_min),
                f64::from(clip_box.x_max),
                f64::from(clip_box.y_max),
            ),
            self.transform.to_kurbo(),
        ));
    }

    fn pop_clip(&mut self) {
        self.clips.pop();
    }

    fn fill(&mut self, brush: Brush<'_>) {
        self.raster_fill(&brush);
    }

    fn paint_cached_color_glyph(
        &mut self,
        _glyph: GlyphId,
    ) -> Result<PaintCachedColorGlyph, PaintError> {
        Ok(PaintCachedColorGlyph::Unimplemented)
    }

    fn push_layer(&mut self, composite_mode: CompositeMode) {
        let transparent = vec![UiLinearColorPixel::TRANSPARENT; self.canvas.len()];
        self.layers.push(ColorLayer {
            mode: composite_mode,
            destination: core::mem::replace(&mut self.canvas, transparent),
        });
    }

    fn pop_layer(&mut self) {
        let Some(layer) = self.layers.pop() else {
            self.failed = true;
            return;
        };
        let source = core::mem::replace(
            &mut self.canvas,
            vec![UiLinearColorPixel::TRANSPARENT; layer.destination.len()],
        );
        for (index, destination) in layer.destination.into_iter().enumerate() {
            self.canvas[index] = match compose(source[index], destination, layer.mode) {
                Some(pixel) => pixel,
                None => {
                    self.failed = true;
                    return;
                }
            };
        }
    }
}

fn to_byte(value: f64) -> u8 {
    (value.clamp(0.0, 1.0) * 255.0 + 0.5) as u8
}
