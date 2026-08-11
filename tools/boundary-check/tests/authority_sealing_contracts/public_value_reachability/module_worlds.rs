use super::AuthoritySealingTestRepository;

const HOST_DEV_WORLD: &str = "worlds = [{ name = \"host-dev-default\", target = \"host\", profile = \"dev\", default_features = true, features = [] }]";

#[test]
fn cfg_path_module_bodies_remain_distinct_per_cargo_world() {
    let repository = AuthoritySealingTestRepository::create("mw-profile");
    repository.assemble_public_value_witness_contract(
        r#"
#[cfg_attr(debug_assertions, path = "dev.rs")]
#[cfg_attr(not(debug_assertions), path = "release.rs")]
pub mod selected;
"#,
        r#"
pub(crate) fn selected() -> worth_proof::selected::WorldValue {
    #[cfg(debug_assertions)]
    { worth_proof::selected::WorldValue { value: 1 } }
    #[cfg(not(debug_assertions))]
    { worth_proof::selected::issue() }
}
"#,
        r#"
[[rule_contracts.public_value_reachability.witnesses]]
definition_path = "selected::WorldValue"
function = "selected"
public_type_path = "::worth_proof::selected::WorldValue"
posture = "value"
worlds = ["host-dev-default", "host-release-default"]
"#,
        "",
    );
    repository.write_file(
        "crates/worth-proof/src/dev.rs",
        "pub struct WorldValue { pub value: u8 }\n",
    );
    repository.write_file(
        "crates/worth-proof/src/release.rs",
        "pub struct WorldValue { value: u8 }\npub fn issue()->WorldValue{WorldValue{value:1}}\n",
    );
    repository.replace_public_value_config(
        HOST_DEV_WORLD,
        r#"worlds = [
  { name = "host-dev-default", target = "host", profile = "dev", default_features = true, features = [] },
  { name = "host-release-default", target = "host", profile = "release", default_features = true, features = [] },
]"#,
    );
    let (ok, output) = repository.run_boundary_check();
    repository.cleanup();
    assert!(ok, "cfg-selected module bodies must not merge:\n{output}");
}

#[test]
fn cfg_exclusive_platform_declarations_load_only_the_target_body() {
    let repository = AuthoritySealingTestRepository::create("mw-target");
    repository.assemble_public_value_witness_contract(
        r#"
#[cfg(not(target_arch = "wasm32"))]
#[path = "host.rs"]
pub mod platform;
#[cfg(target_arch = "wasm32")]
#[path = "wasm.rs"]
pub mod platform;
"#,
        r#"
pub(crate) fn platform() -> worth_proof::platform::PlatformValue {
    #[cfg(not(target_arch = "wasm32"))]
    { worth_proof::platform::PlatformValue { value: 1 } }
    #[cfg(target_arch = "wasm32")]
    { worth_proof::platform::issue() }
}
"#,
        r#"
[[rule_contracts.public_value_reachability.witnesses]]
definition_path = "platform::PlatformValue"
function = "platform"
public_type_path = "::worth_proof::platform::PlatformValue"
posture = "value"
worlds = ["host-dev-default", "wasm-dev-default"]
"#,
        "",
    );
    repository.write_file(
        "crates/worth-proof/src/host.rs",
        "pub struct PlatformValue { pub value: u8 }\n",
    );
    repository.write_file(
        "crates/worth-proof/src/wasm.rs",
        "pub struct PlatformValue { value: u8 }\npub fn issue()->PlatformValue{PlatformValue{value:1}}\n",
    );
    repository.replace_public_value_config(
        HOST_DEV_WORLD,
        r#"worlds = [
  { name = "host-dev-default", target = "host", profile = "dev", default_features = true, features = [] },
  { name = "wasm-dev-default", target = "wasm32-unknown-unknown", profile = "dev", default_features = true, features = [] },
]"#,
    );
    let (ok, output) = repository.run_boundary_check();
    repository.cleanup();
    assert!(ok, "platform bodies must remain target-affine:\n{output}");
}
