use super::{assert_contract_allowed, assert_contract_denied};

const EXEMPTION: &str = r#"
[[rule_contracts.public_value_reachability.exemptions]]
type_path = "Marker"
posture = "uninhabited_type"
reason = "zero-variant enum cannot exist as a value"
"#;

#[test]
fn zero_variant_enum_is_an_uninhabited_exemption_regardless_of_type_uses() {
    assert_contract_allowed(
        "uninhabited-type",
        r#"
pub enum Marker {}
pub struct Carrier(pub Vec<Marker>);
pub trait RuntimeType { type Value; fn consume(value: Self::Value); }
pub struct Owner;
impl RuntimeType for Owner {
    type Value = Marker;
    fn consume(_: Self::Value) {}
}
"#,
        r#"
pub(crate) fn carrier() -> worth_proof::Carrier { worth_proof::Carrier(Vec::new()) }
pub(crate) fn owner() -> worth_proof::Owner { worth_proof::Owner }
"#,
        r#"
[[rule_contracts.public_value_reachability.witnesses]]
definition_path = "Carrier"
function = "carrier"
public_type_path = "::worth_proof::Carrier"
posture = "value"
worlds = ["host-dev-default"]

[[rule_contracts.public_value_reachability.witnesses]]
definition_path = "Owner"
function = "owner"
public_type_path = "::worth_proof::Owner"
posture = "value"
worlds = ["host-dev-default"]
"#,
        EXEMPTION,
    );
}

#[test]
fn constructible_representations_cannot_claim_uninhabited_exemption() {
    for (label, source) in [
        ("constructible-struct", "pub struct Marker { value: u8 }"),
        ("constructible-unit-struct", "pub struct Marker;"),
        ("constructible-enum", "pub enum Marker { Value }"),
    ] {
        assert_contract_denied(label, source, "", "", EXEMPTION);
    }
}
