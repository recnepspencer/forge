use super::fixtures::rewire_operator_enforcement_closeout;

#[test]
fn certification_entrypoint_routes_operator_closeout_to_selected_obligation_cutover() {
    let enforcement = rewire_operator_enforcement_closeout();
    let cutover =
        crate::certification::certify_topology_operator_selected_obligation_cutover(&enforcement)
            .expect(
                "certification must route operator closeout through selected obligation cutover",
            );

    assert_eq!(
        cutover.phase_seven_enforcement_seed_digest(),
        enforcement.phase_seven_seed().seed_digest()
    );
    assert_eq!(
        cutover.counters().selected_obligation_closeout_row_count(),
        enforcement.enforcement_receipts().len()
    );
}
