use std::path::Path;

use worth_store_test_support::structural_preflight::{
    PreflightEvidenceIdentity, StructuralPredicate, StructuralPredicateEvidence,
    StructuralPredicateFailure, StructuralPredicateVerdict, StructuralPreflightEvidence,
    StructuralPreflightPlan,
};

use crate::evidence::sha256_serialized;
use crate::classification::{
    validate_feature_semantic_authority_policy, validate_production_dependency_policy,
};
use crate::ValidatedProofInventory;

use super::{residue, tool_execution};

pub(super) fn execute(
    forge_root: &Path,
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
            predicate.predicate,
            &input_identity,
            inventory,
            validation_failure,
            tool_outcome,
        )?;
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

fn evaluate_predicate(
    forge_root: &Path,
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
                vec![tool.identity().to_owned()],
            ));
        }
    }
    match predicate {
        Predicate::Boundary | Predicate::AgentContext | Predicate::LineCap | Predicate::Naming => {
            let tool = tool.ok_or_else(|| format!("{predicate:?} has no declared tool result"))?;
            Ok(passed(predicate, input_identity, tool.identity()))
        }
        Predicate::Inventory | Predicate::Preservation => {
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
                _ => unreachable!(),
            };
            Ok(passed(predicate, input_identity, &inventory_identity))
        }
        Predicate::Feature => evaluate_feature_predicate(
            predicate,
            input_identity,
            inventory,
            validation_failure,
        ),
        Predicate::Dependency => evaluate_dependency_predicate(
            predicate,
            input_identity,
            inventory,
            validation_failure,
        ),
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

fn evaluate_feature_predicate(
    predicate: StructuralPredicate,
    input_identity: &str,
    inventory: Option<&ValidatedProofInventory>,
    validation_failure: Option<&str>,
) -> Result<StructuralPredicateVerdict, String> {
    let Some(inventory) = inventory else {
        return Ok(repository_validation_failed(
            predicate,
            input_identity,
            validation_failure,
        ));
    };
    match validate_feature_semantic_authority_policy(&inventory.inventory().discovered) {
        Ok(_) => Ok(passed(
            predicate,
            input_identity,
            &sha256_serialized(&(
                "feature-semantic-authority-validated-v1",
                &inventory.inventory().discovered.feature_semantic_authority,
            ))?,
        )),
        Err(violations) => Ok(failed(
            predicate,
            "invalid_feature_semantic_authority",
            &joined_violations(&violations),
            vec![input_identity.to_owned()],
        )),
    }
}

fn evaluate_dependency_predicate(
    predicate: StructuralPredicate,
    input_identity: &str,
    inventory: Option<&ValidatedProofInventory>,
    validation_failure: Option<&str>,
) -> Result<StructuralPredicateVerdict, String> {
    let Some(inventory) = inventory else {
        return Ok(repository_validation_failed(
            predicate,
            input_identity,
            validation_failure,
        ));
    };
    let discovered = &inventory.inventory().discovered;
    let authority = match validate_feature_semantic_authority_policy(discovered) {
        Ok(authority) => authority,
        Err(violations) => {
            return Ok(failed(
                predicate,
                "feature_authority_prerequisite_rejected",
                &joined_violations(&violations),
                vec!["feature-semantic-authority".to_owned()],
            ));
        }
    };
    match validate_production_dependency_policy(discovered, &authority) {
        Ok(()) => Ok(passed(
            predicate,
            input_identity,
            &sha256_serialized(&(
                "production-dependency-policy-validated-v1",
                &discovered.build_graph,
                input_identity,
            ))?,
        )),
        Err(violations) => Ok(failed(
            predicate,
            "forbidden_dependency",
            &joined_violations(&violations),
            vec![input_identity.to_owned()],
        )),
    }
}

fn repository_validation_failed(
    predicate: StructuralPredicate,
    input_identity: &str,
    validation_failure: Option<&str>,
) -> StructuralPredicateVerdict {
    failed(
        predicate,
        "repository_validation_failed",
        validation_failure.unwrap_or("repository validation produced no inventory"),
        vec![input_identity.to_owned()],
    )
}

fn joined_violations(violations: &[impl std::fmt::Display]) -> String {
    violations
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n")
}

fn passed(
    predicate: StructuralPredicate,
    input_identity: &str,
    authority: &str,
) -> StructuralPredicateVerdict {
    let identity = sha256_serialized(&(predicate, input_identity, authority))
        .expect("structural predicate identity is serializable");
    StructuralPredicateVerdict::Passed {
        authority_basis_identity: authority.to_owned(),
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
