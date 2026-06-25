use super::super::slice_classification::WorthGraphReadAccessUnresolvedSliceKind;
use super::production_phase_five_closeout;

#[test]
fn spatial_graph_reads_use_query_plan_or_required_posture() {
    let closeout = production_phase_five_closeout();

    assert_eq!(
        closeout.posture_projections().len(),
        closeout.unresolved_slices().len()
    );
    assert!(closeout
        .posture_projections()
        .iter()
        .all(|projection| !projection.query_posture().is_empty()));
    assert!(
        closeout.counters().spatial_slice_count() > 0,
        "production Phase 5 seed must include spatial graph-read coverage"
    );
    assert!(closeout
        .posture_projections()
        .iter()
        .filter(|projection| {
            projection.slice_kind() == WorthGraphReadAccessUnresolvedSliceKind::SpatialGraphRead
        })
        .all(|projection| !projection.claims_graph_read_receipt()));
    assert_eq!(closeout.counters().receipt_claim_count(), 0);
}
