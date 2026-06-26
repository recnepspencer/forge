use std::fs;
use std::path::{Path, PathBuf};

use crate::validation_authority_inventory::discovery::WorthValidationAuthorityScanPattern;
use crate::validation_authority_inventory::error::WorthValidationAuthorityInventoryError;
use crate::validation_authority_inventory::inventory::WorthValidationAuthorityInventory;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthValidationAuthoritySourceFirewallViolation {
    path: PathBuf,
    pattern: &'static str,
}

impl WorthValidationAuthoritySourceFirewallViolation {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub const fn pattern(&self) -> &'static str {
        self.pattern
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthValidationAuthoritySourceFirewallReport {
    scanned_file_count: usize,
    violations: Vec<WorthValidationAuthoritySourceFirewallViolation>,
}

impl WorthValidationAuthoritySourceFirewallReport {
    pub fn scan_default_workspace() -> Result<Self, WorthValidationAuthorityInventoryError> {
        Self::scan_root(Path::new("crates/worth-topo/src"))
    }

    pub fn scan_root(root: &Path) -> Result<Self, WorthValidationAuthorityInventoryError> {
        let mut report = Self {
            scanned_file_count: 0,
            violations: Vec::new(),
        };
        scan_dir(root, &mut report)?;
        Ok(report)
    }

    pub fn scan_root_against_inventory(
        root: &Path,
        inventory: &WorthValidationAuthorityInventory,
    ) -> Result<Self, WorthValidationAuthorityInventoryError> {
        let discovery =
            crate::validation_authority_inventory::discovery::WorthValidationAuthorityDiscoveryReport::scan_root(root)?;
        let mut report = Self {
            scanned_file_count: discovery.scanned_file_count(),
            violations: Vec::new(),
        };
        for discovered in discovery.discovered_sources() {
            if !inventory.contains_discovered_source(discovered) {
                report
                    .violations
                    .push(WorthValidationAuthoritySourceFirewallViolation {
                        path: discovered.path().to_path_buf(),
                        pattern: discovered.pattern().pattern(),
                    });
            }
        }
        Ok(report)
    }

    pub const fn scanned_file_count(&self) -> usize {
        self.scanned_file_count
    }

    pub fn violations(&self) -> &[WorthValidationAuthoritySourceFirewallViolation] {
        &self.violations
    }

    pub fn ensure_clean(&self) -> Result<(), WorthValidationAuthorityInventoryError> {
        if self.violations.is_empty() {
            return Ok(());
        }
        let violation_summary = self
            .violations
            .iter()
            .map(|violation| {
                format!(
                    "{} contains `{}`",
                    violation.path.display(),
                    violation.pattern
                )
            })
            .collect::<Vec<_>>()
            .join("; ");
        Err(WorthValidationAuthorityInventoryError::SourceFirewallViolation(violation_summary))
    }
}

fn scan_dir(
    dir: &Path,
    report: &mut WorthValidationAuthoritySourceFirewallReport,
) -> Result<(), WorthValidationAuthorityInventoryError> {
    let entries = fs::read_dir(dir).map_err(|error| {
        WorthValidationAuthorityInventoryError::SourceFirewallViolation(format!(
            "cannot scan {}: {error}",
            dir.display()
        ))
    })?;

    for entry in entries {
        let entry = entry.map_err(|error| {
            WorthValidationAuthorityInventoryError::SourceFirewallViolation(error.to_string())
        })?;
        let path = entry.path();
        if path.is_dir() {
            scan_dir(&path, report)?;
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            scan_file(&path, report)?;
        }
    }
    Ok(())
}

fn scan_file(
    path: &Path,
    report: &mut WorthValidationAuthoritySourceFirewallReport,
) -> Result<(), WorthValidationAuthorityInventoryError> {
    report.scanned_file_count += 1;
    if is_allowed_old_authority_path(path) {
        return Ok(());
    }
    let text = fs::read_to_string(path).map_err(|error| {
        WorthValidationAuthorityInventoryError::SourceFirewallViolation(format!(
            "cannot read {}: {error}",
            path.display()
        ))
    })?;
    for pattern in WorthValidationAuthorityScanPattern::all() {
        if text.contains(pattern.pattern()) {
            report
                .violations
                .push(WorthValidationAuthoritySourceFirewallViolation {
                    path: path.to_path_buf(),
                    pattern: pattern.pattern(),
                });
        }
    }
    Ok(())
}

fn is_allowed_old_authority_path(path: &Path) -> bool {
    let normalized = path.to_string_lossy().replace('\\', "/");
    ALLOWED_OLD_AUTHORITY_PATHS
        .iter()
        .any(|allowed| normalized.ends_with(allowed))
        || normalized.contains("/validation_authority_inventory/")
        || normalized.contains("/certification/public_facade_contracts/compile_fail")
}

const ALLOWED_OLD_AUTHORITY_PATHS: &[&str] = &[
    "validation/facade.rs",
    "validation/rule_registry.rs",
    "validation/reference_integrity/mod.rs",
    "runtime_support.rs",
    "certification/core.rs",
    "certification/requirements.rs",
    "certification/shared.rs",
    "certification/authority_closeout/closures.rs",
    "certification/authority_closeout/aggregates_a.rs",
    "certification/topology_operator_closeout/shared.rs",
    "certification/topology_operator_closeout/acceptance_rows/validator_family_coverage.rs",
    "certification/topology_operator_closeout/validation_breadth_row.rs",
    "certification/topology_operator_closeout/report.rs",
    "certification/topology_operator_closeout/acceptance_rows/validation_breadth/mod.rs",
    "workload_platform/topology_seed/seed_recipe.rs",
    "workload_platform/nmt_topology_construction/construction.rs",
];
