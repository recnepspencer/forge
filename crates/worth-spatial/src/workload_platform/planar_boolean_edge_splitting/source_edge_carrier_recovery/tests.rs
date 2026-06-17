use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanSplitEventParticipationDenialKind, PlanarBooleanSplitEventParticipationIndex,
};
use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanSegmentCarrier, PlanarBooleanSegmentCarrierEndpointFacts as EndpointFacts,
};

use super::test_support::{
    carrier, carrier_input_with_all_provenance, carrier_with_provenance, carrier_with_source_edge,
    event_ledger_for, event_ledger_with_interval_event, group_with_carrier,
    interval_event_with_unknown_relation_carriers, production_segment_pair_receipt, recover,
    source_carriers, subject_with_carriers, subject_with_ledger,
};
use super::PlanarBooleanSplitSourceEdgeCarrierRecoveryDenialKind as RecoveryDenialKind;
use super::{
    PlanarBooleanSplitSourceEdgeCarrierCounters, PlanarBooleanSplitSourceEdgeCarrierRecoveryInput,
    PlanarBooleanSplitSourceEdgeCarrierSet,
};
use PlanarBooleanSplitEventParticipationDenialKind as ParticipationDenialKind;

#[test]
fn source_edge_carrier_recovery_preserves_face_loop_edge_and_carrier_identity() {
    let subject = subject_with_carriers(source_carriers());
    let recovered = recover(&subject);

    assert_eq!(
        recovered.event_ledger_identity(),
        subject.ledger.event_ledger_identity()
    );
    assert_eq!(
        recovered.segment_carrier_set_identity(),
        subject.ledger.segment_carrier_set_identity()
    );
    assert_eq!(
        recovered.scope_admission_identity(),
        subject.scope.scope_admission_identity()
    );
    assert_eq!(
        recovered.carriers().len(),
        subject.ledger.segment_carriers().len()
    );
    assert_eq!(
        recovered.counters().topology_bound_carrier_count(),
        recovered.carriers().len()
    );
    assert!(recovered.carriers().iter().all(|carrier| {
        !carrier.source_face_identity().is_empty()
            && !carrier.source_loop_identity().is_empty()
            && !carrier.source_edge_identity().is_empty()
            && !carrier.carrier_identity().is_empty()
            && !carrier.recovered_carrier_identity().is_empty()
    }));
}

#[test]
fn source_edge_carrier_recovery_rejects_coordinate_only_event_rows() {
    let subject = subject_with_carriers(vec![carrier_with_source_edge("")]);
    let denial = PlanarBooleanSplitSourceEdgeCarrierSet::recover(
        PlanarBooleanSplitSourceEdgeCarrierRecoveryInput::from_scope_and_event_ledger(
            &subject.scope,
            &subject.ledger,
        ),
    )
    .expect_err("coordinate-backed carriers without source edge identity must deny");

    assert_eq!(denial.kind(), RecoveryDenialKind::MissingSourceEdgeIdentity);
}

#[test]
fn source_edge_carrier_recovery_rejects_missing_required_provenance_fields() {
    let missing_provenance_cases = [
        (
            RecoveryDenialKind::MissingSourceFaceIdentity,
            carrier_without_source_face as fn() -> PlanarBooleanSegmentCarrier,
        ),
        (
            RecoveryDenialKind::MissingSourceLoopIdentity,
            carrier_without_source_loop,
        ),
        (
            RecoveryDenialKind::MissingLocalFrameIdentity,
            carrier_without_local_frame,
        ),
        (
            RecoveryDenialKind::MissingProjectionStageIdentity,
            carrier_without_projection_stage,
        ),
        (
            RecoveryDenialKind::MissingPrecisionBasisIdentity,
            carrier_without_precision_basis,
        ),
        (
            RecoveryDenialKind::MissingEndpointSourceIdentity,
            carrier_without_start_source_endpoint,
        ),
        (
            RecoveryDenialKind::MissingProjectedEndpointFactIdentity,
            carrier_without_start_projected_endpoint_fact,
        ),
    ];

    for (expected_denial, malformed_carrier) in missing_provenance_cases {
        let subject = subject_with_carriers(vec![malformed_carrier()]);
        let denial = PlanarBooleanSplitSourceEdgeCarrierSet::recover(
            PlanarBooleanSplitSourceEdgeCarrierRecoveryInput::from_scope_and_event_ledger(
                &subject.scope,
                &subject.ledger,
            ),
        )
        .expect_err("missing source-edge recovery provenance must deny, not warn");

        assert_eq!(denial.kind(), expected_denial);
    }
}

#[test]
fn source_edge_carrier_recovery_is_stable_under_event_order_variation() {
    let original = subject_with_carriers(source_carriers());
    let mut reversed_carriers = source_carriers();
    reversed_carriers.reverse();
    let reversed = subject_with_carriers(reversed_carriers);

    let original_recovered = recover(&original);
    let reversed_recovered = recover(&reversed);

    assert_eq!(
        original_recovered
            .carriers()
            .iter()
            .map(|carrier| carrier.carrier_identity())
            .collect::<Vec<_>>(),
        reversed_recovered
            .carriers()
            .iter()
            .map(|carrier| carrier.carrier_identity())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        original_recovered.counters().recovered_carrier_count(),
        reversed_recovered.counters().recovered_carrier_count()
    );
}

#[test]
fn source_edge_carrier_recovery_keys_source_edges_by_operand_side() {
    let carriers = vec![
        carrier(
            PlanarBooleanCommonPlaneOperandSide::Left,
            "shared-source-edge",
        ),
        carrier(
            PlanarBooleanCommonPlaneOperandSide::Right,
            "shared-source-edge",
        ),
    ];
    let subject = subject_with_carriers(carriers);
    let recovered = recover(&subject);

    assert_eq!(recovered.counters().distinct_source_edge_count(), 2);
    assert_eq!(
        recovered
            .carriers_for_source_edge(
                PlanarBooleanCommonPlaneOperandSide::Left,
                "shared-source-edge"
            )
            .len(),
        1
    );
    assert_eq!(
        recovered
            .carriers_for_source_edge(
                PlanarBooleanCommonPlaneOperandSide::Right,
                "shared-source-edge"
            )
            .len(),
        1
    );
}

#[test]
fn source_edge_carrier_recovery_rejects_scope_event_ledger_mismatch() {
    let subject = subject_with_carriers(source_carriers());
    let foreign = event_ledger_for(
        subject.segment_pairs.segment_pair_enumeration_identity(),
        source_carriers(),
        Vec::new(),
        "foreign-ledger",
    );
    let denial = PlanarBooleanSplitSourceEdgeCarrierSet::recover(
        PlanarBooleanSplitSourceEdgeCarrierRecoveryInput::from_scope_and_event_ledger(
            &subject.scope,
            &foreign,
        ),
    )
    .expect_err("scope admission must bind carrier recovery to its event ledger");

    assert_eq!(
        denial.kind(),
        RecoveryDenialKind::ScopeLedgerIdentityMismatch
    );
}

#[test]
fn source_edge_carrier_recovery_rejects_unknown_group_carrier() {
    let known = source_carriers();
    let unknown = carrier_with_source_edge("unknown-edge");
    let group = group_with_carrier("group-with-unknown-carrier", unknown.carrier_identity());
    let subject = subject_with_ledger(event_ledger_for(
        production_segment_pair_receipt(&known).segment_pair_enumeration_identity(),
        known,
        vec![group],
        "event-ledger-with-unknown-group-carrier",
    ));

    let denial = PlanarBooleanSplitSourceEdgeCarrierSet::recover(
        PlanarBooleanSplitSourceEdgeCarrierRecoveryInput::from_scope_and_event_ledger(
            &subject.scope,
            &subject.ledger,
        ),
    )
    .expect_err("event groups may not reference carriers outside recovered rows");

    assert_eq!(
        denial.kind(),
        RecoveryDenialKind::UnknownGroupedCarrierReference
    );
}

#[test]
fn source_edge_carrier_recovery_rejects_unknown_interval_carrier() {
    let known = source_carriers();
    let interval_event = interval_event_with_unknown_relation_carriers();
    let subject = subject_with_ledger(event_ledger_with_interval_event(
        production_segment_pair_receipt(&known).segment_pair_enumeration_identity(),
        known,
        interval_event,
        "event-ledger-with-unknown-interval-carrier",
    ));

    let denial = PlanarBooleanSplitSourceEdgeCarrierSet::recover(
        PlanarBooleanSplitSourceEdgeCarrierRecoveryInput::from_scope_and_event_ledger(
            &subject.scope,
            &subject.ledger,
        ),
    )
    .expect_err("interval events may not reference carriers outside recovered rows");

    assert_eq!(
        denial.kind(),
        RecoveryDenialKind::UnknownIntervalEventCarrierReference
    );
}

#[test]
fn event_participation_index_requires_recovered_split_carriers() {
    let subject = subject_with_carriers(source_carriers());
    let recovered = recover(&subject);
    let index = PlanarBooleanSplitEventParticipationIndex::from_recovered_carriers(
        &recovered,
        &subject.ledger,
    )
    .expect("participation index should consume Phase 5 recovered carriers");

    assert_eq!(
        index.recovered_carrier_set_identity(),
        recovered.carrier_set_identity()
    );
    assert_eq!(
        index.counters().carriers_indexed(),
        recovered.carriers().len()
    );
}

#[test]
fn event_participation_index_identity_includes_recovered_carrier_set_authority() {
    let subject = subject_with_carriers(source_carriers());
    let recovered = recover(&subject);
    let alternate_recovered = PlanarBooleanSplitSourceEdgeCarrierSet::new(
        "alternate scope admission".to_string(),
        recovered.split_request_identity().to_string(),
        recovered.event_ledger_identity().to_string(),
        recovered.segment_carrier_set_identity().to_string(),
        recovered.candidate_index_product_identity().to_string(),
        recovered.query_index_plan_digest().to_string(),
        recovered.carriers().to_vec(),
        PlanarBooleanSplitSourceEdgeCarrierCounters::new(
            recovered.counters().recovered_carrier_count(),
            recovered.counters().distinct_source_edge_count(),
            recovered.counters().point_carrier_references_inspected(),
            recovered.counters().interval_carrier_references_inspected(),
            recovered.counters().group_carrier_references_inspected(),
            recovered
                .counters()
                .duplicate_carrier_references_collapsed(),
            recovered.counters().topology_bound_carrier_count(),
        ),
    );

    let original_index = PlanarBooleanSplitEventParticipationIndex::from_recovered_carriers(
        &recovered,
        &subject.ledger,
    )
    .expect("original recovered carriers should index");
    let alternate_index = PlanarBooleanSplitEventParticipationIndex::from_recovered_carriers(
        &alternate_recovered,
        &subject.ledger,
    )
    .expect("alternate recovered carrier authority should still index");

    assert_ne!(
        original_index.recovered_carrier_set_identity(),
        alternate_index.recovered_carrier_set_identity()
    );
    assert_ne!(
        original_index.index_identity(),
        alternate_index.index_identity()
    );
}

#[test]
fn event_participation_index_rejects_foreign_recovered_carrier_set_authority() {
    let subject = subject_with_carriers(source_carriers());
    let recovered = recover(&subject);
    let foreign_recovered = PlanarBooleanSplitSourceEdgeCarrierSet::new(
        recovered.scope_admission_identity().to_string(),
        recovered.split_request_identity().to_string(),
        recovered.event_ledger_identity().to_string(),
        "foreign segment carrier set".to_string(),
        recovered.candidate_index_product_identity().to_string(),
        recovered.query_index_plan_digest().to_string(),
        recovered.carriers().to_vec(),
        PlanarBooleanSplitSourceEdgeCarrierCounters::new(
            recovered.counters().recovered_carrier_count(),
            recovered.counters().distinct_source_edge_count(),
            recovered.counters().point_carrier_references_inspected(),
            recovered.counters().interval_carrier_references_inspected(),
            recovered.counters().group_carrier_references_inspected(),
            recovered
                .counters()
                .duplicate_carrier_references_collapsed(),
            recovered.counters().topology_bound_carrier_count(),
        ),
    );

    let denial = PlanarBooleanSplitEventParticipationIndex::from_recovered_carriers(
        &foreign_recovered,
        &subject.ledger,
    )
    .expect_err("participation indexing must reject foreign recovered carrier-set authority");

    assert_eq!(
        denial.kind(),
        ParticipationDenialKind::CarrierSetIdentityMismatch
    );
}

fn carrier_without_source_face() -> PlanarBooleanSegmentCarrier {
    let mut input = carrier_input_with_all_provenance();
    input.source_face_identity.clear();
    carrier_with_provenance(input)
}

fn carrier_without_source_loop() -> PlanarBooleanSegmentCarrier {
    let mut input = carrier_input_with_all_provenance();
    input.source_loop_identity.clear();
    carrier_with_provenance(input)
}

fn carrier_without_local_frame() -> PlanarBooleanSegmentCarrier {
    let mut input = carrier_input_with_all_provenance();
    input.local_frame_identity.clear();
    carrier_with_provenance(input)
}

fn carrier_without_projection_stage() -> PlanarBooleanSegmentCarrier {
    let mut input = carrier_input_with_all_provenance();
    input.projection_stage_identity.clear();
    carrier_with_provenance(input)
}

fn carrier_without_precision_basis() -> PlanarBooleanSegmentCarrier {
    let mut input = carrier_input_with_all_provenance();
    input.precision_basis_identity.clear();
    carrier_with_provenance(input)
}

fn carrier_without_start_source_endpoint() -> PlanarBooleanSegmentCarrier {
    let mut input = carrier_input_with_all_provenance();
    input.start =
        EndpointFacts::for_canonical_segment_test([0.0, 0.0], "", "test projected endpoint fact");
    carrier_with_provenance(input)
}

fn carrier_without_start_projected_endpoint_fact() -> PlanarBooleanSegmentCarrier {
    let mut input = carrier_input_with_all_provenance();
    input.start = EndpointFacts::for_canonical_segment_test([0.0, 0.0], "test source endpoint", "");
    carrier_with_provenance(input)
}
