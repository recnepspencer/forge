use std::collections::BTreeSet;
use std::io::Read;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use super::{UiCargoConfigurationIdentity, UiProofRunFailure};

pub(super) fn observe(
    workspace_root: &Path,
) -> Result<Vec<UiCargoConfigurationIdentity>, UiProofRunFailure> {
    let workspace_root = workspace_root.canonicalize().map_err(|error| {
        UiProofRunFailure::EnvironmentObservation(format!(
            "canonicalize Cargo configuration root {}: {error}",
            workspace_root.display()
        ))
    })?;
    configuration_candidates(&workspace_root)
        .into_iter()
        .map(|path| {
            let content_sha256 = if path.is_file() {
                Some(file_digest(&path)?)
            } else {
                None
            };
            Ok(UiCargoConfigurationIdentity {
                path: path.to_string_lossy().replace('\\', "/"),
                content_sha256,
            })
        })
        .collect()
}

fn configuration_candidates(workspace_root: &Path) -> Vec<PathBuf> {
    let mut roots = workspace_root
        .ancestors()
        .map(|root| root.join(".cargo"))
        .collect::<BTreeSet<_>>();
    if let Some(cargo_home) = cargo_home() {
        roots.insert(cargo_home);
    }
    roots
        .into_iter()
        .flat_map(|root| [root.join("config"), root.join("config.toml")])
        .collect()
}

fn cargo_home() -> Option<PathBuf> {
    std::env::var_os("CARGO_HOME")
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var_os(if cfg!(windows) { "USERPROFILE" } else { "HOME" })
                .map(PathBuf::from)
                .map(|home| home.join(".cargo"))
        })
}

fn file_digest(path: &Path) -> Result<String, UiProofRunFailure> {
    let mut file = std::fs::File::open(path)
        .map_err(|error| UiProofRunFailure::EnvironmentObservation(error.to_string()))?;
    let mut digest = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let read = file
            .read(&mut buffer)
            .map_err(|error| UiProofRunFailure::EnvironmentObservation(error.to_string()))?;
        if read == 0 {
            break;
        }
        digest.update(&buffer[..read]);
    }
    Ok(format!("{:x}", digest.finalize()))
}
