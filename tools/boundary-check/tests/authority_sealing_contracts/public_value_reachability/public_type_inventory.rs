use super::{assert_contract_allowed, assert_contract_denied, value_row};

#[test]
fn every_public_value_definition_requires_exact_coverage() {
    assert_contract_denied(
        "hidden-field-type-carrier",
        r#"
mod hidden { pub(super) struct Key; }
pub struct Sealed { pub key: hidden::Key }
"#,
        "",
        "",
        "",
    );

    assert_contract_allowed(
        "open-public-literal",
        "pub struct Sealed { pub value: u8 }",
        "pub(crate) fn sealed()->worth_proof::Sealed{worth_proof::Sealed{value:1}}",
        value_row(),
        "",
    );
}

#[test]
fn governed_worth_proof_inventory_has_77_witnesses_and_one_exemption() {
    let config = include_str!("../../../config/road1.toml");
    assert_eq!(
        config
            .matches("[[rule_contracts.public_value_reachability.witnesses]]")
            .count(),
        77
    );
    assert_eq!(
        config
            .matches("[[rule_contracts.public_value_reachability.exemptions]]")
            .count(),
        1
    );
}
