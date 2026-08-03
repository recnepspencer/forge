use std::path::Path;
use std::process::Command;

use serde::Deserialize;

use crate::classification::{classify, CiTestLane};

#[derive(Debug, Clone)]
pub(crate) struct TestCatalog {
    packages: Vec<String>,
    targets: Vec<TestTarget>,
    #[cfg(test)]
    binary_targets: Vec<(String, String)>,
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
        #[cfg(test)]
        let mut binary_targets = Vec::new();

        for package in metadata.packages {
            packages.push(package.name.clone());
            for target in package.targets {
                #[cfg(test)]
                if target.kind.iter().any(|kind| kind == "bin") {
                    binary_targets.push((package.name.clone(), target.name.clone()));
                }
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
        #[cfg(test)]
        binary_targets.sort();
        Ok(Self {
            packages,
            targets,
            #[cfg(test)]
            binary_targets,
        })
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

    #[cfg(test)]
    pub(crate) fn binary_target_count(&self, package: &str, target: &str) -> usize {
        self.binary_targets
            .iter()
            .filter(|candidate| candidate.0 == package && candidate.1 == target)
            .count()
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

    #[test]
    fn c4_owns_one_journey_observer_and_ui_target() {
        let catalog = TestCatalog::load(workspace_root()).unwrap();
        for target in ["physical_media_journeys", "physical_media_authority_ui"] {
            let matches = catalog
                .targets()
                .iter()
                .filter(|candidate| candidate.package == "worth-store" && candidate.name == target)
                .count();
            assert_eq!(matches, 1, "C.4 target `{target}` must be unique");
        }
        assert_eq!(
            catalog.binary_target_count("worth-store", "physical_media_os_observer"),
            1,
            "C.4 observer must remain one ordinary binary target"
        );
        assert_eq!(
            catalog.binary_target_count(
                "worth-store-offline-verifier",
                "physical_store_offline_observer",
            ),
            1,
            "C.5 must retain one separately linked offline record observer"
        );
        assert!(
            catalog
                .targets()
                .iter()
                .all(|target| target.name != "physical_media_os_observer"),
            "the dependency-minimal observer must not acquire an empty libtest harness"
        );
        for target in ["physical_media_journeys", "physical_media_authority_ui"] {
            let target = catalog.integration_target("worth-store", target).unwrap();
            assert_eq!(
                target.required_features,
                ["certification-test-authority"],
                "C.4 release proof must declare its feature authority directly"
            );
        }

        let fixture_root =
            workspace_root().join("crates/worth-store/tests/physical_media_authority");
        let mut fixtures = std::fs::read_dir(fixture_root)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.extension().and_then(|value| value.to_str()) == Some("rs"))
            .map(|path| path.file_name().unwrap().to_string_lossy().into_owned())
            .collect::<Vec<_>>();
        fixtures.sort();
        assert_eq!(
            fixtures,
            [
                "maximal_features_cannot_mint_authority.rs",
                "media_runtime_authority_is_sealed.rs",
                "non_authority_values_cannot_promote.rs",
                "optional_capabilities_require_handles.rs",
                "raw_media_surface_is_private.rs",
                "supported_media_admission.rs",
            ]
        );

        let runner = std::fs::read_to_string(
            workspace_root().join("crates/worth-store/tests/physical_media_authority_ui.rs"),
        )
        .unwrap();
        for fixture in fixtures {
            assert!(
                runner.contains(&fixture),
                "unclassified C.4 UI fixture: {fixture}"
            );
        }
    }

    #[test]
    fn worth_store_integration_target_budget_is_explicit() {
        let catalog = TestCatalog::load(workspace_root()).unwrap();
        assert!(
            catalog
                .integration_target("worth-store", "physical_adapter_authority_ui")
                .unwrap()
                .required_features
                .is_empty(),
            "the adapter UI target must retain the ordinary feature graph that it seals"
        );
        assert_eq!(
            catalog
                .integration_target("worth-store", "physical_runtime_authority_ui")
                .unwrap()
                .required_features,
            ["certification-test-authority"],
            "the runtime UI product must exercise the maximal authority profile once"
        );
        let mut targets = catalog
            .targets()
            .iter()
            .filter(|target| {
                target.package == "worth-store"
                    && target.kind == super::CargoTargetKind::Integration
            })
            .map(|target| target.name.as_str())
            .collect::<Vec<_>>();
        targets.sort_unstable();
        assert_eq!(
            targets,
            [
                "physical_adapter_authority_ui",
                "physical_media_authority_ui",
                "physical_media_journeys",
                "physical_record_journeys",
                "physical_runtime_authority_ui",
                "public_facade_downstream",
                "runtime_authority_pressure_journey",
                "sealed_runtime_lifecycle_journey",
            ],
            "a new worth-store integration binary requires an explicit compile-cost review"
        );
    }

    fn workspace_root() -> &'static Path {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .unwrap()
    }
}
