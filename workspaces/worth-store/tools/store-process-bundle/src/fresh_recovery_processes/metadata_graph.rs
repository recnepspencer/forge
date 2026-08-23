use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;

use super::targets::{TargetSet, TargetSpec};

pub(crate) struct Metadata {
    workspace_root: PathBuf,
    packages: BTreeMap<String, Package>,
    nodes: BTreeMap<String, Node>,
}

pub(crate) struct Package {
    id: String,
    name: String,
    manifest: PathBuf,
    root: PathBuf,
    dependency_roots: Vec<PathBuf>,
    dependencies: Vec<String>,
    binaries: BTreeSet<String>,
    source: Option<String>,
}

struct Node {
    dependencies: Vec<String>,
    features: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FeatureNode {
    package_id: String,
    features: Box<[String]>,
    dependencies: Box<[String]>,
}

impl FeatureNode {
    pub fn package_id(&self) -> &str {
        &self.package_id
    }

    pub fn features(&self) -> &[String] {
        &self.features
    }

    pub fn dependencies(&self) -> &[String] {
        &self.dependencies
    }
}

impl Metadata {
    pub(crate) fn workspace_root(&self) -> &Path {
        &self.workspace_root
    }

    pub(crate) fn package(&self, id: &str) -> Option<&Package> {
        self.packages.get(id)
    }

    pub(crate) fn package_id<R>(&self, target: &TargetSpec<R>) -> Result<String, String> {
        self.packages
            .values()
            .find(|package| {
                package.name == target.package
                    && package.binaries.contains(target.binary)
                    && package.source.is_none()
            })
            .map(|package| package.id.clone())
            .ok_or_else(|| format!("metadata omitted {}::{}", target.package, target.binary))
    }

    pub(crate) fn package_name(&self, id: &str) -> Option<&str> {
        self.packages.get(id).map(|package| package.name.as_str())
    }
}

impl Package {
    pub(crate) fn manifest(&self) -> &Path {
        &self.manifest
    }

    pub(crate) fn root(&self) -> &Path {
        &self.root
    }
}

pub(crate) fn load(cargo: &OsStr, workspace: &Path, features: &[&str]) -> Result<Metadata, String> {
    let mut command = Command::new(cargo);
    command.current_dir(workspace).args([
        "metadata",
        "--locked",
        "--format-version",
        "1",
        "--manifest-path",
    ]);
    command.arg(workspace.join("Cargo.toml"));
    if !features.is_empty() {
        command.arg("--features").arg(features.join(","));
    }
    let output = command
        .output()
        .map_err(|error| format!("spawn Cargo metadata: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "Cargo metadata failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    parse(&output.stdout, workspace)
}

pub(crate) fn dependency_ids(
    metadata: &Metadata,
    targets: &TargetSet,
    packages: &[&str],
) -> Result<BTreeSet<String>, String> {
    let mut pending = vec![
        metadata.package_id(&targets.writer)?,
        metadata.package_id(&targets.observer)?,
        metadata.package_id(&targets.recovery)?,
    ];
    for package in packages {
        pending.push(
            metadata
                .packages
                .values()
                .find(|candidate| candidate.name == *package && candidate.source.is_none())
                .map(|candidate| candidate.id.clone())
                .ok_or_else(|| format!("metadata omitted local package {package}"))?,
        );
    }
    let mut visited = BTreeSet::new();
    while let Some(id) = pending.pop() {
        if !visited.insert(id.clone()) {
            continue;
        }
        let package = metadata
            .packages
            .get(&id)
            .ok_or_else(|| format!("metadata omitted package {id}"))?;
        pending.extend(package.dependencies.iter().cloned());
    }
    Ok(visited)
}

pub(crate) fn feature_graph(metadata: &Metadata, packages: &[&str]) -> Vec<FeatureNode> {
    let roots = feature_roots(metadata, packages);
    let mut ids = roots.clone();
    let mut index = 0;
    while index < ids.len() {
        if let Some(node) = metadata.nodes.get(&ids[index]) {
            let next = node
                .dependencies
                .iter()
                .filter(|id| !ids.contains(id))
                .cloned()
                .collect::<Vec<_>>();
            ids.extend(next);
        }
        index += 1;
    }
    ids.sort();
    ids.dedup();
    ids.into_iter()
        .filter_map(|id| {
            metadata.nodes.get(&id).map(|node| FeatureNode {
                package_id: id,
                features: node.features.clone().into_boxed_slice(),
                dependencies: node.dependencies.clone().into_boxed_slice(),
            })
        })
        .collect()
}

pub(crate) fn feature_roots(metadata: &Metadata, packages: &[&str]) -> Vec<String> {
    let mut roots = packages
        .iter()
        .filter_map(|name| {
            metadata
                .packages
                .values()
                .find(|package| package.name == *name && package.source.is_none())
                .map(|package| package.id.clone())
        })
        .collect::<Vec<_>>();
    roots.sort();
    roots.dedup();
    roots
}

fn parse(bytes: &[u8], workspace: &Path) -> Result<Metadata, String> {
    let document: Value = serde_json::from_slice(bytes)
        .map_err(|error| format!("decode Cargo metadata JSON: {error}"))?;
    let workspace_root = canonical_path(
        Path::new(required_field(&document, "workspace_root")?),
        "metadata workspace root",
    )?;
    if !canonical_path(workspace, "requested workspace")?.starts_with(&workspace_root) {
        return Err("Cargo metadata escaped the requested workspace".to_owned());
    }
    let package_values = document
        .get("packages")
        .and_then(Value::as_array)
        .ok_or_else(|| "Cargo metadata omitted packages".to_owned())?;
    let mut packages = BTreeMap::new();
    let mut roots = BTreeMap::new();
    for value in package_values {
        let id = required_field(value, "id")?.to_owned();
        let manifest = canonical_path(
            Path::new(required_field(value, "manifest_path")?),
            "manifest",
        )?;
        let root = manifest
            .parent()
            .ok_or_else(|| format!("manifest has no parent: {manifest:?}"))?
            .to_owned();
        let name = required_field(value, "name")?.to_owned();
        let binaries = value
            .get("targets")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter(|target| is_binary(target))
            .filter_map(|target| target.get("name").and_then(Value::as_str))
            .map(str::to_owned)
            .collect();
        let dependency_roots = value
            .get("dependencies")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter(|dependency| dependency.get("kind").and_then(Value::as_str) != Some("dev"))
            .filter_map(|dependency| dependency.get("path").and_then(Value::as_str))
            .map(|path| canonical_path(Path::new(path), "path dependency"))
            .collect::<Result<Vec<_>, _>>()?;
        roots.insert(root.clone(), id.clone());
        packages.insert(
            id.clone(),
            Package {
                id,
                name,
                manifest,
                root,
                dependency_roots,
                dependencies: Vec::new(),
                binaries,
                source: value
                    .get("source")
                    .and_then(Value::as_str)
                    .map(str::to_owned),
            },
        );
    }
    for package in packages.values_mut() {
        package.dependencies = package
            .dependency_roots
            .iter()
            .map(|root| {
                roots
                    .get(root)
                    .cloned()
                    .ok_or_else(|| format!("metadata omitted path dependency {root:?}"))
            })
            .collect::<Result<Vec<_>, _>>()?;
    }
    let nodes = parse_nodes(&document)?;
    Ok(Metadata {
        workspace_root,
        packages,
        nodes,
    })
}

fn parse_nodes(document: &Value) -> Result<BTreeMap<String, Node>, String> {
    let mut nodes = BTreeMap::new();
    let values = document
        .get("resolve")
        .and_then(|resolve| resolve.get("nodes"))
        .and_then(Value::as_array)
        .ok_or_else(|| "Cargo metadata omitted resolve nodes".to_owned())?;
    for value in values {
        let id = required_field(value, "id")?.to_owned();
        let dependencies = value
            .get("deps")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(|dependency| dependency.get("pkg").and_then(Value::as_str))
            .map(str::to_owned)
            .collect();
        let features = value
            .get("features")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .map(str::to_owned)
            .collect();
        nodes.insert(
            id.clone(),
            Node {
                dependencies,
                features,
            },
        );
    }
    Ok(nodes)
}

fn is_binary(target: &Value) -> bool {
    target
        .get("kind")
        .and_then(Value::as_array)
        .is_some_and(|kinds| kinds.iter().any(|kind| kind == "bin"))
}

fn required_field<'a>(value: &'a Value, field: &str) -> Result<&'a str, String> {
    value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("Cargo metadata omitted string field {field}"))
}

fn canonical_path(path: &Path, label: &str) -> Result<PathBuf, String> {
    path.canonicalize()
        .map_err(|error| format!("canonicalize {label} {path:?}: {error}"))
}
