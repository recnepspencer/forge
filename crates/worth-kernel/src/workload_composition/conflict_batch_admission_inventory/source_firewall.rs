use std::path::{Path, PathBuf};

use super::closeout::ConflictBatchAdmissionInventory;
use super::discovery::ConflictBatchAdmissionDiscoveryReport;
use super::error::ConflictBatchAdmissionInventoryError;
use super::scan_pattern::ConflictBatchAdmissionScanPattern;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConflictBatchAdmissionSourceFirewallViolation {
    path: PathBuf,
    surface_name: String,
    pattern: ConflictBatchAdmissionScanPattern,
}

impl ConflictBatchAdmissionSourceFirewallViolation {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn surface_name(&self) -> &str {
        &self.surface_name
    }

    pub const fn scan_pattern(&self) -> ConflictBatchAdmissionScanPattern {
        self.pattern
    }

    pub const fn pattern(&self) -> &'static str {
        self.pattern.pattern()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConflictBatchAdmissionSourceFirewallReport {
    scanned_file_count: usize,
    violations: Vec<ConflictBatchAdmissionSourceFirewallViolation>,
}

impl ConflictBatchAdmissionSourceFirewallReport {
    pub fn scan_default_workspace_against_inventory(
        inventory: &ConflictBatchAdmissionInventory,
    ) -> Result<Self, ConflictBatchAdmissionInventoryError> {
        Self::scan_roots_against_inventory(&default_workspace_scan_roots(), inventory)
    }

    pub fn scan_root_against_inventory(
        root: &Path,
        inventory: &ConflictBatchAdmissionInventory,
    ) -> Result<Self, ConflictBatchAdmissionInventoryError> {
        Self::scan_roots_against_inventory(&[root], inventory)
    }

    pub fn scan_roots_against_inventory(
        roots: &[impl AsRef<Path>],
        inventory: &ConflictBatchAdmissionInventory,
    ) -> Result<Self, ConflictBatchAdmissionInventoryError> {
        let discovery = ConflictBatchAdmissionDiscoveryReport::scan_roots(roots)?;
        let violations = discovery
            .discovered_surfaces()
            .iter()
            .filter(|surface| !inventory.contains_discovered_surface(surface))
            .map(|surface| ConflictBatchAdmissionSourceFirewallViolation {
                path: surface.path().to_path_buf(),
                surface_name: surface.surface_name().to_owned(),
                pattern: surface.pattern(),
            })
            .collect();
        Ok(Self {
            scanned_file_count: discovery.scanned_file_count(),
            violations,
        })
    }

    pub const fn scanned_file_count(&self) -> usize {
        self.scanned_file_count
    }

    pub fn violations(&self) -> &[ConflictBatchAdmissionSourceFirewallViolation] {
        &self.violations
    }

    pub fn ensure_clean(&self) -> Result<(), ConflictBatchAdmissionInventoryError> {
        if self.violations.is_empty() {
            return Ok(());
        }
        Err(
            ConflictBatchAdmissionInventoryError::SourceFirewallViolation(
                self.violations
                    .iter()
                    .map(|violation| {
                        format!(
                            "{} contains `{}` on `{}`",
                            violation.path().display(),
                            violation.pattern(),
                            violation.surface_name()
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("; "),
            ),
        )
    }
}

fn default_workspace_scan_roots() -> [PathBuf; 3] {
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("worth-kernel should live under workspace/crates/worth-kernel")
        .to_path_buf();
    [
        workspace_root.join("crates/worth-kernel/src/workload_composition"),
        workspace_root.join("crates/worth-topo/src"),
        workspace_root.join("crates/worth-spatial/src"),
    ]
}
