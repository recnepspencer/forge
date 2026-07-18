use std::path::Path;
use std::process::Command;

use serde::Deserialize;

use crate::classification::{classify, CiTestLane};

#[derive(Debug, Clone)]
pub(crate) struct TestCatalog {
    packages: Vec<String>,
    targets: Vec<TestTarget>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct TestTarget {
    pub(crate) package: String,
    pub(crate) name: String,
    pub(crate) source: String,
    pub(crate) kind: CargoTargetKind,
    pub(crate) test: bool,
    pub(crate) doctest: bool,
    pub(crate) required_features: Vec<String>,
    pub(crate) lane: CiTestLane,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum CargoTargetKind {
    Library,
    Binary,
    Example,
    Bench,
    Integration,
}

impl TestCatalog {
    pub(crate) fn load(workspace_root: &Path) -> Result<Self, String> {
        let output = Command::new("cargo")
            .args(["metadata", "--format-version", "1", "--no-deps"])
            .current_dir(workspace_root)
            .output()
            .map_err(|error| format!("failed to start cargo metadata: {error}"))?;
        if !output.status.success() {
            return Err(format!(
                "cargo metadata failed:\n{}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        let metadata: CargoMetadata = serde_json::from_slice(&output.stdout)
            .map_err(|error| format!("invalid cargo metadata: {error}"))?;
        Self::from_metadata(metadata)
    }

    fn from_metadata(metadata: CargoMetadata) -> Result<Self, String> {
        let mut packages = Vec::new();
        let mut targets = Vec::new();

        for package in metadata.packages {
            packages.push(package.name.clone());
            for target in package.targets {
                if !target.test && !target.doctest {
                    continue;
                }
                let kind = CargoTargetKind::from_metadata(&target.kind).ok_or_else(|| {
                    format!(
                        "unsupported test target kind {:?} for {}::{}",
                        target.kind, package.name, target.name
                    )
                })?;
                let lane = classify(
                    &package.name,
                    &target.name,
                    &target.src_path,
                    kind == CargoTargetKind::Integration,
                )?;
                targets.push(TestTarget {
                    package: package.name.clone(),
                    name: target.name,
                    source: target.src_path,
                    kind,
                    test: target.test,
                    doctest: target.doctest,
                    required_features: target.required_features,
                    lane,
                });
            }
        }

        packages.sort();
        packages.dedup();
        targets.sort_by(|left, right| {
            (&left.package, &left.name, left.kind.rank()).cmp(&(
                &right.package,
                &right.name,
                right.kind.rank(),
            ))
        });
        Ok(Self { packages, targets })
    }

    pub(crate) fn contains_package(&self, package: &str) -> bool {
        self.packages.binary_search(&package.to_owned()).is_ok()
    }

    pub(crate) fn targets(&self) -> &[TestTarget] {
        &self.targets
    }

    pub(crate) fn integration_target(&self, package: &str, target: &str) -> Option<&TestTarget> {
        self.targets.iter().find(|candidate| {
            candidate.package == package
                && candidate.name == target
                && candidate.kind == CargoTargetKind::Integration
        })
    }
}

impl CargoTargetKind {
    fn from_metadata(kinds: &[String]) -> Option<Self> {
        if kinds.iter().any(|kind| kind == "test") {
            Some(Self::Integration)
        } else if kinds
            .iter()
            .any(|kind| kind == "lib" || kind == "proc-macro")
        {
            Some(Self::Library)
        } else if kinds.iter().any(|kind| kind == "bin") {
            Some(Self::Binary)
        } else if kinds.iter().any(|kind| kind == "example") {
            Some(Self::Example)
        } else if kinds.iter().any(|kind| kind == "bench") {
            Some(Self::Bench)
        } else {
            None
        }
    }

    fn rank(self) -> u8 {
        match self {
            Self::Library => 0,
            Self::Binary => 1,
            Self::Example => 2,
            Self::Bench => 3,
            Self::Integration => 4,
        }
    }
}

#[derive(Deserialize)]
struct CargoMetadata {
    packages: Vec<CargoPackage>,
}

#[derive(Deserialize)]
struct CargoPackage {
    name: String,
    targets: Vec<CargoTarget>,
}

#[derive(Deserialize)]
struct CargoTarget {
    name: String,
    kind: Vec<String>,
    src_path: String,
    test: bool,
    doctest: bool,
    #[serde(rename = "required-features", default)]
    required_features: Vec<String>,
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{CargoMetadata, TestCatalog};
    use crate::classification::CiTestLane;
    use crate::product::smoke_cases;

    #[test]
    fn current_workspace_is_totally_classified() {
        let catalog = TestCatalog::load(workspace_root()).unwrap();
        assert!(!catalog.targets().is_empty());
        assert!(catalog
            .targets()
            .iter()
            .any(|target| target.lane == CiTestLane::Ui));
        assert!(catalog
            .targets()
            .iter()
            .any(|target| target.lane == CiTestLane::Formal));
    }

    #[test]
    fn every_smoke_target_resolves() {
        let catalog = TestCatalog::load(workspace_root()).unwrap();
        for smoke in smoke_cases() {
            assert!(
                catalog
                    .integration_target(smoke.package, smoke.target)
                    .is_some(),
                "missing {}::{}",
                smoke.package,
                smoke.target
            );
        }
    }

    #[test]
    fn cargo_required_features_are_preserved_as_execution_authority() {
        let metadata: CargoMetadata = serde_json::from_str(
            r#"{
                "packages": [{
                    "name": "example",
                    "targets": [{
                        "name": "feature_guarded",
                        "kind": ["test"],
                        "src_path": "/repo/example/tests/feature_guarded.rs",
                        "test": true,
                        "doctest": false,
                        "required-features": ["certification"]
                    }]
                }]
            }"#,
        )
        .unwrap();

        let catalog = TestCatalog::from_metadata(metadata).unwrap();
        assert_eq!(catalog.targets()[0].required_features, ["certification"]);
    }

    fn workspace_root() -> &'static Path {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .unwrap()
    }
}
