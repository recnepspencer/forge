mod authority_trace;
mod cargo_graph;
mod closure_ledger;
mod cutover_inventory;
mod destination_topology;
mod documents;
mod facade_inventory;
mod persisted_input_contract;

use std::path::{Path, PathBuf};

pub(super) fn repository_root() -> PathBuf {
    crate::workspace_root()
        .parent()
        .and_then(Path::parent)
        .expect("worth-store workspace must live under workspaces")
        .to_path_buf()
}
