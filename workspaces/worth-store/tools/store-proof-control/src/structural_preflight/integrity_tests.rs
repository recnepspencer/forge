use worth_store_test_support::structural_preflight::{
    PreflightEvidenceIdentity, PreflightInputScope, StructuralPredicate,
    StructuralPredicateEvidence, StructuralPredicatePlan, StructuralPredicateVerdict,
    StructuralPreflightEvaluatorIdentity, StructuralPreflightEvidence,
    StructuralPreflightIntegrityDenial, StructuralPreflightPlan, StructuralPreflightProfile,
    StructuralPreflightRequest, StructuralToolDeclaration, StructuralToolExecutionEvidence,
};

use crate::evidence::{sha256_bytes, sha256_serialized};

#[test]
fn required_external_tool_cannot_be_omitted_from_a_predicate_plan() {
    let mut evidence = passed_external_evidence();
    let context = predicate_mut(&mut evidence, StructuralPredicate::AgentContext);
    context.tool = None;
    seal_plan(&mut evidence.plan);

    assert_eq!(
        evidence.plan.validate_integrity(),
        Err(StructuralPreflightIntegrityDenial::PredicateExecutionModeMismatch)
    );
}

#[test]
fn internal_predicate_cannot_claim_an_external_tool_authority() {
    let evidence = passed_external_evidence();
    let mut plan = evidence.plan.clone();
    let tool = plan.predicates[0].tool.clone();
    plan.predicates.push(StructuralPredicatePlan {
        predicate: StructuralPredicate::Dependency,
        input_scopes: vec![input_scope("dependency-authority")],
        tool,
    });
    plan.request
        .predicates
        .push(StructuralPredicate::Dependency);
    plan.request.predicates.sort();
    seal_plan(&mut plan);

    assert_eq!(
        plan.validate_integrity(),
        Err(StructuralPreflightIntegrityDenial::PredicateExecutionModeMismatch)
    );
}

#[test]
fn successful_tool_executions_cannot_be_swapped_between_predicates() {
    let mut evidence = passed_external_evidence();
    let first = evidence.tool_executions[0].authority_identity.clone();
    let second = evidence.tool_executions[1].authority_identity.clone();
    admit_basis(&mut evidence.predicates[0], second);
    admit_basis(&mut evidence.predicates[1], first);
    seal_evidence(&mut evidence);

    assert_eq!(
        evidence.validate_integrity(),
        Err(StructuralPreflightIntegrityDenial::PredicateToolIdentityMismatch)
    );
}

#[test]
fn identical_commands_launch_once_but_admit_each_predicate_tool_identity() {
    let first = synthetic_tool("boundary-tool", "shared-authority", "--list");
    let mut second = first.clone();
    second.tool_identity = "naming-tool".to_owned();
    let mut plan = external_plan(vec![
        predicate_plan(StructuralPredicate::Boundary, "shared-authority", first),
        predicate_plan(StructuralPredicate::Naming, "shared-authority", second),
    ]);
    seal_plan(&mut plan);

    let evidence = super::execution::execute_with_stable_test_plan(
        &std::env::current_dir().unwrap(),
        plan,
        None,
        None,
    )
    .unwrap();

    assert_eq!(evidence.tool_executions.len(), 1);
    assert_eq!(
        evidence.tool_executions[0].declared_tool_identities,
        vec!["boundary-tool".to_owned(), "naming-tool".to_owned()]
    );
    evidence.validate_integrity().unwrap();
}

pub(super) fn passed_external_evidence() -> StructuralPreflightEvidence {
    let mut plan = external_plan(vec![
        predicate_plan(
            StructuralPredicate::AgentContext,
            "agent-context-authority",
            synthetic_tool("agent-context-tool", "agent-context-authority", "--list"),
        ),
        predicate_plan(
            StructuralPredicate::LineCap,
            "line-cap-authority",
            synthetic_tool("line-cap-tool", "line-cap-authority", "--help"),
        ),
    ]);
    seal_plan(&mut plan);
    let tool_executions = plan
        .predicates
        .iter()
        .map(|predicate| synthetic_execution(predicate.tool.as_ref().unwrap()))
        .collect::<Vec<_>>();
    let predicates = plan
        .predicates
        .iter()
        .zip(&tool_executions)
        .map(|(plan, execution)| {
            let input_identity = predicate_input_identity(plan);
            let authority_basis_identity = execution.authority_identity.clone();
            let authority_identity =
                sha256_serialized(&(plan.predicate, &input_identity, &authority_basis_identity))
                    .unwrap();
            StructuralPredicateEvidence {
                predicate: plan.predicate,
                input_identity,
                tool_identity: Some(authority_basis_identity.clone()),
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
        tool_executions,
        evidence_identity: PreflightEvidenceIdentity(String::new()),
    };
    seal_evidence(&mut evidence);
    evidence.validate_integrity().unwrap();
    evidence
}

fn external_plan(predicates: Vec<StructuralPredicatePlan>) -> StructuralPreflightPlan {
    let request = StructuralPreflightRequest::new(
        StructuralPreflightProfile::Complete,
        predicates
            .iter()
            .map(|predicate| predicate.predicate)
            .collect(),
    )
    .unwrap();
    StructuralPreflightPlan {
        schema_version: 1,
        request,
        evaluator: StructuralPreflightEvaluatorIdentity {
            responsibility: "synthetic-preflight-evaluator".to_owned(),
            executable_path: "synthetic/preflight-evaluator".to_owned(),
            executable_sha256: sha256_serialized(&"evaluator").unwrap(),
            version_identity: "synthetic-evaluator/v1".to_owned(),
        },
        predicates,
        plan_identity: String::new(),
    }
}

fn predicate_plan(
    predicate: StructuralPredicate,
    scope: &str,
    tool: StructuralToolDeclaration,
) -> StructuralPredicatePlan {
    StructuralPredicatePlan {
        predicate,
        input_scopes: vec![input_scope(scope)],
        tool: Some(tool),
    }
}

fn input_scope(scope: &str) -> PreflightInputScope {
    PreflightInputScope {
        scope_identity: scope.to_owned(),
        source_paths: vec![format!("{scope}.fixture")],
        included_extensions: vec!["fixture".to_owned()],
        input_identity: sha256_serialized(&(scope, "input")).unwrap(),
    }
}

fn synthetic_tool(identity: &str, scope: &str, argument: &str) -> StructuralToolDeclaration {
    StructuralToolDeclaration::workspace_owned(
        identity,
        std::env::current_exe().unwrap().to_string_lossy(),
        "test-harness-binary-v1",
        vec![argument.to_owned()],
        scope,
        30_000,
        "single-process-test-fixture",
    )
    .unwrap()
}

fn synthetic_execution(tool: &StructuralToolDeclaration) -> StructuralToolExecutionEvidence {
    let mut execution = StructuralToolExecutionEvidence {
        command_identity: super::tool_execution::command_identity(tool),
        provenance: tool.provenance.clone(),
        program: tool.program.clone(),
        resolved_program_path: tool.resolved_program_path.clone(),
        program_sha256: tool.program_sha256.clone(),
        program_version_identity: tool.program_version_identity.clone(),
        arguments: tool.arguments.clone(),
        supporting_tools: tool.supporting_tools.clone(),
        environment: tool.environment.clone(),
        removed_environment: tool.removed_environment.clone(),
        declared_tool_identities: vec![tool.tool_identity.clone()],
        timeout_millis: tool.timeout_millis,
        resource_posture: tool.resource_posture.clone(),
        process_id: std::process::id(),
        exit_code: Some(0),
        timed_out: false,
        stdout_sha256: sha256_bytes(b""),
        stderr_sha256: sha256_bytes(b""),
        observation_failure: None,
        successful: true,
        authority_identity: String::new(),
    };
    execution.seal_authority_identity().unwrap();
    execution
}

fn predicate_input_identity(plan: &StructuralPredicatePlan) -> String {
    sha256_serialized(
        &plan
            .input_scopes
            .iter()
            .map(|scope| (&scope.scope_identity, &scope.input_identity))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

fn admit_basis(evidence: &mut StructuralPredicateEvidence, basis: String) {
    evidence.tool_identity = Some(basis.clone());
    evidence.verdict = StructuralPredicateVerdict::Passed {
        authority_identity: sha256_serialized(&(
            evidence.predicate,
            &evidence.input_identity,
            &basis,
        ))
        .unwrap(),
        authority_basis_identity: basis,
    };
}

fn predicate_mut(
    evidence: &mut StructuralPreflightEvidence,
    predicate: StructuralPredicate,
) -> &mut StructuralPredicatePlan {
    evidence
        .plan
        .predicates
        .iter_mut()
        .find(|candidate| candidate.predicate == predicate)
        .unwrap()
}

fn seal_plan(plan: &mut StructuralPreflightPlan) {
    plan.plan_identity.clear();
    plan.plan_identity = sha256_serialized(plan).unwrap();
}

fn seal_evidence(evidence: &mut StructuralPreflightEvidence) {
    evidence.evidence_identity = PreflightEvidenceIdentity(String::new());
    evidence.evidence_identity = PreflightEvidenceIdentity(sha256_serialized(evidence).unwrap());
}
