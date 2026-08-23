use harfrust::{Direction, Language};
use worth_ui_host_contract::{
    UiQualifiedTextRunInput, UiQualifiedTextRunRecord, UiTextOriginalRange,
};

use super::{
    cluster_boundaries, missing_cluster_glyph, qualified_glyph, range_for_cluster,
    UiTextShapingCost, UiTextShapingDenial,
};
use crate::{UiFallbackTextParagraph, UiGlobalTextProfile, UiSelectedTextCluster};

pub(super) struct ShapedRecords {
    pub(super) runs: Box<[UiQualifiedTextRunRecord]>,
    pub(super) glyphs: Box<[worth_ui_host_contract::UiQualifiedTextGlyphRecord]>,
    pub(super) unsafe_break_boundaries: Box<[u32]>,
    pub(super) cost: UiTextShapingCost,
}

pub(super) fn shape(
    fallback: &UiFallbackTextParagraph,
    line_boundaries: &[u32],
) -> Result<ShapedRecords, UiTextShapingDenial> {
    let ranges = run_ranges(fallback.clusters())?;
    if ranges.len() > fallback.capacity().runs() as usize {
        return Err(UiTextShapingDenial::RunCapacityExceeded);
    }
    let mut runs = Vec::with_capacity(ranges.len());
    let mut glyphs = Vec::new();
    let mut unsafe_breaks = Vec::new();
    let mut cost = UiTextShapingCost::default();
    for (start, end) in ranges {
        let first = fallback.clusters()[start];
        let last = fallback.clusters()[end - 1];
        let original_start = first.original_range().start();
        let original_end = last.original_range().end();
        let glyph_start = u32::try_from(glyphs.len()).expect("glyph capacity fits u32");
        let mut metrics = None;
        let mut segment_start = start;
        while segment_start < end {
            let segment_end = (segment_start + 1..end)
                .find(|index| {
                    line_boundaries
                        .binary_search(&fallback.clusters()[*index].original_range().start())
                        .is_ok()
                })
                .unwrap_or(end);
            let segment_metrics = shape_segment(
                fallback,
                segment_start,
                segment_end,
                &mut glyphs,
                &mut unsafe_breaks,
                &mut cost,
            )?;
            match metrics {
                Some(current) if current != segment_metrics => {
                    return Err(UiTextShapingDenial::InconsistentRunMetrics);
                }
                Some(_) => {}
                None => metrics = Some(segment_metrics),
            }
            segment_start = segment_end;
        }
        let glyph_end = u32::try_from(glyphs.len()).expect("glyph capacity fits u32");
        runs.push(run_record(
            first,
            original_start,
            original_end,
            glyph_start,
            glyph_end,
            metrics.expect("semantic run has one shaped segment"),
        ));
    }
    unsafe_breaks.sort_unstable();
    unsafe_breaks.dedup();
    Ok(ShapedRecords {
        runs: runs.into_boxed_slice(),
        glyphs: glyphs.into_boxed_slice(),
        unsafe_break_boundaries: unsafe_breaks.into_boxed_slice(),
        cost,
    })
}

fn shape_segment(
    fallback: &UiFallbackTextParagraph,
    start: usize,
    end: usize,
    glyphs: &mut Vec<worth_ui_host_contract::UiQualifiedTextGlyphRecord>,
    unsafe_breaks: &mut Vec<u32>,
    cost: &mut UiTextShapingCost,
) -> Result<RunMetrics, UiTextShapingDenial> {
    let first = fallback.clusters()[start];
    let last = fallback.clusters()[end - 1];
    let original_start = first.original_range().start();
    let original_end = last.original_range().end();
    let text = &fallback.source()[original_start as usize..original_end as usize];
    let style = fallback.styles()[first.style_index()].style();
    let language = Language::new(style.language()).expect("admitted language");
    let direction = if first.bidi_level().is_multiple_of(2) {
        Direction::LeftToRight
    } else {
        Direction::RightToLeft
    };
    let crate::font_collection::UiFontShapedRun {
        units_per_em,
        ascender_font_units,
        descender_font_units,
        line_gap_font_units,
        glyphs: shaped_glyphs,
    } = fallback.fonts().shape_run(
        first.face_slot().expect("renderable run has a face"),
        text,
        original_start,
        direction,
        &language,
        first.script_tag(),
        style,
    );
    let emitted = if first.coverage() == crate::UiTextCoverageDisposition::MissingCluster {
        1
    } else {
        shaped_glyphs.len()
    };
    if glyphs
        .len()
        .checked_add(emitted)
        .is_none_or(|count| count > fallback.capacity().glyphs() as usize)
    {
        return Err(UiTextShapingDenial::GlyphCapacityExceeded);
    }
    if shaped_glyphs.iter().any(|glyph| glyph.glyph_id == 0) {
        return Err(UiTextShapingDenial::NotdefAfterFaceSelection);
    }
    unsafe_breaks.extend(
        shaped_glyphs
            .iter()
            .filter(|glyph| glyph.unsafe_to_break)
            .map(|glyph| glyph.cluster),
    );
    let glyph_start = u32::try_from(glyphs.len()).expect("glyph capacity fits u32");
    if first.coverage() == crate::UiTextCoverageDisposition::MissingCluster {
        glyphs.push(missing_cluster_glyph(
            &shaped_glyphs,
            original_start,
            original_end,
        )?);
    } else {
        let boundaries = cluster_boundaries(&shaped_glyphs, original_start, original_end)?;
        for glyph in shaped_glyphs {
            let range = range_for_cluster(&boundaries, glyph.cluster)?;
            glyphs.push(qualified_glyph(glyph, range));
        }
    }
    if glyphs.len() > UiGlobalTextProfile::MAX_GLYPHS {
        return Err(UiTextShapingDenial::GlyphCapacityExceeded);
    }
    let glyph_end = u32::try_from(glyphs.len()).expect("glyph capacity fits u32");
    cost.runs_shaped += 1;
    cost.input_scalars_shaped +=
        u32::try_from(text.chars().count()).expect("admitted text fits u32");
    cost.glyphs_emitted += glyph_end - glyph_start;
    Ok(RunMetrics {
        units_per_em,
        ascender_font_units,
        descender_font_units,
        line_gap_font_units,
    })
}

fn run_record(
    first: UiSelectedTextCluster,
    original_start: u32,
    original_end: u32,
    glyph_start: u32,
    glyph_end: u32,
    metrics: RunMetrics,
) -> UiQualifiedTextRunRecord {
    UiQualifiedTextRunRecord::from_text_mechanics(UiQualifiedTextRunInput {
        original_range: UiTextOriginalRange::from_text_mechanics(original_start, original_end)
            .expect("ordered run range"),
        glyph_start,
        glyph_end,
        face: first.face().expect("renderable run has a face"),
        script_tag: first.script_tag(),
        bidi_level: first.bidi_level(),
        units_per_em: metrics.units_per_em,
        style_index: u16::try_from(first.style_index()).expect("style cap fits u16"),
        ascender_font_units: metrics.ascender_font_units,
        descender_font_units: metrics.descender_font_units,
        line_gap_font_units: metrics.line_gap_font_units,
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct RunMetrics {
    units_per_em: u16,
    ascender_font_units: i16,
    descender_font_units: i16,
    line_gap_font_units: i16,
}

fn run_ranges(
    clusters: &[UiSelectedTextCluster],
) -> Result<Vec<(usize, usize)>, UiTextShapingDenial> {
    let mut ranges = Vec::new();
    let mut start = 0;
    while start < clusters.len() {
        if clusters[start].face_slot().is_none() {
            start += 1;
            continue;
        }
        let first = clusters[start];
        let mut end = start + 1;
        while end < clusters.len()
            && clusters[end].face_slot().is_some()
            && first.coverage() != crate::UiTextCoverageDisposition::MissingCluster
            && clusters[end].coverage() != crate::UiTextCoverageDisposition::MissingCluster
            && clusters[end].face_slot() == first.face_slot()
            && clusters[end].script_tag() == first.script_tag()
            && clusters[end].bidi_level() == first.bidi_level()
            && clusters[end].style_index() == first.style_index()
        {
            end += 1;
        }
        ranges.push((start, end));
        if ranges.len() > UiGlobalTextProfile::MAX_RUNS_PER_PARAGRAPH {
            return Err(UiTextShapingDenial::RunCapacityExceeded);
        }
        start = end;
    }
    Ok(ranges)
}
