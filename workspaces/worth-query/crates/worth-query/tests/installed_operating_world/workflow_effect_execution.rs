use worth_proof::TransitionOutcome;
use worth_query::facade::{domain, foundation};

use super::installed_operation_fixture::{
    mutation_workflow_workspace, workflow_workspace, GeometryDomain, MutationFamily, ReadFamily,
    WorkflowMutation, WorkflowRead,
};

#[test]
fn workflow_effect_uses_real_mutation_authority_and_retains_its_receipt() {
    let mut workspace = mutation_workflow_workspace("workflow-mutation").unwrap();
    let initial_snapshot = workspace.snapshot_identity().clone();
    let installed_domain = workspace.domain(GeometryDomain).unwrap();
    let observation_denial = match workspace
        .operating_world(observation_basis())
        .family(MutationFamily)
        .bind(&installed_domain, WorkflowMutation)
    {
        Ok(_) => panic!("an observation basis cannot authorize workflow mutation"),
        Err(denial) => denial,
    };
    assert_eq!(
        observation_denial.kind(),
        domain::WorthQueryOperationBindingDenialKind::BasisLaneInsufficient
    );
    assert_eq!(observation_denial.counters().graph_binding_lookups, 0);

    let bound = workspace
        .operating_world(mutation_basis())
        .family(MutationFamily)
        .bind(&installed_domain, WorkflowMutation)
        .unwrap();
    assert_eq!(
        bound.commit_posture(),
        domain::WorthQueryBoundCommitPosture::Atomic
    );
    let run = bound
        .start_workflow()
        .unwrap()
        .advance(
            "mutate",
            domain::WorthQueryWorkflowValue::Text("commit".into()),
            &mut workspace,
        )
        .unwrap();
    let trace = run.complete().unwrap();
    assert_ne!(workspace.snapshot_identity(), initial_snapshot);
    assert_eq!(trace.counters().effect_receipt_checks, 1);
    let receipt = &trace.stage_receipts()[0];
    assert_eq!(receipt.counters().effect_receipt_checks, 1);
    assert_eq!(receipt.counters().invariant_checks, 1);
    assert_eq!(receipt.invariant_outcomes().len(), 1);
    assert_eq!(
        receipt.invariant_outcomes()[0].invariant_role(),
        "workflow-invariant:1"
    );
    assert!(!receipt.invariant_outcomes()[0]
        .installed_invariant_identity()
        .is_empty());
    let write = receipt.effect_evidence()[0].mutation_receipt().unwrap();
    assert_eq!(
        write.terminal_declared_collection_projection(),
        Some("Vertex")
    );
}

#[test]
fn failure_after_effect_retains_the_query_executed_partial_outcome() {
    let mut workspace = mutation_workflow_workspace("workflow-partial-effect").unwrap();
    let initial_snapshot = workspace.snapshot_identity().clone();
    let installed_domain = workspace.domain(GeometryDomain).unwrap();
    let run = workspace
        .operating_world(mutation_basis())
        .family(MutationFamily)
        .bind(&installed_domain, WorkflowMutation)
        .unwrap()
        .start_workflow()
        .unwrap();
    let denial = match run.advance(
        "mutate",
        domain::WorthQueryWorkflowValue::Text("fail-after-effect".into()),
        &mut workspace,
    ) {
        TransitionOutcome::Failed(denial) => denial,
        _ => panic!("post-effect executor did not produce an execution failure"),
    };
    assert_ne!(workspace.snapshot_identity(), initial_snapshot);
    assert_eq!(
        denial.kind(),
        &domain::WorthQueryWorkflowAdvanceDenialKind::StageExecutor {
            class: domain::WorthQueryOperationFailureClass::Dependency,
            detail: "declared failure after mutation".into(),
        }
    );
    assert_eq!(denial.executed_effects().len(), 1);
    assert!(denial.executed_effects()[0].mutation_receipt().is_some());
}

#[test]
fn workflow_executor_failures_are_typed_and_must_be_declared() {
    let declared = failing_stage_denial("workflow-declared-failure", "fail-dependency");
    assert_eq!(
        declared.kind(),
        &domain::WorthQueryWorkflowAdvanceDenialKind::StageExecutor {
            class: domain::WorthQueryOperationFailureClass::Dependency,
            detail: "declared dependency failure".into(),
        }
    );
    let undeclared = failing_stage_denial("workflow-undeclared-failure", "fail-unsupported");
    assert_eq!(
        undeclared.kind(),
        &domain::WorthQueryWorkflowAdvanceDenialKind::UndeclaredFailureClass(
            domain::WorthQueryOperationFailureClass::Unsupported,
        )
    );
}

#[test]
fn workflow_read_requires_the_exact_stage_local_primary_role() {
    let denial = failing_stage_denial("workflow-undeclared-read", "read-undeclared");
    assert_eq!(
        denial.kind(),
        &domain::WorthQueryWorkflowAdvanceDenialKind::UndeclaredFailureClass(
            domain::WorthQueryOperationFailureClass::Indeterminate,
        )
    );
    assert_eq!(denial.counters().stage_executor_contacts, 2);
    assert_eq!(denial.counters().graph_read_contacts, 0);
    assert_eq!(denial.completed_stage_receipts().len(), 1);
}

#[test]
fn workflow_stage_cannot_skip_its_declared_primary_read() {
    let mut workspace = workflow_workspace("workflow-skipped-read").unwrap();
    let installed_domain = workspace.domain(GeometryDomain).unwrap();
    let run = workspace
        .operating_world(observation_basis())
        .family(ReadFamily)
        .bind(&installed_domain, WorkflowRead)
        .unwrap()
        .start_workflow()
        .unwrap()
        .advance(
            "start",
            domain::WorthQueryWorkflowValue::NotRequired,
            &mut workspace,
        )
        .unwrap()
        .advance(
            "left",
            domain::WorthQueryWorkflowValue::Text("left".into()),
            &mut workspace,
        )
        .unwrap()
        .advance(
            "right",
            domain::WorthQueryWorkflowValue::Text("right".into()),
            &mut workspace,
        )
        .unwrap();
    let denial = match run.advance(
        "publish",
        domain::WorthQueryWorkflowValue::Text("skip-read".into()),
        &mut workspace,
    ) {
        TransitionOutcome::Failed(denial) => denial,
        _ => panic!("skipped primary read did not produce an execution failure"),
    };
    assert_eq!(
        denial.kind(),
        &domain::WorthQueryWorkflowAdvanceDenialKind::UndeclaredFailureClass(
            domain::WorthQueryOperationFailureClass::Indeterminate,
        )
    );
    assert_eq!(denial.counters().graph_read_contacts, 0);
    assert_eq!(denial.completed_stage_receipts().len(), 3);
    assert_eq!(
        denial
            .completed_stage_receipts()
            .iter()
            .map(|receipt| receipt.stage_identity())
            .collect::<Vec<_>>(),
        ["start", "left", "right"]
    );
    let first = &denial.completed_stage_receipts()[0];
    assert!(!first.operation_identity().is_empty());
    assert!(denial
        .completed_stage_receipts()
        .iter()
        .all(|receipt| receipt.operation_identity() == first.operation_identity()));
    assert!(!first.basis_identity().is_empty());
}

fn failing_stage_denial(name: &str, input: &str) -> domain::WorthQueryWorkflowAdvanceDenial {
    let mut workspace = workflow_workspace(name).unwrap();
    let installed_domain = workspace.domain(GeometryDomain).unwrap();
    let run = workspace
        .operating_world(observation_basis())
        .family(ReadFamily)
        .bind(&installed_domain, WorkflowRead)
        .unwrap()
        .start_workflow()
        .unwrap()
        .advance(
            "start",
            domain::WorthQueryWorkflowValue::NotRequired,
            &mut workspace,
        )
        .unwrap();
    match run.advance(
        "left",
        domain::WorthQueryWorkflowValue::Text(input.into()),
        &mut workspace,
    ) {
        TransitionOutcome::Failed(denial) => denial,
        _ => panic!("failing stage did not produce an execution failure"),
    }
}

fn observation_basis() -> foundation::AdmittedBasisCapability<foundation::ObservationLaneWitness> {
    foundation::basis_lifecycle()
        .current_head()
        .for_observation()
        .unwrap()
        .admit()
        .unwrap()
        .capability()
        .clone()
}

fn mutation_basis(
) -> foundation::AdmittedBasisCapability<foundation::MutationPreparationLaneWitness> {
    foundation::basis_lifecycle()
        .current_head()
        .for_mutation_preparation()
        .unwrap()
        .admit()
        .unwrap()
}
