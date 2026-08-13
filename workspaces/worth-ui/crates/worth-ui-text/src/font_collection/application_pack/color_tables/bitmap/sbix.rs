use harfrust::FontRef;
use read_fonts::{types::GlyphId, TableProvider};
use std::collections::{BTreeMap, BTreeSet};

use crate::font_collection::UiFontCollectionAdmissionDenial;

use super::super::{malformed, png::validate_png, traversal::validate_table};

pub(in crate::font_collection) fn validate(
    font: &FontRef<'_>,
    glyph_count: u16,
) -> Result<Box<[u16]>, UiFontCollectionAdmissionDenial> {
    let sbix = font.sbix().map_err(|_| malformed())?;
    validate_sbix_table(&sbix, glyph_count)
}

fn validate_sbix_table(
    sbix: &read_fonts::tables::sbix::Sbix<'_>,
    glyph_count: u16,
) -> Result<Box<[u16]>, UiFontCollectionAdmissionDenial> {
    validate_table(sbix, 0, glyph_count, 0, None)?;
    let table_bytes = sbix.offset_data().as_bytes();
    let flags = u16::from_be_bytes(
        table_bytes
            .get(2..4)
            .ok_or_else(malformed)?
            .try_into()
            .unwrap(),
    );
    let strike_offsets: Vec<_> = sbix
        .strike_offsets()
        .iter()
        .map(|offset| offset.get().to_u32())
        .collect();
    let strike_directory_end = 8_u32
        .checked_add(sbix.num_strikes().checked_mul(4).ok_or_else(malformed)?)
        .ok_or_else(malformed)?;
    if sbix.version() != 1
        || flags & 1 == 0
        || flags & !0x0003 != 0
        || sbix.num_strikes() == 0
        || strike_offsets.len() != sbix.num_strikes() as usize
        || strike_offsets.first().copied().unwrap_or_default() < strike_directory_end
        || strike_offsets.windows(2).any(|pair| pair[0] >= pair[1])
        || strike_offsets
            .last()
            .is_none_or(|offset| *offset as usize >= table_bytes.len())
    {
        return Err(malformed());
    }
    let mut png_graphics = 0_u32;
    let mut covered = BTreeSet::new();
    for strike in sbix.strikes().iter() {
        let strike = strike.map_err(|_| malformed())?;
        validate_table(&strike, 0, glyph_count, 0, None)?;
        if strike.ppem() == 0
            || strike.ppi() == 0
            || strike
                .glyph_data_offsets()
                .windows(2)
                .any(|pair| pair[0] > pair[1])
        {
            return Err(malformed());
        }
        let mut graphics = BTreeMap::new();
        for glyph in 0..glyph_count {
            let Some(data) = strike
                .glyph_data(GlyphId::new(glyph.into()))
                .map_err(|_| malformed())?
            else {
                continue;
            };
            let graphic_type = data.graphic_type().to_be_bytes();
            if graphic_type == *b"png " {
                validate_png(data.data())?;
                graphics.insert(glyph, SbixGraphic::Png);
                png_graphics = png_graphics.checked_add(1).ok_or_else(malformed)?;
            } else if graphic_type == *b"dupe" && data.data().len() == 2 {
                let target = u16::from_be_bytes(data.data().try_into().unwrap());
                if target >= glyph_count {
                    return Err(malformed());
                }
                graphics.insert(glyph, SbixGraphic::Duplicate(target));
            } else {
                return Err(UiFontCollectionAdmissionDenial::UnsupportedColorFontTable);
            }
        }
        for graphic in graphics.values() {
            if let SbixGraphic::Duplicate(target) = graphic {
                if graphics.get(target) != Some(&SbixGraphic::Png) {
                    return Err(malformed());
                }
            }
        }
        covered.extend(graphics.keys().copied());
    }
    if png_graphics == 0 {
        Err(malformed())
    } else {
        Ok(covered.into_iter().collect())
    }
}
#[derive(Clone, Copy, Eq, PartialEq)]
enum SbixGraphic {
    Png,
    Duplicate(u16),
}

#[cfg(test)]
mod tests {
    use read_fonts::{tables::sbix::Sbix, FontData};

    use super::validate_sbix_table;

    #[test]
    fn sbix_duplicate_is_exactly_one_hop_to_a_valid_png() {
        let png = png();
        let valid = sbix(&[SbixGraphicData::Duplicate(1), SbixGraphicData::Png(&png)]);
        let valid = Sbix::read(FontData::new(&valid), 2).unwrap();
        validate_sbix_table(&valid, 2).unwrap();

        let cycle = sbix(&[SbixGraphicData::Duplicate(1), SbixGraphicData::Duplicate(0)]);
        let cycle = Sbix::read(FontData::new(&cycle), 2).unwrap();
        assert!(validate_sbix_table(&cycle, 2).is_err());

        let empty = sbix(&[SbixGraphicData::Empty, SbixGraphicData::Empty]);
        let empty = Sbix::read(FontData::new(&empty), 2).unwrap();
        assert!(validate_sbix_table(&empty, 2).is_err());

        let mut missing_required_flag = valid_bytes();
        missing_required_flag[3] = 0;
        let missing_required_flag = Sbix::read(FontData::new(&missing_required_flag), 2).unwrap();
        assert!(validate_sbix_table(&missing_required_flag, 2).is_err());

        let mut reserved_flag = valid_bytes();
        reserved_flag[3] = 5;
        let reserved_flag = Sbix::read(FontData::new(&reserved_flag), 2).unwrap();
        assert!(validate_sbix_table(&reserved_flag, 2).is_err());
    }

    enum SbixGraphicData<'a> {
        Empty,
        Png(&'a [u8]),
        Duplicate(u16),
    }

    fn sbix(graphics: &[SbixGraphicData<'_>]) -> Vec<u8> {
        let strike_header = 4 + (graphics.len() + 1) * 4;
        let mut strike = vec![0, 16, 0, 72];
        let mut data = Vec::new();
        for graphic in graphics {
            strike.extend_from_slice(
                &u32::try_from(strike_header + data.len())
                    .unwrap()
                    .to_be_bytes(),
            );
            match graphic {
                SbixGraphicData::Empty => {}
                SbixGraphicData::Png(png) => {
                    data.extend_from_slice(&[0, 0, 0, 0]);
                    data.extend_from_slice(b"png ");
                    data.extend_from_slice(png);
                }
                SbixGraphicData::Duplicate(target) => {
                    data.extend_from_slice(&[0, 0, 0, 0]);
                    data.extend_from_slice(b"dupe");
                    data.extend_from_slice(&target.to_be_bytes());
                }
            }
        }
        strike.extend_from_slice(
            &u32::try_from(strike_header + data.len())
                .unwrap()
                .to_be_bytes(),
        );
        strike.extend_from_slice(&data);
        let mut table = Vec::from([0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 12]);
        table.extend_from_slice(&strike);
        table
    }

    fn valid_bytes() -> Vec<u8> {
        let png = png();
        sbix(&[SbixGraphicData::Duplicate(1), SbixGraphicData::Png(&png)])
    }

    fn png() -> Vec<u8> {
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
}
