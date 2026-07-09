use crate::diagnostics::BridgeMergeExplanation;
use crate::routing::canonicalization::digest_string;

pub(super) fn merge_diagnostics_digest(explanation: &BridgeMergeExplanation) -> String {
    digest_string(
        "merge-diagnostics-digest",
        &format!(
            "record={}|contract={}|lowered={}|reduced={}|continuity={}|remap={}|explanation={}|outcome={:?}|blocked_stage={:?}|denial={:?}",
            explanation.record_identity().as_str(),
            explanation.contract_identity(),
            explanation.lowered_digest(),
            explanation.reduced_digest(),
            explanation.continuity_digest().unwrap_or("none"),
            explanation.remap_digest().unwrap_or("none"),
            explanation.explanation_digest(),
            explanation.outcome_class(),
            explanation.blocked_stage(),
            explanation.denial_class(),
        ),
    )
    .to_string()
}
