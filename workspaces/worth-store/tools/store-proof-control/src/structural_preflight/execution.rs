use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

use worth_store_test_support::structural_preflight::{
    PreflightEvidenceIdentity, StructuralPredicate, StructuralPredicateEvidence,
    StructuralPredicateFailure, StructuralPredicateVerdict, StructuralPreflightEvidence,
    StructuralPreflightPlan, StructuralToolDeclaration,
};

use crate::evidence::{sha256_bytes, sha256_serialized};
use crate::ValidatedProofInventory;

use super::residue;

pub(super) fn execute(
    forge_root: &Path,
    store_root: &Path,
    plan: StructuralPreflightPlan,
    inventory: Option<&ValidatedProofInventory>,
    validation_failure: Option<&str>,
) -> Result<StructuralPreflightEvidence, String> {
    let tools = execute_tools_once(forge_root, &plan);
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

fn execute_tools_once(
    forge_root: &Path,
    plan: &StructuralPreflightPlan,
) -> BTreeMap<String, ToolOutcome> {
    let mut outcomes = BTreeMap::new();
    for tool in plan
        .predicates
        .iter()
        .filter_map(|predicate| predicate.tool.as_ref())
    {
        let key = tool_key(tool);
        outcomes.entry(key).or_insert_with(|| run_tool(forge_root, tool));
    }
    outcomes
}

fn run_tool(root: &Path, tool: &StructuralToolDeclaration) -> ToolOutcome {
    let output = Command::new(&tool.program)
        .args(&tool.arguments)
        .current_dir(root)
        .output();
    match output {
        Ok(output) => {
            let root_text = root.to_string_lossy();
            let stdout = normalize(&output.stdout, &root_text);
            let stderr = normalize(&output.stderr, &root_text);
            let identity = sha256_bytes(
                format!(
                    "{}\n{}\n{}\n{}",
                    tool.tool_identity,
                    output.status.code().unwrap_or(-1),
                    stdout,
                    stderr
                )
                .as_bytes(),
            );
            ToolOutcome {
                identity,
                failure: (!output.status.success()).then(|| {
                    format!(
                        "{} rejected with {:?}: {}",
                        tool.tool_identity,
                        output.status.code(),
                        stderr
                    )
                }),
            }
        }
        Err(error) => ToolOutcome {
            identity: tool.tool_identity.clone(),
            failure: Some(format!("could not launch {}: {error}", tool.tool_identity)),
        },
    }
}

fn tool_key(tool: &StructuralToolDeclaration) -> String {
    serde_json::to_string(&(&tool.program, &tool.arguments))
        .expect("tool command identity is serializable")
}

fn normalize(bytes: &[u8], root: &str) -> String {
    String::from_utf8_lossy(bytes)
        .replace(root, "<forge-root>")
        .replace('\\', "/")
}

struct ToolOutcome {
    identity: String,
    failure: Option<String>,
}
