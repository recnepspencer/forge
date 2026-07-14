//! Cross-family equivalence proof for Query paths carried by macro tokens.

use super::query_source_fixture::{entry_case, run_source_case, SourceCase};

#[test]
fn macro_tokens_are_classified_by_source_and_exported_surface() {
    let (wrong_band_ok, wrong_band_output) = run_source_case(SourceCase {
        label: "source-wrong-band-macro",
        workspace: "worth-contracts",
        lane: "worth-contracts",
        prefix: "worth-schema-",
        package: "worth-schema-core",
        tier: "worth",
        band: "schema",
        domain: "core",
        dependencies: "",
        source:
            "macro_rules! bypass { () => { worth_query_decl::facade::CanonicalQueryArtifact }; }\n",
        additional_sources: &[],
        manifest_suffix: "",
    });
    assert!(
        !wrong_band_ok,
        "wrong-band macro tokens passed:\n{wrong_band_output}"
    );
    assert!(
        wrong_band_output.contains("BC3101_QUERY_SOURCE_PATH"),
        "{wrong_band_output}"
    );

    let (exported_ok, exported_output) = run_source_case(entry_case(
        "source-exported-macro-signature",
        r#"use worth_query_decl::facade::CanonicalQueryArtifact;
#[macro_export]
macro_rules! leak { () => { pub fn leaked(value: CanonicalQueryArtifact) { let _ = value; } }; }
"#,
    ));
    assert!(
        !exported_ok,
        "exported Query signature macro passed:\n{exported_output}"
    );
    assert!(
        exported_output.contains("BC3102_QUERY_PUBLIC_SIGNATURE"),
        "{exported_output}"
    );

    for (label, source, expected_code) in [
        (
            "source-exported-macro-struct",
            r#"use worth_query_decl::facade::CanonicalQueryArtifact;
#[macro_export]
macro_rules! leak { () => { pub struct Leak { pub value: CanonicalQueryArtifact } }; }
"#,
            "BC3102_QUERY_PUBLIC_SIGNATURE",
        ),
        (
            "source-exported-macro-reexport",
            r#"#[macro_export]
macro_rules! leak { () => { pub use worth_query_decl::facade::CanonicalQueryArtifact; }; }
"#,
            "BC3103_QUERY_PUBLIC_REEXPORT",
        ),
        (
            "source-item-macro-public-type",
            "define_public!(pub type Leak = CanonicalQueryArtifact);\n",
            "BC3102_QUERY_PUBLIC_SIGNATURE",
        ),
    ] {
        let (ok, output) = run_source_case(entry_case(label, source));
        assert!(!ok, "macro-carried public surface passed:\n{output}");
        assert!(output.contains(expected_code), "{output}");
    }
}
