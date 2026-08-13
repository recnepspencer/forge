use harfrust::{FontRef, Tag};
use worth_ui_host_contract::UiFontSlant;

use super::{
    color_tables, container, metadata_inventory, metadata_inventory::UiApplicationFaceMetadata,
    UiApplicationFontFaceDefinition,
};
use crate::font_collection::{face::axis_range, UiFontCollectionAdmissionDenial};

pub(super) fn validate_face_definition(
    face: &UiApplicationFontFaceDefinition,
) -> Result<UiApplicationFaceMetadata, UiFontCollectionAdmissionDenial> {
    use UiFontCollectionAdmissionDenial as Denial;
    if face.family.is_empty()
        || face.family.len() > 128
        || face.weight == 0
        || face.weight > 1_000
        || !(50_000..=200_000).contains(&face.width_milli_percent)
        || face.license.identifier.is_empty()
        || face.license.notice.is_empty()
    {
        return Err(Denial::MalformedPackDefinition);
    }
    container::validate(&face.bytes, face.face_index)?;
    let font =
        FontRef::from_index(&face.bytes, face.face_index).map_err(|_| Denial::MalformedFont)?;
    if [*b"morx", *b"mort"]
        .into_iter()
        .any(|tag| font.table_data(Tag::from_be_bytes(tag)).is_some())
    {
        return Err(Denial::UnsupportedShapingTable);
    }
    let intrinsic_color = !color_tables::validate(&font)?.is_empty();
    validate_face_metadata(&font, face)?;
    metadata_inventory::derive(&font, intrinsic_color)
}

fn validate_face_metadata(
    font: &FontRef<'_>,
    definition: &UiApplicationFontFaceDefinition,
) -> Result<(), UiFontCollectionAdmissionDenial> {
    use UiFontCollectionAdmissionDenial as Denial;
    let os2 = font
        .table_data(Tag::from_be_bytes(*b"OS/2"))
        .ok_or(Denial::FaceMetadataMismatch)?;
    let os2 = os2.as_bytes();
    let observed_weight = be_u16(os2, 4).ok_or(Denial::FaceMetadataMismatch)?;
    let observed_width =
        width_class_milli_percent(be_u16(os2, 6).ok_or(Denial::FaceMetadataMismatch)?)
            .ok_or(Denial::FaceMetadataMismatch)?;
    let weight_matches = axis_range(font, *b"wght").map_or(
        observed_weight == definition.weight,
        |(minimum, maximum)| {
            let weight = f32::from(definition.weight);
            weight >= minimum && weight <= maximum
        },
    );
    let width_matches = axis_range(font, *b"wdth").map_or(
        observed_width == definition.width_milli_percent,
        |(minimum, maximum)| {
            let width = definition.width_milli_percent as f32 / 1_000.0;
            width >= minimum && width <= maximum
        },
    );
    let selection = be_u16(os2, 62).unwrap_or(0);
    let observed_slant = if selection & (1 << 9) != 0 {
        UiFontSlant::Oblique
    } else if selection & 1 != 0 {
        UiFontSlant::Italic
    } else {
        UiFontSlant::Upright
    };
    let slant_matches = match definition.slant {
        UiFontSlant::Upright => {
            observed_slant == UiFontSlant::Upright
                || axis_range(font, *b"ital").is_some_and(|(min, max)| min <= 0.0 && max >= 0.0)
                || axis_range(font, *b"slnt").is_some_and(|(min, max)| min <= 0.0 && max >= 0.0)
        }
        UiFontSlant::Italic => {
            observed_slant == UiFontSlant::Italic
                || axis_range(font, *b"ital").is_some_and(|(min, max)| min <= 1.0 && max >= 1.0)
        }
        UiFontSlant::Oblique => {
            observed_slant == UiFontSlant::Oblique
                || axis_range(font, *b"slnt").is_some_and(|(min, max)| min < 0.0 || max > 0.0)
        }
    };
    if weight_matches && width_matches && slant_matches {
        Ok(())
    } else {
        Err(Denial::FaceMetadataMismatch)
    }
}

fn be_u16(bytes: &[u8], start: usize) -> Option<u16> {
    Some(u16::from_be_bytes(
        bytes.get(start..start + 2)?.try_into().ok()?,
    ))
}

fn width_class_milli_percent(class: u16) -> Option<u32> {
    Some(match class {
        1 => 50_000,
        2 => 62_500,
        3 => 75_000,
        4 => 87_500,
        5 => 100_000,
        6 => 112_500,
        7 => 125_000,
        8 => 150_000,
        9 => 200_000,
        _ => return None,
    })
}
