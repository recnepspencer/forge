use std::path::Path;

use worth_store_test_support::structural_preflight::{
    PreflightEvidenceIdentity, StructuralPredicate, StructuralPredicateEvidence,
    StructuralPredicateFailure, StructuralPredicateVerdict, StructuralPreflightEvidence,
    StructuralPreflightPlan,
};

use crate::evidence::sha256_serialized;
use crate::ValidatedProofInventory;

use super::{residue, tool_execution};

pub(super) fn execute(
    forge_root: &Path,
    store_root: &Path,
    plan: StructuralPreflightPlan,
    inventory: Option<&ValidatedProofInventory>,
    validation_failure: Option<&str>,
) -> Result<StructuralPreflightEvidence, String> {
    let tools = tool_execution::execute_once(forge_root, &plan);
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
            .and_then(|tool| tools.get(&tool_key(tool)));
        let verdict = evaluate_predicate(
            forge_root,
            store_root,
            predicate.predicate,
            &input_identity,
            inventory,
            validation_failure,
            tool_outcome,
        )?;
        predicates.push(StructuralPredicateEvidence {
            predicate: predicate.predicate,
            input_identity,
            tool_identity: tool_outcome.map(|outcome| outcome.identity.clone()),
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

fn evaluate_predicate(
    forge_root: &Path,
    _store_root: &Path,
    predicate: StructuralPredicate,
    input_identity: &str,
    inventory: Option<&ValidatedProofInventory>,
    validation_failure: Option<&str>,
    tool: Option<&ToolOutcome>,
) -> Result<StructuralPredicateVerdict, String> {
    use StructuralPredicate as Predicate;
    if let Some(tool) = tool {
        if let Some(message) = &tool.failure {
            return Ok(failed(
                predicate,
                "tool_rejected",
                message,
                vec![tool.identity.clone()],
            ));
        }
    }
    match predicate {
        Predicate::Boundary | Predicate::AgentContext | Predicate::LineCap | Predicate::Naming => {
            let tool = tool.ok_or_else(|| format!("{predicate:?} has no declared tool result"))?;
            Ok(passed(predicate, input_identity, &tool.identity))
        }
        Predicate::Inventory
        | Predicate::Preservation
        | Predicate::Feature
        | Predicate::Dependency => {
            let Some(inventory) = inventory else {
                return Ok(failed(
                    predicate,
                    "repository_validation_failed",
                    validation_failure.unwrap_or("repository validation produced no inventory"),
                    vec![input_identity.to_owned()],
                ));
            };
            let inventory_identity = match predicate {
                Predicate::Inventory => sha256_serialized(inventory.inventory())?,
                Predicate::Preservation => sha256_serialized(&(
                    "proof-preservation-validated-v1",
                    inventory.inventory().proofs.len(),
                    input_identity,
                ))?,
                Predicate::Feature => sha256_serialized(&(
                    "feature-graph-validated-v1",
                    &inventory.inventory().discovered.packages,
                ))?,
                Predicate::Dependency => sha256_serialized(&(
                    "dependency-boundary-validated-v1",
                    &inventory.inventory().discovered.packages,
                    input_identity,
                ))?,
                _ => unreachable!(),
            };
            Ok(passed(predicate, input_identity, &inventory_identity))
        }
        Predicate::AdmittedResidue => match residue::validate(forge_root) {
            Ok(posture) => Ok(passed(
                predicate,
                input_identity,
                &sha256_serialized(&posture)?,
            )),
            Err(violations) => Ok(failed(
                predicate,
                "displaced_path_retained",
                &violations.join("\n"),
                vec![input_identity.to_owned()],
            )),
        },
    }
}

fn passed(
    predicate: StructuralPredicate,
    input_identity: &str,
    authority: &str,
) -> StructuralPredicateVerdict {
    let identity = sha256_serialized(&(predicate, input_identity, authority))
        .expect("structural predicate identity is serializable");
    StructuralPredicateVerdict::Passed {
        authority_identity: identity,
    }
}

fn failed(
    predicate: StructuralPredicate,
    code: &str,
    message: &str,
    invalidated_inputs: Vec<String>,
) -> StructuralPredicateVerdict {
    StructuralPredicateVerdict::Failed {
        failure: StructuralPredicateFailure {
            predicate,
            failure_code: code.to_owned(),
            message: message.to_owned(),
            invalidated_inputs,
        },
    }
}
