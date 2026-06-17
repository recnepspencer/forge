#[test]
fn query_identity_authority_substitution_boundaries_hold() {
    let workspace_temp = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("trybuild-query-identity-authority");
    std::fs::create_dir_all(&workspace_temp).expect("trybuild temp directory");
    std::env::set_var("HOME", &workspace_temp);
    std::env::set_var("USERPROFILE", &workspace_temp);
    std::env::set_var("TMP", &workspace_temp);
    std::env::set_var("TEMP", &workspace_temp);
    std::env::set_var("CARGO_TARGET_DIR", workspace_temp.join("cargo-target"));

    let t = trybuild::TestCases::new();
    for target in
        forge_query::facade::identity_authority::forge_query_identity_phase_one_compile_fail_targets(
        )
    {
        t.compile_fail(target.path());
    }
    for target in forge_query::facade::identity_authority::forge_query_identity_phase_one_subscription_phase_seven_reentry_targets(
    ) {
        t.compile_fail(target.path());
    }
}
