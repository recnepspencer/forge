mod bitmap;
mod colr;
mod png;
mod traversal;

use harfrust::FontRef;
use read_fonts::TableProvider;
use std::collections::BTreeSet;

use crate::font_collection::UiFontCollectionAdmissionDenial;

pub(in crate::font_collection) struct UiColorGlyphCoverage(Box<[u16]>);

impl UiColorGlyphCoverage {
    pub(in crate::font_collection) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(in crate::font_collection) fn contains(&self, glyph: u16) -> bool {
        self.0.binary_search(&glyph).is_ok()
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
    let mut coverage = BTreeSet::new();
    if colr {
        coverage.extend(colr::validate(font, glyphs)?);
    }
    if cbdt {
        coverage.extend(bitmap::validate_cbdt(font, glyphs)?);
    }
    if has(b"sbix") {
        coverage.extend(bitmap::validate_sbix(font, glyphs)?);
    }
    Ok(UiColorGlyphCoverage(
        coverage.into_iter().collect::<Vec<_>>().into_boxed_slice(),
    ))
}

const fn malformed() -> UiFontCollectionAdmissionDenial {
    UiFontCollectionAdmissionDenial::MalformedColorFontTables
}
