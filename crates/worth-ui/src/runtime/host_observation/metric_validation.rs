use std::collections::BTreeSet;

use crate::runtime::{
    WorthUiHostFrameObservationDraft, WorthUiHostObservationAdmissionDenial,
    WorthUiHostObservationAdmissionDenialCode,
};

pub(super) fn validate_metric_rows(
    draft: &WorthUiHostFrameObservationDraft,
    denials: &mut Vec<WorthUiHostObservationAdmissionDenial>,
) {
    if draft
        .dpi_scale()
        .is_some_and(|scale| !scale.is_finite() || scale <= 0.0)
    {
        push_invalid_metric_denial(denials, "dpi_scale");
    }
    validate_box_metrics(denials, "available_bounds", draft.available_bounds());
    validate_box_metrics(denials, "viewport", draft.viewports());
    for row in draft.scroll_viewports() {
        validate_dimension(
            denials,
            "scroll_viewport",
            row.node_id(),
            "scroll_x",
            row.scroll_x_points(),
        );
        validate_dimension(
            denials,
            "scroll_viewport",
            row.node_id(),
            "scroll_y",
            row.scroll_y_points(),
        );
        validate_dimension(
            denials,
            "scroll_viewport",
            row.node_id(),
            "width",
            row.width_points(),
        );
        validate_dimension(
            denials,
            "scroll_viewport",
            row.node_id(),
            "height",
            row.height_points(),
        );
    }
    for row in draft.text_metrics() {
        validate_content_metric(
            denials,
            "text_metric",
            row.node_id(),
            row.width_points(),
            row.height_points(),
            row.baseline_points(),
        );
    }
    for row in draft.icon_metrics() {
        validate_content_metric(
            denials,
            "icon_metric",
            row.node_id(),
            row.width_points(),
            row.height_points(),
            row.baseline_points(),
        );
    }
    validate_elapsed_lanes(draft, denials);
}

fn validate_box_metrics(
    denials: &mut Vec<WorthUiHostObservationAdmissionDenial>,
    family: &str,
    rows: &[impl HostBoxMetricRow],
) {
    for row in rows {
        validate_dimension(denials, family, row.node_id(), "width", row.width_points());
        validate_dimension(
            denials,
            family,
            row.node_id(),
            "height",
            row.height_points(),
        );
    }
}

fn validate_content_metric(
    denials: &mut Vec<WorthUiHostObservationAdmissionDenial>,
    family: &str,
    node_id: &str,
    width_points: f32,
    height_points: f32,
    baseline_points: f32,
) {
    validate_dimension(denials, family, node_id, "width", width_points);
    validate_dimension(denials, family, node_id, "height", height_points);
    validate_dimension(denials, family, node_id, "baseline", baseline_points);
    if baseline_points.is_finite() && height_points.is_finite() && baseline_points > height_points {
        push_invalid_metric_denial(denials, &format!("{family}:{node_id}:baseline"));
    }
}

fn validate_dimension(
    denials: &mut Vec<WorthUiHostObservationAdmissionDenial>,
    family: &str,
    node_id: &str,
    field: &str,
    value: f32,
) {
    if !value.is_finite() || value < 0.0 {
        push_invalid_metric_denial(denials, &format!("{family}:{node_id}:{field}"));
    }
}

fn validate_elapsed_lanes(
    draft: &WorthUiHostFrameObservationDraft,
    denials: &mut Vec<WorthUiHostObservationAdmissionDenial>,
) {
    let mut elapsed_lanes = BTreeSet::new();
    for row in draft.elapsed_time() {
        if row.lane().is_empty() {
            push_invalid_metric_denial(denials, "elapsed_time:lane");
        }
        if !elapsed_lanes.insert(row.lane().to_owned()) {
            denials.push(WorthUiHostObservationAdmissionDenial::new(
                WorthUiHostObservationAdmissionDenialCode::DuplicateObservationRow,
                format!("elapsed_time:{}", row.lane()),
            ));
        }
    }
}

fn push_invalid_metric_denial(
    denials: &mut Vec<WorthUiHostObservationAdmissionDenial>,
    subject: &str,
) {
    denials.push(WorthUiHostObservationAdmissionDenial::new(
        WorthUiHostObservationAdmissionDenialCode::InvalidMetricBasis,
        subject,
    ));
}

trait HostBoxMetricRow {
    fn node_id(&self) -> &str;
    fn width_points(&self) -> f32;
    fn height_points(&self) -> f32;
}

impl HostBoxMetricRow for crate::runtime::WorthUiHostAvailableBoundsObservation {
    fn node_id(&self) -> &str {
        self.node_id()
    }

    fn width_points(&self) -> f32 {
        self.width_points()
    }

    fn height_points(&self) -> f32 {
        self.height_points()
    }
}

impl HostBoxMetricRow for crate::runtime::WorthUiHostViewportObservation {
    fn node_id(&self) -> &str {
        self.node_id()
    }

    fn width_points(&self) -> f32 {
        self.width_points()
    }

    fn height_points(&self) -> f32 {
        self.height_points()
    }
}
