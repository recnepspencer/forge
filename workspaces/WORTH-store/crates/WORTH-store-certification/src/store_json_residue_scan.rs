use crate::{StoreJsonResidueDenial, StoreJsonResidueOccurrence, StoreJsonResidueTokenKind};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub(crate) fn scan_current_store_json_residue(
) -> Result<Vec<StoreJsonResidueOccurrence>, StoreJsonResidueDenial> {
    let mut occurrences = Vec::new();
    for root in scan_roots()? {
        collect_occurrences(&root, &mut occurrences)?;
    }
    occurrences.sort();
    Ok(occurrences)
}

pub(crate) fn scan_source_text(path: &str, text: &str) -> Vec<StoreJsonResidueOccurrence> {
    text.lines()
        .enumerate()
        .flat_map(|(index, line)| occurrences_for_line(path, index as u32 + 1, line))
        .collect()
}

fn collect_occurrences(
    path: &Path,
    occurrences: &mut Vec<StoreJsonResidueOccurrence>,
) -> Result<(), StoreJsonResidueDenial> {
    let metadata = fs::metadata(path).map_err(scan_error)?;
    if metadata.is_dir() {
        let mut entries = fs::read_dir(path)
            .map_err(scan_error)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(scan_error)?;
        entries.sort_by_key(|entry| entry.path());
        for entry in entries {
            let child = entry.path();
            if is_skipped_dir(&child) {
                continue;
            }
            collect_occurrences(&child, occurrences)?;
        }
        return Ok(());
    }
    if !is_scanned_source_file(path) {
        return Ok(());
    }
    let source = fs::read_to_string(path).map_err(scan_error)?;
    let relative = repository_relative_path(path)?;
    occurrences.extend(scan_source_text(&relative, &source));
    Ok(())
}

fn occurrences_for_line(
    path: &str,
    line_number: u32,
    line: &str,
) -> Vec<StoreJsonResidueOccurrence> {
    let mut tokens = Vec::new();
    if line.contains("serde_json") {
        tokens.push(StoreJsonResidueTokenKind::SerdeJson);
    }
    if line.contains("json!") {
        tokens.push(StoreJsonResidueTokenKind::JsonMacro);
    }
    if contains_word(line, "DeserializeOwned") {
        tokens.push(StoreJsonResidueTokenKind::DeserializeOwned);
    }
    if contains_word(line, "Deserialize") {
        tokens.push(StoreJsonResidueTokenKind::Deserialize);
    }
    if contains_word(line, "Serialize") {
        tokens.push(StoreJsonResidueTokenKind::Serialize);
    }
    if contains_raw_json_helper(line) {
        tokens.push(StoreJsonResidueTokenKind::RawJsonHelper);
    }
    tokens
        .into_iter()
        .map(|token| {
            StoreJsonResidueOccurrence::new(path, line_number, token, line.trim().to_string())
        })
        .collect()
}

fn contains_raw_json_helper(line: &str) -> bool {
    [
        "canonical_json",
        "semantic_json",
        "stable_json_digest",
        "to_canonical_json_bytes",
        "validate_canonical_json_bytes",
        "payload_json",
        "deserialize_json",
        "deserialize_optional_json",
        "serialize_optional_json",
        "persist_bulk_json_record",
        "JsonDocument",
        "json_document",
        "fixture_json",
    ]
    .iter()
    .any(|needle| line.contains(needle))
}

fn contains_word(line: &str, word: &str) -> bool {
    let mut search_start = 0;
    while let Some(offset) = line[search_start..].find(word) {
        let start = search_start + offset;
        let end = start + word.len();
        if is_boundary(line, start, end) {
            return true;
        }
        search_start = end;
    }
    false
}

fn is_boundary(line: &str, start: usize, end: usize) -> bool {
    let before = line[..start].chars().next_back();
    let after = line[end..].chars().next();
    !is_word_char(before) && !is_word_char(after)
}

fn is_word_char(character: Option<char>) -> bool {
    character.is_some_and(|value| value.is_ascii_alphanumeric() || value == '_')
}

fn scan_roots() -> Result<[PathBuf; 2], StoreJsonResidueDenial> {
    let root = repository_root()?;
    Ok([
        root.join("crates/worth-store"),
        root.join("workspaces/worth-store"),
    ])
}

fn repository_relative_path(path: &Path) -> Result<String, StoreJsonResidueDenial> {
    let root = repository_root()?;
    let relative = path.strip_prefix(root).map_err(scan_error)?;
    Ok(relative.to_string_lossy().replace('\\', "/"))
}

fn repository_root() -> Result<PathBuf, StoreJsonResidueDenial> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .map(Path::to_path_buf)
        .ok_or_else(|| StoreJsonResidueDenial::SourceScanFailed("repository root".to_string()))
}

fn is_scanned_source_file(path: &Path) -> bool {
    path.extension().is_some_and(|extension| extension == "rs")
        || path.file_name().is_some_and(|name| name == "Cargo.toml")
}

fn is_skipped_dir(path: &Path) -> bool {
    path.file_name().is_some_and(|name| {
        matches!(
            name.to_string_lossy().as_ref(),
            "target" | ".git" | ".idea" | ".vscode"
        )
    })
}

fn scan_error(error: impl ToString) -> StoreJsonResidueDenial {
    StoreJsonResidueDenial::SourceScanFailed(error.to_string())
}
