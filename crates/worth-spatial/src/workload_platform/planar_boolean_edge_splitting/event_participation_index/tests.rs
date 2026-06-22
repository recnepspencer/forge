use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanSplitSourceEdgeCarrier, PlanarBooleanSplitSourceEdgeCarrierCounters,
    PlanarBooleanSplitSourceEdgeCarrierSet,
};
use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanEventGroup, PlanarBooleanEventGroupInput, PlanarBooleanEventGroupKind,
    PlanarBooleanEventLedgerCounters, PlanarBooleanEventLedgerReceipt,
    PlanarBooleanEventLedgerReceiptInput, PlanarBooleanLoopRole, PlanarBooleanOrderedEventSet,
    PlanarBooleanSegmentCarrier, PlanarBooleanSegmentCarrierEndpointFacts,
    PlanarBooleanSegmentCarrierInput,
};

use super::builder::build_participation_index;
use super::denial::PlanarBooleanSplitEventParticipationDenialKind;

#[test]
fn split_event_participation_index_rejects_event_with_unknown_carrier() {
    let known_carrier = carrier("known carrier edge", [0.0, 0.0], [1.0, 0.0]);
    let unknown_carrier = carrier("unknown carrier edge", [2.0, 0.0], [3.0, 0.0]);
    let group = group_with_carrier(
        "group with orphan carrier",
        unknown_carrier.carrier_identity(),
    );
    let ledger = ledger_with_carriers_and_groups(vec![known_carrier], vec![group]);
    let recovered = recovered_carriers_for(&ledger);
    let denial = build_participation_index(&recovered, &ledger)
        .expect_err("group carrier outside the carrier rows must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanSplitEventParticipationDenialKind::UnknownCarrierReference
    );
    assert_eq!(denial.rejected_orphan_references(), 1);
}

#[test]
fn split_event_participation_index_rejects_grouped_point_event_outside_ledger() {
    let known_carrier = carrier("known carrier edge", [0.0, 0.0], [1.0, 0.0]);
    let group = group_with_unknown_point_event(known_carrier.carrier_identity());
    let ledger = ledger_with_carriers_and_groups(vec![known_carrier], vec![group]);
    let recovered = recovered_carriers_for(&ledger);
    let denial = build_participation_index(&recovered, &ledger)
        .expect_err("grouped point event outside the event ledger must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanSplitEventParticipationDenialKind::UnknownGroupedPointEvent
    );
    assert_eq!(denial.rejected_orphan_references(), 1);
}

#[test]
fn split_event_participation_index_rejects_grouped_interval_event_outside_ledger() {
    let known_carrier = carrier("known carrier edge", [0.0, 0.0], [1.0, 0.0]);
    let group = group_with_unknown_interval_event(known_carrier.carrier_identity());
    let ledger = ledger_with_carriers_and_groups(vec![known_carrier], vec![group]);
    let recovered = recovered_carriers_for(&ledger);
    let denial = build_participation_index(&recovered, &ledger)
        .expect_err("grouped interval event outside the event ledger must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanSplitEventParticipationDenialKind::UnknownGroupedIntervalEvent
    );
    assert_eq!(denial.rejected_orphan_references(), 1);
}

#[test]
fn split_event_participation_index_collapses_duplicate_group_references() {
    let known_carrier = carrier("known carrier edge", [0.0, 0.0], [1.0, 0.0]);
    let group = PlanarBooleanEventGroup::new(PlanarBooleanEventGroupInput {
        group_identity: "duplicate carrier group".to_string(),
        kind: PlanarBooleanEventGroupKind::CoincidentPoint,
        canonical_group_key: "duplicate carrier group key".to_string(),
        point_event_identities: Vec::new(),
        interval_event_identities: Vec::new(),
        segment_pair_identities: Vec::new(),
        participating_carrier_identities: vec![
            known_carrier.carrier_identity().to_string(),
            known_carrier.carrier_identity().to_string(),
        ],
        source_endpoint_identities: Vec::new(),
        source_interval_identities: Vec::new(),
    });
    let ledger = ledger_with_carriers_and_groups(vec![known_carrier], vec![group]);
    let recovered = recovered_carriers_for(&ledger);
    let index = build_participation_index(&recovered, &ledger)
        .expect("duplicate references should canonicalize into one row reference");

    assert_eq!(index.rows()[0].event_group_identities().len(), 1);
    assert_eq!(index.counters().duplicate_references_collapsed(), 1);
}

#[test]
fn split_event_participation_index_counts_canonical_rows_not_ledger_carrier_copies() {
    let known_carrier = carrier("known carrier edge", [0.0, 0.0], [1.0, 0.0]);
    let ledger =
        ledger_with_carriers_and_groups(vec![known_carrier.clone(), known_carrier], Vec::new());
    let recovered = recovered_carriers_for(&ledger);
    let index = build_participation_index(&recovered, &ledger)
        .expect("duplicate carrier copies should collapse into one indexed row");

    assert_eq!(ledger.segment_carriers().len(), 2);
    assert_eq!(index.rows().len(), 1);
    assert_eq!(index.counters().carriers_indexed(), 1);
}

#[test]
fn split_event_participation_index_is_stable_under_carrier_and_group_order_variation() {
    let first_carrier = carrier("first carrier edge", [0.0, 0.0], [1.0, 0.0]);
    let second_carrier = carrier("second carrier edge", [2.0, 0.0], [3.0, 0.0]);
    let first_group = group_with_carrier("first group", first_carrier.carrier_identity());
    let second_group = group_with_carrier("second group", second_carrier.carrier_identity());

    let ordinary = ledger_with_carriers_and_groups(
        vec![first_carrier.clone(), second_carrier.clone()],
        vec![first_group.clone(), second_group.clone()],
    );
    let reordered = ledger_with_carriers_and_groups(
        vec![second_carrier, first_carrier],
        vec![second_group, first_group],
    );
    let ordinary_index = build_participation_index(&recovered_carriers_for(&ordinary), &ordinary)
        .expect("ordinary participation index should build");
    let reordered_index =
        build_participation_index(&recovered_carriers_for(&reordered), &reordered)
            .expect("reordered participation index should build");

    assert_eq!(
        ordinary_index
            .rows()
            .iter()
            .map(|row| row.carrier_identity())
            .collect::<Vec<_>>(),
        reordered_index
            .rows()
            .iter()
            .map(|row| row.carrier_identity())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        ordinary_index
            .rows()
            .iter()
            .map(|row| row.participation_row_identity())
            .collect::<Vec<_>>(),
        reordered_index
            .rows()
            .iter()
            .map(|row| row.participation_row_identity())
            .collect::<Vec<_>>()
    );
}

fn ledger_with_carriers_and_groups(
    segment_carriers: Vec<PlanarBooleanSegmentCarrier>,
    event_groups: Vec<PlanarBooleanEventGroup>,
) -> PlanarBooleanEventLedgerReceipt {
    PlanarBooleanEventLedgerReceipt::new(PlanarBooleanEventLedgerReceiptInput {
        reduced_pair_identity: "test reduced pair".to_string(),
        event_extraction_request_identity: "test request".to_string(),
        segment_carrier_set_identity: "test carrier set".to_string(),
        segment_carriers,
        segment_pair_enumeration_identity: "test pair enumeration".to_string(),
        predicate_binding_identity: "test predicate binding".to_string(),
        point_event_extraction_identity: "test point extraction".to_string(),
        collinear_relation_receipt_identity: "test collinear relations".to_string(),
        interval_event_extraction_identity: "test interval extraction".to_string(),
        point_events: Vec::new(),
        interval_events: Vec::new(),
        relation_diagnostics: Vec::new(),
        ordered_events: PlanarBooleanOrderedEventSet::from_events_and_groups(
            &[],
            &[],
            &event_groups,
            Vec::new(),
        ),
        event_groups,
        counters: PlanarBooleanEventLedgerCounters::default(),
        event_ledger_identity: "test event ledger".to_string(),
        downstream_consumption_identity: "test downstream consumption".to_string(),
    })
}

fn recovered_carriers_for(
    ledger: &PlanarBooleanEventLedgerReceipt,
) -> PlanarBooleanSplitSourceEdgeCarrierSet {
    let carriers = ledger
        .segment_carriers()
        .iter()
        .map(|carrier| {
            PlanarBooleanSplitSourceEdgeCarrier::from_segment_carrier(
                "test scope admission",
                ledger.event_ledger_identity(),
                carrier,
            )
        })
        .collect::<Vec<_>>();
    PlanarBooleanSplitSourceEdgeCarrierSet::new(
        "test scope admission".to_string(),
        "test split request".to_string(),
        ledger.event_ledger_identity().to_string(),
        ledger.segment_carrier_set_identity().to_string(),
        "test candidate index product".to_string(),
        "test query index plan".to_string(),
        carriers,
        PlanarBooleanSplitSourceEdgeCarrierCounters::new(
            ledger.segment_carriers().len(),
            ledger.segment_carriers().len(),
            0,
            0,
            0,
            0,
            ledger.segment_carriers().len(),
        ),
    )
}

fn group_with_carrier(group_identity: &str, carrier_identity: &str) -> PlanarBooleanEventGroup {
    PlanarBooleanEventGroup::new(PlanarBooleanEventGroupInput {
        group_identity: group_identity.to_string(),
        kind: PlanarBooleanEventGroupKind::CoincidentPoint,
        canonical_group_key: format!("{group_identity} key"),
        point_event_identities: Vec::new(),
        interval_event_identities: Vec::new(),
        segment_pair_identities: Vec::new(),
        participating_carrier_identities: vec![carrier_identity.to_string()],
        source_endpoint_identities: Vec::new(),
        source_interval_identities: Vec::new(),
    })
}

fn group_with_unknown_point_event(carrier_identity: &str) -> PlanarBooleanEventGroup {
    PlanarBooleanEventGroup::new(PlanarBooleanEventGroupInput {
        group_identity: "group with unknown point".to_string(),
        kind: PlanarBooleanEventGroupKind::CoincidentPoint,
        canonical_group_key: "group with unknown point key".to_string(),
        point_event_identities: vec!["missing point event".to_string()],
        interval_event_identities: Vec::new(),
        segment_pair_identities: Vec::new(),
        participating_carrier_identities: vec![carrier_identity.to_string()],
        source_endpoint_identities: Vec::new(),
        source_interval_identities: Vec::new(),
    })
}

fn group_with_unknown_interval_event(carrier_identity: &str) -> PlanarBooleanEventGroup {
    PlanarBooleanEventGroup::new(PlanarBooleanEventGroupInput {
        group_identity: "group with unknown interval".to_string(),
        kind: PlanarBooleanEventGroupKind::CoincidentInterval,
        canonical_group_key: "group with unknown interval key".to_string(),
        point_event_identities: Vec::new(),
        interval_event_identities: vec!["missing interval event".to_string()],
        segment_pair_identities: Vec::new(),
        participating_carrier_identities: vec![carrier_identity.to_string()],
        source_endpoint_identities: Vec::new(),
        source_interval_identities: Vec::new(),
    })
}

fn carrier(
    source_edge_identity: &str,
    start_point: [f64; 2],
    end_point: [f64; 2],
) -> PlanarBooleanSegmentCarrier {
    PlanarBooleanSegmentCarrier::new(PlanarBooleanSegmentCarrierInput {
        operand_side: PlanarBooleanCommonPlaneOperandSide::Left,
        source_face_identity: "test face".to_string(),
        source_loop_identity: "test loop".to_string(),
        source_edge_identity: source_edge_identity.to_string(),
        loop_role: PlanarBooleanLoopRole::OuterBoundary,
        start: endpoint(start_point, &format!("{source_edge_identity} start")),
        end: endpoint(end_point, &format!("{source_edge_identity} end")),
        local_frame_identity: "test local frame".to_string(),
        projection_stage_identity: "test projection stage".to_string(),
        precision_basis_identity: "test precision basis".to_string(),
    })
}

fn endpoint(
    point: [f64; 2],
    source_endpoint_identity: &str,
) -> PlanarBooleanSegmentCarrierEndpointFacts {
    PlanarBooleanSegmentCarrierEndpointFacts::for_canonical_segment_test(
        point,
        source_endpoint_identity,
        &format!("{source_endpoint_identity} projected fact"),
    )
}
