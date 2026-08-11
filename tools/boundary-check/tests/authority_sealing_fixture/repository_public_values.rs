//! Compiler-witness fixtures for exact public-value coverage contracts.

use super::repository::AuthoritySealingTestRepository;

impl AuthoritySealingTestRepository {
    pub fn public_value_fixture_path(&self, relative: &str) -> std::path::PathBuf {
        self.root.join(relative)
    }

    pub fn assemble_public_value_witness_contract(
        &self,
        lib_source: &str,
        witness_source: &str,
        witness_rows: &str,
        exemptions: &str,
    ) {
        let default_contract = r#"[rule_contracts.public_value_reachability]
package = "public-value-fixture"
crate_root = "vendor/public-value-fixture"
witness_source = "tools/boundary-check/public_value_witnesses/public_value_fixture/mod.rs"
worlds = [{ name = "host-dev-default", target = "host", profile = "dev", default_features = true, features = [] }]
host_timeout_ms = 30000
compilation_timeout_ms = 30000
max_output_bytes = 65536
guidance = "Expose a checked public introduction site."
"#;
        let contract = format!(
            r#"[rule_contracts.public_value_reachability]
package = "worth-proof"
crate_root = "crates/worth-proof"
witness_source = "tools/boundary-check/public_value_witnesses/worth_proof/mod.rs"
worlds = [{{ name = "host-dev-default", target = "host", profile = "dev", default_features = true, features = [] }}]
host_timeout_ms = 30000
compilation_timeout_ms = 30000
max_output_bytes = 65536
guidance = "Provide an exact compiler witness."
{witness_rows}
{exemptions}
"#
        );
        let config = self.minimal_config().replace(default_contract, &contract);
        self.assemble_with_lib_source_and_config("pub struct EntrySurface;", &config);
        self.write_file(
            "crates/worth-proof/Cargo.toml",
            r#"[package]
name = "worth-proof"
version = "0.1.0"
edition = "2021"
[workspace]
"#,
        );
        self.write_file("crates/worth-proof/src/lib.rs", lib_source);
        self.write_file(
            "tools/boundary-check/public_value_witnesses/worth_proof/mod.rs",
            witness_source,
        );
    }

    pub fn assemble_without_public_value_contract(&self) {
        let contract = r#"[rule_contracts.public_value_reachability]
package = "public-value-fixture"
crate_root = "vendor/public-value-fixture"
witness_source = "tools/boundary-check/public_value_witnesses/public_value_fixture/mod.rs"
worlds = [{ name = "host-dev-default", target = "host", profile = "dev", default_features = true, features = [] }]
host_timeout_ms = 30000
compilation_timeout_ms = 30000
max_output_bytes = 65536
guidance = "Expose a checked public introduction site."
"#;
        let config = self.minimal_config().replace(contract, "");
        self.assemble_with_lib_source_and_config("pub struct EntrySurface;", &config);
    }

    pub fn replace_public_value_config(&self, from: &str, to: &str) {
        let path = self.root.join("tools/boundary-check/config/road1.toml");
        let source = std::fs::read_to_string(&path).expect("read public-value config");
        std::fs::write(path, source.replace(from, to)).expect("rewrite public-value config");
    }

    pub fn unique_public_value_escape_root(&self) -> String {
        format!(
            "../{}-outside",
            self.root
                .file_name()
                .expect("temporary repository name")
                .to_string_lossy()
        )
    }

    pub fn write_outside_public_value_crate(&self, relative: &str, source: &str) {
        self.write_file(
            &format!("{relative}/Cargo.toml"),
            r#"[package]
name = "worth-proof"
version = "0.1.0"
edition = "2021"
[workspace]
"#,
        );
        self.write_file(&format!("{relative}/src/lib.rs"), source);
    }

    pub fn remove_outside_public_value_root(&self, relative: &str) {
        let _ = std::fs::remove_dir_all(self.root.join(relative));
    }
}
