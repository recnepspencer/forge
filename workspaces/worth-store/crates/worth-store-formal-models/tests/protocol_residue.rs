use std::fs;
use std::path::{Path, PathBuf};

#[test]
fn protocol_cutover_leaves_no_placeholder_or_provenance_shaped_production_surface() {
    let formal = Path::new(env!("CARGO_MANIFEST_DIR"));
    let roots = [
        formal.join("src"),
        formal.join("../worth-store-replication/src"),
        formal.join("../worth-store-certification/src/courtroom/protocol_models"),
    ];
    for file in roots.iter().flat_map(|root| rust_sources(root)) {
        let source = fs::read_to_string(&file).expect("production source is readable");
        for forbidden in [
            "ModeledStateMachine",
            "LayoutFormalObservation",
            "TransitionReceipt",
            "OwnerCaseCatalog",
            "S8Protocol",
            "S9Protocol",
            "Phase15",
            "MilestoneProtocol",
            "RoadmapProtocol",
        ] {
            assert!(
                !source.contains(forbidden),
                "stale or provenance-shaped surface {forbidden} remains in {}",
                file.display()
            );
        }
    }

    assert!(
        !rust_sources(&formal.join("src/mutants")).any(|_| true),
        "controlled defects belong to certification, not checked semantics"
    );
}

fn rust_sources(root: &Path) -> impl Iterator<Item = PathBuf> {
    let mut pending = vec![root.to_path_buf()];
    let mut files = Vec::new();
    while let Some(path) = pending.pop() {
        let Ok(entries) = fs::read_dir(path) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                files.push(path);
            }
        }
    }
    files.into_iter()
}
