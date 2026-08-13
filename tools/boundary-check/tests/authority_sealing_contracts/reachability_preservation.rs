//! Regression contracts for shared BC7001 public-surface reachability.

use super::authority_sealing_fixture::AuthoritySealingTestRepository;

fn assert_authority_denial(label: &str, source: &str) -> String {
    let repository = AuthoritySealingTestRepository::create(label);
    repository.assemble_with_lib_source(source);
    let (ok, output) = repository.run_boundary_check();
    repository.cleanup();
    assert!(!ok, "{label} must fail authority sealing:\n{output}");
    assert!(
        output.contains("BC7001_AUTHORITY_SEALING"),
        "{label}: expected BC7001, got:\n{output}"
    );
    output
}

fn assert_authority_pass(label: &str, source: &str) {
    let repository = AuthoritySealingTestRepository::create(label);
    repository.assemble_with_lib_source(source);
    let (ok, output) = repository.run_boundary_check();
    repository.cleanup();
    assert!(ok, "{label} must pass authority sealing:\n{output}");
}

#[test]
fn relative_named_reexport_chain_preserves_authority_sealing() {
    assert_authority_denial(
        "relative-named-chain",
        r#"
pub trait AuthorityMarker: 'static {}

mod facade {
    mod leaf {
        use super::super::AuthorityMarker;

        pub fn admit<A: AuthorityMarker>(_authority: A) {}
    }

    pub use leaf::admit as renamed;
}

pub use facade::renamed as admit;
"#,
    );
}

#[test]
fn imported_type_alias_preserves_authority_sealing() {
    assert_authority_denial(
        "imported-type-alias",
        r#"
pub trait AuthorityMarker: 'static {}

mod leaf {
    use super::AuthorityMarker;

    pub struct Hidden;

    impl Hidden {
        pub fn admit<A: AuthorityMarker>(_authority: A) {}
    }
}

use leaf::Hidden as Local;
pub type Public = Local;
"#,
    );
}

#[test]
fn module_alias_reexport_preserves_authority_sealing() {
    assert_authority_denial(
        "module-alias-reexport",
        r#"
pub trait AuthorityMarker: 'static {}

mod leaf {
    use super::AuthorityMarker;

    pub fn admit<A: AuthorityMarker>(_authority: A) {}
}

use leaf as lane;
pub use lane::admit;
"#,
    );
}

#[test]
fn module_alias_type_rhs_preserves_authority_sealing() {
    assert_authority_denial(
        "module-alias-type-rhs",
        r#"
pub trait AuthorityMarker: 'static {}

mod leaf {
    use super::AuthorityMarker;

    pub struct Hidden;

    impl Hidden {
        pub fn admit<A: AuthorityMarker>(_authority: A) {}
    }
}

use leaf as lane;
pub type Public = lane::Hidden;
"#,
    );
}

#[test]
fn production_glob_import_requires_an_explicit_authority_surface() {
    let output = assert_authority_denial(
        "production-glob-import",
        r#"
mod pack {
    pub mod leaf {
        pub fn concrete() {}
    }
}

use pack::*;
pub fn ordinary_surface() {}
"#,
    );
    assert!(
        output.contains("Authority-governed source must use named imports and reexports"),
        "expected explicit glob diagnostic:\n{output}"
    );
}

#[test]
fn generic_alias_parameter_does_not_resolve_to_same_named_local_type() {
    assert_authority_pass(
        "generic-alias-shadow",
        r#"
pub trait AuthorityMarker: 'static {}

struct T;

impl T {
    pub fn admit<A: AuthorityMarker>(_authority: A) {}
}

pub type Public<T> = T;
"#,
    );
}
