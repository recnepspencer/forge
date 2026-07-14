//! Production-binary proofs for BC3103 Query public re-export classification.

use super::query_source_fixture::{entry_case, run_source_case};

#[test]
fn public_query_reexport_has_distinct_code() {
    for (label, source) in [
        (
            "source-public-reexport-direct",
            "pub use worth_query_decl::facade::CanonicalQueryArtifact;\n",
        ),
        (
            "source-public-reexport-rename",
            "pub use worth_query_decl::facade::CanonicalQueryArtifact as Artifact;\n",
        ),
        (
            "source-public-reexport-group",
            "pub use worth_query_decl::facade::{CanonicalQueryArtifact as Artifact};\n",
        ),
        (
            "source-public-reexport-glob",
            "pub use worth_query_decl::facade::*;\n",
        ),
    ] {
        let (ok, output) = run_source_case(entry_case(label, source));
        assert!(!ok, "Query re-export passed:\n{output}");
        assert!(output.contains("BC3103_QUERY_PUBLIC_REEXPORT"), "{output}");
        assert!(
            !output.contains("BC3102_QUERY_PUBLIC_SIGNATURE"),
            "{output}"
        );
        assert!(output.contains("rule_contracts.query_audience"), "{output}");
    }
}

#[test]
fn public_extern_crate_query_reexport_has_bc3103_and_legal_home() {
    let (ok, output) = run_source_case(entry_case(
        "source-public-extern-crate",
        "pub extern crate worth_query_decl as query_api;\n",
    ));
    assert!(!ok, "public extern crate Query re-export passed:\n{output}");
    assert!(output.contains("BC3103_QUERY_PUBLIC_REEXPORT"), "{output}");
    assert!(
        !output.contains("BC3102_QUERY_PUBLIC_SIGNATURE"),
        "{output}"
    );
    assert!(output.contains("rule_contracts.query_audience"), "{output}");
}
