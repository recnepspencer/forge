use forge_query::facade::ForgeQueryExistingTruthAssertionMode;
use worth_schema::facade::topology_authoring::{
    seed_milestone_one_primitive, WorthMilestoneOnePrimitiveCase,
};
use worth_schema::facade::WorthTopologyRelationKind;

use super::relation_update_support::{query_relation_id_from_row, RelationUpdateQuerySupport};
use crate::edit::{
    WorthTopologyEditApplicationMode, WorthTopologyEditBatch, WorthTopologyEditContract,
    WorthTopologyEditFamily, WorthTopologyEditRejectionClass, WorthTopologyQueryEditExecutionError,
};
use crate::query::{
    worth_topology_runtime, WorthTopologyQueryAssembly, WorthTopologyRuntimeAdapters,
};
use crate::runtime_invariants::build_worth_milestone_one_runtime;

#[test]
fn current_head_runtime_executes_splice_radial_adjacency_through_query_native_edit_runner() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        "worth.current-head.query-edit-splice-radial",
        &WorthMilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
    )
    .expect("seed primitive");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.current-head.query-edit-splice-radial")
            .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let support = RelationUpdateQuerySupport::load(&workspace, &assembly);
    let relation_rows = workspace.read(assembly.relations());
    let relation = relation_rows
        .iter()
        .find(|row| {
            row.payload
                .get("topology")
                .and_then(|value| value.get("kind"))
                .and_then(|value| value.as_str())
                .is_some_and(|kind_name| {
                    kind_name == WorthTopologyRelationKind::HalfEdgeRadialNext.kind_name()
                })
        })
        .expect("seeded topology should contain a radial relation");
    let source_identity = relation
        .payload
        .get("topology")
        .and_then(|value| value.get("source_identity"))
        .and_then(|value| value.as_str())
        .expect("radial relation should expose topology.source_identity");
    let current_target_identity = relation
        .payload
        .get("topology")
        .and_then(|value| value.get("target_identity"))
        .and_then(|value| value.as_str())
        .expect("radial relation should expose topology.target_identity");
    let half_edge_id = support.find_entity_id_by_identity(source_identity);
    let radial_next_half_edge_id = support.alternate_same_edge_half_edge_id(
        &mut workspace,
        source_identity,
        current_target_identity,
    );
    let batch =
        WorthTopologyEditBatch::new(vec![WorthTopologyEditContract::splice_radial_adjacency(
            query_relation_id_from_row(relation),
            half_edge_id,
            radial_next_half_edge_id,
        )])
        .expect("non-empty edit batch");

    let execution = assembly
        .apply_edit(
            &mut workspace,
            batch,
            WorthTopologyEditApplicationMode::Mainline,
        )
        .expect("radial splice should execute through the admitted runtime family");

    assert_eq!(
        execution.families,
        vec![WorthTopologyEditFamily::SpliceRadialAdjacency]
    );
    assert_eq!(
        execution.inspection.component_operations()[0].family(),
        "update"
    );
    assert_eq!(
        execution.inspection.component_operations()[0].target_collection(),
        Some("WorthTopologyRelation")
    );
    assert_eq!(
        execution
            .receipt
            .batch_mutation_evidence()
            .backend_verified_update_count(),
        1
    );
    assert_eq!(
        execution.inspection.component_operations()[0]
            .existing_truth_assertion_evidence()
            .expect("radial splice receipt should retain backend verification evidence")
            .mode(),
        ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
    );
    let half_edge = execution
        .materialized
        .topology()
        .half_edges
        .iter()
        .find(|half_edge| half_edge.entity_id == half_edge_id)
        .expect("rewired halfedge should remain present");
    assert_eq!(
        half_edge.radial_next_half_edge_id,
        Some(radial_next_half_edge_id)
    );
}

#[test]
fn current_head_runtime_denies_splice_radial_adjacency_with_mismatched_source_binding() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        "worth.current-head.query-edit-splice-radial-source-mismatch",
        &WorthMilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
    )
    .expect("seed primitive");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(
        adapters,
        "worth.current-head.query-edit-splice-radial-source-mismatch",
    )
    .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let support = RelationUpdateQuerySupport::load(&workspace, &assembly);
    let relation_rows = workspace.read(assembly.relations());
    let relation = relation_rows
        .iter()
        .find(|row| {
            row.payload
                .get("topology")
                .and_then(|value| value.get("kind"))
                .and_then(|value| value.as_str())
                .is_some_and(|kind_name| {
                    kind_name == WorthTopologyRelationKind::HalfEdgeRadialNext.kind_name()
                })
        })
        .expect("seeded topology should contain a radial relation");
    let source_identity = relation
        .payload
        .get("topology")
        .and_then(|value| value.get("source_identity"))
        .and_then(|value| value.as_str())
        .expect("radial relation should expose topology.source_identity");
    let current_target_identity = relation
        .payload
        .get("topology")
        .and_then(|value| value.get("target_identity"))
        .and_then(|value| value.as_str())
        .expect("radial relation should expose topology.target_identity");
    let wrong_half_edge_id = support.find_entity_id_by_identity(current_target_identity);
    let radial_next_half_edge_id = support.alternate_same_edge_half_edge_id(
        &mut workspace,
        source_identity,
        current_target_identity,
    );
    let batch =
        WorthTopologyEditBatch::new(vec![WorthTopologyEditContract::splice_radial_adjacency(
            query_relation_id_from_row(relation),
            wrong_half_edge_id,
            radial_next_half_edge_id,
        )])
        .expect("non-empty edit batch");

    let error = assembly
        .apply_edit(
            &mut workspace,
            batch.clone(),
            WorthTopologyEditApplicationMode::Mainline,
        )
        .expect_err("radial splice with mismatched source binding must fail typed and early");

    assert!(matches!(
        error,
        WorthTopologyQueryEditExecutionError::ExistingRelationSourceMismatch {
            relation_id,
            expected_source_entity_id,
            ..
        } if relation_id == query_relation_id_from_row(relation)
            && expected_source_entity_id == wrong_half_edge_id
    ));
    assert_eq!(
        error.rejection_class(),
        Some(WorthTopologyEditRejectionClass::InvariantBlocked)
    );
    let report = error
        .rejected_edit_scope_report(&batch)
        .expect("invariant-block denial should expose exact rejected scope report");
    assert_eq!(report.rows.len(), 1);
    assert_eq!(
        report.rows[0].rejection_class,
        WorthTopologyEditRejectionClass::InvariantBlocked
    );
    assert_eq!(
        report.rows[0].family,
        WorthTopologyEditFamily::SpliceRadialAdjacency
    );
    assert!(report.rows[0]
        .changed_scopes
        .contains(&crate::edit::WorthTopologyEditChangedScope::RadialNeighborhood));
    assert!(report.rows[0]
        .derived_regions
        .contains(&crate::edit::WorthTopologyDerivedRegion::RadialNeighborhoodRegion));
}

#[test]
fn current_head_runtime_denies_splice_radial_adjacency_across_different_edges() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        "worth.current-head.query-edit-splice-radial-mismatch",
        &WorthMilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
    )
    .expect("seed primitive");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(
        adapters,
        "worth.current-head.query-edit-splice-radial-mismatch",
    )
    .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let support = RelationUpdateQuerySupport::load(&workspace, &assembly);
    let relation_rows = workspace.read(assembly.relations());
    let relation = relation_rows
        .iter()
        .find(|row| {
            row.payload
                .get("topology")
                .and_then(|value| value.get("kind"))
                .and_then(|value| value.as_str())
                .is_some_and(|kind_name| {
                    kind_name == WorthTopologyRelationKind::HalfEdgeRadialNext.kind_name()
                })
        })
        .expect("seeded topology should contain a radial relation");
    let source_identity = relation
        .payload
        .get("topology")
        .and_then(|value| value.get("source_identity"))
        .and_then(|value| value.as_str())
        .expect("radial relation should expose topology.source_identity");
    let half_edge_id = support.find_entity_id_by_identity(source_identity);
    let expected_target_half_edge_id =
        support.different_edge_half_edge_id(&mut workspace, source_identity);
    let batch =
        WorthTopologyEditBatch::new(vec![WorthTopologyEditContract::splice_radial_adjacency(
            query_relation_id_from_row(relation),
            half_edge_id,
            expected_target_half_edge_id,
        )])
        .expect("non-empty edit batch");

    let error = assembly
        .apply_edit(
            &mut workspace,
            batch,
            WorthTopologyEditApplicationMode::Mainline,
        )
        .expect_err("radial splice across different edges must fail typed and early");

    assert!(matches!(
        error,
        WorthTopologyQueryEditExecutionError::ExistingHalfEdgesNotOnSameEdge {
            source_half_edge_id,
            target_half_edge_id,
            ..
        } if source_half_edge_id == half_edge_id
            && target_half_edge_id == expected_target_half_edge_id
    ));
}
