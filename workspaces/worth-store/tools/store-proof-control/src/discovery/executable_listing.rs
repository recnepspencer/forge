use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

use serde::{Deserialize, Serialize};

use super::rustdoc_listing::{observe_rustdoc_listings, validate_rustdoc_listing};
use super::{CaseKind, TestSurfaceInventory};
use crate::evidence::sha256_serialized;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentExecutableListing {
    pub schema_version: u32,
    pub test_topology_digest: String,
    pub environment: ExecutableListingEnvironment,
    pub libtest_targets: Vec<ExecutableTargetListing>,
    pub rustdoc_targets: Vec<ExecutableTargetListing>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutableListingEnvironment {
    pub rustc_identity: String,
    pub cargo_identity: String,
    pub operating_system: String,
    pub architecture: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutableTargetListing {
    pub target_identity: String,
    pub listed_cases: Vec<String>,
}

pub fn observe_executable_listing(
    workspace_root: &Path,
    inventory: &TestSurfaceInventory,
) -> Result<CurrentExecutableListing, String> {
    let package_names = package_id_names(workspace_root)?;
    let output = Command::new("cargo")
        .args(["test", "--workspace", "--no-run", "--message-format=json"])
        .current_dir(workspace_root)
        .output()
        .map_err(|error| format!("could not launch executable listing build: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "executable listing build failed: {}",
            super::cargo_diagnostics::rendered_failure(&output.stdout, &output.stderr)
        ));
    }
    let mut libtest_targets = artifact_listings(&output.stdout, &package_names)?;
    for target in inventory
        .targets
        .iter()
        .filter(|target| !target.required_features.is_empty())
        .filter(|target| !target.kinds.iter().any(|kind| kind == "doc"))
    {
        libtest_targets.extend(feature_gated_target_listing(
            workspace_root,
            target,
            &package_names,
        )?);
    }
    libtest_targets.sort_by(|left, right| left.target_identity.cmp(&right.target_identity));
    libtest_targets.dedup_by(|left, right| left.target_identity == right.target_identity);
    let rustdoc_targets = observe_rustdoc_listings(workspace_root, inventory)?;
    Ok(CurrentExecutableListing {
        schema_version: 2,
        test_topology_digest: topology_digest(inventory)?,
        environment: observe_listing_environment(workspace_root)?,
        libtest_targets,
        rustdoc_targets,
    })
}

fn artifact_listings(
    stdout: &[u8],
    package_names: &BTreeMap<String, String>,
) -> Result<Vec<ExecutableTargetListing>, String> {
    let mut listings = Vec::new();
    for line in String::from_utf8_lossy(stdout).lines() {
        let Ok(event) = serde_json::from_str::<CargoArtifactEvent>(line) else {
            continue;
        };
        if event.reason != "compiler-artifact" || !event.profile.test {
            continue;
        }
        let Some(executable) = event.executable else {
            continue;
        };
        let package = package_names
            .get(event.package_id.as_str())
            .ok_or_else(|| {
                format!(
                    "Cargo artifact has unknown package id: {}",
                    event.package_id
                )
            })?;
        let mut kinds = event.target.kind;
        kinds.sort();
        let target_identity = format!("{}::{}::{}", package, kinds.join("+"), event.target.name);
        listings.push(ExecutableTargetListing {
            target_identity,
            listed_cases: list_executable_cases(Path::new(&executable))?,
        });
    }
    Ok(listings)
}

fn feature_gated_target_listing(
    workspace_root: &Path,
    target: &super::TestTargetIdentity,
    package_names: &BTreeMap<String, String>,
) -> Result<Vec<ExecutableTargetListing>, String> {
    let mut arguments = vec!["test".to_owned(), "-p".to_owned(), target.package.clone()];
    if target.kinds.iter().any(|kind| kind == "lib") {
        arguments.push("--lib".to_owned());
    } else if target.kinds.iter().any(|kind| kind == "bin") {
        arguments.extend(["--bin".to_owned(), target.name.clone()]);
    } else {
        arguments.extend(["--test".to_owned(), target.name.clone()]);
    }
    arguments.extend([
        "--features".to_owned(),
        target.required_features.join(","),
        "--no-run".to_owned(),
        "--message-format=json".to_owned(),
    ]);
    let output = Command::new("cargo")
        .args(arguments)
        .current_dir(workspace_root)
        .output()
        .map_err(|error| {
            format!(
                "could not build feature-gated target {}: {error}",
                target.identity
            )
        })?;
    if !output.status.success() {
        return Err(format!(
            "feature-gated listing build failed for {}: {}",
            target.identity,
            super::cargo_diagnostics::rendered_failure(&output.stdout, &output.stderr)
        ));
    }
    artifact_listings(&output.stdout, package_names)
}

pub fn validate_executable_listing(
    inventory: &TestSurfaceInventory,
    listing: &CurrentExecutableListing,
) -> Result<(), Vec<String>> {
    let mut violations = Vec::new();
    if listing.schema_version != 2 {
        violations.push(format!(
            "unsupported current executable listing schema: {}",
            listing.schema_version
        ));
    }
    match observe_listing_environment(Path::new(&inventory.workspace_root)) {
        Ok(environment) if environment != listing.environment => violations.push(
            "current executable listing was observed under a different Cargo, Rust, OS, or architecture identity"
                .to_owned(),
        ),
        Err(error) => violations.push(error),
        _ => {}
    }
    match topology_digest(inventory) {
        Ok(digest) if digest != listing.test_topology_digest => violations.push(
            "current executable listing is stale for the discovered test topology".to_owned(),
        ),
        Err(error) => violations.push(error),
        _ => {}
    }
    super::libtest_listing::validate_libtest_listing(inventory, listing, &mut violations);
    validate_rustdoc_listing(inventory, listing, &mut violations);
    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations)
    }
}

fn observe_listing_environment(
    workspace_root: &Path,
) -> Result<ExecutableListingEnvironment, String> {
    Ok(ExecutableListingEnvironment {
        rustc_identity: command_identity(workspace_root, "rustc", &["-Vv"])?,
        cargo_identity: command_identity(workspace_root, "cargo", &["-Vv"])?,
        operating_system: std::env::consts::OS.to_owned(),
        architecture: std::env::consts::ARCH.to_owned(),
    })
}

fn command_identity(
    workspace_root: &Path,
    program: &str,
    arguments: &[&str],
) -> Result<String, String> {
    let output = Command::new(program)
        .args(arguments)
        .current_dir(workspace_root)
        .output()
        .map_err(|error| format!("could not observe {program} identity: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "{program} identity observation failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn package_id_names(workspace_root: &Path) -> Result<BTreeMap<String, String>, String> {
    let output = Command::new("cargo")
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .current_dir(workspace_root)
        .output()
        .map_err(|error| format!("could not launch listing metadata: {error}"))?;
    let metadata: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|error| format!("could not decode listing metadata: {error}"))?;
    Ok(metadata["packages"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|package| {
            Some((
                package["id"].as_str()?.to_owned(),
                package["name"].as_str()?.to_owned(),
            ))
        })
        .collect())
}

fn list_executable_cases(executable: &Path) -> Result<Vec<String>, String> {
    let output = Command::new(executable)
        .args(["--list", "--format", "terse"])
        .output()
        .map_err(|error| format!("could not list {}: {error}", executable.display()))?;
    if !output.status.success() {
        return Err(format!(
            "libtest listing failed for {}: {}",
            executable.display(),
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(parse_harness_listing(&output.stdout))
}

pub(super) fn parse_harness_listing(bytes: &[u8]) -> Vec<String> {
    String::from_utf8_lossy(bytes)
        .lines()
        .filter_map(|line| {
            line.strip_suffix(": test")
                .or_else(|| line.strip_suffix(": benchmark"))
        })
        .map(str::to_owned)
        .collect()
}

fn topology_digest(inventory: &TestSurfaceInventory) -> Result<String, String> {
    let basis: Vec<_> = inventory
        .cases
        .iter()
        .filter(|case| {
            matches!(
                case.kind,
                CaseKind::RustTest
                    | CaseKind::DoctestRunnable
                    | CaseKind::DoctestCompileFail
                    | CaseKind::DoctestIgnored
            )
        })
        .map(|case| {
            (
                &case.identity.stable_id,
                &case.target_identity,
                case.kind,
                case.ignored,
            )
        })
        .collect();
    sha256_serialized(&basis)
}

#[derive(Deserialize)]
struct CargoArtifactEvent {
    reason: String,
    package_id: String,
    target: CargoArtifactTarget,
    profile: CargoArtifactProfile,
    executable: Option<String>,
}

#[derive(Deserialize)]
struct CargoArtifactTarget {
    name: String,
    kind: Vec<String>,
}

#[derive(Deserialize)]
struct CargoArtifactProfile {
    test: bool,
}
