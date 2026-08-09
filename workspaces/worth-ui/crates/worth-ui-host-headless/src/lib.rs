//! Contract-only headless mechanics and record-only presentation evidence.

mod headless_host;
mod headless_measurement;
mod headless_recorder;
#[cfg(test)]
mod headless_static_paint_tests;
mod headless_transcript;
mod headless_translation;
#[cfg(test)]
mod headless_translation_effect_tests;

pub use headless_host::WorthUiHeadlessHost;
pub use headless_recorder::WorthUiHeadlessRecorder;
pub use headless_transcript::{
    UiHeadlessClipMechanic, UiHeadlessFilledRectMechanic, UiHeadlessLayerMechanic,
    UiHeadlessMountedFrameTranscript, UiHeadlessNodeMechanic, UiHeadlessNodePaintMechanic,
    UiHeadlessPaintBatchMechanic, UiHeadlessRecorderCapacity, UiHeadlessResolvedClip,
    UiHeadlessResourceContact, UiHeadlessSemanticTextMechanic, UiHeadlessUnperformedEffect,
};
