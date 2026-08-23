#[path = "compile_certification/compile_fail_cases.rs"]
mod compile_fail_cases;
#[path = "compile_certification/pass_cases.rs"]
mod pass_cases;

#[test]
fn query_compiler_boundaries_hold() {
    let cases = trybuild::TestCases::new();
    pass_cases::register(&cases);
    compile_fail_cases::register(&cases);
}

#[path = "certification_kit_contracts.rs"]
mod certification_kit_contracts;
#[path = "hostile_provider_fixture.rs"]
mod hostile_provider_fixture;
