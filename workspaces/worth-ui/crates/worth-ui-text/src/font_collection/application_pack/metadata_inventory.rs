use harfrust::{FontRef, Tag};

use super::super::{UiQualifiedFontAxisReceipt, UiQualifiedFontNameRecordReceipt};
use crate::font_collection::{coverage::UiFontCoverageIndex, UiFontCollectionAdmissionDenial};

pub(super) struct UiApplicationFaceMetadata {
    pub(super) family_names: Box<[UiQualifiedFontNameRecordReceipt]>,
    pub(super) style_names: Box<[UiQualifiedFontNameRecordReceipt]>,
    pub(super) axes: Box<[UiQualifiedFontAxisReceipt]>,
    pub(super) feature_tags: Box<[[u8; 4]]>,
    pub(super) coverage_range_count: u32,
    pub(super) intrinsic_color: bool,
    pub(super) max_glyphs_per_input_byte: usize,
}

pub(super) fn derive(
    font: &FontRef<'_>,
    intrinsic_color: bool,
) -> Result<UiApplicationFaceMetadata, UiFontCollectionAdmissionDenial> {
    let (family_names, style_names) = super::name_inventory::derive(font)?;
    let coverage = UiFontCoverageIndex::from_font(font)
        .ok_or(UiFontCollectionAdmissionDenial::MissingUnicodeCoverage)?;
    Ok(UiApplicationFaceMetadata {
        family_names,
        style_names,
        axes: axis_receipts(font)?,
        feature_tags: crate::font_collection::face::feature_inventory::derive(font)?,
        coverage_range_count: u32::try_from(coverage.range_count())
            .map_err(|_| UiFontCollectionAdmissionDenial::MissingUnicodeCoverage)?,
        intrinsic_color,
        max_glyphs_per_input_byte: super::glyph_expansion::derive(font)?,
    })
}

fn axis_receipts(
    font: &FontRef<'_>,
) -> Result<Box<[UiQualifiedFontAxisReceipt]>, UiFontCollectionAdmissionDenial> {
    let Some(data) = font.table_data(Tag::from_be_bytes(*b"fvar")) else {
        return Ok(Box::new([]));
    };
    let bytes = data.as_bytes();
    let offset =
        usize::from(be_u16(bytes, 4).ok_or(UiFontCollectionAdmissionDenial::FaceMetadataMismatch)?);
    let count =
        usize::from(be_u16(bytes, 8).ok_or(UiFontCollectionAdmissionDenial::FaceMetadataMismatch)?);
    let size = usize::from(
        be_u16(bytes, 10).ok_or(UiFontCollectionAdmissionDenial::FaceMetadataMismatch)?,
    );
    if size < 20 {
        return Err(UiFontCollectionAdmissionDenial::FaceMetadataMismatch);
    }
    let mut axes = Vec::with_capacity(count);
    for index in 0..count {
        let start = offset
            .checked_add(
                index
                    .checked_mul(size)
                    .ok_or(UiFontCollectionAdmissionDenial::FaceMetadataMismatch)?,
            )
            .ok_or(UiFontCollectionAdmissionDenial::FaceMetadataMismatch)?;
        let tag: [u8; 4] = bytes
            .get(start..start + 4)
            .and_then(|value| value.try_into().ok())
            .ok_or(UiFontCollectionAdmissionDenial::FaceMetadataMismatch)?;
        let minimum_milli = fixed_milli(bytes, start + 4)?;
        let default_milli = fixed_milli(bytes, start + 8)?;
        let maximum_milli = fixed_milli(bytes, start + 12)?;
        if minimum_milli > default_milli || default_milli > maximum_milli {
            return Err(UiFontCollectionAdmissionDenial::FaceMetadataMismatch);
        }
        axes.push(UiQualifiedFontAxisReceipt {
            tag,
            minimum_milli,
            default_milli,
            maximum_milli,
        });
    }
    axes.sort_by_key(|axis| axis.tag());
    if axes.windows(2).any(|pair| pair[0].tag() == pair[1].tag()) {
        return Err(UiFontCollectionAdmissionDenial::FaceMetadataMismatch);
    }
    Ok(axes.into_boxed_slice())
}

fn fixed_milli(bytes: &[u8], start: usize) -> Result<i32, UiFontCollectionAdmissionDenial> {
    let raw = i32::from_be_bytes(
        bytes
            .get(start..start + 4)
            .and_then(|value| value.try_into().ok())
            .ok_or(UiFontCollectionAdmissionDenial::FaceMetadataMismatch)?,
    );
    i32::try_from(i64::from(raw) * 1_000 / 65_536)
        .map_err(|_| UiFontCollectionAdmissionDenial::FaceMetadataMismatch)
}

fn be_u16(bytes: &[u8], start: usize) -> Option<u16> {
    Some(u16::from_be_bytes(
        bytes.get(start..start + 2)?.try_into().ok()?,
    ))
}
