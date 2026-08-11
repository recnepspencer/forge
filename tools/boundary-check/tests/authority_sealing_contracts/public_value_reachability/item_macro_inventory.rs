use super::AuthoritySealingTestRepository;
use super::{assert_contract_allowed, assert_contract_denied, value_row};

#[test]
fn macro_generated_sealed_export_fails_closed() {
    assert_contract_denied(
        "macro-generated-sealed",
        r#"
macro_rules! declare_sealed {
    ($name:ident) => { pub struct $name { value: u8 } };
}
pub(crate) use declare_sealed;
crate::declare_sealed!(Sealed);
"#,
        "",
        "",
        "",
    );
}

#[test]
fn nested_repetition_that_only_emits_impls_is_mechanically_safe() {
    assert_contract_allowed(
        "macro-impl-only",
        r#"
pub struct Sealed { value: u8 }
pub fn issue() -> Sealed { Sealed { value: 1 } }
macro_rules! implement_debug {
    ($($name:ident),+) => {$(
        impl core::fmt::Debug for $name {
            fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str(stringify!($name))
            }
        }
    )+};
}
pub(crate) use implement_debug;
crate::implement_debug!(Sealed);
"#,
        "pub(crate) fn sealed()->worth_proof::Sealed{worth_proof::issue()}",
        value_row(),
        "",
    );
}

#[test]
fn proc_macro_attribute_generated_export_fails_closed() {
    let repository = proc_macro_repository(
        "proc-attribute-export",
        "#[fixture_macros::export_sealed] pub struct Anchor;",
    );
    let (ok, output) = repository.run_boundary_check();
    repository.cleanup();
    assert!(!ok, "proc-macro generated export unexpectedly passed");
    assert!(
        output.contains("not an exact compiler-owned inert attribute"),
        "unexpected diagnostic:\n{output}"
    );
}

#[test]
fn proc_macro_derive_generated_export_fails_closed() {
    let repository = proc_macro_repository(
        "proc-derive-export",
        "#[derive(fixture_macros::GenerateSealed)] pub struct Anchor;",
    );
    let (ok, output) = repository.run_boundary_check();
    repository.cleanup();
    assert!(
        !ok,
        "proc-macro derive generated export unexpectedly passed"
    );
    assert!(
        output.contains("is not an exact compiler built-in"),
        "unexpected diagnostic:\n{output}"
    );
}

#[test]
fn imported_same_named_proc_derive_cannot_pose_as_compiler_builtin() {
    let repository = proc_macro_repository(
        "proc-derive-shadow",
        "use fixture_macros::GenerateSealed as Debug; #[derive(Debug)] pub struct Anchor;",
    );
    let (ok, output) = repository.run_boundary_check();
    repository.cleanup();
    assert!(!ok, "imported proc derive unexpectedly passed by name");
    assert!(
        output.contains("explicit import binding"),
        "unexpected diagnostic:\n{output}"
    );
}

#[test]
fn transitive_proc_derive_alias_cannot_pose_as_compiler_builtin() {
    let repository = proc_macro_repository(
        "proc-derive-transitive-shadow",
        r#"
mod first { pub use fixture_macros::GenerateSealed as Hidden; }
mod second { pub use crate::first::Hidden as Further; }
use crate::second::Further as Debug;
#[derive(Debug)]
pub struct Anchor;
"#,
    );
    let (ok, output) = repository.run_boundary_check();
    repository.cleanup();
    assert!(!ok, "transitively imported proc derive unexpectedly passed");
    assert!(
        output.contains("fixture_macros::GenerateSealed"),
        "diagnostic omitted the resolved derive origin:\n{output}"
    );
}

#[test]
fn compiler_builtin_derive_remains_legal() {
    assert_contract_allowed(
        "builtin-derive",
        "#[derive(Debug)] pub struct Sealed { value: u8 } pub fn issue() -> Sealed { Sealed { value: 1 } }",
        "pub(crate) fn sealed()->worth_proof::Sealed{worth_proof::issue()}",
        value_row(),
        "",
    );
}

#[test]
fn unrelated_local_alias_does_not_block_compiler_builtin_derive() {
    assert_contract_allowed(
        "builtin-derive-with-unrelated-alias",
        r#"
mod local { pub struct Other; }
use crate::local::Other as Hidden;
#[derive(Debug)]
pub struct Sealed { value: u8 }
pub fn issue() -> Sealed { let _ = core::mem::size_of::<Hidden>(); Sealed { value: 1 } }
"#,
        "pub(crate) fn sealed()->worth_proof::Sealed{worth_proof::issue()}",
        value_row(),
        "",
    );
}

fn proc_macro_repository(label: &str, source: &str) -> AuthoritySealingTestRepository {
    let repository = AuthoritySealingTestRepository::create(label);
    repository.assemble_public_value_witness_contract(source, "", "", "");
    repository.write_file(
        "fixture-macros/Cargo.toml",
        r#"[package]
name = "fixture-macros"
version = "0.1.0"
edition = "2021"
[lib]
proc-macro = true
[workspace]
"#,
    );
    repository.write_file(
        "fixture-macros/src/lib.rs",
        r#"extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn export_sealed(_attribute: TokenStream, item: TokenStream) -> TokenStream {
    format!("{item} pub struct GeneratedSealed {{ value: u8 }}")
        .parse()
        .expect("valid generated item")
}

#[proc_macro_derive(GenerateSealed)]
pub fn generate_sealed(_item: TokenStream) -> TokenStream {
    "pub struct GeneratedSealed { value: u8 }"
        .parse()
        .expect("valid generated item")
}
"#,
    );
    repository.write_file(
        "crates/worth-proof/Cargo.toml",
        r#"[package]
name = "worth-proof"
version = "0.1.0"
edition = "2021"
[dependencies]
fixture-macros = { path = "../../fixture-macros" }
[workspace]
"#,
    );
    repository
}
