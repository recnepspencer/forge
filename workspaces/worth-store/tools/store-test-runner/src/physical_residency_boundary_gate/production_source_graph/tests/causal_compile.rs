use super::super::*;

#[test]
fn included_fragments_resolve_modules_from_the_included_source_directory() {
    let root = tempfile::tempdir().expect("temporary included-fragment package");
    let source = root.path().join("src");
    let fragments = source.join("fragments");
    std::fs::create_dir_all(&fragments).expect("create fragment directory");
    std::fs::write(
        root.path().join("Cargo.toml"),
        "[package]\nname='included-module-proof'\nversion='0.0.0'\nedition='2021'\n",
    )
    .expect("write included-fragment manifest");
    std::fs::write(
        source.join("lib.rs"),
        "std::include!(\"fragments/root.rs\");\nmod external;\nmod inline { include!(\"fragments/inline.rs\"); }\n",
    )
    .expect("write included-fragment root");
    std::fs::write(
        source.join("external.rs"),
        "include!(\"fragments/external.rs\");",
    )
    .expect("write external module include");
    for owner in ["root", "external", "inline"] {
        std::fs::write(
            fragments.join(format!("{owner}.rs")),
            format!("#[path=\"{owner}_direct_actual.rs\"] mod direct; mod {owner}_conventional;"),
        )
        .expect("write included fragment");
        for name in [
            format!("{owner}_direct_actual.rs"),
            format!("{owner}_conventional.rs"),
        ] {
            std::fs::write(fragments.join(&name), "pub fn actual() {}")
                .expect("write actual included module");
            std::fs::write(source.join(&name), "compile_error!(\"decoy compiled\");")
                .expect("write included-module decoy");
        }
    }

    cargo_check(root.path());
    let sources = production_rust_sources(root.path()).expect("discover included modules");
    for owner in ["root", "external", "inline"] {
        for name in [
            format!("{owner}_direct_actual.rs"),
            format!("{owner}_conventional.rs"),
        ] {
            assert!(sources.contains(&canonical(&fragments.join(&name)).expect("actual source")));
            assert!(!sources.contains(&canonical(&source.join(&name)).expect("decoy source")));
        }
    }
}

#[test]
fn block_and_macro_generated_sources_have_causal_reachability_proof() {
    let root = tempfile::tempdir().expect("temporary block-module package");
    let source = root.path().join("src");
    std::fs::create_dir_all(&source).expect("create block-module source");
    std::fs::write(
        root.path().join("Cargo.toml"),
        "[package]\nname='block-module-proof'\nversion='0.0.0'\nedition='2021'\n",
    )
    .expect("write block-module manifest");
    let hidden = source.join("hidden.rs");
    std::fs::write(&hidden, "pub fn run() {} ").expect("write hidden source");
    std::fs::write(
        source.join("lib.rs"),
        "pub fn call() { #[path=\"hidden.rs\"] mod hidden; hidden::run(); }",
    )
    .expect("write block-local module declaration");
    cargo_check(root.path());
    let sources = production_rust_sources(root.path()).expect("discover block-local module");
    assert!(sources.contains(&canonical(&hidden).expect("block-local source")));

    for (source_text, expected) in [
        (
            "macro_rules! load_hidden { () => { mod hidden; } } load_hidden!();",
            "macro-generated module",
        ),
        (
            "macro_rules! load_hidden { () => { include!(\"hidden.rs\"); } } load_hidden!();",
            "macro-generated include",
        ),
        (
            "use std::include as load; load!(\"hidden.rs\");",
            "include macro import alias",
        ),
    ] {
        std::fs::write(source.join("lib.rs"), source_text).expect("write hostile macro source");
        cargo_check(root.path());
        let denial =
            production_rust_sources(root.path()).expect_err("macro source must fail closed");
        assert!(denial.contains(expected), "wrong macro denial: {denial}");
    }

    let syntax = syn::parse_file(
        "fn guarded() { #[cfg(test)] mod hidden; } #[cfg(test)] macro_rules! load_hidden { () => { include!(\"hidden.rs\"); } } #[cfg(test)] load_hidden!();",
    )
    .expect("parse test-only source expansion");
    let mut pending = Vec::new();
    collect_external_modules(&syntax.items, &super::root_context(&source), &mut pending)
        .expect("ignore test-only source expansion");
    assert!(pending.is_empty());
}

#[test]
fn raw_identifiers_preserve_constructor_and_include_provenance() {
    let root = tempfile::tempdir().expect("temporary raw-identifier package");
    let source = root.path().join("src");
    std::fs::create_dir_all(&source).expect("create raw-identifier source");
    std::fs::write(
        root.path().join("Cargo.toml"),
        "[package]\nname='raw-identifier-proof'\nversion='0.0.0'\nedition='2021'\n",
    )
    .expect("write raw-identifier manifest");
    std::fs::write(source.join("hidden.rs"), "pub fn hidden() {}")
        .expect("write raw-included source");
    let constructors = "pub struct PhysicalResidencyPoolOwner; impl PhysicalResidencyPoolOwner { pub fn open() {} } pub fn direct() { r#PhysicalResidencyPoolOwner::open(); PhysicalResidencyPoolOwner::r#open(); }";
    std::fs::write(
        source.join("lib.rs"),
        format!("r#include!(\"hidden.rs\"); {constructors}"),
    )
    .expect("write raw include and constructor calls");

    cargo_check(root.path());
    let sources = production_rust_sources(root.path()).expect("discover raw literal include");
    assert!(sources.contains(&canonical(&source.join("hidden.rs")).expect("raw include source")));
    let spec = crate::physical_residency_boundary_gate::constructor_syntax::ConstructorSpec {
        owner: "PhysicalResidencyPoolOwner",
        method: "open",
    };
    let inspect = crate::physical_residency_boundary_gate::constructor_syntax::constructor_calls;
    let calls = inspect(constructors, &[spec]).expect("inspect compiled raw constructor spelling");
    assert_eq!(calls.len(), 2);
    for hostile in [
        "use crate::r#PhysicalResidencyPoolOwner as PoolOwner; let _ = PoolOwner::open();",
        "type PoolOwner = r#PhysicalResidencyPoolOwner; let _ = PoolOwner::open();",
        "macro_rules! call { ($owner:ident) => { $owner::open() } } call!(r#PhysicalResidencyPoolOwner);",
    ] {
        inspect(hostile, &[spec]).expect_err("raw constructor indirection must fail closed");
    }
    inspect(
        "struct r#Unrelated; impl r#Unrelated { fn r#open() {} } fn control() { r#Unrelated::r#open(); }",
        &[spec],
    )
    .expect("unrelated raw identifiers must remain admitted");

    std::fs::write(
        source.join("lib.rs"),
        "use std::r#include as load; load!(\"hidden.rs\");",
    )
    .expect("write raw include alias");
    cargo_check(root.path());
    let denial = production_rust_sources(root.path()).expect_err("raw include alias must deny");
    assert!(
        denial.contains("include macro import alias"),
        "wrong denial: {denial}"
    );
}

#[test]
fn raw_keyword_identifiers_do_not_counterfeit_source_grammar() {
    let root = tempfile::tempdir().expect("temporary raw-keyword package");
    let source = root.path().join("src");
    std::fs::create_dir_all(&source).expect("create raw-keyword source");
    std::fs::write(
        root.path().join("Cargo.toml"),
        "[package]\nname='raw-keyword-proof'\nversion='0.0.0'\nedition='2021'\n",
    )
    .expect("write raw-keyword manifest");
    std::fs::write(
        source.join("lib.rs"),
        "macro_rules! value { () => {{ let r#mod = 1_u8; r#mod }} } macro_rules! r#macro_rules { () => { 2_u8 } } pub fn control() -> u8 { value!() + r#macro_rules!() }",
    )
    .expect("write raw-keyword macro controls");

    cargo_check(root.path());
    production_rust_sources(root.path()).expect("raw names are not source grammar keywords");
}

#[test]
fn external_dependency_include_alias_fails_closed_before_expansion() {
    let product = tempfile::tempdir().expect("temporary macro-alias product");
    let dependency = tempfile::tempdir().expect("temporary macro-alias dependency");
    std::fs::create_dir_all(product.path().join("src")).expect("create product source");
    std::fs::create_dir_all(dependency.path().join("src")).expect("create dependency source");
    let dependency_path = dependency.path().display().to_string().replace('\\', "/");
    std::fs::write(
        product.path().join("Cargo.toml"),
        format!(
            "[package]\nname='include-alias-product'\nversion='0.0.0'\nedition='2021'\n[dependencies]\ninclude-alias={{path='{dependency_path}'}}\n"
        ),
    )
    .expect("write product manifest");
    std::fs::write(
        product.path().join("src/lib.rs"),
        "pub struct PhysicalResidencyPoolOwner; impl PhysicalResidencyPoolOwner { pub fn open() {} } include_alias::load!(\"hidden.rs\");",
    )
    .expect("write product source");
    std::fs::write(
        product.path().join("src/hidden.rs"),
        "pub fn alternate() { PhysicalResidencyPoolOwner::open(); }",
    )
    .expect("write hidden constructor source");
    std::fs::write(
        dependency.path().join("Cargo.toml"),
        "[package]\nname='include-alias'\nversion='0.0.0'\nedition='2021'\n",
    )
    .expect("write dependency manifest");
    std::fs::write(
        dependency.path().join("src/lib.rs"),
        "pub use std::include as load;",
    )
    .expect("write external include alias");

    cargo_check(product.path());
    let denial = production_rust_sources(product.path())
        .expect_err("external dependency source expansion must fail closed");
    assert!(
        denial.contains("external declarative macro source")
            && denial.contains("include macro alias"),
        "wrong external macro denial: {denial}"
    );

    assert_generated_external_alias_denied(product.path(), dependency.path());
    assert_literal_external_aliases_denied(product.path(), dependency.path());
}

fn assert_generated_external_alias_denied(product: &Path, dependency: &Path) {
    std::fs::write(
        dependency.join("Cargo.toml"),
        "[package]\nname='include-alias'\nversion='0.0.0'\nedition='2021'\nbuild='build.rs'\n",
    )
    .expect("write generated dependency manifest");
    std::fs::write(
        dependency.join("build.rs"),
        r#"fn main() { let output = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("generated.rs"); std::fs::write(output, "pub use std::include as load;").unwrap(); }"#,
    )
    .expect("write dependency build script");
    std::fs::write(
        dependency.join("src/lib.rs"),
        "include!(concat!(env!(\"OUT_DIR\"), \"/generated.rs\"));",
    )
    .expect("write generated include owner");
    cargo_check(product);
    let denial =
        production_rust_sources(product).expect_err("external computed include must fail closed");
    assert!(
        denial.contains("external declarative macro source")
            && denial.contains("computed external include"),
        "wrong generated macro denial: {denial}"
    );
}

fn assert_literal_external_aliases_denied(product: &Path, dependency: &Path) {
    let in_package = dependency.join("src/generated.inc");
    std::fs::write(&in_package, "pub use std::include as load;")
        .expect("write arbitrary-extension include alias");
    std::fs::write(
        dependency.join("src/lib.rs"),
        "include!(\"generated.inc\");",
    )
    .expect("write arbitrary-extension literal include");
    cargo_check(product);
    let denial = production_rust_sources(product)
        .expect_err("arbitrary-extension external include must be traversed");
    assert!(
        denial.contains("include macro alias") && denial.contains("generated.inc"),
        "wrong arbitrary-extension denial: {denial}"
    );

    let shared = tempfile::tempdir().expect("temporary outside-root include");
    let outside = shared.path().join("generated.rs");
    std::fs::write(&outside, "pub use std::include as load;")
        .expect("write outside-root include alias");
    let outside = outside.display().to_string().replace('\\', "/");
    std::fs::write(
        dependency.join("src/lib.rs"),
        format!("include!(\"{outside}\");"),
    )
    .expect("write outside-root literal include");
    cargo_check(product);
    let denial = production_rust_sources(product)
        .expect_err("outside-root external include must be traversed");
    assert!(
        denial.contains("include macro alias") && denial.contains("generated.rs"),
        "wrong outside-root denial: {denial}"
    );
}

fn cargo_check(root: &Path) {
    let output = std::process::Command::new("cargo")
        .args(["check", "--quiet", "--manifest-path"])
        .arg(root.join("Cargo.toml"))
        .env("CARGO_TARGET_DIR", root.join("target"))
        .env_remove("RUSTFLAGS")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .output()
        .expect("run Cargo check for causal fixture");
    assert!(
        output.status.success(),
        "causal fixture failed to compile: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
