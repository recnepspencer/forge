mod support;

use support::compile_fail::run_compile_fail_families;
use support::milestone1;

#[test]
fn fixed_shape_and_proven_collection_boundaries_hold() {
    let bundle = milestone1::compile_fail_bundle();
    run_compile_fail_families(
        &bundle,
        &["proven_collection_boundaries", "fixed_shape_boundaries"],
    );
}
