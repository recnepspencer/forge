use crate::workload_composition::{
    worth_workload_ordinary_consumer_residue_rows, LookupConsumedWorkloadDenial,
    ReplayUndoBoundaryDenial, WorthWorkloadOrdinaryConsumerResidueBoundary,
    WorthWorkloadOrdinaryConsumerResidueSurface,
};

use super::tests_support_completed_split::{
    attached_completed_split_handoff, ordinary_completed_split_handoff,
};
use super::tests_support_replay_undo_scope::with_replay_undo_scope_products;

#[path = "../../../certification/public_facade_contracts/contracts/public_api_planar_boolean_loop_reconstruction_workload_evidence_support/ordinary_topology_undo_support.rs"]
mod ordinary_topology_undo_support;
#[path = "../../../certification/public_facade_contracts/contracts/public_api_planar_boolean_loop_reconstruction_workload_evidence_support.rs"]
mod replay_support;
#[path = "../../operator_harness/tests_vertical_migration/support/spatial_batch_execution_slice.rs"]
mod spatial_batch_execution_slice;
#[path = "../../operator_harness/tests_vertical_migration/support/spatial_batch_execution_aspect_slice.rs"]
mod spatial_batch_execution_aspect_slice;

#[test]
fn lookup_consumed_cluster_requires_workload_attached_batch_execution_receipt() {
    let completed_split_handoff =
        ordinary_completed_split_handoff("phase11 ordinary consumer sweep lookup cluster denial");

    let error = completed_split_handoff
        .completed_workload()
        .admit_lookup_consumed_batch_execution_cluster(
            completed_split_handoff.lookup_consumed_workload_handoff(),
            spatial_batch_execution_slice::disjoint_parallel_spatial_batch_execution_slice(
            )
            .execution_receipt(),
        )
        .expect_err(
            "lookup-consumed grouped cluster must require workload-attached batch execution",
        );

    assert_eq!(
        error.lookup_consumed_workload_denial(),
        Some(&LookupConsumedWorkloadDenial::MissingWorkloadAttachedBatchAdmissionExecutionReceipt)
    );
}

#[test]
fn lookup_consumed_cluster_rejects_mismatched_batch_execution_receipt() {
    let attached_split_handoff = attached_completed_split_handoff(
        "phase11 ordinary consumer sweep lookup cluster mismatch",
        spatial_batch_execution_slice::disjoint_parallel_spatial_batch_execution_slice()
            .execution_receipt(),
    );

    let error = attached_split_handoff
        .completed_workload()
        .admit_lookup_consumed_batch_execution_cluster(
            attached_split_handoff.lookup_consumed_workload_handoff(),
            spatial_batch_execution_aspect_slice::compatible_aspect_parallel_spatial_batch_execution_slice()
                .execution_receipt(),
        )
        .expect_err("lookup-consumed grouped cluster must reject a mismatched batch execution");

    assert_eq!(
        error.lookup_consumed_workload_denial(),
        Some(&LookupConsumedWorkloadDenial::SuppliedBatchAdmissionExecutionReceiptMismatch)
    );
}

#[test]
fn lookup_consumed_cluster_binds_explicit_batch_execution_authority_chain() {
    let batch_execution =
        spatial_batch_execution_slice::disjoint_parallel_spatial_batch_execution_slice()
            .execution_receipt()
            .clone();
    let attached_split_handoff = attached_completed_split_handoff(
        "phase11 ordinary consumer sweep lookup cluster positive",
        &batch_execution,
    );

    let cluster = attached_split_handoff
        .completed_workload()
        .admit_lookup_consumed_batch_execution_cluster(
            attached_split_handoff.lookup_consumed_workload_handoff(),
            &batch_execution,
        )
        .expect("lookup-consumed grouped cluster should admit from explicit batch authority");

    assert_eq!(
        cluster
            .workload()
            .batch_admission_execution()
            .expect("attached workload batch execution")
            .execution_receipt_digest(),
        batch_execution.execution_receipt_digest()
    );
    assert_eq!(
        cluster
            .workload()
            .batch_admission_execution()
            .expect("attached workload batch execution")
            .selected_batch_plan_digest(),
        batch_execution.selected_batch_plan_digest()
    );
    assert_eq!(
        cluster.batch_execution().execution_receipt_digest(),
        batch_execution.execution_receipt_digest()
    );
    assert_eq!(
        cluster.batch_execution().selected_batch_plan_digest(),
        batch_execution.selected_batch_plan_digest()
    );
    assert_eq!(
        cluster
            .lookup_consumed()
            .handoff()
            .workload_stage_index_identity(),
        cluster
            .workload()
            .evidence_ledger()
            .stage_index()
            .index_identity()
    );
}

#[test]
fn downstream_split_consumption_requires_workload_attached_batch_execution_receipt() {
    let subject = replay_support::MetabossEventExtractionSubject::certify(
        "phase11 ordinary consumer sweep downstream split denial",
    );
    let replay_subject = replay_support::build_edge_split_replay_parity_subject(&subject);
    let completed_split_handoff =
        replay_support::completed_split_handoff_for(&subject, &replay_subject);

    let error = completed_split_handoff
        .admit_batch_execution_cluster()
        .and_then(|cluster| {
            cluster.admit_downstream_split_consumption(
                replay_subject.original_decision_log.receipt(),
                &replay_subject.original_products.validation,
                &replay_subject.original_products.naming,
                replay_support::replay_parity_report(&replay_subject).receipt(),
            )
        })
        .expect_err("downstream split consumer must require explicit batch execution authority");

    assert_eq!(
        error.lookup_consumed_workload_denial(),
        Some(&LookupConsumedWorkloadDenial::MissingWorkloadAttachedBatchAdmissionExecutionReceipt)
    );
}

#[test]
fn downstream_split_consumption_binds_explicit_batch_execution_authority_chain() {
    let subject = replay_support::MetabossEventExtractionSubject::certify(
        "phase11 ordinary consumer sweep downstream split positive",
    );
    let replay_subject = replay_support::build_edge_split_replay_parity_subject(&subject);
    let batch_execution =
        spatial_batch_execution_slice::disjoint_parallel_spatial_batch_execution_slice()
            .execution_receipt()
            .clone();
    let attached_split_handoff =
        replay_support::completed_split_handoff_for(&subject, &replay_subject)
            .with_batch_admission_execution(&batch_execution)
            .expect("completed split handoff should attach the selected batch execution");

    let cluster = attached_split_handoff
        .admit_batch_execution_cluster()
        .expect("attached split handoff should admit the batch execution cluster");
    let downstream = cluster
        .admit_downstream_split_consumption(
            replay_subject.original_decision_log.receipt(),
            &replay_subject.original_products.validation,
            &replay_subject.original_products.naming,
            replay_support::replay_parity_report(&replay_subject).receipt(),
        )
        .expect("downstream split consumption should close from the explicit cluster");

    assert_eq!(
        cluster
            .lookup_consumed_cluster()
            .batch_execution()
            .selected_batch_plan_digest(),
        batch_execution.selected_batch_plan_digest()
    );
    assert_eq!(
        downstream.workload_stage_index_identity(),
        attached_split_handoff.workload_stage_index_identity()
    );
    assert_eq!(
        downstream.lookup_execution_receipt_digest(),
        attached_split_handoff
            .lookup_consumed_workload_handoff()
            .lookup_execution_receipt_digest()
    );
}

#[test]
fn boolean_split_replay_undo_boundary_requires_workload_attached_batch_execution_receipt() {
    let label = "phase11 ordinary consumer sweep replay undo boundary denial";
    let completed_split_handoff = ordinary_completed_split_handoff(label);
    let topology_undo_support =
        ordinary_topology_undo_support::ordinary_traversal_views_undo_scope_support();
    let topology_undo_scope_product = topology_undo_support
        .lower_undo_scope_product()
        .expect("ordinary topology undo scope product");

    let error = with_replay_undo_scope_products(
        label,
        &completed_split_handoff,
        |replay_scope, undo_scope| {
            completed_split_handoff
                .admit_batch_execution_cluster()
                .and_then(|cluster| {
                    cluster.admit_boolean_split_replay_undo_boundary(
                        crate::workload_composition::BooleanSplitReplayUndoBoundaryRequest::new(
                            &topology_undo_scope_product,
                            replay_scope,
                            undo_scope,
                        ),
                    )
                })
        },
    )
    .expect_err("replay/undo boundary consumer must require explicit batch execution authority");

    assert_eq!(
        error.lookup_consumed_workload_denial(),
        Some(&LookupConsumedWorkloadDenial::MissingWorkloadAttachedBatchAdmissionExecutionReceipt)
    );
}

#[test]
fn boolean_split_replay_undo_boundary_binds_explicit_batch_execution_authority_chain() {
    let label = "phase11 ordinary consumer sweep replay undo boundary positive";
    let batch_execution =
        spatial_batch_execution_slice::disjoint_parallel_spatial_batch_execution_slice()
            .execution_receipt()
            .clone();
    let attached_split_handoff = attached_completed_split_handoff(label, &batch_execution);
    let topology_undo_support =
        ordinary_topology_undo_support::ordinary_traversal_views_undo_scope_support();
    let topology_undo_scope_product = topology_undo_support
        .lower_undo_scope_product()
        .expect("ordinary topology undo scope product");

    let admitted_boundary = with_replay_undo_scope_products(
        label,
        &attached_split_handoff,
        |replay_scope, undo_scope| {
            attached_split_handoff
                .admit_batch_execution_cluster()
                .expect("attached split handoff should admit the batch execution cluster")
                .admit_boolean_split_replay_undo_boundary(
                    crate::workload_composition::BooleanSplitReplayUndoBoundaryRequest::new(
                        &topology_undo_scope_product,
                        replay_scope,
                        undo_scope,
                    ),
                )
        },
    )
    .expect("replay/undo boundary should close from the explicit cluster");

    assert_eq!(
        admitted_boundary
            .completed_split_handoff()
            .completed_workload()
            .batch_admission_execution()
            .expect("attached workload batch execution")
            .selected_batch_plan_digest(),
        batch_execution.selected_batch_plan_digest()
    );
    assert_eq!(
        admitted_boundary
            .transaction_boundary_packet()
            .stage_index_identity()
            .digest(),
        attached_split_handoff.workload_stage_index_identity()
    );
    assert_eq!(
        admitted_boundary
            .transaction_boundary_packet()
            .evidence_lookup_receipt_identity()
            .digest(),
        attached_split_handoff
            .lookup_consumed_workload_handoff()
            .lookup_execution_receipt_digest()
    );
}

#[test]
fn replay_undo_boundary_denial_kind_is_typed() {
    let label = "phase11 ordinary consumer sweep replay undo boundary typed denial";
    let foreign_label = "phase11 ordinary consumer sweep replay undo boundary foreign";
    let batch_execution =
        spatial_batch_execution_slice::disjoint_parallel_spatial_batch_execution_slice()
            .execution_receipt()
            .clone();
    let attached_split_handoff = attached_completed_split_handoff(label, &batch_execution);
    let foreign_attached_split_handoff =
        attached_completed_split_handoff(foreign_label, &batch_execution);
    let topology_undo_support =
        ordinary_topology_undo_support::ordinary_traversal_views_undo_scope_support();
    let topology_undo_scope_product = topology_undo_support
        .lower_undo_scope_product()
        .expect("ordinary topology undo scope product");
    let error = with_replay_undo_scope_products(
        foreign_label,
        &foreign_attached_split_handoff,
        |foreign_replay_scope, foreign_undo_scope| {
            attached_split_handoff
                .admit_batch_execution_cluster()
                .expect("attached split handoff should admit the batch execution cluster")
                .admit_boolean_split_replay_undo_boundary(
                    crate::workload_composition::BooleanSplitReplayUndoBoundaryRequest::new(
                        &topology_undo_scope_product,
                        foreign_replay_scope,
                        foreign_undo_scope,
                    ),
                )
        },
    )
    .expect_err("foreign undo scope should fail the replay/undo boundary");

    assert_eq!(
        error.replay_undo_boundary_denial(),
        Some(&ReplayUndoBoundaryDenial::PacketStageIndexMismatchCompletedSplit)
    );
}

#[test]
fn worth_workload_consumer_sweep_residue_rows_are_exact_and_non_authoritative() {
    let rows = worth_workload_ordinary_consumer_residue_rows();

    assert_eq!(rows.len(), 2);
    assert_eq!(
        rows[0].surface(),
        WorthWorkloadOrdinaryConsumerResidueSurface::PlanarBooleanLoopRuntimeRegistrationProof
    );
    assert_eq!(
        rows[0].boundary(),
        WorthWorkloadOrdinaryConsumerResidueBoundary::QueryProofAccompanimentOnly
    );
    assert_eq!(
        rows[1].surface(),
        WorthWorkloadOrdinaryConsumerResidueSurface::BooleanChainIntegrationHandoff
    );
    assert_eq!(
        rows[1].boundary(),
        WorthWorkloadOrdinaryConsumerResidueBoundary::ReplayUndoCloseoutOnly
    );
    assert!(rows.iter().all(|row| !row.owner().is_empty()));
    assert!(rows.iter().all(|row| !row.blocker().is_empty()));
    assert!(rows.iter().all(|row| !row.removal_trigger().is_empty()));
}
