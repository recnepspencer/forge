#[test]
fn external_code_cannot_construct_surface_registrations_directly() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/ui/private_surface_registration_constructor.rs");
    tests.compile_fail("tests/ui/private_server_constructor.rs");
    tests.compile_fail("tests/ui/server_cannot_serve_twice.rs");
}
