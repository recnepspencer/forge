//! Carry the governed Cargo workspace profile tables into witness consumers.

use super::bounded_process;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::process::Command;

pub(super) struct WorkspaceProfiles {
    source: String,
}

impl WorkspaceProfiles {
    pub(super) fn load(crate_root: &Path, limits: bounded_process::Limits) -> Result<Self, String> {
        let workspace_root = cargo_workspace_root(crate_root, limits)?;
        let workspace_manifest = workspace_root.join("Cargo.toml");
        let crate_manifest = crate_root.join("Cargo.toml");
        let workspace_value = read_manifest(&workspace_manifest)?;
        let crate_value = read_manifest(&crate_manifest)?;
        reject_ignored_profile_tables(
            &crate_value,
            crate_manifest
                .canonicalize()
                .map_err(display_io("canonicalize", &crate_manifest))?
                != workspace_manifest
                    .canonicalize()
                    .map_err(display_io("canonicalize", &workspace_manifest))?,
        )?;
        let source = serialize_profile_table(&workspace_value)?;
        Ok(Self { source })
    }

    pub(super) fn validate_profile(&self, profile: &str) -> Result<(), String> {
        if profile.is_empty()
            || !profile.chars().all(|character| {
                character.is_ascii_alphanumeric() || matches!(character, '-' | '_')
            })
        {
            return Err(format!("invalid Cargo profile name `{profile}`"));
        }
        if matches!(profile, "dev" | "release") || profile_table_has(&self.source, profile)? {
            Ok(())
        } else {
            Err(format!(
                "custom Cargo profile `{profile}` is absent from the governed workspace profile table"
            ))
        }
    }

    pub(super) fn manifest_source(&self) -> &str {
        &self.source
    }
}

#[derive(Deserialize)]
struct CargoMetadata {
    workspace_root: PathBuf,
}

fn cargo_workspace_root(
    crate_root: &Path,
    limits: bounded_process::Limits,
) -> Result<PathBuf, String> {
    let mut command = Command::new(cargo());
    command
        .current_dir(crate_root)
        .args([
            "metadata",
            "--no-deps",
            "--format-version",
            "1",
            "--manifest-path",
        ])
        .arg(crate_root.join("Cargo.toml"));
    let output = bounded_process::run(&mut command, None, limits, "Cargo workspace metadata")?;
    if !output.status.success() {
        return Err(format!(
            "Cargo rejected governed workspace metadata: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    let metadata: CargoMetadata = serde_json::from_slice(&output.stdout)
        .map_err(|error| format!("parse Cargo workspace metadata: {error}"))?;
    metadata.workspace_root.canonicalize().map_err(display_io(
        "canonicalize Cargo workspace root",
        &metadata.workspace_root,
    ))
}

fn read_manifest(path: &Path) -> Result<toml::Value, String> {
    let source = std::fs::read_to_string(path).map_err(display_io("read Cargo manifest", path))?;
    toml::from_str(&source)
        .map_err(|error| format!("parse Cargo manifest {}: {error}", path.display()))
}

fn reject_ignored_profile_tables(
    manifest: &toml::Value,
    is_workspace_member: bool,
) -> Result<(), String> {
    if manifest
        .get("package")
        .and_then(|package| package.get("profile"))
        .is_some()
    {
        return Err("Cargo profile configuration nested under `[package]` is ignored".to_owned());
    }
    if is_workspace_member && manifest.get("profile").is_some() {
        return Err(
            "Cargo ignores `[profile]` tables outside the governed workspace root manifest"
                .to_owned(),
        );
    }
    Ok(())
}

fn serialize_profile_table(manifest: &toml::Value) -> Result<String, String> {
    let Some(profile) = manifest.get("profile") else {
        return Ok(String::new());
    };
    let mut root = toml::map::Map::new();
    root.insert("profile".to_owned(), profile.clone());
    toml::to_string(&toml::Value::Table(root))
        .map_err(|error| format!("serialize governed workspace profile table: {error}"))
}

fn profile_table_has(source: &str, profile: &str) -> Result<bool, String> {
    if source.is_empty() {
        return Ok(false);
    }
    let value: toml::Value = toml::from_str(source)
        .map_err(|error| format!("reparse governed workspace profile table: {error}"))?;
    Ok(value
        .get("profile")
        .and_then(|profiles| profiles.get(profile))
        .is_some())
}

fn display_io<'a>(
    action: &'static str,
    path: &'a Path,
) -> impl FnOnce(std::io::Error) -> String + 'a {
    move |error| format!("{action} {}: {error}", path.display())
}

fn cargo() -> std::ffi::OsString {
    std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into())
}
