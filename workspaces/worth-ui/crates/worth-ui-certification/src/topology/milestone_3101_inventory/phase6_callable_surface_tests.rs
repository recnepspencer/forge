use std::collections::BTreeSet;
use std::path::Path;

use crate::topology::WorkspaceSourceInventory;

use super::{
    audit, audit_one_ordinary_mounted_entry, collect_file_callables, ledger,
    reject_forbidden_symbols_in_source, source_calls_method, Callable,
};

fn workspace_inventory() -> WorkspaceSourceInventory {
    WorkspaceSourceInventory::capture(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("crate parent")
            .parent()
            .expect("workspace root"),
    )
}

fn callable_manifest() -> toml::Value {
    let repository_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .and_then(Path::parent)
        .and_then(Path::parent)
        .expect("repository root");
    ledger::load(
        &repository_root.join("_docs/worth-ui/milestone-3.10.1-phase-6-callable-surface.toml"),
    )
    .expect("Phase 6 callable manifest")
}

fn owners(entries: &[(&'static str, &str)]) -> BTreeSet<(&'static str, String)> {
    entries
        .iter()
        .map(|(kind, owner)| (*kind, (*owner).to_owned()))
        .collect()
}

fn collect(source: &str, entries: &[(&'static str, &str)]) -> Result<BTreeSet<Callable>, String> {
    let syntax = syn::parse_file(source).expect("hostile fixture parses");
    let mut actual = BTreeSet::new();
    collect_file_callables(&syntax, "fixture.rs", &owners(entries), &mut actual)?;
    Ok(actual)
}

#[test]
fn phase6_real_callable_manifest_is_exact() {
    audit(&workspace_inventory(), &callable_manifest()).expect("Phase 6 callable manifest");
}

#[test]
fn phase6_collects_renamed_forwarding_wrappers_as_surface_growth() {
    let actual = collect(
        r#"
        pub struct WorthUiActiveApplicationSession;
        impl WorthUiActiveApplicationSession {
            pub fn renamed_midpoint(&mut self) { self.internal_midpoint(); }
            fn internal_midpoint(&mut self) {}
        }
        "#,
        &[("inherent", "WorthUiActiveApplicationSession")],
    )
    .expect("fixture should collect");
    assert!(actual
        .iter()
        .any(|callable| callable.method == "renamed_midpoint"));
}

#[test]
fn phase6_rejects_feature_gated_public_inherent_routes() {
    let error = collect(
        r#"
        pub struct WorthUiActiveApplicationSession;
        impl WorthUiActiveApplicationSession {
            #[cfg(feature = "certification-support")]
            pub fn execute_framework_turn(&mut self) {}
        }
        "#,
        &[("inherent", "WorthUiActiveApplicationSession")],
    )
    .expect_err("feature-gated public route should fail");
    assert!(error.contains("may not be feature- or test-gated"));
}

#[test]
fn phase6_one_entry_check_rejects_a_second_execute_route() {
    let actual = collect(
        r#"
        pub struct WorthUiActiveApplicationSession;
        impl WorthUiActiveApplicationSession {
            pub fn execute_mounted_frame(&mut self) {}
            pub fn execute_renamed_midpoint(&mut self) {}
        }
        "#,
        &[("inherent", "WorthUiActiveApplicationSession")],
    )
    .expect("fixture should collect");
    let error =
        audit_one_ordinary_mounted_entry(&actual).expect_err("a second execute route should fail");
    assert!(error.contains("exactly one ordinary execute entry"));
}

#[test]
fn phase6_collects_extension_trait_methods_exactly() {
    let actual = collect(
        r#"
        pub trait WorthUiFrameworkTurnCertificationExt {
            fn execute_framework_turn(&mut self);
        }
        "#,
        &[("extension_trait", "WorthUiFrameworkTurnCertificationExt")],
    )
    .expect("fixture should collect");
    assert!(actual
        .iter()
        .any(|callable| callable.method == "execute_framework_turn"));
}

#[test]
fn phase6_rejects_public_predecessor_type_aliases() {
    let forbidden = ["WorthUiBuilder"].into_iter().collect();
    let error = reject_forbidden_symbols_in_source(
        Path::new("fixture.rs"),
        "pub type WorthUiBuilder = WorthUiApplicationBuilder;",
        &forbidden,
    )
    .expect_err("public predecessor alias should fail");
    assert!(error.contains("WorthUiBuilder"));
}

#[test]
fn phase6_rejects_public_predecessor_reexport_aliases() {
    let forbidden = ["WorthUiBuilder"].into_iter().collect();
    let error = reject_forbidden_symbols_in_source(
        Path::new("fixture.rs"),
        "pub use crate::WorthUiApplicationBuilder as WorthUiBuilder;",
        &forbidden,
    )
    .expect_err("public predecessor reexport alias should fail");
    assert!(error.contains("WorthUiBuilder"));
}

#[test]
fn phase6_rejects_cfg_gated_impl_routes() {
    let error = collect(
        r#"
        pub struct WorthUiActiveApplicationSession;
        #[cfg(feature = "certification-support")]
        impl WorthUiActiveApplicationSession {
            pub fn execute_framework_turn(&mut self) {}
        }
        "#,
        &[("inherent", "WorthUiActiveApplicationSession")],
    )
    .expect_err("cfg-gated impl route should fail");
    assert!(error.contains("may not be feature- or test-gated"));
}

#[test]
fn phase6_rejects_cfg_gated_inline_module_routes() {
    let error = collect(
        r#"
        pub struct WorthUiActiveApplicationSession;
        #[cfg(feature = "certification-support")]
        mod restored_route {
            impl super::WorthUiActiveApplicationSession {
                pub fn execute_framework_turn(&mut self) {}
            }
        }
        "#,
        &[("inherent", "WorthUiActiveApplicationSession")],
    )
    .expect_err("cfg-gated inline module route should fail");
    assert!(error.contains("may not be feature- or test-gated"));
}

#[test]
fn phase6_caller_evidence_requires_a_real_call_expression() {
    assert!(!source_calls_method(
        "// execute_mounted_frame() is intentionally not called",
        "execute_mounted_frame",
    )
    .expect("comment-only fixture parses"));
    assert!(!source_calls_method(
        "const CLAIM: &str = \"execute_mounted_frame()\";",
        "execute_mounted_frame",
    )
    .expect("string-only fixture parses"));
    assert!(source_calls_method(
        "fn proof(session: &mut Session) { session.execute_mounted_frame(); }",
        "execute_mounted_frame",
    )
    .expect("call fixture parses"));
    assert!(source_calls_method(
        "fn proof(session: &mut Session) { assert!(session.execute_mounted_frame()); }",
        "execute_mounted_frame",
    )
    .expect("macro call fixture parses"));
}
