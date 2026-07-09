mod artifact_identity;
mod digests;
mod invalid_inputs;

use super::{WorthQuerySessionLabel, WorthQuerySessionLabelSegment, WorthQuerySessionNamespace};

fn typed_temporal_preview_label() -> WorthQuerySessionLabel {
    WorthQuerySessionLabel::scoped(
        WorthQuerySessionNamespace::new("worth-kernel").expect("namespace should build"),
        [
            WorthQuerySessionLabelSegment::new("temporal").expect("segment should build"),
            WorthQuerySessionLabelSegment::new("preview").expect("segment should build"),
        ],
    )
    .expect("label should build")
}

fn render_collision_labels() -> (WorthQuerySessionLabel, WorthQuerySessionLabel) {
    let left = WorthQuerySessionLabel::scoped(
        WorthQuerySessionNamespace::new("worth.kernel").expect("namespace should build"),
        [WorthQuerySessionLabelSegment::new("preview").expect("segment should build")],
    )
    .expect("label should build");
    let right = WorthQuerySessionLabel::scoped(
        WorthQuerySessionNamespace::new("worth").expect("namespace should build"),
        [
            WorthQuerySessionLabelSegment::new("kernel").expect("segment should build"),
            WorthQuerySessionLabelSegment::new("preview").expect("segment should build"),
        ],
    )
    .expect("label should build");
    (left, right)
}
