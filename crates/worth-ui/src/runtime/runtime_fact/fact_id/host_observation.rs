use super::{WorthUiRuntimeFactFamily, WorthUiRuntimeFactId};

impl WorthUiRuntimeFactId {
    pub fn host_measurement_observation(identity: impl Into<String>) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::HostMeasurementObservation,
            identity,
        )
    }
}
