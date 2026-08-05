use std::{fs, path::Path};

const FORBIDDEN_DEPENDENCIES: [&str; 5] = [
    "worth-query-replay",
    "worth-query-execution",
    "worth-query-installation",
    "worth-query-admission",
    "sha2",
];

const FORBIDDEN_RESIDUE: [&str; 6] = [
    "BankEstateOracles",
    "EstateActorContext",
    "EstateCapabilityUse",
    "EstateDecision",
    "EstateDenial",
    "AuthorityMarker",
];

#[test]
fn certification_uses_only_the_entry_audience_and_foundational_surface() {
    let manifest = fs::read_to_string(manifest_dir().join("Cargo.toml")).unwrap();
    assert!(manifest.contains("worth-query-host.workspace = true"));
    for dependency in FORBIDDEN_DEPENDENCIES {
        assert!(
            !manifest.contains(dependency),
            "forbidden dependency: {dependency}"
        );
    }
}

#[test]
fn production_bank_source_contains_no_superseded_authority_or_oracle_lane() {
    let bank_world = manifest_dir().join("..").join("..");
    for source in [
        bank_world.join("crates").join("bank-domain").join("src"),
        bank_world.join("crates").join("bank-server").join("src"),
    ] {
        inspect_rust_sources(&source);
    }
}

fn inspect_rust_sources(root: &Path) {
    let mut pending = vec![root.to_path_buf()];
    while let Some(path) = pending.pop() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                let source = fs::read_to_string(&path).unwrap();
                for residue in FORBIDDEN_RESIDUE {
                    assert!(
                        !source.contains(residue),
                        "forbidden residue {residue} remains in {}",
                        path.display()
                    );
                }
            }
        }
    }
}

fn manifest_dir() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}
