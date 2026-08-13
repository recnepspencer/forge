use worth_ui_host_contract::{
    UiQualifiedTextCaretRecord, UiQualifiedTextSelectionRect, UiTextCaretAffinity,
    UiTextCaretPosition, UiTextHitResult, UiTextOriginalRange, UiTextPoint, UiTextRect,
    UiTextVisualEdge,
};

#[derive(Clone, Copy, Debug)]
pub(super) struct PositionedCluster {
    pub(super) original_range: UiTextOriginalRange,
    pub(super) line_index: u32,
    pub(super) visual_run_index: u32,
    pub(super) bidi_level: u8,
    pub(super) bounds: UiTextRect,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct PositionedLineAnchor {
    pub(super) original_boundary: u32,
    pub(super) line_index: u32,
    pub(super) visual_run_index: u32,
    pub(super) bounds: UiTextRect,
    pub(super) hit_bounds: UiTextRect,
}

#[derive(Clone, Copy)]
struct CaretGeometry {
    line_index: u32,
    visual_run_index: u32,
    x_millipoints: i64,
    top_millipoints: i64,
    bottom_millipoints: i64,
}

pub(super) fn carets(
    clusters: &[PositionedCluster],
    line_anchors: &[PositionedLineAnchor],
) -> Vec<UiQualifiedTextCaretRecord> {
    let mut carets = Vec::with_capacity(clusters.len() * 2 + line_anchors.len());
    for cluster in clusters {
        let rtl = !cluster.bidi_level.is_multiple_of(2);
        let leading_x = if rtl {
            cluster.bounds.right_millipoints()
        } else {
            cluster.bounds.left_millipoints()
        };
        let trailing_x = if rtl {
            cluster.bounds.left_millipoints()
        } else {
            cluster.bounds.right_millipoints()
        };
        carets.push(caret(
            *cluster,
            cluster.original_range.start(),
            UiTextVisualEdge::Leading,
            UiTextCaretAffinity::Downstream,
            leading_x,
        ));
        carets.push(caret(
            *cluster,
            cluster.original_range.end(),
            UiTextVisualEdge::Trailing,
            UiTextCaretAffinity::Upstream,
            trailing_x,
        ));
    }
    carets.extend(line_anchors.iter().map(|anchor| {
        caret_record(
            anchor.original_boundary,
            UiTextVisualEdge::Leading,
            UiTextCaretAffinity::Downstream,
            CaretGeometry {
                line_index: anchor.line_index,
                visual_run_index: anchor.visual_run_index,
                x_millipoints: anchor.bounds.left_millipoints(),
                top_millipoints: anchor.bounds.top_millipoints(),
                bottom_millipoints: anchor.bounds.bottom_millipoints(),
            },
        )
    }));
    carets
}

pub(super) fn hit_test(
    clusters: &[PositionedCluster],
    line_anchors: &[PositionedLineAnchor],
    carets: &[UiQualifiedTextCaretRecord],
    point: UiTextPoint,
) -> Option<UiTextHitResult> {
    let cluster = clusters
        .iter()
        .min_by_key(|cluster| distance(cluster.bounds, point));
    let line_anchor = line_anchors
        .iter()
        .min_by_key(|anchor| distance(anchor.hit_bounds, point));
    match (cluster, line_anchor) {
        (Some(cluster), Some(anchor))
            if distance(anchor.hit_bounds, point) < distance(cluster.bounds, point) =>
        {
            anchor_hit(*anchor, carets)
        }
        (Some(cluster), _) => cluster_hit(*cluster, carets, point),
        (None, Some(anchor)) => anchor_hit(*anchor, carets),
        (None, None) => None,
    }
}

fn cluster_hit(
    cluster: PositionedCluster,
    carets: &[UiQualifiedTextCaretRecord],
    point: UiTextPoint,
) -> Option<UiTextHitResult> {
    let midpoint = cluster.bounds.left_millipoints() + cluster.bounds.width_millipoints() / 2;
    let visual_left = point.x_millipoints() <= midpoint;
    let rtl = !cluster.bidi_level.is_multiple_of(2);
    let edge = if visual_left == rtl {
        UiTextVisualEdge::Trailing
    } else {
        UiTextVisualEdge::Leading
    };
    let boundary = match edge {
        UiTextVisualEdge::Leading => cluster.original_range.start(),
        UiTextVisualEdge::Trailing => cluster.original_range.end(),
    };
    let caret = carets.iter().copied().find(|caret| {
        caret.line_index() == cluster.line_index
            && caret.visual_run_index() == cluster.visual_run_index
            && caret.position().original_boundary().start() == boundary
            && caret.position().visual_edge() == edge
    })?;
    Some(UiTextHitResult::from_text_mechanics(
        caret,
        cluster.original_range,
        edge,
    ))
}

fn anchor_hit(
    anchor: PositionedLineAnchor,
    carets: &[UiQualifiedTextCaretRecord],
) -> Option<UiTextHitResult> {
    let caret = carets.iter().copied().find(|caret| {
        caret.line_index() == anchor.line_index
            && caret.visual_run_index() == anchor.visual_run_index
            && caret.position().original_boundary().start() == anchor.original_boundary
    })?;
    let boundary = UiTextOriginalRange::from_text_mechanics(
        anchor.original_boundary,
        anchor.original_boundary,
    )?;
    Some(UiTextHitResult::from_text_mechanics(
        caret,
        boundary,
        UiTextVisualEdge::Leading,
    ))
}

pub(super) fn selection_rects(
    source: &str,
    graphemes: &[worth_ui_host_contract::UiQualifiedTextGraphemeRecord],
    glyphs: &[worth_ui_host_contract::UiQualifiedTextGlyphRecord],
    clusters: &[PositionedCluster],
    selected: UiTextOriginalRange,
) -> Result<Box<[UiQualifiedTextSelectionRect]>, super::UiTextSelectionDenial> {
    validate_selection(source, graphemes, glyphs, selected)?;
    let mut output = Vec::new();
    let mut index = 0usize;
    while index < clusters.len() {
        if !overlaps(clusters[index].original_range, selected) {
            index += 1;
            continue;
        }
        let first = clusters[index];
        let mut end = index + 1;
        let mut left = first.bounds.left_millipoints();
        let mut right = first.bounds.right_millipoints();
        let mut source_start = first.original_range.start();
        let mut source_end = first.original_range.end();
        while end < clusters.len()
            && clusters[end].line_index == first.line_index
            && clusters[end].visual_run_index == first.visual_run_index
            && overlaps(clusters[end].original_range, selected)
        {
            left = left.min(clusters[end].bounds.left_millipoints());
            right = right.max(clusters[end].bounds.right_millipoints());
            source_start = source_start.min(clusters[end].original_range.start());
            source_end = source_end.max(clusters[end].original_range.end());
            end += 1;
        }
        let selected_range = UiTextOriginalRange::from_text_mechanics(
            source_start.max(selected.start()),
            source_end.min(selected.end()),
        )
        .expect("overlap is ordered");
        output.push(UiQualifiedTextSelectionRect::from_text_mechanics(
            selected_range,
            first.line_index,
            first.visual_run_index,
            UiTextRect::from_text_mechanics(
                left,
                first.bounds.top_millipoints(),
                right,
                first.bounds.bottom_millipoints(),
            )
            .expect("selection bounds are ordered"),
        ));
        index = end;
    }
    Ok(output.into_boxed_slice())
}

fn validate_selection(
    source: &str,
    graphemes: &[worth_ui_host_contract::UiQualifiedTextGraphemeRecord],
    glyphs: &[worth_ui_host_contract::UiQualifiedTextGlyphRecord],
    selected: UiTextOriginalRange,
) -> Result<(), super::UiTextSelectionDenial> {
    use super::UiTextSelectionDenial as Denial;
    let start = usize::try_from(selected.start()).map_err(|_| Denial::RangeOutOfBounds)?;
    let end = usize::try_from(selected.end()).map_err(|_| Denial::RangeOutOfBounds)?;
    if end > source.len() {
        return Err(Denial::RangeOutOfBounds);
    }
    if !source.is_char_boundary(start) || !source.is_char_boundary(end) {
        return Err(Denial::NotUtf8Boundary);
    }
    let is_cluster_boundary = |boundary| {
        boundary == 0
            || boundary == source.len() as u32
            || (graphemes.iter().any(|grapheme| {
                grapheme.original_range().start() == boundary
                    || grapheme.original_range().end() == boundary
            }) && !glyphs.iter().any(|glyph| {
                glyph.original_range().start() < boundary && boundary < glyph.original_range().end()
            }))
    };
    if !is_cluster_boundary(selected.start()) || !is_cluster_boundary(selected.end()) {
        return Err(Denial::NotClusterBoundary);
    }
    Ok(())
}

fn caret(
    cluster: PositionedCluster,
    boundary: u32,
    visual_edge: UiTextVisualEdge,
    affinity: UiTextCaretAffinity,
    x_millipoints: i64,
) -> UiQualifiedTextCaretRecord {
    caret_record(
        boundary,
        visual_edge,
        affinity,
        CaretGeometry {
            line_index: cluster.line_index,
            visual_run_index: cluster.visual_run_index,
            x_millipoints,
            top_millipoints: cluster.bounds.top_millipoints(),
            bottom_millipoints: cluster.bounds.bottom_millipoints(),
        },
    )
}

fn caret_record(
    boundary: u32,
    visual_edge: UiTextVisualEdge,
    affinity: UiTextCaretAffinity,
    geometry: CaretGeometry,
) -> UiQualifiedTextCaretRecord {
    let boundary = UiTextOriginalRange::from_text_mechanics(boundary, boundary)
        .expect("empty boundary is ordered");
    UiQualifiedTextCaretRecord::from_text_mechanics(
        UiTextCaretPosition::from_text_mechanics(boundary, visual_edge, affinity)
            .expect("empty range forms a caret"),
        geometry.line_index,
        geometry.visual_run_index,
        geometry.x_millipoints,
        geometry.top_millipoints,
        geometry.bottom_millipoints,
    )
}

fn overlaps(left: UiTextOriginalRange, right: UiTextOriginalRange) -> bool {
    left.start() < right.end() && right.start() < left.end()
}

fn distance(rect: UiTextRect, point: UiTextPoint) -> u64 {
    let dx = if point.x_millipoints() < rect.left_millipoints() {
        rect.left_millipoints() - point.x_millipoints()
    } else if point.x_millipoints() > rect.right_millipoints() {
        point.x_millipoints() - rect.right_millipoints()
    } else {
        0
    };
    let dy = if point.y_millipoints() < rect.top_millipoints() {
        rect.top_millipoints() - point.y_millipoints()
    } else if point.y_millipoints() > rect.bottom_millipoints() {
        point.y_millipoints() - rect.bottom_millipoints()
    } else {
        0
    };
    dx.unsigned_abs().saturating_add(dy.unsigned_abs())
}
