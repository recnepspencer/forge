mod artifact_identity;
mod digests;
mod invalid_inputs;

use super::{ForgeQuerySessionLabel, ForgeQuerySessionLabelSegment, ForgeQuerySessionNamespace};

fn typed_temporal_preview_label() -> ForgeQuerySessionLabel {
    ForgeQuerySessionLabel::scoped(
        ForgeQuerySessionNamespace::new("worth-kernel").expect("namespace should build"),
        [
            ForgeQuerySessionLabelSegment::new("temporal").expect("segment should build"),
            ForgeQuerySessionLabelSegment::new("preview").expect("segment should build"),
        ],
    )
    .expect("label should build")
}

fn render_collision_labels() -> (ForgeQuerySessionLabel, ForgeQuerySessionLabel) {
    let left = ForgeQuerySessionLabel::scoped(
        ForgeQuerySessionNamespace::new("worth.kernel").expect("namespace should build"),
        [ForgeQuerySessionLabelSegment::new("preview").expect("segment should build")],
    )
    .expect("label should build");
    let right = ForgeQuerySessionLabel::scoped(
        ForgeQuerySessionNamespace::new("worth").expect("namespace should build"),
        [
            ForgeQuerySessionLabelSegment::new("kernel").expect("segment should build"),
            ForgeQuerySessionLabelSegment::new("preview").expect("segment should build"),
        ],
    )
    .expect("label should build");
    (left, right)
}
