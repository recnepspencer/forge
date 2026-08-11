use super::AuthoritySealingTestRepository;

const HOST_DEV_WORLD: &str = "worlds = [{ name = \"host-dev-default\", target = \"host\", profile = \"dev\", default_features = true, features = [] }]";
const TWO_HOST_WORLDS: &str = r#"worlds = [
  { name = "host-dev-default", target = "host", profile = "dev", default_features = true, features = [] },
  { name = "host-release-default", target = "host", profile = "release", default_features = true, features = [] },
]"#;

#[test]
fn dev_and_release_worlds_inventory_and_compile_their_own_values() {
    let repository = AuthoritySealingTestRepository::create("cw-profile");
    repository.assemble_public_value_witness_contract(
        r#"
#[cfg(debug_assertions)]
pub struct DevSealed { value: u8 }
#[cfg(debug_assertions)]
pub fn issue_dev() -> DevSealed { DevSealed { value: 1 } }
#[cfg(not(debug_assertions))]
pub struct ReleaseSealed { value: u8 }
#[cfg(not(debug_assertions))]
pub fn issue_release() -> ReleaseSealed { ReleaseSealed { value: 2 } }
"#,
        r#"
#[cfg(debug_assertions)]
pub(crate) fn dev() -> worth_proof::DevSealed { worth_proof::issue_dev() }
#[cfg(not(debug_assertions))]
pub(crate) fn release() -> worth_proof::ReleaseSealed { worth_proof::issue_release() }
"#,
        r#"
[[rule_contracts.public_value_reachability.witnesses]]
definition_path = "DevSealed"
function = "dev"
public_type_path = "::worth_proof::DevSealed"
posture = "value"
worlds = ["host-dev-default"]

[[rule_contracts.public_value_reachability.witnesses]]
definition_path = "ReleaseSealed"
function = "release"
public_type_path = "::worth_proof::ReleaseSealed"
posture = "value"
worlds = ["host-release-default"]
"#,
        "",
    );
    install_two_host_worlds(&repository);
    let (ok, output) = repository.run_boundary_check();
    repository.cleanup();
    assert!(ok, "profile worlds must compile independently:\n{output}");
}

#[test]
fn build_script_cfg_is_observed_from_the_configured_cargo_world() {
    let repository = AuthoritySealingTestRepository::create("cw-build");
    repository.assemble_public_value_witness_contract(
        r#"
#[cfg(fixture_build_sealed)]
pub struct BuildSealed { value: u8 }
#[cfg(fixture_build_sealed)]
pub fn issue() -> BuildSealed { BuildSealed { value: 1 } }
"#,
        "pub(crate) fn build_value() -> worth_proof::BuildSealed { worth_proof::issue() }",
        r#"
[[rule_contracts.public_value_reachability.witnesses]]
definition_path = "BuildSealed"
function = "build_value"
public_type_path = "::worth_proof::BuildSealed"
posture = "value"
worlds = ["host-dev-default"]
"#,
        "",
    );
    repository.write_file(
        "crates/worth-proof/Cargo.toml",
        r#"[package]
name = "worth-proof"
version = "0.1.0"
edition = "2021"
build = "build.rs"
[workspace]
"#,
    );
    repository.write_file(
        "crates/worth-proof/build.rs",
        r#"fn main() {
    println!("cargo:rustc-check-cfg=cfg(fixture_build_sealed)");
    println!("cargo:rustc-cfg=fixture_build_sealed");
}
"#,
    );
    let (ok, output) = repository.run_boundary_check();
    repository.cleanup();
    assert!(ok, "build-script cfg must come from Cargo:\n{output}");
}

#[test]
fn explicit_features_and_default_feature_posture_are_shared_with_the_witness() {
    let repository = AuthoritySealingTestRepository::create("cw-feature");
    repository.assemble_public_value_witness_contract(
        r#"
#[cfg(feature = "default_value")]
pub struct DefaultSealed { value: u8 }
#[cfg(feature = "wide")]
pub struct WideSealed { value: u8 }
#[cfg(feature = "wide")]
pub fn issue() -> WideSealed { WideSealed { value: 1 } }
"#,
        "pub(crate) fn wide() -> worth_proof::WideSealed { worth_proof::issue() }",
        r#"
[[rule_contracts.public_value_reachability.witnesses]]
definition_path = "WideSealed"
function = "wide"
public_type_path = "::worth_proof::WideSealed"
posture = "value"
worlds = ["host-dev-wide"]
"#,
        "",
    );
    repository.write_file(
        "crates/worth-proof/Cargo.toml",
        r#"[package]
name = "worth-proof"
version = "0.1.0"
edition = "2021"
[features]
default = ["default_value"]
default_value = []
wide = []
[workspace]
"#,
    );
    repository.replace_public_value_config(
        HOST_DEV_WORLD,
        "worlds = [{ name = \"host-dev-wide\", target = \"host\", profile = \"dev\", default_features = false, features = [\"wide\"] }]",
    );
    let (ok, output) = repository.run_boundary_check();
    repository.cleanup();
    assert!(ok, "configured feature posture must be shared:\n{output}");
}

#[test]
fn governed_cargo_rustflags_are_shared_with_the_witness_build() {
    let repository = AuthoritySealingTestRepository::create("cw-rustflag");
    repository.assemble_public_value_witness_contract(
        r#"
#[cfg(fixture_rustflag_sealed)]
pub struct RustflagSealed { value: u8 }
#[cfg(fixture_rustflag_sealed)]
pub fn issue() -> RustflagSealed { RustflagSealed { value: 1 } }
"#,
        "pub(crate) fn rustflag() -> worth_proof::RustflagSealed { worth_proof::issue() }",
        r#"
[[rule_contracts.public_value_reachability.witnesses]]
definition_path = "RustflagSealed"
function = "rustflag"
public_type_path = "::worth_proof::RustflagSealed"
posture = "value"
worlds = ["host-dev-default"]
"#,
        "",
    );
    repository.write_file(
        "crates/worth-proof/.cargo/config.toml",
        r#"[build]
rustflags = ["--cfg", "fixture_rustflag_sealed", "--check-cfg=cfg(fixture_rustflag_sealed)"]
"#,
    );
    let (ok, output) = repository.run_boundary_check();
    repository.cleanup();
    assert!(ok, "governed Cargo rustflags must be shared:\n{output}");
}

#[test]
fn inherited_custom_workspace_profile_is_shared_with_the_witness_build() {
    let repository = AuthoritySealingTestRepository::create("cw-profile-table");
    repository.assemble_public_value_witness_contract(
        r#"
#[cfg(debug_assertions)]
pub struct ProfileSealed { value: u8 }
#[cfg(debug_assertions)]
pub fn issue() -> ProfileSealed { ProfileSealed { value: 1 } }
"#,
        r#"
#[cfg(debug_assertions)]
pub(crate) fn profile_value() -> worth_proof::ProfileSealed { worth_proof::issue() }
"#,
        r#"
[[rule_contracts.public_value_reachability.witnesses]]
definition_path = "ProfileSealed"
function = "profile_value"
public_type_path = "::worth_proof::ProfileSealed"
posture = "value"
worlds = ["host-audit-default"]
"#,
        "",
    );
    repository.write_file(
        "crates/worth-proof/Cargo.toml",
        r#"[package]
name = "worth-proof"
version = "0.1.0"
edition = "2021"
[workspace]

[profile.audit]
inherits = "release"
debug-assertions = true
"#,
    );
    repository.replace_public_value_config(
        HOST_DEV_WORLD,
        "worlds = [{ name = \"host-audit-default\", target = \"host\", profile = \"audit\", default_features = true, features = [] }]",
    );
    let (ok, output) = repository.run_boundary_check();
    repository.cleanup();
    assert!(
        ok,
        "the consumer must inherit the exact custom workspace profile:\n{output}"
    );
}

#[test]
fn noisy_and_hung_build_scripts_are_bounded_during_cfg_discovery() {
    for (label, build_script, timeout_ms, max_output_bytes, expected) in [
        (
            "cw-noisy-build",
            r#"fn main() {
    for index in 0..10_000 { eprintln!("noise-{index:05}-xxxxxxxxxxxxxxxx"); }
    panic!("force Cargo to surface captured build output");
}"#,
            30_000,
            4096,
            "exceeded configured output",
        ),
        (
            "cw-hung-build",
            r#"fn main() { std::thread::sleep(std::time::Duration::from_secs(3)); }"#,
            1000,
            65_536,
            "timed out",
        ),
    ] {
        let repository = AuthoritySealingTestRepository::create(label);
        repository.assemble_public_value_witness_contract("pub struct Open;", "", "", "");
        repository.write_file(
            "crates/worth-proof/Cargo.toml",
            r#"[package]
name = "worth-proof"
version = "0.1.0"
edition = "2021"
build = "build.rs"
[workspace]
"#,
        );
        repository.write_file("crates/worth-proof/build.rs", build_script);
        repository.replace_public_value_config(
            "compilation_timeout_ms = 30000",
            &format!("compilation_timeout_ms = {timeout_ms}"),
        );
        repository.replace_public_value_config(
            "max_output_bytes = 65536",
            &format!("max_output_bytes = {max_output_bytes}"),
        );
        let started = std::time::Instant::now();
        let (ok, output) = repository.run_boundary_check();
        let elapsed = started.elapsed();
        repository.cleanup();
        assert!(!ok, "unbounded build script unexpectedly passed");
        assert!(output.contains(expected), "unexpected denial:\n{output}");
        assert!(
            elapsed < std::time::Duration::from_secs(8),
            "bounded discovery exceeded its lifecycle budget: {elapsed:?}"
        );
    }
}

#[test]
fn ignored_package_profile_table_fails_closed() {
    let repository = AuthoritySealingTestRepository::create("cw-ignored-profile");
    repository.assemble_public_value_witness_contract("pub struct Open;", "", "", "");
    repository.write_file(
        "crates/worth-proof/Cargo.toml",
        r#"[package]
name = "worth-proof"
version = "0.1.0"
edition = "2021"
[package.profile.dev]
debug-assertions = true
[workspace]
"#,
    );
    let (ok, output) = repository.run_boundary_check();
    repository.cleanup();
    assert!(!ok, "Cargo-ignored package profile unexpectedly passed");
    assert!(
        output.contains("nested under `[package]` is ignored"),
        "unexpected denial:\n{output}"
    );
}

fn install_two_host_worlds(repository: &AuthoritySealingTestRepository) {
    repository.replace_public_value_config(HOST_DEV_WORLD, TWO_HOST_WORLDS);
}
