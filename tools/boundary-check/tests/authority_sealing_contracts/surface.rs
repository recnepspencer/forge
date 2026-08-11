//! Production-binary legal/hostile matrix for the authority sealing law.

use super::authority_sealing_fixture::{
    entry_reexport_external_admit, entry_reexport_external_admit_renamed,
    entry_reexport_external_group, external_hostile_aliased_marker_admit,
    external_hostile_generic_admit, external_legal_concrete_admit, hostile_authority_marker_bound,
    hostile_authority_proves_bound, hostile_capability_where_clause, hostile_const_type_carrier,
    hostile_custom_derive_on_public_type, hostile_enum_named_field_carrier,
    hostile_impl_trait_return, hostile_macro_expanded_public_fn,
    hostile_macro_export_generic_ceremony, hostile_nested_cfg_attr_custom_derive,
    hostile_nested_cfg_attr_opaque, hostile_nested_reexport_method,
    hostile_opaque_attribute_on_public_fn, hostile_parent_alias_in_child_module,
    hostile_private_trait_alias_bound, hostile_proof_set_authorized_by, hostile_pub_extern_crate,
    hostile_public_field_carrier, hostile_public_method_impl_trait_param,
    hostile_reexport_promotion, hostile_renamed_import, hostile_trait_associated_type_bound,
    hostile_transitive_use_alias, hostile_type_alias_chain_owned_method,
    hostile_type_alias_owned_method, hostile_type_alias_rhs, legal_cfg_attr_safe_nested,
    legal_concrete_direct, legal_concrete_reexport, legal_private_macro_rules_with_marker,
    legal_private_type_alias_method, legal_std_derive_on_public_type,
    private_generic_bound_is_legal, private_type_public_method_is_legal,
    AuthoritySealingTestRepository,
};

const EXTERNAL_PACKAGE: &str = "worth-schema-external";

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

fn assert_external_reexport_denial(label: &str, entry_lib: &str, external_lib: &str) {
    let repo = AuthoritySealingTestRepository::create(label);
    repo.assemble_with_external_path_dependency(entry_lib, EXTERNAL_PACKAGE, external_lib);
    let (ok, output) = repo.run_boundary_check();
    repo.cleanup();
    assert!(
        !ok,
        "{label} must fail external re-export sealing:\n{output}"
    );
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
            || output.contains("public re-export"),
        "{label}: expected sealing pattern or re-export fence, got:\n{output}"
    );
}

fn assert_external_reexport_pass(label: &str, entry_lib: &str, external_lib: &str) {
    let repo = AuthoritySealingTestRepository::create(label);
    repo.assemble_with_external_path_dependency(entry_lib, EXTERNAL_PACKAGE, external_lib);
    let (ok, output) = repo.run_boundary_check();
    repo.cleanup();
    assert!(ok, "{label} must pass:\n{output}");
    assert!(
        !output.contains("BC7001_AUTHORITY_SEALING"),
        "{label}: unexpected sealing diagnostic:\n{output}"
    );
}

#[test]
fn legal_concrete_direct_signature_passes() {
    assert_sealing_pass("legal-direct", legal_concrete_direct());
}

#[test]
fn legal_concrete_reexport_passes_identically() {
    assert_sealing_pass("legal-reexport", legal_concrete_reexport());
}

#[test]
fn private_generic_authority_bound_is_not_governed_public() {
    assert_sealing_pass("private-generic", private_generic_bound_is_legal());
}

#[test]
fn public_authority_marker_bound_is_denied_with_law_and_pattern() {
    assert_sealing_denial("hostile-auth-marker", hostile_authority_marker_bound());
}

#[test]
fn public_capability_where_clause_is_denied() {
    assert_sealing_denial("hostile-cap-where", hostile_capability_where_clause());
}

#[test]
fn public_authority_proves_bound_is_denied() {
    assert_sealing_denial("hostile-auth-proves", hostile_authority_proves_bound());
}

#[test]
fn public_proof_set_authorized_by_is_denied() {
    assert_sealing_denial("hostile-proof-set", hostile_proof_set_authorized_by());
}

#[test]
fn renamed_authority_marker_import_is_denied() {
    assert_sealing_denial("hostile-rename", hostile_renamed_import());
}

#[test]
fn reexport_promotion_of_generic_authority_fn_is_denied() {
    assert_sealing_denial("hostile-reexport", hostile_reexport_promotion());
}

#[test]
fn impl_trait_authority_marker_return_is_denied() {
    assert_sealing_denial("hostile-impl-trait", hostile_impl_trait_return());
}

#[test]
fn public_method_impl_trait_param_is_denied() {
    assert_sealing_denial(
        "hostile-method-impl-trait",
        hostile_public_method_impl_trait_param(),
    );
}

#[test]
fn public_trait_associated_type_bound_is_denied() {
    assert_sealing_denial("hostile-assoc-type", hostile_trait_associated_type_bound());
}

#[test]
fn public_field_carrier_is_denied() {
    assert_sealing_denial("hostile-field-carrier", hostile_public_field_carrier());
}

#[test]
fn public_type_alias_rhs_is_denied() {
    assert_sealing_denial("hostile-type-alias", hostile_type_alias_rhs());
}

#[test]
fn public_const_type_carrier_is_denied() {
    assert_sealing_denial("hostile-const-carrier", hostile_const_type_carrier());
}

#[test]
fn item_macro_expanded_public_ceremony_is_denied() {
    assert_sealing_denial("hostile-macro-expand", hostile_macro_expanded_public_fn());
}

#[test]
fn nested_reexport_method_ceremony_is_denied() {
    assert_sealing_denial("hostile-nested-method", hostile_nested_reexport_method());
}

#[test]
fn private_type_public_method_is_not_governed() {
    assert_sealing_pass("private-type-method", private_type_public_method_is_legal());
}

#[test]
fn external_direct_reexport_of_generic_authority_is_denied() {
    assert_external_reexport_denial(
        "hostile-ext-direct",
        entry_reexport_external_admit(),
        external_hostile_generic_admit(),
    );
}

#[test]
fn external_renamed_reexport_of_generic_authority_is_denied() {
    assert_external_reexport_denial(
        "hostile-ext-rename",
        entry_reexport_external_admit_renamed(),
        external_hostile_generic_admit(),
    );
}

#[test]
fn external_group_reexport_of_generic_authority_is_denied() {
    assert_external_reexport_denial(
        "hostile-ext-group",
        entry_reexport_external_group(),
        external_hostile_generic_admit(),
    );
}

#[test]
fn external_direct_reexport_of_concrete_ceremony_passes() {
    assert_external_reexport_pass(
        "legal-ext-direct",
        entry_reexport_external_admit(),
        external_legal_concrete_admit(),
    );
}

#[test]
fn external_renamed_reexport_of_concrete_ceremony_passes() {
    assert_external_reexport_pass(
        "legal-ext-rename",
        entry_reexport_external_admit_renamed(),
        external_legal_concrete_admit(),
    );
}

#[test]
fn external_group_reexport_of_concrete_ceremony_passes() {
    assert_external_reexport_pass(
        "legal-ext-group",
        entry_reexport_external_group(),
        external_legal_concrete_admit(),
    );
}

#[test]
fn macro_export_generic_ceremony_body_is_denied() {
    assert_sealing_denial(
        "hostile-macro-export",
        hostile_macro_export_generic_ceremony(),
    );
}

#[test]
fn private_macro_rules_with_marker_is_not_governed() {
    assert_sealing_pass(
        "legal-private-macro",
        legal_private_macro_rules_with_marker(),
    );
}

#[test]
fn opaque_attribute_on_public_fn_is_denied() {
    assert_sealing_denial(
        "hostile-opaque-attr",
        hostile_opaque_attribute_on_public_fn(),
    );
}

#[test]
fn custom_derive_on_public_type_is_denied() {
    assert_sealing_denial(
        "hostile-custom-derive",
        hostile_custom_derive_on_public_type(),
    );
}

#[test]
fn std_derive_on_public_type_passes() {
    assert_sealing_pass("legal-std-derive", legal_std_derive_on_public_type());
}

#[test]
fn transitive_use_alias_of_authority_marker_is_denied() {
    assert_sealing_denial("hostile-transitive-alias", hostile_transitive_use_alias());
}

#[test]
fn private_trait_alias_bound_on_public_fn_is_denied() {
    assert_sealing_denial("hostile-trait-alias", hostile_private_trait_alias_bound());
}

#[test]
fn parent_module_alias_rebinding_in_child_is_denied() {
    assert_sealing_denial(
        "hostile-parent-alias-child",
        hostile_parent_alias_in_child_module(),
    );
}

#[test]
fn nested_cfg_attr_opaque_attribute_is_denied() {
    assert_sealing_denial("hostile-nested-cfg-attr", hostile_nested_cfg_attr_opaque());
}

#[test]
fn nested_cfg_attr_custom_derive_is_denied() {
    assert_sealing_denial(
        "hostile-nested-cfg-derive",
        hostile_nested_cfg_attr_custom_derive(),
    );
}

#[test]
fn legal_cfg_attr_with_safe_nested_attrs_passes() {
    assert_sealing_pass("legal-cfg-attr-safe", legal_cfg_attr_safe_nested());
}

#[test]
fn external_reexport_of_aliased_marker_ceremony_is_denied() {
    assert_external_reexport_denial(
        "hostile-ext-alias",
        entry_reexport_external_admit(),
        external_hostile_aliased_marker_admit(),
    );
}

#[test]
fn type_alias_owned_method_is_denied() {
    assert_sealing_denial(
        "hostile-alias-owned-method",
        hostile_type_alias_owned_method(),
    );
}

#[test]
fn type_alias_chain_owned_method_is_denied() {
    assert_sealing_denial(
        "hostile-alias-chain-method",
        hostile_type_alias_chain_owned_method(),
    );
}

#[test]
fn enum_named_field_carrier_is_denied() {
    assert_sealing_denial(
        "hostile-enum-named-field",
        hostile_enum_named_field_carrier(),
    );
}

#[test]
fn public_extern_crate_is_denied() {
    assert_sealing_denial("hostile-pub-extern-crate", hostile_pub_extern_crate());
}

#[test]
fn private_type_alias_owned_method_is_not_governed() {
    assert_sealing_pass(
        "legal-private-alias-method",
        legal_private_type_alias_method(),
    );
}
