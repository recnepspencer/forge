//! Production-binary matrix: definition-resolved authority identity (BC7001).

use super::authority_sealing_fixture::{
    dep_non_authority_export, dep_renames_authority_marker_to_gate, entry_legal_non_authority_dep,
    entry_private_use_dep_gate, entry_qualified_dep_gate, hostile_assoc_projection_where_launder,
    hostile_href_where_launder, hostile_nonbare_where_on_param_self,
    hostile_nonbare_where_wrapper_launder, legal_concrete_resolve_control,
    AuthoritySealingTestRepository,
};

// Schema-band package name so band-dependency law admits the path dep; sealing
// still resolves the renamed authority export inside it.
const DEP_PACKAGE: &str = "worth-schema-authgate";

fn assert_denial(label: &str, ok: bool, output: &str) {
    assert!(!ok, "{label} must fail authority sealing:\n{output}");
    assert!(
        output.contains("BC7001_AUTHORITY_SEALING"),
        "{label}: expected BC7001, got:\n{output}"
    );
    assert!(
        output.contains("Authority sealing law"),
        "{label}: expected law quote, got:\n{output}"
    );
}

fn assert_pass(label: &str, ok: bool, output: &str) {
    assert!(ok, "{label} must pass:\n{output}");
    assert!(
        !output.contains("BC7001_AUTHORITY_SEALING"),
        "{label}: unexpected sealing diagnostic:\n{output}"
    );
}

fn assert_sealing_denial(label: &str, source: &str) {
    let repo = AuthoritySealingTestRepository::create(label);
    repo.assemble_with_lib_source(source);
    let (ok, output) = repo.run_boundary_check();
    repo.cleanup();
    assert_denial(label, ok, &output);
}

fn assert_sealing_pass(label: &str, source: &str) {
    let repo = AuthoritySealingTestRepository::create(label);
    repo.assemble_with_lib_source(source);
    let (ok, output) = repo.run_boundary_check();
    repo.cleanup();
    assert_pass(label, ok, &output);
}

fn assert_dep_denial(label: &str, entry: &str, dep: &str) {
    let repo = AuthoritySealingTestRepository::create(label);
    repo.assemble_with_external_path_dependency(entry, DEP_PACKAGE, dep);
    let (ok, output) = repo.run_boundary_check();
    repo.cleanup();
    assert_denial(label, ok, &output);
}

fn assert_dep_pass(label: &str, entry: &str, dep: &str) {
    let repo = AuthoritySealingTestRepository::create(label);
    repo.assemble_with_external_path_dependency(entry, DEP_PACKAGE, dep);
    let (ok, output) = repo.run_boundary_check();
    repo.cleanup();
    assert_pass(label, ok, &output);
}

#[test]
fn private_use_of_dep_renamed_authority_is_denied() {
    assert_dep_denial(
        "hostile-dep-use-gate",
        entry_private_use_dep_gate(),
        dep_renames_authority_marker_to_gate(),
    );
}

#[test]
fn qualified_dep_renamed_authority_is_denied() {
    assert_dep_denial(
        "hostile-dep-qualified-gate",
        entry_qualified_dep_gate(),
        dep_renames_authority_marker_to_gate(),
    );
}

#[test]
fn nonbare_where_wrapper_launder_is_denied() {
    assert_sealing_denial(
        "hostile-nonbare-wrapper",
        hostile_nonbare_where_wrapper_launder(),
    );
}

#[test]
fn nonbare_where_on_param_self_is_denied() {
    assert_sealing_denial(
        "hostile-nonbare-param-self",
        hostile_nonbare_where_on_param_self(),
    );
}

#[test]
fn assoc_projection_where_launder_is_denied() {
    assert_sealing_denial(
        "hostile-assoc-projection",
        hostile_assoc_projection_where_launder(),
    );
}

#[test]
fn href_where_launder_is_denied() {
    assert_sealing_denial("hostile-href-where", hostile_href_where_launder());
}

#[test]
fn non_authority_dep_trait_is_not_sealed() {
    assert_dep_pass(
        "legal-dep-describe",
        entry_legal_non_authority_dep(),
        dep_non_authority_export(),
    );
}

#[test]
fn concrete_ceremony_still_passes() {
    assert_sealing_pass("legal-concrete-resolve", legal_concrete_resolve_control());
}
