use harfrust::FontRef;
use read_fonts::{
    tables::{
        bitmap::{BitmapContent, BitmapDataFormat, BitmapSize, IndexSubtable},
        cbdt::Cbdt,
        cblc::Cblc,
    },
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
    validate_header(&cblc)?;
    let mut bitmaps = 0_u32;
    let mut covered = BTreeSet::new();
    for size in cblc.bitmap_sizes() {
        let (size_bitmaps, size_covered) = validate_size(&cblc, &cbdt, size, glyph_count)?;
        bitmaps = bitmaps.checked_add(size_bitmaps).ok_or_else(malformed)?;
        covered.extend(size_covered);
    }
    if bitmaps == 0 {
        Err(malformed())
    } else {
        Ok(covered.into_iter().collect())
    }
}

fn validate_header(cblc: &Cblc<'_>) -> Result<(), UiFontCollectionAdmissionDenial> {
    if cblc.major_version() != 3 || cblc.bitmap_sizes().is_empty() {
        return Err(malformed());
    }
    Ok(())
}

fn validate_size(
    cblc: &Cblc<'_>,
    cbdt: &Cbdt<'_>,
    size: &BitmapSize,
    glyph_count: u16,
) -> Result<(u32, BTreeSet<u16>), UiFontCollectionAdmissionDenial> {
    validate_size_fields(size, glyph_count)?;
    let context = CbdtValidationContext {
        cblc,
        cbdt,
        size,
        glyph_count,
    };
    let (bitmaps, graph) = collect_bitmap_graph(&context)?;
    validate_composite_graph(&graph)?;
    let covered = graph
        .iter()
        .enumerate()
        .filter_map(|(glyph, data)| data.as_ref().map(|_| glyph as u16))
        .collect();
    Ok((bitmaps, covered))
}

struct CbdtValidationContext<'font, 'borrow> {
    cblc: &'borrow Cblc<'font>,
    cbdt: &'borrow Cbdt<'font>,
    size: &'borrow BitmapSize,
    glyph_count: u16,
}

fn validate_size_fields(
    size: &BitmapSize,
    glyph_count: u16,
) -> Result<(), UiFontCollectionAdmissionDenial> {
    if size.start_glyph_index().to_u16() > size.end_glyph_index().to_u16()
        || size.end_glyph_index().to_u16() >= glyph_count
        || size.ppem_x() == 0
        || size.ppem_y() == 0
    {
        return Err(malformed());
    }
    if size.bit_depth() != 32 {
        return Err(UiFontCollectionAdmissionDenial::UnsupportedColorFontTable);
    }
    Ok(())
}

fn collect_bitmap_graph(
    context: &CbdtValidationContext<'_, '_>,
) -> Result<(u32, Vec<Option<Box<[u16]>>>), UiFontCollectionAdmissionDenial> {
    let list = context
        .size
        .index_subtable_list(context.cblc.offset_data())
        .map_err(|_| malformed())?;
    validate_table(&list, 0, context.glyph_count, 0, None)?;
    let mut previous_last = None;
    let mut graph = vec![None; usize::from(context.glyph_count)];
    let mut bitmaps = 0_u32;
    for record in list.index_subtable_records() {
        let first = record.first_glyph_index().to_u16();
        let last = record.last_glyph_index().to_u16();
        validate_record_range(context, previous_last, first, last)?;
        previous_last = Some(last);
        let subtable = record
            .index_subtable(list.offset_data())
            .map_err(|_| malformed())?;
        validate_table(&subtable, 0, context.glyph_count, 0, None)?;
        validate_sparse_order(&subtable, first, last)?;
        bitmaps += collect_record_bitmaps(context, &mut graph, first, last)?;
    }
    Ok((bitmaps, graph))
}

fn validate_record_range(
    context: &CbdtValidationContext<'_, '_>,
    previous_last: Option<u16>,
    first: u16,
    last: u16,
) -> Result<(), UiFontCollectionAdmissionDenial> {
    if first > last
        || first < context.size.start_glyph_index().to_u16()
        || last > context.size.end_glyph_index().to_u16()
        || last >= context.glyph_count
        || previous_last.is_some_and(|prior| first <= prior)
    {
        Err(malformed())
    } else {
        Ok(())
    }
}

fn collect_record_bitmaps(
    context: &CbdtValidationContext<'_, '_>,
    graph: &mut [Option<Box<[u16]>>],
    first: u16,
    last: u16,
) -> Result<u32, UiFontCollectionAdmissionDenial> {
    let mut bitmaps = 0_u32;
    for glyph in first..=last {
        let location = match context
            .size
            .location(context.cblc.offset_data(), GlyphId::new(glyph.into()))
        {
            Ok(location) => location,
            Err(read_fonts::ReadError::InvalidCollectionIndex(_)) => continue,
            Err(_) => return Err(malformed()),
        };
        if location.is_empty() {
            continue;
        }
        let bitmap = context.cbdt.data(&location).map_err(|_| malformed())?;
        if graph[usize::from(glyph)].is_some() {
            return Err(malformed());
        }
        graph[usize::from(glyph)] = Some(bitmap_components(&bitmap.content, context.glyph_count)?);
        bitmaps = bitmaps.checked_add(1).ok_or_else(malformed)?;
    }
    Ok(bitmaps)
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
        BitmapContent::Data(_, _) => {
            Err(UiFontCollectionAdmissionDenial::UnsupportedColorFontTable)
        }
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
