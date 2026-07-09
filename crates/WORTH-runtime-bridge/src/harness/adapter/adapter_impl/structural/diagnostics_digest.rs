use super::*;
use crate::routing::canonicalization::digest_string;

pub(super) fn remap_diagnostics_digest(record: &BridgeCanonicalStructuralRemapRecord) -> String {
    let explanation =
        crate::diagnostics::BridgeStructuralRemapExplanation::from_canonical_record(record);
    digest_string(
        "structural-remap-diagnostics-digest",
        &format!(
            "record={}|declaration={}|family={:?}|version={}|outcome={:?}|artifact={}",
            explanation.record_identity().as_str(),
            explanation.declaration_identity(),
            explanation.fingerprint_family(),
            explanation.semantics_version(),
            explanation.outcome_class(),
            explanation.artifact_digest(),
        ),
    )
    .to_string()
}

pub(super) fn branch_diagnostics_digest(
    record: &BridgeCanonicalStructuralBranchComparisonRecord,
) -> String {
    let explanation =
        crate::diagnostics::BridgeStructuralBranchComparisonExplanation::from_canonical_record(
            record,
        );
    digest_string(
        "structural-branch-diagnostics-digest",
        &format!(
            "record={}|declaration={}|family={:?}|version={}|branch-diffs={}|artifact={}",
            explanation.record_identity().as_str(),
            explanation.declaration_identity(),
            explanation.fingerprint_family(),
            explanation.semantics_version(),
            explanation.branch_diff_count(),
            explanation.artifact_digest(),
        ),
    )
    .to_string()
}

pub(super) fn rejection_diagnostics_digest(
    contract: &AdmittedStructuralComparisonContract,
    planned: &PlannedStructuralMatchPacketSet,
    reduced: &ReducedStructuralMatchSet,
) -> String {
    digest_string(
        "structural-rejection-diagnostics-digest",
        &format!(
            "declaration={}|planned={}|reduced={}|outcome={:?}",
            declaration_identity(contract).as_str(),
            planned.digest(),
            reduced.digest(),
            reduced.outcome_class(),
        ),
    )
    .to_string()
}
