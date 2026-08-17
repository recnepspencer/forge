use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::process::Command;

const DEPENDENCIES: [(&str, &str); 10] = [
    ("harfrust", "harfrust"),
    ("icu_segmenter", "icu_segmenter"),
    ("read_fonts", "read-fonts"),
    ("skrifa", "skrifa"),
    ("kurbo", "kurbo"),
    ("linesweeper", "linesweeper"),
    ("png", "png"),
    ("swash", "swash"),
    ("unicode_bidi", "unicode-bidi"),
    ("unicode_segmentation", "unicode-segmentation"),
];

pub(super) fn validate(profile_root: &Path, profile: &toml::Value) -> Result<(), String> {
    let repository = profile_root
        .ancestors()
        .nth(4)
        .ok_or("text profile is outside its repository topology")?;
    let relative = super::string(profile, "dependency_qualification_manifest")?;
    let manifest_path = repository.join(relative);
    let lock_path = manifest_path.with_file_name("Cargo.lock");
    let lock_bytes = fs::read(&lock_path).map_err(|error| error.to_string())?;
    let expected_lock = super::string(profile, "dependency_qualification_lock_sha256")?;
    if format!("{:x}", Sha256::digest(&lock_bytes)) != expected_lock {
        return Err("text dependency qualification lock drifted".to_owned());
    }
    let qualification = parse_toml(&fs::read(&manifest_path).map_err(|e| e.to_string())?)?;
    let lock = parse_toml(&lock_bytes)?;
    validate_direct_declarations(profile, &qualification)?;
    validate_locked_packages(profile, &lock)?;
    validate_resolved_features(profile, &manifest_path)
}

fn validate_direct_declarations(
    profile: &toml::Value,
    qualification: &toml::Value,
) -> Result<(), String> {
    let expected = super::table(profile, "dependencies")?;
    let actual = super::table(qualification, "dependencies")?;
    if actual.len() != DEPENDENCIES.len() {
        return Err("qualification dependency inventory drifted".to_owned());
    }
    for (profile_name, cargo_name) in DEPENDENCIES {
        let expected = dependency(expected, profile_name)?;
        let actual = dependency(actual, cargo_name)?;
        equal_string(expected, actual, "version", profile_name)?;
        let default = actual
            .get("default-features")
            .and_then(toml::Value::as_bool);
        if default
            != expected
                .get("default_features")
                .and_then(toml::Value::as_bool)
        {
            return Err(format!(
                "dependency default features drifted: {profile_name}"
            ));
        }
        if string_array(actual, "features")? != string_array(expected, "features")? {
            return Err(format!(
                "dependency requested features drifted: {profile_name}"
            ));
        }
    }
    Ok(())
}

fn validate_locked_packages(profile: &toml::Value, lock: &toml::Value) -> Result<(), String> {
    let expected = super::table(profile, "dependencies")?;
    let packages = super::array(lock, "package")?;
    for (profile_name, cargo_name) in DEPENDENCIES {
        let contract = dependency(expected, profile_name)?;
        let version = super::table_string(contract, "version")?.trim_start_matches('=');
        let matches: Vec<_> = packages
            .iter()
            .filter_map(toml::Value::as_table)
            .filter(|package| {
                package.get("name").and_then(toml::Value::as_str) == Some(cargo_name)
                    && package.get("version").and_then(toml::Value::as_str) == Some(version)
            })
            .collect();
        if matches.len() != 1
            || matches[0].get("checksum").and_then(toml::Value::as_str)
                != contract.get("checksum").and_then(toml::Value::as_str)
        {
            return Err(format!("dependency lock record drifted: {profile_name}"));
        }
    }
    Ok(())
}

fn validate_resolved_features(profile: &toml::Value, path: &Path) -> Result<(), String> {
    let output = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
        .args(["metadata", "--quiet", "--locked", "--format-version", "1"])
        .arg("--manifest-path")
        .arg(path)
        .output()
        .map_err(|error| format!("could not run dependency resolver: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "dependency resolver failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let metadata: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    let expected = super::table(profile, "dependencies")?;
    let packages = metadata["packages"]
        .as_array()
        .ok_or("metadata packages missing")?;
    let nodes = metadata["resolve"]["nodes"]
        .as_array()
        .ok_or("metadata resolve graph missing")?;
    for (profile_name, cargo_name) in DEPENDENCIES {
        let contract = dependency(expected, profile_name)?;
        let version = super::table_string(contract, "version")?.trim_start_matches('=');
        let package = packages
            .iter()
            .find(|package| package["name"] == cargo_name && package["version"] == version)
            .ok_or_else(|| format!("resolved dependency missing: {profile_name}"))?;
        let node = nodes
            .iter()
            .find(|node| node["id"] == package["id"])
            .ok_or_else(|| format!("resolved dependency node missing: {profile_name}"))?;
        let actual = json_strings(&node["features"])?;
        let expected = string_array(contract, "resolved_features")?;
        if actual != expected {
            return Err(format!(
                "resolved dependency features drifted: {profile_name}"
            ));
        }
    }
    Ok(())
}

fn parse_toml(bytes: &[u8]) -> Result<toml::Value, String> {
    std::str::from_utf8(bytes)
        .map_err(|error| error.to_string())?
        .parse()
        .map_err(|error: toml::de::Error| error.to_string())
}

fn dependency<'a>(
    table: &'a toml::value::Table,
    name: &str,
) -> Result<&'a toml::value::Table, String> {
    table
        .get(name)
        .and_then(toml::Value::as_table)
        .ok_or_else(|| format!("dependency declaration missing: {name}"))
}

fn equal_string(
    expected: &toml::value::Table,
    actual: &toml::value::Table,
    key: &str,
    name: &str,
) -> Result<(), String> {
    if expected.get(key).and_then(toml::Value::as_str)
        != actual.get(key).and_then(toml::Value::as_str)
    {
        return Err(format!("dependency {key} drifted: {name}"));
    }
    Ok(())
}

fn string_array(table: &toml::value::Table, key: &str) -> Result<BTreeSet<String>, String> {
    match table.get(key) {
        None => Ok(BTreeSet::new()),
        Some(value) => value
            .as_array()
            .ok_or_else(|| format!("dependency {key} is not an array"))?
            .iter()
            .map(|item| {
                item.as_str()
                    .map(str::to_owned)
                    .ok_or_else(|| format!("dependency {key} is not text"))
            })
            .collect(),
    }
}

fn json_strings(value: &Value) -> Result<BTreeSet<String>, String> {
    value
        .as_array()
        .ok_or_else(|| "resolved features are not an array".to_owned())?
        .iter()
        .map(|item| {
            item.as_str()
                .map(str::to_owned)
                .ok_or_else(|| "resolved feature is not text".to_owned())
        })
        .collect()
}
