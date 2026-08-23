use worth_ui_host_contract::{
    UiGlyphRunView, UiMountedPaintCommand, UiMountedPaintCommandChange,
    UiMountedPresentationWorkView, UiMountedSemanticTextMechanic, UiMountedTextRasterWork,
};

pub(in crate::native::mechanics_adapter) fn admits(
    view: &worth_ui_host_contract::UiMountedFrameConsumptionView<'_>,
    raster: &UiMountedTextRasterWork<'_>,
) -> bool {
    let shape = demand_shape_admits(
        raster.glyph_runs().len(),
        raster
            .demands()
            .iter()
            .map(|batch| batch.records().len())
            .sum(),
    );
    let runs = raster
        .glyph_runs()
        .iter()
        .all(|run| admits_run(view, raster, *run));
    let demands = raster.demands().iter().all(|batch| {
        batch.records().iter().all(|record| {
            raster
                .glyph_runs()
                .iter()
                .any(|run| run.raster_key() == record.key())
        })
    });
    shape && runs && demands
}

fn demand_shape_admits(glyph_run_count: usize, demand_record_count: usize) -> bool {
    (glyph_run_count == 0) == (demand_record_count == 0)
}

fn admits_run(
    view: &worth_ui_host_contract::UiMountedFrameConsumptionView<'_>,
    raster: &UiMountedTextRasterWork<'_>,
    run: UiGlyphRunView,
) -> bool {
    demand_contains_run(raster, run)
        && semantic_mechanic(view.presentation_work(), run).is_some_and(|mechanic| {
            view.qualified_text_layout(mechanic)
                .is_some_and(|layout| mechanic_contains_run(mechanic, layout, run))
        })
}

fn demand_contains_run(raster: &UiMountedTextRasterWork<'_>, run: UiGlyphRunView) -> bool {
    raster.demands().iter().any(|batch| {
        batch.layout_identity() == run.layout_identity()
            && batch.records().iter().any(|record| {
                record.key() == run.raster_key()
                    && record.attribution().original_range() == run.original_range()
            })
    })
}

fn semantic_mechanic(
    presentation: UiMountedPresentationWorkView<'_>,
    run: UiGlyphRunView,
) -> Option<&UiMountedSemanticTextMechanic> {
    match presentation {
        UiMountedPresentationWorkView::Initial(initial) => initial
            .commands()
            .iter()
            .find_map(|command| matching_mechanic(command, run)),
        UiMountedPresentationWorkView::Reconstruction(reconstruction) => reconstruction
            .commands()
            .iter()
            .find_map(|command| matching_mechanic(command, run)),
        UiMountedPresentationWorkView::Delta(delta) => {
            delta.changes().iter().find_map(|change| match change {
                UiMountedPaintCommandChange::Insert(command)
                | UiMountedPaintCommandChange::Replace {
                    successor: command, ..
                } => matching_mechanic(command, run),
                UiMountedPaintCommandChange::Remove(_) => None,
            })
        }
        UiMountedPresentationWorkView::Unchanged(_) => None,
    }
}

fn matching_mechanic(
    command: &UiMountedPaintCommand,
    run: UiGlyphRunView,
) -> Option<&UiMountedSemanticTextMechanic> {
    match command {
        UiMountedPaintCommand::SemanticText { identity, mechanic }
            if *identity == run.mechanic() =>
        {
            Some(mechanic)
        }
        UiMountedPaintCommand::FilledRect { .. } | UiMountedPaintCommand::SemanticText { .. } => {
            None
        }
    }
}

fn mechanic_contains_run(
    mechanic: &UiMountedSemanticTextMechanic,
    layout: worth_ui_host_contract::UiQualifiedTextLayoutView<'_>,
    run: UiGlyphRunView,
) -> bool {
    mechanic.qualified_layout_identity() == run.layout_identity()
        && layout.identity() == run.layout_identity()
        && run.clip_bounds() == mechanic.clip_bounds()
        && run.layer_semantic_order() == mechanic.layer_semantic_order()
        && positioned_glyph_matches(mechanic, layout, run)
        && mechanic.foregrounds().iter().any(|foreground| {
            foreground.identity() == run.paint_span()
                && range_contains(foreground.original_range(), run.original_range())
                && foreground.color() == run.foreground()
        })
}

fn positioned_glyph_matches(
    mechanic: &UiMountedSemanticTextMechanic,
    layout: worth_ui_host_contract::UiQualifiedTextLayoutView<'_>,
    run: UiGlyphRunView,
) -> bool {
    layout.positioned_glyphs().iter().any(|positioned| {
        let Some(glyph) = usize::try_from(positioned.source_glyph_index())
            .ok()
            .and_then(|index| layout.glyphs().get(index))
        else {
            return false;
        };
        glyph.glyph_id() == run.raster_key().glyph_id()
            && glyph.original_range() == run.original_range()
            && positioned.line_index() == run.line_index()
            && positioned.visual_run_index() == run.visual_run_index()
            && mounted_origin_millipoints(mechanic.origin_x(), positioned.origin_x_millipoints())
                == Some(run.origin_x_millipoints())
            && mounted_origin_millipoints(mechanic.origin_y(), positioned.origin_y_millipoints())
                == Some(run.origin_y_millipoints())
    })
}

fn mounted_origin_millipoints(origin: f32, positioned: i64) -> Option<i64> {
    if !origin.is_finite() {
        return None;
    }
    let mounted = (f64::from(origin) * 1_000.0).round();
    if mounted < i64::MIN as f64 || mounted > i64::MAX as f64 {
        return None;
    }
    (mounted as i64).checked_add(positioned)
}

fn range_contains(
    span: worth_ui_host_contract::UiTextOriginalRange,
    glyph: worth_ui_host_contract::UiTextOriginalRange,
) -> bool {
    span.start() <= glyph.start() && span.end() >= glyph.end()
}

#[cfg(test)]
mod tests {
    use super::{demand_shape_admits, range_contains};
    use worth_ui_host_contract::UiTextOriginalRange;

    #[test]
    fn a_glyph_cluster_must_be_contained_by_its_exact_paint_span() {
        let span = UiTextOriginalRange::new(4, 12).unwrap();
        assert!(range_contains(
            span,
            UiTextOriginalRange::new(6, 8).unwrap()
        ));
        assert!(!range_contains(
            span,
            UiTextOriginalRange::new(2, 8).unwrap()
        ));
        assert!(!range_contains(
            span,
            UiTextOriginalRange::new(8, 14).unwrap()
        ));
    }

    #[test]
    fn release_only_work_admits_no_runs_and_no_demands() {
        assert!(demand_shape_admits(0, 0));
        assert!(demand_shape_admits(2, 2));
        assert!(!demand_shape_admits(0, 1));
        assert!(!demand_shape_admits(1, 0));
    }
}
