use super::{
    assert_contract_allowed, assert_contract_denied, value_row, AuthoritySealingTestRepository,
};

#[test]
fn configured_roots_are_canonical_descendants_and_aliases_fail_closed() {
    let alias = AuthoritySealingTestRepository::create("configured-root-alias");
    alias.assemble_public_value_witness_contract(
        "pub struct Sealed { value:u8 } pub fn issue()->Sealed{Sealed{value:1}}",
        "pub(crate) fn sealed()->worth_proof::Sealed{worth_proof::issue()}",
        value_row(),
        "",
    );
    alias.replace_public_value_config(
        "crate_root = \"crates/worth-proof\"",
        "crate_root = \"crates/../crates/worth-proof\"",
    );
    alias.replace_public_value_config(
        "witness_source = \"tools/boundary-check/public_value_witnesses/worth_proof/mod.rs\"",
        "witness_source = \"tools/../tools/boundary-check/public_value_witnesses/worth_proof/mod.rs\"",
    );
    let (ok, output) = alias.run_boundary_check();
    alias.cleanup();
    assert!(!ok, "configured path aliases must fail closed:\n{output}");

    let witness_alias = AuthoritySealingTestRepository::create("witness-source-alias");
    witness_alias.assemble_public_value_witness_contract(
        "pub struct Sealed { value:u8 } pub fn issue()->Sealed{Sealed{value:1}}",
        "pub(crate) fn sealed()->worth_proof::Sealed{worth_proof::issue()}",
        value_row(),
        "",
    );
    witness_alias.replace_public_value_config(
        "witness_source = \"tools/boundary-check/public_value_witnesses/worth_proof/mod.rs\"",
        "witness_source = \"tools/../tools/boundary-check/public_value_witnesses/worth_proof/mod.rs\"",
    );
    let (ok, output) = witness_alias.run_boundary_check();
    witness_alias.cleanup();
    assert!(!ok, "witness path aliases must fail closed:\n{output}");

    for field in ["crate_root", "witness_source"] {
        assert_escape_denied(field);
    }
}

#[test]
fn every_configured_target_world_requires_its_own_witness_row() {
    let repository = AuthoritySealingTestRepository::create("target-world-coverage");
    repository.assemble_public_value_witness_contract(
        "pub struct Sealed { value:u8 } pub fn issue()->Sealed{Sealed{value:1}}",
        "pub(crate) fn sealed()->worth_proof::Sealed{worth_proof::issue()}",
        value_row(),
        "",
    );
    repository.replace_public_value_config(
        "worlds = [{ name = \"host-dev-default\", target = \"host\", profile = \"dev\", default_features = true, features = [] }]\nhost_timeout_ms = 30000\ncompilation_timeout_ms = 30000\nmax_output_bytes = 65536",
        "worlds = [\n  { name = \"host-dev-default\", target = \"host\", profile = \"dev\", default_features = true, features = [] },\n  { name = \"wasm-dev-default\", target = \"wasm32-unknown-unknown\", profile = \"dev\", default_features = true, features = [] },\n]\nhost_timeout_ms = 30000\ncompilation_timeout_ms = 30000\nmax_output_bytes = 65536",
    );
    let (ok, output) = repository.run_boundary_check();
    repository.cleanup();
    assert!(!ok, "the wasm world needs its own row:\n{output}");
    assert!(output.contains("wasm-dev-default"), "{output}");
}

#[test]
fn exact_host_cfg_atoms_and_cfg_attr_project_one_real_world() {
    let (present, absent) = if cfg!(windows) {
        ("windows", "unix")
    } else {
        ("unix", "windows")
    };
    assert_contract_denied(
        "present-host-cfg-is-sealed",
        &present_host_cfg_source(present),
        "",
        "",
        "",
    );
    assert_contract_allowed(
        "cfg-absent",
        &absent_host_cfg_source(absent),
        OPEN_CFG_WITNESSES,
        &open_cfg_rows(),
        "",
    );
}

fn present_host_cfg_source(present: &str) -> String {
    format!(
        "pub struct PresentField {{ #[cfg({present})] hidden: u8, pub exposed: u8 }}\n\
         #[cfg_attr({present}, non_exhaustive)]\n\
         pub struct PresentAttribute {{ pub exposed: u8 }}\n"
    )
}

fn absent_host_cfg_source(absent: &str) -> String {
    format!(
        "pub struct OpenField {{ #[cfg({absent})] hidden: u8, pub exposed: u8 }}\n\
         #[cfg_attr({absent}, non_exhaustive)]\n\
         pub struct OpenAttribute {{ pub exposed: u8 }}\n\
         pub struct Sealed {{ value: u8 }}\n\
         pub fn issue() -> Sealed {{ Sealed {{ value: 1 }} }}\n"
    )
}

const OPEN_CFG_WITNESSES: &str = r#"
pub(crate) fn sealed() -> worth_proof::Sealed { worth_proof::issue() }
pub(crate) fn open_field() -> worth_proof::OpenField { worth_proof::OpenField { exposed: 1 } }
pub(crate) fn open_attribute() -> worth_proof::OpenAttribute {
    worth_proof::OpenAttribute { exposed: 2 }
}
"#;

fn open_cfg_rows() -> String {
    format!(
        r#"{}
[[rule_contracts.public_value_reachability.witnesses]]
definition_path = "OpenField"
function = "open_field"
public_type_path = "::worth_proof::OpenField"
posture = "value"
worlds = ["host-dev-default"]

[[rule_contracts.public_value_reachability.witnesses]]
definition_path = "OpenAttribute"
function = "open_attribute"
public_type_path = "::worth_proof::OpenAttribute"
posture = "value"
worlds = ["host-dev-default"]
"#,
        value_row()
    )
}

fn assert_escape_denied(field: &str) {
    let repository = AuthoritySealingTestRepository::create(&format!("{field}-escape"));
    repository.assemble_public_value_witness_contract(
        "pub struct Sealed { value:u8 } pub fn issue()->Sealed{Sealed{value:1}}",
        "pub(crate) fn sealed()->worth_proof::Sealed{worth_proof::issue()}",
        value_row(),
        "",
    );
    let outside = repository.unique_public_value_escape_root();
    repository.write_outside_public_value_crate(
        &outside,
        "pub struct Sealed { value:u8 } pub fn issue()->Sealed{Sealed{value:1}}",
    );
    if field == "crate_root" {
        repository.replace_public_value_config(
            "crate_root = \"crates/worth-proof\"",
            &format!("crate_root = \"{outside}\""),
        );
    } else {
        repository.write_file(
            &format!("{outside}/witness.rs"),
            "pub(crate) fn sealed()->worth_proof::Sealed{worth_proof::issue()}",
        );
        repository.replace_public_value_config(
            "witness_source = \"tools/boundary-check/public_value_witnesses/worth_proof/mod.rs\"",
            &format!("witness_source = \"{outside}/witness.rs\""),
        );
    }
    let (ok, output) = repository.run_boundary_check();
    repository.remove_outside_public_value_root(&outside);
    repository.cleanup();
    assert!(!ok, "{field} escape must fail closed:\n{output}");
}
