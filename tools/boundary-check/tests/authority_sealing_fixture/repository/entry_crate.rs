//! Entry-band crate layout and generated-source fixture writers.

use super::super::facade_projection::facade_reexports;
use super::AuthoritySealingTestRepository;

impl AuthoritySealingTestRepository {
    pub(super) fn write_generated_source_exemptions(&self, paths: &[&str]) {
        self.write_file(
            "tools/boundary-check/config/generated_source_exemptions.txt",
            &paths.join("\n"),
        );
    }

    pub(super) fn write_entry_crate(&self, lib_source: &str, dependency_package: Option<&str>) {
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

    pub(super) fn write_entry_crate_layout(
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
}
