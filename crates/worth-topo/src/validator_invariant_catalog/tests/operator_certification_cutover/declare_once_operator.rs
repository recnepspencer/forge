use super::fixtures::{
    loop_successor_operator_enforcement_closeout, rewire_operator_enforcement_closeout,
    worth_family_digests,
};
use crate::validator_invariant_catalog::WorthTopologyOperatorCertificationCutoverCloseout;

#[test]
fn one_catalog_family_covers_multiple_operator_touched_bases_without_operator_lists() {
    let rewire_enforcement = rewire_operator_enforcement_closeout();
    let rewire_cutover =
        WorthTopologyOperatorCertificationCutoverCloseout::from_selected_graph_obligation_enforcement(
            &rewire_enforcement,
        )
        .expect("rewire cutover should close");

    let loop_successor_enforcement = loop_successor_operator_enforcement_closeout();
    let loop_successor_cutover =
        WorthTopologyOperatorCertificationCutoverCloseout::from_selected_graph_obligation_enforcement(
            &loop_successor_enforcement,
        )
        .expect("loop-successor cutover should close");

    let rewire_families = worth_family_digests(&rewire_cutover);
    let loop_successor_families = worth_family_digests(&loop_successor_cutover);
    assert!(!rewire_families.is_empty());
    assert_eq!(
        rewire_families, loop_successor_families,
        "matching operator touched bases must share catalog-selected family authority"
    );
    assert_ne!(
        rewire_cutover.selected_plan_digest(),
        loop_successor_cutover.selected_plan_digest(),
        "each operator keeps its own selected plan identity even when family authority is shared"
    );
}
