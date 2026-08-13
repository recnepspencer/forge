use harfrust::FontRef;
use read_fonts::{
    tables::bitmap::{BitmapContent, BitmapDataFormat, IndexSubtable},
    types::GlyphId,
    TableProvider,
};
use std::collections::BTreeSet;

use crate::font_collection::UiFontCollectionAdmissionDenial;

use super::super::{malformed, png::validate_png, traversal::validate_table};

pub(in crate::font_collection) fn validate(
    font: &FontRef<'_>,
    glyph_count: u16,
) -> Result<Box<[u16]>, UiFontCollectionAdmissionDenial> {
    let cblc = font.cblc().map_err(|_| malformed())?;
    let cbdt = font.cbdt().map_err(|_| malformed())?;
    validate_table(&cblc, 0, glyph_count, 0, None)?;
    validate_table(&cbdt, 0, glyph_count, 0, None)?;
    if cblc.major_version() != 3 || cblc.bitmap_sizes().is_empty() {
        return Err(malformed());
    }
    let mut bitmaps = 0_u32;
    let mut covered = BTreeSet::new();
    for size in cblc.bitmap_sizes() {
        if size.start_glyph_index().to_u16() > size.end_glyph_index().to_u16()
            || size.end_glyph_index().to_u16() >= glyph_count
            || size.ppem_x() == 0
            || size.ppem_y() == 0
            || !matches!(size.bit_depth(), 1 | 2 | 4 | 8 | 32)
        {
            return Err(malformed());
        }
        let list = size
            .index_subtable_list(cblc.offset_data())
            .map_err(|_| malformed())?;
        validate_table(&list, 0, glyph_count, 0, None)?;
        let mut previous_last = None;
        let mut glyph_graph = vec![None; usize::from(glyph_count)];
        for record in list.index_subtable_records() {
            let first = record.first_glyph_index().to_u16();
            let last = record.last_glyph_index().to_u16();
            if first > last
                || first < size.start_glyph_index().to_u16()
                || last > size.end_glyph_index().to_u16()
                || last >= glyph_count
                || previous_last.is_some_and(|prior| first <= prior)
            {
                return Err(malformed());
            }
            previous_last = Some(last);
            let subtable = record
                .index_subtable(list.offset_data())
                .map_err(|_| malformed())?;
            validate_table(&subtable, 0, glyph_count, 0, None)?;
            validate_sparse_order(&subtable, first, last)?;
            for glyph in first..=last {
                let location = match size.location(cblc.offset_data(), GlyphId::new(glyph.into())) {
                    Ok(location) => location,
                    Err(read_fonts::ReadError::InvalidCollectionIndex(_)) => continue,
                    Err(_) => return Err(malformed()),
                };
                if location.is_empty() {
                    continue;
                }
                let bitmap = cbdt.data(&location).map_err(|_| malformed())?;
                if glyph_graph[usize::from(glyph)].is_some() {
                    return Err(malformed());
                }
                glyph_graph[usize::from(glyph)] =
                    Some(bitmap_components(&bitmap.content, glyph_count)?);
                bitmaps = bitmaps.checked_add(1).ok_or_else(malformed)?;
            }
        }
        validate_composite_graph(&glyph_graph)?;
        covered.extend(
            glyph_graph
                .iter()
                .enumerate()
                .filter_map(|(glyph, data)| data.as_ref().map(|_| glyph as u16)),
        );
    }
    if bitmaps == 0 {
        Err(malformed())
    } else {
        Ok(covered.into_iter().collect())
    }
}
fn bitmap_components(
    content: &BitmapContent<'_>,
    glyph_count: u16,
) -> Result<Box<[u16]>, UiFontCollectionAdmissionDenial> {
    match content {
        BitmapContent::Data(BitmapDataFormat::Png, bytes) => {
            validate_png(bytes)?;
            Ok(Box::new([]))
        }
        BitmapContent::Data(_, bytes) if !bytes.is_empty() => Ok(Box::new([])),
        BitmapContent::Composite(components) => {
            if components.is_empty()
                || components
                    .iter()
                    .any(|component| component.glyph_id().to_u16() >= glyph_count)
            {
                return Err(malformed());
            }
            Ok(components
                .iter()
                .map(|component| component.glyph_id().to_u16())
                .collect())
        }
        _ => Err(malformed()),
    }
}

fn validate_sparse_order(
    subtable: &IndexSubtable<'_>,
    first: u16,
    last: u16,
) -> Result<(), UiFontCollectionAdmissionDenial> {
    match subtable {
        IndexSubtable::Format4(table) => {
            let glyphs = table.glyph_array();
            let count = usize::try_from(table.num_glyphs()).map_err(|_| malformed())?;
            if glyphs.len() != count + 1
                || glyphs[..count].iter().any(|item| {
                    let glyph = item.glyph_id().to_u16();
                    glyph < first || glyph > last
                })
                || glyphs.windows(2).any(|pair| {
                    pair[0].glyph_id().to_u16() >= pair[1].glyph_id().to_u16()
                        || pair[0].sbit_offset() > pair[1].sbit_offset()
                })
            {
                Err(malformed())
            } else {
                Ok(())
            }
        }
        IndexSubtable::Format5(table) => {
            let glyphs = table.glyph_array();
            if glyphs.len() != table.num_glyphs() as usize
                || glyphs.iter().any(|item| {
                    let glyph = item.get().to_u16();
                    glyph < first || glyph > last
                })
                || glyphs
                    .windows(2)
                    .any(|pair| pair[0].get().to_u16() >= pair[1].get().to_u16())
            {
                Err(malformed())
            } else {
                Ok(())
            }
        }
        _ => Ok(()),
    }
}

fn validate_composite_graph(
    graph: &[Option<Box<[u16]>>],
) -> Result<(), UiFontCollectionAdmissionDenial> {
    let mut state = vec![0_u8; graph.len()];
    for start in 0..graph.len() {
        if graph[start].is_none() || state[start] != 0 {
            continue;
        }
        state[start] = 1;
        let mut stack = vec![(start, 0_usize)];
        while let Some((node, next_edge)) = stack.last_mut() {
            let edges = graph[*node].as_deref().ok_or_else(malformed)?;
            if *next_edge == edges.len() {
                state[*node] = 2;
                stack.pop();
                continue;
            }
            let target = usize::from(edges[*next_edge]);
            *next_edge += 1;
            if graph.get(target).and_then(Option::as_ref).is_none() || state[target] == 1 {
                return Err(malformed());
            }
            if state[target] == 0 {
                state[target] = 1;
                stack.push((target, 0));
            }
        }
    }
    Ok(())
}
