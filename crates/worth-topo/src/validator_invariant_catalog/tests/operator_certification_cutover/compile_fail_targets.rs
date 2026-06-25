use std::collections::BTreeSet;

use crate::validator_invariant_catalog::worth_topology_legality_catalog_compile_fail_targets;

#[test]
fn phase_seven_public_compile_fail_targets_are_registered() {
    let targets = worth_topology_legality_catalog_compile_fail_targets();
    let paths = targets
        .iter()
        .map(|target| target.path())
        .collect::<BTreeSet<_>>();

    for expected in [
        "tests/ui/validator_invariant_catalog/operator_certification_cutover_closeout_struct_literal.rs",
        "tests/ui/validator_invariant_catalog/operator_selected_obligation_row_struct_literal.rs",
        "tests/ui/validator_invariant_catalog/operator_certification_phase_eight_seed_struct_literal.rs",
        "tests/ui/validator_invariant_catalog/raw_expectation_residue_cannot_replace_cutover.rs",
    ] {
        assert!(
            paths.contains(expected),
            "Phase 7 public compile-fail target `{expected}` must stay registered"
        );
    }
}
