//! Production-binary proofs for BC3102 Query public-signature classification.

use super::query_source_fixture::{entry_case, run_source_case};

#[test]
fn public_query_return_and_field_share_signature_code() {
    for (label, source) in [
        (
            "source-public-return",
            "pub fn leak() -> worth_query_decl::facade::CanonicalQueryArtifact { todo!() }\n",
        ),
        (
            "source-public-field",
            "pub struct Leak { pub artifact: worth_query_decl::facade::CanonicalQueryArtifact }\n",
        ),
    ] {
        let (ok, output) = run_source_case(entry_case(label, source));
        assert!(!ok, "public Query signature passed:\n{output}");
        assert!(output.contains("BC3102_QUERY_PUBLIC_SIGNATURE"), "{output}");
        assert!(!output.contains("BC3103_QUERY_PUBLIC_REEXPORT"), "{output}");
        assert!(output.contains("rule_contracts.query_audience"), "{output}");
    }
}

#[test]
fn bare_facade_item_name_in_public_signature_is_denied() {
    let (ok, output) = run_source_case(entry_case(
        "source-bare-public-type",
        r#"use worth_query_decl::facade::CanonicalQueryArtifact;
pub fn leak(value: CanonicalQueryArtifact) { let _ = value; }
"#,
    ));
    assert!(!ok, "bare Query facade item passed:\n{output}");
    assert!(output.contains("BC3102_QUERY_PUBLIC_SIGNATURE"), "{output}");
}

#[test]
fn trait_supertraits_and_impl_headers_are_signature_checked() {
    for (label, source) in [
        (
            "source-trait-supertrait",
            r#"use worth_query_decl::facade::CanonicalQueryArtifact;
pub trait External<T> {}
pub trait Leak: External<CanonicalQueryArtifact> {}
"#,
        ),
        (
            "source-impl-header",
            r#"use worth_query_decl::facade::CanonicalQueryArtifact;
pub trait External<T> {}
pub struct Host;
impl External<CanonicalQueryArtifact> for Host {}
"#,
        ),
    ] {
        let (ok, output) = run_source_case(entry_case(label, source));
        assert!(!ok, "Query-bearing trait/impl header passed:\n{output}");
        assert!(output.contains("BC3102_QUERY_PUBLIC_SIGNATURE"), "{output}");
        assert!(output.contains("rule_contracts.query_audience"), "{output}");
    }
}
