use super::super::test_support::ordinary_touched_closure;
use super::super::{
    admit_batch_admission_grouped_input, current_batch_admission_family_catalog_closeout,
    execute_selected_batch_admission_plan, lower_selected_batch_admission_plan,
    BatchAdmissionCandidate, BatchAdmissionGroupedInput, BatchAdmissionPairwiseIndependenceProof,
};
use crate::workload_composition::{
    admit_topology_conflict_input, lower_selected_topology_conflict_plan,
    prove_topology_conflict_independence, TopologyConflictIndependenceRequest,
    TopologyConflictInputRequest, WorkloadCatalog, WorthWorkload, WorthWorkloadParts,
};
use topology::facade::TopologySeed;
use topology::touched_graph_conflict::current_topology_conflict_family_catalog_closeout;
use worth_spatial::facade::workload_vocabulary::{
    DiagnosticWorkload, GeometryBindingWorkload, ProjectionWorkload, ResponseWorkload,
    RetainedReplayWorkload, SurfaceSupportWorkload, TransformWorkload, WorkloadEvidenceLedger,
    WorkloadEvidenceRow, WorkloadEvidenceStage,
};

#[test]
fn worth_workload_accepts_batch_admission_execution_as_first_class_stage() {
    let (parts, _) = workload_parts_with_batch_execution();

    let workload = WorthWorkload::compose(parts).expect("worth workload should certify");

    assert_eq!(
        workload
            .batch_admission_execution()
            .expect("real batch execution receipt should be present")
            .evidence_stage(),
        WorkloadEvidenceStage::BatchAdmissionExecution
    );
}

#[test]
fn worth_workload_accepts_real_batch_execution_without_generic_ledger_stage_row() {
    let (parts, batch_execution) = workload_parts_with_batch_execution();
    let workload = WorthWorkload::compose(parts).expect("worth workload should certify");

    assert_eq!(
        workload
            .batch_admission_execution()
            .expect("real batch execution receipt should be preserved")
            .execution_receipt_digest(),
        batch_execution.execution_receipt_digest()
    );
}

#[test]
fn workload_catalog_does_not_claim_batch_admission_execution_without_real_receipt() {
    let workload = WorkloadCatalog::cube()
        .with_retained_replay_artifacts()
        .build()
        .expect("catalog cube workload should build")
        .into_workload();

    assert!(workload.batch_admission_execution().is_none());
}

fn workload_parts_with_batch_execution() -> (
    WorthWorkloadParts,
    crate::workload_composition::BatchAdmissionExecutionReceipt,
) {
    let topology = TopologySeed::cube()
        .with_declaration("phase9 worth workload topology")
        .build()
        .expect("topology seed should certify");
    let topology_receipt = topology.query_receipts().declaration_receipt().clone();
    let geometry_binding = GeometryBindingWorkload::for_topology_receipt(&topology_receipt)
        .declared("phase9 worth workload geometry binding")
        .admit()
        .expect("geometry binding should certify");
    let surface_support = SurfaceSupportWorkload::for_geometry_binding(&geometry_binding)
        .declared("phase9 worth workload surface support")
        .admit()
        .expect("surface support should certify");
    let projection = ProjectionWorkload::for_surface_support(&surface_support)
        .declared("phase9 worth workload projection")
        .admit()
        .expect("projection should certify");
    let transform = TransformWorkload::for_projection(&projection)
        .declared("phase9 worth workload transform")
        .admit()
        .expect("transform should certify");
    let retained_replay = RetainedReplayWorkload::for_transform(&transform)
        .declared("phase9 worth workload replay")
        .admit()
        .expect("retained replay should certify");
    let diagnostics = DiagnosticWorkload::for_retained_replay(&retained_replay)
        .declared("phase9 worth workload diagnostics")
        .admit()
        .expect("diagnostics should certify");
    let response = ResponseWorkload::for_diagnostics(&diagnostics)
        .declared("phase9 worth workload response")
        .admit()
        .expect("response should certify");
    let batch_execution = real_batch_execution_receipt();
    let evidence_ledger = WorkloadEvidenceLedger::from_rows(vec![
        WorkloadEvidenceRow::from_topology_seed_receipt(&topology),
        WorkloadEvidenceRow::from_geometry_binding_receipt(&geometry_binding),
        WorkloadEvidenceRow::from_surface_support_receipt(&surface_support),
        WorkloadEvidenceRow::from_projection_receipt(&projection),
        WorkloadEvidenceRow::from_transform_receipt(&transform),
        WorkloadEvidenceRow::from_retained_replay_receipt(&retained_replay),
        WorkloadEvidenceRow::from_diagnostic_receipt(&diagnostics),
        WorkloadEvidenceRow::from_response_receipt(&response),
    ])
    .expect("ledger rows stay inspectable")
    .certify_complete()
    .expect("complete ledger certifies");

    (
        WorthWorkloadParts {
            topology: topology_receipt,
            geometry_binding,
            surface_support,
            projection,
            transform,
            retained_replay,
            batch_admission_execution: Some(batch_execution.clone()),
            diagnostics,
            response,
            evidence_ledger,
        },
        batch_execution,
    )
}

fn real_batch_execution_receipt() -> crate::workload_composition::BatchAdmissionExecutionReceipt {
    let left_closure = ordinary_touched_closure(20, 10, 11);
    let right_closure = ordinary_touched_closure(30, 21, 22);
    let closeout = current_topology_conflict_family_catalog_closeout().expect("catalog closes");
    let left = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&left_closure)
            .with_touched_aspect(topology::facade::TopologyTouchedAspect::TopologyBoundary),
    )
    .expect("left admits");
    let right = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&right_closure)
            .with_touched_aspect(topology::facade::TopologyTouchedAspect::TopologyBoundary),
    )
    .expect("right admits");
    let left_plan = lower_selected_topology_conflict_plan(&closeout, &left);
    let right_plan = lower_selected_topology_conflict_plan(&closeout, &right);
    let proof = prove_topology_conflict_independence(TopologyConflictIndependenceRequest::new(
        &left_plan,
        &right_plan,
    ));
    let selected_plan = lower_selected_batch_admission_plan(
        &current_batch_admission_family_catalog_closeout(),
        &admit_batch_admission_grouped_input(
            BatchAdmissionGroupedInput::new([
                BatchAdmissionCandidate::Topology(&left_plan),
                BatchAdmissionCandidate::Topology(&right_plan),
            ])
            .with_pairwise_independence(BatchAdmissionPairwiseIndependenceProof::Topology(&proof)),
        )
        .expect("group admits"),
    );

    execute_selected_batch_admission_plan(&selected_plan)
}
