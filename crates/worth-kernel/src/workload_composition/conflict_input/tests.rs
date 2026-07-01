use super::{
    admit_spatial_conflict_input, admit_topology_conflict_input, AdmittedSpatialConflictRoute,
    AdmittedTopologyConflictRoute, ConflictInputAdmissionErrorKind, SpatialConflictInputRequest,
    TopologyConflictInputRequest,
};
use crate::workload_composition::BooleanSplitReplayUndoBoundaryRequest;
use crate::workload_composition::WorkloadCompositionError;
use schema::facade::platform::authority::touched_graph_conflict::ConflictOverlapCategory;
use topology::facade::{
    DerivedInvalidationTouchedClosure, EntityId, LoopSuccessorKind, PartitionId, RelationId,
    TopologyLoopSuccessorRewireMember, TopologyRewireLoopSuccessorProgramDeclaration,
    TopologyTouchedAspect, TopologyTouchedOperatingWorld,
};
use worth_spatial::facade::replay_family_catalog::current_spatial_replay_family_catalog;
use worth_spatial::facade::replay_undo_semantic_graph::{
    admit_prepared_spatial_replay_semantic_graph_input,
    boolean_event_ledger_spatial_boundary_fixture, current_boolean_event_ledger_spatial_boundary,
    lower_spatial_replay_scope_product_from_admitted_input,
    lower_spatial_undo_scope_product_from_boolean_event_ledger_request,
    prepare_spatial_replay_semantic_graph_request, projection_receipt_spatial_boundary_fixture,
    BooleanEventLedgerRollbackRequest, SpatialReplaySemanticGraphPreparationRequest,
};
#[path = "../../certification/public_facade_contracts/contracts/public_api_planar_boolean_loop_reconstruction_workload_evidence_support/ordinary_topology_undo_support.rs"]
mod ordinary_topology_undo_support;
#[path = "../../certification/public_facade_contracts/contracts/public_api_planar_boolean_loop_reconstruction_workload_evidence_support.rs"]
mod replay_support;

#[test]
fn topology_conflict_input_aspect_route_preserves_aspect_aware_overlap() {
    let touched_closure = ordinary_touched_closure(20, 10, 11);
    let aspect = *touched_closure
        .basis()
        .aspects()
        .first()
        .expect("ordinary touched closure declares an aspect");

    let admitted = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&touched_closure).with_touched_aspect(aspect),
    )
    .expect("ordinary touched closure should admit conflict input");

    assert_eq!(
        admitted.routing_contract().overlap_identity().category(),
        ConflictOverlapCategory::Aspect
    );
    match admitted.route() {
        AdmittedTopologyConflictRoute::AspectLocality(admitted_aspect) => {
            assert_eq!(admitted_aspect, aspect);
        }
        AdmittedTopologyConflictRoute::ReplayBoundary(_) => {
            panic!("aspect-locality admission must preserve the admitted touched aspect")
        }
    }
}

#[test]
fn topology_conflict_input_rejects_missing_explicit_aspect_before_planning() {
    let touched_closure = ordinary_touched_closure(20, 10, 11);

    let error =
        match admit_topology_conflict_input(TopologyConflictInputRequest::new(&touched_closure)) {
            Ok(_) => panic!("topology admission must not guess a touched aspect"),
            Err(error) => error,
        };

    assert_conflict_kind(
        error,
        ConflictInputAdmissionErrorKind::MissingTopologyConflictRoute,
    );
}

#[test]
fn topology_conflict_input_rejects_foreign_aspect_before_planning() {
    let touched_closure = ordinary_touched_closure(20, 10, 11);
    let foreign_aspect = TopologyTouchedAspect::ALL
        .iter()
        .copied()
        .find(|aspect| !touched_closure.basis().aspects().contains(aspect))
        .expect("fixture exposes at least one non-touched aspect");

    let error = match admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&touched_closure).with_touched_aspect(foreign_aspect),
    ) {
        Ok(_) => {
            panic!("topology admission must reject aspects not present in the touched closure")
        }
        Err(error) => error,
    };

    assert_conflict_kind(error, ConflictInputAdmissionErrorKind::MissingTouchedAspect);
}

#[test]
fn topology_conflict_input_replay_route_rejects_foreign_touched_authority_before_selection() {
    let foreign_touched_closure = ordinary_touched_closure(21, 12, 13);
    let boundary = packet_backed_boundary("phase4.topology-conflict.foreign-authority");

    let error = match admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&foreign_touched_closure).with_replay_boundary(&boundary),
    ) {
        Ok(_) => panic!("foreign replay boundary must fail admission before family selection"),
        Err(error) => error,
    };

    assert_conflict_kind(error, ConflictInputAdmissionErrorKind::WrongAuthority);
}

#[test]
fn topology_conflict_input_replay_route_preserves_typed_boundary_proof() {
    let touched_closure = ordinary_touched_closure(20, 10, 11);
    let boundary = packet_backed_boundary("phase4.topology-conflict.replay");

    let admitted = admit_topology_conflict_input(
        TopologyConflictInputRequest::new(&touched_closure).with_replay_boundary(&boundary),
    )
    .expect("typed replay boundary should admit topology conflict input");

    assert_eq!(
        admitted.routing_contract().overlap_identity().category(),
        ConflictOverlapCategory::ReplayUndo
    );
    match admitted.route() {
        AdmittedTopologyConflictRoute::ReplayBoundary(admitted_boundary) => {
            assert_eq!(
                admitted_boundary.packet_identity(),
                boundary.packet_identity()
            );
        }
        AdmittedTopologyConflictRoute::AspectLocality(_) => {
            panic!("replay admission must preserve typed replay boundary proof")
        }
    }
}

#[test]
fn spatial_conflict_input_evidence_route_uses_receipt_backed_lookup_proof() {
    let fixture = boolean_event_ledger_spatial_boundary_fixture();

    let admitted = admit_spatial_conflict_input(
        SpatialConflictInputRequest::new(fixture.authority())
            .with_evidence_lookup(fixture.workload_handoff(), fixture.execution_receipt()),
    )
    .expect("receipt-backed evidence lookup should admit spatial conflict input");

    assert_eq!(
        admitted.routing_contract().overlap_identity().category(),
        ConflictOverlapCategory::Evidence
    );
    match admitted.route() {
        AdmittedSpatialConflictRoute::EvidenceLookup {
            handoff,
            execution_receipt,
        } => {
            assert_eq!(
                handoff.semantic_graph_identity(),
                fixture.workload_handoff().semantic_graph_identity()
            );
            assert_eq!(
                execution_receipt.execution_receipt_digest(),
                fixture.execution_receipt().execution_receipt_digest()
            );
        }
        AdmittedSpatialConflictRoute::LookupCompiledProduct { .. } => {
            panic!("receipt-backed admission must preserve the receipt-backed lookup route")
        }
        AdmittedSpatialConflictRoute::ReplayBoundary(_) => {
            panic!("evidence-backed admission must preserve typed lookup proof")
        }
    }
}

#[test]
fn spatial_conflict_input_lookup_compiled_product_route_uses_real_compiled_product_proof() {
    let boundary = current_boolean_event_ledger_spatial_boundary().expect("current boundary");

    let admitted = admit_spatial_conflict_input(
        SpatialConflictInputRequest::new(boundary.authority())
            .with_lookup_compiled_product(boundary.workload_handoff(), boundary.index_product()),
    )
    .expect("compiled-product-backed lookup proof should admit spatial conflict input");

    assert_eq!(
        admitted.routing_contract().overlap_identity().category(),
        ConflictOverlapCategory::Evidence
    );
    match admitted.route() {
        AdmittedSpatialConflictRoute::LookupCompiledProduct { handoff, product } => {
            assert_eq!(
                handoff.semantic_graph_identity(),
                boundary.workload_handoff().semantic_graph_identity()
            );
            assert_eq!(
                product.index_product_digest(),
                boundary.index_product().index_product_digest()
            );
        }
        AdmittedSpatialConflictRoute::EvidenceLookup { .. } => {
            panic!("compiled-product-backed admission must preserve the compiled-product route")
        }
        AdmittedSpatialConflictRoute::ReplayBoundary(_) => {
            panic!("compiled-product-backed admission must not degrade into replay proof")
        }
    }
}

#[test]
fn spatial_conflict_input_rejects_wrong_receipt_family_before_selection() {
    let fixture = boolean_event_ledger_spatial_boundary_fixture();
    let foreign_fixture = projection_receipt_spatial_boundary_fixture();

    let error = match admit_spatial_conflict_input(
        SpatialConflictInputRequest::new(fixture.authority()).with_evidence_lookup(
            fixture.workload_handoff(),
            foreign_fixture.execution_receipt(),
        ),
    ) {
        Ok(_) => panic!("wrong lookup family must fail admission before family selection"),
        Err(error) => error,
    };

    assert_conflict_kind(error, ConflictInputAdmissionErrorKind::WrongReceiptFamily);
}

#[test]
fn spatial_conflict_input_rejects_stage_index_mismatch_before_selection() {
    let authority_fixture = boolean_event_ledger_spatial_boundary_fixture();
    let foreign_fixture = projection_receipt_spatial_boundary_fixture();

    let error = match admit_spatial_conflict_input(
        SpatialConflictInputRequest::new(authority_fixture.authority()).with_evidence_lookup(
            foreign_fixture.workload_handoff(),
            authority_fixture.execution_receipt(),
        ),
    ) {
        Ok(_) => panic!("stage-index mismatch must fail admission before family selection"),
        Err(error) => error,
    };

    assert_conflict_kind(error, ConflictInputAdmissionErrorKind::StageIndexMismatch);
}

#[test]
fn spatial_conflict_input_lookup_compiled_product_route_rejects_selected_plan_mismatch_before_selection(
) {
    let boundary = current_boolean_event_ledger_spatial_boundary().expect("current boundary");
    let hostile_product = boundary
        .index_product()
        .clone()
        .with_test_selected_plan_digest("forged.selected-plan-digest");

    assert_spatial_compiled_product_denial(
        boundary.authority(),
        boundary.workload_handoff(),
        hostile_product,
        ConflictInputAdmissionErrorKind::WrongReceiptFamily,
    );
}

#[test]
fn spatial_conflict_input_rejects_selected_family_mismatch_before_selection() {
    let fixture = boolean_event_ledger_spatial_boundary_fixture();
    let handoff = fixture.workload_handoff_with_test_selected_equivalence_family_identity(
        "spatial.selected-equivalence.retained-replay-semantic-parity",
    );

    assert_spatial_handoff_denial(handoff, ConflictInputAdmissionErrorKind::WrongReceiptFamily);
}

#[test]
fn spatial_conflict_input_lookup_compiled_product_route_rejects_selected_family_mismatch_before_selection(
) {
    let boundary = current_boolean_event_ledger_spatial_boundary().expect("current boundary");
    let hostile_product = boundary
        .index_product()
        .clone()
        .with_test_selected_equivalence_family_identity(
            "spatial.selected-equivalence.retained-replay-semantic-parity",
        );

    assert_spatial_compiled_product_denial(
        boundary.authority(),
        boundary.workload_handoff(),
        hostile_product,
        ConflictInputAdmissionErrorKind::WrongReceiptFamily,
    );
}

#[test]
fn spatial_conflict_input_rejects_selected_reuse_basis_mismatch_before_selection() {
    let fixture = boolean_event_ledger_spatial_boundary_fixture();
    let handoff = fixture.workload_handoff_with_test_selected_reuse_basis_identity_digest(
        "forged.selected-reuse-basis",
    );

    assert_spatial_handoff_denial(handoff, ConflictInputAdmissionErrorKind::WrongReceiptFamily);
}

#[test]
fn spatial_conflict_input_lookup_compiled_product_route_rejects_selected_reuse_basis_mismatch_before_selection(
) {
    let boundary = current_boolean_event_ledger_spatial_boundary().expect("current boundary");
    let hostile_product = boundary
        .index_product()
        .clone()
        .with_test_selected_reuse_basis_identity_digest("forged.selected-reuse-basis");

    assert_spatial_compiled_product_denial(
        boundary.authority(),
        boundary.workload_handoff(),
        hostile_product,
        ConflictInputAdmissionErrorKind::WrongReceiptFamily,
    );
}

#[test]
fn spatial_conflict_input_lookup_compiled_product_route_rejects_wrong_authority_before_selection() {
    let boundary = current_boolean_event_ledger_spatial_boundary().expect("current boundary");
    let forged_handoff = boundary
        .workload_handoff()
        .clone()
        .with_test_stage_receipt_identity("forged-stage-receipt-identity");

    let error = match admit_spatial_conflict_input(
        SpatialConflictInputRequest::new(boundary.authority())
            .with_lookup_compiled_product(&forged_handoff, boundary.index_product()),
    ) {
        Ok(_) => {
            panic!("compiled-product-backed admission must reject foreign handoff authority before selection")
        }
        Err(error) => error,
    };

    assert_conflict_kind(error, ConflictInputAdmissionErrorKind::WrongAuthority);
}

#[test]
fn spatial_conflict_input_rejects_wrong_authority_before_selection() {
    let fixture = boolean_event_ledger_spatial_boundary_fixture();
    let handoff =
        fixture.workload_handoff_with_test_stage_receipt_identity("foreign-stage-receipt");

    assert_spatial_handoff_denial(handoff, ConflictInputAdmissionErrorKind::WrongAuthority);
}

#[test]
fn spatial_conflict_input_rejects_raw_row_scan_fallback_before_selection() {
    let fixture = boolean_event_ledger_spatial_boundary_fixture();
    let handoff = fixture.workload_handoff_with_test_raw_row_scan_count(1);

    assert_spatial_handoff_denial(handoff, ConflictInputAdmissionErrorKind::RawRowScanDenied);
}

#[test]
fn spatial_conflict_input_rejects_broad_receipt_scan_fallback_before_selection() {
    let fixture = boolean_event_ledger_spatial_boundary_fixture();
    let handoff = fixture.workload_handoff_with_test_broad_receipt_scan_count(1);

    assert_spatial_handoff_denial(
        handoff,
        ConflictInputAdmissionErrorKind::BroadReceiptScanDenied,
    );
}

#[test]
fn spatial_conflict_input_rejects_caller_owned_scan_fallback_before_selection() {
    let fixture = boolean_event_ledger_spatial_boundary_fixture();
    let handoff = fixture.workload_handoff_with_test_caller_owned_scan_count(1);

    assert_spatial_handoff_denial(
        handoff,
        ConflictInputAdmissionErrorKind::CallerOwnedScanDenied,
    );
}

#[test]
fn spatial_conflict_input_replay_route_accepts_typed_boundary_proof() {
    let subject =
        replay_support::MetabossEventExtractionSubject::certify("phase4.spatial-conflict.replay");
    let replay_subject = replay_support::build_edge_split_replay_parity_subject(&subject);
    let completed_split_handoff =
        replay_support::completed_split_handoff_for(&subject, &replay_subject);
    let authority = completed_split_handoff
        .admit_split_spatial_touch_authority()
        .expect("split handoff admits spatial touch authority");
    let boundary = packet_backed_boundary("phase4.spatial-conflict.replay");

    let admitted = admit_spatial_conflict_input(
        SpatialConflictInputRequest::new(&authority).with_replay_boundary(&boundary),
    )
    .expect("typed replay boundary should admit spatial conflict input");

    assert_eq!(
        admitted.routing_contract().overlap_identity().category(),
        ConflictOverlapCategory::ReplayUndo
    );
    match admitted.route() {
        AdmittedSpatialConflictRoute::ReplayBoundary(admitted_boundary) => {
            assert_eq!(
                admitted_boundary.packet_identity(),
                boundary.packet_identity()
            );
        }
        AdmittedSpatialConflictRoute::EvidenceLookup { .. }
        | AdmittedSpatialConflictRoute::LookupCompiledProduct { .. } => {
            panic!("replay admission must preserve typed replay boundary proof")
        }
    }
}

fn ordinary_touched_closure(
    relation_slot: u64,
    source_slot: u64,
    target_slot: u64,
) -> DerivedInvalidationTouchedClosure {
    let declaration = TopologyRewireLoopSuccessorProgramDeclaration::new(vec![
        TopologyLoopSuccessorRewireMember::new(
            RelationId::new(PartitionId::main(), relation_slot, 1),
            LoopSuccessorKind::Next,
            EntityId::new(PartitionId::main(), source_slot, 1),
            EntityId::new(PartitionId::main(), target_slot, 1),
        ),
    ]);
    let proof = declaration
        .declared_touched_basis_proof(
            "topology.rewire_loop_successor_program",
            TopologyTouchedOperatingWorld::mainline(),
        )
        .expect("ordinary topology declaration lowers touched proof");
    DerivedInvalidationTouchedClosure::from_declared_touch(&proof)
}

fn packet_backed_boundary(
    label: &'static str,
) -> crate::workload_composition::AdmittedBooleanSplitReplayUndoBoundary {
    let subject = replay_support::MetabossEventExtractionSubject::certify(label);
    let replay_subject = replay_support::build_edge_split_replay_parity_subject(&subject);
    let completed_split_handoff =
        replay_support::completed_split_handoff_for(&subject, &replay_subject);
    let topology_undo_support =
        ordinary_topology_undo_support::ordinary_traversal_views_undo_scope_support();
    let topology_undo_scope_product = topology_undo_support
        .lower_undo_scope_product()
        .expect("ordinary topology undo scope product");
    let authority = completed_split_handoff
        .admit_split_spatial_touch_authority()
        .expect("split handoff admits spatial touch authority");
    let event_ledger_lookup_packet = subject
        .pair()
        .left()
        .workload()
        .require_boolean_event_ledger_lookup_execution_packet(subject.ledger())
        .expect("event-ledger lookup packet");
    let request = prepare_spatial_replay_semantic_graph_request(
        SpatialReplaySemanticGraphPreparationRequest::new(
            boolean_event_ledger_spatial_boundary_fixture().replay_family_identity(),
            &authority,
            event_ledger_lookup_packet.execution_receipt(),
            completed_split_handoff.lookup_consumed_workload_handoff(),
        )
        .with_retained_replay_receipt(
            completed_split_handoff
                .completed_workload()
                .retained_replay(),
        ),
    )
    .expect("prepared replay request");
    let admitted = admit_prepared_spatial_replay_semantic_graph_input(
        &current_spatial_replay_family_catalog(),
        &request,
    )
    .expect("admitted replay request");
    let replay_scope =
        lower_spatial_replay_scope_product_from_admitted_input(&admitted).expect("replay scope");
    let undo_scope = lower_spatial_undo_scope_product_from_boolean_event_ledger_request(
        BooleanEventLedgerRollbackRequest::new(
            &authority,
            event_ledger_lookup_packet.execution_receipt(),
            completed_split_handoff
                .completed_workload()
                .evidence_ledger()
                .stage_index(),
            completed_split_handoff.lookup_consumed_workload_handoff(),
        ),
    )
    .expect("undo scope");
    completed_split_handoff
        .admit_batch_execution_cluster()
        .expect("packet-backed split handoff admits batch execution cluster")
        .admit_boolean_split_replay_undo_boundary(BooleanSplitReplayUndoBoundaryRequest::new(
            &topology_undo_scope_product,
            &replay_scope,
            &undo_scope,
        ))
        .expect("packet-backed split handoff admits replay/undo boundary")
}

fn assert_conflict_kind(
    error: WorkloadCompositionError,
    expected: ConflictInputAdmissionErrorKind,
) {
    match error {
        WorkloadCompositionError::ConflictInput(error) => assert_eq!(error.kind(), expected),
        other => panic!("expected conflict-input error, got {other:?}"),
    }
}

fn assert_spatial_handoff_denial(
    handoff: worth_spatial::facade::evidence_lookup_workload_cutover::EvidenceLookupConsumedWorkloadHandoff,
    expected: ConflictInputAdmissionErrorKind,
) {
    let fixture = boolean_event_ledger_spatial_boundary_fixture();
    let error = match admit_spatial_conflict_input(
        SpatialConflictInputRequest::new(fixture.authority())
            .with_evidence_lookup(&handoff, fixture.execution_receipt()),
    ) {
        Ok(_) => panic!("mutated lookup handoff must fail admission before selection"),
        Err(error) => error,
    };
    assert_conflict_kind(error, expected);
}

fn assert_spatial_compiled_product_denial(
    authority: &worth_spatial::facade::workload_vocabulary::SpatialGeometryEvidenceTouchAuthority,
    handoff: &worth_spatial::facade::evidence_lookup_workload_cutover::EvidenceLookupConsumedWorkloadHandoff,
    product: worth_spatial::facade::evidence_lookup_index_product::EvidenceLookupIndexProduct,
    expected: ConflictInputAdmissionErrorKind,
) {
    let error = match admit_spatial_conflict_input(
        SpatialConflictInputRequest::new(authority).with_lookup_compiled_product(handoff, &product),
    ) {
        Ok(_) => panic!("mutated compiled product must fail admission before selection"),
        Err(error) => error,
    };
    assert_conflict_kind(error, expected);
}
