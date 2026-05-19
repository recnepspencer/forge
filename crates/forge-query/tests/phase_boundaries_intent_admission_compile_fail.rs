#[test]
fn intent_admission_dx_boundaries_hold() {
    let t = trybuild::TestCases::new();
    for transcript in
        forge_query::facade::runtime::forge_query_intent_admission_golden_transcripts()
    {
        t.pass(transcript.path());
    }
    for target in
        forge_query::facade::runtime::forge_query_intent_admission_crate_doc_example_targets()
    {
        t.pass(target.path());
    }
    for target in forge_query::facade::runtime::forge_query_intent_admission_compile_fail_targets()
    {
        t.compile_fail(target.path());
    }
}
