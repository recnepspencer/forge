mod support;

use support::compile_fail::run_compile_fail_bundle;
use support::milestone6;

#[test]
fn fixed_arity_fork_join_boundaries_hold() {
    run_compile_fail_bundle(&milestone6::compile_fail_bundle());
}
