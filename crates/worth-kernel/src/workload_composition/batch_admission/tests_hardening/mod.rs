mod triad_identity;

use super::test_support::{
    alternate_packet_backed_boundary, ordinary_touched_closure, packet_backed_boundary,
    spatial_evidence_route_fixture,
};
use super::{
    admit_batch_admission_grouped_input, current_batch_admission_family_catalog_closeout,
    lower_selected_batch_admission_plan, BatchAdmissionAdvisoryWitnessShape,
    BatchAdmissionCandidate, BatchAdmissionFamilyCatalog, BatchAdmissionFamilyCatalogCloseout,
    BatchAdmissionFamilyDeclaration, BatchAdmissionFamilyDeclarationInput,
    BatchAdmissionFamilyIdentity, BatchAdmissionFamilyPosture, BatchAdmissionGroupedInput,
    BatchAdmissionIndependenceRequirement, BatchAdmissionPairwiseIndependenceProof,
    BatchAdmissionPlanDenialKind,
};
use crate::workload_composition::{
    admit_spatial_conflict_input, admit_topology_conflict_input,
    lower_selected_spatial_conflict_plan, lower_selected_topology_conflict_plan,
    prove_topology_conflict_independence, ConflictPlanDownstreamProofCategory,
    SpatialConflictInputRequest, TopologyConflictIndependenceRequest, TopologyConflictInputRequest,
};
use schema::facade::platform::authority::touched_graph_conflict::ConflictOverlapCategory;
use topology::touched_graph_conflict::current_topology_conflict_family_catalog_closeout;
use worth_spatial::facade::replay_undo_semantic_graph::boolean_event_ledger_spatial_boundary_fixture;
use worth_spatial::touched_graph_conflict::current_spatial_conflict_family_catalog_closeout;

#[test]
fn current_owner_spatial_lane_denies_parallel_without_explicit_proof() {
    let left_fixture = spatial_evidence_route_fixture();
    let right_fixture = boolean_event_ledger_spatial_boundary_fixture();
    let closeout = current_spatial_conflict_family_catalog_closeout().expect("catalog closes");
    let left = admit_spatial_conflict_input(
        SpatialConflictInputRequest::new(left_fixture.authority()).with_evidence_lookup(
            left_fixture.workload_handoff(),
            left_fixture.execution_receipt(),
        ),
    )
    .expect("left admits");
    let right = admit_spatial_conflict_input(
        SpatialConflictInputRequest::new(right_fixture.authority()).with_evidence_lookup(
            right_fixture.workload_handoff(),
            right_fixture.execution_receipt(),
        ),
    )
    .expect("right admits");
    let left_plan = lower_selected_spatial_conflict_plan(&closeout, &left);
    let right_plan = lower_selected_spatial_conflict_plan(&closeout, &right);

    let plan = lower_selected_batch_admission_plan(
        &current_batch_admission_family_catalog_closeout(),
        &admit_batch_admission_grouped_input(BatchAdmissionGroupedInput::new([
            BatchAdmissionCandidate::Spatial(&left_plan),
            BatchAdmissionCandidate::Spatial(&right_plan),
        ]))
        .expect("group admits"),
    );

    assert_eq!(plan.posture(), BatchAdmissionFamilyPosture::Denied);
    assert_eq!(
        plan.selected_family_rows()[0].identity(),
        BatchAdmissionFamilyIdentity::DeniedGroupedOverlap
    );
    assert_eq!(
        plan.denial().expect("denial row").kind(),
        BatchAdmissionPlanDenialKind::MissingExplicitIndependenceProof
    );
}

#[test]
fn triad_group_requires_complete_unordered_pair_coverage() {
    let closeout = current_topology_conflict_family_catalog_closeout().expect("catalog closes");
    let left_closure = ordinary_touched_closure(20, 10, 11);
    let middle_closure = ordinary_touched_closure(30, 21, 22);
    let right_closure = ordinary_touched_closure(40, 31, 32);
    let left = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&left_closure)
            .with_touched_aspect(topology::facade::TopologyTouchedAspect::TopologyBoundary),
    )
    .expect("left admits");
    let middle = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&middle_closure)
            .with_touched_aspect(topology::facade::TopologyTouchedAspect::TopologyBoundary),
    )
    .expect("middle admits");
    let right = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&right_closure)
            .with_touched_aspect(topology::facade::TopologyTouchedAspect::TopologyBoundary),
    )
    .expect("right admits");
    let left_plan = lower_selected_topology_conflict_plan(&closeout, &left);
    let middle_plan = lower_selected_topology_conflict_plan(&closeout, &middle);
    let right_plan = lower_selected_topology_conflict_plan(&closeout, &right);
    let left_middle = prove_topology_conflict_independence(
        TopologyConflictIndependenceRequest::new(&left_plan, &middle_plan),
    );
    let left_right = prove_topology_conflict_independence(
        TopologyConflictIndependenceRequest::new(&left_plan, &right_plan),
    );

    let plan = lower_selected_batch_admission_plan(
        &current_batch_admission_family_catalog_closeout(),
        &admit_batch_admission_grouped_input(
            BatchAdmissionGroupedInput::new([
                BatchAdmissionCandidate::Topology(&left_plan),
                BatchAdmissionCandidate::Topology(&middle_plan),
                BatchAdmissionCandidate::Topology(&right_plan),
            ])
            .with_pairwise_independence(BatchAdmissionPairwiseIndependenceProof::Topology(
                &left_middle,
            ))
            .with_pairwise_independence(
                BatchAdmissionPairwiseIndependenceProof::Topology(&left_right),
            ),
        )
        .expect("group admits"),
    );

    assert_eq!(plan.posture(), BatchAdmissionFamilyPosture::Denied);
    assert_eq!(
        plan.selected_family_rows()[0].identity(),
        BatchAdmissionFamilyIdentity::DeniedGroupedOverlap
    );
    assert_eq!(
        plan.denial().expect("denial row").kind(),
        BatchAdmissionPlanDenialKind::MissingExplicitIndependenceProof
    );
}

#[test]
fn replay_query_boundary_group_selects_advisory_serial_family() {
    let closeout = current_topology_conflict_family_catalog_closeout().expect("catalog closes");
    let left_closure = ordinary_touched_closure(20, 10, 11);
    let right_closure = ordinary_touched_closure(21, 12, 13);
    let left_boundary = packet_backed_boundary("phase7.batch.topology.replay.left");
    let right_boundary = alternate_packet_backed_boundary("phase7.batch.topology.replay.right");
    let left = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&left_closure).with_replay_boundary(&left_boundary),
    )
    .expect("left admits");
    let right = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&right_closure).with_replay_boundary(&right_boundary),
    )
    .expect("right admits");
    let left_plan = lower_selected_topology_conflict_plan(&closeout, &left);
    let right_plan = lower_selected_topology_conflict_plan(&closeout, &right);
    let proof = prove_topology_conflict_independence(TopologyConflictIndependenceRequest::new(
        &left_plan,
        &right_plan,
    ));

    let plan = lower_selected_batch_admission_plan(
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

    assert_eq!(
        plan.posture(),
        BatchAdmissionFamilyPosture::AdvisorySerialAdmit
    );
    assert_eq!(
        plan.selected_family_rows()[0].identity(),
        BatchAdmissionFamilyIdentity::AdvisoryQueryBoundaryParallel
    );
    assert_eq!(
        plan.advisory().expect("advisory witness").witness_shape(),
        BatchAdmissionAdvisoryWitnessShape::QueryBoundarySerialCoordination
    );
}

#[test]
#[should_panic(
    expected = "current batch-admission family catalog must match exactly one grouped plan declaration"
)]
fn ambiguous_batch_family_match_cannot_resolve_by_catalog_order() {
    let closeout = current_topology_conflict_family_catalog_closeout().expect("catalog closes");
    let left_closure = ordinary_touched_closure(20, 10, 11);
    let right_closure = ordinary_touched_closure(30, 21, 22);
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
    let duplicate_declaration =
        BatchAdmissionFamilyDeclaration::new(BatchAdmissionFamilyDeclarationInput {
            identity: BatchAdmissionFamilyIdentity::ParallelProjectionConsumption,
            posture: BatchAdmissionFamilyPosture::ParallelAdmit,
            accepted_overlap_categories: vec![
                ConflictOverlapCategory::Aspect,
                ConflictOverlapCategory::Evidence,
                ConflictOverlapCategory::Locality,
            ],
            accepted_downstream_proof_categories: vec![
                ConflictPlanDownstreamProofCategory::ProjectionConsumption,
            ],
            require_all_selected_plans_admitted: true,
            independence_requirement: BatchAdmissionIndependenceRequirement::CompleteParallelProof,
            advisory_witness_shape: None,
        });
    let ambiguous_closeout =
        BatchAdmissionFamilyCatalogCloseout::close(BatchAdmissionFamilyCatalog::new(vec![
            duplicate_declaration.clone(),
            duplicate_declaration,
        ]));

    let _ = lower_selected_batch_admission_plan(
        &ambiguous_closeout,
        &admit_batch_admission_grouped_input(
            BatchAdmissionGroupedInput::new([
                BatchAdmissionCandidate::Topology(&left_plan),
                BatchAdmissionCandidate::Topology(&right_plan),
            ])
            .with_pairwise_independence(BatchAdmissionPairwiseIndependenceProof::Topology(&proof)),
        )
        .expect("group admits"),
    );
}
