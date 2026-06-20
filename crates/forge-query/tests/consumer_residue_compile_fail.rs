#[test]
fn consumer_residue_query_owned_authority_is_not_consumer_forgeable() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/consumer_residue/*.rs");
}
