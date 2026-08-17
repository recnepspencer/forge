mod bitmap;
pub(crate) mod bitmap_selection;
mod boundedness;
mod colr;
pub(crate) mod path;
mod png;
mod traversal;

use harfrust::FontRef;
use read_fonts::TableProvider;
use std::collections::BTreeMap;

use crate::font_collection::UiFontCollectionAdmissionDenial;
use crate::layout_artifact::{UiQualifiedTextColorGlyph, UiQualifiedTextColorSource};

pub(in crate::font_collection) struct UiColorGlyphCoverage(Box<[UiQualifiedTextColorGlyph]>);

impl UiColorGlyphCoverage {
    pub(in crate::font_collection) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(in crate::font_collection) fn contains(&self, glyph: u16) -> bool {
        self.0.iter().any(|candidate| candidate.glyph_id() == glyph)
    }

    pub(in crate::font_collection) fn iter(
        &self,
    ) -> impl Iterator<Item = UiQualifiedTextColorGlyph> + '_ {
        self.0.iter().copied()
    }
}

pub(in crate::font_collection) fn validate(
    font: &FontRef<'_>,
) -> Result<UiColorGlyphCoverage, UiFontCollectionAdmissionDenial> {
    use UiFontCollectionAdmissionDenial as Denial;
    let has = |tag| {
        font.data_for_tag(read_fonts::types::Tag::new(tag))
            .is_some()
    };
    if has(b"SVG ") {
        return Err(Denial::UnsupportedColorFontTable);
    }
    let colr = has(b"COLR");
    let cpal = has(b"CPAL");
    let cbdt = has(b"CBDT");
    let cblc = has(b"CBLC");
    if colr != cpal || cbdt != cblc {
        return Err(Denial::MalformedColorFontTables);
    }
    let glyphs = font.maxp().map_err(|_| Denial::MalformedFont)?.num_glyphs();
    let mut coverage = BTreeMap::new();
    if colr {
        insert_source(
            &mut coverage,
            colr::validate(font, glyphs)?,
            UiQualifiedTextColorSource::Outline,
        )?;
    }
    if cbdt {
        insert_source(
            &mut coverage,
            bitmap::validate_cbdt(font, glyphs)?,
            UiQualifiedTextColorSource::Bitmap,
        )?;
    }
    if has(b"sbix") {
        insert_source(
            &mut coverage,
            bitmap::validate_sbix(font, glyphs)?,
            UiQualifiedTextColorSource::Bitmap,
        )?;
    }
    Ok(UiColorGlyphCoverage(
        coverage
            .into_iter()
            .map(|(glyph, source)| UiQualifiedTextColorGlyph::new(glyph, source))
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    ))
}

fn insert_source(
    coverage: &mut BTreeMap<u16, UiQualifiedTextColorSource>,
    glyphs: impl IntoIterator<Item = u16>,
    source: UiQualifiedTextColorSource,
) -> Result<(), UiFontCollectionAdmissionDenial> {
    for glyph in glyphs {
        match coverage.get(&glyph).copied() {
            None => {
                coverage.insert(glyph, source);
            }
            Some(prior) if prior == source => {}
            Some(UiQualifiedTextColorSource::Outline) => {}
            Some(UiQualifiedTextColorSource::Bitmap) => {
                coverage.insert(glyph, source);
            }
        }
    }
    Ok(())
}

const fn malformed() -> UiFontCollectionAdmissionDenial {
    UiFontCollectionAdmissionDenial::MalformedColorFontTables
}
