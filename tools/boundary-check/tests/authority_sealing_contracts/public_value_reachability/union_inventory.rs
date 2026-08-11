use super::{assert_contract_allowed, assert_contract_denied, value_row};

const UNION_ROW: &str = r#"
[[rule_contracts.public_value_reachability.witnesses]]
definition_path = "SealedUnion"
function = "sealed_union"
public_type_path = "::worth_proof::SealedUnion"
posture = "value"
worlds = ["host-dev-default"]
"#;

#[test]
fn public_union_with_only_private_fields_requires_a_witness() {
    assert_contract_denied(
        "sealed-union-without-producer",
        "pub union SealedUnion { value: u8 }",
        "",
        "",
        "",
    );
}

#[test]
fn public_union_with_a_public_field_is_downstream_constructible() {
    assert_contract_allowed(
        "mixed-union",
        r#"
pub union MixedUnion { pub exposed: u8, hidden: u16 }
pub struct Sealed { value: u8 }
pub fn issue() -> Sealed { Sealed { value: 1 } }
"#,
        r#"
pub(crate) fn sealed() -> worth_proof::Sealed {
    worth_proof::issue()
}
pub(crate) fn mixed_union() -> worth_proof::MixedUnion {
    worth_proof::MixedUnion { exposed: 1 }
}
"#,
        &format!(
            r#"{}
[[rule_contracts.public_value_reachability.witnesses]]
definition_path = "MixedUnion"
function = "mixed_union"
public_type_path = "::worth_proof::MixedUnion"
posture = "value"
worlds = ["host-dev-default"]
"#,
            value_row()
        ),
        "",
    );
}

#[test]
fn public_union_with_private_field_accepts_real_public_producer() {
    assert_contract_allowed(
        "sealed-union-with-producer",
        r#"
pub union SealedUnion { value: u8 }
pub fn issue_union() -> SealedUnion { SealedUnion { value: 1 } }
"#,
        r#"
pub(crate) fn sealed_union() -> worth_proof::SealedUnion {
    worth_proof::issue_union()
}
"#,
        UNION_ROW,
        "",
    );
}
