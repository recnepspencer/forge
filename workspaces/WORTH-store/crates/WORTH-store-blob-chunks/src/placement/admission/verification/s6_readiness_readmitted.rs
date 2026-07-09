use worth_store_io_scheduler::S6LaterReadinessReadmissionState;

use crate::placement::admission::{
    BlobPlacementAdmissionDenial, BlobPlacementCounterSnapshot, BlobPlacementIntent,
};

pub(crate) fn verify_s6_readiness_readmitted(
    intent: &BlobPlacementIntent,
) -> Result<(), BlobPlacementAdmissionDenial> {
    let readmission = intent.readiness().handoff().readmission_state();
    if readmission != S6LaterReadinessReadmissionState::ReadmittedAfterPublication {
        return Err(BlobPlacementAdmissionDenial::StaleS6Readiness {
            readmission,
            counters: BlobPlacementCounterSnapshot::for_class(intent.class()),
        });
    }
    Ok(())
}
