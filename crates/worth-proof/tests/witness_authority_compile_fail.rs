mod support;

use support::compile_fail::run_compile_fail_families;
use support::milestone2;

#[test]
fn witness_authority_boundaries_hold() {
    let bundle = milestone2::compile_fail_bundle();
    run_compile_fail_families(
        &bundle,
        &["witness_minting", "witness_boundaries", "recipe_boundaries"],
    );
}
