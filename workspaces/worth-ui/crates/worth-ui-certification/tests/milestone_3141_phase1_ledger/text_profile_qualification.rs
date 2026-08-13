use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

const ROOT: &str = "workspaces/worth-ui/profiles/worth-ui-global-text-v2";
const PROFILE_DIGEST: &str = "cec6005c5baef6d69ada9c30c02ced25b0f253f80c012784fe925e307935c3f2";

#[path = "text_profile_qualification/dependency_graph.rs"]
mod dependency_graph;
#[path = "text_profile_qualification/emoji.rs"]
mod emoji;
#[path = "text_profile_qualification/generated_indexes.rs"]
mod generated_indexes;
#[path = "text_profile_qualification/license.rs"]
mod license;
#[path = "text_profile_qualification/manifest.rs"]
mod manifest;

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
    manifest::validate(&manifest)?;
    dependency_graph::validate(&root, &manifest)?;
    validate_inventory(&root, &manifest)?;
    license::validate(&root, &manifest)?;
    validate_faces(&root, &manifest)?;
    emoji::validate(&root, &manifest)?;
    generated_indexes::validate(&root)?;
    validate_generated_indexes(&root, &manifest)
}

fn parse_toml(bytes: &[u8]) -> Result<toml::Value, String> {
    std::str::from_utf8(bytes)
        .map_err(|error| error.to_string())?
        .parse::<toml::Value>()
        .map_err(|error| error.to_string())
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
#[ignore = "Phase 4 closure: invokes the pinned external profile-index builder"]
fn global_text_profile_assets_indexes_and_dependencies_are_exact() {
    let digest = digest(&fs::read(manifest_path()).expect("manifest bytes"));
    assert_eq!(digest, PROFILE_DIGEST);
    validate_profile(PROFILE_DIGEST).expect("qualified Unicode text profile");
    println!("WORTH_UI_LEDGER_COUNTERS={{\"P4-TEXT-PROFILE-01\":34}}");
}

#[test]
fn global_text_profile_rejects_manifest_and_artifact_drift() {
    let bytes = fs::read(manifest_path()).expect("manifest bytes");
    assert!(validate_profile(&"0".repeat(64)).is_err());
    let manifest = parse_toml(&bytes).expect("manifest");
    let mut mutated = manifest.clone();
    mutated["artifact_inventory_sha256"] = toml::Value::String("0".repeat(64));
    assert!(validate_inventory(&profile_root(), &mutated).is_err());

    let mut dependency_mutant = manifest.clone();
    dependency_mutant["dependencies"]["icu_segmenter"]["resolved_features"] =
        toml::Value::Array(vec!["auto".into(), "compiled_data".into()]);
    assert!(manifest::validate(&dependency_mutant).is_err());

    let mut color_parser_mutant = manifest.clone();
    color_parser_mutant["dependencies"]["read_fonts"]["features"] =
        toml::Value::Array(vec!["std".into()]);
    assert!(manifest::validate(&color_parser_mutant).is_err());

    let mut saturation_mutant = manifest;
    saturation_mutant["saturation"]["live_eviction"] = toml::Value::Boolean(true);
    assert!(manifest::validate(&saturation_mutant).is_err());
    println!(
        "WORTH_UI_LEDGER_MUTATION_CONTROLS={{\"P4-TEXT-PROFILE-01\":\"font-or-unicode-digest-drift\"}}"
    );
}

#[test]
fn global_text_profile_rejects_interaction_color_capacity_and_locality_drift() {
    let bytes = fs::read(manifest_path()).expect("manifest bytes");
    let manifest = parse_toml(&bytes).expect("manifest");

    let mut caret = manifest.clone();
    caret["layout"]["bidi_boundary_rule"] = toml::Value::String("one-caret".into());
    assert!(manifest::validate(&caret).is_err());

    let mut accessibility = manifest.clone();
    accessibility["layout_identity"]["consumers"]
        .as_array_mut()
        .expect("consumer list")
        .pop();
    assert!(manifest::validate(&accessibility).is_err());

    let mut color = manifest.clone();
    color["application_fonts"]["admitted_color_tables"]
        .as_array_mut()
        .expect("color table list")
        .remove(1);
    assert!(manifest::validate(&color).is_err());

    let mut sbix = manifest.clone();
    sbix["application_fonts"]["sbix_graphic_types"] =
        toml::Value::Array(vec!["jpg".into(), "dupe".into()]);
    assert!(manifest::validate(&sbix).is_err());

    let mut capacity = manifest.clone();
    capacity["capacity_admission"]["staging"] = toml::Value::String("published".into());
    assert!(manifest::validate(&capacity).is_err());

    let mut locality = manifest;
    locality["locality"]["content_edit"] = toml::Value::String("global".into());
    assert!(manifest::validate(&locality).is_err());
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

#[test]
fn global_text_profile_rejects_count_preserving_emoji_set_drift() {
    let root = profile_root();
    let test = fs::read_to_string(root.join("unicode/emoji/emoji-test.txt")).expect("emoji-test");
    let sequences =
        fs::read_to_string(root.join("unicode/emoji/emoji-sequences.txt")).expect("sequences");
    let zwj = fs::read_to_string(root.join("unicode/emoji/emoji-zwj-sequences.txt")).expect("ZWJ");
    let mutated = sequences.replacen("0023 FE0F 20E3", "0024 FE0F 20E3", 1);
    assert_eq!(
        emoji::require_exact_rgi_set(&test, &mutated, &zwj),
        Err("Unicode 17 RGI emoji corpus sets disagree".to_owned())
    );
}

#[test]
fn global_text_profile_rejects_count_preserving_variation_pair_drift() {
    let path = profile_root().join("unicode/ucd/emoji/emoji-variation-sequences.txt");
    let variations = fs::read_to_string(path).expect("variation sequences");
    let mutated = variations.replacen("0023 FE0E", "0024 FE0E", 1);
    assert_eq!(
        emoji::require_complete_variation_pairs(&mutated),
        Err("emoji text/presentation variation pair is incomplete".to_owned())
    );
}
