use forge_query::facade::{
    ForgeQueryGraphReadAccessAdmissionPosture, ForgeQueryGraphReadAccessDenialKind,
    ForgeQueryRuntimeError, ForgeQueryWorkspace,
};
use topology::certification::milestone_one_runtime_builder;
use topology::runtime_support::{topology_runtime, TopologyRuntimeAdapters};

use super::super::admitted_scaffold::{
    prepare_primitive_construction_admitted_artifact,
    prepare_primitive_construction_executed_admitted_artifact,
};
use super::super::intent::PrimitiveConstructionIntent;
use super::super::query_access_planning::{
    execute_planned_construction_query_access, execute_planned_topology_birth,
    plan_phase_chain_topology_check, plan_topology_birth, plan_topology_birth_broad_scan,
    primitive_construction_query_access_coverage, PrimitiveConstructionQueryAccessError,
    PrimitiveConstructionQueryAccessSurface,
};
use super::super::request::{
    PrimitiveConstructionFamily, PrimitiveConstructionRequest, PRIMITIVE_CONSTRUCTION_FAMILIES,
};
use super::super::specs::{
    OrthotopeSpec, RegularPrismSpec, RegularPyramidSpec, ShellWithHoleSpec, SimplexSolidSpec,
    WireBodySpec,
};

#[test]
fn topology_birth_construction_reads_execute_with_query_access_receipts_for_all_families() {
    let coverage = primitive_construction_query_access_coverage();
    assert_eq!(coverage.rows().len(), PRIMITIVE_CONSTRUCTION_FAMILIES.len());
    for row in coverage.rows() {
        assert!(PRIMITIVE_CONSTRUCTION_FAMILIES.contains(&row.family()));
        assert_eq!(
            row.surfaces(),
            [
                PrimitiveConstructionQueryAccessSurface::TopologyBirth,
                PrimitiveConstructionQueryAccessSurface::PhaseChainTopologyCheck
            ]
        );
    }

    for request in admitted_primitive_construction_requests() {
        let family = request.family();
        let mut planned_workspace = primitive_topology_workspace(&format!(
            "worth-kernel.phase-17.topology-birth.planned.{}",
            family.as_str()
        ));
        let mut direct_workspace = primitive_topology_workspace(&format!(
            "worth-kernel.phase-17.topology-birth.direct.{}",
            family.as_str()
        ));
        let admitted = prepare_primitive_construction_admitted_artifact(&request)
            .expect("request should admit before graph access planning");
        let planned = plan_topology_birth(&mut planned_workspace, &admitted)
            .expect("topology birth read should plan");
        let planned_digest = planned.plan_digest().to_string();
        let direct_artifact = prepare_primitive_construction_executed_admitted_artifact(
            &mut direct_workspace,
            &request,
        )
        .expect("direct compose should remain certified");

        let consumed_access = execute_planned_topology_birth(&mut planned_workspace, planned)
            .expect("planned topology birth read should execute through Query");
        let receipt = consumed_access.receipt();
        let executed_artifact = prepare_primitive_construction_executed_admitted_artifact(
            &mut planned_workspace,
            &request,
        )
        .expect("compose should still use construction admission and topology obligation");

        assert_family_parity_after_planned_access(&executed_artifact, &direct_artifact, family);
        assert_consumed_access_receipt(receipt, &consumed_access, planned_digest.as_str());
    }
}

#[test]
fn phase_chain_topology_check_carries_its_own_planned_access_proof() {
    let mut workspace = primitive_topology_workspace("worth-kernel.phase-17.phase-chain-check");
    let request = PrimitiveConstructionIntent::regular_prism(RegularPrismSpec {
        sides: 6,
        radius: 1.0,
        height: 2.0,
    })
    .into_request();
    let admitted = prepare_primitive_construction_admitted_artifact(&request)
        .expect("regular prism should admit before graph access planning");
    let planned = plan_phase_chain_topology_check(&mut workspace, &admitted)
        .expect("phase-chain topology check should plan through Query");

    assert_eq!(
        planned.surface(),
        PrimitiveConstructionQueryAccessSurface::PhaseChainTopologyCheck
    );
    assert!(!planned.family_digest().is_empty());
    assert!(!planned.plan_digest().is_empty());
    assert!(matches!(
        planned.admission_posture(),
        ForgeQueryGraphReadAccessAdmissionPosture::InlineIndexed
            | ForgeQueryGraphReadAccessAdmissionPosture::BoundedEphemeralIndex
            | ForgeQueryGraphReadAccessAdmissionPosture::AdmittedPagedStreaming
    ));
    let consumed = execute_planned_construction_query_access(&mut workspace, planned)
        .expect("phase-chain planned check should execute through Query access plan");
    assert_eq!(
        consumed.receipt().surface(),
        PrimitiveConstructionQueryAccessSurface::PhaseChainTopologyCheck
    );
    assert!(consumed.receipt().no_caller_owned_graph_work());
}

#[test]
fn broad_construction_graph_scan_denies_before_local_materialization_or_escapes_typed() {
    let mut workspace = primitive_topology_workspace("worth-kernel.phase-17.broad-scan");
    let request = PrimitiveConstructionIntent::shell_with_hole(ShellWithHoleSpec {
        outer_loop_edge_count: 6,
        hole_loop_edge_counts: vec![3, 4],
    })
    .into_request();
    let admitted = prepare_primitive_construction_admitted_artifact(&request)
        .expect("shell-with-hole should admit before graph access planning");

    match plan_topology_birth_broad_scan(&mut workspace, &admitted, 64) {
        Ok(plan) => assert!(matches!(
            plan.admission_posture(),
            ForgeQueryGraphReadAccessAdmissionPosture::AdmittedPagedStreaming
                | ForgeQueryGraphReadAccessAdmissionPosture::BoundedEphemeralIndex
        )),
        Err(PrimitiveConstructionQueryAccessError::AccessDenied(denial)) => {
            assert!(!denial.admission_digest().is_empty());
            assert_eq!(
                denial.admission_posture(),
                &ForgeQueryGraphReadAccessAdmissionPosture::Denied
            );
            assert!(matches!(
                denial.denial_kind(),
                Some(ForgeQueryGraphReadAccessDenialKind::BudgetExceeded)
            ));
            assert_eq!(
                denial.suggested_posture(),
                Some(&ForgeQueryGraphReadAccessAdmissionPosture::AsyncMaterializationRequired)
            );
            assert_eq!(denial.executor_entry_count(), 0);
            assert_eq!(denial.materialized_row_count(), 0);
        }
        Err(error) => {
            panic!("broad construction scan should produce typed access posture: {error:?}")
        }
    }
}

fn admitted_primitive_construction_requests() -> [PrimitiveConstructionRequest; 6] {
    [
        PrimitiveConstructionIntent::simplex_solid(SimplexSolidSpec::new(1.0)).into_request(),
        PrimitiveConstructionIntent::orthotope(OrthotopeSpec {
            half_extents: [1.0, 2.0, 3.0],
        })
        .into_request(),
        PrimitiveConstructionIntent::regular_prism(RegularPrismSpec {
            sides: 6,
            radius: 1.0,
            height: 2.0,
        })
        .into_request(),
        PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
            sides: 5,
            radius: 1.0,
            height: 2.0,
        })
        .into_request(),
        PrimitiveConstructionIntent::wire_body(WireBodySpec { edge_count: 8 }).into_request(),
        PrimitiveConstructionIntent::shell_with_hole(ShellWithHoleSpec {
            outer_loop_edge_count: 6,
            hole_loop_edge_counts: vec![3, 4],
        })
        .into_request(),
    ]
}

fn assert_family_parity_after_planned_access(
    executed_artifact: &super::super::admitted_scaffold::PreparedPrimitiveConstructionAdmittedArtifact,
    direct_artifact: &super::super::admitted_scaffold::PreparedPrimitiveConstructionAdmittedArtifact,
    expected_family: PrimitiveConstructionFamily,
) {
    assert_eq!(executed_artifact.family(), expected_family);
    assert_eq!(executed_artifact.family(), direct_artifact.family());
    assert_eq!(
        executed_artifact.admitted_handoff_digest(),
        direct_artifact.admitted_handoff_digest()
    );
    assert_eq!(
        executed_artifact.birth_consequence_digest(),
        direct_artifact.birth_consequence_digest()
    );
    assert_eq!(
        executed_artifact.birth_mapping_digest(),
        direct_artifact.birth_mapping_digest()
    );
    assert_eq!(
        executed_artifact.realization_digest(),
        direct_artifact.realization_digest()
    );
    assert_eq!(
        planned_compose_program_digest(executed_artifact),
        planned_compose_program_digest(direct_artifact)
    );
}

fn assert_consumed_access_receipt(
    receipt: &super::super::query_access_planning::PrimitiveConstructionExecutedQueryAccessReceipt,
    consumed_access: &super::super::query_access_planning::PrimitiveConstructionConsumedQueryAccess,
    planned_digest: &str,
) {
    assert_eq!(
        receipt.surface(),
        PrimitiveConstructionQueryAccessSurface::TopologyBirth
    );
    let consumed_plan_digest = consumed_access
        .result()
        .receipt()
        .graph_read_access_plan()
        .map(|plan| plan.digest())
        .expect("consumed Query read result should retain graph access plan");
    assert_eq!(consumed_plan_digest, receipt.plan_digest());
    assert!(!receipt.family_digest().is_empty());
    assert_eq!(receipt.plan_digest(), planned_digest);
    assert!(!receipt.admission_digest().is_empty());
    assert!(matches!(
        receipt.admission_posture(),
        ForgeQueryGraphReadAccessAdmissionPosture::InlineIndexed
            | ForgeQueryGraphReadAccessAdmissionPosture::BoundedEphemeralIndex
            | ForgeQueryGraphReadAccessAdmissionPosture::AdmittedPagedStreaming
    ));
    assert!(!receipt.requirement_set_digest().is_empty());
    assert!(!receipt.cost_estimate_digest().is_empty());
    assert!(!receipt.budget_digest().is_empty());
    assert!(!receipt
        .graph_index_inventory_match_report_digest()
        .is_empty());
    assert!(!receipt.plan_consumption_digest().is_empty());
    assert_eq!(receipt.executor_entry_count(), 1);
    assert_eq!(receipt.materialized_row_count(), 1);
    assert!(receipt.no_caller_owned_graph_work());
}

#[test]
fn access_plan_digest_drift_denies_before_execution_binding() {
    let mut workspace = primitive_topology_workspace("worth-kernel.phase-17.plan-drift");
    let wire_request =
        PrimitiveConstructionIntent::wire_body(WireBodySpec { edge_count: 8 }).into_request();
    let prism_request = PrimitiveConstructionIntent::regular_prism(RegularPrismSpec {
        sides: 6,
        radius: 1.0,
        height: 2.0,
    })
    .into_request();
    let wire_artifact = prepare_primitive_construction_admitted_artifact(&wire_request)
        .expect("wire request should admit");
    let prism_artifact = prepare_primitive_construction_admitted_artifact(&prism_request)
        .expect("prism request should admit");
    let wire_plan = plan_topology_birth(&mut workspace, &wire_artifact)
        .expect("wire topology birth read should plan");
    let prism_plan = plan_topology_birth(&mut workspace, &prism_artifact)
        .expect("prism topology birth read should plan");

    let error = workspace
        .execute_read_family_with_access_plan(prism_plan.family(), wire_plan.plan().clone())
        .expect_err("mismatched access plan must deny before execution");

    match error {
        ForgeQueryRuntimeError::ReadCompositionDenied(denial) => {
            let mismatch = denial
                .access_plan_binding_mismatch()
                .expect("plan drift denial should carry typed mismatch proof");
            assert_eq!(mismatch.provided_plan_digest(), wire_plan.plan().digest());
            assert_eq!(
                mismatch.provided_admission_digest(),
                wire_plan.plan().admission().digest()
            );
            assert_eq!(
                mismatch.admitted_read_graph_digest(),
                wire_plan
                    .plan()
                    .admission()
                    .requirement_set()
                    .read_graph_digest()
            );
            assert_eq!(
                mismatch.execution_read_graph_digest(),
                prism_plan.family().read_graph().digest()
            );
            assert!(denial.graph_read_access_execution_counters().is_none());
        }
        other => panic!("expected read-composition denial for plan drift, got {other:?}"),
    }
}

fn primitive_topology_workspace(name: &str) -> ForgeQueryWorkspace {
    let runtime = milestone_one_runtime_builder()
        .expect("topology runtime builder should build")
        .build();
    topology_runtime(TopologyRuntimeAdapters::current_head(runtime), name)
        .expect("topology workspace should open")
}

fn planned_compose_program_digest(
    artifact: &super::super::admitted_scaffold::PreparedPrimitiveConstructionAdmittedArtifact,
) -> &str {
    artifact
        .topology_compose_evidence()
        .expect("executed artifact should carry topology compose evidence")
        .compose_program_digest()
}
