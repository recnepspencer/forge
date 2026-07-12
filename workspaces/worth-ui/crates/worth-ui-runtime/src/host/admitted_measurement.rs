use crate::evidence::{UiCurrentMeasurementResult, UiMeasurementResult};

/// Host measurement truth after freshness and source-position admission.
#[derive(Debug, PartialEq)]
pub struct UiAdmittedHostMeasurement {
    result: UiMeasurementResult,
    source_identity: u64,
    source_generation: u64,
    source_order: u64,
}

impl UiAdmittedHostMeasurement {
    pub(crate) fn from_collected(result: UiMeasurementResult) -> Self {
        let (source_identity, source_generation, source_order) = result.host_source_position();
        Self {
            result,
            source_identity,
            source_generation,
            source_order,
        }
    }

    pub fn from_current(current: UiCurrentMeasurementResult<'_>) -> Self {
        Self::from_collected(current.to_owned_result())
    }

    pub fn result(&self) -> &UiMeasurementResult {
        &self.result
    }

    pub fn source_identity(&self) -> u64 {
        self.source_identity
    }

    pub fn source_generation(&self) -> u64 {
        self.source_generation
    }

    pub fn source_order(&self) -> u64 {
        self.source_order
    }
}

#[cfg(test)]
impl Clone for UiAdmittedHostMeasurement {
    fn clone(&self) -> Self {
        Self {
            result: self.result.clone(),
            source_identity: self.source_identity,
            source_generation: self.source_generation,
            source_order: self.source_order,
        }
    }
}
