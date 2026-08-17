use read_fonts::{
    tables::colr::Colr,
    traversal::{FieldType, SomeArray, SomeTable},
};

use crate::font_collection::UiFontCollectionAdmissionDenial;

use super::{colr::valid_palette, malformed};

const MAX_COLOR_GRAPH_DEPTH: u8 = 64;

pub(super) fn validate_table(
    table: &dyn SomeTable<'_>,
    depth: u8,
    glyph_count: u16,
    palette_entries: u16,
    colr: Option<&Colr<'_>>,
) -> Result<(), UiFontCollectionAdmissionDenial> {
    if depth >= MAX_COLOR_GRAPH_DEPTH {
        return Err(malformed());
    }
    let mut first_layer = None;
    let mut layer_count = None;
    let mut index = 0;
    while let Some(field) = table.get_field(index) {
        index += 1;
        match (&*field.name, &field.value) {
            ("glyph_id", FieldType::GlyphId16(glyph)) if glyph.to_u16() >= glyph_count => {
                return Err(malformed());
            }
            ("palette_index", FieldType::U16(value)) if !valid_palette(*value, palette_entries) => {
                return Err(malformed());
            }
            ("composite_mode", FieldType::U8(value)) if *value > 27 => {
                return Err(malformed());
            }
            ("extend", FieldType::U8(value)) if *value > 2 => return Err(malformed()),
            ("first_layer_index", FieldType::U32(value)) => first_layer = Some(*value),
            ("num_layers", FieldType::U8(value)) => layer_count = Some(*value),
            _ => {}
        }
        validate_field(field.value, depth + 1, glyph_count, palette_entries, colr)?;
    }
    if table.type_name() == "PaintColrLayers" {
        let colr = colr.ok_or_else(malformed)?;
        let start = first_layer.ok_or_else(malformed)?;
        let count = u32::from(layer_count.ok_or_else(malformed)?);
        if count == 0 {
            return Err(malformed());
        }
        let end = start.checked_add(count).ok_or_else(malformed)?;
        for index in start..end {
            let (paint, _) = colr.v1_layer(index as usize).map_err(|_| malformed())?;
            validate_table(&paint, depth + 1, glyph_count, palette_entries, Some(colr))?;
        }
    }
    Ok(())
}

fn validate_field(
    field: FieldType<'_>,
    depth: u8,
    glyph_count: u16,
    palette_entries: u16,
    colr: Option<&Colr<'_>>,
) -> Result<(), UiFontCollectionAdmissionDenial> {
    match field {
        FieldType::ResolvedOffset(offset) => validate_table(
            offset.target.map_err(|_| malformed())?.as_ref(),
            depth,
            glyph_count,
            palette_entries,
            colr,
        ),
        FieldType::Record(record) => {
            validate_table(&record, depth, glyph_count, palette_entries, colr)
        }
        FieldType::ArrayOffset(array) => validate_array(
            array.target.map_err(|_| malformed())?.as_ref(),
            depth,
            glyph_count,
            palette_entries,
            colr,
        ),
        FieldType::Array(array) => {
            validate_array(array.as_ref(), depth, glyph_count, palette_entries, colr)
        }
        FieldType::StringOffset(string) => string
            .target
            .map(|value| value.iter_chars().for_each(drop))
            .map_err(|_| malformed()),
        FieldType::BareOffset(offset) if offset.to_u32() != 0 => Err(malformed()),
        FieldType::Unknown => Err(malformed()),
        _ => Ok(()),
    }
}

fn validate_array(
    array: &dyn SomeArray<'_>,
    depth: u8,
    glyph_count: u16,
    palette_entries: u16,
    colr: Option<&Colr<'_>>,
) -> Result<(), UiFontCollectionAdmissionDenial> {
    for index in 0..array.len() {
        let field = array.get(index).ok_or_else(malformed)?;
        validate_field(field, depth, glyph_count, palette_entries, colr)?;
    }
    Ok(())
}
