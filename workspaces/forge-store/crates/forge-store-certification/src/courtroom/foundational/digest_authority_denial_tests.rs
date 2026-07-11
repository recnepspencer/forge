use crate::courtroom::source_tree::{certification_source, store_crate_source};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[test]
fn serde_json_digest_basis_is_rejected() {
    for source in phase_six_production_authority_sources() {
        let contents = fs::read_to_string(&source).unwrap();
        for forbidden in [
            "serde_json::to_vec",
            "serde_json :: to_vec",
            "T: Serialize",
            "T : Serialize",
            "serde::Serialize",
            "impl Serialize",
            "where T: serde::Serialize",
            "where T : serde::Serialize",
        ] {
            assert!(
                !contents.contains(forbidden),
                "{} exposes forbidden digest authority input {forbidden}",
                source.display()
            );
        }
    }
}

#[test]
fn digest_text_cannot_satisfy_store_authority() {
    let compile_fail_proofs = fs::read_to_string(certification_source(
        "courtroom/cross_cutting/certification_compile_fail_proofs.md",
    ))
    .unwrap();

    for required_proof in [
        "Digest strings cannot construct Store aspect identity authority",
        "Digest strings cannot construct Store recovery source authority",
        "Digest strings cannot construct Store checkpoint authority",
        "Digest strings cannot construct Store page authority",
        "Digest strings cannot construct Store WAL authority",
        "Digest strings cannot construct Store certification authority",
    ] {
        assert!(
            compile_fail_proofs.contains(required_proof),
            "missing compile-fail proof: {required_proof}"
        );
    }
}

#[test]
fn physical_authority_basis_rejects_debug_projection_text() {
    for source in physical_authority_basis_sources() {
        let contents = fs::read_to_string(&source).unwrap();
        for forbidden in ["{:?}", "Debug"] {
            assert!(
                !contents.contains(forbidden),
                "{} lets projection formatting enter authority basis through {forbidden}",
                source.display()
            );
        }
    }
}

fn phase_six_production_authority_sources() -> Vec<PathBuf> {
    let mut sources = Vec::new();
    for crate_name in [
        "forge-store-aspect-native",
        "forge-store-readiness",
        "forge-store-physical-integrity",
        "forge-store-physical-format",
    ] {
        collect_rust_sources(&store_crate_source(crate_name), &mut sources);
    }

    sources
}

fn physical_authority_basis_sources() -> Vec<PathBuf> {
    let source_dir = store_crate_source("forge-store-physical-integrity");
    [
        "authority/integrity_authority_basis_entries.rs",
        "authority/integrity_authority_basis_tokens.rs",
        "authority/mod.rs",
    ]
    .into_iter()
    .map(|file_name| source_dir.join(file_name))
    .collect()
}

fn collect_rust_sources(directory: &Path, sources: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(directory).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            collect_rust_sources(&path, sources);
            continue;
        }
        if path.extension().and_then(|extension| extension.to_str()) == Some("rs") {
            sources.push(path);
        }
    }
}
