//! Temporary governed repository writer for authority sealing proofs.

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use super::facade_projection::{facade_reexports, facade_reexports_with_external};

/// Isolated temporary repository exercising the production boundary-check binary.
pub struct AuthoritySealingTestRepository {
    pub(super) root: PathBuf,
}

impl AuthoritySealingTestRepository {
    pub fn create(label: &str) -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock")
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "boundary-check-authority-sealing-{label}-{}-{nanos}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&root);
        Self { root }
    }

    /// Assemble thin orchestrator + one entry-band governed crate with given lib.rs.
    pub fn assemble_with_lib_source(&self, lib_source: &str) {
        self.assemble_with_lib_source_and_config(lib_source, &self.minimal_config());
    }

    /// Assemble with an explicit road1.toml body (for substrate-admission hostiles).
    pub fn assemble_with_lib_source_and_config(&self, lib_source: &str, config: &str) {
        self.write_file("cad/docs/worthy-foundations/NAMING.md", "# naming\n");
        fs::create_dir_all(self.root.join("cad/workspaces")).expect("cad/workspaces");
        self.write_file(
            "Cargo.toml",
            r#"[workspace]
resolver = "2"
exclude = ["cad/workspaces/*"]
members = []

[workspace.metadata.worth_topology]
role = "thin_orchestrator"
road1_subworkspaces = []
forbidden_member_prefixes = ["cad/workspaces/"]
boundary_check_manifest = "tools/boundary-check/Cargo.toml"
boundary_check_config = "tools/boundary-check/config/road1.toml"
"#,
        );
        self.write_file(
            "tools/boundary-check/Cargo.toml",
            r#"[package]
name = "boundary-check"
version = "0.1.0"
edition = "2021"
publish = false
[workspace]
"#,
        );
        self.write_file(
            "tools/boundary-check/snapshots/legacy-references.toml",
            "schema_version = 1\nreferences = []\n",
        );
        self.write_file("tools/boundary-check/config/road1.toml", config);
        self.write_query_audience_leaf_facades();
        self.write_worth_proof_stub();
        self.write_entry_crate(lib_source, None);
    }

    /// Assemble entry crate that path-depends on a stub substrate package.
    pub fn assemble_with_substrate_dependency(
        &self,
        lib_source: &str,
        config: &str,
        substrate_package: &str,
    ) {
        self.assemble_with_lib_source_and_config(lib_source, config);
        self.write_file(
            &format!("vendor/{substrate_package}/Cargo.toml"),
            &format!(
                r#"[package]
name = "{substrate_package}"
version = "0.1.0"
edition = "2021"
[workspace]
"#
            ),
        );
        self.write_file(
            &format!("vendor/{substrate_package}/src/lib.rs"),
            "// stub law substrate\n",
        );
        // Rewrite entry crate manifest with the substrate path dependency.
        self.write_entry_crate(lib_source, Some(substrate_package));
    }

    /// Assemble entry crate that publicly re-exports a non-governed path dependency.
    ///
    /// The dependency is placed under `vendor/` so it is not itself scanned as a
    /// governed crate — the re-export seam is the ordinary public surface under test.
    pub fn assemble_with_external_path_dependency(
        &self,
        entry_lib: &str,
        external_package: &str,
        external_lib: &str,
    ) {
        self.assemble_with_lib_source_and_config(entry_lib, &self.minimal_config());
        self.write_file(
            &format!("vendor/{external_package}/Cargo.toml"),
            &format!(
                r#"[package]
name = "{external_package}"
version = "0.1.0"
edition = "2021"
[workspace]
"#
            ),
        );
        self.write_file(
            &format!("vendor/{external_package}/src/lib.rs"),
            external_lib,
        );
        self.write_entry_crate(entry_lib, Some(external_package));
        self.write_file(
            "cad/workspaces/worth-entry/crates/worth-entry-adoption/src/facade.rs",
            &facade_reexports_with_external(entry_lib, external_package, external_lib),
        );
    }

    /// Assemble with a custom `[lib] path` and additional crate-relative source files.
    ///
    /// `crate_files` are paths relative to the entry crate root (e.g. `src/ordinary_api.rs`).
    pub fn assemble_with_lib_path_and_files(&self, lib_path: &str, crate_files: &[(&str, &str)]) {
        self.assemble_with_lib_source_and_config("// placeholder", &self.minimal_config());
        if lib_path == "src/lib.rs" {
            self.write_entry_crate_layout(lib_path, crate_files, None);
            self.write_generated_source_exemptions(&[
                "cad/workspaces/worth-entry/crates/worth-entry-adoption/src/facade.rs",
                "cad/workspaces/worth-entry/crates/worth-entry-adoption/src/test_surface.rs",
            ]);
        } else {
            let specimen = crate_files
                .iter()
                .find_map(|(path, source)| (*path == lib_path).then_some(*source))
                .expect("custom lib source");
            let mut migrated = crate_files.to_vec();
            migrated.retain(|(path, _)| *path != lib_path);
            migrated.push((lib_path, "pub(crate) mod test_surface;\npub mod facade;\n"));
            migrated.push(("src/test_surface.rs", specimen));
            self.write_entry_crate_layout(lib_path, &migrated, None);
            self.write_file(
                "cad/workspaces/worth-entry/crates/worth-entry-adoption/src/facade.rs",
                &facade_reexports(specimen),
            );
            self.write_generated_source_exemptions(&[
                "cad/workspaces/worth-entry/crates/worth-entry-adoption/src/lib.rs",
            ]);
        }
    }

    fn write_generated_source_exemptions(&self, paths: &[&str]) {
        self.write_file(
            "tools/boundary-check/config/generated_source_exemptions.txt",
            &paths.join("\n"),
        );
    }

    fn write_entry_crate(&self, lib_source: &str, dependency_package: Option<&str>) {
        let migrated_source = lib_source.replace("crate::", "crate::test_surface::");
        self.write_entry_crate_layout(
            "src/lib.rs",
            &[
                (
                    "src/lib.rs",
                    "pub(crate) mod test_surface;\npub mod facade;\n",
                ),
                ("src/test_surface.rs", &migrated_source),
            ],
            dependency_package,
        );
        self.write_file(
            "cad/workspaces/worth-entry/crates/worth-entry-adoption/src/facade.rs",
            &facade_reexports(&migrated_source),
        );
    }

    fn write_entry_crate_layout(
        &self,
        lib_path: &str,
        crate_files: &[(&str, &str)],
        dependency_package: Option<&str>,
    ) {
        self.write_file(
            "cad/workspaces/worth-entry/Cargo.toml",
            r#"[workspace]
resolver = "2"
members = ["crates/worth-entry-adoption"]

[workspace.metadata.worth_topology]
role = "road1_subworkspace"
constitutional_lane = "worth-entry"
member_lane = "crates/*"
allowed_crate_prefixes = ["worth-entry-"]
"#,
        );
        self.write_file(
            "cad/workspaces/worth-entry/README.md",
            "# worth-entry\n\nRoad 1 subworkspace charter for tests.\n",
        );
        let mut dependency_block = String::from(
            r#"
[dependencies]
worth-proof = { path = "../../../../../vendor/worth-proof" }
"#,
        );
        if let Some(package) = dependency_package.filter(|package| *package != "worth-proof") {
            dependency_block.push_str(&format!(
                "{package} = {{ path = \"../../../../../vendor/{package}\" }}\n"
            ));
        }
        let lib_table = if lib_path == "src/lib.rs" {
            String::new()
        } else {
            format!("\n[lib]\npath = \"{lib_path}\"\n")
        };
        self.write_file(
            "cad/workspaces/worth-entry/crates/worth-entry-adoption/Cargo.toml",
            &format!(
                r#"[package]
name = "worth-entry-adoption"
version = "0.1.0"
edition = "2021"
{lib_table}{dependency_block}"#
            ),
        );
        for (rel, contents) in crate_files {
            self.write_file(
                &format!("cad/workspaces/worth-entry/crates/worth-entry-adoption/{rel}"),
                contents,
            );
        }
        self.write_file(
            "cad/workspaces/worth-entry/crates/worth-entry-adoption/src/facade.rs",
            "// Phase 6 fixture facade: intentionally no exports.\n",
        );
    }

    fn write_worth_proof_stub(&self) {
        self.write_file(
            "vendor/worth-proof/Cargo.toml",
            r#"[package]
name = "worth-proof"
version = "0.1.0"
edition = "2021"
[workspace]
"#,
        );
        self.write_file(
            "vendor/worth-proof/src/lib.rs",
            r#"pub struct AuthorityWitness<A>(core::marker::PhantomData<A>);
pub struct CapabilityWitness<C>(core::marker::PhantomData<C>);
pub struct Proof<P, A>(core::marker::PhantomData<(P, A)>);
"#,
        );
    }

    fn write_query_audience_leaf_facades(&self) {
        self.write_file(
            "crates/worth-query/Cargo.toml",
            r#"[package]
name = "worth-query"
version = "0.1.0"
edition = "2021"
[workspace]
"#,
        );
        self.write_file("crates/worth-query/src/lib.rs", "// stub engine\n");
        for (package, export) in [
            (
                "worth-query-decl",
                "pub use worth_query::facade::foundation::CanonicalQueryArtifact;\n",
            ),
            (
                "worth-query-host",
                "pub use worth_query::facade::domain;\npub use worth_query::facade::runtime;\n",
            ),
            (
                "worth-query-replay",
                "pub use worth_query::facade::foundation::ScopedReplayBasis;\n",
            ),
        ] {
            self.write_file(
                &format!("crates/{package}/Cargo.toml"),
                &format!(
                    r#"[package]
name = "{package}"
version = "0.1.0"
edition = "2021"

[dependencies]
worth-query = {{ path = "../worth-query" }}

[workspace]
"#
                ),
            );
            self.write_file(&format!("crates/{package}/src/lib.rs"), "pub mod facade;\n");
            self.write_file(&format!("crates/{package}/src/facade.rs"), export);
        }
    }

    pub fn write_file(&self, relative: &str, contents: &str) {
        let path = self.root.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("create parent directories");
        }
        fs::write(path, contents).expect("write file");
    }
}
