//! Resolve exact configured Cargo build worlds and their compiler cfg authority.

use super::production_attribute_projection::projected_attributes_in_world;
use super::production_availability::attributes_are_available_in_world;
use super::{bounded_process, cargo_workspace_profile::WorkspaceProfiles};
use crate::config::PublicValueWorld;
use std::collections::{BTreeMap, BTreeSet};
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::process::Command;

pub(super) struct ProductionWorld {
    pub(super) name: String,
    pub(super) target: String,
    pub(super) is_host: bool,
    pub(super) profile: String,
    pub(super) workspace_profile_source: String,
    pub(super) default_features: bool,
    pub(super) features: Vec<String>,
    enabled_features: BTreeSet<String>,
    cfg_atoms: BTreeMap<String, bool>,
}

impl ProductionWorld {
    pub(super) fn load(
        crate_root: &Path,
        configured: &[PublicValueWorld],
        limits: bounded_process::Limits,
    ) -> Result<Vec<Self>, String> {
        if configured.is_empty() {
            return Err("public-value reachability requires at least one Cargo world".to_owned());
        }
        let host = rustc_host(limits)?;
        let workspace_profiles = WorkspaceProfiles::load(crate_root, limits)?;
        let mut worlds = Vec::new();
        let mut names = BTreeSet::new();
        let mut resolved = BTreeSet::new();
        for configured in configured {
            validate_name_and_features(configured, &mut names)?;
            workspace_profiles.validate_profile(&configured.profile)?;
            let mut features = configured.features.clone();
            features.sort();
            let target = if configured.target == "host" {
                host.clone()
            } else {
                configured.target.clone()
            };
            let tuple = format!(
                "{}|{}|{}|{}",
                target,
                configured.profile,
                configured.default_features,
                features.join(",")
            );
            if !resolved.insert(tuple) {
                return Err(format!(
                    "duplicate public-value Cargo world `{}`",
                    configured.name
                ));
            }
            let cfg_atoms = cargo_cfg(crate_root, configured, &target, limits)?;
            worlds.push(Self {
                name: configured.name.clone(),
                is_host: target == host,
                profile: configured.profile.clone(),
                workspace_profile_source: workspace_profiles.manifest_source().to_owned(),
                default_features: configured.default_features,
                features,
                enabled_features: enabled_features(&cfg_atoms),
                cfg_atoms,
                target,
            });
        }
        Ok(worlds)
    }

    pub(super) fn includes(&self, attributes: &[syn::Attribute]) -> bool {
        attributes_are_available_in_world(attributes, &self.enabled_features, &self.cfg_atoms)
    }

    pub(super) fn project_attributes(&self, attributes: &[syn::Attribute]) -> Vec<syn::Attribute> {
        projected_attributes_in_world(attributes, &self.enabled_features, &self.cfg_atoms)
    }

    pub(super) fn artifact_directory(&self) -> &str {
        if self.profile == "dev" {
            "debug"
        } else {
            &self.profile
        }
    }
}

fn validate_name_and_features(
    configured: &PublicValueWorld,
    names: &mut BTreeSet<String>,
) -> Result<(), String> {
    if configured.name.trim().is_empty() || !names.insert(configured.name.clone()) {
        return Err(format!(
            "duplicate or empty public-value world `{}`",
            configured.name
        ));
    }
    let mut features = configured.features.clone();
    features.sort();
    if features.iter().any(|feature| feature.trim().is_empty())
        || features.windows(2).any(|pair| pair[0] == pair[1])
    {
        return Err(format!(
            "public-value world `{}` has empty or duplicate features",
            configured.name
        ));
    }
    Ok(())
}

fn rustc_host(limits: bounded_process::Limits) -> Result<String, String> {
    let mut command = Command::new(rustc());
    command.arg("-vV");
    let output = bounded_process::run(&mut command, None, limits, "rustc host discovery")?;
    if !output.status.success() {
        return Err(format!(
            "rustc -vV failed for public-value target authority: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .find_map(|line| line.strip_prefix("host: "))
        .map(str::to_owned)
        .ok_or_else(|| "rustc -vV did not report a host target".to_owned())
}

fn cargo_cfg(
    crate_root: &Path,
    configured: &PublicValueWorld,
    target: &str,
    limits: bounded_process::Limits,
) -> Result<BTreeMap<String, bool>, String> {
    let mut command = cargo_cfg_command(crate_root, configured, target);
    let short_target = short_target_directory(crate_root);
    if let Some(directory) = &short_target {
        command.env("CARGO_TARGET_DIR", directory);
    }
    let output = bounded_process::run(
        &mut command,
        None,
        limits,
        &format!("Cargo cfg discovery for world `{}`", configured.name),
    )?;
    if let Some(directory) = short_target {
        let _ = std::fs::remove_dir_all(directory);
    }
    if !output.status.success() {
        return Err(format!(
            "Cargo rejected public-value world `{}`: {}",
            configured.name,
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    parse_cfg_output(configured, &output.stdout)
}

fn cargo_cfg_command(crate_root: &Path, configured: &PublicValueWorld, target: &str) -> Command {
    let mut command = Command::new(cargo());
    command
        .current_dir(crate_root)
        .args(["rustc", "--quiet", "--manifest-path"])
        .arg(crate_root.join("Cargo.toml"))
        .args(["--lib", "--profile", &configured.profile]);
    if configured.target != "host" {
        command.args(["--target", target]);
    }
    if !configured.default_features {
        command.arg("--no-default-features");
    }
    if !configured.features.is_empty() {
        command.arg("--features").arg(configured.features.join(","));
    }
    command.args(["--", "--print", "cfg"]);
    command
}

fn parse_cfg_output(
    configured: &PublicValueWorld,
    stdout: &[u8],
) -> Result<BTreeMap<String, bool>, String> {
    let mut atoms = BTreeMap::new();
    for line in String::from_utf8_lossy(stdout).lines() {
        let meta: syn::Meta = syn::parse_str(line).map_err(|error| {
            format!(
                "parse Cargo cfg `{line}` for world `{}`: {error}",
                configured.name
            )
        })?;
        atoms.insert(super::production_availability::cfg_atom_key(&meta), true);
    }
    Ok(atoms)
}

fn short_target_directory(crate_root: &Path) -> Option<std::path::PathBuf> {
    if crate_root.as_os_str().to_string_lossy().len() <= 90 {
        return None;
    }
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    crate_root.hash(&mut hasher);
    Some(std::env::temp_dir().join(format!(
        "bc-cfg-{}-{:x}",
        std::process::id(),
        hasher.finish()
    )))
}

fn enabled_features(cfg_atoms: &BTreeMap<String, bool>) -> BTreeSet<String> {
    cfg_atoms
        .keys()
        .filter_map(|atom| atom.strip_prefix("feature = \"")?.strip_suffix('"'))
        .map(str::to_owned)
        .collect()
}

fn rustc() -> std::ffi::OsString {
    std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into())
}

fn cargo() -> std::ffi::OsString {
    std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into())
}
