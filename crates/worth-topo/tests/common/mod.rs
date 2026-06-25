pub(crate) fn configure_shared_trybuild_workspace() {
    let workspace_temp = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("trybuild-shared");
    std::fs::create_dir_all(&workspace_temp).expect("shared trybuild temp directory");

    std::env::set_var("HOME", workspace_temp.join("home"));
    std::env::set_var("USERPROFILE", workspace_temp.join("home"));
    std::env::set_var("TMP", workspace_temp.join("tmp"));
    std::env::set_var("TEMP", workspace_temp.join("tmp"));
    std::env::set_var("CARGO_TARGET_DIR", workspace_temp.join("cargo-target"));

    std::fs::create_dir_all(workspace_temp.join("home")).expect("shared trybuild home");
    std::fs::create_dir_all(workspace_temp.join("tmp")).expect("shared trybuild temp");
}
