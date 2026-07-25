use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
    process::Command,
    time::Duration,
};

use serde::Deserialize;
use worth_store::physical_runtime::{
    PhysicalWorkFeatureGraphEvidence, PhysicalWorkFeatureNodeEvidence,
};

use super::super::super::process_execution;

const METADATA_TIMEOUT: Duration = Duration::from_secs(10);
const ROOT_PACKAGES: [&str; 3] = [
    "worth-store",
    "worth-store-offline-verifier",
    "store-test-runner",
];
const COURTROOM_FEATURES: &str =
    "worth-store/certification-test-authority,store-test-runner/physical-work-evidence";

pub(super) struct RuntimeMetadataEvidence {
    pub(super) package_roots: Vec<PathBuf>,
    pub(super) feature_graph: PhysicalWorkFeatureGraphEvidence,
}

pub(super) fn runtime_metadata_evidence(
    workspace: &Path,
    repository: &Path,
) -> Result<RuntimeMetadataEvidence, String> {
    let metadata = cargo_metadata(workspace)?;
    let packages = metadata
        .packages
        .iter()
        .map(|package| (package.id.as_str(), package))
        .collect::<BTreeMap<_, _>>();
    let nodes = resolved_nodes(&metadata)?;
    let roots = root_package_ids(&metadata)?;
    let package_ids = runtime_package_ids(&roots, &nodes)?;
    let package_roots = local_package_roots(package_ids.clone(), &packages, repository)?;
    let feature_graph = feature_graph(&roots, &package_ids, &nodes)?;
    Ok(RuntimeMetadataEvidence {
        package_roots,
        feature_graph,
    })
}

#[derive(Deserialize)]
struct CargoMetadata {
    packages: Vec<CargoPackage>,
    resolve: Option<CargoResolve>,
}

#[derive(Deserialize)]
struct CargoPackage {
    id: String,
    name: String,
    manifest_path: PathBuf,
    source: Option<String>,
}

#[derive(Deserialize)]
struct CargoResolve {
    nodes: Vec<CargoNode>,
}

#[derive(Deserialize)]
struct CargoNode {
    id: String,
    deps: Vec<CargoDependency>,
    features: Vec<String>,
}

#[derive(Deserialize)]
struct CargoDependency {
    pkg: String,
    dep_kinds: Vec<CargoDependencyKind>,
}

#[derive(Deserialize)]
struct CargoDependencyKind {
    kind: Option<String>,
}

fn cargo_metadata(workspace: &Path) -> Result<CargoMetadata, String> {
    let mut command = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
    command.current_dir(workspace).args([
        "metadata",
        "--format-version",
        "1",
        "--locked",
        "--features",
        COURTROOM_FEATURES,
        "--manifest-path",
    ]);
    command.arg(workspace.join("Cargo.toml"));
    let output = process_execution::run_success_allowing_stderr(
        &mut command,
        METADATA_TIMEOUT,
        "courtroom source metadata",
    )?;
    serde_json::from_str(&output.stdout().join("\n"))
        .map_err(|error| format!("cannot decode courtroom Cargo metadata: {error}"))
}

fn resolved_nodes(metadata: &CargoMetadata) -> Result<BTreeMap<&str, &CargoNode>, String> {
    let resolve = metadata
        .resolve
        .as_ref()
        .ok_or_else(|| "courtroom Cargo metadata omitted its resolved graph".to_owned())?;
    Ok(resolve
        .nodes
        .iter()
        .map(|node| (node.id.as_str(), node))
        .collect())
}

fn runtime_package_ids(
    roots: &[String],
    nodes: &BTreeMap<&str, &CargoNode>,
) -> Result<BTreeSet<String>, String> {
    let mut pending = roots.to_vec();
    let mut visited = BTreeSet::new();
    while let Some(id) = pending.pop() {
        if !visited.insert(id.clone()) {
            continue;
        }
        let node = nodes
            .get(id.as_str())
            .ok_or_else(|| format!("resolved courtroom package `{id}` omitted its node"))?;
        pending.extend(
            node.deps
                .iter()
                .filter(|dependency| runtime_dependency(dependency))
                .map(|dependency| dependency.pkg.clone()),
        );
    }
    Ok(visited)
}

fn feature_graph(
    roots: &[String],
    package_ids: &BTreeSet<String>,
    nodes: &BTreeMap<&str, &CargoNode>,
) -> Result<PhysicalWorkFeatureGraphEvidence, String> {
    let mut evidence = Vec::with_capacity(package_ids.len());
    for id in package_ids {
        let node = nodes
            .get(id.as_str())
            .ok_or_else(|| format!("resolved courtroom package `{id}` omitted its node"))?;
        let dependencies = node
            .deps
            .iter()
            .filter(|dependency| {
                runtime_dependency(dependency) && package_ids.contains(&dependency.pkg)
            })
            .map(|dependency| dependency.pkg.clone());
        evidence.push(
            PhysicalWorkFeatureNodeEvidence::new(id.clone(), node.features.clone(), dependencies)
                .map_err(|denial| format!("courtroom feature node denied: {denial:?}"))?,
        );
    }
    PhysicalWorkFeatureGraphEvidence::new(roots.iter().cloned(), evidence)
        .map_err(|denial| format!("courtroom feature graph denied: {denial:?}"))
}

fn root_package_ids(metadata: &CargoMetadata) -> Result<Vec<String>, String> {
    ROOT_PACKAGES
        .iter()
        .map(|name| {
            let matching = metadata
                .packages
                .iter()
                .filter(|package| package.name == *name && package.source.is_none())
                .collect::<Vec<_>>();
            let [package] = matching.as_slice() else {
                return Err(format!(
                    "courtroom source graph requires one local `{name}` package"
                ));
            };
            Ok(package.id.clone())
        })
        .collect()
}

fn runtime_dependency(dependency: &CargoDependency) -> bool {
    dependency
        .dep_kinds
        .iter()
        .any(|kind| kind.kind.as_deref() != Some("dev"))
}

fn local_package_roots(
    package_ids: BTreeSet<String>,
    packages: &BTreeMap<&str, &CargoPackage>,
    repository: &Path,
) -> Result<Vec<PathBuf>, String> {
    let mut roots = Vec::new();
    for id in package_ids {
        let package = packages
            .get(id.as_str())
            .ok_or_else(|| format!("resolved courtroom package `{id}` omitted metadata"))?;
        if package.source.is_some() {
            continue;
        }
        roots.push(local_package_root(package, repository)?);
    }
    roots.sort();
    roots.dedup();
    Ok(roots)
}

fn local_package_root(package: &CargoPackage, repository: &Path) -> Result<PathBuf, String> {
    let manifest = package
        .manifest_path
        .canonicalize()
        .map_err(|error| format!("cannot canonicalize `{}`: {error}", package.name))?;
    if !manifest.starts_with(repository) {
        return Err(format!(
            "local courtroom package `{}` escaped repository source ownership",
            package.name
        ));
    }
    manifest
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| format!("package `{}` has no manifest parent", package.name))
}
