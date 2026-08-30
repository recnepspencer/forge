use std::fs::OpenOptions;
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;

use serde::Serialize;
use worth_ui_native_platform::{
    UiNativePlatformCloseReceipt, UiNativePresentationWorkKind, UiNativeRetainedFrameObservation,
};

const EVIDENCE_PATH_ENVIRONMENT: &str = "WORTH_UI_EXECUTABLE_WORLD_NATIVE_CLOSE_EVIDENCE";

#[derive(Serialize)]
struct NativeCloseEvidence {
    schema_version: u8,
    sample_frames: Vec<NativeSampleFrameEvidence>,
}

#[derive(Serialize)]
struct NativeSampleFrameEvidence {
    frame: u64,
    presentation_epoch: Option<u64>,
    presentation_attempt: Option<u64>,
    retained_baseline_rgba8: [u8; 4],
    retained_center_rgba8: [u8; 4],
    logical_damage_regions: u64,
    rendered_pixels: u64,
    queue_submissions: u64,
    presents: u64,
}

pub(crate) fn write(receipt: &UiNativePlatformCloseReceipt) -> io::Result<()> {
    let Some(path) = std::env::var_os(EVIDENCE_PATH_ENVIRONMENT).map(PathBuf::from) else {
        return Ok(());
    };
    let evidence = NativeCloseEvidence {
        schema_version: 1,
        sample_frames: receipt
            .retained_frames()
            .iter()
            .filter(|frame| frame.kind() == UiNativePresentationWorkKind::Sample)
            .map(sample_frame_evidence)
            .collect(),
    };
    let file = OpenOptions::new().create_new(true).write(true).open(path)?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &evidence).map_err(io::Error::other)?;
    writer.flush()
}

fn sample_frame_evidence(frame: &UiNativeRetainedFrameObservation) -> NativeSampleFrameEvidence {
    let cost = frame.cost();
    NativeSampleFrameEvidence {
        frame: frame.frame(),
        presentation_epoch: frame
            .sample_presentation_epoch()
            .map(|epoch| epoch.diagnostic_value()),
        presentation_attempt: frame
            .presentation()
            .map(|presentation| presentation.presentation_attempt()),
        retained_baseline_rgba8: frame.retained_baseline_rgba8(),
        retained_center_rgba8: frame.retained_center_rgba8(),
        logical_damage_regions: cost.logical_damage_regions(),
        rendered_pixels: cost.rendered_pixels(),
        queue_submissions: cost.queue_submissions(),
        presents: cost.presents(),
    }
}
