use worth_store_test_support::structural_preflight::{
    PreflightEvidenceFreshness, PreflightEvidenceIdentity, PreflightInputScope,
    StructuralPredicate, StructuralPredicateEvidence, StructuralPredicateFailure,
    StructuralPredicatePlan, StructuralPredicateVerdict, StructuralPreflightEvidence,
    StructuralPreflightPlan, StructuralPreflightProfile, StructuralPreflightRequest,
    StructuralToolDeclaration,
};

use crate::evidence::sha256_serialized;

use super::freshness;

#[test]
fn fresh_evidence_identity_is_reused_by_multiple_compatible_consumers() {
    let evidence = passed_evidence();

    let first = freshness::compare(&evidence, &evidence.plan).unwrap();
    let second = freshness::compare(&evidence, &evidence.plan).unwrap();

    assert_eq!(first, second);
    assert_eq!(
        first,
        PreflightEvidenceFreshness::Fresh {
            evidence_identity: evidence.evidence_identity.clone(),
        }
    );
}

#[test]
fn changed_manifest_scope_names_only_the_invalidated_dependency_input() {
    let evidence = passed_evidence();
    let mut current = evidence.plan.clone();
    let dependency = current
        .predicates
        .iter_mut()
        .find(|predicate| predicate.predicate == StructuralPredicate::Dependency)
        .unwrap();
    dependency.input_scopes[0].input_identity = "changed-manifest-identity".to_owned();
    current.plan_identity.clear();
    current.plan_identity = sha256_serialized(&current).unwrap();

    let freshness = freshness::compare(&evidence, &current).unwrap();

    let PreflightEvidenceFreshness::Stale { failures } = freshness else {
        panic!("changed manifest scope was accepted as fresh");
    };
    assert_eq!(failures.len(), 1);
    assert_eq!(failures[0].predicate, StructuralPredicate::Dependency);
    assert_eq!(
        failures[0].invalidated_inputs,
        vec!["store-dependency-manifests"]
    );
}

#[test]
fn dependency_and_generated_context_failures_remain_separate_predicates() {
    let mut evidence = passed_evidence();
    evidence.predicates = evidence
        .predicates
        .into_iter()
        .map(|mut row| {
            row.verdict = StructuralPredicateVerdict::Failed {
                failure: StructuralPredicateFailure {
                    predicate: row.predicate,
                    failure_code: match row.predicate {
                        StructuralPredicate::AgentContext => "stale_generated_context",
                        StructuralPredicate::Dependency => "forbidden_dependency",
                        _ => unreachable!(),
                    }
                    .to_owned(),
                    message: format!("localized {:?} failure", row.predicate),
                    invalidated_inputs: vec![row.input_identity.clone()],
                },
            };
            row
        })
        .collect();

    let failures = evidence.failures();

    assert_eq!(failures.len(), 2);
    assert_eq!(failures[0].predicate, StructuralPredicate::AgentContext);
    assert_eq!(failures[1].predicate, StructuralPredicate::Dependency);
    assert_ne!(failures[0].failure_code, failures[1].failure_code);
}

#[test]
fn claimed_bundle_identity_cannot_hide_mutated_predicate_evidence() {
    let mut evidence = passed_evidence();
    evidence.predicates[0].verdict = StructuralPredicateVerdict::Passed {
        authority_identity: "mutated-authority".to_owned(),
    };

    assert!(evidence.validate_integrity().is_err());
}

#[test]
fn identical_boundary_and_naming_commands_launch_one_observed_tool_process() {
    let executable = std::env::current_exe().unwrap();
    let tool = StructuralToolDeclaration::workspace_owned(
        "synthetic-structural-tool",
        executable.to_string_lossy(),
        "test-harness-binary-v1",
        vec!["--list".to_owned()],
        "synthetic-structural-scope",
        30_000,
        "single-process-test-fixture",
    )
    .unwrap();
    let request = StructuralPreflightRequest::new(
        StructuralPreflightProfile::Complete,
        vec![StructuralPredicate::Boundary, StructuralPredicate::Naming],
    )
    .unwrap();
    let mut plan = StructuralPreflightPlan {
        schema_version: 1,
        request,
        predicates: vec![
            predicate_plan(StructuralPredicate::Boundary, "synthetic-structural-scope"),
            predicate_plan(StructuralPredicate::Naming, "synthetic-structural-scope"),
        ],
        plan_identity: String::new(),
    };
    for predicate in &mut plan.predicates {
        predicate.tool = Some(tool.clone());
    }
    plan.plan_identity = sha256_serialized(&plan).unwrap();
    let root = std::env::current_dir().unwrap();

    let evidence = super::execution::execute(&root, &root, plan, None, None).unwrap();

    assert_eq!(evidence.tool_executions.len(), 1);
    assert_ne!(evidence.tool_executions[0].process_id, 0);
    assert_eq!(
        evidence.tool_executions[0].declared_tool_identities,
        vec!["synthetic-structural-tool"]
    );
    evidence.validate_integrity().unwrap();
}

fn passed_evidence() -> StructuralPreflightEvidence {
    let request = StructuralPreflightRequest::new(
        StructuralPreflightProfile::DeveloperSmoke,
        vec![
            StructuralPredicate::AgentContext,
            StructuralPredicate::Dependency,
        ],
    )
    .unwrap();
    let mut plan = StructuralPreflightPlan {
        schema_version: 1,
        request,
        predicates: vec![
            predicate_plan(
                StructuralPredicate::AgentContext,
                "agent-context-authority",
            ),
            predicate_plan(
                StructuralPredicate::Dependency,
                "store-dependency-manifests",
            ),
        ],
        plan_identity: String::new(),
    };
    plan.plan_identity = sha256_serialized(&plan).unwrap();
    let predicates = plan
        .predicates
        .iter()
        .map(|predicate| {
            let input_identity = sha256_serialized(
                &predicate
                    .input_scopes
                    .iter()
                    .map(|scope| (&scope.scope_identity, &scope.input_identity))
                    .collect::<Vec<_>>(),
            )
            .unwrap();
            StructuralPredicateEvidence {
                predicate: predicate.predicate,
                input_identity,
                tool_identity: None,
                verdict: StructuralPredicateVerdict::Passed {
                    authority_identity: format!("{:?}-authority", predicate.predicate),
                },
            }
        })
        .collect();
    let mut evidence = StructuralPreflightEvidence {
        schema_version: 1,
        plan,
        predicates,
        tool_executions: Vec::new(),
        evidence_identity: PreflightEvidenceIdentity(String::new()),
    };
    evidence.evidence_identity =
        PreflightEvidenceIdentity(sha256_serialized(&evidence).unwrap());
    evidence.validate_integrity().unwrap();
    evidence
}

fn predicate_plan(
    predicate: StructuralPredicate,
    scope_identity: &str,
) -> StructuralPredicatePlan {
    StructuralPredicatePlan {
        predicate,
        input_scopes: vec![PreflightInputScope {
            scope_identity: scope_identity.to_owned(),
            source_paths: vec![format!("{scope_identity}.fixture")],
            included_extensions: vec!["fixture".to_owned()],
            input_identity: format!("{scope_identity}-input"),
        }],
        tool: None,
    }
}
