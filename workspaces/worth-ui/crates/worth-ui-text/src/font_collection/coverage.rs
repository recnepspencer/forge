use harfrust::{FontRef, Tag};

#[derive(Clone)]
pub(super) struct UiFontCoverageIndex {
    ranges: Box<[(u32, u32)]>,
}

impl UiFontCoverageIndex {
    pub(super) fn from_font(font: &FontRef<'_>) -> Option<Self> {
        let cmap = font.table_data(Tag::from_be_bytes(*b"cmap"))?;
        let bytes = cmap.as_bytes();
        let record_count = usize::from(be_u16(bytes, 2)?);
        let mut ranges = Vec::new();
        for record_index in 0..record_count {
            let record = 4usize.checked_add(record_index.checked_mul(8)?)?;
            let platform = be_u16(bytes, record)?;
            let encoding = be_u16(bytes, record + 2)?;
            if platform != 0 && !(platform == 3 && matches!(encoding, 1 | 10)) {
                continue;
            }
            let offset = usize::try_from(be_u32(bytes, record + 4)?).ok()?;
            let subtable = bytes.get(offset..)?;
            match be_u16(subtable, 0)? {
                0 => append_format_zero(subtable, &mut ranges)?,
                4 => append_format_four(subtable, &mut ranges)?,
                6 => append_format_six(subtable, &mut ranges)?,
                10 => append_format_ten(subtable, &mut ranges)?,
                12 => append_format_twelve(subtable, &mut ranges, false)?,
                13 => append_format_twelve(subtable, &mut ranges, true)?,
                14 => append_format_fourteen(subtable, &mut ranges)?,
                _ => {}
            }
        }
        normalize(&mut ranges);
        (!ranges.is_empty()).then(|| Self {
            ranges: ranges.into_boxed_slice(),
        })
    }

    pub(super) fn contains_cluster(&self, text: &str) -> bool {
        text.chars()
            .map(u32::from)
            .filter(|scalar| !is_default_ignorable(*scalar))
            .all(|scalar| self.contains(scalar))
    }

    pub(super) fn range_count(&self) -> usize {
        self.ranges.len()
    }

    fn contains(&self, scalar: u32) -> bool {
        self.ranges
            .binary_search_by(|(start, end)| {
                if scalar < *start {
                    std::cmp::Ordering::Greater
                } else if scalar > *end {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Equal
                }
            })
            .is_ok()
    }
}

fn append_format_zero(bytes: &[u8], ranges: &mut Vec<(u32, u32)>) -> Option<()> {
    let length = usize::from(be_u16(bytes, 2)?);
    let glyphs = bytes.get(6..length)?;
    let mut open = None;
    for (scalar, glyph) in glyphs.iter().copied().enumerate() {
        append_scalar(ranges, &mut open, u32::try_from(scalar).ok()?, glyph != 0);
    }
    close_range(ranges, &mut open);
    Some(())
}

fn append_format_six(bytes: &[u8], ranges: &mut Vec<(u32, u32)>) -> Option<()> {
    let length = usize::from(be_u16(bytes, 2)?);
    let bytes = bytes.get(..length)?;
    let first = u32::from(be_u16(bytes, 6)?);
    let count = usize::from(be_u16(bytes, 8)?);
    let mut open = None;
    for index in 0..count {
        let scalar = first.checked_add(u32::try_from(index).ok()?)?;
        let glyph = be_u16(bytes, 10 + index * 2)?;
        append_scalar(ranges, &mut open, scalar, glyph != 0);
    }
    close_range(ranges, &mut open);
    Some(())
}

fn append_format_ten(bytes: &[u8], ranges: &mut Vec<(u32, u32)>) -> Option<()> {
    let length = usize::try_from(be_u32(bytes, 4)?).ok()?;
    let bytes = bytes.get(..length)?;
    let first = be_u32(bytes, 12)?;
    let count = usize::try_from(be_u32(bytes, 16)?).ok()?;
    let mut open = None;
    for index in 0..count {
        let scalar = first.checked_add(u32::try_from(index).ok()?)?;
        if scalar > 0x10FFFF {
            return None;
        }
        let glyph = be_u16(bytes, 20 + index * 2)?;
        append_scalar(ranges, &mut open, scalar, glyph != 0);
    }
    close_range(ranges, &mut open);
    Some(())
}

fn append_format_fourteen(bytes: &[u8], ranges: &mut Vec<(u32, u32)>) -> Option<()> {
    let length = usize::try_from(be_u32(bytes, 2)?).ok()?;
    let bytes = bytes.get(..length)?;
    let count = usize::try_from(be_u32(bytes, 6)?).ok()?;
    for index in 0..count {
        let record = 10usize.checked_add(index.checked_mul(11)?)?;
        let default_offset = usize::try_from(be_u32(bytes, record + 3)?).ok()?;
        let nondefault_offset = usize::try_from(be_u32(bytes, record + 7)?).ok()?;
        if default_offset != 0 {
            append_default_variations(bytes.get(default_offset..)?, ranges)?;
        }
        if nondefault_offset != 0 {
            append_nondefault_variations(bytes.get(nondefault_offset..)?, ranges)?;
        }
    }
    Some(())
}

fn append_default_variations(bytes: &[u8], ranges: &mut Vec<(u32, u32)>) -> Option<()> {
    let count = usize::try_from(be_u32(bytes, 0)?).ok()?;
    for index in 0..count {
        let record = 4usize.checked_add(index.checked_mul(4)?)?;
        let start = be_u24(bytes, record)?;
        let end = start.checked_add(u32::from(*bytes.get(record + 3)?))?;
        (end <= 0x10FFFF).then_some(())?;
        ranges.push((start, end));
    }
    Some(())
}

fn append_nondefault_variations(bytes: &[u8], ranges: &mut Vec<(u32, u32)>) -> Option<()> {
    let count = usize::try_from(be_u32(bytes, 0)?).ok()?;
    for index in 0..count {
        let record = 4usize.checked_add(index.checked_mul(5)?)?;
        let scalar = be_u24(bytes, record)?;
        let glyph = be_u16(bytes, record + 3)?;
        if scalar > 0x10FFFF {
            return None;
        }
        if glyph != 0 {
            ranges.push((scalar, scalar));
        }
    }
    Some(())
}

fn append_format_four(bytes: &[u8], ranges: &mut Vec<(u32, u32)>) -> Option<()> {
    let length = usize::from(be_u16(bytes, 2)?);
    let bytes = bytes.get(..length)?;
    let segment_count = usize::from(be_u16(bytes, 6)?) / 2;
    let ends = 14usize;
    let starts = ends
        .checked_add(segment_count.checked_mul(2)?)?
        .checked_add(2)?;
    let deltas = starts.checked_add(segment_count.checked_mul(2)?)?;
    let offsets = deltas.checked_add(segment_count.checked_mul(2)?)?;
    for index in 0..segment_count {
        let end = u32::from(be_u16(bytes, ends + index * 2)?);
        let start = u32::from(be_u16(bytes, starts + index * 2)?);
        let delta = u32::from(be_u16(bytes, deltas + index * 2)?);
        let range_offset_position = offsets + index * 2;
        let range_offset = usize::from(be_u16(bytes, range_offset_position)?);
        if start > end {
            return None;
        }
        let mut open = None;
        for scalar in start..=end.min(0xFFFE) {
            let glyph = if range_offset == 0 {
                (scalar + delta) & 0xFFFF
            } else {
                let glyph_offset = range_offset_position
                    .checked_add(range_offset)?
                    .checked_add(usize::try_from((scalar - start) * 2).ok()?)?;
                let base = u32::from(be_u16(bytes, glyph_offset)?);
                if base == 0 {
                    0
                } else {
                    (base + delta) & 0xFFFF
                }
            };
            append_scalar(ranges, &mut open, scalar, glyph != 0);
        }
        close_range(ranges, &mut open);
    }
    Some(())
}

fn append_format_twelve(
    bytes: &[u8],
    ranges: &mut Vec<(u32, u32)>,
    constant_glyph: bool,
) -> Option<()> {
    let length = usize::try_from(be_u32(bytes, 4)?).ok()?;
    let bytes = bytes.get(..length)?;
    let group_count = usize::try_from(be_u32(bytes, 12)?).ok()?;
    for index in 0..group_count {
        let record = 16usize.checked_add(index.checked_mul(12)?)?;
        let start = be_u32(bytes, record)?;
        let end = be_u32(bytes, record + 4)?;
        let glyph = be_u32(bytes, record + 8)?;
        if start > end || end > 0x10FFFF {
            return None;
        }
        if constant_glyph {
            if glyph != 0 {
                ranges.push((start, end));
            }
        } else if glyph == 0 {
            if start < end {
                ranges.push((start + 1, end));
            }
        } else {
            ranges.push((start, end));
        }
    }
    Some(())
}

fn append_scalar(
    ranges: &mut Vec<(u32, u32)>,
    open: &mut Option<(u32, u32)>,
    scalar: u32,
    covered: bool,
) {
    match (*open, covered) {
        (Some((start, end)), true) if scalar == end + 1 => *open = Some((start, scalar)),
        (Some(range), true) => {
            ranges.push(range);
            *open = Some((scalar, scalar));
        }
        (None, true) => *open = Some((scalar, scalar)),
        (Some(range), false) => {
            ranges.push(range);
            *open = None;
        }
        (None, false) => {}
    }
}

fn close_range(ranges: &mut Vec<(u32, u32)>, open: &mut Option<(u32, u32)>) {
    if let Some(range) = open.take() {
        ranges.push(range);
    }
}

fn normalize(ranges: &mut Vec<(u32, u32)>) {
    ranges.sort_unstable();
    let mut output: Vec<(u32, u32)> = Vec::with_capacity(ranges.len());
    for (start, end) in ranges.drain(..) {
        if let Some(last) = output
            .last_mut()
            .filter(|last| start <= last.1.saturating_add(1))
        {
            last.1 = last.1.max(end);
        } else {
            output.push((start, end));
        }
    }
    *ranges = output;
}

const fn is_default_ignorable(scalar: u32) -> bool {
    matches!(
        scalar,
        0x00AD
            | 0x034F
            | 0x061C
            | 0x115F..=0x1160
            | 0x17B4..=0x17B5
            | 0x180B..=0x180F
            | 0x200B..=0x200F
            | 0x202A..=0x202E
            | 0x2060..=0x206F
            | 0x3164
            | 0xFE00..=0xFE0F
            | 0xFEFF
            | 0xFFA0
            | 0xFFF0..=0xFFF8
            | 0x1BCA0..=0x1BCA3
            | 0x1D173..=0x1D17A
            | 0xE0000..=0xE0FFF
    )
}

fn be_u16(bytes: &[u8], start: usize) -> Option<u16> {
    Some(u16::from_be_bytes(
        bytes.get(start..start + 2)?.try_into().ok()?,
    ))
}

fn be_u32(bytes: &[u8], start: usize) -> Option<u32> {
    Some(u32::from_be_bytes(
        bytes.get(start..start + 4)?.try_into().ok()?,
    ))
}

fn be_u24(bytes: &[u8], start: usize) -> Option<u32> {
    let value = bytes.get(start..start + 3)?;
    Some((u32::from(value[0]) << 16) | (u32::from(value[1]) << 8) | u32::from(value[2]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_ten_and_nondefault_variation_coverage_are_indexed() {
        let mut format_ten = vec![0u8; 24];
        format_ten[0..2].copy_from_slice(&10u16.to_be_bytes());
        format_ten[4..8].copy_from_slice(&24u32.to_be_bytes());
        format_ten[12..16].copy_from_slice(&0x10000u32.to_be_bytes());
        format_ten[16..20].copy_from_slice(&2u32.to_be_bytes());
        format_ten[20..22].copy_from_slice(&7u16.to_be_bytes());
        let mut ranges = Vec::new();
        append_format_ten(&format_ten, &mut ranges).unwrap();
        normalize(&mut ranges);
        let index = UiFontCoverageIndex {
            ranges: ranges.into_boxed_slice(),
        };
        assert!(index.contains(0x10000));
        assert!(!index.contains(0x10001));

        let mut format_fourteen = vec![0u8; 30];
        format_fourteen[0..2].copy_from_slice(&14u16.to_be_bytes());
        format_fourteen[2..6].copy_from_slice(&30u32.to_be_bytes());
        format_fourteen[6..10].copy_from_slice(&1u32.to_be_bytes());
        format_fourteen[10..13].copy_from_slice(&[0x00, 0xFE, 0x0E]);
        format_fourteen[17..21].copy_from_slice(&21u32.to_be_bytes());
        format_fourteen[21..25].copy_from_slice(&1u32.to_be_bytes());
        format_fourteen[25..28].copy_from_slice(&[0x00, 0x27, 0x64]);
        format_fourteen[28..30].copy_from_slice(&9u16.to_be_bytes());
        let mut ranges = Vec::new();
        append_format_fourteen(&format_fourteen, &mut ranges).unwrap();
        normalize(&mut ranges);
        let index = UiFontCoverageIndex {
            ranges: ranges.into_boxed_slice(),
        };
        assert!(index.contains_cluster("\u{2764}\u{FE0E}"));
    }
}
