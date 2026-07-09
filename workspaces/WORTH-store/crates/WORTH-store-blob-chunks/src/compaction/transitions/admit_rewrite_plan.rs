use crate::compaction::classification::{
    assemble_compaction_denial, classify_compaction_eligibility, CompactionEligibilityCase,
};
use crate::compaction::receipt_construction::rewrite_plan::{
    base_counters, construct_rewrite_plan,
};
use crate::compaction::types::{BlobCompactionIntent, BlobCompactionRewritePlan};
use crate::BlobCompactionDenial;

pub(crate) fn admit(
    intent: BlobCompactionIntent,
) -> Result<BlobCompactionRewritePlan, BlobCompactionDenial> {
    let counters = base_counters(&intent);
    let case = classify_compaction_eligibility(&intent);
    if case != CompactionEligibilityCase::Admit {
        return Err(assemble_compaction_denial(case, &intent, counters));
    }
    Ok(construct_rewrite_plan(intent, counters))
}
