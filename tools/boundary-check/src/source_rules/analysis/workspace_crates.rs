//! Discover crate roots owned by one configured Cargo workspace.

use super::crate_modules::GovernedCrate;
use crate::cargo_graph::{normalize_path, package_name_from_manifest};
use std::fs;
use std::path::Path;

pub(super) fn discover_workspace_crates(
    root: &Path,
    relative_workspace: &str,
) -> Result<Vec<GovernedCrate>, String> {
    let member_lane = root.join(relative_workspace).join("crates");
    let mut crates = Vec::new();
    if !member_lane.is_dir() {
        return Ok(crates);
    }
    for entry in fs::read_dir(&member_lane)
        .map_err(|error| format!("read member lane {}: {error}", member_lane.display()))?
    {
        let crate_path = entry
            .map_err(|error| format!("read member entry: {error}"))?
            .path();
        let manifest = crate_path.join("Cargo.toml");
        if !crate_path.is_dir() || !manifest.is_file() {
            continue;
        }
        let package = package_name_from_manifest(&manifest)?;
        let relative_crate_root = normalize_path(
            crate_path
                .strip_prefix(root)
                .map_err(|error| format!("strip root from {}: {error}", crate_path.display()))?,
        );
        crates.push(GovernedCrate {
            package,
            crate_root: crate_path,
            relative_crate_root,
        });
    }
    crates.sort_by(|left, right| left.package.cmp(&right.package));
    Ok(crates)
}
