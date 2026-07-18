use std::path::Path;
use std::process::Command;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageSurface {
    pub name: String,
    pub manifest_path: String,
    pub package_root: String,
    pub features: Vec<String>,
    #[serde(default)]
    pub feature_definitions: std::collections::BTreeMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct TestTargetIdentity {
    pub identity: String,
    pub package: String,
    pub name: String,
    pub kinds: Vec<String>,
    pub source_path: String,
    pub required_features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct DependencyEdge {
    pub consumer: String,
    pub provider: String,
    #[serde(default)]
    pub manifest_name: String,
    pub dependency_kind: String,
    pub features: Vec<String>,
    pub optional: bool,
    #[serde(default = "default_true")]
    pub uses_default_features: bool,
    pub target: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservedBuildGraph {
    pub dependency_edges: Vec<DependencyEdge>,
}

pub type ObservedFeatureGraph = ObservedBuildGraph;

pub(crate) struct CargoSurface {
    pub workspace_root: String,
    pub target_root: String,
    pub packages: Vec<PackageSurface>,
    pub targets: Vec<TestTargetIdentity>,
    pub build_graph: ObservedBuildGraph,
}

#[derive(Deserialize)]
struct CargoMetadata {
    packages: Vec<MetadataPackage>,
    workspace_members: Vec<String>,
    workspace_root: String,
    target_directory: String,
}

#[derive(Deserialize)]
struct MetadataPackage {
    id: String,
    name: String,
    manifest_path: String,
    targets: Vec<MetadataTarget>,
    dependencies: Vec<MetadataDependency>,
    features: std::collections::BTreeMap<String, Vec<String>>,
}

#[derive(Deserialize)]
struct MetadataTarget {
    name: String,
    kind: Vec<String>,
    src_path: String,
    test: bool,
    doctest: bool,
    #[serde(default, rename = "required-features")]
    required_features: Vec<String>,
}

#[derive(Deserialize)]
struct MetadataDependency {
    name: String,
    rename: Option<String>,
    kind: Option<String>,
    #[serde(default)]
    features: Vec<String>,
    optional: bool,
    uses_default_features: bool,
    target: Option<String>,
}

pub(crate) fn discover_cargo_surface(workspace_root: &Path) -> Result<CargoSurface, String> {
    let output = Command::new("cargo")
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .current_dir(workspace_root)
        .output()
        .map_err(|error| format!("could not launch cargo metadata: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "cargo metadata failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let metadata: CargoMetadata = serde_json::from_slice(&output.stdout)
        .map_err(|error| format!("could not decode cargo metadata: {error}"))?;
    let members: std::collections::BTreeSet<_> = metadata.workspace_members.into_iter().collect();
    let mut packages = Vec::new();
    let mut targets = Vec::new();
    let mut dependency_edges = Vec::new();

    for package in metadata
        .packages
        .into_iter()
        .filter(|package| members.contains(&package.id))
    {
        let manifest = Path::new(&package.manifest_path);
        let package_root = manifest
            .parent()
            .ok_or_else(|| format!("manifest has no parent: {}", package.manifest_path))?;
        let feature_definitions = package.features;
        let mut features: Vec<_> = feature_definitions.keys().cloned().collect();
        features.sort();
        packages.push(PackageSurface {
            name: package.name.clone(),
            manifest_path: normalized(manifest),
            package_root: normalized(package_root),
            features: features.clone(),
            feature_definitions,
        });
        for target in package.targets {
            if target.test {
                let mut kinds = target.kind;
                kinds.sort();
                let identity = format!("{}::{}::{}", package.name, kinds.join("+"), target.name);
                if target.doctest && kinds.iter().any(|kind| kind == "lib") {
                    targets.push(TestTargetIdentity {
                        identity: format!("{}::doc::{}", package.name, target.name),
                        package: package.name.clone(),
                        name: target.name.clone(),
                        kinds: vec!["doc".to_owned()],
                        source_path: normalized(Path::new(&target.src_path)),
                        required_features: target.required_features.clone(),
                    });
                    for feature in features.iter().filter(|feature| *feature != "default") {
                        targets.push(TestTargetIdentity {
                            identity: format!("{}::doc+{}::{}", package.name, feature, target.name),
                            package: package.name.clone(),
                            name: target.name.clone(),
                            kinds: vec!["doc".to_owned()],
                            source_path: normalized(Path::new(&target.src_path)),
                            required_features: vec![feature.clone()],
                        });
                    }
                }
                targets.push(TestTargetIdentity {
                    identity,
                    package: package.name.clone(),
                    name: target.name,
                    kinds,
                    source_path: normalized(Path::new(&target.src_path)),
                    required_features: target.required_features,
                });
            }
        }
        for dependency in package.dependencies {
            let mut features = dependency.features;
            features.sort();
            dependency_edges.push(DependencyEdge {
                consumer: package.name.clone(),
                manifest_name: dependency
                    .rename
                    .clone()
                    .unwrap_or_else(|| dependency.name.clone()),
                provider: dependency.name,
                dependency_kind: dependency.kind.unwrap_or_else(|| "normal".to_owned()),
                features,
                optional: dependency.optional,
                uses_default_features: dependency.uses_default_features,
                target: dependency.target,
            });
        }
    }
    packages.sort_by(|left, right| left.name.cmp(&right.name));
    targets.sort();
    dependency_edges.sort();
    Ok(CargoSurface {
        workspace_root: normalized(Path::new(&metadata.workspace_root)),
        target_root: normalized(Path::new(&metadata.target_directory)),
        packages,
        targets,
        build_graph: ObservedBuildGraph { dependency_edges },
    })
}

const fn default_true() -> bool {
    true
}

pub(crate) fn normalized(path: &Path) -> String {
    let normalized = path
        .canonicalize()
        .unwrap_or_else(|_| path.to_path_buf())
        .to_string_lossy()
        .replace('\\', "/");
    normalized
        .strip_prefix("//?/")
        .unwrap_or(&normalized)
        .to_owned()
}
