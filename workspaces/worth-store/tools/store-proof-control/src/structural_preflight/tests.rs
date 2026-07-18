use worth_store_test_support::structural_preflight::{
    PreflightEvidenceFreshness, PreflightEvidenceIdentity, PreflightInputScope,
    StructuralPredicate, StructuralPredicateEvidence, StructuralPredicatePlan,
    StructuralPredicateVerdict, StructuralPreflightEvaluatorIdentity, StructuralPreflightEvidence,
    StructuralPreflightPlan, StructuralPreflightProfile, StructuralPreflightRequest,
    StructuralToolDeclaration,
};

use crate::classification::{
    ClassifiedInventory, FeatureAuthorityClass, FeatureSemanticAuthority,
    FeatureSemanticDeclaration,
};
use crate::discovery::{
    DependencyEdge, ObservedArtifactFootprint, ObservedBuildGraph, PackageSurface,
    TestSurfaceInventory,
};
use crate::evidence::sha256_serialized;
use crate::ValidatedProofInventory;

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
    dependency.input_scopes[0].input_identity =
        sha256_serialized(&"changed-manifest-identity").unwrap();
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
fn forbidden_production_edge_fails_dependency_without_poisoning_feature_authority() {
    let inventory = inventory_with_forbidden_feature_edge();
    let plan = unsigned_plan(vec![
        predicate_plan(StructuralPredicate::Feature, "feature-authority"),
        predicate_plan(StructuralPredicate::Dependency, "dependency-manifests"),
    ]);

    let evidence = super::execution::execute_with_stable_test_plan(
        std::path::Path::new("."),
        plan,
        Some(&inventory),
        None,
    )
    .unwrap();

    assert!(evidence
        .passed_identity(StructuralPredicate::Feature)
        .is_some());
    let failures = evidence.failures();
    assert_eq!(failures.len(), 1);
    assert_eq!(failures[0].predicate, StructuralPredicate::Dependency);
    assert_eq!(failures[0].failure_code, "forbidden_dependency");
    assert!(failures[0].message.contains("fixture-app"));
    assert!(failures[0].message.contains("fixture-substrate"));
}

#[test]
fn repository_failure_has_one_owner_and_explicit_dependents() {
    let failure = super::RepositoryPredicateFailure::new(
        StructuralPredicate::Inventory,
        "current_inventory_invalid",
        "current proof inventory drifted",
        ["store-proof-inventory"],
    );
    let plan = unsigned_plan(vec![
        predicate_plan(StructuralPredicate::Inventory, "inventory"),
        predicate_plan(StructuralPredicate::Preservation, "preservation"),
        predicate_plan(StructuralPredicate::Feature, "feature"),
        predicate_plan(StructuralPredicate::Dependency, "dependency"),
    ]);

    let evidence = super::execution::execute_with_stable_test_plan(
        std::path::Path::new("."),
        plan,
        None,
        Some(&failure),
    )
    .unwrap();
    let failures = evidence.failures();
    assert_eq!(failures.len(), 4);
    assert_eq!(
        failures
            .iter()
            .filter(|failure| failure.failure_code == "current_inventory_invalid")
            .count(),
        1
    );
    assert_eq!(
        failures
            .iter()
            .filter(|failure| failure.failure_code == "prerequisite_unavailable")
            .count(),
        3
    );
}

#[test]
fn changed_generated_context_names_only_agent_context_input() {
    let evidence = passed_evidence();
    let mut current = evidence.plan.clone();
    let context = current
        .predicates
        .iter_mut()
        .find(|predicate| predicate.predicate == StructuralPredicate::AgentContext)
        .unwrap();
    context.input_scopes[0].input_identity = sha256_serialized(&"stale-generated-context").unwrap();
    current.plan_identity.clear();
    current.plan_identity = sha256_serialized(&current).unwrap();

    let freshness = freshness::compare(&evidence, &current).unwrap();

    let PreflightEvidenceFreshness::Stale { failures } = freshness else {
        panic!("changed generated context was accepted as fresh");
    };
    assert_eq!(failures.len(), 1);
    assert_eq!(failures[0].predicate, StructuralPredicate::AgentContext);
    assert_eq!(
        failures[0].invalidated_inputs,
        vec!["agent-context-authority"]
    );
}

#[test]
fn changed_evaluator_invalidates_every_reusable_predicate() {
    let evidence = passed_evidence();
    let mut current = evidence.plan.clone();
    current.evaluator.executable_sha256 = sha256_serialized(&"new-evaluator").unwrap();
    current.plan_identity.clear();
    current.plan_identity = sha256_serialized(&current).unwrap();

    let freshness = freshness::compare(&evidence, &current).unwrap();

    let PreflightEvidenceFreshness::Stale { failures } = freshness else {
        panic!("evidence from a displaced evaluator was accepted as fresh");
    };
    assert_eq!(failures.len(), evidence.plan.predicates.len());
    assert!(failures.iter().all(|failure| failure
        .invalidated_inputs
        .contains(&"preflight-evaluator".to_owned())));
}

#[test]
fn claimed_bundle_identity_cannot_hide_mutated_predicate_evidence() {
    let mut evidence = passed_evidence();
    evidence.predicates[0].verdict = StructuralPredicateVerdict::Passed {
        authority_basis_identity: "mutated-basis".to_owned(),
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
        evaluator: synthetic_evaluator(),
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

    let evidence =
        super::execution::execute_with_stable_test_plan(&root, plan, None, None).unwrap();

    assert_eq!(evidence.tool_executions.len(), 1);
    assert_ne!(evidence.tool_executions[0].process_id, 0);
    assert_eq!(
        evidence.tool_executions[0].declared_tool_identities,
        vec!["synthetic-structural-tool"]
    );
    evidence.validate_integrity().unwrap();

    let mut forged = evidence.clone();
    forged.tool_executions[0].exit_code = Some(7);
    forged.evidence_identity = PreflightEvidenceIdentity(String::new());
    forged.evidence_identity = PreflightEvidenceIdentity(sha256_serialized(&forged).unwrap());
    assert!(forged.validate_integrity().is_err());
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
        evaluator: synthetic_evaluator(),
        predicates: vec![
            predicate_plan(StructuralPredicate::AgentContext, "agent-context-authority"),
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
            let authority_basis_identity =
                sha256_serialized(&(predicate.predicate, "synthetic-basis")).unwrap();
            let authority_identity = sha256_serialized(&(
                predicate.predicate,
                &input_identity,
                &authority_basis_identity,
            ))
            .unwrap();
            StructuralPredicateEvidence {
                predicate: predicate.predicate,
                input_identity,
                tool_identity: None,
                verdict: StructuralPredicateVerdict::Passed {
                    authority_basis_identity,
                    authority_identity,
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
    evidence.evidence_identity = PreflightEvidenceIdentity(sha256_serialized(&evidence).unwrap());
    evidence.validate_integrity().unwrap();
    evidence
}

fn predicate_plan(predicate: StructuralPredicate, scope_identity: &str) -> StructuralPredicatePlan {
    StructuralPredicatePlan {
        predicate,
        input_scopes: vec![PreflightInputScope {
            scope_identity: scope_identity.to_owned(),
            source_paths: vec![format!("{scope_identity}.fixture")],
            included_extensions: vec!["fixture".to_owned()],
            input_identity: sha256_serialized(&(scope_identity, "input")).unwrap(),
        }],
        tool: None,
    }
}

fn unsigned_plan(predicates: Vec<StructuralPredicatePlan>) -> StructuralPreflightPlan {
    let request = StructuralPreflightRequest::new(
        StructuralPreflightProfile::Complete,
        predicates.iter().map(|plan| plan.predicate).collect(),
    )
    .unwrap();
    let mut plan = StructuralPreflightPlan {
        schema_version: 1,
        request,
        evaluator: synthetic_evaluator(),
        predicates,
        plan_identity: String::new(),
    };
    plan.plan_identity = sha256_serialized(&plan).unwrap();
    plan
}

fn synthetic_evaluator() -> StructuralPreflightEvaluatorIdentity {
    StructuralPreflightEvaluatorIdentity {
        responsibility: "synthetic-preflight-evaluator".to_owned(),
        executable_path: "synthetic/preflight-evaluator".to_owned(),
        executable_sha256: sha256_serialized(&"synthetic-preflight-evaluator").unwrap(),
        version_identity: "synthetic-preflight-evaluator/v1".to_owned(),
    }
}

fn inventory_with_forbidden_feature_edge() -> ValidatedProofInventory {
    let provider_feature = "certification-test-authority".to_owned();
    let discovered = TestSurfaceInventory {
        schema_version: 2,
        workspace_root: "fixture".to_owned(),
        target_root: "fixture/target".to_owned(),
        packages: vec![
            PackageSurface {
                name: "fixture-app".to_owned(),
                manifest_path: "fixture-app/Cargo.toml".to_owned(),
                package_root: "fixture-app".to_owned(),
                features: Vec::new(),
                feature_definitions: Default::default(),
            },
            PackageSurface {
                name: "fixture-substrate".to_owned(),
                manifest_path: "fixture-substrate/Cargo.toml".to_owned(),
                package_root: "fixture-substrate".to_owned(),
                features: vec![provider_feature.clone()],
                feature_definitions: [(provider_feature.clone(), Vec::new())]
                    .into_iter()
                    .collect(),
            },
        ],
        targets: Vec::new(),
        cases: Vec::new(),
        build_graph: ObservedBuildGraph {
            dependency_edges: vec![DependencyEdge {
                consumer: "fixture-app".to_owned(),
                provider: "fixture-substrate".to_owned(),
                manifest_name: "fixture-substrate".to_owned(),
                dependency_kind: "normal".to_owned(),
                features: vec![provider_feature.clone()],
                optional: false,
                uses_default_features: false,
                target: None,
            }],
        },
        workflow_commands: Vec::new(),
        historical_artifacts: ObservedArtifactFootprint::not_observed("fixture/target"),
        feature_semantic_authority: FeatureSemanticAuthority {
            schema_version: 1,
            declarations: vec![FeatureSemanticDeclaration {
                package: "fixture-substrate".to_owned(),
                feature: provider_feature,
                authority: FeatureAuthorityClass::TestAuthority,
            }],
        },
    };
    ValidatedProofInventory::from_classified(ClassifiedInventory {
        schema_version: 1,
        discovered,
        proofs: Vec::new(),
    })
}
