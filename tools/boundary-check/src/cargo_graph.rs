use crate::config::SubworkspaceConfig;
use crate::manifest_types::{CargoMetadata, CargoMetadataPackage, Road1Package, WorkspaceManifest};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub(crate) fn parse_workspace_manifest(path: &Path) -> Result<WorkspaceManifest, String> {
    let text =
        fs::read_to_string(path).map_err(|e| format!("read manifest {}: {e}", path.display()))?;
    toml::from_str(&text).map_err(|e| format!("parse manifest {}: {e}", path.display()))
}

pub(crate) fn cargo_metadata(root: &Path, manifest_path: &Path) -> Result<CargoMetadata, String> {
    let output = Command::new("cargo")
        .arg("metadata")
        .arg("--format-version")
        .arg("1")
        .arg("--manifest-path")
        .arg(manifest_path)
        .current_dir(root)
        .output()
        .map_err(|e| format!("spawn cargo metadata for {}: {e}", manifest_path.display()))?;
    if !output.status.success() {
        return Err(format!(
            "cargo metadata failed for {}: {}",
            manifest_path.display(),
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    serde_json::from_slice(&output.stdout).map_err(|e| {
        format!(
            "parse cargo metadata output for {}: {e}",
            manifest_path.display()
        )
    })
}

pub(crate) fn package_name_from_manifest(path: &Path) -> Result<String, String> {
    let manifest = parse_package_manifest(path)?;
    Ok(manifest.name)
}

pub(crate) fn parse_package_manifest(path: &Path) -> Result<Road1Package, String> {
    let text = fs::read_to_string(path)
        .map_err(|e| format!("read crate manifest {}: {e}", path.display()))?;
    let value: toml::Value = toml::from_str(&text)
        .map_err(|e| format!("parse crate manifest {}: {e}", path.display()))?;
    let name = value
        .get("package")
        .and_then(|package| package.get("name"))
        .and_then(|name| name.as_str())
        .ok_or_else(|| format!("crate manifest {} missing package.name", path.display()))?
        .to_owned();

    Ok(Road1Package {
        name,
        dependencies: Vec::new(),
    })
}

pub(crate) fn discover_road1_packages(
    root: &Path,
    subworkspaces: &[SubworkspaceConfig],
) -> Result<Vec<Road1Package>, String> {
    let mut packages = BTreeMap::<String, Road1Package>::new();

    for subworkspace in subworkspaces {
        let manifest_path = root.join(&subworkspace.path).join("Cargo.toml");
        let workspace_root = root.join(&subworkspace.path);
        let workspace_manifest = parse_workspace_manifest(&manifest_path)?;
        let has_members = workspace_manifest
            .workspace
            .as_ref()
            .and_then(|workspace| workspace.members.as_ref())
            .map(|members| !members.is_empty())
            .unwrap_or(false);
        if !has_members {
            continue;
        }
        let metadata = cargo_metadata(root, &manifest_path)?;
        let package_by_id = metadata_package_map(&metadata.packages);
        let dependency_map = metadata_dependency_map(metadata.resolve.as_ref());

        for package in &metadata.packages {
            if !is_workspace_package(&workspace_root, package) {
                continue;
            }
            let dependencies = dependency_map
                .get(&package.id)
                .into_iter()
                .flat_map(|ids| ids.iter())
                .filter_map(|id| package_by_id.get(id))
                .map(|dependency| dependency.name.clone())
                .collect::<Vec<_>>();

            packages.insert(
                package.name.clone(),
                Road1Package {
                    name: package.name.clone(),
                    dependencies,
                },
            );
        }
    }

    Ok(packages.into_values().collect())
}

fn metadata_package_map(
    packages: &[CargoMetadataPackage],
) -> BTreeMap<String, CargoMetadataPackage> {
    packages
        .iter()
        .map(|package| (package.id.clone(), package.clone()))
        .collect()
}

fn metadata_dependency_map(
    resolve: Option<&crate::manifest_types::CargoMetadataResolve>,
) -> BTreeMap<String, Vec<String>> {
    let mut map = BTreeMap::new();
    for node in resolve.map(|resolve| &resolve.nodes).into_iter().flatten() {
        map.insert(node.id.clone(), node.dependencies.clone());
    }
    map
}

fn is_workspace_package(workspace_root: &Path, package: &CargoMetadataPackage) -> bool {
    PathBuf::from(&package.manifest_path).starts_with(workspace_root)
}

pub(crate) fn normalize_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

pub(crate) fn normalize_str(path: &str) -> String {
    path.replace('\\', "/")
}
