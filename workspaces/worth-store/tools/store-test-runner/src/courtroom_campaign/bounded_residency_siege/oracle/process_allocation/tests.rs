use std::num::NonZeroU32;

use super::{verify, BoundedResidencyProcessAllocationObservation};

const PROCESS: NonZeroU32 = NonZeroU32::new(41).unwrap();
const STORE_PAYLOAD_BYTES: u64 = 114_486_784;

#[test]
fn request_observation_accepts_runtime_growth_below_the_complete_store() {
    assert!(verify(observation(52_483_200), PROCESS, STORE_PAYLOAD_BYTES).is_ok());
}

#[test]
fn request_observation_rejects_one_complete_store_request() {
    assert_eq!(
        verify(
            observation(STORE_PAYLOAD_BYTES),
            PROCESS,
            STORE_PAYLOAD_BYTES,
        )
        .unwrap_err(),
        "Courtroom C serving process issued a complete-Store allocation request: \
         request=114486784 payload=114486784"
    );
}

#[test]
fn request_observation_requires_real_nonzero_evidence() {
    assert_eq!(
        verify(observation(0), PROCESS, STORE_PAYLOAD_BYTES).unwrap_err(),
        "Courtroom C omitted serving process-allocation evidence"
    );
}

fn observation(
    largest_successful_request_bytes: u64,
) -> BoundedResidencyProcessAllocationObservation {
    BoundedResidencyProcessAllocationObservation {
        process: PROCESS,
        largest_successful_request_bytes,
    }
}
