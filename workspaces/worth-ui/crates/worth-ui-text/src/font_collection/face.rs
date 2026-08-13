use std::sync::Arc;

use harfrust::{
    BufferClusterLevel, Direction, FontRef, Language, Script, ShapeOptions, ShaperData,
    ShaperInstance, Tag, UnicodeBuffer,
};
use worth_ui_host_contract::{
    UiFontSlant, UiQualifiedFontFaceIdentity, UiQualifiedFontFamilyIdentity,
    UiQualifiedFontPackIdentity,
};

use super::{coverage::UiFontCoverageIndex, UiFontShapeProbe, UiFontShapedGlyph, UiFontShapedRun};

pub(in crate::font_collection) mod feature_inventory;
mod variation;

pub(in crate::font_collection) use variation::axis_range;
use variation::{
    features, variable_face_axes, variations, variations_are_qualified, UiVariableFaceAxes,
};

pub(super) struct UiQualifiedFontFace {
    bytes: Arc<[u8]>,
    face_index: u32,
    identity: UiQualifiedFontFaceIdentity,
    family: UiQualifiedFontFamilyIdentity,
    pack: Option<UiQualifiedFontPackIdentity>,
    weight: u16,
    width_milli_percent: u32,
    slant: UiFontSlant,
    variable_axes: UiVariableFaceAxes,
    emoji: bool,
    color_glyphs: super::application_pack::color_tables::UiColorGlyphCoverage,
    last_resort: bool,
    shaper_data: ShaperData,
    horizontal_metrics: UiFontHorizontalMetrics,
    coverage: UiFontCoverageIndex,
    feature_tags: Box<[[u8; 4]]>,
}

#[derive(Clone, Copy)]
struct UiFontHorizontalMetrics {
    ascender: i16,
    descender: i16,
    line_gap: i16,
}

impl UiQualifiedFontFace {
    pub(super) fn admit(
        input: UiQualifiedFontFaceInput,
    ) -> Result<Self, super::UiFontCollectionAdmissionDenial> {
        use super::UiFontCollectionAdmissionDenial as Denial;
        let font = FontRef::from_index(&input.bytes, input.face_index)
            .map_err(|_| Denial::MalformedFont)?;
        let shaper_data = ShaperData::new(&font);
        let horizontal_metrics = horizontal_metrics(&font).ok_or(Denial::MalformedFont)?;
        let variable_axes = variable_face_axes(&font);
        let coverage =
            UiFontCoverageIndex::from_font(&font).ok_or(Denial::MissingUnicodeCoverage)?;
        let feature_tags = feature_inventory::derive(&font)?;
        let color_glyphs = super::application_pack::color_tables::validate(&font)?;
        if input.intrinsic_color != !color_glyphs.is_empty() {
            return Err(Denial::MalformedColorFontTables);
        }
        Ok(Self {
            identity: input.identity,
            bytes: input.bytes,
            face_index: input.face_index,
            family: input.family,
            pack: input.pack,
            weight: input.weight,
            width_milli_percent: input.width_milli_percent,
            slant: input.slant,
            variable_axes,
            emoji: input.emoji,
            color_glyphs,
            last_resort: input.last_resort,
            shaper_data,
            horizontal_metrics,
            coverage,
            feature_tags,
        })
    }

    pub(super) const fn identity(&self) -> UiQualifiedFontFaceIdentity {
        self.identity
    }

    pub(super) const fn is_emoji(&self) -> bool {
        self.emoji
    }

    pub(super) fn has_intrinsic_color(&self) -> bool {
        !self.color_glyphs.is_empty()
    }

    pub(super) const fn is_last_resort(&self) -> bool {
        self.last_resort
    }

    pub(super) const fn family(&self) -> UiQualifiedFontFamilyIdentity {
        self.family
    }

    pub(super) const fn pack(&self) -> Option<UiQualifiedFontPackIdentity> {
        self.pack
    }

    pub(super) const fn weight(&self) -> u16 {
        self.weight
    }

    pub(super) const fn width_milli_percent(&self) -> u32 {
        self.width_milli_percent
    }

    pub(super) const fn slant(&self) -> UiFontSlant {
        self.slant
    }

    pub(super) const fn has_variable_weight(&self) -> bool {
        self.variable_axes.has_weight()
    }

    pub(super) const fn has_variable_width(&self) -> bool {
        self.variable_axes.has_width()
    }

    pub(super) const fn has_variable_slant(&self) -> bool {
        self.variable_axes.has_slant()
    }

    pub(super) fn weight_distance(&self, requested: u16) -> u32 {
        if self.has_variable_weight() {
            self.variable_axes.weight_distance(requested)
        } else {
            u32::from(self.weight.abs_diff(requested)) * 1_000
        }
    }

    pub(super) fn width_distance(&self, requested: u32) -> u32 {
        if self.has_variable_width() {
            self.variable_axes.width_distance(requested)
        } else {
            self.width_milli_percent.abs_diff(requested)
        }
    }

    pub(super) fn slant_supports(&self, requested: UiFontSlant) -> bool {
        self.has_variable_slant() && self.variable_axes.slant_supports(requested)
    }

    pub(super) fn contains_cluster(&self, text: &str) -> bool {
        self.last_resort || self.coverage.contains_cluster(text)
    }

    pub(super) fn coverage_range_count(&self) -> usize {
        self.coverage.range_count()
    }

    pub(super) fn resource(&self) -> crate::UiQualifiedTextFaceResource {
        crate::UiQualifiedTextFaceResource::new(
            self.identity,
            self.family,
            self.pack,
            Arc::clone(&self.bytes),
            self.has_intrinsic_color(),
        )
    }

    pub(super) fn probe(
        &self,
        text: &str,
        direction: Direction,
        language: &Language,
        style: &crate::UiTextStyle,
        require_color: bool,
    ) -> UiFontShapeProbe {
        let font =
            FontRef::from_index(&self.bytes, self.face_index).expect("admitted font remains valid");
        let mut buffer = UnicodeBuffer::new();
        buffer.push_str(text);
        buffer.set_cluster_level(BufferClusterLevel::MonotoneGraphemes);
        buffer.set_direction(direction);
        buffer.set_language(language.clone());
        buffer.guess_segment_properties();
        let script = buffer.script();
        if !require_color && !variations_are_qualified(&font, style) {
            return UiFontShapeProbe {
                script,
                glyph_count: 0,
                has_notdef: true,
                variation_qualified: false,
                features_qualified: true,
                color_qualified: !require_color,
            };
        }
        if !require_color
            && !style
                .features()
                .iter()
                .all(|feature| self.feature_tags.binary_search(&feature.tag()).is_ok())
        {
            return UiFontShapeProbe {
                script,
                glyph_count: 0,
                has_notdef: true,
                variation_qualified: true,
                features_qualified: false,
                color_qualified: !require_color,
            };
        }
        let features = features(style);
        let variations = variations(&font, style);
        let instance = ShaperInstance::from_variations(&font, variations);
        let glyphs = self
            .shaper_data
            .shaper(&font)
            .instance(Some(&instance))
            .build()
            .shape(buffer, ShapeOptions::new().features(&features));
        let color_qualified = !require_color
            || glyphs.glyph_infos().iter().all(|glyph| {
                u16::try_from(glyph.glyph_id)
                    .ok()
                    .is_some_and(|glyph| self.color_glyphs.contains(glyph))
            });
        UiFontShapeProbe {
            script,
            glyph_count: glyphs.glyph_infos().len(),
            has_notdef: glyphs.glyph_infos().iter().any(|glyph| glyph.glyph_id == 0),
            variation_qualified: true,
            features_qualified: true,
            color_qualified,
        }
    }

    pub(super) fn shape_run(
        &self,
        text: &str,
        original_start: u32,
        direction: Direction,
        language: &Language,
        script_tag: [u8; 4],
        style: &crate::UiTextStyle,
    ) -> UiFontShapedRun {
        let font =
            FontRef::from_index(&self.bytes, self.face_index).expect("admitted font remains valid");
        let mut buffer = UnicodeBuffer::new();
        for (offset, character) in text.char_indices() {
            buffer.add(
                character,
                original_start + u32::try_from(offset).expect("admitted text fits u32"),
            );
        }
        buffer.set_cluster_level(BufferClusterLevel::MonotoneGraphemes);
        buffer.set_direction(direction);
        buffer.set_language(language.clone());
        buffer.set_script(
            Script::from_iso15924_tag(Tag::from_be_bytes(script_tag)).expect("valid script tag"),
        );
        let features = features(style);
        let variations = variations(&font, style);
        let instance = ShaperInstance::from_variations(&font, variations);
        let shaper = self
            .shaper_data
            .shaper(&font)
            .instance(Some(&instance))
            .build();
        let units_per_em = u16::try_from(shaper.units_per_em()).expect("OpenType units per em");
        let glyph_buffer = shaper.shape(buffer, ShapeOptions::new().features(&features));
        let glyphs = glyph_buffer
            .glyph_infos()
            .iter()
            .zip(glyph_buffer.glyph_positions())
            .map(|(info, position)| UiFontShapedGlyph {
                glyph_id: info.glyph_id,
                cluster: info.cluster,
                x_advance: position.x_advance,
                y_advance: position.y_advance,
                x_offset: position.x_offset,
                y_offset: position.y_offset,
                ink_bounds: super::ink_bounds::for_glyph(&font, &instance, info.glyph_id),
                unsafe_to_break: info.unsafe_to_break(),
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        UiFontShapedRun {
            units_per_em,
            ascender_font_units: self.horizontal_metrics.ascender,
            descender_font_units: self.horizontal_metrics.descender,
            line_gap_font_units: self.horizontal_metrics.line_gap,
            glyphs,
        }
    }
}

pub(super) struct UiQualifiedFontFaceInput {
    pub(super) bytes: Arc<[u8]>,
    pub(super) face_index: u32,
    pub(super) identity: UiQualifiedFontFaceIdentity,
    pub(super) family: UiQualifiedFontFamilyIdentity,
    pub(super) pack: Option<UiQualifiedFontPackIdentity>,
    pub(super) weight: u16,
    pub(super) width_milli_percent: u32,
    pub(super) slant: UiFontSlant,
    pub(super) emoji: bool,
    pub(super) intrinsic_color: bool,
    pub(super) last_resort: bool,
}

fn horizontal_metrics(font: &FontRef<'_>) -> Option<UiFontHorizontalMetrics> {
    let data = font.table_data(Tag::from_be_bytes(*b"hhea"))?;
    let bytes = data.as_bytes();
    Some(UiFontHorizontalMetrics {
        ascender: be_i16(bytes, 4)?,
        descender: be_i16(bytes, 6)?,
        line_gap: be_i16(bytes, 8)?,
    })
}

fn be_i16(bytes: &[u8], start: usize) -> Option<i16> {
    Some(i16::from_be_bytes(
        bytes.get(start..start + 2)?.try_into().ok()?,
    ))
}

#[cfg(test)]
#[path = "face_tests.rs"]
mod tests;
