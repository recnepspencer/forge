use std::path::{Path, PathBuf};

pub(crate) fn repository_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .expect("certification crate lives under workspaces/worth-store/crates")
}

#[cfg(test)]
pub(crate) fn repository_source(relative_path: &str) -> PathBuf {
    repository_root().join(relative_path)
}

pub(crate) fn store_crate_source(crate_name: &str) -> PathBuf {
    repository_root()
        .join("workspaces/worth-store/crates")
        .join(crate_name)
        .join("src")
}

#[cfg(test)]
pub(crate) fn certification_source(relative_path: &str) -> PathBuf {
    store_crate_source("worth-store-certification").join(relative_path)
}
