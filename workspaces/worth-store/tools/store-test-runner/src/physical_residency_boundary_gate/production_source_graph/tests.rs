use super::*;

mod causal_compile;

#[test]
fn external_macro_scan_preserves_raw_keyword_identifiers() {
    for source in [
        "macro_rules! value { () => {{ let r#mod = 1_u8; r#mod }} }",
        "macro_rules! r#macro_rules { () => { 1_u8 } } r#macro_rules!();",
    ] {
        let evidence = macro_provenance::external_declarative_source_expansion(source)
            .expect("inspect external declarative macro source");
        assert!(evidence.denial.is_none());
        assert!(evidence.literal_includes.is_empty());
    }
}

#[test]
fn unguarded_tests_named_module_is_production_reachable() {
    let root = tempfile::tempdir().expect("temporary module graph");
    let source = root.path().join("src");
    std::fs::create_dir(&source).expect("create source directory");
    let tests = source.join("tests.rs");
    std::fs::write(&tests, "pub fn production_owner() {}")
        .expect("write tests-named production module");
    let syntax = syn::parse_file("mod tests;").expect("parse production module declaration");
    let mut pending = Vec::new();
    collect_external_modules(&syntax.items, &root_context(&source), &mut pending)
        .expect("resolve production module");
    assert_eq!(
        pending,
        [PendingSource {
            source: canonical(&tests).expect("canonical module"),
            module_dir: source.join("tests"),
            path_attr_dir: source.clone(),
        }]
    );

    let syntax =
        syn::parse_file("#[cfg(test)] mod tests;").expect("parse test-only module declaration");
    pending.clear();
    collect_external_modules(&syntax.items, &root_context(&source), &mut pending)
        .expect("resolve test-only module posture");
    assert!(pending.is_empty());
}

#[test]
fn nested_literal_include_is_reachable_and_cfg_attr_path_fails_closed() {
    let root = tempfile::tempdir().expect("temporary source graph");
    let source = root.path().join("src");
    std::fs::create_dir(&source).expect("create source directory");
    let included = source.join("alternate_owner_expr.rs");
    std::fs::write(&included, "PhysicalResidencyPoolOwner::open(store, limits)")
        .expect("write included constructor expression");
    let syntax = syn::parse_file("fn alternate() { include!(\"alternate_owner_expr.rs\"); }")
        .expect("parse nested literal include");
    let mut pending = Vec::new();
    collect_external_modules(&syntax.items, &root_context(&source), &mut pending)
        .expect("resolve nested literal include");
    assert_eq!(
        pending,
        [PendingSource {
            source: canonical(&included).expect("canonical include"),
            module_dir: source.clone(),
            path_attr_dir: source.clone(),
        }]
    );

    std::fs::write(source.join("x.rs"), "pub fn decoy() {}")
        .expect("write conventional decoy module");
    std::fs::write(source.join("hidden.rs"), "pub fn hidden_owner() {}")
        .expect("write redirected module");
    let syntax = syn::parse_file("#[cfg_attr(not(test), path = \"hidden.rs\")] mod x;")
        .expect("parse cfg-redirected module");
    pending.clear();
    let denial = collect_external_modules(&syntax.items, &root_context(&source), &mut pending)
        .expect_err("cfg-dependent path must fail closed");
    assert!(
        denial.contains("cfg_attr module path"),
        "wrong denial: {denial}"
    );

    let syntax =
        syn::parse_file("#[cfg_attr(not(test), cfg_attr(not(test), path = \"hidden.rs\"))] mod x;")
            .expect("parse recursively cfg-redirected module");
    pending.clear();
    let denial = collect_external_modules(&syntax.items, &root_context(&source), &mut pending)
        .expect_err("nested cfg-dependent path must fail closed");
    assert!(
        denial.contains("cfg_attr module path"),
        "wrong nested denial: {denial}"
    );

    let syntax = syn::parse_file("std::include!(concat!(\"hidden\", \".rs\"));")
        .expect("parse qualified computed include");
    pending.clear();
    let denial = collect_external_modules(&syntax.items, &root_context(&source), &mut pending)
        .expect_err("qualified computed include must fail closed");
    assert!(
        denial.contains("computed Rust include"),
        "wrong qualified include denial: {denial}"
    );
}

#[test]
fn custom_cargo_target_roots_resolve_children_from_their_parent() {
    let root = tempfile::tempdir().expect("temporary Cargo package");
    let source = root.path().join("src");
    std::fs::create_dir_all(source.join("custom_root")).expect("create custom lib decoy directory");
    std::fs::create_dir_all(source.join("custom_bin_root"))
        .expect("create custom bin decoy directory");
    std::fs::write(
        root.path().join("Cargo.toml"),
        "[package]\nname='custom-root-proof'\nversion='0.0.0'\nedition='2021'\n[lib]\npath='src/custom_root.rs'\n[[bin]]\nname='custom-bin'\npath='src/custom_bin_root.rs'\n",
    )
    .expect("write custom-target manifest");
    std::fs::write(source.join("custom_root.rs"), "mod child;").expect("write custom lib root");
    std::fs::write(source.join("custom_bin_root.rs"), "mod bin_child;")
        .expect("write custom bin root");
    std::fs::write(source.join("child.rs"), "pub fn real_lib_owner() {}")
        .expect("write real lib child");
    std::fs::write(source.join("bin_child.rs"), "pub fn real_bin_owner() {}")
        .expect("write real bin child");
    let lib_decoy = source.join("custom_root/child.rs");
    let bin_decoy = source.join("custom_bin_root/bin_child.rs");
    std::fs::write(&lib_decoy, "pub fn decoy() {}").expect("write lib decoy");
    std::fs::write(&bin_decoy, "pub fn decoy() {}").expect("write bin decoy");

    let sources = production_rust_sources(root.path()).expect("discover custom target sources");
    assert!(sources.contains(&canonical(&source.join("child.rs")).expect("real lib child")));
    assert!(sources.contains(&canonical(&source.join("bin_child.rs")).expect("real bin child")));
    assert!(!sources.contains(&canonical(&lib_decoy).expect("lib decoy")));
    assert!(!sources.contains(&canonical(&bin_decoy).expect("bin decoy")));
}

#[test]
fn path_overridden_modules_resolve_nested_paths_from_their_source_parent() {
    let root = tempfile::tempdir().expect("temporary path-override package");
    let source = root.path().join("src");
    let tree = source.join("tree");
    std::fs::create_dir_all(tree.join("outer")).expect("create real nested module directory");
    std::fs::create_dir_all(source.join("outer")).expect("create nested decoy directory");
    std::fs::write(
        root.path().join("Cargo.toml"),
        "[package]\nname='path-override-proof'\nversion='0.0.0'\nedition='2021'\n[lib]\npath='src/custom.rs'\n",
    )
    .expect("write path-override manifest");
    std::fs::write(
        source.join("custom.rs"),
        "#[path = \"tree/outer.rs\"] mod outer;",
    )
    .expect("write target root");
    std::fs::write(
        tree.join("outer.rs"),
        "#[path = \"outer/real.rs\"] mod real;",
    )
    .expect("write path-overridden module");
    let real = tree.join("outer/real.rs");
    let decoy = source.join("outer/real.rs");
    std::fs::write(&real, "pub fn real_owner() {}").expect("write real nested source");
    std::fs::write(&decoy, "pub fn decoy() {}").expect("write nested decoy source");

    let sources = production_rust_sources(root.path()).expect("discover path-overridden sources");
    assert!(sources.contains(&canonical(&real).expect("real nested source")));
    assert!(!sources.contains(&canonical(&decoy).expect("decoy nested source")));
}

#[test]
fn inline_modules_preserve_path_bases_across_source_styles_and_includes() {
    let root = tempfile::tempdir().expect("temporary inline-path package");
    let (actual, decoys) = write_inline_path_fixture(root.path());
    let sources = production_rust_sources(root.path()).expect("discover inline path sources");
    for path in actual {
        assert!(sources.contains(&canonical(&path).expect("actual inline source")));
    }
    for path in decoys {
        assert!(!sources.contains(&canonical(&path).expect("inline decoy source")));
    }
}

fn write_inline_path_fixture(root: &Path) -> ([PathBuf; 4], [PathBuf; 4]) {
    let source = root.join("src");
    for directory in [
        "inline",
        "non_mod/nested",
        "mod_style/nested",
        "included",
        "mod_style",
    ] {
        std::fs::create_dir_all(source.join(directory)).expect("create inline module directory");
    }
    std::fs::write(
        root.join("Cargo.toml"),
        "[package]\nname='inline-path-proof'\nversion='0.0.0'\nedition='2021'\n",
    )
    .expect("write inline-path manifest");
    std::fs::write(
        source.join("lib.rs"),
        "mod inline { #[path=\"root_actual.rs\"] mod hidden; }\nmod non_mod;\nmod mod_style;\nmod included { include!(\"included/fragment.rs\"); }\n",
    )
    .expect("write inline-path root");
    std::fs::write(
        source.join("non_mod.rs"),
        "mod nested { #[path=\"non_mod_actual.rs\"] mod hidden; }",
    )
    .expect("write non-mod-rs inline owner");
    std::fs::write(
        source.join("mod_style/mod.rs"),
        "mod nested { #[path=\"mod_actual.rs\"] mod hidden; }",
    )
    .expect("write mod-rs inline owner");
    std::fs::write(
        source.join("included/fragment.rs"),
        "#[path=\"included_actual.rs\"] mod hidden;",
    )
    .expect("write included inline fragment");
    let actual = [
        source.join("inline/root_actual.rs"),
        source.join("non_mod/nested/non_mod_actual.rs"),
        source.join("mod_style/nested/mod_actual.rs"),
        source.join("included/included_actual.rs"),
    ];
    let decoys = [
        source.join("root_actual.rs"),
        source.join("non_mod/non_mod_actual.rs"),
        source.join("mod_style/mod_actual.rs"),
        source.join("included_actual.rs"),
    ];
    for path in &actual {
        std::fs::write(path, "pub fn real_owner() {}").expect("write actual inline source");
    }
    for path in &decoys {
        std::fs::write(path, "pub fn decoy() {}").expect("write inline decoy source");
    }

    (actual, decoys)
}

#[test]
fn split_string_local_procedural_macro_fails_closed_before_expansion() {
    let root = tempfile::tempdir().expect("temporary proc-macro workspace");
    let macro_root = root.path().join("hidden-macro");
    std::fs::create_dir_all(root.path().join("src")).expect("create product source");
    std::fs::create_dir_all(macro_root.join("src")).expect("create proc-macro source");
    std::fs::write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers=['hidden-macro']\nresolver='2'\n[package]\nname='proc-macro-product'\nversion='0.0.0'\nedition='2021'\n[dependencies]\nhidden-macro={path='hidden-macro'}\n",
    )
    .expect("write proc-macro product manifest");
    std::fs::write(
        root.path().join("src/lib.rs"),
        "pub fn alternate() { hidden_macro::make_owner!(store, limits); }",
    )
    .expect("write proc-macro invocation");
    std::fs::write(
        macro_root.join("Cargo.toml"),
        "[package]\nname='hidden-macro'\nversion='0.0.0'\nedition='2021'\n[lib]\nproc-macro=true\n",
    )
    .expect("write proc-macro manifest");
    std::fs::write(
        macro_root.join("src/lib.rs"),
        "extern crate proc_macro; #[proc_macro] pub fn make_owner(_: proc_macro::TokenStream) -> proc_macro::TokenStream { let owner=[\"Physical\",\"Residency\",\"Pool\",\"Owner\"].concat(); let method=[\"op\",\"en\"].concat(); format!(\"{owner}::{method}(store, limits)\").parse().unwrap() }",
    )
    .expect("write split-string proc macro");

    let denial = production_rust_sources(root.path())
        .expect_err("an unreviewed production proc macro must fail closed");
    assert!(
        denial.contains("unreviewed procedural macro"),
        "wrong denial: {denial}"
    );
}

fn root_context(source: &Path) -> SourceContext {
    SourceContext {
        module_dir: source.to_path_buf(),
        path_attr_dir: source.to_path_buf(),
        include_dir: source.to_path_buf(),
    }
}
