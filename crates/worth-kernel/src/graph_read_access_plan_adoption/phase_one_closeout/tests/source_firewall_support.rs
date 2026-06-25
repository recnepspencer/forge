use std::fs;
use std::path::{Path, PathBuf};

pub(crate) fn production_adoption_lane_sources() -> Vec<ProductionSource> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/graph_read_access_plan_adoption");
    let mut sources = Vec::new();
    for phase_one_root in PHASE_ONE_PRODUCTION_SOURCE_ROOTS {
        collect_production_sources(&root.join(phase_one_root), &mut sources);
    }
    sources
}

fn collect_production_sources(dir: &Path, sources: &mut Vec<ProductionSource>) {
    for entry in fs::read_dir(dir).expect("read adoption lane source directory") {
        let entry = entry.expect("read adoption lane source entry");
        let path = entry.path();
        if path.is_dir() {
            let dir_name = path.file_name().and_then(|name| name.to_str());
            if dir_name != Some("test_fixtures") && dir_name != Some("tests") {
                collect_production_sources(&path, sources);
            }
            continue;
        }
        if path.extension().and_then(|extension| extension.to_str()) == Some("rs")
            && path.file_name().and_then(|name| name.to_str()) != Some("tests.rs")
        {
            sources.push(ProductionSource {
                contents: fs::read_to_string(&path).expect("read adoption lane production source"),
                path,
            });
        }
    }
}

const PHASE_ONE_PRODUCTION_SOURCE_ROOTS: &[&str] = &[
    "phase_one_closeout",
    "seed_admission",
    "execution_folklore_inventory",
];

pub(crate) struct ProductionSource {
    contents: String,
    path: PathBuf,
}

impl ProductionSource {
    pub(crate) fn contents(&self) -> &str {
        &self.contents
    }

    pub(crate) fn path(&self) -> &Path {
        &self.path
    }
}
