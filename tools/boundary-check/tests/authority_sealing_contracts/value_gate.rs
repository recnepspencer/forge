//! Production-binary proofs for concrete ceremony marker value-gating.

use super::authority_sealing_fixture::{
    hostile_aliased_carrier, hostile_aliased_marker_factory, hostile_array_wrapped_self_factory,
    hostile_associated_output_marker_factory, hostile_box_wrapped_marker_factory,
    hostile_boxed_function_marker_factory, hostile_boxed_iterator_marker_factory,
    hostile_default_impl_marker, hostile_derived_default_marker, hostile_enum_empty_named_marker,
    hostile_enum_empty_tuple_marker, hostile_enum_unit_marker,
    hostile_function_pointer_alias_const, hostile_impl_trait_marker_factory,
    hostile_method_unit_marker, hostile_option_wrapped_trait_factory,
    hostile_public_constructor_marker, hostile_public_fields_marker,
    hostile_public_free_marker_factory, hostile_public_marker_const,
    hostile_qualified_derived_default_marker, hostile_qualified_exact_marker_factory,
    hostile_qualified_same_named_marker, hostile_result_wrapped_self_factory,
    hostile_same_named_phantom_wrapper_factory, hostile_trait_marker_factory,
    hostile_trait_self_marker_factory, hostile_type_aliased_carrier,
    hostile_unit_marker_authority_witness, hostile_unrelated_impl_marker_factory,
    hostile_wrapped_alias_marker_factory, hostile_wrapped_public_marker_const,
    hostile_wrapped_trait_declaration_factory, legal_borrowed_marker_factory,
    legal_borrowed_trait_object_factory, legal_phantom_marker_metadata_factory,
    legal_reexported_value_gated_marker, legal_same_named_foreign_carrier,
    legal_unrelated_self_factory, legal_value_gated_authority_witness,
    legal_value_gated_capability_and_proof, legal_wrong_qualified_same_named_factory,
    AuthoritySealingTestRepository,
};

pub(super) fn assert_value_gate_denial(label: &str, source: &str) {
    let repo = AuthoritySealingTestRepository::create(label);
    repo.assemble_with_lib_source(source);
    let (ok, output) = repo.run_boundary_check();
    repo.cleanup();
    assert!(!ok, "{label} must fail value-gate sealing:\n{output}");
    assert!(
        output.contains("BC7001_AUTHORITY_SEALING"),
        "{label}: expected BC7001, got:\n{output}"
    );
    assert!(
        output.contains("Authority sealing law"),
        "{label}: expected law quote, got:\n{output}"
    );
    assert!(
        output.contains("value-gated") || output.contains("mintable marker"),
        "{label}: expected value-gate wording, got:\n{output}"
    );
    assert!(
        output.contains("AuthorityWitness<ConcreteAuthority>")
            || output.contains("CapabilityWitness<ConcreteCapability>"),
        "{label}: expected concrete pattern, got:\n{output}"
    );
}

pub(super) fn assert_value_gate_pass(label: &str, source: &str) {
    let repo = AuthoritySealingTestRepository::create(label);
    repo.assemble_with_lib_source(source);
    let (ok, output) = repo.run_boundary_check();
    repo.cleanup();
    assert!(ok, "{label} must pass:\n{output}");
    assert!(
        !output.contains("BC7001_AUTHORITY_SEALING"),
        "{label}: unexpected sealing diagnostic:\n{output}"
    );
}

#[test]
fn value_gated_authority_witness_ceremony_passes() {
    assert_value_gate_pass(
        "legal-value-gate-auth",
        legal_value_gated_authority_witness(),
    );
}

#[test]
fn value_gated_capability_and_proof_ceremonies_pass() {
    assert_value_gate_pass(
        "legal-value-gate-cap-proof",
        legal_value_gated_capability_and_proof(),
    );
}

#[test]
fn unit_marker_on_authority_witness_is_denied() {
    assert_value_gate_denial(
        "hostile-unit-marker",
        hostile_unit_marker_authority_witness(),
    );
}

#[test]
fn public_fields_marker_is_denied() {
    assert_value_gate_denial("hostile-pub-fields-marker", hostile_public_fields_marker());
}

#[test]
fn default_impl_marker_is_denied() {
    assert_value_gate_denial("hostile-default-marker", hostile_default_impl_marker());
}

#[test]
fn derived_default_marker_is_denied() {
    assert_value_gate_denial("hostile-derived-default", hostile_derived_default_marker());
}

#[test]
fn public_constructor_marker_is_denied() {
    assert_value_gate_denial("hostile-ctor-marker", hostile_public_constructor_marker());
}

#[test]
fn public_free_marker_factory_is_denied() {
    assert_value_gate_denial(
        "hostile-free-marker-factory",
        hostile_public_free_marker_factory(),
    );
}

#[test]
fn aliased_marker_factory_is_denied() {
    assert_value_gate_denial(
        "hostile-aliased-marker-factory",
        hostile_aliased_marker_factory(),
    );
}

#[test]
fn wrapped_alias_marker_factory_is_denied() {
    assert_value_gate_denial(
        "hostile-wrapped-alias-factory",
        hostile_wrapped_alias_marker_factory(),
    );
}

#[test]
fn function_pointer_alias_const_is_denied() {
    assert_value_gate_denial(
        "hostile-function-pointer-alias",
        hostile_function_pointer_alias_const(),
    );
}

#[test]
fn impl_trait_marker_factory_is_denied() {
    assert_value_gate_denial(
        "hostile-impl-trait-marker-factory",
        hostile_impl_trait_marker_factory(),
    );
}

#[test]
fn qualified_derived_default_marker_is_denied() {
    assert_value_gate_denial(
        "hostile-qualified-derived-default",
        hostile_qualified_derived_default_marker(),
    );
}

#[test]
fn wrapped_trait_declaration_factory_is_denied() {
    assert_value_gate_denial(
        "hostile-wrapped-trait-declaration",
        hostile_wrapped_trait_declaration_factory(),
    );
}

#[test]
fn enum_unit_marker_is_denied() {
    assert_value_gate_denial("hostile-enum-unit-marker", hostile_enum_unit_marker());
}

#[test]
fn enum_empty_named_marker_is_denied() {
    assert_value_gate_denial(
        "hostile-enum-empty-named",
        hostile_enum_empty_named_marker(),
    );
}

#[test]
fn enum_empty_tuple_marker_is_denied() {
    assert_value_gate_denial(
        "hostile-enum-empty-tuple",
        hostile_enum_empty_tuple_marker(),
    );
}

#[test]
fn method_ceremony_unit_marker_is_denied() {
    assert_value_gate_denial("hostile-method-unit-marker", hostile_method_unit_marker());
}

#[test]
fn cargo_identified_carrier_alias_is_denied() {
    assert_value_gate_denial("hostile-carrier-alias", hostile_aliased_carrier());
}

#[test]
fn type_aliased_platform_carrier_is_denied() {
    assert_value_gate_denial("hostile-type-carrier-alias", hostile_type_aliased_carrier());
}

#[test]
fn same_named_non_platform_carrier_is_ignored() {
    assert_value_gate_pass(
        "legal-same-name-carrier",
        legal_same_named_foreign_carrier(),
    );
}

#[test]
fn reexported_value_gated_marker_preserves_identity() {
    assert_value_gate_pass(
        "legal-reexported-marker",
        legal_reexported_value_gated_marker(),
    );
}

#[test]
fn qualified_marker_uses_its_definition_identity() {
    assert_value_gate_denial(
        "hostile-qualified-marker",
        hostile_qualified_same_named_marker(),
    );
}

#[test]
fn public_marker_constant_is_denied() {
    assert_value_gate_denial("hostile-marker-const", hostile_public_marker_const());
}

#[test]
fn wrapped_public_marker_constant_is_denied() {
    assert_value_gate_denial(
        "hostile-wrapped-marker-const",
        hostile_wrapped_public_marker_const(),
    );
}

#[test]
fn associated_output_marker_factory_is_denied() {
    assert_value_gate_denial(
        "hostile-associated-output-factory",
        hostile_associated_output_marker_factory(),
    );
}

#[test]
fn public_trait_marker_factory_is_denied() {
    assert_value_gate_denial("hostile-trait-factory", hostile_trait_marker_factory());
}

#[test]
fn unrelated_inherent_marker_factory_is_denied() {
    assert_value_gate_denial(
        "hostile-unrelated-impl-factory",
        hostile_unrelated_impl_marker_factory(),
    );
}

#[test]
fn trait_self_marker_factory_is_denied() {
    assert_value_gate_denial(
        "hostile-trait-self-factory",
        hostile_trait_self_marker_factory(),
    );
}

#[test]
fn unrelated_self_factory_does_not_mint_the_marker() {
    assert_value_gate_pass(
        "legal-unrelated-self-factory",
        legal_unrelated_self_factory(),
    );
}

#[test]
fn wrong_qualified_same_named_factory_does_not_mint_the_marker() {
    assert_value_gate_pass(
        "legal-wrong-qualified-same-name-factory",
        legal_wrong_qualified_same_named_factory(),
    );
}

#[test]
fn qualified_exact_marker_factory_is_denied() {
    assert_value_gate_denial(
        "hostile-qualified-exact-factory",
        hostile_qualified_exact_marker_factory(),
    );
}

#[test]
fn result_wrapped_self_factory_is_denied() {
    assert_value_gate_denial(
        "hostile-result-wrapped-self-factory",
        hostile_result_wrapped_self_factory(),
    );
}

#[test]
fn option_wrapped_trait_factory_is_denied() {
    assert_value_gate_denial(
        "hostile-option-wrapped-trait-factory",
        hostile_option_wrapped_trait_factory(),
    );
}

#[test]
fn box_wrapped_marker_factory_is_denied() {
    assert_value_gate_denial(
        "hostile-box-wrapped-marker-factory",
        hostile_box_wrapped_marker_factory(),
    );
}

#[test]
fn array_wrapped_self_factory_is_denied() {
    assert_value_gate_denial(
        "hostile-array-wrapped-self-factory",
        hostile_array_wrapped_self_factory(),
    );
}

#[test]
fn borrowed_marker_factory_does_not_mint_owned_authority() {
    assert_value_gate_pass(
        "legal-borrowed-marker-factory",
        legal_borrowed_marker_factory(),
    );
}

#[test]
fn boxed_iterator_marker_factory_is_denied() {
    assert_value_gate_denial(
        "hostile-boxed-iterator-marker-factory",
        hostile_boxed_iterator_marker_factory(),
    );
}

#[test]
fn boxed_function_marker_factory_is_denied() {
    assert_value_gate_denial(
        "hostile-boxed-function-marker-factory",
        hostile_boxed_function_marker_factory(),
    );
}

#[test]
fn borrowed_trait_object_factory_does_not_mint_owned_authority() {
    assert_value_gate_pass(
        "legal-borrowed-trait-object-factory",
        legal_borrowed_trait_object_factory(),
    );
}

#[test]
fn phantom_marker_metadata_does_not_mint_owned_authority() {
    assert_value_gate_pass(
        "PhantomData carries type metadata without producing an owned marker",
        legal_phantom_marker_metadata_factory(),
    );
}

#[test]
fn same_named_owning_phantom_wrapper_is_denied() {
    assert_value_gate_denial(
        "hostile-same-named-owning-phantom-wrapper",
        hostile_same_named_phantom_wrapper_factory(),
    );
}
