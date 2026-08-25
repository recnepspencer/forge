use worth_ui_host_headless::{UiHeadlessUnperformedEffect, WorthUiHeadlessRecorder};

pub(super) fn assert_translated_cross_lane_frame(
    recorder: &WorthUiHeadlessRecorder,
    frame: worth_ui_runtime::facade::mounted::UiMountedFrameIdentity,
) {
    let transcripts = recorder.observed_transcripts();
    let transcript = transcripts
        .iter()
        .find(|transcript| transcript.frame() == frame)
        .unwrap_or_else(|| {
            panic!(
                "mounted frame {frame:?} has no headless transcript; recorded={:?}",
                transcripts
                    .iter()
                    .map(|transcript| transcript.frame())
                    .collect::<Vec<_>>()
            )
        });
    assert_translated_cross_lane_meaning(transcript);
}

fn assert_translated_cross_lane_meaning(
    transcript: &worth_ui_host_headless::UiHeadlessMountedFrameTranscript,
) {
    assert!(transcript
        .unperformed_effects()
        .iter()
        .any(|effect| matches!(
            effect,
            UiHeadlessUnperformedEffect::CanvasSpatial {
                primitive_count: 64,
                ..
            }
        )));
    assert!(transcript
        .unperformed_effects()
        .iter()
        .any(|effect| matches!(
            effect,
            UiHeadlessUnperformedEffect::Realtime {
                overlay_row_count: 2,
                ..
            }
        )));
}
