use std::path::{Path, PathBuf};

pub(crate) fn repository_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .expect("certification crate lives under workspaces/forge-store/crates")
}

pub(crate) fn repository_source(relative_path: &str) -> PathBuf {
    repository_root().join(relative_path)
}

pub(crate) fn store_crate_source(crate_name: &str) -> PathBuf {
    repository_root()
        .join("workspaces/forge-store/crates")
        .join(crate_name)
        .join("src")
}

pub(crate) fn certification_source(relative_path: &str) -> PathBuf {
    store_crate_source("forge-store-certification").join(relative_path)
}
