use super::crate_modules::GovernedCrate;
use crate::cargo_graph::{normalize_path, package_name_from_manifest};
use std::fs;
use std::path::Path;

pub(super) fn load(
    root: &Path,
    relative_crate_root: &str,
    expected_package: &str,
) -> Result<GovernedCrate, String> {
    let canonical_root = fs::canonicalize(root)
        .map_err(|error| format!("canonicalize repository root {}: {error}", root.display()))?;
    let requested_root = root.join(relative_crate_root);
    let crate_root = fs::canonicalize(&requested_root).map_err(|error| {
        format!(
            "configured public-value crate root is missing or unresolved: {}: {error}",
            requested_root.display()
        )
    })?;
    if !crate_root.starts_with(&canonical_root) {
        return Err(format!(
            "configured public-value crate root escapes the repository root: {relative_crate_root}"
        ));
    }
    let manifest = crate_root.join("Cargo.toml");
    if !crate_root.is_dir() || !manifest.is_file() {
        return Err(format!(
            "configured public-value crate root is missing: {}",
            crate_root.display()
        ));
    }
    let package = package_name_from_manifest(&manifest)?;
    if package != expected_package {
        return Err(format!(
            "configured public-value package `{expected_package}` resolves to manifest package `{package}` at {}",
            manifest.display()
        ));
    }
    let relative = normalize_path(
        crate_root
            .strip_prefix(&canonical_root)
            .map_err(|error| format!("strip root from {}: {error}", crate_root.display()))?,
    );
    if relative != normalize_path(Path::new(relative_crate_root)) {
        return Err(format!(
            "configured public-value crate root escapes or aliases the repository root: {relative_crate_root}"
        ));
    }
    Ok(GovernedCrate {
        package,
        crate_root,
        relative_crate_root: relative,
    })
}
