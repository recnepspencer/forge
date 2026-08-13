use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use super::{derive_family_at, facade_exports, FacadeFamily};

mod namespace_tests;
mod private_import_tests;

static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);

#[test]
fn export_resolution_retains_module_provenance_and_detects_omission() {
    let fixture = Fixture::new("path-provenance");
    fixture.write("allowed.rs", "pub struct Twin;\n");
    fixture.write("outside.rs", "pub struct Twin;\n");

    fixture.write(
        "lib.rs",
        "mod allowed; mod outside; pub use allowed::Twin;\n",
    );
    assert_eq!(
        resolved_relative_path(&fixture),
        Some("allowed.rs".to_owned())
    );

    fixture.write(
        "lib.rs",
        "mod allowed; mod outside; pub use outside::Twin as AllowedTwin;\n",
    );
    assert_eq!(
        resolved_relative_path(&fixture),
        Some("outside.rs".to_owned())
    );

    fixture.write(
        "lib.rs",
        "mod allowed; mod outside; pub use allowed::Missing;\n",
    );
    assert_eq!(resolved_relative_path(&fixture), None);
}

#[test]
fn full_facade_includes_outside_aliases_and_globbed_surfaces() {
    let fixture = Fixture::new("complete-facade");
    fixture.write("allowed.rs", "pub struct Twin; pub struct Globbed;\n");
    fixture.write("outside.rs", "pub struct Extra;\n");
    fixture.write(
        "lib.rs",
        concat!(
            "pub mod allowed; pub mod outside;\n",
            "pub use allowed::Twin;\n",
            "pub use outside::Extra as Alias;\n",
            "pub use allowed::*;\n",
        ),
    );
    let delivered = derive_family_at(&fixture.root, &fixture_family()).expect("derive facade");

    assert!(delivered.contains(&("Alias".to_owned(), "fixture/outside".to_owned())));
    assert!(delivered.contains(&("Globbed".to_owned(), "fixture/allowed".to_owned())));
}

#[test]
fn associated_items_bind_to_canonical_type_and_structured_cfg() {
    let fixture = Fixture::new("canonical-associated");
    fixture.write(
        "allowed.rs",
        concat!(
            "pub struct Twin;\n",
            "impl Twin { pub fn expected(&self) {} }\n",
        ),
    );
    fixture.write(
        "outside.rs",
        concat!(
            "pub struct Twin;\n",
            "impl crate::outside::Twin { pub fn misrouted(&self) {} }\n",
        ),
    );
    fixture.write(
        "impls.rs",
        concat!(
            "use crate::allowed::Twin;\n",
            "impl Twin {\n",
            "  pub fn cross_owner(&self) {}\n",
            "  #[cfg(any(feature = \"certification-test-authority\", target_os = \"windows\"))]\n",
            "  pub fn supported(&self) {}\n",
            "  #[cfg(all(feature = \"certification-test-authority\", target_os = \"windows\"))]\n",
            "  pub fn certification_only(&self) {}\n",
            "}\n",
        ),
    );
    fixture.write(
        "lib.rs",
        "pub mod allowed; pub mod outside; mod impls; pub use allowed::Twin;\n",
    );
    let delivered = derive_family_at(&fixture.root, &fixture_family()).expect("derive facade");

    for surface in ["Twin::expected", "Twin::cross_owner", "Twin::supported"] {
        assert!(delivered.iter().any(|(actual, _)| actual == surface));
    }
    assert!(!delivered.iter().any(|(surface, _)| {
        matches!(
            surface.as_str(),
            "Twin::misrouted" | "Twin::certification_only"
        )
    }));
}

#[test]
fn same_leaf_method_misrouting_cannot_replace_the_exported_method() {
    let fixture = Fixture::new("associated-misrouting");
    fixture.write("allowed.rs", "pub struct Twin;\n");
    fixture.write(
        "outside.rs",
        "pub struct Twin; impl crate::outside::Twin { pub fn expected(&self) {} }\n",
    );
    fixture.write(
        "lib.rs",
        "pub mod allowed; pub mod outside; pub use allowed::Twin;\n",
    );
    let delivered = derive_family_at(&fixture.root, &fixture_family()).expect("derive facade");
    assert!(!delivered
        .iter()
        .any(|(surface, _)| surface == "Twin::expected"));
}

#[test]
fn production_module_graph_rejects_orphans_disabled_modules_and_path_remapping() {
    let fixture = Fixture::new("module-membership");
    fixture.write("allowed.rs", "pub struct Twin;\n");
    fixture.write(
        "impls.rs",
        "use crate::allowed::Twin; impl Twin { pub fn expected(&self) {} }\n",
    );
    fixture.write(
        "outside.rs",
        "pub struct Twin; impl Twin { pub fn remapped(&self) {} }\n",
    );

    fixture.write("lib.rs", "mod allowed; pub use allowed::Twin;\n");
    let orphaned = derive_family_at(&fixture.root, &fixture_family()).expect("orphan fixture");
    assert!(!orphaned
        .iter()
        .any(|(surface, _)| surface == "Twin::expected"));

    fixture.write(
        "lib.rs",
        "mod allowed; #[cfg(any())] mod impls; pub use allowed::Twin;\n",
    );
    let disabled = derive_family_at(&fixture.root, &fixture_family()).expect("disabled fixture");
    assert!(!disabled
        .iter()
        .any(|(surface, _)| surface == "Twin::expected"));

    fixture.write(
        "lib.rs",
        "#[path = \"outside.rs\"] mod allowed; pub use allowed::Twin;\n",
    );
    let remapped = derive_family_at(&fixture.root, &fixture_family()).expect("remapped fixture");
    assert!(remapped
        .iter()
        .any(|(surface, owner)| { surface == "Twin::remapped" && owner == "fixture/outside" }));
    assert!(!remapped
        .iter()
        .any(|(surface, _)| surface == "Twin::expected"));
}

#[test]
fn inline_production_modules_supply_reachable_types_and_methods() {
    let fixture = Fixture::new("inline-module");
    fixture.write(
        "lib.rs",
        concat!(
            "pub mod inline {\n",
            "  pub struct Inner;\n",
            "  impl Inner { pub fn reachable(&self) {} }\n",
            "}\n",
            "pub use inline::Inner;\n",
        ),
    );
    let delivered = derive_family_at(&fixture.root, &fixture_family()).expect("inline fixture");
    assert!(delivered
        .iter()
        .any(|(surface, owner)| { surface == "Inner::reachable" && owner == "fixture/lib" }));
}

#[test]
fn supported_module_variants_and_cfg_attr_paths_are_unioned() {
    let fixture = Fixture::new("module-variants");
    fixture.write(
        "platform_unix.rs",
        "pub struct Twin; impl Twin { pub fn unix_only(&self) {} }\n",
    );
    fixture.write(
        "platform_windows.rs",
        "pub struct Twin; impl Twin { pub fn windows_only(&self) {} }\n",
    );
    fixture.write(
        "lib.rs",
        concat!(
            "#[cfg(unix)] #[path = \"platform_unix.rs\"] mod platform;\n",
            "#[cfg(windows)] #[path = \"platform_windows.rs\"] mod platform;\n",
            "pub use platform::Twin;\n",
        ),
    );
    let variants = derive_family_at(&fixture.root, &fixture_family()).expect("variant fixture");
    for surface in ["Twin::unix_only", "Twin::windows_only"] {
        assert!(variants.iter().any(|(actual, _)| actual == surface));
    }

    fixture.write(
        "lib.rs",
        concat!(
            "#[cfg_attr(unix, path = \"platform_unix.rs\")]\n",
            "#[cfg_attr(windows, path = \"platform_windows.rs\")]\n",
            "mod platform; pub use platform::Twin;\n",
        ),
    );
    let cfg_attr = derive_family_at(&fixture.root, &fixture_family()).expect("cfg_attr fixture");
    for surface in ["Twin::unix_only", "Twin::windows_only"] {
        assert!(cfg_attr.iter().any(|(actual, _)| actual == surface));
    }
}

#[test]
fn same_named_inline_cfg_variants_retain_each_syntax_occurrence() {
    let fixture = Fixture::new("inline-variants");
    fixture.write(
        "lib.rs",
        concat!(
            "#[cfg(unix)] mod platform {\n",
            "  pub struct Twin; impl Twin { pub fn unix_only(&self) {} }\n",
            "}\n",
            "#[cfg(windows)] mod platform {\n",
            "  pub struct Twin; impl Twin { pub fn windows_only(&self) {} }\n",
            "}\n",
            "pub use platform::Twin;\n",
        ),
    );
    let delivered = derive_family_at(&fixture.root, &fixture_family()).expect("inline variants");
    for surface in ["Twin::unix_only", "Twin::windows_only"] {
        assert!(delivered.iter().any(|(actual, _)| actual == surface));
    }
}

#[test]
fn direct_static_and_union_exports_are_visible_and_macros_fail_closed() {
    let fixture = Fixture::new("direct-kinds");
    fixture.write(
        "lib.rs",
        "pub static EXTRA: u8 = 0; pub union Extra { pub value: u8 }\n",
    );
    let direct = derive_family_at(&fixture.root, &fixture_family()).expect("direct kinds");
    for surface in ["EXTRA", "Extra"] {
        assert!(direct.iter().any(|(actual, _)| actual == surface));
    }

    fixture.write(
        "lib.rs",
        "macro_rules! generate_public_surface { () => { pub fn generated() {} } }\n",
    );
    assert!(derive_family_at(&fixture.root, &fixture_family())
        .expect("private macro definition")
        .is_empty());
    fixture.write(
        "lib.rs",
        concat!(
            "macro_rules! generate_public_surface { () => { pub fn generated() {} } }\n",
            "generate_public_surface!();\n",
        ),
    );
    let denial = derive_family_at(&fixture.root, &fixture_family()).expect_err("macro must fail");
    assert!(denial.contains("public expansion is not provable"));

    fixture.write(
        "lib.rs",
        "#[macro_export] macro_rules! public_macro { () => {} }\n",
    );
    let exported = derive_family_at(&fixture.root, &fixture_family()).expect_err("macro export");
    assert!(exported.contains("public expansion is not provable"));

    fixture.write(
        "private.rs",
        "#[macro_export] macro_rules! private_path_export { () => {} }\n",
    );
    fixture.write("lib.rs", "mod private;\n");
    let private_path = derive_family_at(&fixture.root, &fixture_family())
        .expect_err("private module macro export");
    assert!(private_path.contains("public expansion is not provable"));

    fixture.write(
        "private.rs",
        "#[cfg_attr(windows, macro_export)] macro_rules! configured_export { () => {} }\n",
    );
    let configured =
        derive_family_at(&fixture.root, &fixture_family()).expect_err("configured macro export");
    assert!(configured.contains("public expansion is not provable"));

    fixture.write("lib.rs", "pub extern crate core as exported_core;\n");
    let extern_crate = derive_family_at(&fixture.root, &fixture_family())
        .expect_err("public extern crate must fail closed");
    assert!(extern_crate.contains("extern-crate export"));

    fixture.write("lib.rs", "extern \"C\" { pub fn exported_foreign(); }\n");
    let foreign = derive_family_at(&fixture.root, &fixture_family())
        .expect_err("public foreign item must fail closed");
    assert!(foreign.contains("foreign-module surface"));
}

fn fixture_family() -> FacadeFamily {
    FacadeFamily {
        facade: "lib.rs",
        source_root: "",
        owner_prefix: "fixture/",
        preserve_underscores: false,
    }
}

fn resolved_relative_path(fixture: &Fixture) -> Option<String> {
    let facade = fixture.root.join("lib.rs");
    let graph = super::source_layout::ModuleGraph::build(&fixture.root).expect("fixture graph");
    let facade_module = graph
        .module_for_file(&facade)
        .expect("fixture facade module");
    let export = facade_exports(&facade)
        .expect("fixture facade")
        .into_iter()
        .next()
        .expect("fixture export");
    super::export_resolution::resolve_export(facade_module, &graph, &fixture.root, &export)
        .expect("resolve fixture export")
        .into_iter()
        .next()
        .map(|declaration| {
            declaration
                .path
                .strip_prefix(&fixture.root)
                .expect("fixture-relative declaration")
                .to_string_lossy()
                .replace('\\', "/")
        })
}

struct Fixture {
    root: PathBuf,
}

impl Fixture {
    fn new(label: &str) -> Self {
        let sequence = NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "worth-c8-api-{label}-{}-{sequence}",
            std::process::id()
        ));
        std::fs::create_dir_all(&root).expect("create API fixture");
        Self { root }
    }

    fn write(&self, relative: impl AsRef<Path>, source: &str) {
        let path = self.root.join(relative);
        std::fs::create_dir_all(path.parent().expect("fixture file parent"))
            .expect("create API fixture directory");
        std::fs::write(path, source).expect("write API fixture source");
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.root).expect("remove API fixture");
    }
}
