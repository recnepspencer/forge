#[path = "compile_certification/compile_fail_cases.rs"]
mod compile_fail_cases;

#[test]
fn query_compiler_boundaries_hold() {
    let cases = trybuild::TestCases::new();
    compile_fail_cases::register(&cases);
}
