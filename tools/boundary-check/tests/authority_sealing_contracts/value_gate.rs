//! Production-binary proofs for concrete ceremony marker value-gating.

use super::authority_sealing_fixture::{
    hostile_aliased_carrier, hostile_default_impl_marker, hostile_enum_unit_marker,
    hostile_method_unit_marker, hostile_public_constructor_marker, hostile_public_fields_marker,
    hostile_public_marker_const, hostile_qualified_same_named_marker, hostile_trait_marker_factory,
    hostile_type_aliased_carrier, hostile_unit_marker_authority_witness,
    legal_reexported_value_gated_marker, legal_same_named_foreign_carrier,
    legal_value_gated_authority_witness, legal_value_gated_capability_and_proof,
    AuthoritySealingTestRepository,
};

fn assert_value_gate_denial(label: &str, source: &str) {
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

fn assert_value_gate_pass(label: &str, source: &str) {
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
fn public_constructor_marker_is_denied() {
    assert_value_gate_denial("hostile-ctor-marker", hostile_public_constructor_marker());
}

#[test]
fn enum_unit_marker_is_denied() {
    assert_value_gate_denial("hostile-enum-unit-marker", hostile_enum_unit_marker());
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
fn public_trait_marker_factory_is_denied() {
    assert_value_gate_denial("hostile-trait-factory", hostile_trait_marker_factory());
}
