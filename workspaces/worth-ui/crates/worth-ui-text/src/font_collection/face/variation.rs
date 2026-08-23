use harfrust::{Feature, FontRef, Tag, Variation};
use worth_ui_host_contract::{
    UiFontSlant, UiQualifiedTextStyleRecord, UiQualifiedTextVariationRecord,
};

#[derive(Clone, Copy, Default)]
pub(super) struct UiVariableFaceAxes {
    weight: Option<(i32, i32)>,
    width: Option<(i32, i32)>,
    italic: Option<(i32, i32)>,
    slant: Option<(i32, i32)>,
}

impl UiVariableFaceAxes {
    pub(super) const fn has_weight(self) -> bool {
        self.weight.is_some()
    }

    pub(super) const fn has_width(self) -> bool {
        self.width.is_some()
    }

    pub(super) const fn has_slant(self) -> bool {
        self.italic.is_some() || self.slant.is_some()
    }

    pub(super) fn weight_distance(self, request: u16) -> u32 {
        range_distance(self.weight, i32::from(request) * 1_000)
    }

    pub(super) fn width_distance(self, request_milli_percent: u32) -> u32 {
        range_distance(
            self.width,
            i32::try_from(request_milli_percent).unwrap_or(i32::MAX),
        )
    }

    pub(super) fn slant_supports(self, request: UiFontSlant) -> bool {
        match request {
            UiFontSlant::Upright => contains(self.italic, 0) || contains(self.slant, 0),
            UiFontSlant::Italic => contains(self.italic, 1_000),
            UiFontSlant::Oblique => self.slant.is_some_and(|range| range.0 < 0 || range.1 > 0),
        }
    }
}

pub(super) fn features(style: &crate::UiTextStyle) -> Vec<Feature> {
    style
        .features()
        .iter()
        .map(|feature| Feature::new(Tag::from_be_bytes(feature.tag()), feature.value(), ..))
        .collect()
}

pub(super) fn variations(font: &FontRef<'_>, style: &crate::UiTextStyle) -> Vec<Variation> {
    let mut variations = style
        .variations()
        .iter()
        .map(|variation| Variation {
            tag: Tag::from_be_bytes(variation.axis()),
            value: variation.value_milli() as f32 / 1_000.0,
        })
        .collect::<Vec<_>>();
    for (tag, requested) in derived_variations(style) {
        if style
            .variations()
            .iter()
            .any(|variation| variation.axis() == tag)
        {
            continue;
        }
        if let Some((minimum, maximum)) = axis_range(font, tag) {
            variations.push(Variation {
                tag: Tag::from_be_bytes(tag),
                value: requested.clamp(minimum, maximum),
            });
        }
    }
    variations
}

pub(super) fn qualified_variation_records(
    font: &FontRef<'_>,
    style: &UiQualifiedTextStyleRecord,
) -> Vec<UiQualifiedTextVariationRecord> {
    let mut records = style.variations().to_vec();
    for (axis, requested) in qualified_derived_variations(style) {
        if records.iter().any(|variation| variation.axis() == axis) {
            continue;
        }
        if let Some((minimum, maximum)) = axis_range_milli(font, axis) {
            records.push(UiQualifiedTextVariationRecord::from_text_mechanics(
                axis,
                requested.clamp(minimum, maximum),
            ));
        }
    }
    records
}

pub(super) fn variations_are_qualified(font: &FontRef<'_>, style: &crate::UiTextStyle) -> bool {
    if style.variations().is_empty() {
        return true;
    }
    let Some(bytes) = font.table_data(Tag::from_be_bytes(*b"fvar")) else {
        return false;
    };
    let bytes = bytes.as_bytes();
    let Some(axis_offset) = be_u16(bytes, 4).map(usize::from) else {
        return false;
    };
    let (Some(axis_count), Some(axis_size)) = (
        be_u16(bytes, 8).map(usize::from),
        be_u16(bytes, 10).map(usize::from),
    ) else {
        return false;
    };
    if axis_size < 20 {
        return false;
    }
    style.variations().iter().all(|variation| {
        let requested = variation.value_milli() as f32 / 1_000.0;
        axis_range_from_bytes(bytes, axis_offset, axis_count, axis_size, variation.axis())
            .is_some_and(|(minimum, maximum)| requested >= minimum && requested <= maximum)
    })
}

pub(super) fn variable_face_axes(font: &FontRef<'_>) -> UiVariableFaceAxes {
    UiVariableFaceAxes {
        weight: axis_range_milli(font, *b"wght"),
        width: axis_range_milli(font, *b"wdth"),
        italic: axis_range_milli(font, *b"ital"),
        slant: axis_range_milli(font, *b"slnt"),
    }
}

fn axis_range_milli(font: &FontRef<'_>, tag: [u8; 4]) -> Option<(i32, i32)> {
    let (minimum, maximum) = axis_range(font, tag)?;
    Some(((minimum * 1_000.0) as i32, (maximum * 1_000.0) as i32))
}

pub(in crate::font_collection) fn axis_range(
    font: &FontRef<'_>,
    tag: [u8; 4],
) -> Option<(f32, f32)> {
    let data = font.table_data(Tag::from_be_bytes(*b"fvar"))?;
    let bytes = data.as_bytes();
    let axis_offset = usize::from(be_u16(bytes, 4)?);
    let axis_count = usize::from(be_u16(bytes, 8)?);
    let axis_size = usize::from(be_u16(bytes, 10)?);
    (axis_size >= 20)
        .then(|| axis_range_from_bytes(bytes, axis_offset, axis_count, axis_size, tag))
        .flatten()
}

fn axis_range_from_bytes(
    bytes: &[u8],
    axis_offset: usize,
    axis_count: usize,
    axis_size: usize,
    tag: [u8; 4],
) -> Option<(f32, f32)> {
    (0..axis_count).find_map(|index| {
        let start = axis_offset + index * axis_size;
        let record = bytes.get(start..start + 16)?;
        (record[0..4] == tag).then(|| {
            let minimum = i32::from_be_bytes(record[4..8].try_into().expect("fixed field"));
            let maximum = i32::from_be_bytes(record[12..16].try_into().expect("fixed field"));
            (fixed_to_f32(minimum), fixed_to_f32(maximum))
        })
    })
}

fn derived_variations(style: &crate::UiTextStyle) -> [([u8; 4], f32); 4] {
    [
        (*b"wght", f32::from(style.face_request().weight())),
        (
            *b"wdth",
            style.face_request().width_milli_percent() as f32 / 1_000.0,
        ),
        (
            *b"ital",
            f32::from(style.face_request().slant() == UiFontSlant::Italic),
        ),
        (
            *b"slnt",
            if style.face_request().slant() == UiFontSlant::Oblique {
                -12.0
            } else {
                0.0
            },
        ),
    ]
}

fn qualified_derived_variations(style: &UiQualifiedTextStyleRecord) -> [([u8; 4], i32); 4] {
    [
        (*b"wght", i32::from(style.weight()) * 1_000),
        (
            *b"wdth",
            i32::try_from(style.width_milli_percent()).unwrap_or(i32::MAX),
        ),
        (
            *b"ital",
            i32::from(style.slant() == UiFontSlant::Italic) * 1_000,
        ),
        (
            *b"slnt",
            if style.slant() == UiFontSlant::Oblique {
                -12_000
            } else {
                0
            },
        ),
    ]
}

fn fixed_to_f32(value: i32) -> f32 {
    value as f32 / 65_536.0
}

fn range_distance(range: Option<(i32, i32)>, request: i32) -> u32 {
    let Some((minimum, maximum)) = range else {
        return u32::MAX;
    };
    if request < minimum {
        minimum.abs_diff(request)
    } else if request > maximum {
        request.abs_diff(maximum)
    } else {
        0
    }
}

fn contains(range: Option<(i32, i32)>, request: i32) -> bool {
    range.is_some_and(|(minimum, maximum)| (minimum..=maximum).contains(&request))
}

fn be_u16(bytes: &[u8], start: usize) -> Option<u16> {
    Some(u16::from_be_bytes(
        bytes.get(start..start + 2)?.try_into().ok()?,
    ))
}
