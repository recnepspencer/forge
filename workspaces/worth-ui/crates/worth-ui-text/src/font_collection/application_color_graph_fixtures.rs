//! Checksum-valid CBDT graph fixtures for production-boundary raster controls.

use std::sync::Arc;

use super::application_color_fixtures::{
    clip_to_glyph, colored_png, colr_v1_with_paint, maxp_glyph_count, with_tables,
};

pub(super) fn colr_v1_clipped_solid(glyph: u16, palette: u16) -> Vec<u8> {
    let mut paint = vec![2];
    paint.extend_from_slice(&palette.to_be_bytes());
    paint.extend_from_slice(&0x4000_u16.to_be_bytes());
    colr_v1_with_paint(clip_to_glyph(paint, glyph), glyph)
}

pub(super) fn colr_v1_unbounded_gradient(glyph: u16) -> Vec<u8> {
    let mut paint = vec![4, 0, 0, 16];
    paint.extend_from_slice(&[0; 12]);
    paint.extend_from_slice(&[0, 0, 1, 0, 0, 0, 0, 0x40, 0]);
    colr_v1_with_paint(paint, glyph)
}

pub(super) fn colr_v1_bounded_then_unbounded_solid(first: u16, second: u16) -> Vec<u8> {
    let bounded = clip_to_glyph(vec![2, 0, 0, 0x40, 0], first);
    let unbounded = vec![2, 0, 0, 0x40, 0];
    colr_v1_two_roots(first, second, bounded, unbounded)
}

pub(super) fn colr_v1_two_bounded_solids(first: u16, second: u16) -> Vec<u8> {
    let first_paint = clip_to_glyph(vec![2, 0, 0, 0x40, 0], first);
    let second_paint = clip_to_glyph(vec![2, 0, 0, 0x40, 0], second);
    colr_v1_two_roots(first, second, first_paint, second_paint)
}

fn colr_v1_two_roots(
    first: u16,
    second: u16,
    first_paint: Vec<u8>,
    second_paint: Vec<u8>,
) -> Vec<u8> {
    let mut table = colr_v1_header();
    table.extend_from_slice(&2_u32.to_be_bytes());
    table.extend_from_slice(&first.to_be_bytes());
    table.extend_from_slice(&16_u32.to_be_bytes());
    table.extend_from_slice(&second.to_be_bytes());
    table.extend_from_slice(&(16_u32 + first_paint.len() as u32).to_be_bytes());
    table.extend_from_slice(&first_paint);
    table.extend_from_slice(&second_paint);
    table
}

fn colr_v1_header() -> Vec<u8> {
    let mut table = Vec::new();
    table.extend_from_slice(&1_u16.to_be_bytes());
    table.extend_from_slice(&0_u16.to_be_bytes());
    table.extend_from_slice(&0_u32.to_be_bytes());
    table.extend_from_slice(&0_u32.to_be_bytes());
    table.extend_from_slice(&0_u16.to_be_bytes());
    table.extend_from_slice(&34_u32.to_be_bytes());
    table.extend_from_slice(&[0; 16]);
    table
}

#[derive(Clone, Copy)]
pub(super) enum CbdtCompositeTarget {
    Child,
    Missing,
    Cycle,
}

pub(super) fn cbdt_composite_font(
    outline: &[u8],
    parent: u16,
    target: CbdtCompositeTarget,
) -> Arc<[u8]> {
    let glyph_count = maxp_glyph_count(outline);
    let child = if parent + 1 < glyph_count {
        parent + 1
    } else {
        parent - 1
    };
    let target = match target {
        CbdtCompositeTarget::Child => child,
        CbdtCompositeTarget::Missing => {
            if child + 1 < glyph_count {
                child + 1
            } else {
                child - 1
            }
        }
        CbdtCompositeTarget::Cycle => parent,
    };
    let mut images = vec![
        BitmapImage::png(child, colored_png([255, 0, 0, 192])),
        BitmapImage::composite(parent, target),
    ];
    images.sort_by_key(|image| image.glyph);
    let (cblc, cbdt) = bitmap_tables(&images);
    with_tables(outline, &[(b"CBLC", cblc), (b"CBDT", cbdt)])
}

pub(super) fn unsupported_sbix_font(
    outline: &[u8],
    glyph: u16,
    graphic_type: [u8; 4],
) -> Arc<[u8]> {
    let glyph_count = maxp_glyph_count(outline);
    let header = 4 + (usize::from(glyph_count) + 1) * 4;
    let mut strike = Vec::from([0, 16, 0, 72]);
    let mut data = Vec::new();
    for candidate in 0..glyph_count {
        strike.extend_from_slice(&u32::try_from(header + data.len()).unwrap().to_be_bytes());
        if candidate == glyph {
            data.extend_from_slice(&[0, 0, 0, 0]);
            data.extend_from_slice(&graphic_type);
            data.extend_from_slice(b"unsupported-image");
        }
    }
    strike.extend_from_slice(&u32::try_from(header + data.len()).unwrap().to_be_bytes());
    strike.extend_from_slice(&data);
    let mut sbix = Vec::from([0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 12]);
    sbix.extend_from_slice(&strike);
    with_tables(outline, &[(b"sbix", sbix)])
}

struct BitmapImage {
    glyph: u16,
    format: u16,
    data: Vec<u8>,
}

impl BitmapImage {
    fn png(glyph: u16, png: Vec<u8>) -> Self {
        let mut data = Vec::from([1, 1, 0, 1, 1]);
        data.extend_from_slice(&u32::try_from(png.len()).unwrap().to_be_bytes());
        data.extend_from_slice(&png);
        Self {
            glyph,
            format: 17,
            data,
        }
    }

    fn composite(glyph: u16, target: u16) -> Self {
        let mut data = Vec::from([2, 2, 0, 2, 2, 0]);
        data.extend_from_slice(&1_u16.to_be_bytes());
        data.extend_from_slice(&target.to_be_bytes());
        data.extend_from_slice(&[0, 0]);
        Self {
            glyph,
            format: 8,
            data,
        }
    }
}

fn bitmap_tables(images: &[BitmapImage]) -> (Vec<u8>, Vec<u8>) {
    let mut cbdt = Vec::from([0, 3, 0, 0]);
    let mut image_offsets = Vec::new();
    for image in images {
        image_offsets.push(u32::try_from(cbdt.len()).unwrap());
        cbdt.extend_from_slice(&image.data);
    }

    let array_offset = 56_u32;
    let array_bytes = u32::try_from(images.len() * 8).unwrap();
    let subtable_bytes = u32::try_from(images.len() * 16).unwrap();
    let mut cblc = Vec::from([0, 3, 0, 0, 0, 0, 0, 1]);
    cblc.extend_from_slice(&array_offset.to_be_bytes());
    cblc.extend_from_slice(&(array_bytes + subtable_bytes).to_be_bytes());
    cblc.extend_from_slice(&u32::try_from(images.len()).unwrap().to_be_bytes());
    cblc.extend_from_slice(&0_u32.to_be_bytes());
    cblc.extend_from_slice(&[0; 24]);
    cblc.extend_from_slice(&images.first().unwrap().glyph.to_be_bytes());
    cblc.extend_from_slice(&images.last().unwrap().glyph.to_be_bytes());
    cblc.extend_from_slice(&[16, 16, 32, 1]);
    for (index, image) in images.iter().enumerate() {
        cblc.extend_from_slice(&image.glyph.to_be_bytes());
        cblc.extend_from_slice(&image.glyph.to_be_bytes());
        let offset = array_bytes + u32::try_from(index * 16).unwrap();
        cblc.extend_from_slice(&offset.to_be_bytes());
    }
    for (image, image_offset) in images.iter().zip(image_offsets) {
        cblc.extend_from_slice(&1_u16.to_be_bytes());
        cblc.extend_from_slice(&image.format.to_be_bytes());
        cblc.extend_from_slice(&image_offset.to_be_bytes());
        cblc.extend_from_slice(&0_u32.to_be_bytes());
        cblc.extend_from_slice(&u32::try_from(image.data.len()).unwrap().to_be_bytes());
    }
    (cblc, cbdt)
}
