use std::sync::Arc;

use worth_ui_host_contract::UiFontSlant;

use super::{
    application_selection_tests::selected_faces,
    application_test_world::{assert_pack_denial, face, profile_collection_and_sources},
    UiApplicationFontPackDefinition, UiFontCollectionAdmissionDenial,
};
use crate::{UiFontFamilyStack, UiTextFaceRequest};

#[test]
pub(super) fn owned_color_emoji_bytes_are_selected_as_one_complete_application_cluster() {
    let (profile, sources) = profile_collection_and_sources();
    let (collection, receipt, _) = profile
        .register_application_pack(
            worth_ui_host_contract::UiFontCollectionGeneration::new(2).unwrap(),
            UiApplicationFontPackDefinition {
                name: Arc::from("owned application color emoji"),
                faces: Box::new([face(
                    "Application Emoji",
                    Arc::clone(&sources["noto-color-emoji"]),
                    0,
                    UiFontSlant::Upright,
                )]),
            },
        )
        .unwrap();
    let family = receipt.family("Application Emoji").unwrap();
    let face = receipt.faces()[0].identity();
    assert!(receipt.faces()[0].has_intrinsic_color());

    let selected = selected_faces(
        &Arc::new(collection),
        UiFontFamilyStack::new(Box::new([family])).unwrap(),
        UiTextFaceRequest::regular(),
        "\u{1F469}\u{200D}\u{1F4BB}",
    );
    assert_eq!(&*selected, &[face]);
}

#[test]
pub(super) fn every_qualified_color_table_format_crosses_public_pack_admission() {
    let (mut collection, sources) = profile_collection_and_sources();
    let outline = &sources["noto-sans-roman"];
    let glyph_count = maxp_glyph_count(outline);
    let sources = [
        (
            "COLRv0",
            with_tables(outline, &[(b"COLR", colr_v0()), (b"CPAL", cpal())]),
        ),
        (
            "COLRv1",
            with_tables(outline, &[(b"COLR", colr_v1()), (b"CPAL", cpal())]),
        ),
        ("CBDT/CBLC", Arc::clone(&sources["noto-color-emoji"])),
        (
            "sbix",
            with_tables(outline, &[(b"sbix", sbix(glyph_count))]),
        ),
    ];
    for (index, (format, bytes)) in sources.into_iter().enumerate() {
        assert!(
            harfrust::FontRef::from_index(&bytes, 0).is_ok(),
            "{format} parser"
        );
        let generation = worth_ui_host_contract::UiFontCollectionGeneration::new(
            u64::try_from(index).unwrap() + 2,
        )
        .unwrap();
        let (successor, receipt, _) = collection
            .register_application_pack(
                generation,
                UiApplicationFontPackDefinition {
                    name: Arc::from(format),
                    faces: Box::new([face(
                        &format!("Application {format}"),
                        bytes,
                        0,
                        UiFontSlant::Upright,
                    )]),
                },
            )
            .unwrap_or_else(|denial| panic!("{format} public admission failed: {denial:?}"));
        assert!(receipt.faces()[0].has_intrinsic_color(), "{format}");
        collection = successor;
    }
}

#[test]
pub(super) fn qualified_colrv1_composite_and_gradient_enums_cross_public_admission() {
    let (mut collection, sources) = profile_collection_and_sources();
    let outline = &sources["noto-sans-roman"];
    for (index, table) in [colr_v1_composite(3), colr_v1_gradient(2)]
        .into_iter()
        .enumerate()
    {
        let generation =
            worth_ui_host_contract::UiFontCollectionGeneration::new(index as u64 + 2).unwrap();
        let (successor, receipt, _) = collection
            .register_application_pack(
                generation,
                UiApplicationFontPackDefinition {
                    name: Arc::from(format!("qualified COLRv1 enums {index}")),
                    faces: Box::new([face(
                        &format!("Qualified COLRv1 {index}"),
                        with_tables(outline, &[(b"COLR", table), (b"CPAL", cpal())]),
                        0,
                        UiFontSlant::Upright,
                    )]),
                },
            )
            .unwrap();
        assert!(receipt.faces()[0].has_intrinsic_color());
        collection = successor;
    }
}

#[test]
pub(super) fn unknown_colrv1_composite_and_gradient_enums_deny_before_publication() {
    let (profile, sources) = profile_collection_and_sources();
    let outline = &sources["noto-sans-roman"];
    for (name, table) in [
        ("unknown COLRv1 composite mode", colr_v1_composite(0xFF)),
        ("unknown COLRv1 gradient extend", colr_v1_gradient(0xFF)),
    ] {
        let bytes = with_tables(outline, &[(b"COLR", table), (b"CPAL", cpal())]);
        assert!(harfrust::FontRef::from_index(&bytes, 0).is_ok(), "{name}");
        assert_pack_denial(
            &profile,
            UiApplicationFontPackDefinition {
                name: Arc::from(name),
                faces: Box::new([face(name, bytes, 0, UiFontSlant::Upright)]),
            },
            UiFontCollectionAdmissionDenial::MalformedColorFontTables,
        );
    }
}

#[test]
pub(super) fn repository_color_emoji_requires_resolvable_locations_and_intact_png_chunks() {
    let (profile, sources) = profile_collection_and_sources();
    let emoji = Arc::clone(&sources["noto-color-emoji"]);
    for (name, bytes) in [
        ("corrupt CBLC location", corrupt_cblc_location(&emoji)),
        ("corrupt CBDT image", corrupt_cbdt_png(&emoji)),
    ] {
        assert!(harfrust::FontRef::from_index(&bytes, 0).is_ok());
        assert_pack_denial(
            &profile,
            UiApplicationFontPackDefinition {
                name: Arc::from(name),
                faces: Box::new([face("Application Emoji", bytes, 0, UiFontSlant::Upright)]),
            },
            UiFontCollectionAdmissionDenial::MalformedColorFontTables,
        );
    }
}

#[test]
pub(super) fn malformed_colr_and_sbix_tables_cannot_hide_behind_a_parseable_outline_font() {
    let (profile, sources) = profile_collection_and_sources();
    let outline = Arc::clone(&sources["noto-sans-roman"]);
    let malformed_colr = rename_tables(&outline, &[(*b"name", *b"COLR"), (*b"post", *b"CPAL")]);
    let malformed_sbix = rename_tables(&outline, &[(*b"name", *b"sbix")]);
    for (name, bytes) in [
        ("malformed COLR graph", malformed_colr),
        ("malformed sbix strikes", malformed_sbix),
    ] {
        assert!(harfrust::FontRef::from_index(&bytes, 0).is_ok());
        assert_pack_denial(
            &profile,
            UiApplicationFontPackDefinition {
                name: Arc::from(name),
                faces: Box::new([face("Application Color", bytes, 0, UiFontSlant::Upright)]),
            },
            UiFontCollectionAdmissionDenial::MalformedColorFontTables,
        );
    }
}

fn corrupt_cblc_location(bytes: &[u8]) -> Arc<[u8]> {
    let mut mutated = bytes.to_vec();
    let table = table_range(&mutated, *b"CBLC");
    assert!(table.len() >= 12, "fixture owns one BitmapSize record");
    mutated[table.start + 8..table.start + 12].copy_from_slice(&u32::MAX.to_be_bytes());
    refresh_named_table_checksum(&mut mutated, *b"CBLC");
    Arc::from(mutated)
}

fn corrupt_cbdt_png(bytes: &[u8]) -> Arc<[u8]> {
    let mut mutated = bytes.to_vec();
    let table = table_range(&mutated, *b"CBDT");
    let signature = mutated[table.clone()]
        .windows(8)
        .position(|window| window == b"\x89PNG\r\n\x1a\n")
        .expect("fixture owns PNG-backed CBDT data");
    let ihdr_payload = table.start + signature + 16;
    mutated[ihdr_payload] ^= 1;
    refresh_named_table_checksum(&mut mutated, *b"CBDT");
    Arc::from(mutated)
}

fn rename_tables(bytes: &[u8], replacements: &[([u8; 4], [u8; 4])]) -> Arc<[u8]> {
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
    bytes.chunks(4).fold(0u32, |sum, chunk| {
        let mut word = [0; 4];
        word[..chunk.len()].copy_from_slice(chunk);
        sum.wrapping_add(u32::from_be_bytes(word))
    })
}

fn be_u16(bytes: &[u8], start: usize) -> u16 {
    u16::from_be_bytes(bytes[start..start + 2].try_into().unwrap())
}

fn be_u32(bytes: &[u8], start: usize) -> u32 {
    u32::from_be_bytes(bytes[start..start + 4].try_into().unwrap())
}

fn maxp_glyph_count(bytes: &[u8]) -> u16 {
    let range = table_range(bytes, *b"maxp");
    be_u16(bytes, range.start + 4)
}

fn colr_v0() -> Vec<u8> {
    vec![
        0, 0, 0, 1, 0, 0, 0, 14, 0, 0, 0, 20, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0,
    ]
}

fn colr_v1() -> Vec<u8> {
    colr_v1_with_paint(vec![2, 0, 0, 0x40, 0])
}

fn colr_v1_composite(mode: u8) -> Vec<u8> {
    let mut paint = vec![32, 0, 0, 8, mode, 0, 0, 13];
    paint.extend_from_slice(&[2, 0, 0, 0x40, 0]);
    paint.extend_from_slice(&[2, 0, 0, 0x40, 0]);
    colr_v1_with_paint(paint)
}

fn colr_v1_gradient(extend: u8) -> Vec<u8> {
    let mut paint = vec![4, 0, 0, 16];
    paint.extend_from_slice(&[0; 12]);
    paint.extend_from_slice(&[extend, 0, 1, 0, 0, 0, 0, 0x40, 0]);
    colr_v1_with_paint(paint)
}

fn colr_v1_with_paint(paint: Vec<u8>) -> Vec<u8> {
    let mut table = Vec::new();
    table.extend_from_slice(&1_u16.to_be_bytes());
    table.extend_from_slice(&0_u16.to_be_bytes());
    table.extend_from_slice(&0_u32.to_be_bytes());
    table.extend_from_slice(&0_u32.to_be_bytes());
    table.extend_from_slice(&0_u16.to_be_bytes());
    table.extend_from_slice(&34_u32.to_be_bytes());
    table.extend_from_slice(&[0; 16]);
    table.extend_from_slice(&1_u32.to_be_bytes());
    table.extend_from_slice(&1_u16.to_be_bytes());
    table.extend_from_slice(&10_u32.to_be_bytes());
    table.extend_from_slice(&paint);
    table
}

fn cpal() -> Vec<u8> {
    vec![0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 14, 0, 0, 0, 0, 0, 255]
}

fn sbix(glyph_count: u16) -> Vec<u8> {
    let png = transparent_png();
    let header = 4 + (usize::from(glyph_count) + 1) * 4;
    let mut strike = vec![0, 16, 0, 72];
    let mut data = Vec::new();
    for glyph in 0..glyph_count {
        strike.extend_from_slice(&u32::try_from(header + data.len()).unwrap().to_be_bytes());
        if glyph == 1 {
            data.extend_from_slice(&[0, 0, 0, 0]);
            data.extend_from_slice(b"png ");
            data.extend_from_slice(&png);
        } else if glyph == 2 {
            data.extend_from_slice(&[0, 0, 0, 0]);
            data.extend_from_slice(b"dupe");
            data.extend_from_slice(&1_u16.to_be_bytes());
        }
    }
    strike.extend_from_slice(&u32::try_from(header + data.len()).unwrap().to_be_bytes());
    strike.extend_from_slice(&data);
    let mut table = Vec::from([0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 12]);
    table.extend_from_slice(&strike);
    table
}

fn transparent_png() -> Vec<u8> {
    let mut bytes = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut bytes, 1, 1);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&[0, 0, 0, 0]).unwrap();
    }
    bytes
}

fn with_tables(bytes: &[u8], replacements: &[(&[u8; 4], Vec<u8>)]) -> Arc<[u8]> {
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

fn sfnt_table_checksum(tag: [u8; 4], data: &[u8]) -> u32 {
    if tag != *b"head" {
        return table_checksum(data);
    }
    let mut head = data.to_vec();
    head[8..12].fill(0);
    table_checksum(&head)
}
