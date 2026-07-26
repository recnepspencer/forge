use std::collections::BTreeMap;

use worth_query_admission::facade::resource_admission::WorthQueryAdmittedWorkflowResourcePlan;
use worth_query_admission::integration::reserve_workflow_resource_plan;

use super::tests::{admitted_plan, runtime};
use super::WorthQueryWorkflowExecutionResourceAttempt;
use crate::domain_computation::operation_binding::workflow_authority;

#[test]
fn workflow_attempt_mints_only_the_operation_session() {
    let operation = admitted_plan("workflow", 8);
    let stage = admitted_plan("workflow-stage", 4);
    let mut stages = BTreeMap::new();
    stages.insert("stage".to_owned(), stage);
    let resources = WorthQueryAdmittedWorkflowResourcePlan::assemble(operation, stages);
    let runtime = runtime();
    let authority = workflow_authority(&runtime, &resources);
    let reserved = reserve_workflow_resource_plan(resources).unwrap();
    let attempt = WorthQueryWorkflowExecutionResourceAttempt::start(reserved, &authority);

    assert_eq!(attempt.resources().counters().provider_session_mints, 1);
    assert_eq!(
        attempt
            .operation_resources()
            .counters()
            .provider_session_mints,
        1
    );
    assert_eq!(
        attempt
            .resources()
            .stage("stage")
            .unwrap()
            .counters()
            .provider_session_mints,
        0
    );
    assert_eq!(
        attempt.provider_session().attempt_identity(),
        attempt.attempt_identity().as_str()
    );
    assert_eq!(
        attempt.evidence().admission_identity(),
        attempt.operation_resources().identity()
    );

    let direct_denial = match attempt
        .provider_session()
        .bind_direct_domain_evidence("snapshot:1", "output:1")
    {
        Err(denial) => denial,
        Ok(_) => panic!("workflow session minted direct evidence binding"),
    };
    assert_eq!(
        direct_denial,
        crate::domain_computation::WorthQueryDomainEvidenceBindingDenial::DirectOperationRequired
    );
    let binding = attempt
        .provider_session()
        .bind_workflow_stage_domain_evidence("run:1", "stage", "snapshot:1", "output:1")
        .unwrap();
    assert_eq!(binding.run_identity(), Some("run:1"));
    assert_eq!(binding.stage_identity(), Some("stage"));
    let stage_denial = match attempt
        .provider_session()
        .bind_workflow_stage_domain_evidence("run:1", "foreign-stage", "snapshot:1", "output:1")
    {
        Err(denial) => denial,
        Ok(_) => panic!("workflow session minted evidence for a foreign stage"),
    };
    assert_eq!(
        stage_denial,
        crate::domain_computation::WorthQueryDomainEvidenceBindingDenial::StageNotInstalled
    );

    let artifacts = attempt.bind_workflow_artifacts().unwrap();
    assert!(!artifacts.run_identity().is_empty());
    assert_ne!(artifacts.run_identity(), "run:1");
    assert_eq!(
        artifacts.registry().run_identity(),
        artifacts.run_identity()
    );
    assert!(artifacts.production_authority("stage").unwrap().is_none());
    assert!(artifacts.access_authority("stage").unwrap().is_none());
    let transfer_denial = match artifacts.transfer_admission("foreign-stage", "stage") {
        Err(denial) => denial,
        Ok(_) => panic!("artifact authority admitted an undeclared workflow edge"),
    };
    assert_eq!(
        transfer_denial.kind(),
        crate::domain_computation::WorthQueryArtifactDenialKind::StageMismatch
    );
}

#[test]
fn workflow_attempt_owns_at_most_one_live_artifact_run() {
    let operation = admitted_plan("single-live-workflow-run", 8);
    let resources = WorthQueryAdmittedWorkflowResourcePlan::assemble(operation, BTreeMap::new());
    let runtime = runtime();
    let authority = workflow_authority(&runtime, &resources);
    let reserved = reserve_workflow_resource_plan(resources).unwrap();
    let attempt = WorthQueryWorkflowExecutionResourceAttempt::start(reserved, &authority);

    let first = attempt.bind_workflow_artifacts().unwrap();
    let denial = match attempt.bind_workflow_artifacts() {
        Err(denial) => denial,
        Ok(_) => panic!("one resource attempt minted two live artifact runs"),
    };
    assert_eq!(
        denial.kind(),
        crate::domain_computation::WorthQueryArtifactDenialKind::ActiveWorkflowRun
    );

    first.registry().close_cancelled();
    let replacement = attempt.bind_workflow_artifacts().unwrap();
    assert_ne!(replacement.run_identity(), first.run_identity());
}
