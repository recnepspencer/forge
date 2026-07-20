use worth_proof::TransitionOutcome;
use worth_query::facade::{domain, foundation, read};

use super::installed_operation_fixture::{
    workflow_workspace, GeometryDomain, ReadFamily, WorkflowRead,
};

#[test]
fn installed_dag_mints_one_query_owned_trace_and_publication() {
    let mut workspace = workflow_workspace("installed-workflow").unwrap();
    let installed_domain = workspace.domain(GeometryDomain).unwrap();
    let bound = workspace
        .operating_world(observation_basis())
        .family(ReadFamily)
        .bind(&installed_domain, WorkflowRead)
        .unwrap();
    let consumer = bound.consumer_projection_contract().unwrap();
    let run = bound.start_workflow().unwrap();
    let run = run
        .advance(
            "start",
            domain::WorthQueryWorkflowValue::NotRequired,
            &mut workspace,
        )
        .unwrap()
        .advance(
            "right",
            domain::WorthQueryWorkflowValue::Text("start".into()),
            &mut workspace,
        )
        .unwrap()
        .advance(
            "left",
            domain::WorthQueryWorkflowValue::Text("start".into()),
            &mut workspace,
        )
        .unwrap()
        .advance(
            "publish",
            domain::WorthQueryWorkflowValue::Text("join".into()),
            &mut workspace,
        )
        .unwrap();
    let trace = run.complete().unwrap();
    assert_eq!(trace.stage_receipts().len(), 4);
    assert_eq!(trace.stage_receipts()[0].predecessor_proof_count(), 0);
    assert_eq!(trace.stage_receipts()[1].predecessor_proof_count(), 1);
    assert_eq!(trace.stage_receipts()[2].predecessor_proof_count(), 1);
    assert_eq!(trace.stage_receipts()[3].predecessor_proof_count(), 2);
    let counters = trace.counters();
    assert_eq!(counters.stage_index_lookups, 4);
    assert_eq!(counters.predecessor_checks, 4);
    assert_eq!(counters.predecessor_receipt_lookups, 4);
    assert_eq!(counters.required_capability_checks, 0);
    assert_eq!(counters.required_domain_checks, 0);
    assert_eq!(counters.graph_read_contacts, 1);
    assert_eq!(counters.touch_effect_contacts, 0);
    assert_eq!(counters.commit_admission_contacts, 0);
    assert_eq!(counters.invariant_checks, 0);
    assert_eq!(counters.parallel_admission_checks, 0);
    assert_eq!(counters.unrelated_run_scans, 0);
    let settled = trace
        .publish()
        .unwrap()
        .consume(consumer, read::project_facts().entity_identities())
        .unwrap()
        .settle()
        .unwrap();
    assert_eq!(settled.trace().stage_receipts().len(), 4);
    assert_eq!(settled.counters().consumption_contacts, 1);
    assert!(!settled.authority().receipt().receipt_digest().is_empty());
    assert!(!settled.identity().is_empty());
}

#[test]
fn skipping_a_predecessor_denies_before_stage_executor_contact() {
    let mut workspace = workflow_workspace("installed-workflow-skip").unwrap();
    let installed_domain = workspace.domain(GeometryDomain).unwrap();
    let run = workspace
        .operating_world(observation_basis())
        .family(ReadFamily)
        .bind(&installed_domain, WorkflowRead)
        .unwrap()
        .start_workflow()
        .unwrap();
    let denial = match run.advance(
        "publish",
        domain::WorthQueryWorkflowValue::Text("skip".into()),
        &mut workspace,
    ) {
        TransitionOutcome::Denied(denial) => denial,
        _ => panic!("skipped predecessor did not produce an exact denial"),
    };
    assert_eq!(
        denial.kind(),
        &domain::WorthQueryWorkflowAdvanceDenialKind::PredecessorIncomplete("left".into())
    );
    assert_eq!(denial.counters().stage_executor_contacts, 0);
    assert_eq!(denial.counters().graph_read_contacts, 0);
    assert_eq!(denial.counters().touch_effect_contacts, 0);
    assert_eq!(denial.counters().commit_admission_contacts, 0);
}

#[test]
fn duplicate_stage_advancement_denies_without_a_second_executor_contact() {
    let mut workspace = workflow_workspace("installed-workflow-duplicate-advance").unwrap();
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
    let denial = match run.advance(
        "start",
        domain::WorthQueryWorkflowValue::NotRequired,
        &mut workspace,
    ) {
        TransitionOutcome::Denied(denial) => denial,
        _ => panic!("duplicate stage did not produce an exact denial"),
    };

    assert_eq!(
        denial.kind(),
        &domain::WorthQueryWorkflowAdvanceDenialKind::StageAlreadyCompleted
    );
    assert_eq!(denial.counters().stage_executor_contacts, 1);
}

#[test]
fn foreign_runtime_denies_stage_progression_before_executor_contact() {
    let owner = workflow_workspace("installed-workflow-owner").unwrap();
    let installed_domain = owner.domain(GeometryDomain).unwrap();
    let run = owner
        .operating_world(observation_basis())
        .family(ReadFamily)
        .bind(&installed_domain, WorkflowRead)
        .unwrap()
        .start_workflow()
        .unwrap();
    let mut foreign = workflow_workspace("installed-workflow-foreign").unwrap();
    let denial = match run.advance(
        "start",
        domain::WorthQueryWorkflowValue::NotRequired,
        &mut foreign,
    ) {
        TransitionOutcome::Denied(denial) => denial,
        _ => panic!("foreign runtime did not produce an exact denial"),
    };

    assert_eq!(
        denial.kind(),
        &domain::WorthQueryWorkflowAdvanceDenialKind::RuntimeAuthority(
            domain::WorthQueryDomainHandleDenialKind::ForeignRuntime,
        )
    );
    assert_eq!(denial.counters().stage_executor_contacts, 0);
    assert_eq!(denial.counters().graph_read_contacts, 0);
    assert_eq!(denial.counters().touch_effect_contacts, 0);
    assert_eq!(denial.counters().commit_admission_contacts, 0);
}

#[test]
fn independent_stage_order_converges_to_one_semantic_trace() {
    let left_first = complete_trace("workflow-left-first", ["left", "right"]);
    let right_first = complete_trace("workflow-right-first", ["right", "left"]);
    assert_eq!(left_first, right_first);
}

fn complete_trace(name: &str, order: [&str; 2]) -> String {
    let mut workspace = workflow_workspace(name).unwrap();
    let installed_domain = workspace.domain(GeometryDomain).unwrap();
    let mut run = workspace
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
    for stage in order {
        run = run
            .advance(
                stage,
                domain::WorthQueryWorkflowValue::Text("start".into()),
                &mut workspace,
            )
            .unwrap();
    }
    run.advance(
        "publish",
        domain::WorthQueryWorkflowValue::Text("join".into()),
        &mut workspace,
    )
    .unwrap()
    .complete()
    .unwrap()
    .semantic_identity()
    .to_string()
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
