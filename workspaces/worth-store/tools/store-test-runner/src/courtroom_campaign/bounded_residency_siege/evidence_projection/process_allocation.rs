use serde_json::{json, Value};

use super::super::protocol::BoundedResidencyProcessAllocationObservation;

pub(super) fn value(observation: BoundedResidencyProcessAllocationObservation) -> Value {
    json!({
        "process": observation.process.get(),
        "largest_successful_request_bytes": observation.largest_successful_request_bytes,
    })
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;

    use super::{value, BoundedResidencyProcessAllocationObservation};

    #[test]
    fn projection_preserves_each_process_request_observation() {
        let projected = value(BoundedResidencyProcessAllocationObservation {
            process: NonZeroU32::new(41).unwrap(),
            largest_successful_request_bytes: 8_388_608,
        });

        assert_eq!(projected["process"], 41);
        assert_eq!(projected["largest_successful_request_bytes"], 8_388_608);
    }
}
