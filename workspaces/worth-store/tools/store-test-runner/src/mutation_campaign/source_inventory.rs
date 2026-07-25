use std::{
    collections::BTreeSet,
    io::{Read, Seek, SeekFrom},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::{Duration, Instant},
};

use serde::{Deserialize, Serialize};

const SOURCE_BINDING: &str = "worth.store.c5_1.mutation-source-closure.v1";
const METADATA_TIMEOUT: Duration = Duration::from_secs(30);
const REQUIRED_PACKAGES: [&str; 4] = [
    "store-test-runner",
    "worth-store",
    "worth-store-buffer-pool",
    "worth-store-io-scheduler",
];

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct MutationSourceBinding {
    pub(crate) binding: String,
    pub(crate) sha256: String,
}

pub(super) fn bind(workspace: &Path) -> Result<MutationSourceBinding, String> {
    let workspace = workspace
        .canonicalize()
        .map_err(|error| format!("cannot canonicalize Store workspace: {error}"))?;
    let repository = repository_root(&workspace)?;
    let package_roots = local_package_roots(&workspace, &repository)?;
    let build_inputs = build_inputs(&workspace, &repository)?;
    bind_inventory(&repository, &package_roots, &build_inputs)
}

fn repository_root(workspace: &Path) -> Result<PathBuf, String> {
    workspace
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| "Store workspace omitted its repository ancestors".to_owned())?
        .canonicalize()
        .map_err(|error| format!("cannot canonicalize repository root: {error}"))
}

fn local_package_roots(workspace: &Path, repository: &Path) -> Result<Vec<PathBuf>, String> {
    let metadata = cargo_metadata(workspace)?;
    let local_names = metadata
        .packages
        .iter()
        .filter(|package| package.source.is_none())
        .map(|package| package.name.as_str())
        .collect::<BTreeSet<_>>();
    for required in REQUIRED_PACKAGES {
        if !local_names.contains(required) {
            return Err(format!(
                "mutation source inventory omitted required local package `{required}`"
            ));
        }
    }
    let mut roots = metadata
        .packages
        .into_iter()
        .filter(|package| package.source.is_none())
        .map(|package| local_package_root(package, repository))
        .collect::<Result<Vec<_>, _>>()?;
    roots.sort();
    roots.dedup();
    Ok(roots)
}

fn local_package_root(package: CargoPackage, repository: &Path) -> Result<PathBuf, String> {
    let manifest = package
        .manifest_path
        .canonicalize()
        .map_err(|error| format!("cannot canonicalize `{}`: {error}", package.name))?;
    if !manifest.starts_with(repository) {
        return Err(format!(
            "local mutation package `{}` escaped repository source ownership",
            package.name
        ));
    }
    manifest
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| format!("package `{}` has no manifest parent", package.name))
}

fn cargo_metadata(workspace: &Path) -> Result<CargoMetadata, String> {
    let mut command = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
    command.current_dir(workspace).args([
        "metadata",
        "--format-version",
        "1",
        "--locked",
        "--all-features",
        "--manifest-path",
    ]);
    command.arg(workspace.join("Cargo.toml"));
    let encoded = run_bounded(&mut command, METADATA_TIMEOUT, "mutation source metadata")?;
    serde_json::from_slice(&encoded)
        .map_err(|error| format!("cannot decode mutation Cargo metadata: {error}"))
}

fn run_bounded(command: &mut Command, timeout: Duration, label: &str) -> Result<Vec<u8>, String> {
    let mut stdout =
        tempfile::tempfile().map_err(|error| format!("cannot create {label} stdout: {error}"))?;
    let mut stderr =
        tempfile::tempfile().map_err(|error| format!("cannot create {label} stderr: {error}"))?;
    command
        .stdout(Stdio::from(stdout.try_clone().map_err(|error| {
            format!("cannot clone {label} stdout: {error}")
        })?))
        .stderr(Stdio::from(stderr.try_clone().map_err(|error| {
            format!("cannot clone {label} stderr: {error}")
        })?));
    let mut child = command
        .spawn()
        .map_err(|error| format!("cannot spawn {label}: {error}"))?;
    let deadline = Instant::now() + timeout;
    let status = loop {
        if let Some(status) = child
            .try_wait()
            .map_err(|error| format!("cannot inspect {label}: {error}"))?
        {
            break status;
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            return Err(format!("{label} exceeded {}ms", timeout.as_millis()));
        }
        std::thread::sleep(Duration::from_millis(5));
    };
    let stdout = read_captured(&mut stdout, label, "stdout")?;
    let stderr = read_captured(&mut stderr, label, "stderr")?;
    if status.success() {
        Ok(stdout)
    } else {
        Err(format!(
            "{label} exited with {status}\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&stdout),
            String::from_utf8_lossy(&stderr)
        ))
    }
}

fn read_captured(file: &mut std::fs::File, label: &str, stream: &str) -> Result<Vec<u8>, String> {
    file.seek(SeekFrom::Start(0))
        .map_err(|error| format!("cannot rewind {label} {stream}: {error}"))?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .map_err(|error| format!("cannot read {label} {stream}: {error}"))?;
    Ok(bytes)
}

fn build_inputs(workspace: &Path, repository: &Path) -> Result<Vec<PathBuf>, String> {
    let mut inputs = vec![
        repository.join("Cargo.toml"),
        workspace.join("Cargo.toml"),
        workspace.join("Cargo.lock"),
    ];
    for configuration in [
        repository.join(".cargo/config.toml"),
        workspace.join(".cargo/config.toml"),
    ] {
        if configuration.is_file() {
            inputs.push(configuration);
        }
    }
    inputs
        .into_iter()
        .map(|path| {
            path.canonicalize()
                .map_err(|error| format!("cannot bind source input {}: {error}", path.display()))
        })
        .collect()
}

fn bind_inventory(
    repository: &Path,
    package_roots: &[PathBuf],
    build_inputs: &[PathBuf],
) -> Result<MutationSourceBinding, String> {
    let mut sources = build_inputs.to_vec();
    for root in package_roots {
        collect_package_tree(root, &mut sources)?;
    }
    sources.sort();
    sources.dedup();
    let digest = crate::local_source_fingerprint::hash_sources(repository, &sources)?;
    Ok(MutationSourceBinding {
        binding: SOURCE_BINDING.to_owned(),
        sha256: hex(digest),
    })
}

fn collect_package_tree(root: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    let mut pending = vec![root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(&directory)
            .map_err(|error| format!("cannot read source {}: {error}", directory.display()))?
        {
            let entry = entry.map_err(|error| format!("cannot read source entry: {error}"))?;
            let kind = entry
                .file_type()
                .map_err(|error| format!("cannot inspect source entry: {error}"))?;
            if kind.is_symlink() {
                return Err(format!(
                    "source inventory refuses symbolic link {}",
                    entry.path().display()
                ));
            }
            if kind.is_dir() {
                if !matches!(entry.file_name().to_str(), Some("target") | Some(".git")) {
                    pending.push(entry.path());
                }
            } else if kind.is_file() {
                files.push(entry.path());
            }
        }
    }
    Ok(())
}

fn hex(bytes: [u8; 32]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[derive(Deserialize)]
struct CargoMetadata {
    packages: Vec<CargoPackage>,
}

#[derive(Deserialize)]
struct CargoPackage {
    name: String,
    manifest_path: PathBuf,
    source: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{bind, bind_inventory, SOURCE_BINDING};

    #[test]
    fn real_source_inventory_binds_the_complete_store_workspace() {
        let binding = bind(&crate::workspace_root()).unwrap();

        assert_eq!(binding.binding, SOURCE_BINDING);
        assert_eq!(binding.sha256.len(), 64);
    }

    #[test]
    fn integration_test_source_drift_changes_the_campaign_binding() {
        let repository = tempfile::tempdir().unwrap();
        let package = repository.path().join("package");
        std::fs::create_dir_all(package.join("tests")).unwrap();
        let test = package.join("tests/physical.rs");
        std::fs::write(&test, b"original").unwrap();
        let before =
            bind_inventory(repository.path(), std::slice::from_ref(&package), &[]).unwrap();

        std::fs::write(test, b"mutation").unwrap();
        let after = bind_inventory(repository.path(), &[package], &[]).unwrap();

        assert_eq!(before.binding, SOURCE_BINDING);
        assert_ne!(before.sha256, after.sha256);
    }

    #[test]
    fn build_configuration_participates_in_the_campaign_binding() {
        let repository = tempfile::tempdir().unwrap();
        let configuration = repository.path().join("config.toml");
        std::fs::write(&configuration, b"[build]\n").unwrap();
        let before =
            bind_inventory(repository.path(), &[], std::slice::from_ref(&configuration)).unwrap();

        std::fs::write(&configuration, b"[alias]\n").unwrap();
        let after = bind_inventory(repository.path(), &[], &[configuration]).unwrap();

        assert_ne!(before.sha256, after.sha256);
    }
}
