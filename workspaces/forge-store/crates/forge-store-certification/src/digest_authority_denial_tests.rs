use std::fs;
use std::path::{Path, PathBuf};

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
    let compile_fail_proofs = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("certification_compile_fail_proofs.md"),
    )
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
    let workspace = workspace_root();
    let mut sources = Vec::new();
    for crate_name in [
        "forge-store-aspect-native",
        "forge-store-readiness",
        "forge-store-physical-integrity",
        "forge-store-physical-format",
    ] {
        collect_rust_sources(
            &workspace.join("crates").join(crate_name).join("src"),
            &mut sources,
        );
    }

    sources
}

fn physical_authority_basis_sources() -> Vec<PathBuf> {
    let source_dir = workspace_root()
        .join("crates")
        .join("forge-store-physical-integrity")
        .join("src");
    [
        "integrity_authority_basis_entries.rs",
        "integrity_authority_basis_tokens.rs",
        "integrity_authority_claim_basis.rs",
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

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}
