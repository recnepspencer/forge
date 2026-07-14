#[test]
fn subscription_phase_seven_compile_fail_boundaries_hold() {
    let workspace_temp = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("trybuild-subscription-phase-seven");
    std::fs::create_dir_all(&workspace_temp).expect("trybuild temp directory");
    std::env::set_var("HOME", &workspace_temp);
    std::env::set_var("USERPROFILE", &workspace_temp);
    std::env::set_var("TMP", &workspace_temp);
    std::env::set_var("TEMP", &workspace_temp);
    std::env::set_var("CARGO_TARGET_DIR", workspace_temp.join("cargo-target"));

    let t = trybuild::TestCases::new();
    for golden in
        worth_query::facade::certification::worth_query_subscription_phase_seven_golden_paths()
    {
        t.pass(golden.path());
    }
    for target in
        worth_query::facade::certification::worth_query_subscription_phase_seven_compile_fail_targets()
    {
        t.compile_fail(target.path());
    }
}

#[test]
fn subscription_phase_seven_manifest_counts_hold() {
    assert_eq!(
        worth_query::facade::certification::worth_query_subscription_phase_seven_compile_fail_targets().len(),
        worth_query::facade::certification::WORTH_QUERY_SUBSCRIPTION_PHASE_SEVEN_COMPILE_FAIL_TARGET_COUNT
    );
    assert_eq!(
        worth_query::facade::certification::worth_query_subscription_phase_seven_golden_paths()
            .len(),
        worth_query::facade::certification::WORTH_QUERY_SUBSCRIPTION_PHASE_SEVEN_GOLDEN_PATH_COUNT
    );
}
