mod support;

use support::compile_fail::run_compile_fail_bundle;
use support::compile_pass::run_compile_pass_bundle;
use support::contracts;

#[test]
fn every_upfront_contract_door_is_compiler_enforced() {
    run_compile_fail_bundle(&contracts::compile_fail_bundle());
}

#[test]
fn the_upfront_contract_vocabulary_still_composes_on_the_public_lane() {
    run_compile_pass_bundle(&contracts::compile_pass_bundle());
}

#[test]
fn every_contract_failure_family_is_represented() {
    let bundle = contracts::compile_fail_bundle();

    // Naming the families here means retiring one is a test change, not a
    // quietly smaller suite. Each is a distinct way a caller could supply what
    // the runtime must derive.
    let mut families = bundle.families();
    families.sort_unstable();
    assert_eq!(
        families,
        [
            "dynamic_value_authority",
            "evidence_minting",
            "instance_identity",
            "linearity",
            "source_substitution",
        ]
    );
}
