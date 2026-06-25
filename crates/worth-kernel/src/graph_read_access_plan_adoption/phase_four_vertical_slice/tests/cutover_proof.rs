use super::super::cutover_proof::WorthGraphReadAccessSliceCutoverStatus;
use super::production_phase_four_closeout;

#[test]
fn cutover_proof_does_not_delete_without_receipt() {
    let closeout = production_phase_four_closeout();
    let cutover = closeout.cutover_proof();

    assert_eq!(
        WorthGraphReadAccessSliceCutoverStatus::CappedUntilQueryExecutionSurfaceExists,
        cutover.status()
    );
    assert!(!cutover.old_path_identity().is_empty());
    assert_ne!(
        cutover.old_path_identity(),
        cutover.displaced_evidence_identity()
    );
    assert_eq!(
        cutover.old_path_identity(),
        cutover.deletion_target_identity()
    );
    assert!(cutover
        .deletion_target_identity()
        .contains(cutover.displaced_evidence_identity()));
    assert!(cutover.blocker().is_some());
    assert_eq!(
        "graph_read_access_plan_adoption/phase_four_vertical_slice",
        cutover.source_firewall_region()
    );
}
