use forge_store_budgets::CounterEvidenceStrength;

use crate::BlobStreamingReadDenial;

pub(crate) fn require_exact(
    counter_strength: CounterEvidenceStrength,
) -> Result<(), BlobStreamingReadDenial> {
    if counter_strength.satisfies(CounterEvidenceStrength::Exact) {
        Ok(())
    } else {
        Err(BlobStreamingReadDenial::MissingExactCounters {
            actual: counter_strength,
        })
    }
}