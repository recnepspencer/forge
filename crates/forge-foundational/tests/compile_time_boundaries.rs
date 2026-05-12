#[test]
fn facade_internal_homes_are_private() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/ui/facade_boundary/*.rs");
}

#[test]
fn value_vocabulary_rejects_generic_document_authority() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/ui/value_vocabulary/*.rs");
}

#[test]
fn contract_validation_requires_proof_bearing_outputs() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/ui/contract_validation/*.rs");
}

#[test]
fn mask_admissibility_preserves_mode_types() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/ui/mask_admissibility/*.rs");
}

#[test]
fn struct_aspect_values_keep_fields_sealed() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/ui/struct_aspect_values/*.rs");
}
