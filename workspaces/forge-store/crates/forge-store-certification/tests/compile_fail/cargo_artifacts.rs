use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

#[path = "cargo_artifact_message.rs"]
mod cargo_artifact_message;

pub fn dependency_dir() -> PathBuf {
    store_workspace_root()
        .join("target")
        .join("debug")
        .join("deps")
}

pub fn compiled_extern(test_target: &str, crate_name: &str) -> PathBuf {
    let graphs = artifact_graphs()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    graphs
        .get(test_target)
        .and_then(|graph| graph.get(crate_name))
        .cloned()
        .unwrap_or_else(|| panic!("Cargo did not emit {crate_name} for {test_target}"))
}

pub fn discover(test_target: &str) {
    let mut graphs = artifact_graphs()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    graphs
        .entry(test_target.to_owned())
        .or_insert_with(|| discover_artifact_graph(test_target));
}

fn artifact_graphs() -> &'static Mutex<HashMap<String, HashMap<String, PathBuf>>> {
    static GRAPHS: OnceLock<Mutex<HashMap<String, HashMap<String, PathBuf>>>> = OnceLock::new();
    GRAPHS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn discover_artifact_graph(test_target: &str) -> HashMap<String, PathBuf> {
    let output =
        std::process::Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
            .arg("check")
            .arg("--offline")
            .arg("--message-format=json")
            .arg("-p")
            .arg("forge-store-certification")
            .arg("--test")
            .arg(test_target)
            .current_dir(store_workspace_root())
            .output()
            .expect("Cargo must be available for compile-fail artifact discovery");
    assert!(
        output.status.success(),
        "Cargo artifact discovery failed for {test_target}:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(compiled_library_artifact)
        .collect()
}

fn compiled_library_artifact(line: &str) -> Option<(String, PathBuf)> {
    let message = cargo_artifact_message::parse(line)?;
    let artifact = message.filenames.into_iter().find(|path| {
        path.extension().is_some_and(|extension| {
            extension == "rlib" || extension == "rmeta" || extension == "dll"
        })
    })?;
    Some((message.target_name.replace('-', "_"), artifact))
}

fn store_workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("certification crate lives under workspaces/forge-store/crates")
}
