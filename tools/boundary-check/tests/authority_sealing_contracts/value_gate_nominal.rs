//! Nominal producer-graph coverage for aliases and owning wrappers.

use super::authority_sealing_fixture::{
    hostile_public_alias_to_private_marker_factory, hostile_public_enum_wrapper_marker_factory,
    hostile_public_tuple_wrapper_marker_factory, legal_private_wrapper_marker_factory,
};
use super::value_gate::{assert_value_gate_denial, assert_value_gate_pass};

#[test]
fn public_alias_to_private_marker_factory_is_denied() {
    assert_value_gate_denial(
        "hostile-public-alias-private-marker",
        hostile_public_alias_to_private_marker_factory(),
    );
}

#[test]
fn public_tuple_wrapper_payload_is_denied() {
    assert_value_gate_denial(
        "hostile-public-tuple-wrapper",
        hostile_public_tuple_wrapper_marker_factory(),
    );
}

#[test]
fn public_enum_wrapper_payload_is_denied() {
    assert_value_gate_denial(
        "hostile-public-enum-wrapper",
        hostile_public_enum_wrapper_marker_factory(),
    );
}

#[test]
fn private_wrapper_payload_does_not_mint_marker() {
    assert_value_gate_pass(
        "legal-private-wrapper",
        legal_private_wrapper_marker_factory(),
    );
}
