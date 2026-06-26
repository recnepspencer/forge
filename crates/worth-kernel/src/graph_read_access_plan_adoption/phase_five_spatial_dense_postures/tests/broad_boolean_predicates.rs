use super::super::slice_classification::WorthGraphReadAccessUnresolvedSliceKind;
use super::broad_boolean_phase_five_closeout;

#[test]
fn broad_boolean_predicate_does_not_degrade_to_whole_graph_scan() {
    let closeout = broad_boolean_phase_five_closeout();

    assert!(
        closeout.counters().broad_boolean_slice_count() > 0,
        "broad boolean read-family target must survive into Phase 5 classification"
    );
    assert_eq!(closeout.counters().unbounded_ephemeral_index_count(), 0);
    assert!(closeout
        .posture_projections()
        .iter()
        .filter(|projection| {
            projection.slice_kind()
                == WorthGraphReadAccessUnresolvedSliceKind::BroadBooleanPredicateRead
        })
        .all(|projection| !projection.claims_graph_read_receipt()));
}
