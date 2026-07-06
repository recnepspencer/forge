mod compaction_decision_table;
mod compaction_eligibility_case;

pub(crate) use compaction_decision_table::{
    assemble_compaction_denial, classify_compaction_eligibility,
};
pub(crate) use compaction_eligibility_case::CompactionEligibilityCase;