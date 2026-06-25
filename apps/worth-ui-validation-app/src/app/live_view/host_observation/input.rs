use worth_ui::facade::WorthUiHostFrameObservationDraft;

#[derive(Clone, Debug, PartialEq)]
pub struct ValidationHostObservationInput {
    width_points: f32,
    height_points: f32,
    frame_epoch: u64,
    dpi_scale: Option<f32>,
    scroll_viewports: Vec<ValidationHostScrollViewportInput>,
    text_metrics: Vec<ValidationHostTextMetricInput>,
    icon_metrics: Vec<ValidationHostIconMetricInput>,
    elapsed_time: Vec<ValidationHostElapsedTimeInput>,
}

#[derive(Clone, Debug, PartialEq)]
struct ValidationHostScrollViewportInput {
    node_id: String,
    scroll_x_points: f32,
    scroll_y_points: f32,
    width_points: f32,
    height_points: f32,
}

#[derive(Clone, Debug, PartialEq)]
struct ValidationHostTextMetricInput {
    node_id: String,
    text_digest: u64,
    width_points: f32,
    height_points: f32,
    baseline_points: f32,
}

#[derive(Clone, Debug, PartialEq)]
struct ValidationHostIconMetricInput {
    node_id: String,
    icon_digest: u64,
    width_points: f32,
    height_points: f32,
    baseline_points: f32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ValidationHostElapsedTimeInput {
    lane: String,
    elapsed_micros: u64,
}

impl ValidationHostObservationInput {
    pub fn new(width_points: f32, height_points: f32, frame_epoch: u64) -> Self {
        Self {
            width_points,
            height_points,
            frame_epoch,
            dpi_scale: None,
            scroll_viewports: Vec::new(),
            text_metrics: Vec::new(),
            icon_metrics: Vec::new(),
            elapsed_time: Vec::new(),
        }
    }

    pub fn with_dpi_scale(mut self, dpi_scale: f32) -> Self {
        self.dpi_scale = Some(dpi_scale);
        self
    }

    pub fn with_scroll_viewport(
        mut self,
        node_id: impl Into<String>,
        scroll_x_points: f32,
        scroll_y_points: f32,
        width_points: f32,
        height_points: f32,
    ) -> Self {
        self.scroll_viewports
            .push(ValidationHostScrollViewportInput {
                node_id: node_id.into(),
                scroll_x_points,
                scroll_y_points,
                width_points,
                height_points,
            });
        self
    }

    pub fn with_text_metric(
        mut self,
        node_id: impl Into<String>,
        text_digest: u64,
        width_points: f32,
        height_points: f32,
        baseline_points: f32,
    ) -> Self {
        self.text_metrics.push(ValidationHostTextMetricInput {
            node_id: node_id.into(),
            text_digest,
            width_points,
            height_points,
            baseline_points,
        });
        self
    }

    pub fn with_icon_metric(
        mut self,
        node_id: impl Into<String>,
        icon_digest: u64,
        width_points: f32,
        height_points: f32,
        baseline_points: f32,
    ) -> Self {
        self.icon_metrics.push(ValidationHostIconMetricInput {
            node_id: node_id.into(),
            icon_digest,
            width_points,
            height_points,
            baseline_points,
        });
        self
    }

    pub fn with_elapsed_time(mut self, lane: impl Into<String>, elapsed_micros: u64) -> Self {
        self.elapsed_time.push(ValidationHostElapsedTimeInput {
            lane: lane.into(),
            elapsed_micros,
        });
        self
    }

    pub(super) fn into_draft(
        self,
        mounted_product_view_digest: u64,
        surface_node_id: String,
    ) -> WorthUiHostFrameObservationDraft {
        let mut draft =
            self.root_frame_observation_draft(mounted_product_view_digest, surface_node_id);
        draft = self.apply_optional_dpi_observation(draft);
        draft = self.apply_scroll_viewport_observations(draft);
        draft = self.apply_text_metric_observations(draft);
        draft = self.apply_icon_metric_observations(draft);
        self.apply_elapsed_time_observations(draft)
    }

    fn root_frame_observation_draft(
        &self,
        mounted_product_view_digest: u64,
        surface_node_id: String,
    ) -> WorthUiHostFrameObservationDraft {
        WorthUiHostFrameObservationDraft::for_mounted_product_view(
            mounted_product_view_digest,
            self.frame_epoch,
        )
        .observe_available_bounds(
            surface_node_id.clone(),
            self.width_points,
            self.height_points,
        )
        .observe_viewport(surface_node_id, self.width_points, self.height_points)
    }

    fn apply_optional_dpi_observation(
        &self,
        draft: WorthUiHostFrameObservationDraft,
    ) -> WorthUiHostFrameObservationDraft {
        self.dpi_scale
            .map_or(draft.clone(), |dpi_scale| draft.observe_dpi(dpi_scale))
    }

    fn apply_scroll_viewport_observations(
        &self,
        mut draft: WorthUiHostFrameObservationDraft,
    ) -> WorthUiHostFrameObservationDraft {
        for row in &self.scroll_viewports {
            draft = draft.observe_scroll_viewport(
                row.node_id.clone(),
                row.scroll_x_points,
                row.scroll_y_points,
                row.width_points,
                row.height_points,
            );
        }
        draft
    }

    fn apply_text_metric_observations(
        &self,
        mut draft: WorthUiHostFrameObservationDraft,
    ) -> WorthUiHostFrameObservationDraft {
        for row in &self.text_metrics {
            draft = draft.observe_text_metric(
                row.node_id.clone(),
                row.text_digest,
                row.width_points,
                row.height_points,
                row.baseline_points,
            );
        }
        draft
    }

    fn apply_icon_metric_observations(
        &self,
        mut draft: WorthUiHostFrameObservationDraft,
    ) -> WorthUiHostFrameObservationDraft {
        for row in &self.icon_metrics {
            draft = draft.observe_icon_metric(
                row.node_id.clone(),
                row.icon_digest,
                row.width_points,
                row.height_points,
                row.baseline_points,
            );
        }
        draft
    }

    fn apply_elapsed_time_observations(
        self,
        mut draft: WorthUiHostFrameObservationDraft,
    ) -> WorthUiHostFrameObservationDraft {
        for row in self.elapsed_time {
            draft = draft.observe_elapsed_time(row.lane, row.elapsed_micros);
        }
        draft
    }
}
