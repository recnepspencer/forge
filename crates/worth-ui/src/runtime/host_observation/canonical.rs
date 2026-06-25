use crate::runtime::{
    WorthUiHostFrameObservationDraft, WorthUiHostMeasurementReadinessPosture,
    WorthUiHostObservationCounters,
};

pub(super) fn canonical_observation_parts(
    draft: &WorthUiHostFrameObservationDraft,
    readiness: WorthUiHostMeasurementReadinessPosture,
    counters: WorthUiHostObservationCounters,
) -> Vec<String> {
    let mut parts = counter_parts(draft, readiness, counters);
    parts.extend(draft.available_bounds().iter().map(|row| {
        format!(
            "available_bounds:{}:{}:{}",
            row.node_id(),
            metric_token(row.width_points()),
            metric_token(row.height_points())
        )
    }));
    parts.extend(draft.viewports().iter().map(|row| {
        format!(
            "viewport:{}:{}:{}",
            row.node_id(),
            metric_token(row.width_points()),
            metric_token(row.height_points())
        )
    }));
    parts.extend(draft.scroll_viewports().iter().map(|row| {
        format!(
            "scroll_viewport:{}:{}:{}:{}:{}",
            row.node_id(),
            metric_token(row.scroll_x_points()),
            metric_token(row.scroll_y_points()),
            metric_token(row.width_points()),
            metric_token(row.height_points())
        )
    }));
    parts.extend(draft.text_metrics().iter().map(|row| {
        format!(
            "text_metric:{}:{}:{}:{}:{}",
            row.node_id(),
            row.text_digest(),
            metric_token(row.width_points()),
            metric_token(row.height_points()),
            metric_token(row.baseline_points())
        )
    }));
    parts.extend(draft.icon_metrics().iter().map(|row| {
        format!(
            "icon_metric:{}:{}:{}:{}:{}",
            row.node_id(),
            row.icon_digest(),
            metric_token(row.width_points()),
            metric_token(row.height_points()),
            metric_token(row.baseline_points())
        )
    }));
    if let Some(dpi_scale) = draft.dpi_scale() {
        parts.push(format!("dpi_scale:{}", metric_token(dpi_scale)));
    }
    parts.extend(
        draft
            .elapsed_time()
            .iter()
            .map(|row| format!("elapsed_time:{}:{}", row.lane(), row.elapsed_micros())),
    );
    parts.sort();
    parts
}

fn counter_parts(
    draft: &WorthUiHostFrameObservationDraft,
    readiness: WorthUiHostMeasurementReadinessPosture,
    counters: WorthUiHostObservationCounters,
) -> Vec<String> {
    vec![
        "host_frame_observation_values".to_owned(),
        draft.basis().basis_digest().to_string(),
        readiness.token().to_owned(),
        format!(
            "available_bounds_count:{}",
            counters.available_bounds_count()
        ),
        format!("viewport_count:{}", counters.viewport_count()),
        format!("scroll_viewport_count:{}", counters.scroll_viewport_count()),
        format!("text_metric_count:{}", counters.text_metric_count()),
        format!("icon_metric_count:{}", counters.icon_metric_count()),
        format!("dpi_count:{}", counters.dpi_count()),
        format!("elapsed_time_count:{}", counters.elapsed_time_count()),
    ]
}

fn metric_token(value: f32) -> u32 {
    if value == 0.0 {
        0.0f32.to_bits()
    } else {
        value.to_bits()
    }
}
