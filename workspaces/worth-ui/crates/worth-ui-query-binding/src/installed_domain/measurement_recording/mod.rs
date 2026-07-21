mod definition;
mod executor;
mod operation;

pub(crate) use definition::measurement_recording_definition;
pub(crate) use executor::WorthUiMeasurementRecordingExecutor;
pub use operation::{WorthUiMeasurementRecording, WorthUiMeasurementRecordingFamily};

pub(crate) const LOWERING_FAMILY: &str = "worth-ui-measurement-recording-v1";
pub(crate) const IDENTIFY_STAGE: &str = "identify-measurement";
pub(crate) const RECORD_STAGE: &str = "record-measurement";
