use std::path::{Path, PathBuf};

#[test]
fn production_bootstrap_consumers_have_no_raw_catalog_decode_route() {
    let recovery = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let store = recovery.parent().unwrap().join("worth-store");
    for source_root in [recovery.join("src"), store.join("src")] {
        let offenders = rust_sources(&source_root)
            .into_iter()
            .filter(|path| {
                std::fs::read_to_string(path)
                    .unwrap()
                    .contains("BootstrapCatalog::decode")
            })
            .collect::<Vec<_>>();
        assert!(
            offenders.is_empty(),
            "production raw bootstrap decode routes remain: {offenders:?}"
        );
    }
}

#[test]
fn production_consumers_name_the_integrated_ingress_and_resident_projection() {
    let recovery = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let discovery = read(
        &recovery
            .join("src")
            .join("orchestration/discovery/observation.rs"),
    );
    assert!(discovery.contains("admit_observed_bootstrap_catalog"));
    assert!(discovery.contains("projection.current_root_generation"));

    let ordinary_open = read(
        &recovery
            .parent()
            .unwrap()
            .join("worth-store/src/physical_runtime/record_serving/admission/open.rs"),
    );
    assert!(ordinary_open.contains("admit_resident_bootstrap_catalog"));
    assert!(ordinary_open.contains("admitted"));
    assert!(ordinary_open.contains(".project(admission_context)"));
}

fn rust_sources(root: &Path) -> Vec<PathBuf> {
    let mut pending = vec![root.to_path_buf()];
    let mut sources = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(directory).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                sources.push(path);
            }
        }
    }
    sources
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|error| panic!("read {path:?}: {error}"))
}
