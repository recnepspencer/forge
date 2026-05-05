mod support;

use support::compile_pass::run_compile_pass_bundle;
use support::milestone6;

#[test]
fn fixed_arity_fork_join_representative_progression_compiles() {
    run_compile_pass_bundle(&milestone6::compile_pass_bundle());
}
