//! Shared qualified bitmap source and strike selection.

use read_fonts::{
    tables::{bitmap::BitmapContent, cbdt::Cbdt, cblc::Cblc},
    types::{GlyphId, Tag},
    FontData, FontRef, TableProvider,
};
use skrifa::{
    bitmap::{BitmapData, BitmapFormat, BitmapGlyph, BitmapStrikes, Origin},
    instance::{LocationRef, Size},
    MetadataProvider,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiBitmapSelectionError {
    Malformed,
    Unsupported,
}

pub(crate) enum UiBitmapSelection<'a> {
    Direct(BitmapGlyph<'a>),
    CbdtComposite(UiCbdtCompositeSelection<'a>),
}

impl UiBitmapSelection<'_> {
    pub(crate) fn ppem(&self) -> f32 {
        match self {
            Self::Direct(bitmap) => bitmap.ppem_y,
            Self::CbdtComposite(composite) => f32::from(composite.size.ppem_y()),
        }
    }
}

pub(crate) struct UiCbdtCompositeSelection<'a> {
    pub(crate) cblc: Cblc<'a>,
    pub(crate) cbdt: Cbdt<'a>,
    pub(crate) size: read_fonts::tables::bitmap::BitmapSize,
    pub(crate) glyph: GlyphId,
    pub(crate) data: read_fonts::tables::bitmap::BitmapData<'a>,
}

pub(crate) fn select<'font>(
    face: &FontRef<'font>,
    glyph: GlyphId,
    desired_ppem: f32,
) -> Result<Option<UiBitmapSelection<'font>>, UiBitmapSelectionError> {
    let mut selected = None;
    for candidate in selections(face, glyph)? {
        select_candidate(&mut selected, candidate, desired_ppem);
    }
    Ok(selected)
}

fn select_candidate<'font>(
    selected: &mut Option<UiBitmapSelection<'font>>,
    candidate: UiBitmapSelection<'font>,
    desired_ppem: f32,
) {
    let ppem = candidate.ppem();
    if selected
        .as_ref()
        .is_none_or(|current| closer_strike(ppem, current.ppem(), desired_ppem))
    {
        *selected = Some(candidate);
    }
}

pub(crate) fn selections<'font>(
    face: &FontRef<'font>,
    glyph: GlyphId,
) -> Result<Vec<UiBitmapSelection<'font>>, UiBitmapSelectionError> {
    let mut selections = select_cbdt_direct(face, glyph)?
        .into_iter()
        .map(UiBitmapSelection::Direct)
        .collect::<Vec<_>>();
    selections.extend(
        select_cbdt_composite(face, glyph)?
            .into_iter()
            .map(UiBitmapSelection::CbdtComposite),
    );
    selections.extend(
        select_sbix(face, glyph)?
            .into_iter()
            .map(UiBitmapSelection::Direct),
    );
    Ok(selections)
}

fn select_cbdt_direct<'font>(
    face: &FontRef<'font>,
    glyph: GlyphId,
) -> Result<Vec<BitmapGlyph<'font>>, UiBitmapSelectionError> {
    let has_cblc = face.data_for_tag(Tag::new(b"CBLC")).is_some();
    let has_cbdt = face.data_for_tag(Tag::new(b"CBDT")).is_some();
    if !has_cblc && !has_cbdt {
        return Ok(Vec::new());
    }
    if has_cblc != has_cbdt {
        return Err(UiBitmapSelectionError::Malformed);
    }
    let strikes = BitmapStrikes::with_format(face, BitmapFormat::Cbdt)
        .ok_or(UiBitmapSelectionError::Malformed)?;
    let mut selected = Vec::new();
    for strike in strikes.iter() {
        if let Some(bitmap) = strike.get(glyph) {
            selected.push(bitmap);
        }
    }
    Ok(selected)
}

fn select_cbdt_composite<'font>(
    face: &FontRef<'font>,
    glyph: GlyphId,
) -> Result<Vec<UiCbdtCompositeSelection<'font>>, UiBitmapSelectionError> {
    if face.data_for_tag(Tag::new(b"CBLC")).is_none()
        && face.data_for_tag(Tag::new(b"CBDT")).is_none()
    {
        return Ok(Vec::new());
    }
    let cblc = face.cblc().map_err(|_| UiBitmapSelectionError::Malformed)?;
    let cbdt = face.cbdt().map_err(|_| UiBitmapSelectionError::Malformed)?;
    let mut selected = Vec::new();
    for size in cblc.bitmap_sizes() {
        let location = size
            .location(cblc.offset_data(), glyph)
            .map_err(|_| UiBitmapSelectionError::Malformed)?;
        if location.is_empty() {
            continue;
        }
        let data = cbdt
            .data(&location)
            .map_err(|_| UiBitmapSelectionError::Malformed)?;
        if !matches!(data.content, BitmapContent::Composite(_)) {
            continue;
        }
        selected.push(UiCbdtCompositeSelection {
            cblc: cblc.clone(),
            cbdt: cbdt.clone(),
            size: *size,
            glyph,
            data,
        });
    }
    Ok(selected)
}

fn select_sbix<'font>(
    face: &FontRef<'font>,
    glyph: GlyphId,
) -> Result<Vec<BitmapGlyph<'font>>, UiBitmapSelectionError> {
    select_sbix_candidates(face, glyph)?
        .into_iter()
        .map(|candidate| resolve_sbix_candidate(face, glyph, candidate))
        .collect()
}

struct UiSbixCandidate<'a> {
    strike: read_fonts::tables::sbix::Strike<'a>,
    glyph: read_fonts::tables::sbix::GlyphData<'a>,
    ppem: f32,
}

fn select_sbix_candidates<'font>(
    face: &FontRef<'font>,
    glyph: GlyphId,
) -> Result<Vec<UiSbixCandidate<'font>>, UiBitmapSelectionError> {
    if face.data_for_tag(Tag::new(b"sbix")).is_none() {
        return Ok(Vec::new());
    }
    let sbix = face.sbix().map_err(|_| UiBitmapSelectionError::Malformed)?;
    let mut selected = Vec::new();
    for strike in sbix.strikes().iter() {
        let strike = strike.map_err(|_| UiBitmapSelectionError::Malformed)?;
        let Some(data) = strike
            .glyph_data(glyph)
            .map_err(|_| UiBitmapSelectionError::Malformed)?
        else {
            continue;
        };
        let graphic_type = data.graphic_type().to_be_bytes();
        if graphic_type != *b"png " && graphic_type != *b"dupe" {
            return Err(UiBitmapSelectionError::Unsupported);
        }
        let ppem = f32::from(strike.ppem());
        selected.push(UiSbixCandidate {
            strike,
            glyph: data,
            ppem,
        });
    }
    Ok(selected)
}

fn resolve_sbix_candidate<'font>(
    face: &FontRef<'font>,
    glyph: GlyphId,
    candidate: UiSbixCandidate<'font>,
) -> Result<BitmapGlyph<'font>, UiBitmapSelectionError> {
    let (png, origin_x, origin_y) = match candidate.glyph.graphic_type().to_be_bytes() {
        type_name if type_name == *b"png " => (
            candidate.glyph.data(),
            candidate.glyph.origin_offset_x(),
            candidate.glyph.origin_offset_y(),
        ),
        type_name if type_name == *b"dupe" => {
            let target = candidate
                .glyph
                .data()
                .try_into()
                .ok()
                .map(u16::from_be_bytes)
                .ok_or(UiBitmapSelectionError::Malformed)?;
            let target = candidate
                .strike
                .glyph_data(GlyphId::from(target))
                .map_err(|_| UiBitmapSelectionError::Malformed)?
                .ok_or(UiBitmapSelectionError::Malformed)?;
            if target.graphic_type().to_be_bytes() != *b"png " {
                return Err(UiBitmapSelectionError::Unsupported);
            }
            (
                target.data(),
                candidate.glyph.origin_offset_x(),
                candidate.glyph.origin_offset_y(),
            )
        }
        _ => return Err(UiBitmapSelectionError::Unsupported),
    };
    let reader = FontData::new(png);
    let width = reader
        .read_at::<u32>(16)
        .map_err(|_| UiBitmapSelectionError::Malformed)?;
    let height = reader
        .read_at::<u32>(20)
        .map_err(|_| UiBitmapSelectionError::Malformed)?;
    let metrics = face.glyph_metrics(Size::unscaled(), LocationRef::default());
    let bounds = metrics.bounds(glyph).unwrap_or_default();
    Ok(BitmapGlyph {
        data: BitmapData::Png(png),
        bearing_x: metrics.left_side_bearing(glyph).unwrap_or_default(),
        bearing_y: bounds.y_min,
        inner_bearing_x: f32::from(origin_x),
        inner_bearing_y: f32::from(origin_y),
        ppem_x: candidate.ppem,
        ppem_y: candidate.ppem,
        width,
        height,
        advance: None,
        placement_origin: Origin::BottomLeft,
    })
}

fn closer_strike(candidate: f32, current: f32, desired: f32) -> bool {
    let candidate_distance = (candidate - desired).abs();
    let current_distance = (current - desired).abs();
    candidate_distance < current_distance
        || (candidate_distance == current_distance && candidate < current)
}
