use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

const ROOT: &str = "workspaces/worth-ui/profiles/worth-ui-global-text-v2";

#[path = "text_profile_qualification/emoji.rs"]
mod emoji;

fn profile_root() -> PathBuf {
    super::source_digest::repository_root().join(ROOT)
}

fn manifest_path() -> PathBuf {
    profile_root().join("manifest.toml")
}

pub(super) fn validate_profile(expected_digest: &str) -> Result<(), String> {
    let root = profile_root();
    let manifest_bytes = fs::read(manifest_path()).map_err(|error| error.to_string())?;
    if digest(&manifest_bytes) != expected_digest {
        return Err("qualified text manifest digest drifted".to_owned());
    }
    let manifest = parse_toml(&manifest_bytes)?;
    validate_manifest_contract(&manifest)?;
    validate_inventory(&root, &manifest)?;
    validate_faces(&root, &manifest)?;
    emoji::validate(&root, &manifest)?;
    validate_generated_indexes(&root, &manifest)
}

fn parse_toml(bytes: &[u8]) -> Result<toml::Value, String> {
    std::str::from_utf8(bytes)
        .map_err(|error| error.to_string())?
        .parse::<toml::Value>()
        .map_err(|error| error.to_string())
}

fn validate_manifest_contract(manifest: &toml::Value) -> Result<(), String> {
    require_string(manifest, "schema", "worth-ui-global-text-profile-v2")?;
    require_string(manifest, "profile", "worth-ui-global-text-v2")?;
    require_string(manifest, "unicode_version", "17.0.0")?;
    require_bool(manifest, "ambient_system_fonts", false)?;
    require_string(
        manifest,
        "fallback",
        "complete-cluster-first-qualified-face",
    )?;
    let dependencies = table(manifest, "dependencies")?;
    require_dependency(dependencies, "unicode_segmentation", "=1.13.3")?;
    require_dependency(dependencies, "unicode_bidi", "=0.3.18")?;
    require_dependency(dependencies, "icu_segmenter", "=2.2.0")?;
    require_dependency(dependencies, "harfrust", "=0.12.0")?;
    require_dependency(dependencies, "swash", "=0.2.10")?;
    validate_capacities(table(manifest, "capacity")?)
}

fn validate_capacities(capacity: &toml::value::Table) -> Result<(), String> {
    let exact = [
        ("retained_paragraphs", 4096),
        ("retained_utf8_bytes", 8_388_608),
        ("paragraph_utf8_bytes", 65_536),
        ("glyphs", 262_144),
        ("grapheme_cluster_records", 262_144),
        ("line_records", 65_536),
        ("runs_per_paragraph", 32),
        ("atlas_entries", 8192),
        ("staged_upload_bytes", 8_388_608),
    ];
    for (field, expected) in exact {
        if capacity.get(field).and_then(toml::Value::as_integer) != Some(expected) {
            return Err(format!("text capacity drifted: {field}"));
        }
    }
    Ok(())
}

fn validate_inventory(root: &Path, manifest: &toml::Value) -> Result<(), String> {
    let relative = string(manifest, "artifact_inventory")?;
    let bytes = fs::read(root.join(relative)).map_err(|error| error.to_string())?;
    require_integer(manifest, "artifact_inventory_bytes", bytes.len() as i64)?;
    require_string(manifest, "artifact_inventory_sha256", &digest(&bytes))?;
    let inventory = parse_toml(&bytes)?;
    let artifacts = array(&inventory, "artifact")?;
    require_integer(
        manifest,
        "artifact_inventory_entries",
        artifacts.len() as i64,
    )?;
    let mut declared = BTreeSet::new();
    for artifact in artifacts {
        let record = artifact.as_table().ok_or("artifact is not a table")?;
        let path = table_string(record, "path")?;
        if !declared.insert(path.to_owned()) {
            return Err(format!("duplicate text artifact: {path}"));
        }
        validate_file(root.join(path), record)?;
    }
    let observed = observed_artifacts(root)?;
    if declared != observed {
        return Err("text artifact inventory is incomplete or contains residue".to_owned());
    }
    Ok(())
}

fn observed_artifacts(root: &Path) -> Result<BTreeSet<String>, String> {
    let mut files = BTreeSet::new();
    for directory in ["unicode", "licenses", "generated"] {
        visit_files(root, &root.join(directory), &mut files)?;
    }
    Ok(files)
}

fn visit_files(root: &Path, directory: &Path, files: &mut BTreeSet<String>) -> Result<(), String> {
    for entry in fs::read_dir(directory).map_err(|error| error.to_string())? {
        let path = entry.map_err(|error| error.to_string())?.path();
        if path.is_dir() {
            visit_files(root, &path, files)?;
        } else {
            let relative = path.strip_prefix(root).map_err(|error| error.to_string())?;
            files.insert(relative.to_string_lossy().replace('\\', "/"));
        }
    }
    Ok(())
}

fn validate_faces(root: &Path, manifest: &toml::Value) -> Result<(), String> {
    let faces = array(manifest, "face")?;
    if faces.len() != 30 {
        return Err("qualified text profile must enumerate exactly 30 faces".to_owned());
    }
    let mut ids = BTreeSet::new();
    for (rank, face) in faces.iter().enumerate() {
        let record = face.as_table().ok_or("face is not a table")?;
        let id = table_string(record, "id")?;
        if !ids.insert(id) || table_integer(record, "fallback_rank")? != rank as i64 {
            return Err("font identities or fallback ranks are not unique and exact".to_owned());
        }
        validate_file(root.join(table_string(record, "path")?), record)?;
        let license = root.join(table_string(record, "license")?);
        if !license.is_file() || !table_string(record, "source")?.starts_with("https://") {
            return Err(format!("font source or license is incomplete: {id}"));
        }
    }
    Ok(())
}

fn validate_generated_indexes(root: &Path, manifest: &toml::Value) -> Result<(), String> {
    let coverage: serde_json::Value = read_json(root.join("generated/font-coverage-v2.json"))?;
    let fallback: serde_json::Value = read_json(root.join("generated/fallback-order-v2.json"))?;
    if coverage["unicode"] != "17.0.0" || coverage["faces"].as_array().map(Vec::len) != Some(30) {
        return Err("generated font coverage index is incomplete".to_owned());
    }
    if fallback["faces"].as_array().map(Vec::len) != Some(30) {
        return Err("generated fallback index is incomplete".to_owned());
    }
    validate_index_correspondence(manifest, &coverage, &fallback)?;
    validate_special_faces(&coverage)
}

fn validate_index_correspondence(
    manifest: &toml::Value,
    coverage: &serde_json::Value,
    fallback: &serde_json::Value,
) -> Result<(), String> {
    let expected: Vec<_> = array(manifest, "face")?
        .iter()
        .map(|face| face["id"].as_str().unwrap_or_default())
        .collect();
    let coverage_ids = json_face_ids(coverage, "id")?;
    let fallback_ids = json_face_ids(fallback, "face")?;
    if expected != coverage_ids || expected != fallback_ids {
        return Err("generated font indexes do not preserve manifest order".to_owned());
    }
    Ok(())
}

fn validate_special_faces(coverage: &serde_json::Value) -> Result<(), String> {
    let faces = coverage["faces"]
        .as_array()
        .ok_or("coverage faces missing")?;
    let by_id: BTreeMap<_, _> = faces
        .iter()
        .filter_map(|face| Some((face["id"].as_str()?, face)))
        .collect();
    let emoji = by_id.get("noto-color-emoji").ok_or("emoji face missing")?;
    let tables = emoji["color_tables"]
        .as_array()
        .ok_or("emoji color tables missing")?;
    if !tables.iter().any(|table| table == "CBDT") || !tables.iter().any(|table| table == "CBLC") {
        return Err("emoji face has no qualified color bitmap tables".to_owned());
    }
    let last = by_id
        .get("last-resort-17")
        .ok_or("Last Resort face missing")?;
    if last["coverage_ranges"] != serde_json::json!([[0, 1_114_111]]) {
        return Err("Last Resort does not cover every Unicode scalar position".to_owned());
    }
    Ok(())
}

fn json_face_ids<'a>(value: &'a serde_json::Value, field: &str) -> Result<Vec<&'a str>, String> {
    value["faces"]
        .as_array()
        .ok_or_else(|| "generated face list missing".to_owned())?
        .iter()
        .map(|face| {
            face[field]
                .as_str()
                .ok_or_else(|| "generated face id missing".to_owned())
        })
        .collect()
}

fn validate_file(path: PathBuf, record: &toml::value::Table) -> Result<(), String> {
    let bytes = fs::read(&path).map_err(|error| error.to_string())?;
    if table_integer(record, "bytes").or_else(|_| table_integer(record, "byte_length"))?
        != bytes.len() as i64
        || table_string(record, "sha256")? != digest(&bytes)
    {
        return Err(format!("qualified artifact drifted: {}", path.display()));
    }
    Ok(())
}

fn read_json(path: PathBuf) -> Result<serde_json::Value, String> {
    serde_json::from_slice(&fs::read(path).map_err(|error| error.to_string())?)
        .map_err(|error| error.to_string())
}

fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn table<'a>(value: &'a toml::Value, key: &str) -> Result<&'a toml::value::Table, String> {
    value
        .get(key)
        .and_then(toml::Value::as_table)
        .ok_or_else(|| format!("missing table: {key}"))
}

fn array<'a>(value: &'a toml::Value, key: &str) -> Result<&'a Vec<toml::Value>, String> {
    value
        .get(key)
        .and_then(toml::Value::as_array)
        .ok_or_else(|| format!("missing array: {key}"))
}

fn string<'a>(value: &'a toml::Value, key: &str) -> Result<&'a str, String> {
    value
        .get(key)
        .and_then(toml::Value::as_str)
        .ok_or_else(|| format!("missing string: {key}"))
}

fn require_string(value: &toml::Value, key: &str, expected: &str) -> Result<(), String> {
    if string(value, key)? == expected {
        Ok(())
    } else {
        Err(format!("field drifted: {key}"))
    }
}

fn require_integer(value: &toml::Value, key: &str, expected: i64) -> Result<(), String> {
    if value.get(key).and_then(toml::Value::as_integer) == Some(expected) {
        Ok(())
    } else {
        Err(format!("field drifted: {key}"))
    }
}

fn require_bool(value: &toml::Value, key: &str, expected: bool) -> Result<(), String> {
    if value.get(key).and_then(toml::Value::as_bool) == Some(expected) {
        Ok(())
    } else {
        Err(format!("field drifted: {key}"))
    }
}

fn require_dependency(table: &toml::value::Table, key: &str, version: &str) -> Result<(), String> {
    let dependency = table
        .get(key)
        .and_then(toml::Value::as_table)
        .ok_or("dependency missing")?;
    if table_string(dependency, "version")? == version {
        Ok(())
    } else {
        Err(format!("dependency drifted: {key}"))
    }
}

fn table_string<'a>(table: &'a toml::value::Table, key: &str) -> Result<&'a str, String> {
    table
        .get(key)
        .and_then(toml::Value::as_str)
        .ok_or_else(|| format!("missing string: {key}"))
}

fn table_integer(table: &toml::value::Table, key: &str) -> Result<i64, String> {
    table
        .get(key)
        .and_then(toml::Value::as_integer)
        .ok_or_else(|| format!("missing integer: {key}"))
}

#[test]
fn global_text_profile_assets_indexes_and_dependencies_are_exact() {
    let digest = digest(&fs::read(manifest_path()).expect("manifest bytes"));
    validate_profile(&digest).expect("qualified Unicode text profile");
}

#[test]
fn global_text_profile_rejects_manifest_and_artifact_drift() {
    let bytes = fs::read(manifest_path()).expect("manifest bytes");
    assert!(validate_profile(&"0".repeat(64)).is_err());
    let manifest = parse_toml(&bytes).expect("manifest");
    let mut mutated = manifest.clone();
    mutated["artifact_inventory_sha256"] = toml::Value::String("0".repeat(64));
    assert!(validate_inventory(&profile_root(), &mutated).is_err());
}

#[test]
fn global_text_profile_rejects_emoji_sequence_class_or_count_drift() {
    let bytes = fs::read(manifest_path()).expect("manifest bytes");
    let mut manifest = parse_toml(&bytes).expect("manifest");
    manifest["emoji"]["flag_sequence_records"] = toml::Value::Integer(258);
    assert_eq!(
        emoji::validate(&profile_root(), &manifest),
        Err("emoji corpus count drifted: flag_sequence_records".to_owned())
    );
}
