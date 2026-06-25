use crate::runtime::{WorthUiAdmittedHostFrameObservationReceipt, WorthUiMountedNodeReceipt};

const DEFAULT_HUG_WIDTH: f32 = 160.0;
const DEFAULT_HUG_HEIGHT: f32 = 44.0;

#[derive(Clone, Debug)]
pub(super) struct WorthUiLayoutNaturalMetrics {
    pub(super) width: f32,
    pub(super) height: f32,
    pub(super) baseline: f32,
    pub(super) basis: String,
}

pub(super) fn natural_metrics_for_mounted_node(
    node: &WorthUiMountedNodeReceipt,
    observations: &WorthUiAdmittedHostFrameObservationReceipt,
) -> WorthUiLayoutNaturalMetrics {
    if let Some(metrics) = observed_natural_metrics(node, observations) {
        return metrics;
    }
    fallback_natural_metrics(node)
}

fn observed_natural_metrics(
    node: &WorthUiMountedNodeReceipt,
    observations: &WorthUiAdmittedHostFrameObservationReceipt,
) -> Option<WorthUiLayoutNaturalMetrics> {
    match node {
        WorthUiMountedNodeReceipt::Text(node) => observations
            .text_metrics()
            .iter()
            .find(|row| row.node_id() == node.node_id())
            .map(|row| WorthUiLayoutNaturalMetrics {
                width: row.width_points(),
                height: row.height_points(),
                baseline: row.baseline_points(),
                basis: format!("host_text_metric:{}:{}", row.node_id(), row.text_digest()),
            }),
        WorthUiMountedNodeReceipt::Icon(node) => observations
            .icon_metrics()
            .iter()
            .find(|row| row.node_id() == node.node_id())
            .map(|row| WorthUiLayoutNaturalMetrics {
                width: row.width_points(),
                height: row.height_points(),
                baseline: row.baseline_points(),
                basis: format!("host_icon_metric:{}:{}", row.node_id(), row.icon_digest()),
            }),
        _ => None,
    }
}

fn fallback_natural_metrics(node: &WorthUiMountedNodeReceipt) -> WorthUiLayoutNaturalMetrics {
    let (width, height, baseline, basis) = match node {
        WorthUiMountedNodeReceipt::Control(frame) => {
            let style = frame.style();
            let height = 32.0 + style.padding_top_points() + style.padding_bottom_points();
            (
                180.0 + style.padding_left_points() + style.padding_right_points(),
                height,
                height * 0.8,
                format!("control_host_frame:{}", frame.frame_digest()),
            )
        }
        WorthUiMountedNodeReceipt::Content(node) => {
            let width = node
                .content()
                .items()
                .iter()
                .map(|item| item.width_points())
                .sum::<f32>();
            let height = node
                .content()
                .items()
                .iter()
                .map(|item| item.height_points())
                .fold(0.0, f32::max);
            let baseline = node
                .content()
                .items()
                .iter()
                .map(|item| item.baseline_points())
                .fold(0.0, f32::max);
            (
                width.max(1.0),
                height.max(1.0),
                baseline.max(1.0),
                format!("primitive_content:{}", node.content().receipt_digest()),
            )
        }
        WorthUiMountedNodeReceipt::Interaction(node) => (
            128.0,
            48.0,
            38.4,
            format!("interaction:{}", node.receipt_digest()),
        ),
        WorthUiMountedNodeReceipt::Text(node) => (
            120.0,
            24.0,
            19.2,
            format!("text_default:{}", node.receipt_digest()),
        ),
        WorthUiMountedNodeReceipt::Icon(node) => (
            24.0,
            24.0,
            12.0,
            format!("icon_default:{}", node.receipt_digest()),
        ),
        WorthUiMountedNodeReceipt::FlowContainer(_) | WorthUiMountedNodeReceipt::Surface(_) => (
            DEFAULT_HUG_WIDTH,
            DEFAULT_HUG_HEIGHT,
            DEFAULT_HUG_HEIGHT * 0.8,
            "container_default".to_owned(),
        ),
        WorthUiMountedNodeReceipt::Evidence(_)
        | WorthUiMountedNodeReceipt::DiagnosticPanel(_)
        | WorthUiMountedNodeReceipt::PortalHost(_)
        | WorthUiMountedNodeReceipt::MosaicRegion(_) => (
            DEFAULT_HUG_WIDTH,
            DEFAULT_HUG_HEIGHT,
            DEFAULT_HUG_HEIGHT * 0.8,
            "placeholder_default".to_owned(),
        ),
    };
    WorthUiLayoutNaturalMetrics {
        width,
        height,
        baseline,
        basis,
    }
}
