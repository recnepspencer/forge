//! Production-binary matrix: semantic authority-bound laundering is BC7001-denied.

use super::authority_sealing_fixture::{
    hostile_array_carrier_launder, hostile_blanket_impl_private_gate,
    hostile_blanket_impl_public_gate, hostile_blanket_impl_renamed_marker,
    hostile_blanket_impl_where_clause, hostile_carrier_plus_glob_combo, hostile_glob_alias_launder,
    hostile_multihop_blanket_launder, hostile_multihop_supertrait_bound,
    hostile_multiparam_wrapper_launder, hostile_private_glob_alias_launder,
    hostile_private_subtrait_bound, hostile_public_subtrait_bound, hostile_qualified_alias_launder,
    hostile_ref_carrier_launder, hostile_tuple_carrier_launder,
    hostile_where_clause_laundered_gate, hostile_wrapper_carrier_launder,
    legal_concrete_not_laundered, legal_non_authority_trait_bound, AuthoritySealingTestRepository,
};

fn assert_sealing_denial(label: &str, source: &str) {
    let repo = AuthoritySealingTestRepository::create(label);
    repo.assemble_with_lib_source(source);
    let (ok, output) = repo.run_boundary_check();
    repo.cleanup();
    assert!(!ok, "{label} must fail authority sealing:\n{output}");
    assert!(
        output.contains("BC7001_AUTHORITY_SEALING"),
        "{label}: expected BC7001, got:\n{output}"
    );
    assert!(
        output.contains("Authority sealing law"),
        "{label}: expected law quote, got:\n{output}"
    );
    assert!(
        output.contains("AuthorityWitness<ConcreteAuthority>")
            || output.contains("CapabilityWitness<ConcreteCapability>"),
        "{label}: expected concrete pattern, got:\n{output}"
    );
}

fn assert_sealing_pass(label: &str, source: &str) {
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
fn private_subtrait_of_authority_marker_is_denied() {
    assert_sealing_denial("hostile-private-subtrait", hostile_private_subtrait_bound());
}

#[test]
fn public_subtrait_of_authority_marker_is_denied() {
    assert_sealing_denial("hostile-public-subtrait", hostile_public_subtrait_bound());
}

#[test]
fn multihop_supertrait_chain_is_denied() {
    assert_sealing_denial(
        "hostile-multihop-super",
        hostile_multihop_supertrait_bound(),
    );
}

#[test]
fn blanket_impl_private_gate_launder_is_denied() {
    assert_sealing_denial(
        "hostile-blanket-private",
        hostile_blanket_impl_private_gate(),
    );
}

#[test]
fn blanket_impl_public_gate_launder_is_denied() {
    assert_sealing_denial("hostile-blanket-public", hostile_blanket_impl_public_gate());
}

#[test]
fn blanket_impl_where_clause_launder_is_denied() {
    assert_sealing_denial("hostile-blanket-where", hostile_blanket_impl_where_clause());
}

#[test]
fn blanket_impl_renamed_marker_launder_is_denied() {
    assert_sealing_denial(
        "hostile-blanket-rename",
        hostile_blanket_impl_renamed_marker(),
    );
}

#[test]
fn multihop_blanket_launder_is_denied() {
    assert_sealing_denial(
        "hostile-multihop-blanket",
        hostile_multihop_blanket_launder(),
    );
}

#[test]
fn where_clause_laundered_gate_on_ceremony_is_denied() {
    assert_sealing_denial(
        "hostile-where-laundered",
        hostile_where_clause_laundered_gate(),
    );
}

#[test]
fn non_authority_trait_bound_is_not_sealed() {
    assert_sealing_pass("legal-debug-bound", legal_non_authority_trait_bound());
}

#[test]
fn concrete_authority_ceremony_still_passes() {
    assert_sealing_pass(
        "legal-concrete-launder-control",
        legal_concrete_not_laundered(),
    );
}

#[test]
fn tuple_carrier_blanket_launder_is_denied() {
    assert_sealing_denial("hostile-tuple-carrier", hostile_tuple_carrier_launder());
}

#[test]
fn array_carrier_blanket_launder_is_denied() {
    assert_sealing_denial("hostile-array-carrier", hostile_array_carrier_launder());
}

#[test]
fn ref_carrier_blanket_launder_is_denied() {
    assert_sealing_denial("hostile-ref-carrier", hostile_ref_carrier_launder());
}

#[test]
fn wrapper_carrier_blanket_launder_is_denied() {
    assert_sealing_denial("hostile-wrapper-carrier", hostile_wrapper_carrier_launder());
}

#[test]
fn multiparam_wrapper_carrier_launder_is_denied() {
    assert_sealing_denial(
        "hostile-multiparam-wrapper",
        hostile_multiparam_wrapper_launder(),
    );
}

#[test]
fn glob_alias_launder_is_denied() {
    assert_sealing_denial("hostile-glob-alias", hostile_glob_alias_launder());
}

#[test]
fn qualified_alias_launder_is_denied() {
    assert_sealing_denial("hostile-qualified-alias", hostile_qualified_alias_launder());
}

#[test]
fn private_glob_alias_launder_is_denied() {
    assert_sealing_denial(
        "hostile-private-glob-alias",
        hostile_private_glob_alias_launder(),
    );
}

#[test]
fn carrier_plus_glob_combo_is_denied() {
    assert_sealing_denial(
        "hostile-carrier-glob-combo",
        hostile_carrier_plus_glob_combo(),
    );
}
