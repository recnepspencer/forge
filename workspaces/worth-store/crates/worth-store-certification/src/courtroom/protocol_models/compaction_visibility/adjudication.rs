use worth_store_formal_models::{
    current_compaction_visibility_owner_cases, require_compaction_visibility_refinement_coverage,
    CompactionVisibilityRefinementCoverageDenial,
};

use super::{
    scenarios::execute_compaction_visibility_owner_cases, CompactionVisibilityRefinementEvidence,
};

pub fn adjudicate_compaction_visibility_refinement(
) -> Result<CompactionVisibilityRefinementEvidence, CompactionVisibilityRefinementCoverageDenial> {
    let execution = execute_compaction_visibility_owner_cases();
    let exact_coverage = require_compaction_visibility_refinement_coverage(
        current_compaction_visibility_owner_cases(),
        execution.owner_cases(),
        execution.mapped_cases(),
    )?;
    Ok(CompactionVisibilityRefinementEvidence::from_execution(
        exact_coverage,
        execution,
    ))
}
