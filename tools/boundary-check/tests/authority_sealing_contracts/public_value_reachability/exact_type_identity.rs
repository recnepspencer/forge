use super::{assert_contract_allowed, assert_contract_denied, callback_row, value_row};

#[test]
fn compiler_binds_value_witness_to_exact_external_type() {
    assert_contract_denied(
        "value-shadow",
        "pub struct Sealed { value:u8 }",
        r#"
mod worth_proof { pub struct Sealed; }
pub(crate) fn sealed() -> worth_proof::Sealed { worth_proof::Sealed }
"#,
        value_row(),
        "",
    );
}

#[test]
fn compiler_binds_callback_witness_to_exact_external_type() {
    assert_contract_denied(
        "callback-shadow",
        "pub struct CallbackSealed { value:u8 }",
        r#"
mod worth_proof { pub struct CallbackSealed; }
pub(crate) fn callback(deliver: impl FnOnce(worth_proof::CallbackSealed)) {
    deliver(worth_proof::CallbackSealed);
}
"#,
        callback_row(),
        "",
    );
}

#[test]
fn exact_public_reexport_resolves_to_inventoried_definition() {
    assert_contract_allowed(
        "public-reexport-exact-type",
        r#"
mod owner {
    pub struct Sealed { value:u8 }
    pub fn issue() -> Sealed { Sealed { value:1 } }
}
pub use owner::Sealed as PublicSealed;
pub fn issue() -> PublicSealed { owner::issue() }
"#,
        "pub(crate) fn sealed() -> worth_proof::PublicSealed { worth_proof::issue() }",
        &value_row()
            .replace(
                "definition_path = \"Sealed\"",
                "definition_path = \"owner::Sealed\"",
            )
            .replace(
                "public_type_path = \"::worth_proof::Sealed\"",
                "public_type_path = \"::worth_proof::PublicSealed\"",
            ),
        "",
    );
}
