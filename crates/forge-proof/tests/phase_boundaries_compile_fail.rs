mod support;

use support::compile_fail::run_compile_fail_family;
use support::milestone1;

#[test]
fn phase_boundaries_hold() {
    let bundle = milestone1::compile_fail_bundle();
    run_compile_fail_family(&bundle, "phase_boundaries");
}
