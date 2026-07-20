use worth_proof::TransitionOutcome;
use worth_query::facade::{domain, foundation, read};

use super::installed_operation_fixture::{
    foreign_material_workspace, missing_read_execution_workspace, workspace, CountVertices,
    CountVerticesInput, GeometryDomain, ReadExecutionInput, ReadFamily, ReadVertex,
};

#[test]
fn public_bound_execution_projection_and_settlement_remain_one_chain() {
    let mut workspace = workspace("installed-progression", false).unwrap();
    let installed_domain = workspace.domain(GeometryDomain).unwrap();
    let bound = workspace
        .operating_world(observation_basis())
        .family(ReadFamily)
        .bind(&installed_domain, ReadVertex)
        .unwrap();
    let binding_identity = bound.binding_identity().to_string();
    let consumer = bound.consumer_projection_contract().unwrap();

    let executed = bound
        .execute(ReadExecutionInput::default(), &mut workspace)
        .unwrap();
    assert_eq!(executed.receipt().binding_identity(), binding_identity);
    assert_eq!(executed.counters().executor_contacts, 1);
    let settled = executed
        .publish()
        .unwrap()
        .consume(consumer, read::project_facts().entity_identities())
        .unwrap()
        .settle()
        .unwrap();

    assert_eq!(
        settled.execution_receipt().binding_identity(),
        binding_identity
    );
    assert_eq!(
        settled.result_state(),
        domain::WorthQueryOperationResultState::Ready
    );
    assert_eq!(settled.counters().consumption_contacts, 1);
    assert!(!settled.publication_receipt().identity().is_empty());
    assert!(!settled.identity().is_empty());
}

#[test]
fn non_publishing_execution_is_a_terminal_typed_outcome() {
    let mut workspace = workspace("installed-terminal", false).unwrap();
    let installed_domain = workspace.domain(GeometryDomain).unwrap();
    let bound = workspace
        .operating_world(observation_basis())
        .family(ReadFamily)
        .bind(&installed_domain, CountVertices)
        .unwrap();
    let executed = bound
        .execute(CountVerticesInput { minimum: Some(0) }, &mut workspace)
        .unwrap();
    assert_eq!(*executed.output(), 1);
    assert_eq!(executed.receipt().output_identity(), "u64:1");
    assert_eq!(executed.counters().primary_read_contacts, 1);
    assert_eq!(executed.counters().publication_checks, 0);
}

#[test]
fn installed_parameter_contract_denies_before_graph_or_executor_work() {
    let mut workspace = workspace("installed-parameter-denial", false).unwrap();
    let installed_domain = workspace.domain(GeometryDomain).unwrap();
    let bound = workspace
        .operating_world(observation_basis())
        .family(ReadFamily)
        .bind(&installed_domain, CountVertices)
        .unwrap();
    let denial = match bound.execute(CountVerticesInput { minimum: None }, &mut workspace) {
        TransitionOutcome::Denied(denial) => denial,
        _ => panic!("missing required operation parameter did not produce an exact denial"),
    };
    assert_eq!(
        denial.kind(),
        &domain::WorthQueryBoundExecutionDenialKind::InputContract
    );
    assert_eq!(denial.counters().input_contract_checks, 1);
    assert_eq!(denial.counters().graph_provider_contacts, 0);
    assert_eq!(denial.counters().executor_contacts, 0);
}

#[test]
fn declared_primary_read_cannot_be_skipped_by_a_terminal_executor() {
    let mut workspace = missing_read_execution_workspace("installed-skipped-read").unwrap();
    let installed_domain = workspace.domain(GeometryDomain).unwrap();
    let bound = workspace
        .operating_world(observation_basis())
        .family(ReadFamily)
        .bind(&installed_domain, CountVertices)
        .unwrap();
    let denial = match bound.execute(CountVerticesInput { minimum: Some(0) }, &mut workspace) {
        TransitionOutcome::Failed(denial) => denial,
        _ => panic!("skipped primary read did not produce an execution failure"),
    };
    assert_eq!(
        denial.kind(),
        &domain::WorthQueryBoundExecutionDenialKind::UndeclaredFailureClass(
            domain::WorthQueryOperationFailureClass::Indeterminate,
        )
    );
    assert_eq!(denial.counters().primary_read_contacts, 0);
}

#[test]
fn foreign_workspace_denies_before_graph_or_executor_work() {
    let owner = workspace("installed-execution-owner", false).unwrap();
    let installed_domain = owner.domain(GeometryDomain).unwrap();
    let bound = owner
        .operating_world(observation_basis())
        .family(ReadFamily)
        .bind(&installed_domain, ReadVertex)
        .unwrap();
    let mut foreign = workspace("installed-execution-foreign", false).unwrap();
    let denial = match bound.execute(ReadExecutionInput::default(), &mut foreign) {
        TransitionOutcome::Denied(denial) => denial,
        _ => panic!("foreign runtime did not produce an exact denial"),
    };
    assert_eq!(
        denial.kind(),
        &domain::WorthQueryBoundExecutionDenialKind::RuntimeAuthority(
            domain::WorthQueryDomainHandleDenialKind::ForeignRuntime,
        )
    );
    assert_eq!(denial.counters().graph_provider_contacts, 0);
    assert_eq!(denial.counters().executor_contacts, 0);
}

#[test]
fn equivalent_but_distinct_bound_contract_cannot_splice_the_chain() {
    let mut workspace = workspace("installed-progression-splice", false).unwrap();
    let installed_domain = workspace.domain(GeometryDomain).unwrap();
    let first = workspace
        .operating_world(observation_basis())
        .family(ReadFamily)
        .bind(&installed_domain, ReadVertex)
        .unwrap();
    let second = workspace
        .operating_world(observation_basis())
        .family(ReadFamily)
        .bind(&installed_domain, ReadVertex)
        .unwrap();
    assert_eq!(first.binding_identity(), second.binding_identity());
    let foreign_contract = second.consumer_projection_contract().unwrap();
    let published = first
        .execute(ReadExecutionInput::default(), &mut workspace)
        .unwrap()
        .publish()
        .unwrap();
    assert!(matches!(
        published.consume(foreign_contract, read::project_facts().entity_identities()),
        TransitionOutcome::Denied(domain::WorthQueryProgressionDenial::ConsumerContractMismatch)
    ));
}

#[test]
fn result_state_and_warning_postures_survive_through_settlement() {
    let states = [
        domain::WorthQueryOperationResultState::Ready,
        domain::WorthQueryOperationResultState::Advisory,
        domain::WorthQueryOperationResultState::Pending,
        domain::WorthQueryOperationResultState::Partial,
        domain::WorthQueryOperationResultState::Violation,
    ];
    for (index, state) in states.into_iter().enumerate() {
        let mut workspace = workspace(&format!("installed-posture-{index}"), false).unwrap();
        let installed_domain = workspace.domain(GeometryDomain).unwrap();
        let bound = workspace
            .operating_world(observation_basis())
            .family(ReadFamily)
            .bind(&installed_domain, ReadVertex)
            .unwrap();
        let consumer = bound.consumer_projection_contract().unwrap();
        let warning =
            domain::WorthQueryOperationExecutionWarning::Advisory(format!("posture-{index}"));
        let settled = bound
            .execute(
                ReadExecutionInput {
                    state,
                    warning: Some(warning.clone()),
                    failure: None,
                },
                &mut workspace,
            )
            .unwrap()
            .publish()
            .unwrap()
            .consume(consumer, read::project_facts().entity_identities())
            .unwrap()
            .settle()
            .unwrap();

        assert_eq!(settled.result_state(), state);
        assert_eq!(settled.warnings(), [warning]);
    }
}

#[test]
fn ordinary_executor_failures_are_typed_and_must_be_declared() {
    let declared = operation_failure(
        "ordinary-declared-failure",
        domain::WorthQueryOperationFailureClass::Dependency,
    );
    assert_eq!(
        declared.kind(),
        &domain::WorthQueryBoundExecutionDenialKind::Executor(
            domain::WorthQueryOperationFailureClass::Dependency,
        )
    );
    let undeclared = operation_failure(
        "ordinary-undeclared-failure",
        domain::WorthQueryOperationFailureClass::Unsupported,
    );
    assert_eq!(
        undeclared.kind(),
        &domain::WorthQueryBoundExecutionDenialKind::UndeclaredFailureClass(
            domain::WorthQueryOperationFailureClass::Unsupported,
        )
    );
}

fn operation_failure(
    name: &str,
    class: domain::WorthQueryOperationFailureClass,
) -> domain::WorthQueryBoundExecutionDenial {
    let mut workspace = workspace(name, false).unwrap();
    let installed_domain = workspace.domain(GeometryDomain).unwrap();
    let bound = workspace
        .operating_world(observation_basis())
        .family(ReadFamily)
        .bind(&installed_domain, ReadVertex)
        .unwrap();
    match bound.execute(
        ReadExecutionInput {
            failure: Some(class),
            ..Default::default()
        },
        &mut workspace,
    ) {
        TransitionOutcome::Failed(denial) => denial,
        _ => panic!("deliberately failing executor did not produce an execution failure"),
    }
}

#[test]
fn unsupported_primary_read_basis_denies_before_execution_work() {
    let workspace = workspace("installed-foreign-basis-material", false).unwrap();
    let installed_domain = workspace.domain(GeometryDomain).unwrap();
    let branch_basis = foundation::basis_lifecycle()
        .branch_head("branch:foreign-material", true)
        .for_observation()
        .unwrap()
        .admit()
        .unwrap()
        .capability()
        .clone();
    let denial = match workspace
        .operating_world(branch_basis)
        .family(ReadFamily)
        .bind(&installed_domain, ReadVertex)
    {
        Ok(_) => panic!("unsupported primary-read basis unexpectedly bound"),
        Err(denial) => denial,
    };

    assert_eq!(
        denial.kind(),
        domain::WorthQueryOperationBindingDenialKind::BasisExecutionUnsupported
    );
    assert_eq!(denial.counters().graph_provider_contacts, 0);
    assert_eq!(denial.counters().planning_steps, 0);
}

#[test]
fn same_shaped_read_from_a_foreign_runtime_cannot_publish() {
    let mut workspace = foreign_material_workspace("foreign-runtime-material-owner").unwrap();
    let installed_domain = workspace.domain(GeometryDomain).unwrap();
    let executed = workspace
        .operating_world(observation_basis())
        .family(ReadFamily)
        .bind(&installed_domain, ReadVertex)
        .unwrap()
        .execute(ReadExecutionInput::default(), &mut workspace)
        .unwrap();
    assert_eq!(executed.counters().executor_contacts, 1);
    assert!(matches!(
        executed.publish(),
        TransitionOutcome::Denied(domain::WorthQueryPublicationDenial::ExecutionMaterialMismatch)
    ));
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
