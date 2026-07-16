use worth_store_budgets::{AllocationEnvelopeSet, CounterEvidenceStrength};
use worth_store_buffer_pool::AllocationReceipt;

use super::super::transitions::{admit_read, finish_verified_read, observe_chunk_window};
use super::super::types::BlobStreamingVerifiedRead;
use super::super::verification::{counter_strength, StreamingReadVerifier};
use crate::{
    BlobQuarantineAuthority, BlobStreamingReadAdmission, BlobStreamingReadDenial,
    BlobStreamingReadObservation, BlobStreamingReadRequest, BlobStreamingReadWindow,
};

pub struct BlobStreamingReadExecution {
    window: BlobStreamingReadWindow,
    allocation: AllocationReceipt,
    envelopes: AllocationEnvelopeSet,
    admission: BlobStreamingReadAdmission,
    quarantine_authority: BlobQuarantineAuthority,
    counter_strength: CounterEvidenceStrength,
}

impl BlobStreamingReadExecution {
    pub fn new(
        window: BlobStreamingReadWindow,
        allocation: AllocationReceipt,
        envelopes: AllocationEnvelopeSet,
        admission: BlobStreamingReadAdmission,
        quarantine_authority: BlobQuarantineAuthority,
        counter_strength: CounterEvidenceStrength,
    ) -> Self {
        Self {
            window,
            allocation,
            envelopes,
            admission,
            quarantine_authority,
            counter_strength,
        }
    }
}

impl BlobStreamingVerifiedRead {
    pub fn verify_bounded(
        request: BlobStreamingReadRequest,
        execution: BlobStreamingReadExecution,
        observations: impl IntoIterator<Item = BlobStreamingReadObservation>,
    ) -> Result<Self, BlobStreamingReadDenial> {
        counter_strength::require_exact(execution.counter_strength)?;
        let mut counters = admit_read::admit_read(
            execution.admission,
            &request,
            execution.allocation,
            execution.envelopes,
            execution.counter_strength,
        )?;
        let mut verifier =
            StreamingReadVerifier::new(request, execution.window, execution.quarantine_authority);
        for observation in observations {
            observe_chunk_window::observe_chunk_window(&mut verifier, observation, &mut counters)?;
        }
        finish_verified_read::finish_verified_read(verifier, counters)
    }
}
