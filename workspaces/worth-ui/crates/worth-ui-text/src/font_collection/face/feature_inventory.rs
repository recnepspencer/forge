use harfrust::{FontRef, Tag};

use crate::font_collection::UiFontCollectionAdmissionDenial;

pub(in crate::font_collection) fn derive(
    font: &FontRef<'_>,
) -> Result<Box<[[u8; 4]]>, UiFontCollectionAdmissionDenial> {
    let mut tags = Vec::new();
    for table in [*b"GSUB", *b"GPOS"] {
        let Some(data) = font.table_data(Tag::from_be_bytes(table)) else {
            continue;
        };
        let bytes = data.as_bytes();
        let offset = usize::from(
            be_u16(bytes, 6).ok_or(UiFontCollectionAdmissionDenial::FaceMetadataMismatch)?,
        );
        let count = usize::from(
            be_u16(bytes, offset).ok_or(UiFontCollectionAdmissionDenial::FaceMetadataMismatch)?,
        );
        let end = offset
            .checked_add(2)
            .and_then(|value| value.checked_add(count.checked_mul(6)?))
            .ok_or(UiFontCollectionAdmissionDenial::FaceMetadataMismatch)?;
        if end > bytes.len() {
            return Err(UiFontCollectionAdmissionDenial::FaceMetadataMismatch);
        }
        for index in 0..count {
            let start = offset + 2 + index * 6;
            tags.push(
                bytes[start..start + 4]
                    .try_into()
                    .expect("validated feature record"),
            );
        }
    }
    tags.sort_unstable();
    tags.dedup();
    Ok(tags.into_boxed_slice())
}

fn be_u16(bytes: &[u8], start: usize) -> Option<u16> {
    Some(u16::from_be_bytes(
        bytes.get(start..start + 2)?.try_into().ok()?,
    ))
}
