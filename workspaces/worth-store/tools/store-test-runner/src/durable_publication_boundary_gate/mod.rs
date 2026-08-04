mod authority_trace;
mod cargo_graph;
mod closure_ledger;
mod contract;
mod cutover;
mod inventory;
mod public_api;

use std::path::{Path, PathBuf};

pub(super) fn repository_root() -> PathBuf {
    crate::workspace_root()
        .parent()
        .and_then(Path::parent)
        .expect("worth-store workspace must live under workspaces")
        .to_path_buf()
}

pub(super) fn read_repository_document(path: &str) -> Result<String, String> {
    let path = repository_root().join(path);
    std::fs::read_to_string(&path)
        .map_err(|error| format!("cannot read {}: {error}", path.display()))
}
