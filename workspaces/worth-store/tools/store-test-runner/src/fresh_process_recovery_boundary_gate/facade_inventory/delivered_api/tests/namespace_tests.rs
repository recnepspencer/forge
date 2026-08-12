use super::{derive_family_at, fixture_family, Fixture};

#[test]
fn public_module_namespaces_recursively_expose_qualified_children() {
    let fixture = Fixture::new("public-namespace");
    fixture.write(
        "allowed.rs",
        "pub struct Nested; impl Nested { pub fn method(&self) {} }\n",
    );
    fixture.write("private.rs", "pub struct Hidden;\n");
    fixture.write(
        "lib.rs",
        concat!(
            "pub mod allowed; mod private;\n",
            "pub mod inline { pub fn nested() {} }\n",
        ),
    );
    let delivered = derive_family_at(&fixture.root, &fixture_family()).expect("public modules");
    for surface in [
        "allowed",
        "allowed::Nested",
        "allowed::Nested::method",
        "inline",
        "inline::nested",
    ] {
        assert!(delivered.iter().any(|(actual, _)| actual == surface));
    }
    assert!(!delivered
        .iter()
        .any(|(surface, _)| surface.contains("Hidden")));

    fixture.write("allowed.rs", "pub struct Nested; pub struct Added;\n");
    let mutated = derive_family_at(&fixture.root, &fixture_family()).expect("nested addition");
    assert!(mutated
        .iter()
        .any(|(surface, _)| surface == "allowed::Added"));
}

#[test]
fn module_reexport_aliases_and_configuration_variants_fail_closed() {
    let fixture = Fixture::new("module-alias");
    fixture.write(
        "lib.rs",
        concat!(
            "pub mod a { pub struct A; }\n",
            "pub mod b { pub struct B; }\n",
            "pub use a as alias;\n",
        ),
    );
    let from_a = derive_family_at(&fixture.root, &fixture_family())
        .expect_err("module a alias must fail closed");
    assert!(from_a.contains("module re-export alias"));

    fixture.write(
        "lib.rs",
        concat!(
            "pub mod a { pub struct A; }\n",
            "pub mod b { pub struct B; }\n",
            "pub use b as alias;\n",
        ),
    );
    let from_b = derive_family_at(&fixture.root, &fixture_family())
        .expect_err("module b alias must fail closed");
    assert!(from_b.contains("module re-export alias"));

    fixture.write("a_unix.rs", "pub struct Unix;\n");
    fixture.write("a_windows.rs", "pub struct Windows;\n");
    fixture.write(
        "lib.rs",
        concat!(
            "#[cfg(unix)] #[path = \"a_unix.rs\"] pub mod a;\n",
            "#[cfg(windows)] #[path = \"a_windows.rs\"] pub mod a;\n",
            "pub use a as alias;\n",
        ),
    );
    let variants = derive_family_at(&fixture.root, &fixture_family())
        .expect_err("configuration-varied module alias must fail closed");
    assert!(variants.contains("module re-export alias"));
}

#[test]
fn grouped_self_module_aliases_fail_closed_at_local_crate_and_parent_paths() {
    let fixture = Fixture::new("grouped-self-module-alias");
    for target in ["a", "b"] {
        fixture.write(
            "lib.rs",
            &format!(
                "pub mod a {{ pub struct A; }}\npub mod b {{ pub struct B; }}\npub use {target}::{{self as alias}};\n"
            ),
        );
        let denial = derive_family_at(&fixture.root, &fixture_family())
            .expect_err("grouped local self alias must fail closed");
        assert!(denial.contains("module re-export alias"));
    }

    fixture.write(
        "lib.rs",
        "pub mod a { pub struct A; }\npub use crate::a::{self as alias};\n",
    );
    let crate_denial = derive_family_at(&fixture.root, &fixture_family())
        .expect_err("crate-qualified grouped self alias must fail closed");
    assert!(crate_denial.contains("module re-export alias"));

    fixture.write(
        "lib.rs",
        concat!(
            "pub mod a { pub struct A; }\n",
            "pub mod nested { pub use super::a::{self as alias}; }\n",
        ),
    );
    let parent_denial = derive_family_at(&fixture.root, &fixture_family())
        .expect_err("parent-qualified grouped self alias must fail closed");
    assert!(parent_denial.contains("module re-export alias"));

    fixture.write(
        "lib.rs",
        concat!(
            "pub struct Root;\n",
            "pub mod child { pub struct Child; }\n",
            "pub use crate as alias;\n",
        ),
    );
    let root_denial = derive_family_at(&fixture.root, &fixture_family())
        .expect_err("current-crate root namespace alias must fail closed");
    assert!(root_denial.contains("module re-export alias"));
}

#[test]
fn module_alias_cycles_and_repeated_aliases_fail_before_recursive_projection() {
    let fixture = Fixture::new("module-alias-cycles");
    fixture.write(
        "lib.rs",
        "pub mod a { pub struct A; pub use crate::a as again; }\n",
    );
    let self_cycle = derive_family_at(&fixture.root, &fixture_family())
        .expect_err("self module alias cycle must fail");
    assert!(self_cycle.contains("module re-export alias"));

    fixture.write(
        "lib.rs",
        concat!(
            "pub mod a { pub use crate::b; }\n",
            "pub mod b { pub use crate::a; }\n",
        ),
    );
    let two_module = derive_family_at(&fixture.root, &fixture_family())
        .expect_err("two-module alias cycle must fail");
    assert!(two_module.contains("module re-export alias"));

    fixture.write(
        "lib.rs",
        concat!(
            "pub mod a { pub struct A; }\n",
            "pub use a as first; pub use a as second;\n",
        ),
    );
    let repeated = derive_family_at(&fixture.root, &fixture_family())
        .expect_err("repeated aliases must fail closed before projection");
    assert!(repeated.contains("module re-export alias"));
}

#[test]
fn external_module_alias_hostile_twins_fail_closed_through_the_real_resolver() {
    let fixture = Fixture::new("external-module-alias");
    fixture.write(
        "workspaces/worth-store/crates/worth-store-wal/src/lib.rs",
        concat!(
            "pub mod artifact_store { pub struct Artifact; }\n",
            "pub mod checkpoint { pub struct Checkpoint; }\n",
        ),
    );
    for target in ["artifact_store", "checkpoint"] {
        fixture.write(
            "lib.rs",
            &format!("pub use worth_store_wal::{target} as wal_namespace;\n"),
        );
        let denial = derive_family_at(&fixture.root, &fixture_family())
            .expect_err("cross-family module alias must fail closed");
        assert!(denial.contains("external module re-export alias"));
    }
}

#[test]
fn external_glob_reexports_fail_closed_before_an_opaque_wildcard_is_recorded() {
    let fixture = Fixture::new("external-glob");
    fixture.write(
        "workspaces/worth-store/crates/worth-store-wal/src/lib.rs",
        concat!(
            "pub mod artifact_store { pub struct Artifact; }\n",
            "pub mod checkpoint { pub struct Checkpoint; }\n",
        ),
    );
    fixture.write("lib.rs", "pub use worth_store_wal::*;\n");

    let denial = derive_family_at(&fixture.root, &fixture_family())
        .expect_err("external glob must fail before wildcard evidence is recorded");
    assert!(denial.contains("external or unresolved glob re-export"));
}

#[test]
fn contractual_dependency_crate_namespace_aliases_fail_closed() {
    let fixture = Fixture::new("external-crate-alias");
    fixture.write(
        "workspaces/worth-store/crates/worth-store-wal/src/lib.rs",
        "pub mod artifact_store { pub struct Artifact; }\n",
    );
    for (facade, expected) in [
        (
            "pub use worth_store_wal as wal_namespace;\n",
            "dependency crate namespace re-export",
        ),
        (
            "pub use worth_store_wal::{self as wal_namespace};\n",
            "module re-export alias",
        ),
        (
            "pub use ::worth_store_wal as wal_namespace;\n",
            "dependency crate namespace re-export",
        ),
    ] {
        fixture.write("lib.rs", facade);
        let denial = derive_family_at(&fixture.root, &fixture_family())
            .expect_err("dependency crate namespace alias must fail closed");
        assert!(denial.contains(expected));
    }
}

#[test]
fn supported_visibility_variants_are_order_independent_and_recursive() {
    let fixture = Fixture::new("visibility-variants");
    fixture.write("platform.rs", "pub struct Nested;\n");
    for declarations in [
        "#[cfg(unix)] mod platform; #[cfg(windows)] pub mod platform;\n",
        "#[cfg(windows)] pub mod platform; #[cfg(unix)] mod platform;\n",
    ] {
        fixture.write("lib.rs", declarations);
        let delivered = derive_family_at(&fixture.root, &fixture_family()).expect("cfg variants");
        for surface in ["platform", "platform::Nested"] {
            assert!(delivered.iter().any(|(actual, _)| actual == surface));
        }
    }

    fixture.write("parent.rs", "pub mod child { pub struct DeeplyNested; }\n");
    fixture.write(
        "lib.rs",
        "#[cfg(unix)] mod parent; #[cfg(windows)] pub mod parent;\n",
    );
    let nested = derive_family_at(&fixture.root, &fixture_family()).expect("nested cfg variant");
    assert!(nested
        .iter()
        .any(|(surface, _)| surface == "parent::child::DeeplyNested"));
}

#[test]
fn public_namespace_and_exported_type_macros_fail_closed() {
    let fixture = Fixture::new("namespace-macros");
    fixture.write(
        "exposed.rs",
        concat!(
            "macro_rules! generate { () => { pub struct Undocumented; } }\n",
            "generate!();\n",
        ),
    );
    fixture.write(
        "private.rs",
        concat!(
            "macro_rules! generate { () => { pub struct Hidden; } }\n",
            "generate!();\n",
        ),
    );
    fixture.write("lib.rs", "pub mod exposed; mod private;\n");
    let denial = derive_family_at(&fixture.root, &fixture_family()).expect_err("public macro");
    assert!(denial.contains("public expansion is not provable"));

    fixture.write("lib.rs", "mod private;\n");
    let private = derive_family_at(&fixture.root, &fixture_family()).expect("private module");
    assert!(!private
        .iter()
        .any(|(surface, _)| surface.contains("Hidden")));

    fixture.write(
        "exposed.rs",
        concat!(
            "pub struct Exported;\n",
            "macro_rules! methods { () => { pub fn undocumented(&self) {} } }\n",
            "impl Exported { methods!(); }\n",
        ),
    );
    fixture.write("lib.rs", "pub mod exposed; pub use exposed::Exported;\n");
    let associated =
        derive_family_at(&fixture.root, &fixture_family()).expect_err("associated macro");
    assert!(associated.contains("exported type"));
}

#[test]
fn exact_counter_accessor_expansion_is_proved_and_mutations_fail_closed() {
    let fixture = Fixture::new("associated-counter-macro");
    fixture.write(
        "exposed.rs",
        concat!(
            "pub struct Counters { value: u64 }\n",
            "macro_rules! accessors {\n",
            "  ($($name:ident),+ $(,)?) => {$(\n",
            "    pub const fn $name(self) -> u64 { self.$name }\n",
            "  )+};\n",
            "}\n",
            "impl Counters { accessors!(value); }\n",
        ),
    );
    fixture.write("lib.rs", "pub mod exposed; pub use exposed::Counters;\n");
    let delivered = derive_family_at(&fixture.root, &fixture_family()).expect("exact expansion");
    for surface in ["Counters::value", "exposed::Counters::value"] {
        assert!(delivered.iter().any(|(actual, _)| actual == surface));
    }

    fixture.write(
        "exposed.rs",
        concat!(
            "pub struct Counters { value: u64 }\n",
            "macro_rules! accessors {\n",
            "  ($($name:ident),+ $(,)?) => {$(\n",
            "    pub const fn $name(self) -> u64 { self.$name }\n",
            "    pub const fn extra(self) -> u64 { 0 }\n",
            "  )+};\n",
            "}\n",
            "impl Counters { accessors!(value); }\n",
        ),
    );
    let denial = derive_family_at(&fixture.root, &fixture_family()).expect_err("changed expansion");
    assert!(denial.contains("public expansion is not provable"));

    fixture.write(
        "exposed.rs",
        concat!(
            "pub struct Counters { value: u64 }\n",
            "macro_rules! accessors {\n",
            "  ($($name:ident),+ $(,)?) => {$(\n",
            "    pub const fn $name(self) -> u64 { self.$name }\n",
            "  )+};\n",
            "}\n",
            "mod other {\n",
            "  macro_rules! accessors { ($name:ident) => {\n",
            "    pub const fn $name(self) -> u64 { self.$name }\n",
            "    pub const fn extra(self) -> u64 { 0 }\n",
            "  }; }\n",
            "  pub(crate) use accessors;\n",
            "}\n",
            "impl Counters { other::accessors!(value); }\n",
        ),
    );
    let qualified =
        derive_family_at(&fixture.root, &fixture_family()).expect_err("qualified same leaf");
    assert!(qualified.contains("public expansion is not provable"));
}
