use super::authority_sealing_fixture::AuthoritySealingTestRepository;

#[path = "public_value_reachability/cargo_worlds.rs"]
mod cargo_worlds;
#[path = "public_value_reachability/configuration_paths.rs"]
mod configuration_paths;
#[path = "public_value_reachability/exact_type_identity.rs"]
mod exact_type_identity;
#[path = "public_value_reachability/item_macro_inventory.rs"]
mod item_macro_inventory;
#[path = "public_value_reachability/module_worlds.rs"]
mod module_worlds;
#[path = "public_value_reachability/public_type_inventory.rs"]
mod public_type_inventory;
#[path = "public_value_reachability/reexport_chains.rs"]
mod reexport_chains;
#[path = "public_value_reachability/runtime_honesty.rs"]
mod runtime_honesty;
#[path = "public_value_reachability/uninhabited_exemption.rs"]
mod uninhabited_exemption;
#[path = "public_value_reachability/union_inventory.rs"]
mod union_inventory;

const GOVERNED: &str = r#"
pub struct Sealed { value: u8 }
pub struct CallbackSealed { value: u8 }

pub fn issue() -> Sealed { Sealed { value: 1 } }
pub fn deliver(callback: impl FnOnce(CallbackSealed)) {
    callback(CallbackSealed { value: 2 });
}
"#;

const VALUE_GOVERNED: &str =
    "pub struct Sealed { value:u8 } pub fn issue()->Sealed{Sealed{value:1}}";
const CALLBACK_GOVERNED: &str = r#"
pub struct CallbackSealed { value:u8 }
pub fn deliver(callback: impl FnOnce(CallbackSealed)) {
    callback(CallbackSealed { value:2 });
}
"#;

const HONEST_WITNESSES: &str = r#"
pub(crate) fn sealed() -> worth_proof::Sealed { worth_proof::issue() }
pub(crate) fn callback(deliver: impl FnOnce(worth_proof::CallbackSealed)) {
    worth_proof::deliver(deliver);
}
"#;

const HONEST_ROWS: &str = r#"
[[rule_contracts.public_value_reachability.witnesses]]
definition_path = "Sealed"
function = "sealed"
public_type_path = "::worth_proof::Sealed"
posture = "value"
worlds = ["host-dev-default"]

[[rule_contracts.public_value_reachability.witnesses]]
definition_path = "CallbackSealed"
function = "callback"
public_type_path = "::worth_proof::CallbackSealed"
posture = "callback"
worlds = ["host-dev-default"]
"#;

#[test]
fn exact_value_and_callback_witnesses_compile_and_complete() {
    assert_contract_allowed(
        "compiler-witness-honest",
        GOVERNED,
        HONEST_WITNESSES,
        HONEST_ROWS,
        "",
    );
}

#[test]
fn honest_bounded_loop_and_return_materialize_the_exact_value() {
    assert_contract_allowed(
        "bounded-loop-and-return",
        VALUE_GOVERNED,
        r#"
pub(crate) fn sealed() -> worth_proof::Sealed {
    let mut materialized = None;
    for step in 0..1 {
        if step == 0 {
            materialized = Some(worth_proof::issue());
        }
    }
    return materialized.expect("the bounded loop materializes once");
}
"#,
        value_row(),
        "",
    );
}

#[test]
fn witness_bodies_cannot_manufacture_diverge_or_exit_before_completion() {
    for (label, source) in [
        (
            "unsafe-zeroed",
            "pub(crate) fn sealed() -> worth_proof::Sealed { unsafe { core::mem::zeroed() } }",
        ),
        (
            "callback-ignored",
            "pub(crate) fn callback(_: impl FnOnce(worth_proof::CallbackSealed)) {}",
        ),
        (
            "callback-diverging-argument",
            r#"fn diverge() -> ! { loop {} }
pub(crate) fn callback(deliver: impl FnOnce(worth_proof::CallbackSealed)) {
    worth_proof::deliver(|_| deliver(diverge()));
}"#,
        ),
        (
            "process-exit",
            "pub(crate) fn sealed() -> worth_proof::Sealed { std::process::exit(0) }",
        ),
        (
            "aliased-process-exit",
            "use std::process::exit as finish; pub(crate) fn sealed() -> worth_proof::Sealed { finish(0) }",
        ),
        (
            "stdout-spoof",
            "pub(crate) fn sealed() -> worth_proof::Sealed { println!(\"spoof\"); worth_proof::issue() }",
        ),
    ] {
        let (governed, rows) = if label.starts_with("callback") {
            (CALLBACK_GOVERNED, callback_row())
        } else {
            (VALUE_GOVERNED, value_row())
        };
        assert_contract_denied(label, governed, source, rows, "");
    }
}

#[test]
fn retired_file_receipt_and_function_variable_exit_cannot_fake_completion() {
    assert_contract_denied(
        "retired-file-receipt",
        VALUE_GOVERNED,
        r#"
pub(crate) fn sealed() -> worth_proof::Sealed {
    let finish: fn(i32) -> ! = std::process::exit;
    let old_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("completed");
    std::fs::write(old_path, "1").expect("attempt retired completion receipt");
    finish(0)
}
"#,
        value_row(),
        "",
    );
}

#[test]
fn coverage_rows_fail_closed_when_missing_stale_or_duplicated() {
    assert_contract_denied("missing-row", "pub struct Sealed { value: u8 }", "", "", "");
    assert_contract_denied(
        "stale-row",
        "pub struct Sealed { value: u8 } pub fn issue()->Sealed{Sealed{value:1}}",
        "pub(crate) fn sealed()->worth_proof::Sealed{worth_proof::issue()}",
        &value_row().replace(
            "definition_path = \"Sealed\"",
            "definition_path = \"Missing\"",
        ),
        "",
    );
    assert_contract_denied(
        "duplicate-row",
        "pub struct Sealed { value: u8 } pub fn issue()->Sealed{Sealed{value:1}}",
        "pub(crate) fn sealed()->worth_proof::Sealed{worth_proof::issue()}",
        &format!("{}{}", value_row(), value_row()),
        "",
    );
    let duplicate_public_path = value_row().to_owned()
        + &value_row()
            .replace(
                "definition_path = \"Sealed\"",
                "definition_path = \"Other\"",
            )
            .replace("function = \"sealed\"", "function = \"other\"");
    assert_contract_denied(
        "duplicate-public-path",
        r#"
pub struct Sealed { value:u8 }
pub struct Other { value:u8 }
pub fn issue() -> Sealed { Sealed { value:1 } }
pub fn issue_other() -> Other { Other { value:2 } }
"#,
        r#"
pub(crate) fn sealed() -> worth_proof::Sealed { worth_proof::issue() }
pub(crate) fn other() -> worth_proof::Other { worth_proof::issue_other() }
"#,
        &duplicate_public_path,
        "",
    );
}

#[test]
fn rows_reject_wrong_type_posture_target_and_stale_functions() {
    let single = "pub struct Sealed { value: u8 } pub fn issue()->Sealed{Sealed{value:1}}";
    let source = "pub(crate) fn sealed()->worth_proof::Sealed{worth_proof::issue()}";
    assert_contract_denied(
        "wrong-type",
        single,
        source,
        &value_row().replace(
            "definition_path = \"Sealed\"",
            "definition_path = \"Other\"",
        ),
        "",
    );
    assert_contract_denied(
        "wrong-public-path",
        single,
        source,
        &value_row().replace(
            "public_type_path = \"::worth_proof::Sealed\"",
            "public_type_path = \"worth_proof::Sealed\"",
        ),
        "",
    );
    assert_contract_denied(
        "stale-public-path",
        single,
        source,
        &value_row().replace(
            "public_type_path = \"::worth_proof::Sealed\"",
            "public_type_path = \"::worth_proof::Missing\"",
        ),
        "",
    );
    assert_contract_denied(
        "wrong-posture",
        single,
        source,
        &value_row().replace("posture = \"value\"", "posture = \"callback\""),
        "",
    );
    assert_contract_denied(
        "unknown-target",
        single,
        source,
        &value_row().replace("worlds = [\"host-dev-default\"]", "worlds = [\"unknown\"]"),
        "",
    );
    assert_contract_denied(
        "stale-function",
        single,
        &format!("{source}\npub(crate) fn extra()->worth_proof::Sealed{{worth_proof::issue()}}"),
        value_row(),
        "",
    );
}

#[test]
fn public_value_contract_is_mandatory() {
    let repository = AuthoritySealingTestRepository::create("missing-public-value-contract");
    repository.assemble_without_public_value_contract();
    let (ok, output) = repository.run_boundary_check();
    repository.cleanup();
    assert!(!ok, "missing contract must fail closed:\n{output}");
}

#[test]
fn exact_definition_identity_rejects_same_named_public_value_exports() {
    let governed = r#"
pub mod first {
    pub struct Token { value: u8 }
    pub fn issue() -> Token { Token { value: 1 } }
}
pub mod second { pub struct Token { value: u8 } }
pub use first::Token;
pub use second::Token as OtherToken;
"#;
    let repository = AuthoritySealingTestRepository::create("ambiguous-definition-identity");
    repository.assemble_public_value_witness_contract(
        governed,
        "pub(crate) fn token()->worth_proof::Token{worth_proof::first::issue()}",
        &value_row()
            .replace(
                "definition_path = \"Sealed\"",
                "definition_path = \"first::Token\"",
            )
            .replace(
                "public_type_path = \"::worth_proof::Sealed\"",
                "public_type_path = \"::worth_proof::Token\"",
            )
            .replace("function = \"sealed\"", "function = \"token\""),
        "",
    );
    let (ok, output) = repository.run_boundary_check();
    repository.cleanup();
    assert!(
        !ok,
        "ambiguous definition names must fail closed:\n{output}"
    );
    assert!(
        output.contains("ambiguous across exact definitions"),
        "{output}"
    );
}

fn value_row() -> &'static str {
    r#"
[[rule_contracts.public_value_reachability.witnesses]]
definition_path = "Sealed"
function = "sealed"
public_type_path = "::worth_proof::Sealed"
posture = "value"
worlds = ["host-dev-default"]
"#
}

fn callback_row() -> &'static str {
    r#"
[[rule_contracts.public_value_reachability.witnesses]]
definition_path = "CallbackSealed"
function = "callback"
public_type_path = "::worth_proof::CallbackSealed"
posture = "callback"
worlds = ["host-dev-default"]
"#
}

fn assert_contract_allowed(
    label: &str,
    governed: &str,
    witness_source: &str,
    rows: &str,
    exemptions: &str,
) {
    let repository = AuthoritySealingTestRepository::create(label);
    repository.assemble_public_value_witness_contract(governed, witness_source, rows, exemptions);
    let (ok, output) = repository.run_boundary_check();
    repository.cleanup();
    assert!(ok, "{label} must pass:\n{output}");
}

fn assert_contract_denied(
    label: &str,
    governed: &str,
    witness_source: &str,
    rows: &str,
    exemptions: &str,
) {
    let repository = AuthoritySealingTestRepository::create(label);
    repository.assemble_public_value_witness_contract(governed, witness_source, rows, exemptions);
    let (ok, output) = repository.run_boundary_check();
    repository.cleanup();
    assert!(!ok, "{label} must fail closed:\n{output}");
    assert!(
        output.contains("BC7004_PUBLIC_VALUE_REACHABILITY")
            || output.contains("public-value witness"),
        "{label} failed for the wrong reason:\n{output}"
    );
}
