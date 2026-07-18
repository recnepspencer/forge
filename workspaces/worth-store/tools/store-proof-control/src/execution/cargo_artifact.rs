use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObservedCargoArtifact {
    pub package_id: String,
    pub canonical_package: String,
    pub target_name: String,
    pub target_kinds: Vec<String>,
    pub crate_types: Vec<String>,
    pub features: Vec<String>,
    pub profile_identity: String,
    pub filenames: Vec<String>,
    pub executable: Option<String>,
    pub fresh: bool,
}

impl ObservedCargoArtifact {
    pub fn semantic_identity(&self) -> CargoArtifactSemanticIdentity {
        CargoArtifactSemanticIdentity {
            canonical_package: self.canonical_package.clone(),
            target_name: self.target_name.clone(),
            target_kinds: self.target_kinds.clone(),
            crate_types: self.crate_types.clone(),
        }
    }

    pub fn equivalence_identity(&self) -> CargoArtifactEquivalenceIdentity {
        CargoArtifactEquivalenceIdentity {
            semantic: self.semantic_identity(),
            features: self.features.clone(),
            profile_identity: self.profile_identity.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct CargoArtifactSemanticIdentity {
    pub canonical_package: String,
    pub target_name: String,
    pub target_kinds: Vec<String>,
    pub crate_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct CargoArtifactEquivalenceIdentity {
    pub semantic: CargoArtifactSemanticIdentity,
    pub features: Vec<String>,
    pub profile_identity: String,
}

#[derive(Deserialize)]
struct CompilerArtifactMessage {
    package_id: String,
    target: CompilerArtifactTarget,
    profile: serde_json::Value,
    #[serde(default)]
    features: Vec<String>,
    #[serde(default)]
    filenames: Vec<PathBuf>,
    executable: Option<PathBuf>,
    fresh: bool,
}

#[derive(Deserialize)]
struct CompilerArtifactTarget {
    name: String,
    #[serde(default)]
    kind: Vec<String>,
    #[serde(default)]
    crate_types: Vec<String>,
}

pub(crate) fn read_cargo_artifacts(
    stdout_path: &Path,
) -> Result<Vec<ObservedCargoArtifact>, String> {
    let contents = std::fs::read_to_string(stdout_path)
        .map_err(|error| format!("could not read {}: {error}", stdout_path.display()))?;
    let mut artifacts = Vec::new();
    for (line_index, line) in contents.lines().enumerate() {
        let Ok(value) = serde_json::from_str::<serde_json::Value>(line) else {
            continue;
        };
        if value.get("reason").and_then(serde_json::Value::as_str) != Some("compiler-artifact") {
            continue;
        }
        let message: CompilerArtifactMessage = serde_json::from_value(value).map_err(|error| {
            format!(
                "Cargo compiler-artifact message {} in {} is incomplete: {error}",
                line_index + 1,
                stdout_path.display()
            )
        })?;
        artifacts.push(lower(message)?);
    }
    artifacts.sort();
    artifacts.dedup();
    Ok(artifacts)
}

fn lower(message: CompilerArtifactMessage) -> Result<ObservedCargoArtifact, String> {
    let mut target_kinds = message.target.kind;
    target_kinds.sort();
    target_kinds.dedup();
    let mut crate_types = message.target.crate_types;
    crate_types.sort();
    crate_types.dedup();
    let mut features = message.features;
    features.sort();
    features.dedup();
    let mut filenames: Vec<_> = message.filenames.into_iter().map(normalized).collect();
    filenames.sort();
    filenames.dedup();
    Ok(ObservedCargoArtifact {
        canonical_package: canonical_package(&message.package_id),
        package_id: message.package_id,
        target_name: message.target.name,
        target_kinds,
        crate_types,
        features,
        profile_identity: serde_json::to_string(&message.profile)
            .map_err(|error| format!("could not canonicalize Cargo profile: {error}"))?,
        filenames,
        executable: message.executable.map(normalized),
        fresh: message.fresh,
    })
}

fn canonical_package(package_id: &str) -> String {
    package_id
        .rsplit_once('#')
        .map(|(_, package)| package)
        .unwrap_or(package_id)
        .to_owned()
}

fn normalized(path: PathBuf) -> String {
    path.to_string_lossy().replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiler_artifact_identity_retains_profile_features_and_reuse_status() {
        let root = std::env::temp_dir().join(format!(
            "store-cargo-artifact-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&root).unwrap();
        let path = root.join("cargo.stdout");
        std::fs::write(
            &path,
            concat!(
                "not json\n",
                "{\"reason\":\"compiler-artifact\",\"package_id\":\"path+file:///repo#owner@0.1.0\",",
                "\"target\":{\"name\":\"owner\",\"kind\":[\"lib\"],\"crate_types\":[\"lib\"]},",
                "\"profile\":{\"opt_level\":\"0\",\"test\":true},\"features\":[\"b\",\"a\"],",
                "\"filenames\":[\"target/owner.rlib\"],\"executable\":null,\"fresh\":true}\n"
            ),
        )
        .unwrap();
        let artifacts = read_cargo_artifacts(&path).unwrap();
        assert_eq!(artifacts.len(), 1);
        assert_eq!(artifacts[0].canonical_package, "owner@0.1.0");
        assert_eq!(artifacts[0].features, ["a", "b"]);
        assert!(artifacts[0].fresh);
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn malformed_compiler_artifact_cannot_disappear_as_console_noise() {
        let path = std::env::temp_dir().join(format!(
            "store-malformed-cargo-artifact-{}-{}.stdout",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::write(
            &path,
            "{\"reason\":\"compiler-artifact\",\"fresh\":false}\n",
        )
        .unwrap();
        let denial = read_cargo_artifacts(&path).unwrap_err();
        assert!(denial.contains("compiler-artifact message"));
        std::fs::remove_file(path).unwrap();
    }
}
