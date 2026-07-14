//! Cargo-layout assembly: workspace-inherited, target-specific, and non-path deps.

use super::facade_projection::facade_reexports;
use super::repository::AuthoritySealingTestRepository;

impl AuthoritySealingTestRepository {
    /// Entry depends via `{ workspace = true }`; path lives in workspace.dependencies.
    pub fn assemble_with_workspace_inherited_dep(
        &self,
        entry_lib: &str,
        dep_package: &str,
        dep_lib: &str,
    ) {
        self.assemble_with_lib_source_and_config(entry_lib, &self.minimal_config());
        self.write_file(
            &format!("vendor/{dep_package}/Cargo.toml"),
            &format!(
                r#"[package]
name = "{dep_package}"
version = "0.1.0"
edition = "2021"
[workspace]
"#
            ),
        );
        self.write_file(&format!("vendor/{dep_package}/src/lib.rs"), dep_lib);
        self.write_file(
            "cad/workspaces/worth-entry/Cargo.toml",
            &format!(
                r#"[workspace]
resolver = "2"
members = ["crates/worth-entry-adoption"]

[workspace.dependencies]
{dep_package} = {{ path = "../../../vendor/{dep_package}" }}

[workspace.metadata.worth_topology]
role = "road1_subworkspace"
constitutional_lane = "worth-entry"
member_lane = "crates/*"
allowed_crate_prefixes = ["worth-entry-"]
"#
            ),
        );
        self.write_file("cad/workspaces/worth-entry/README.md", "# worth-entry\n");
        self.write_file(
            "cad/workspaces/worth-entry/crates/worth-entry-adoption/Cargo.toml",
            &format!(
                r#"[package]
name = "worth-entry-adoption"
version = "0.1.0"
edition = "2021"

[dependencies]
{dep_package} = {{ workspace = true }}
"#
            ),
        );
        self.write_file(
            "cad/workspaces/worth-entry/crates/worth-entry-adoption/src/lib.rs",
            "pub(crate) mod test_surface;\npub mod facade;\n",
        );
        self.write_file(
            "cad/workspaces/worth-entry/crates/worth-entry-adoption/src/test_surface.rs",
            entry_lib,
        );
        self.write_file(
            "cad/workspaces/worth-entry/crates/worth-entry-adoption/src/facade.rs",
            &facade_reexports(entry_lib),
        );
    }

    /// Entry depends via target-specific dependency table (any cfg branch).
    pub fn assemble_with_target_specific_dep(
        &self,
        entry_lib: &str,
        dep_package: &str,
        dep_lib: &str,
    ) {
        self.assemble_with_lib_source_and_config(entry_lib, &self.minimal_config());
        self.write_file(
            &format!("vendor/{dep_package}/Cargo.toml"),
            &format!(
                r#"[package]
name = "{dep_package}"
version = "0.1.0"
edition = "2021"
[workspace]
"#
            ),
        );
        self.write_file(&format!("vendor/{dep_package}/src/lib.rs"), dep_lib);
        self.write_file(
            "cad/workspaces/worth-entry/crates/worth-entry-adoption/Cargo.toml",
            &format!(
                r#"[package]
name = "worth-entry-adoption"
version = "0.1.0"
edition = "2021"

[target.'cfg(all())'.dependencies]
{dep_package} = {{ path = "../../../../../vendor/{dep_package}" }}
"#
            ),
        );
        self.write_file(
            "cad/workspaces/worth-entry/crates/worth-entry-adoption/src/lib.rs",
            "pub(crate) mod test_surface;\npub mod facade;\n",
        );
        self.write_file(
            "cad/workspaces/worth-entry/crates/worth-entry-adoption/src/test_surface.rs",
            entry_lib,
        );
        self.write_file(
            "cad/workspaces/worth-entry/crates/worth-entry-adoption/src/facade.rs",
            &facade_reexports(entry_lib),
        );
    }

    /// Entry depends via a version-only crates.io source (real package so cargo
    /// metadata succeeds). BC7001 must fail closed on the non-path source rather
    /// than omitting it from the sealed-export index.
    pub fn assemble_with_version_only_dep(
        &self,
        entry_lib: &str,
        dep_package: &str,
        version: &str,
    ) {
        self.assemble_with_lib_source_and_config(entry_lib, &self.minimal_config());
        self.write_file(
            "cad/workspaces/worth-entry/crates/worth-entry-adoption/Cargo.toml",
            &format!(
                r#"[package]
name = "worth-entry-adoption"
version = "0.1.0"
edition = "2021"

[dependencies]
{dep_package} = "{version}"
"#
            ),
        );
        self.write_file(
            "cad/workspaces/worth-entry/crates/worth-entry-adoption/src/lib.rs",
            "pub(crate) mod test_surface;\npub mod facade;\n",
        );
        self.write_file(
            "cad/workspaces/worth-entry/crates/worth-entry-adoption/src/test_surface.rs",
            entry_lib,
        );
        self.write_file(
            "cad/workspaces/worth-entry/crates/worth-entry-adoption/src/facade.rs",
            &facade_reexports(entry_lib),
        );
    }

    /// Entry depends via a table-shaped registry source (`version` without `path`).
    pub fn assemble_with_registry_table_dep(
        &self,
        entry_lib: &str,
        dep_package: &str,
        version: &str,
    ) {
        self.assemble_with_lib_source_and_config(entry_lib, &self.minimal_config());
        self.write_file(
            "cad/workspaces/worth-entry/crates/worth-entry-adoption/Cargo.toml",
            &format!(
                r#"[package]
name = "worth-entry-adoption"
version = "0.1.0"
edition = "2021"

[dependencies]
{dep_package} = {{ version = "{version}" }}
"#
            ),
        );
        self.write_file(
            "cad/workspaces/worth-entry/crates/worth-entry-adoption/src/lib.rs",
            "pub(crate) mod test_surface;\npub mod facade;\n",
        );
        self.write_file(
            "cad/workspaces/worth-entry/crates/worth-entry-adoption/src/test_surface.rs",
            entry_lib,
        );
        self.write_file(
            "cad/workspaces/worth-entry/crates/worth-entry-adoption/src/facade.rs",
            &facade_reexports(entry_lib),
        );
    }
}
