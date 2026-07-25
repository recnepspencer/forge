use std::sync::Arc;

use worth_query_admission::facade::resource_admission::{
    WorthQueryAdmittedExecutionResourcePlan, WorthQueryExecutionResourceAdmissionCounters,
    WorthQueryExecutionResourceAdmissionDenialKind, WorthQueryExecutionResourceSupport,
    WorthQueryExecutionResourceSupportSnapshot, WorthQueryFixedExecutionCapacity,
};
use worth_query_admission::integration::admit_execution_resource_plan;
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

use super::topology::{test_topology, WorthQueryExecutionResourceTopology};
use super::WorthQueryExecutionBoundOperationAuthority;
use crate::domain_computation::execution_runtime::WorthQueryExecutionRuntimeInstaller;
use crate::domain_computation::operation_binding::{
    WorthQueryInstalledDomainExecutionAuthority, WorthQueryInstalledOperationExecutionSupport,
};

#[test]
fn resource_attempt_requires_the_exact_binding_and_installed_contract() {
    let runtime = runtime();
    let (foreign_binding, contract_identity) = admitted_plan("foreign-binding");
    let installed_authority = authority(
        &runtime,
        "installed-binding",
        &contract_identity,
        &foreign_binding,
    );

    let denial = match runtime.start_direct_resource_attempt(&installed_authority, foreign_binding)
    {
        Err(denial) => denial,
        Ok(_) => panic!("foreign resource-plan binding started an execution attempt"),
    };

    assert_eq!(
        denial.kind(),
        &WorthQueryExecutionResourceAdmissionDenialKind::ResourcePlanAuthorityMismatch
    );
    assert_eq!(denial.counters().capacity_reservation_checks, 0);
    assert_eq!(denial.counters().provider_session_mints, 0);

    let (foreign_contract, _) = admitted_plan("installed-binding");
    let foreign_contract_authority = authority(
        &runtime,
        "installed-binding",
        "another-contract",
        &foreign_contract,
    );
    let denial = match runtime
        .start_direct_resource_attempt(&foreign_contract_authority, foreign_contract)
    {
        Err(denial) => denial,
        Ok(_) => panic!("foreign resource contract started an execution attempt"),
    };
    assert_eq!(
        denial.kind(),
        &WorthQueryExecutionResourceAdmissionDenialKind::ResourcePlanAuthorityMismatch
    );
    assert_eq!(denial.counters().capacity_reservation_checks, 0);
    assert_eq!(denial.counters().provider_session_mints, 0);
}

#[test]
fn exact_resource_plan_binding_can_start_one_direct_attempt() {
    let runtime = runtime();
    let (plan, contract_identity) = admitted_plan("installed-binding");
    let authority = authority(&runtime, "installed-binding", &contract_identity, &plan);

    let attempt = runtime
        .start_direct_resource_attempt(&authority, plan)
        .unwrap();

    assert_eq!(attempt.resources().counters().capacity_reservations, 1);
    assert_eq!(attempt.resources().counters().provider_session_mints, 1);
}

#[test]
fn stale_bound_operation_cannot_start_an_execution_attempt() {
    let mut runtime = runtime();
    let (plan, contract_identity) = admitted_plan("installed-binding");
    let authority = authority(&runtime, "installed-binding", &contract_identity, &plan);
    runtime
        .commit_successor_installation(Arc::new(
            runtime.installed_packages().successor_generation(),
        ))
        .unwrap();

    let denial = match runtime.start_direct_resource_attempt(&authority, plan) {
        Err(denial) => denial,
        Ok(_) => panic!("stale bound operation started an execution attempt"),
    };

    assert_eq!(
        denial.kind(),
        &WorthQueryExecutionResourceAdmissionDenialKind::RuntimeAuthority(
            worth_query_installation::facade::WorthQueryDomainHandleDenialKind::StaleInstallationGeneration,
        )
    );
    assert_eq!(denial.counters().capacity_reservation_checks, 0);
    assert_eq!(denial.counters().capacity_reservations, 0);
    assert_eq!(denial.counters().provider_session_mints, 0);
}

#[test]
fn resource_attempt_rejects_omitted_installed_support_participants() {
    for (conditional_nodes, graph_providers, commit_groups) in [
        (&["operation:gate"][..], &[][..], &[][..]),
        (&[][..], &["geometry"][..], &[][..]),
        (&[][..], &[][..], &["geometry,labels"][..]),
    ] {
        let runtime = runtime();
        let (plan, contract_identity) = admitted_plan("installed-binding");
        let mut authority = authority(&runtime, "installed-binding", &contract_identity, &plan);
        authority.direct_resource_topology = topology(
            conditional_nodes.iter().copied(),
            graph_providers.iter().copied(),
            commit_groups.iter().copied(),
        );

        let denial = match runtime.start_direct_resource_attempt(&authority, plan) {
            Err(denial) => denial,
            Ok(_) => panic!("an omitted installed participant started an execution attempt"),
        };

        assert_eq!(
            denial.kind(),
            &WorthQueryExecutionResourceAdmissionDenialKind::ResourcePlanAuthorityMismatch
        );
        assert_eq!(denial.counters().capacity_reservation_checks, 0);
        assert_eq!(denial.counters().provider_session_mints, 0);
    }
}

#[test]
fn resource_attempt_rejects_caller_reconstructed_support() {
    let runtime = runtime();
    let (installed_plan, contract_identity) =
        admitted_plan_with_support_limit("installed-binding", 2);
    let (reconstructed_plan, reconstructed_contract_identity) =
        admitted_plan_with_support_limit("installed-binding", 8);
    assert_eq!(contract_identity, reconstructed_contract_identity);
    let authority = authority(
        &runtime,
        "installed-binding",
        &contract_identity,
        &installed_plan,
    );

    let denial = match runtime.start_direct_resource_attempt(&authority, reconstructed_plan) {
        Err(denial) => denial,
        Ok(_) => panic!("caller-reconstructed support started an execution attempt"),
    };

    assert_eq!(
        denial.kind(),
        &WorthQueryExecutionResourceAdmissionDenialKind::ResourcePlanAuthorityMismatch
    );
    assert_eq!(denial.counters().capacity_reservation_checks, 0);
    assert_eq!(denial.counters().provider_session_mints, 0);
}

fn topology<'a>(
    conditional_nodes: impl Iterator<Item = &'a str>,
    graph_providers: impl Iterator<Item = &'a str>,
    commit_groups: impl Iterator<Item = &'a str>,
) -> WorthQueryExecutionResourceTopology {
    test_topology(conditional_nodes, graph_providers, commit_groups)
}

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

fn authority(
    runtime: &crate::domain_computation::WorthQueryExecutionRuntime,
    binding_identity: &str,
    contract_identity: &str,
    plan: &WorthQueryAdmittedExecutionResourcePlan,
) -> WorthQueryExecutionBoundOperationAuthority {
    WorthQueryExecutionBoundOperationAuthority {
        runtime_authority: runtime.authority_identity(),
        installation_runtime_ordinal: runtime.installed_packages().runtime_ordinal(),
        binding_identity: Arc::from(binding_identity),
        operation_identity: Arc::from("installed-operation"),
        basis_identity: Arc::from("installed-basis"),
        canonical_query_digest: Arc::from("installed-query"),
        operation_resource_contract_identity: Arc::from(contract_identity),
        commit_posture:
            crate::domain_computation::operation_binding::WorthQueryExecutionCommitPosture::ReadOnly,
        direct_resource_topology: Default::default(),
        workflow_stage_resources: None,
        operation_evidence_contract: None,
        installed_support: WorthQueryInstalledOperationExecutionSupport::direct(
            plan.support_snapshot().clone(),
        ),
        installed_domain: WorthQueryInstalledDomainExecutionAuthority::mint(
            runtime.authority_identity(),
            "test-domain",
            WorthQueryInstallationGeneration::initial(),
            runtime.retain_current_generation(),
        ),
    }
}

pub(crate) fn direct_authority(
    runtime: &crate::domain_computation::WorthQueryExecutionRuntime,
    plan: &WorthQueryAdmittedExecutionResourcePlan,
) -> WorthQueryExecutionBoundOperationAuthority {
    authority(
        runtime,
        plan.binding_identity(),
        plan.contract_identity(),
        plan,
    )
}

pub(crate) fn direct_authority_with_graph(
    runtime: &crate::domain_computation::WorthQueryExecutionRuntime,
    plan: &WorthQueryAdmittedExecutionResourcePlan,
    graph: &worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority,
    access: worth_query_installation::facade::WorthQueryOperationGraphAccess,
) -> WorthQueryExecutionBoundOperationAuthority {
    let mut authority = direct_authority(runtime, plan);
    authority.installation_runtime_ordinal = graph.runtime_ordinal();
    authority.direct_resource_topology = super::topology::resource_topology(
        std::iter::empty(),
        &[graph],
        std::iter::once((graph.role(), access)),
        std::iter::empty(),
        crate::domain_computation::operation_binding::WorthQueryExecutionCommitPosture::ReadOnly,
    );
    authority
}

pub(crate) fn workflow_authority(
    runtime: &crate::domain_computation::WorthQueryExecutionRuntime,
    plan: &worth_query_admission::facade::resource_admission::WorthQueryAdmittedWorkflowResourcePlan,
) -> WorthQueryExecutionBoundOperationAuthority {
    let stages = plan
        .stages()
        .map(|(stage, resources)| {
            (
                Arc::<str>::from(stage),
                super::WorthQueryWorkflowStageResourceAuthority {
                    contract_identity: Arc::from(resources.contract_identity()),
                    topology: Default::default(),
                    predecessors: Arc::from([]),
                    artifact_contracts:
                        crate::domain_computation::artifact_owner::WorthQueryInstalledWorkflowArtifactContracts::empty(),
                },
            )
        })
        .collect();
    let support = WorthQueryInstalledOperationExecutionSupport::workflow(
        plan.operation().support_snapshot().clone(),
        plan.stages()
            .map(|(stage, resources)| (stage.to_owned(), resources.support_snapshot().clone())),
    );
    WorthQueryExecutionBoundOperationAuthority {
        runtime_authority: runtime.authority_identity(),
        installation_runtime_ordinal: runtime.installed_packages().runtime_ordinal(),
        binding_identity: Arc::from(plan.operation().binding_identity()),
        operation_identity: Arc::from("installed-workflow-operation"),
        basis_identity: Arc::from("installed-basis"),
        canonical_query_digest: Arc::from("installed-query"),
        operation_resource_contract_identity: Arc::from(plan.operation().contract_identity()),
        commit_posture:
            crate::domain_computation::operation_binding::WorthQueryExecutionCommitPosture::ReadOnly,
        direct_resource_topology: Default::default(),
        workflow_stage_resources: Some(stages),
        operation_evidence_contract: None,
        installed_support: support,
        installed_domain: WorthQueryInstalledDomainExecutionAuthority::mint(
            runtime.authority_identity(),
            "test-domain",
            WorthQueryInstallationGeneration::initial(),
            runtime.retain_current_generation(),
        ),
    }
}

fn admitted_plan(binding_identity: &str) -> (WorthQueryAdmittedExecutionResourcePlan, String) {
    admitted_plan_with_support_limit(binding_identity, 2)
}

fn admitted_plan_with_support_limit(
    binding_identity: &str,
    support_limit: u64,
) -> (WorthQueryAdmittedExecutionResourcePlan, String) {
    let safe_point = WorthQueryCancellationSafePointFamily::new("operation-boundary").unwrap();
    let envelope = WorthQueryExecutionResourceEnvelope::new(
        WorthQuerySemanticScaleRequest::bounded(2),
        WorthQueryResourceLimitRequest::bounded(2),
        WorthQueryExecutionMode::Synchronous,
        None,
        safe_point.clone(),
    );
    let provider = WorthQueryExecutionProviderFamily::new("installed-provider").unwrap();
    let access = WorthQueryExecutionAccessProductFamily::new("installed-access").unwrap();
    let allocator = WorthQueryExecutionAllocatorFamily::new("installed-arena").unwrap();
    let contract =
        WorthQueryExecutionResourceContract::declared([WorthQueryExecutionStrategyContract::new(
            WorthQueryExecutionStrategyName::new("installed-strategy").unwrap(),
            envelope.clone(),
            WorthQueryExecutionProviderRequirements::new(
                provider.clone(),
                access.clone(),
                allocator.clone(),
            ),
        )])
        .unwrap();
    let contract_identity = contract.canonical_identity();
    let support = WorthQueryExecutionResourceSupportSnapshot::new(
        WorthQueryExecutionResourceSupport::new(
            provider,
            access,
            allocator,
            WorthQueryExecutionResourceEnvelope::new(
                WorthQuerySemanticScaleRequest::bounded(support_limit),
                WorthQueryResourceLimitRequest::bounded(support_limit),
                WorthQueryExecutionMode::Synchronous,
                None,
                safe_point.clone(),
            ),
            Arc::new(WorthQueryFixedExecutionCapacity::mint("operation-binding-test", 8).unwrap()),
        ),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        None,
    );
    let plan = admit_execution_resource_plan(
        binding_identity,
        &contract,
        &WorthQueryExecutionResourceRequest::bounded(2, 2, safe_point),
        support,
        WorthQueryExecutionResourceAdmissionCounters::default(),
    )
    .unwrap();
    (plan, contract_identity)
}
