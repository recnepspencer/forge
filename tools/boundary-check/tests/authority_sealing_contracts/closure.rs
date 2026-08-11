//! Production-binary matrix for externally-callable surface closure (BC7001).

use super::authority_sealing_fixture::{
    entry_reexport_external_module, entry_reexport_external_type, external_hostile_module_admit,
    external_hostile_module_glob, external_hostile_type_method_admit, external_legal_module_admit,
    external_legal_type_method_admit, hostile_foreign_fn_authority_marker,
    hostile_foreign_static_capability_marker, hostile_impl_macro_member,
    hostile_macro_export_trait_bound_template, hostile_opaque_attr_on_foreign_fn,
    hostile_opaque_attr_on_impl, hostile_opaque_attr_on_impl_method,
    hostile_opaque_attr_on_trait_member, legal_foreign_fn_concrete_authority,
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
            || output.contains("CapabilityWitness<ConcreteCapability>")
            || output.contains("opaque")
            || output.contains("macro"),
        "{label}: expected concrete pattern or macro fence, got:\n{output}"
    );
}

fn assert_external_reexport_denial(label: &str, entry_lib: &str, external_lib: &str) -> String {
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
    output
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
fn external_module_reexport_of_generic_ceremony_is_denied() {
    assert_external_reexport_denial(
        "hostile-ext-module",
        entry_reexport_external_module(),
        external_hostile_module_admit(),
    );
}

#[test]
fn external_module_with_internal_glob_requires_explicit_exports() {
    let output = assert_external_reexport_denial(
        "hostile-ext-internal-glob",
        entry_reexport_external_module(),
        external_hostile_module_glob(),
    );
    assert!(
        output.contains("authority-governed dependencies must use named imports and reexports"),
        "expected dependency glob diagnostic:\n{output}"
    );
}

#[test]
fn external_type_reexport_of_method_ceremony_is_denied() {
    assert_external_reexport_denial(
        "hostile-ext-type-method",
        entry_reexport_external_type(),
        external_hostile_type_method_admit(),
    );
}

#[test]
fn impl_macro_member_on_public_type_is_denied() {
    assert_sealing_denial("hostile-impl-macro", hostile_impl_macro_member());
}

#[test]
fn opaque_attribute_on_reachable_impl_is_denied() {
    assert_sealing_denial("hostile-impl-attr", hostile_opaque_attr_on_impl());
}

#[test]
fn opaque_attribute_on_impl_method_is_denied() {
    assert_sealing_denial(
        "hostile-impl-method-attr",
        hostile_opaque_attr_on_impl_method(),
    );
}

#[test]
fn opaque_attribute_on_trait_member_is_denied() {
    assert_sealing_denial(
        "hostile-trait-member-attr",
        hostile_opaque_attr_on_trait_member(),
    );
}

#[test]
fn macro_export_trait_bound_template_is_denied() {
    assert_sealing_denial(
        "hostile-macro-export-template",
        hostile_macro_export_trait_bound_template(),
    );
}

#[test]
fn external_module_reexport_of_concrete_ceremony_passes() {
    assert_external_reexport_pass(
        "legal-ext-module",
        entry_reexport_external_module(),
        external_legal_module_admit(),
    );
}

#[test]
fn external_type_reexport_of_concrete_method_passes() {
    assert_external_reexport_pass(
        "legal-ext-type-method",
        entry_reexport_external_type(),
        external_legal_type_method_admit(),
    );
}

#[test]
fn public_foreign_fn_authority_marker_is_denied() {
    assert_sealing_denial(
        "hostile-foreign-fn-auth",
        hostile_foreign_fn_authority_marker(),
    );
}

#[test]
fn public_foreign_static_capability_marker_is_denied() {
    assert_sealing_denial(
        "hostile-foreign-static-cap",
        hostile_foreign_static_capability_marker(),
    );
}

#[test]
fn opaque_attribute_on_public_foreign_fn_is_denied() {
    assert_sealing_denial(
        "hostile-foreign-fn-attr",
        hostile_opaque_attr_on_foreign_fn(),
    );
}

#[test]
fn public_foreign_fn_concrete_authority_passes() {
    let repo = AuthoritySealingTestRepository::create("legal-foreign-fn");
    repo.assemble_with_lib_source(legal_foreign_fn_concrete_authority());
    let (ok, output) = repo.run_boundary_check();
    repo.cleanup();
    assert!(ok, "legal foreign fn must pass:\n{output}");
    assert!(
        !output.contains("BC7001_AUTHORITY_SEALING"),
        "legal foreign fn: unexpected sealing diagnostic:\n{output}"
    );
}
