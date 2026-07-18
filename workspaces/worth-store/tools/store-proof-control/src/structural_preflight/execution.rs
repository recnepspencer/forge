use std::path::Path;

use worth_store_test_support::structural_preflight::{
    PreflightEvidenceIdentity, StructuralPredicateEvidence, StructuralPredicatePlan,
    StructuralPredicateVerdict, StructuralPreflightEvidence, StructuralPreflightPlan,
};

use crate::evidence::sha256_serialized;
use crate::ValidatedProofInventory;

use super::{predicate_evaluation, tool_execution, RepositoryPredicateFailure};

pub(super) fn execute(
    forge_root: &Path,
    plan: StructuralPreflightPlan,
    inventory: Option<&ValidatedProofInventory>,
    validation_failure: Option<&RepositoryPredicateFailure>,
) -> Result<StructuralPreflightEvidence, String> {
    execute_with_refresh(forge_root, plan, inventory, validation_failure, |plan| {
        super::plan::build(forge_root, plan.request.clone())
    })
}

#[cfg(test)]
pub(super) fn execute_with_stable_test_plan(
    forge_root: &Path,
    plan: StructuralPreflightPlan,
    inventory: Option<&ValidatedProofInventory>,
    validation_failure: Option<&RepositoryPredicateFailure>,
) -> Result<StructuralPreflightEvidence, String> {
    execute_with_refresh(forge_root, plan, inventory, validation_failure, |plan| {
        Ok(plan.clone())
    })
}

fn execute_with_refresh(
    forge_root: &Path,
    plan: StructuralPreflightPlan,
    inventory: Option<&ValidatedProofInventory>,
    validation_failure: Option<&RepositoryPredicateFailure>,
    refresh: impl FnOnce(&StructuralPreflightPlan) -> Result<StructuralPreflightPlan, String>,
) -> Result<StructuralPreflightEvidence, String> {
    let tools = tool_execution::execute_once(forge_root, &plan);
    let refreshed = refresh(&plan)?;
    let mut predicates = Vec::with_capacity(plan.predicates.len());
    for predicate in &plan.predicates {
        let input_identity = sha256_serialized(
            &predicate
                .input_scopes
                .iter()
                .map(|scope| (&scope.scope_identity, &scope.input_identity))
                .collect::<Vec<_>>(),
        )?;
        let tool_outcome = predicate
            .tool
            .as_ref()
            .and_then(|tool| tools.get(&tool_execution::command_identity(tool)));
        let verdict =
            input_drift_failure(predicate, &refreshed, plan.evaluator != refreshed.evaluator)
                .unwrap_or(predicate_evaluation::evaluate(
                    forge_root,
                    predicate.predicate,
                    &input_identity,
                    inventory,
                    validation_failure,
                    tool_outcome,
                )?);
        predicates.push(StructuralPredicateEvidence {
            predicate: predicate.predicate,
            input_identity,
            tool_identity: tool_outcome.map(|outcome| outcome.identity().to_owned()),
            verdict,
        });
    }
    let mut evidence = StructuralPreflightEvidence {
        schema_version: 1,
        plan,
        predicates,
        tool_executions: tool_execution::evidence(&tools),
        evidence_identity: PreflightEvidenceIdentity(String::new()),
    };
    evidence.evidence_identity = PreflightEvidenceIdentity(sha256_serialized(&evidence)?);
    Ok(evidence)
}

fn input_drift_failure(
    declared: &StructuralPredicatePlan,
    refreshed: &StructuralPreflightPlan,
    evaluator_changed: bool,
) -> Option<StructuralPredicateVerdict> {
    let current = refreshed
        .predicates
        .iter()
        .find(|candidate| candidate.predicate == declared.predicate);
    let Some(current) = current else {
        return Some(predicate_evaluation::failed(
            declared.predicate,
            "inputs_changed_during_execution",
            "predicate disappeared while preflight was executing",
            vec!["predicate-set".to_owned()],
        ));
    };
    let mut changed = declared
        .input_scopes
        .iter()
        .filter(|scope| {
            current
                .input_scopes
                .iter()
                .find(|candidate| candidate.scope_identity == scope.scope_identity)
                != Some(scope)
        })
        .map(|scope| scope.scope_identity.clone())
        .collect::<Vec<_>>();
    if declared.input_scopes.len() != current.input_scopes.len() {
        changed.push("predicate-input-scope-set".to_owned());
    }
    if declared.tool != current.tool {
        changed.push("tool-declaration".to_owned());
    }
    if evaluator_changed {
        changed.push("preflight-evaluator".to_owned());
    }
    changed.sort();
    changed.dedup();
    (!changed.is_empty()).then(|| {
        predicate_evaluation::failed(
            declared.predicate,
            "inputs_changed_during_execution",
            "predicate inputs changed while preflight was executing",
            changed,
        )
    })
}
