use std::path::Path;

use worth_store_test_support::structural_preflight::{
    PreflightEvidenceFreshness, StructuralPredicateFailure, StructuralPreflightEvidence,
    StructuralPreflightPlan,
};

use super::plan;

pub fn require_fresh(
    forge_root: &Path,
    evidence: &StructuralPreflightEvidence,
) -> Result<PreflightEvidenceFreshness, String> {
    evidence
        .validate_integrity()
        .map_err(|denial| denial.to_string())?;
    let current = plan::build(forge_root, evidence.plan.request.clone())?;
    compare(evidence, &current)
}

pub(super) fn compare(
    evidence: &StructuralPreflightEvidence,
    current: &StructuralPreflightPlan,
) -> Result<PreflightEvidenceFreshness, String> {
    evidence
        .validate_integrity()
        .map_err(|denial| denial.to_string())?;
    current
        .validate_integrity()
        .map_err(|denial| denial.to_string())?;
    let mut failures = Vec::new();
    for declared in &evidence.plan.predicates {
        let Some(observed) = current
            .predicates
            .iter()
            .find(|candidate| candidate.predicate == declared.predicate)
        else {
            failures.push(StructuralPredicateFailure {
                predicate: declared.predicate,
                failure_code: "predicate_removed".to_owned(),
                message: "fresh preflight plan omitted a declared predicate".to_owned(),
                invalidated_inputs: Vec::new(),
            });
            continue;
        };
        let mut invalidated_inputs = Vec::new();
        for scope in &declared.input_scopes {
            let refreshed = observed
                .input_scopes
                .iter()
                .find(|candidate| candidate.scope_identity == scope.scope_identity);
            if refreshed.is_none_or(|refreshed| {
                refreshed.input_identity != scope.input_identity
                    || refreshed.source_paths != scope.source_paths
                    || refreshed.included_extensions != scope.included_extensions
            }) {
                invalidated_inputs.push(scope.scope_identity.clone());
            }
        }
        if observed.input_scopes.len() != declared.input_scopes.len() {
            invalidated_inputs.push("predicate-input-scope-set".to_owned());
        }
        if observed.tool != declared.tool {
            invalidated_inputs.push(declared.tool.as_ref().map_or_else(
                || "tool-declaration".to_owned(),
                |tool| format!("tool: {}", tool.tool_identity),
            ));
        }
        if !invalidated_inputs.is_empty() {
            failures.push(StructuralPredicateFailure {
                predicate: declared.predicate,
                failure_code: "stale_evidence".to_owned(),
                message: format!("preflight inputs changed for {:?}", declared.predicate),
                invalidated_inputs,
            });
        }
    }
    if failures.is_empty() {
        Ok(PreflightEvidenceFreshness::Fresh {
            evidence_identity: evidence.evidence_identity.clone(),
        })
    } else {
        Ok(PreflightEvidenceFreshness::Stale { failures })
    }
}
