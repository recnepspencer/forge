use std::fs;
use std::path::Path;

use super::discovered_source::WorthValidationAuthorityDiscoveredSource;
use super::scan_pattern::WorthValidationAuthorityScanPattern;
use crate::validation_authority_inventory::error::WorthValidationAuthorityInventoryError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthValidationAuthorityDiscoveryReport {
    scanned_file_count: usize,
    discovered_sources: Vec<WorthValidationAuthorityDiscoveredSource>,
}

impl WorthValidationAuthorityDiscoveryReport {
    pub fn scan_default_workspace() -> Result<Self, WorthValidationAuthorityInventoryError> {
        Self::scan_root(Path::new("crates/worth-topo/src"))
    }

    pub fn scan_root(root: &Path) -> Result<Self, WorthValidationAuthorityInventoryError> {
        let mut report = Self {
            scanned_file_count: 0,
            discovered_sources: Vec::new(),
        };
        scan_dir(root, &mut report)?;
        Ok(report)
    }

    pub const fn scanned_file_count(&self) -> usize {
        self.scanned_file_count
    }

    pub fn discovered_sources(&self) -> &[WorthValidationAuthorityDiscoveredSource] {
        &self.discovered_sources
    }
}

fn scan_dir(
    dir: &Path,
    report: &mut WorthValidationAuthorityDiscoveryReport,
) -> Result<(), WorthValidationAuthorityInventoryError> {
    let entries = fs::read_dir(dir).map_err(|error| {
        WorthValidationAuthorityInventoryError::SourceDiscoveryFailure(format!(
            "cannot scan {}: {error}",
            dir.display()
        ))
    })?;

    for entry in entries {
        let entry = entry.map_err(|error| {
            WorthValidationAuthorityInventoryError::SourceDiscoveryFailure(error.to_string())
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
    report: &mut WorthValidationAuthorityDiscoveryReport,
) -> Result<(), WorthValidationAuthorityInventoryError> {
    report.scanned_file_count += 1;
    let text = fs::read_to_string(path).map_err(|error| {
        WorthValidationAuthorityInventoryError::SourceDiscoveryFailure(format!(
            "cannot read {}: {error}",
            path.display()
        ))
    })?;
    for pattern in WorthValidationAuthorityScanPattern::all() {
        if text.contains(pattern.pattern()) {
            report
                .discovered_sources
                .push(WorthValidationAuthorityDiscoveredSource::from_parts(
                    path.to_path_buf(),
                    *pattern,
                ));
        }
    }
    Ok(())
}
