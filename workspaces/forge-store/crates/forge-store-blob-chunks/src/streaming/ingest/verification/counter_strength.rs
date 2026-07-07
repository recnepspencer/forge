use forge_store_budgets::CounterEvidenceStrength;

use crate::BlobStreamingIngestDenial;

pub(crate) fn require_exact(
    counter_strength: CounterEvidenceStrength,
) -> Result<(), BlobStreamingIngestDenial> {
    if counter_strength.satisfies(CounterEvidenceStrength::Exact) {
        Ok(())
    } else {
        Err(BlobStreamingIngestDenial::MissingExactCounters {
            actual: counter_strength,
        })
    }
}
