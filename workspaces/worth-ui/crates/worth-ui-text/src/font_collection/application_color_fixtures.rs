//! Checksum-valid synthetic color-font tables used by application admission tests.

use std::sync::Arc;

pub(super) fn corrupt_cblc_location(bytes: &[u8]) -> Arc<[u8]> {
    let mut mutated = bytes.to_vec();
    let table = table_range(&mutated, *b"CBLC");
    assert!(table.len() >= 12, "fixture owns one BitmapSize record");
    mutated[table.start + 8..table.start + 12].copy_from_slice(&u32::MAX.to_be_bytes());
    refresh_named_table_checksum(&mut mutated, *b"CBLC");
    Arc::from(mutated)
}

pub(super) fn corrupt_cbdt_png(bytes: &[u8]) -> Arc<[u8]> {
    let mut mutated = bytes.to_vec();
    let table = table_range(&mutated, *b"CBDT");
    let signature = mutated[table.clone()]
        .windows(8)
        .position(|window| window == b"\x89PNG\r\n\x1a\n")
        .expect("fixture owns PNG-backed CBDT data");
    mutated[table.start + signature + 16] ^= 1;
    refresh_named_table_checksum(&mut mutated, *b"CBDT");
    Arc::from(mutated)
}

pub(super) fn unsupported_cbdt_bit_depth(bytes: &[u8]) -> Arc<[u8]> {
    let mut mutated = bytes.to_vec();
    let table = table_range(&mutated, *b"CBLC");
    let size_count = usize::try_from(be_u32(&mutated, table.start + 4)).unwrap();
    for index in 0..size_count {
        mutated[table.start + 8 + index * 48 + 46] = 8;
    }
    refresh_named_table_checksum(&mut mutated, *b"CBLC");
    Arc::from(mutated)
}

pub(super) fn rename_tables(bytes: &[u8], replacements: &[([u8; 4], [u8; 4])]) -> Arc<[u8]> {
    let mut mutated = bytes.to_vec();
    let count = usize::from(be_u16(&mutated, 4));
    for (from, to) in replacements {
        let record = (0..count)
            .map(|index| 12 + index * 16)
            .find(|start| mutated[*start..*start + 4] == *from)
            .expect("fixture owns substituted table");
        mutated[record..record + 4].copy_from_slice(to);
    }
    let mut records = (0..count)
        .map(|index| mutated[12 + index * 16..28 + index * 16].to_vec())
        .collect::<Vec<_>>();
    records.sort_unstable_by(|left, right| left[..4].cmp(&right[..4]));
    for (index, record) in records.into_iter().enumerate() {
        mutated[12 + index * 16..28 + index * 16].copy_from_slice(&record);
    }
    Arc::from(mutated)
}

pub(super) fn maxp_glyph_count(bytes: &[u8]) -> u16 {
    let range = table_range(bytes, *b"maxp");
    be_u16(bytes, range.start + 4)
}

pub(super) fn colr_v0() -> Vec<u8> {
    colr_v0_for_glyph(1)
}

pub(super) fn colr_v0_for_glyph(glyph: u16) -> Vec<u8> {
    let mut table = vec![
        0, 0, 0, 1, 0, 0, 0, 14, 0, 0, 0, 20, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0,
    ];
    table[14..16].copy_from_slice(&glyph.to_be_bytes());
    table[20..22].copy_from_slice(&glyph.to_be_bytes());
    table
}

pub(super) fn colr_v0_layers_for_glyph(glyph: u16, palettes: &[u16]) -> Vec<u8> {
    let base_offset = 14_u32;
    let layer_offset = 20_u32;
    let mut table = Vec::new();
    table.extend_from_slice(&0_u16.to_be_bytes());
    table.extend_from_slice(&1_u16.to_be_bytes());
    table.extend_from_slice(&base_offset.to_be_bytes());
    table.extend_from_slice(&layer_offset.to_be_bytes());
    table.extend_from_slice(&u16::try_from(palettes.len()).unwrap().to_be_bytes());
    table.extend_from_slice(&glyph.to_be_bytes());
    table.extend_from_slice(&0_u16.to_be_bytes());
    table.extend_from_slice(&u16::try_from(palettes.len()).unwrap().to_be_bytes());
    for palette in palettes {
        table.extend_from_slice(&glyph.to_be_bytes());
        table.extend_from_slice(&palette.to_be_bytes());
    }
    table
}

pub(super) fn colr_v1() -> Vec<u8> {
    colr_v1_for_glyph(1)
}

pub(super) fn colr_v1_for_glyph(glyph: u16) -> Vec<u8> {
    let mut paint = vec![10, 0, 0, 6];
    paint.extend_from_slice(&glyph.to_be_bytes());
    paint.extend_from_slice(&[2, 0, 0, 0x40, 0]);
    colr_v1_with_paint(paint, glyph)
}

pub(super) fn colr_v1_composite(mode: u8) -> Vec<u8> {
    let mut paint = vec![32, 0, 0, 8, mode, 0, 0, 13];
    paint.extend_from_slice(&[2, 0, 0, 0x40, 0]);
    paint.extend_from_slice(&[2, 0, 0, 0x40, 0]);
    colr_v1_with_paint(clip_to_glyph(paint, 1), 1)
}

pub(super) fn colr_v1_composite_palettes(
    glyph: u16,
    mode: u8,
    source: u16,
    backdrop: u16,
) -> Vec<u8> {
    let mut paint = vec![32, 0, 0, 8, mode, 0, 0, 13];
    paint.push(2);
    paint.extend_from_slice(&source.to_be_bytes());
    paint.extend_from_slice(&0x2000_u16.to_be_bytes());
    paint.push(2);
    paint.extend_from_slice(&backdrop.to_be_bytes());
    paint.extend_from_slice(&0x2000_u16.to_be_bytes());
    colr_v1_with_paint(clip_to_glyph(paint, glyph), glyph)
}

pub(super) fn colr_v1_gradient(extend: u8) -> Vec<u8> {
    let mut paint = vec![4, 0, 0, 16];
    paint.extend_from_slice(&[0; 12]);
    paint.extend_from_slice(&[extend, 0, 1, 0, 0, 0, 0, 0x40, 0]);
    colr_v1_with_paint(clip_to_glyph(paint, 1), 1)
}

pub(super) fn colr_v1_gradient_palettes(glyph: u16, first: u16, second: u16) -> Vec<u8> {
    let mut paint = vec![4, 0, 0, 16];
    for coordinate in [0_i16, 0, 1_000, 0, 0, 1_000] {
        paint.extend_from_slice(&coordinate.to_be_bytes());
    }
    paint.extend_from_slice(&[0, 0, 2]);
    for (offset, palette) in [(0_u16, first), (0x4000, second)] {
        paint.extend_from_slice(&offset.to_be_bytes());
        paint.extend_from_slice(&palette.to_be_bytes());
        paint.extend_from_slice(&0x4000_u16.to_be_bytes());
    }
    colr_v1_with_paint(clip_to_glyph(paint, glyph), glyph)
}

pub(super) fn clip_to_glyph(paint: Vec<u8>, glyph: u16) -> Vec<u8> {
    let mut clipped = vec![10, 0, 0, 6];
    clipped.extend_from_slice(&glyph.to_be_bytes());
    clipped.extend_from_slice(&paint);
    clipped
}

pub(super) fn colr_v1_with_paint(paint: Vec<u8>, glyph: u16) -> Vec<u8> {
    let mut table = Vec::new();
    table.extend_from_slice(&1_u16.to_be_bytes());
    table.extend_from_slice(&0_u16.to_be_bytes());
    table.extend_from_slice(&0_u32.to_be_bytes());
    table.extend_from_slice(&0_u32.to_be_bytes());
    table.extend_from_slice(&0_u16.to_be_bytes());
    table.extend_from_slice(&34_u32.to_be_bytes());
    table.extend_from_slice(&[0; 16]);
    table.extend_from_slice(&1_u32.to_be_bytes());
    table.extend_from_slice(&glyph.to_be_bytes());
    table.extend_from_slice(&10_u32.to_be_bytes());
    table.extend_from_slice(&paint);
    table
}

pub(super) fn cpal() -> Vec<u8> {
    vec![0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 14, 0, 0, 0, 0, 0, 255]
}

pub(super) fn cpal_colors(colors: &[[u8; 4]]) -> Vec<u8> {
    let count = u16::try_from(colors.len()).unwrap();
    let mut table = Vec::new();
    table.extend_from_slice(&0_u16.to_be_bytes());
    table.extend_from_slice(&count.to_be_bytes());
    table.extend_from_slice(&1_u16.to_be_bytes());
    table.extend_from_slice(&count.to_be_bytes());
    table.extend_from_slice(&14_u32.to_be_bytes());
    table.extend_from_slice(&0_u16.to_be_bytes());
    for [red, green, blue, alpha] in colors {
        table.extend_from_slice(&[*blue, *green, *red, *alpha]);
    }
    table
}

pub(super) fn sbix(glyph_count: u16) -> Vec<u8> {
    sbix_for_glyphs(glyph_count, 1, Some(2))
}

pub(super) fn sbix_for_glyphs(
    glyph_count: u16,
    direct_glyph: u16,
    duplicate_glyph: Option<u16>,
) -> Vec<u8> {
    let png = colored_png([255, 0, 0, 255]);
    let header = 4 + (usize::from(glyph_count) + 1) * 4;
    let mut strike = vec![0, 16, 0, 72];
    let mut data = Vec::new();
    for glyph in 0..glyph_count {
        strike.extend_from_slice(&u32::try_from(header + data.len()).unwrap().to_be_bytes());
        if glyph == direct_glyph {
            data.extend_from_slice(&[0, 0, 0, 0]);
            data.extend_from_slice(b"png ");
            data.extend_from_slice(&png);
        } else if duplicate_glyph == Some(glyph) {
            data.extend_from_slice(&[0, 0, 0, 0]);
            data.extend_from_slice(b"dupe");
            data.extend_from_slice(&direct_glyph.to_be_bytes());
        }
    }
    strike.extend_from_slice(&u32::try_from(header + data.len()).unwrap().to_be_bytes());
    strike.extend_from_slice(&data);
    let mut table = Vec::from([0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 12]);
    table.extend_from_slice(&strike);
    table
}

pub(super) fn sbix_strikes_for_glyph(
    glyph_count: u16,
    glyph: u16,
    strikes: &[(u16, [u8; 4])],
) -> Vec<u8> {
    let mut strike_tables = Vec::new();
    for (ppem, color) in strikes {
        let png = colored_png(*color);
        let header = 4 + (usize::from(glyph_count) + 1) * 4;
        let mut strike = Vec::new();
        strike.extend_from_slice(&ppem.to_be_bytes());
        strike.extend_from_slice(&72_u16.to_be_bytes());
        let mut data = Vec::new();
        for candidate in 0..glyph_count {
            strike.extend_from_slice(&u32::try_from(header + data.len()).unwrap().to_be_bytes());
            if candidate == glyph {
                data.extend_from_slice(&[0, 0, 0, 0]);
                data.extend_from_slice(b"png ");
                data.extend_from_slice(&png);
            }
        }
        strike.extend_from_slice(&u32::try_from(header + data.len()).unwrap().to_be_bytes());
        strike.extend_from_slice(&data);
        strike_tables.push(strike);
    }
    let header = 8 + strikes.len() * 4;
    let mut table = Vec::new();
    table.extend_from_slice(&1_u16.to_be_bytes());
    table.extend_from_slice(&1_u16.to_be_bytes());
    table.extend_from_slice(&u32::try_from(strikes.len()).unwrap().to_be_bytes());
    let mut offset = header;
    for strike in &strike_tables {
        table.extend_from_slice(&u32::try_from(offset).unwrap().to_be_bytes());
        offset += strike.len();
    }
    for strike in strike_tables {
        table.extend_from_slice(&strike);
    }
    table
}

pub(super) fn colored_png(color: [u8; 4]) -> Vec<u8> {
    let mut bytes = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut bytes, 1, 1);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&color).unwrap();
    }
    bytes
}

pub(super) fn with_tables(bytes: &[u8], replacements: &[(&[u8; 4], Vec<u8>)]) -> Arc<[u8]> {
    let count = usize::from(be_u16(bytes, 4));
    let mut tables = (0..count)
        .map(|index| {
            let record = 12 + index * 16;
            let tag: [u8; 4] = bytes[record..record + 4].try_into().unwrap();
            (tag, bytes[table_range(bytes, tag)].to_vec())
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    for (tag, data) in replacements {
        tables.insert(**tag, data.clone());
    }
    rebuild_font(bytes, tables)
}

fn rebuild_font(bytes: &[u8], tables: std::collections::BTreeMap<[u8; 4], Vec<u8>>) -> Arc<[u8]> {
    let count = u16::try_from(tables.len()).unwrap();
    let power = 1_u16 << (15 - count.leading_zeros() as u16);
    let mut output = Vec::new();
    output.extend_from_slice(&bytes[..4]);
    output.extend_from_slice(&count.to_be_bytes());
    output.extend_from_slice(&(power * 16).to_be_bytes());
    output.extend_from_slice(&(power.trailing_zeros() as u16).to_be_bytes());
    output.extend_from_slice(&(count * 16 - power * 16).to_be_bytes());
    let directory = output.len();
    output.resize(directory + tables.len() * 16, 0);
    for (index, (tag, data)) in tables.into_iter().enumerate() {
        while output.len() % 4 != 0 {
            output.push(0);
        }
        let offset = output.len();
        output.extend_from_slice(&data);
        let record = directory + index * 16;
        output[record..record + 4].copy_from_slice(&tag);
        output[record + 4..record + 8]
            .copy_from_slice(&sfnt_table_checksum(tag, &data).to_be_bytes());
        output[record + 8..record + 12]
            .copy_from_slice(&u32::try_from(offset).unwrap().to_be_bytes());
        output[record + 12..record + 16]
            .copy_from_slice(&u32::try_from(data.len()).unwrap().to_be_bytes());
    }
    Arc::from(output)
}

fn refresh_named_table_checksum(bytes: &mut [u8], tag: [u8; 4]) {
    let record = table_record(bytes, tag);
    let range = table_range(bytes, tag);
    let checksum = table_checksum(&bytes[range]);
    bytes[record + 4..record + 8].copy_from_slice(&checksum.to_be_bytes());
}

fn table_record(bytes: &[u8], tag: [u8; 4]) -> usize {
    let count = usize::from(be_u16(bytes, 4));
    (0..count)
        .map(|index| 12 + index * 16)
        .find(|start| bytes[*start..*start + 4] == tag)
        .expect("fixture owns required table")
}

fn table_range(bytes: &[u8], tag: [u8; 4]) -> std::ops::Range<usize> {
    let record = table_record(bytes, tag);
    let start = usize::try_from(be_u32(bytes, record + 8)).unwrap();
    let length = usize::try_from(be_u32(bytes, record + 12)).unwrap();
    start..start + length
}

fn table_checksum(bytes: &[u8]) -> u32 {
    bytes.chunks(4).fold(0, |sum, chunk| {
        let mut word = [0; 4];
        word[..chunk.len()].copy_from_slice(chunk);
        sum.wrapping_add(u32::from_be_bytes(word))
    })
}

fn sfnt_table_checksum(tag: [u8; 4], data: &[u8]) -> u32 {
    if tag != *b"head" {
        return table_checksum(data);
    }
    let mut head = data.to_vec();
    head[8..12].fill(0);
    table_checksum(&head)
}

fn be_u16(bytes: &[u8], start: usize) -> u16 {
    u16::from_be_bytes(bytes[start..start + 2].try_into().unwrap())
}

fn be_u32(bytes: &[u8], start: usize) -> u32 {
    u32::from_be_bytes(bytes[start..start + 4].try_into().unwrap())
}
