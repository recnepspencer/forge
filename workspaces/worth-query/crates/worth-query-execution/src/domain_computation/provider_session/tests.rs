use std::collections::BTreeMap;

use worth_query_admission::facade::resource_admission::{
    WorthQueryAdmittedExecutionResourcePlan, WorthQueryAdmittedWorkflowResourcePlan,
    WorthQueryExecutionResourceAdmissionCounters, WorthQueryExecutionResourceSupport,
    WorthQueryExecutionResourceSupportSnapshot, WorthQueryFixedExecutionCapacity,
};
use worth_query_admission::integration::{
    admit_execution_resource_plan, reserve_execution_resource_plan, reserve_workflow_resource_plan,
};
use worth_query_declaration::facade::domain_computation::{
    WorthQueryCancellationSafePointFamily, WorthQueryExecutionMode,
    WorthQueryExecutionResourceRequest, WorthQueryResourceLimitRequest,
    WorthQuerySemanticScaleRequest,
};
use worth_query_installation::facade::{
    WorthQueryExecutionAccessProductFamily, WorthQueryExecutionAllocatorFamily,
    WorthQueryExecutionProviderFamily, WorthQueryExecutionProviderRequirements,
    WorthQueryExecutionResourceContract, WorthQueryExecutionResourceEnvelope,
    WorthQueryExecutionStrategyContract, WorthQueryExecutionStrategyName,
    WorthQueryInstallationGeneration,
};

use super::{WorthQueryDirectExecutionResourceAttempt, WorthQueryWorkflowExecutionResourceAttempt};
use crate::domain_computation::execution_runtime::WorthQueryExecutionRuntimeInstaller;
use crate::domain_computation::operation_binding::{direct_authority, workflow_authority};

fn runtime() -> crate::domain_computation::WorthQueryExecutionRuntime {
    WorthQueryExecutionRuntimeInstaller::new()
        .install(
            WorthQueryInstallationGeneration::initial(),
            std::iter::empty(),
        )
        .unwrap()
        .into_parts()
        .0
}

fn safe_point() -> WorthQueryCancellationSafePointFamily {
    WorthQueryCancellationSafePointFamily::new("execution-chunk").unwrap()
}

fn provider() -> WorthQueryExecutionProviderFamily {
    WorthQueryExecutionProviderFamily::new("execution-provider").unwrap()
}

fn access() -> WorthQueryExecutionAccessProductFamily {
    WorthQueryExecutionAccessProductFamily::new("execution-access").unwrap()
}

fn allocator() -> WorthQueryExecutionAllocatorFamily {
    WorthQueryExecutionAllocatorFamily::new("execution-arena").unwrap()
}

fn envelope(limit: u64) -> WorthQueryExecutionResourceEnvelope {
    WorthQueryExecutionResourceEnvelope::new(
        WorthQuerySemanticScaleRequest::bounded(limit),
        WorthQueryResourceLimitRequest::bounded(limit),
        WorthQueryExecutionMode::Synchronous,
        None,
        safe_point(),
    )
}

pub(super) fn admitted_plan(binding: &str, limit: u64) -> WorthQueryAdmittedExecutionResourcePlan {
    admitted_plan_with_support_limit(binding, limit, limit)
}

fn admitted_plan_with_support_limit(
    binding: &str,
    limit: u64,
    support_limit: u64,
) -> WorthQueryAdmittedExecutionResourcePlan {
    let contract =
        WorthQueryExecutionResourceContract::declared([WorthQueryExecutionStrategyContract::new(
            WorthQueryExecutionStrategyName::new("bounded").unwrap(),
            envelope(limit),
            WorthQueryExecutionProviderRequirements::new(provider(), access(), allocator()),
        )])
        .unwrap();
    let request = WorthQueryExecutionResourceRequest::bounded(limit, limit, safe_point());
    let support = WorthQueryExecutionResourceSupportSnapshot::new(
        WorthQueryExecutionResourceSupport::new(
            provider(),
            access(),
            allocator(),
            envelope(support_limit),
            std::sync::Arc::new(
                WorthQueryFixedExecutionCapacity::new(
                    format!("provider-session-test:{binding}"),
                    8,
                )
                .unwrap(),
            ),
        ),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        None,
    );

    admit_execution_resource_plan(
        binding,
        &contract,
        &request,
        support,
        WorthQueryExecutionResourceAdmissionCounters::default(),
    )
    .unwrap()
}

pub(super) fn direct_attempt(
    binding: &str,
    limit: u64,
) -> WorthQueryDirectExecutionResourceAttempt {
    let resources = admitted_plan(binding, limit);
    let runtime = runtime();
    let authority = direct_authority(&runtime, &resources);
    let reserved = reserve_execution_resource_plan(resources).unwrap();
    WorthQueryDirectExecutionResourceAttempt::start(reserved, &authority)
}

#[test]
fn direct_attempt_mints_one_session_and_binds_immutable_evidence() {
    let attempt = direct_attempt("direct", 8);

    assert_eq!(attempt.resources().counters().provider_session_mints, 1);
    assert_eq!(
        attempt.provider_session().attempt_identity(),
        attempt.resources().identity()
    );
    assert_eq!(
        attempt.evidence().admission_identity(),
        attempt.resources().identity()
    );
    assert_eq!(
        attempt.evidence().provider_session_identity(),
        attempt.provider_session().identity()
    );
    assert_eq!(
        attempt.evidence().provider_session_attempt_identity(),
        attempt.provider_session().attempt_identity()
    );
}

#[test]
fn repeated_direct_attempts_receive_unique_provider_sessions() {
    let first = direct_attempt("repeat", 8);
    let second = direct_attempt("repeat", 8);

    assert_eq!(
        first.provider_session().attempt_identity(),
        second.provider_session().attempt_identity()
    );
    assert_ne!(
        first.provider_session().identity(),
        second.provider_session().identity()
    );
    assert_ne!(first.evidence().identity(), second.evidence().identity());
}

#[test]
fn provider_session_seals_direct_evidence_to_its_bound_operation() {
    let attempt = direct_attempt("direct-evidence", 8);
    let binding = attempt
        .provider_session()
        .bind_direct_domain_evidence("snapshot:1", "output:1")
        .unwrap();

    assert_eq!(binding.binding_identity(), "direct-evidence");
    assert_eq!(binding.operation_identity(), "installed-operation");
    assert_eq!(binding.basis_identity(), "installed-basis");
    assert_eq!(binding.run_identity(), None);
    assert_eq!(binding.stage_identity(), None);
    assert_eq!(binding.execution_snapshot_identity(), "snapshot:1");
    assert_eq!(binding.output_occurrence_identity(), "output:1");
}

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
        attempt.resources().identity()
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
