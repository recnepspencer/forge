//! Associated-projection cases for the BC7001 public-producer graph.

use super::authority_sealing_fixture::{
    hostile_alias_associated_projection_factory, hostile_alias_chain_associated_projection_factory,
    hostile_blanket_associated_projection_factory, hostile_generic_output_substitution_factory,
    hostile_long_alias_chain_associated_projection_factory,
    hostile_primitive_associated_projection_factory,
    hostile_primitive_trait_associated_marker_factory, hostile_reexported_trait_projection_factory,
    hostile_satisfied_local_constraint_projection, hostile_satisfied_where_constraint_projection,
    legal_primitive_associated_projection_borrow, legal_primitive_trait_borrowed_marker_factory,
    legal_trait_argument_mismatch_projection, legal_unsatisfied_local_constraint_projection,
    legal_wrong_qualified_associated_projection,
};
use super::value_gate::{assert_value_gate_denial, assert_value_gate_pass};

#[test]
fn primitive_trait_associated_marker_factory_is_denied() {
    assert_value_gate_denial(
        "hostile-primitive-trait-associated-marker",
        hostile_primitive_trait_associated_marker_factory(),
    );
}

#[test]
fn primitive_alias_projection_factory_is_denied() {
    assert_value_gate_denial(
        "hostile-alias-associated-projection-factory",
        hostile_alias_associated_projection_factory(),
    );
}

#[test]
fn qualified_alias_chain_projection_factory_is_denied() {
    assert_value_gate_denial(
        "hostile-alias-chain-associated-projection-factory",
        hostile_alias_chain_associated_projection_factory(),
    );
}

#[test]
fn long_alias_chain_projection_factory_is_denied() {
    assert_value_gate_denial(
        "hostile-long-alias-chain-associated-projection-factory",
        hostile_long_alias_chain_associated_projection_factory(),
    );
}

#[test]
fn reexported_trait_projection_factory_is_denied() {
    assert_value_gate_denial(
        "hostile-reexported-trait-associated-projection-factory",
        hostile_reexported_trait_projection_factory(),
    );
}

#[test]
fn blanket_projection_factory_is_denied() {
    assert_value_gate_denial(
        "hostile-blanket-associated-projection-factory",
        hostile_blanket_associated_projection_factory(),
    );
}

#[test]
fn generic_projection_output_substitution_is_denied() {
    assert_value_gate_denial(
        "hostile-generic-output-substitution-factory",
        hostile_generic_output_substitution_factory(),
    );
}

#[test]
fn satisfied_local_constraint_projection_factory_is_denied() {
    assert_value_gate_denial(
        "hostile-satisfied-local-constraint-projection",
        hostile_satisfied_local_constraint_projection(),
    );
}

#[test]
fn satisfied_where_constraint_projection_factory_is_denied() {
    assert_value_gate_denial(
        "hostile-satisfied-where-constraint-projection",
        hostile_satisfied_where_constraint_projection(),
    );
}

#[test]
fn primitive_associated_projection_free_factory_is_denied() {
    assert_value_gate_denial(
        "hostile-primitive-associated-projection-factory",
        hostile_primitive_associated_projection_factory(),
    );
}

#[test]
fn primitive_associated_projection_borrow_does_not_mint_owned_authority() {
    assert_value_gate_pass(
        "legal-primitive-associated-projection-borrow",
        legal_primitive_associated_projection_borrow(),
    );
}

#[test]
fn wrong_qualified_associated_projection_does_not_mint_the_marker() {
    assert_value_gate_pass(
        "legal-wrong-qualified-associated-projection",
        legal_wrong_qualified_associated_projection(),
    );
}

#[test]
fn primitive_trait_borrowed_marker_factory_does_not_mint_owned_authority() {
    assert_value_gate_pass(
        "legal-primitive-trait-borrowed-marker",
        legal_primitive_trait_borrowed_marker_factory(),
    );
}

#[test]
fn trait_argument_mismatch_does_not_associate_an_unselected_marker_output() {
    assert_value_gate_pass(
        "legal-trait-argument-mismatch-projection",
        legal_trait_argument_mismatch_projection(),
    );
}

#[test]
fn unsatisfied_local_constraint_does_not_associate_a_marker_output() {
    assert_value_gate_pass(
        "legal-unsatisfied-local-constraint-projection",
        legal_unsatisfied_local_constraint_projection(),
    );
}
