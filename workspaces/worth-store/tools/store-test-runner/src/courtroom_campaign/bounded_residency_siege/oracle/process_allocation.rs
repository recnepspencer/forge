use std::num::NonZeroU32;

use super::super::protocol::BoundedResidencyProcessAllocationObservation;

pub(super) fn verify(
    observation: BoundedResidencyProcessAllocationObservation,
    expected_process: NonZeroU32,
    store_payload_bytes: u64,
) -> Result<(), String> {
    if observation.process != expected_process {
        return Err("Courtroom C process-allocation evidence named a foreign process".into());
    }
    if observation.largest_successful_request_bytes == 0 {
        return Err("Courtroom C omitted serving process-allocation evidence".into());
    }
    if observation.largest_successful_request_bytes >= store_payload_bytes {
        return Err(format!(
            "Courtroom C serving process issued a complete-Store allocation request: \
             request={} payload={store_payload_bytes}",
            observation.largest_successful_request_bytes
        ));
    }
    Ok(())
}

#[cfg(test)]
#[path = "process_allocation/tests.rs"]
mod tests;
