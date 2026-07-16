//! Full-audience proofs for Query source vocabulary and legal public surfaces.

use super::query_source_fixture::{entry_case, run_source_case, SourceCase};

#[test]
fn every_audience_facade_item_is_indexed_for_public_signatures() {
    for (label, dependency, item) in [
        (
            "source-decl-vocabulary",
            "worth-query-decl",
            "CanonicalQueryArtifact",
        ),
        (
            "source-host-vocabulary",
            "worth-query-host",
            "runtime::WorthQueryRuntime",
        ),
        (
            "source-replay-vocabulary",
            "worth-query-replay",
            "ScopedReplayBasis",
        ),
    ] {
        let dependency_line = audience_dependency(dependency);
        let source = format!(
            "pub fn leak(value: {dependency_name}::facade::{item}) {{ let _ = value; }}\n",
            dependency_name = dependency.replace('-', "_")
        );
        let (ok, output) = run_source_case(cert_case(label, &dependency_line, &source));
        assert!(!ok, "{dependency} vocabulary escaped indexing:\n{output}");
        assert!(output.contains("BC3102_QUERY_PUBLIC_SIGNATURE"), "{output}");
    }
}

#[test]
fn every_allowed_band_and_audience_pair_accepts_private_consumption() {
    for (label, band, dependency, item) in [
        (
            "source-entry-decl-legal",
            "entry",
            "worth-query-decl",
            "CanonicalQueryArtifact",
        ),
        (
            "source-entry-host-legal",
            "entry",
            "worth-query-host",
            "runtime::WorthQueryRuntime",
        ),
        (
            "source-cert-decl-legal",
            "cert",
            "worth-query-decl",
            "CanonicalQueryArtifact",
        ),
        (
            "source-cert-host-legal",
            "cert",
            "worth-query-host",
            "runtime::WorthQueryRuntime",
        ),
        (
            "source-cert-replay-legal",
            "cert",
            "worth-query-replay",
            "ScopedReplayBasis",
        ),
    ] {
        let dependency_line = audience_dependency(dependency);
        let source = format!(
            "fn retain(value: {dependency_name}::facade::{item}) {{ let _ = value; }}\n",
            dependency_name = dependency.replace('-', "_")
        );
        let case = if band == "entry" {
            audience_entry_case(label, &dependency_line, &source)
        } else {
            cert_case(label, &dependency_line, &source)
        };
        let (ok, output) = run_source_case(case);
        assert!(ok, "legal {band} use of {dependency} failed:\n{output}");
    }
}

#[test]
fn query_consumption_does_not_poison_unrelated_public_surfaces() {
    let source = r#"use worth_query_decl::facade::CanonicalQueryArtifact;
pub fn unrelated(value: String) -> usize { value.len() }
mod stable { pub struct Stable; }
pub use stable::Stable;
fn retain(value: CanonicalQueryArtifact) { let _ = value; }
"#;
    let (ok, output) = run_source_case(entry_case("source-unrelated-public-surfaces", source));
    assert!(ok, "unrelated public surface was over-rejected:\n{output}");
}

fn audience_dependency(package: &str) -> String {
    format!(r#"{package} = {{ path = "../../../../../vendor/{package}" }}"#)
}

fn audience_entry_case<'a>(
    label: &'a str,
    dependencies: &'a str,
    source: &'a str,
) -> SourceCase<'a> {
    governed_case(
        label,
        "worth-entry",
        "entry",
        "adoption",
        dependencies,
        source,
    )
}

fn cert_case<'a>(label: &'a str, dependencies: &'a str, source: &'a str) -> SourceCase<'a> {
    governed_case(
        label,
        "worth-cert",
        "cert",
        "adoption",
        dependencies,
        source,
    )
}

fn governed_case<'a>(
    label: &'a str,
    lane: &'a str,
    band: &'a str,
    domain: &'a str,
    dependencies: &'a str,
    source: &'a str,
) -> SourceCase<'a> {
    SourceCase {
        label,
        workspace: lane,
        lane,
        prefix: if band == "entry" {
            "worth-entry-"
        } else {
            "worth-cert-"
        },
        package: if band == "entry" {
            "worth-entry-adoption"
        } else {
            "worth-cert-adoption"
        },
        tier: "worth",
        band,
        domain,
        dependencies,
        source,
        additional_sources: &[],
        manifest_suffix: "",
    }
}
