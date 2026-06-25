use super::basis::WorthUiHostObservationBasis;

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiHostFrameObservationDraft {
    basis: WorthUiHostObservationBasis,
    available_bounds: Vec<WorthUiHostAvailableBoundsObservation>,
    viewports: Vec<WorthUiHostViewportObservation>,
    scroll_viewports: Vec<WorthUiHostScrollViewportObservation>,
    text_metrics: Vec<WorthUiHostTextMetricObservation>,
    icon_metrics: Vec<WorthUiHostIconMetricObservation>,
    dpi_scale: Option<f32>,
    elapsed_time: Vec<WorthUiHostElapsedTimeObservation>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiHostAvailableBoundsObservation {
    node_id: String,
    width_points: f32,
    height_points: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiHostViewportObservation {
    node_id: String,
    width_points: f32,
    height_points: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiHostScrollViewportObservation {
    node_id: String,
    scroll_x_points: f32,
    scroll_y_points: f32,
    width_points: f32,
    height_points: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiHostTextMetricObservation {
    node_id: String,
    text_digest: u64,
    width_points: f32,
    height_points: f32,
    baseline_points: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiHostIconMetricObservation {
    node_id: String,
    icon_digest: u64,
    width_points: f32,
    height_points: f32,
    baseline_points: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiHostElapsedTimeObservation {
    lane: String,
    elapsed_micros: u64,
}

impl WorthUiHostFrameObservationDraft {
    pub fn for_mounted_product_view(mounted_product_view_digest: u64, frame_epoch: u64) -> Self {
        Self {
            basis: WorthUiHostObservationBasis::new(mounted_product_view_digest, frame_epoch),
            available_bounds: Vec::new(),
            viewports: Vec::new(),
            scroll_viewports: Vec::new(),
            text_metrics: Vec::new(),
            icon_metrics: Vec::new(),
            dpi_scale: None,
            elapsed_time: Vec::new(),
        }
    }

    pub fn observe_available_bounds(
        mut self,
        node_id: impl Into<String>,
        width_points: f32,
        height_points: f32,
    ) -> Self {
        self.available_bounds
            .push(WorthUiHostAvailableBoundsObservation {
                node_id: node_id.into(),
                width_points,
                height_points,
            });
        self
    }

    pub fn observe_viewport(
        mut self,
        node_id: impl Into<String>,
        width_points: f32,
        height_points: f32,
    ) -> Self {
        self.viewports.push(WorthUiHostViewportObservation {
            node_id: node_id.into(),
            width_points,
            height_points,
        });
        self
    }

    pub fn observe_scroll_viewport(
        mut self,
        node_id: impl Into<String>,
        scroll_x_points: f32,
        scroll_y_points: f32,
        width_points: f32,
        height_points: f32,
    ) -> Self {
        self.scroll_viewports
            .push(WorthUiHostScrollViewportObservation {
                node_id: node_id.into(),
                scroll_x_points,
                scroll_y_points,
                width_points,
                height_points,
            });
        self
    }

    pub fn observe_text_metric(
        mut self,
        node_id: impl Into<String>,
        text_digest: u64,
        width_points: f32,
        height_points: f32,
        baseline_points: f32,
    ) -> Self {
        self.text_metrics.push(WorthUiHostTextMetricObservation {
            node_id: node_id.into(),
            text_digest,
            width_points,
            height_points,
            baseline_points,
        });
        self
    }

    pub fn observe_icon_metric(
        mut self,
        node_id: impl Into<String>,
        icon_digest: u64,
        width_points: f32,
        height_points: f32,
        baseline_points: f32,
    ) -> Self {
        self.icon_metrics.push(WorthUiHostIconMetricObservation {
            node_id: node_id.into(),
            icon_digest,
            width_points,
            height_points,
            baseline_points,
        });
        self
    }

    pub fn observe_dpi(mut self, dpi_scale: f32) -> Self {
        self.dpi_scale = Some(dpi_scale);
        self
    }

    pub fn observe_elapsed_time(mut self, lane: impl Into<String>, elapsed_micros: u64) -> Self {
        self.elapsed_time.push(WorthUiHostElapsedTimeObservation {
            lane: lane.into(),
            elapsed_micros,
        });
        self
    }

    pub fn basis(&self) -> &WorthUiHostObservationBasis {
        &self.basis
    }

    pub(crate) fn available_bounds(&self) -> &[WorthUiHostAvailableBoundsObservation] {
        &self.available_bounds
    }

    pub(crate) fn viewports(&self) -> &[WorthUiHostViewportObservation] {
        &self.viewports
    }

    pub(crate) fn scroll_viewports(&self) -> &[WorthUiHostScrollViewportObservation] {
        &self.scroll_viewports
    }

    pub(crate) fn text_metrics(&self) -> &[WorthUiHostTextMetricObservation] {
        &self.text_metrics
    }

    pub(crate) fn icon_metrics(&self) -> &[WorthUiHostIconMetricObservation] {
        &self.icon_metrics
    }

    pub(crate) fn dpi_scale(&self) -> Option<f32> {
        self.dpi_scale
    }

    pub(crate) fn elapsed_time(&self) -> &[WorthUiHostElapsedTimeObservation] {
        &self.elapsed_time
    }
}

macro_rules! observation_getters {
    ($ty:ty, $($name:ident -> $ret:ty),+ $(,)?) => {
        impl $ty {
            $(pub fn $name(&self) -> $ret { self.$name })+
        }
    };
}

impl WorthUiHostAvailableBoundsObservation {
    pub fn node_id(&self) -> &str {
        &self.node_id
    }
}
observation_getters!(WorthUiHostAvailableBoundsObservation, width_points -> f32, height_points -> f32);

impl WorthUiHostViewportObservation {
    pub fn node_id(&self) -> &str {
        &self.node_id
    }
}
observation_getters!(WorthUiHostViewportObservation, width_points -> f32, height_points -> f32);

impl WorthUiHostScrollViewportObservation {
    pub fn node_id(&self) -> &str {
        &self.node_id
    }
}
observation_getters!(
    WorthUiHostScrollViewportObservation,
    scroll_x_points -> f32,
    scroll_y_points -> f32,
    width_points -> f32,
    height_points -> f32,
);

impl WorthUiHostTextMetricObservation {
    pub fn node_id(&self) -> &str {
        &self.node_id
    }
}
observation_getters!(
    WorthUiHostTextMetricObservation,
    text_digest -> u64,
    width_points -> f32,
    height_points -> f32,
    baseline_points -> f32,
);

impl WorthUiHostIconMetricObservation {
    pub fn node_id(&self) -> &str {
        &self.node_id
    }
}
observation_getters!(
    WorthUiHostIconMetricObservation,
    icon_digest -> u64,
    width_points -> f32,
    height_points -> f32,
    baseline_points -> f32,
);

impl WorthUiHostElapsedTimeObservation {
    pub fn lane(&self) -> &str {
        &self.lane
    }

    pub fn elapsed_micros(&self) -> u64 {
        self.elapsed_micros
    }
}
