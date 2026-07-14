use worth_store_budgets::{AllocationEnvelopeSet, CounterEvidenceStrength};
use worth_store_buffer_pool::AllocationReceipt;

use super::super::transitions::{admit_read, finish_verified_read, observe_chunk_window};
use super::super::types::BlobStreamingVerifiedRead;
use super::super::verification::{counter_strength, StreamingReadVerifier};
use crate::{
    BlobQuarantineAuthority, BlobStreamingReadAdmission, BlobStreamingReadDenial,
    BlobStreamingReadObservation, BlobStreamingReadRequest, BlobStreamingReadWindow,
};

impl BlobStreamingVerifiedRead {
    pub fn verify_bounded(
        request: BlobStreamingReadRequest,
        window: BlobStreamingReadWindow,
        allocation: AllocationReceipt,
        envelopes: AllocationEnvelopeSet,
        admission: BlobStreamingReadAdmission,
        quarantine_authority: BlobQuarantineAuthority,
        observations: impl IntoIterator<Item = BlobStreamingReadObservation>,
        counter_strength: CounterEvidenceStrength,
    ) -> Result<Self, BlobStreamingReadDenial> {
        counter_strength::require_exact(counter_strength)?;
        let mut counters =
            admit_read::admit_read(admission, &request, allocation, envelopes, counter_strength)?;
        let mut verifier = StreamingReadVerifier::new(request, window, quarantine_authority);
        for observation in observations {
            observe_chunk_window::observe_chunk_window(&mut verifier, observation, &mut counters)?;
        }
        finish_verified_read::finish_verified_read(verifier, counters)
    }
}
