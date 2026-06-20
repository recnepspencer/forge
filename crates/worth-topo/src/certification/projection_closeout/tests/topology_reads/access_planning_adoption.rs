use std::collections::BTreeSet;
use std::path::PathBuf;

use forge_query::facade::consumer_kit::{
    graph_read_bypass_adoption, graph_read_bypass_audit, query_boundary_source_inventory,
    ForgeQueryGraphReadBypassResidueManifest,
};
use forge_query::facade::{
    ForgeQueryGraphReadAccessAdmissionPosture, ForgeQueryGraphReadAccessDenialKind,
};
use schema::facade::platform::relations::TopologyRelationKind;
use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;

use super::support::{
    current_head_query_handle, current_lookup_rows, seeded_primitive_workspace,
    seeded_sheet_disk_workspace,
};
use crate::projection::read_views::domain::read_proof::parity::{
    build_topology_read_view_parity_artifact, TopologyReadViewRef,
};
use crate::projection::read_views::domain::request::{
    TopologyReadAnchorIdentity, TopologyReadRequest,
};
use crate::projection::read_views::domain::{
    TopologyReadErrorKind, TopologyReadRequestFamily, TopologyReadRequestReport,
};
use crate::projection::runtime_boundary::read_execution::{
    execute_loop_cycle_read, TopologyReadExecutionTarget,
};
use crate::query_domain::TopologyCurrentHeadReadHandleExt;

#[derive(Debug, Clone, PartialEq, Eq)]
struct PhaseSixteenGraphAccessAdoptionCertification {
    covered_families: Vec<TopologyReadRequestFamily>,
    source_inventory_count: usize,
}

impl PhaseSixteenGraphAccessAdoptionCertification {
    fn certify(
        reports: &[TopologyReadRequestReport],
        source_inventory_count: usize,
    ) -> Option<Self> {
        if source_inventory_count == 0 {
            return None;
        }
        let covered = phase_sixteen_graph_access_proven_request_families(reports);
        if !TopologyReadRequestFamily::ALL
            .iter()
            .all(|family| covered.contains(family))
        {
            return None;
        }
        Some(Self {
            covered_families: covered.into_iter().collect(),
            source_inventory_count,
        })
    }

    fn covered_families(&self) -> &[TopologyReadRequestFamily] {
        self.covered_families.as_slice()
    }

    fn source_inventory_count(&self) -> usize {
        self.source_inventory_count
    }
}

fn phase_sixteen_graph_access_proven_request_families(
    reports: &[TopologyReadRequestReport],
) -> BTreeSet<TopologyReadRequestFamily> {
    reports
        .iter()
        .filter(|report| report.graph_access_proof().is_some())
        .map(TopologyReadRequestReport::request_family)
        .collect()
}

#[test]
fn phase_sixteen_migrated_topology_reads_carry_access_plan_receipts() {
    let (mut fan_workspace, fan_surfaces, _) = seeded_primitive_workspace(
        "query.phase-16.access-plan-receipts.fan",
        &MilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
    );
    let radial_identity = current_lookup_rows(&mut fan_workspace, &fan_surfaces)
        .lookup()
        .first_source_identity_for_relation_kind(TopologyRelationKind::HalfEdgeRadialNext)
        .expect("edge fan should expose radial source");
    let radial_anchor = TopologyReadAnchorIdentity::from_runtime_row_label(&radial_identity);
    let handle = current_head_query_handle();
    let mut fan_reads = handle.topology_reads(&mut fan_workspace);

    let shared = fan_reads
        .shared_vertex_half_edge_neighborhood(&radial_anchor)
        .expect("shared endpoint topology read should execute through planned access");
    let radial = fan_reads
        .radial_half_edge_neighborhood(&radial_anchor)
        .expect("radial topology read should execute through planned access");

    let (mut disk_workspace, disk_surfaces, _) = seeded_primitive_workspace(
        "query.phase-16.access-plan-receipts.disk",
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
    );
    let start_identity = current_lookup_rows(&mut disk_workspace, &disk_surfaces)
        .lookup()
        .first_source_identity_for_relation_kind(TopologyRelationKind::HalfEdgeNext)
        .expect("sheet disk should expose successor source");
    let disk_anchor = TopologyReadAnchorIdentity::from_runtime_row_label(&start_identity);
    let mut disk_reads = handle.topology_reads(&mut disk_workspace);
    let loop_cycle = disk_reads
        .loop_cycle(&disk_anchor, 4)
        .expect("loop cycle topology read should execute through planned access");
    let local_rewire = disk_reads
        .local_rewire_neighborhood(&disk_anchor, 4)
        .expect("local rewire topology read should execute through planned access");

    assert_access_planned_and_no_caller_owned_graph_work(shared.request_report());
    assert_access_planned_and_no_caller_owned_graph_work(radial.request_report());
    assert_access_planned_and_no_caller_owned_graph_work(loop_cycle.request_report());
    assert_access_planned_and_no_caller_owned_graph_work(local_rewire.request_report());

    let reports = [
        shared.request_report().clone(),
        radial.request_report().clone(),
        loop_cycle.request_report().clone(),
        local_rewire.request_report().clone(),
    ];
    let certification = PhaseSixteenGraphAccessAdoptionCertification::certify(&reports, 2)
        .expect("all supported topology read families should carry graph access proof");
    assert_eq!(
        certification.covered_families(),
        TopologyReadRequestFamily::ALL
    );
    assert_eq!(certification.source_inventory_count(), 2);
}

#[test]
fn phase_sixteen_broad_topology_read_uses_typed_non_inline_posture() {
    let (mut workspace, surfaces, _) = seeded_sheet_disk_workspace("query.phase-16.broad-posture");
    let lookup_rows = current_lookup_rows(&mut workspace, &surfaces);
    let start_identity = lookup_rows
        .lookup()
        .first_source_identity_for_relation_kind(TopologyRelationKind::HalfEdgeNext)
        .expect("sheet disk should expose successor source");
    let anchor = TopologyReadAnchorIdentity::from_runtime_row_label(&start_identity);
    let handle = current_head_query_handle();
    let mut reads = handle.topology_reads(&mut workspace);
    let loop_cycle = reads
        .loop_cycle(&anchor, 6)
        .expect("broad topology read should execute through a typed graph access posture");
    let proof = loop_cycle
        .request_report()
        .graph_access_proof()
        .expect("loop cycle should carry graph access proof");

    assert!(
        matches!(
            proof.admission_posture(),
            ForgeQueryGraphReadAccessAdmissionPosture::AdmittedPagedStreaming
                | ForgeQueryGraphReadAccessAdmissionPosture::PersistentIndexRequired
                | ForgeQueryGraphReadAccessAdmissionPosture::AsyncMaterializationRequired
        ),
        "broad topology read should not hide as an ordinary inline RAM expansion"
    );
    assert!(proof.no_caller_owned_graph_work());
}

#[test]
fn phase_sixteen_runtime_boundary_rejects_depth_proof_drift_before_execution() {
    let (mut workspace, surfaces, _) = seeded_sheet_disk_workspace("query.phase-16.depth-drift");
    let lookup_rows = current_lookup_rows(&mut workspace, &surfaces);
    let start_identity = lookup_rows
        .lookup()
        .first_source_identity_for_relation_kind(TopologyRelationKind::HalfEdgeNext)
        .expect("sheet disk should expose successor source");
    let target = TopologyReadExecutionTarget::current_head();
    let request = TopologyReadRequest::LoopCycleNeighborhood {
        start_half_edge_identity: TopologyReadAnchorIdentity::from_runtime_row_label(
            &start_identity,
        ),
        depth: 2,
    };

    let error = match execute_loop_cycle_read(&mut workspace, &target, &request, &start_identity, 4)
    {
        Ok(_) => panic!("runtime boundary must reject caller depth drift"),
        Err(error) => error,
    };

    assert_eq!(
        error.kind(),
        TopologyReadErrorKind::ReadFamilyExecutionDenied
    );
    assert!(
        error
            .to_string()
            .contains("lowered request proof carries depth `2`"),
        "depth drift denial should name the proof-bearing request depth: {error}"
    );
}

#[test]
fn phase_sixteen_broad_local_rewire_denies_before_inline_expansion() {
    let (mut workspace, surfaces, _) =
        seeded_sheet_disk_workspace("query.phase-16.local-rewire-broad-denial");
    let lookup_rows = current_lookup_rows(&mut workspace, &surfaces);
    let start_identity = lookup_rows
        .lookup()
        .first_source_identity_for_relation_kind(TopologyRelationKind::HalfEdgeNext)
        .expect("sheet disk should expose successor source");
    let anchor = TopologyReadAnchorIdentity::from_runtime_row_label(&start_identity);
    let handle = current_head_query_handle();
    let mut reads = handle.topology_reads(&mut workspace);

    let error = match reads.local_rewire_neighborhood(&anchor, 6) {
        Ok(_) => panic!("broad local rewire must deny instead of expanding inline"),
        Err(error) => error,
    };

    assert_eq!(
        error.kind(),
        TopologyReadErrorKind::ReadFamilyExecutionDenied
    );
    let denial = error
        .graph_access_denial()
        .expect("broad local rewire denial should preserve Query access denial");
    assert_eq!(
        denial.denial_kind(),
        &ForgeQueryGraphReadAccessDenialKind::BudgetExceeded
    );
    assert_eq!(denial.executor_entry_count(), Some(0));
    assert_eq!(denial.materialized_row_count(), Some(0));
    assert!(denial.budget_exceeded().is_some());
}

#[test]
fn phase_sixteen_dense_topology_read_proves_no_caller_owned_graph_access() {
    let (mut workspace, surfaces, _) = seeded_primitive_workspace(
        "query.phase-16.dense-no-caller-owned-work",
        &MilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 9 },
    );
    let lookup_rows = current_lookup_rows(&mut workspace, &surfaces);
    let start_identity = lookup_rows
        .lookup()
        .first_source_identity_for_relation_kind(TopologyRelationKind::HalfEdgeRadialNext)
        .expect("edge fan should expose radial source");
    let anchor = TopologyReadAnchorIdentity::from_runtime_row_label(&start_identity);
    let handle = current_head_query_handle();
    let mut reads = handle.topology_reads(&mut workspace);

    let radial = reads
        .radial_half_edge_neighborhood(&anchor)
        .expect("dense radial read should execute through planned access");
    let proof = radial
        .request_report()
        .graph_access_proof()
        .expect("dense radial read should carry graph access proof");

    assert!(proof.planned_access_step_count() > 0);
    assert_eq!(
        proof.planned_access_step_count(),
        proof.consumed_access_step_count()
    );
    assert!(proof.no_caller_owned_graph_work());
    assert_eq!(radial.request_report().row_scan_fallback_count(), 0);
    assert_eq!(radial.request_report().whole_view_fallback_count(), 0);
}

#[test]
fn phase_sixteen_parity_artifact_is_fused_to_graph_access_proof() {
    let (mut workspace, surfaces, read_basis) =
        seeded_sheet_disk_workspace("query.phase-16.parity-access-proof");
    let lookup_rows = current_lookup_rows(&mut workspace, &surfaces);
    let start_identity = lookup_rows
        .lookup()
        .first_source_identity_for_relation_kind(TopologyRelationKind::HalfEdgeNext)
        .expect("sheet disk should expose successor source");
    let anchor = TopologyReadAnchorIdentity::from_runtime_row_label(&start_identity);
    let handle = current_head_query_handle();
    let mut reads = handle.topology_reads(&mut workspace);

    let local_rewire = reads
        .local_rewire_neighborhood(&anchor, 4)
        .expect("local rewire should execute through planned access");
    let parity_artifact = build_topology_read_view_parity_artifact(
        &read_basis,
        TopologyReadViewRef::LocalRewire(&local_rewire),
    );
    let proof = local_rewire
        .request_report()
        .graph_access_proof()
        .expect("parity-producing read should carry graph access proof");

    assert_eq!(
        parity_artifact.request_family(),
        TopologyReadRequestFamily::LocalRewireNeighborhood
    );
    assert!(proof.no_caller_owned_graph_work());
    assert!(
        !proof.requirement_set_digest().is_empty(),
        "parity proof must stay attached to inspectable Query access requirements"
    );
}

#[test]
fn phase_sixteen_covered_read_views_have_no_unclassified_graph_read_bypass_residue() {
    let inventory = query_boundary_source_inventory("worth-topo")
        .required_root(read_view_domain_views_dir())
        .include_rs_files()
        .seal()
        .expect("read view domain inventory should build");
    let report = graph_read_bypass_audit("worth-topo-phase-16-read-views")
        .required_inventory(&inventory)
        .evaluate()
        .expect("graph-read bypass audit should evaluate read view domain views");
    assert!(
        report.findings().is_empty(),
        "covered read-view helpers must not retain unclassified graph-read bypass folklore: {:?}",
        report.findings()
    );

    let adoption = graph_read_bypass_adoption("worth-topo-phase-16-read-views")
        .audit_report(report)
        .residue_manifest(ForgeQueryGraphReadBypassResidueManifest::empty())
        .certify()
        .expect("zero-residue read-view adoption should certify");
    assert!(adoption.has_no_unclassified_findings());
}

#[test]
fn phase_sixteen_runtime_boundary_has_no_unclassified_graph_read_bypass_residue() {
    let inventory = query_boundary_source_inventory("worth-topo")
        .required_root(runtime_boundary_read_execution_dir())
        .include_rs_files()
        .seal()
        .expect("runtime boundary read execution inventory should build");
    let report = graph_read_bypass_audit("worth-topo-phase-16-runtime-boundary")
        .required_inventory(&inventory)
        .evaluate()
        .expect("graph-read bypass audit should evaluate runtime boundary read execution");
    assert!(
        report.findings().is_empty(),
        "covered runtime-boundary read execution must not retain unclassified graph-read bypass folklore: {:?}",
        report.findings()
    );

    let adoption = graph_read_bypass_adoption("worth-topo-phase-16-runtime-boundary")
        .audit_report(report)
        .residue_manifest(ForgeQueryGraphReadBypassResidueManifest::empty())
        .certify()
        .expect("zero-residue runtime-boundary adoption should certify");
    assert!(adoption.has_no_unclassified_findings());
}

fn assert_access_planned_and_no_caller_owned_graph_work(report: &TopologyReadRequestReport) {
    let proof = report
        .graph_access_proof()
        .expect("topology read should carry graph access proof");
    assert!(!proof.plan_digest().is_empty());
    assert!(!proof.admission_digest().is_empty());
    assert!(!proof.requirement_set_digest().is_empty());
    assert!(!proof.cost_estimate_digest().is_empty());
    assert!(!proof.budget_digest().is_empty());
    assert!(!proof.graph_index_inventory_match_report_digest().is_empty());
    assert_eq!(proof.executor_entry_count(), 1);
    assert!(proof.planned_access_step_count() > 0);
    assert_eq!(
        proof.consumed_access_step_count(),
        proof.planned_access_step_count()
    );
    assert!(proof.no_caller_owned_graph_work());
}

fn runtime_boundary_read_execution_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/projection/runtime_boundary/read_execution")
}

fn read_view_domain_views_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/projection/read_views/domain/views")
}
