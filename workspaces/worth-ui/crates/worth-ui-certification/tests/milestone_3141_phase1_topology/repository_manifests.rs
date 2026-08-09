use std::collections::BTreeMap;
use std::path::Path;

pub(super) fn all(
    inventory: &worth_ui_certification::topology::WorkspaceSourceInventory,
) -> BTreeMap<String, String> {
    let workspace = inventory.root();
    let repository = workspace
        .parent()
        .and_then(Path::parent)
        .expect("Worth UI workspace belongs to the repository");
    let mut manifests = BTreeMap::new();
    visit(repository, repository, workspace, &mut manifests);
    manifests
}

fn visit(
    directory: &Path,
    repository: &Path,
    workspace: &Path,
    manifests: &mut BTreeMap<String, String>,
) {
    let entries = std::fs::read_dir(directory)
        .unwrap_or_else(|error| panic!("cannot enumerate {}: {error}", directory.display()));
    for entry in entries {
        let entry = entry.expect("repository directory entry");
        let path = entry.path();
        if path.is_dir() {
            if !ignored_directory(&path) {
                visit(&path, repository, workspace, manifests);
            }
        } else if path.file_name().is_some_and(|name| name == "Cargo.toml") {
            let identity = manifest_identity(&path, repository, workspace);
            let text = std::fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("cannot read {identity}: {error}"));
            assert!(manifests.insert(identity, text).is_none());
        }
    }
}

fn ignored_directory(path: &Path) -> bool {
    path.file_name().is_some_and(|name| {
        matches!(
            name.to_str(),
            Some(".git" | "target" | "node_modules" | ".venv" | "__pycache__")
        )
    })
}

fn manifest_identity(path: &Path, repository: &Path, workspace: &Path) -> String {
    let relative = path.strip_prefix(repository).expect("repository manifest");
    if let Ok(workspace_relative) = path.strip_prefix(workspace) {
        workspace_relative.to_string_lossy().replace('\\', "/")
    } else {
        format!(
            "repository/{}",
            relative.to_string_lossy().replace('\\', "/")
        )
    }
}
