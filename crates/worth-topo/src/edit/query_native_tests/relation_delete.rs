use forge_query::facade::ForgeQueryMutationFamily;
use worth_schema::facade::{seed_minimal_topology, WorthTopologyRelationKind};

use crate::edit::{
    WorthBoundaryMembershipKind, WorthShellOrWireMembershipKind, WorthTopologyEditApplicationMode,
    WorthTopologyEditBatch, WorthTopologyEditContract, WorthTopologyEditFamily,
    WorthTopologyQueryEditExecutionError, WorthTopologyQueryEditRunner,
};
use crate::query::{
    worth_topology_runtime, WorthTopologyQueryAssembly, WorthTopologyRuntimeAdapters,
};
use crate::runtime_invariants::build_worth_milestone_one_runtime;

use super::super::query_native_test_support::seeded_relation_id;

#[test]
fn query_native_edit_runner_applies_detach_boundary_membership_on_production_runtime() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded = seed_minimal_topology(&mut runtime, "worth.query-native-edit.detach-boundary")
        .expect("seed");
    let loop_owns_half_edge_relation = seeded_relation_id(
        &runtime,
        &seeded.snapshot,
        WorthTopologyRelationKind::LoopOwnsHalfEdge,
    );
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(adapters, "worth.query-native-edit.detach-boundary")
        .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let batch =
        WorthTopologyEditBatch::new(vec![WorthTopologyEditContract::detach_boundary_membership(
            loop_owns_half_edge_relation,
            WorthBoundaryMembershipKind::LoopOwnsHalfEdge,
        )])
        .expect("non-empty batch");

    let execution = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect("query-native detach should succeed");

    assert_eq!(
        execution.families,
        vec![WorthTopologyEditFamily::DetachBoundaryMembership]
    );
    assert_eq!(execution.naming_report.rows.len(), 1);
    assert_eq!(
        execution.receipt.write_receipts()[0].mutation_family(),
        ForgeQueryMutationFamily::Delete
    );
    assert_eq!(
        execution.inspection.component_operations()[0].family(),
        "delete"
    );
    let binding = execution.inspection.component_operations()[0]
        .existing_truth_binding_evidence()
        .expect("detach execution should preserve existing-truth binding evidence");
    assert_eq!(binding.family().as_str(), "direct-relation-identity");
    assert_eq!(binding.target_collection(), Some("WorthTopologyRelation"));
    let loop_record = execution
        .materialized
        .topology()
        .loops
        .iter()
        .find(|loop_record| loop_record.entity_id == seeded.outer_loop)
        .expect("seeded outer loop should remain present");
    assert!(loop_record.half_edge_ids.is_empty());
}

#[test]
fn query_native_edit_runner_applies_detach_face_outer_loop_on_production_runtime() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded = seed_minimal_topology(&mut runtime, "worth.query-native-edit.detach-face-loop")
        .expect("seed");
    let face_outer_loop_relation = seeded_relation_id(
        &runtime,
        &seeded.snapshot,
        WorthTopologyRelationKind::FaceOuterLoop,
    );
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.query-native-edit.detach-face-loop")
            .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let batch =
        WorthTopologyEditBatch::new(vec![WorthTopologyEditContract::detach_boundary_membership(
            face_outer_loop_relation,
            WorthBoundaryMembershipKind::FaceOuterLoop,
        )])
        .expect("non-empty batch");

    let execution = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect("query-native face-outer-loop detach should succeed");

    assert_eq!(
        execution.families,
        vec![WorthTopologyEditFamily::DetachBoundaryMembership]
    );
    let face = execution
        .materialized
        .topology()
        .faces
        .iter()
        .find(|face| face.entity_id == seeded.face)
        .expect("seeded face should remain present");
    let loop_record = execution
        .materialized
        .topology()
        .loops
        .iter()
        .find(|loop_record| loop_record.entity_id == seeded.outer_loop)
        .expect("seeded outer loop should remain present");
    assert_eq!(face.outer_loop_id, None);
    assert!(face.boundary_half_edge_ids.is_empty());
    assert!(loop_record.face_ids.is_empty());
}

#[test]
fn query_native_edit_runner_applies_detach_radial_adjacency_on_production_runtime() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded =
        seed_minimal_topology(&mut runtime, "worth.query-native-edit.detach-radial").expect("seed");
    let radial_relation = seeded_relation_id(
        &runtime,
        &seeded.snapshot,
        WorthTopologyRelationKind::HalfEdgeRadialNext,
    );
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(adapters, "worth.query-native-edit.detach-radial")
        .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let batch =
        WorthTopologyEditBatch::new(vec![WorthTopologyEditContract::detach_radial_adjacency(
            radial_relation,
        )])
        .expect("non-empty batch");

    let execution = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect("query-native radial detach should succeed");

    assert_eq!(
        execution.families,
        vec![WorthTopologyEditFamily::DetachRadialAdjacency]
    );
    assert_eq!(execution.naming_report.rows.len(), 1);
    assert_eq!(
        execution.receipt.write_receipts()[0].mutation_family(),
        ForgeQueryMutationFamily::Delete
    );
    assert_eq!(
        execution.inspection.component_operations()[0].family(),
        "delete"
    );
    let binding = execution.inspection.component_operations()[0]
        .existing_truth_binding_evidence()
        .expect("radial detach should preserve existing-truth binding evidence");
    assert_eq!(binding.family().as_str(), "direct-relation-identity");
    assert_eq!(binding.target_collection(), Some("WorthTopologyRelation"));
    let half_edge = execution
        .materialized
        .topology()
        .half_edges
        .iter()
        .find(|half_edge| half_edge.entity_id == seeded.half_edge)
        .expect("seeded half-edge should remain present");
    assert_eq!(half_edge.radial_next_half_edge_id, None);
}

#[test]
fn query_native_edit_runner_applies_detach_shell_or_wire_membership_on_production_runtime() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded =
        seed_minimal_topology(&mut runtime, "worth.query-native-edit.detach-wire").expect("seed");
    let wire_owns_half_edge_relation = seeded_relation_id(
        &runtime,
        &seeded.snapshot,
        WorthTopologyRelationKind::WireOwnsHalfEdge,
    );
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.query-native-edit.detach-wire").expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let batch = WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::detach_shell_or_wire_membership(
            wire_owns_half_edge_relation,
            WorthShellOrWireMembershipKind::WireOwnsHalfEdge,
        ),
    ])
    .expect("non-empty batch");

    let execution = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect("query-native shell-or-wire detach should succeed");

    assert_eq!(
        execution.families,
        vec![WorthTopologyEditFamily::DetachShellOrWireMembership]
    );
    assert_eq!(execution.naming_report.rows.len(), 1);
    assert_eq!(
        execution.receipt.write_receipts()[0].mutation_family(),
        ForgeQueryMutationFamily::Delete
    );
    assert_eq!(
        execution.inspection.component_operations()[0].family(),
        "delete"
    );
    let binding = execution.inspection.component_operations()[0]
        .existing_truth_binding_evidence()
        .expect("shell-or-wire detach should preserve existing-truth binding evidence");
    assert_eq!(binding.family().as_str(), "direct-relation-identity");
    assert_eq!(binding.target_collection(), Some("WorthTopologyRelation"));
    let wire = execution
        .materialized
        .topology()
        .wires
        .iter()
        .find(|wire| wire.entity_id == seeded.wire)
        .expect("seeded wire should remain present");
    assert!(wire.half_edge_ids.is_empty());
}

#[test]
fn query_native_edit_runner_applies_detach_shell_owns_face_on_production_runtime() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded = seed_minimal_topology(&mut runtime, "worth.query-native-edit.detach-shell-face")
        .expect("seed");
    let shell_owns_face_relation = seeded_relation_id(
        &runtime,
        &seeded.snapshot,
        WorthTopologyRelationKind::ShellOwnsFace,
    );
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.query-native-edit.detach-shell-face")
            .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let batch = WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::detach_shell_or_wire_membership(
            shell_owns_face_relation,
            WorthShellOrWireMembershipKind::ShellOwnsFace,
        ),
    ])
    .expect("non-empty batch");

    let execution = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect("query-native shell-face detach should succeed");

    assert_eq!(
        execution.families,
        vec![WorthTopologyEditFamily::DetachShellOrWireMembership]
    );
    let shell = execution
        .materialized
        .topology()
        .shells
        .iter()
        .find(|shell| shell.entity_id == seeded.shell)
        .expect("seeded shell should remain present");
    let face = execution
        .materialized
        .topology()
        .faces
        .iter()
        .find(|face| face.entity_id == seeded.face)
        .expect("seeded face should remain present");
    assert!(shell.face_ids.is_empty());
    assert_eq!(face.shell_id, None);
}

#[test]
fn query_native_edit_runner_denies_boundary_detach_kind_mismatch_on_production_runtime() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded =
        seed_minimal_topology(&mut runtime, "worth.query-native-edit.detach-kind").expect("seed");
    let face_outer_loop_relation = seeded_relation_id(
        &runtime,
        &seeded.snapshot,
        WorthTopologyRelationKind::FaceOuterLoop,
    );
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.query-native-edit.detach-kind").expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let batch =
        WorthTopologyEditBatch::new(vec![WorthTopologyEditContract::detach_boundary_membership(
            face_outer_loop_relation,
            WorthBoundaryMembershipKind::LoopOwnsHalfEdge,
        )])
        .expect("non-empty batch");

    let error = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect_err("boundary detach kind mismatch must fail closed");

    assert!(matches!(
        error,
        WorthTopologyQueryEditExecutionError::ExistingRelationKindMismatch {
            relation_id,
            expected: WorthTopologyRelationKind::LoopOwnsHalfEdge,
            actual: WorthTopologyRelationKind::FaceOuterLoop,
        } if relation_id == face_outer_loop_relation
    ));
}

#[test]
fn query_native_edit_runner_denies_shell_or_wire_detach_kind_mismatch_on_production_runtime() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded = seed_minimal_topology(&mut runtime, "worth.query-native-edit.detach-wire-kind")
        .expect("seed");
    let shell_owns_face_relation = seeded_relation_id(
        &runtime,
        &seeded.snapshot,
        WorthTopologyRelationKind::ShellOwnsFace,
    );
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.query-native-edit.detach-wire-kind")
            .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let batch = WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::detach_shell_or_wire_membership(
            shell_owns_face_relation,
            WorthShellOrWireMembershipKind::WireOwnsHalfEdge,
        ),
    ])
    .expect("non-empty batch");

    let error = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect_err("shell-or-wire detach kind mismatch must fail closed");

    assert!(matches!(
        error,
        WorthTopologyQueryEditExecutionError::ExistingRelationKindMismatch {
            relation_id,
            expected: WorthTopologyRelationKind::WireOwnsHalfEdge,
            actual: WorthTopologyRelationKind::ShellOwnsFace,
        } if relation_id == shell_owns_face_relation
    ));
}

#[test]
fn query_native_edit_runner_denies_radial_detach_kind_mismatch_on_production_runtime() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded = seed_minimal_topology(&mut runtime, "worth.query-native-edit.detach-radial-kind")
        .expect("seed");
    let face_outer_loop_relation = seeded_relation_id(
        &runtime,
        &seeded.snapshot,
        WorthTopologyRelationKind::FaceOuterLoop,
    );
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.query-native-edit.detach-radial-kind")
            .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let batch =
        WorthTopologyEditBatch::new(vec![WorthTopologyEditContract::detach_radial_adjacency(
            face_outer_loop_relation,
        )])
        .expect("non-empty batch");

    let error = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect_err("radial detach kind mismatch must fail closed");

    assert!(matches!(
        error,
        WorthTopologyQueryEditExecutionError::ExistingRelationKindMismatch {
            relation_id,
            expected: WorthTopologyRelationKind::HalfEdgeRadialNext,
            actual: WorthTopologyRelationKind::FaceOuterLoop,
        } if relation_id == face_outer_loop_relation
    ));
}
