use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use super::{dependency_boundary, destination, retirement};

#[derive(Debug, Eq, PartialEq)]
pub struct HostRetirementTopologyFailure {
    violations: Box<[String]>,
}

impl HostRetirementTopologyFailure {
    pub fn violations(&self) -> &[String] {
        &self.violations
    }
}

impl fmt::Display for HostRetirementTopologyFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.violations.join("\n"))
    }
}

pub struct HostRetirementTopologyWorld {
    repository_root: PathBuf,
}

impl HostRetirementTopologyWorld {
    pub fn capture(repository_root: impl Into<PathBuf>) -> Self {
        Self {
            repository_root: repository_root.into(),
        }
    }

    pub fn certify(&self) -> Result<(), HostRetirementTopologyFailure> {
        let mut failures = Vec::new();
        self.audit_retired_paths(&mut failures);
        self.walk(&self.repository_root, &mut failures);
        self.audit_compile_twins(&mut failures);
        failures.sort();
        failures.dedup();
        if failures.is_empty() {
            Ok(())
        } else {
            Err(HostRetirementTopologyFailure {
                violations: failures.into_boxed_slice(),
            })
        }
    }

    pub fn assert_hiding_mutations_rejected() {
        assert!(!dependency_boundary::mutation_controls().is_empty());
        let retirement_failures = retirement::mutation_controls();
        assert!(retirement_failures
            .iter()
            .any(|failure| failure.contains("unregistered_retired_host.rs")));
        assert!(retirement_failures.iter().any(|failure| failure
            .contains("unregistered/tests/ui/host/retired_egui_surface_is_absent.rs")));
        let mut failures = Vec::new();
        retirement::audit_compile_twins("", "", &mut failures);
        assert!(!failures.is_empty());
    }

    fn audit_retired_paths(&self, failures: &mut Vec<String>) {
        for retired in destination::RETIRED_CRATE_DIRECTORIES
            .into_iter()
            .chain(destination::RETIRED_PRODUCT_FILES)
        {
            let path = self.repository_root.join(retired);
            if path.is_file() || directory_contains_files(&path) {
                failures.push(format!("retired path still exists: {retired}"));
            }
        }
    }

    fn walk(&self, directory: &Path, failures: &mut Vec<String>) {
        let entries = fs::read_dir(directory)
            .unwrap_or_else(|error| panic!("{} should be readable: {error}", directory.display()));
        for entry in entries {
            let entry = entry.expect("repository entry should be readable");
            let path = entry.path();
            let relative = path
                .strip_prefix(&self.repository_root)
                .expect("walked path stays below repository root");
            if destination::is_ignored_tree(relative) {
                continue;
            }
            retirement::audit_path(relative, failures);
            if path.is_dir() {
                self.walk(&path, failures);
            } else {
                self.audit_file(relative, &path, failures);
            }
        }
    }

    fn audit_file(&self, relative: &Path, path: &Path, failures: &mut Vec<String>) {
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            return;
        };
        let Ok(text) = fs::read_to_string(path) else {
            return;
        };
        if name == "Cargo.toml" && !destination::is_exact_negative_fixture(relative) {
            dependency_boundary::audit_manifest(relative, &text, failures);
        } else if name == "Cargo.lock" {
            dependency_boundary::audit_lockfile(relative, &text, failures);
        }
        retirement::audit_source(relative, &text, failures);
    }

    fn audit_compile_twins(&self, failures: &mut Vec<String>) {
        let manifest =
            fs::read_to_string(destination::compile_fixture_manifest(&self.repository_root))
                .expect("compile fixture manifest should be readable");
        let cases = fs::read_to_string(destination::compile_case_inventory(&self.repository_root))
            .expect("compile case inventory should be readable");
        retirement::audit_compile_twins(&manifest, &cases, failures);
    }
}

fn directory_contains_files(path: &Path) -> bool {
    if !path.is_dir() {
        return false;
    }
    fs::read_dir(path)
        .expect("retired directory should be readable")
        .any(|entry| {
            let path = entry
                .expect("retired directory entry should be readable")
                .path();
            path.is_file() || directory_contains_files(&path)
        })
}
