use std::collections::BTreeSet;
use std::path::Path;

use crate::docs_closeout::error::{WorthDocsCloseoutError, WorthDocsCloseoutErrorKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WorthDocKind {
    CrateReadme,
    Foundation,
    Feature,
    Boundary,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthDocMetadata {
    pub crate_name: String,
    pub kind: WorthDocKind,
    pub doc_id: String,
    pub doc_style: Option<String>,
    pub neighbor_crates: BTreeSet<String>,
    pub categories: BTreeSet<String>,
    pub query_integration_required: bool,
    pub query_proof_required: bool,
    pub touches_query: bool,
}

impl WorthDocMetadata {
    pub fn parse(path: &Path, markdown: &str) -> Result<Self, WorthDocsCloseoutError> {
        let metadata_block = extract_metadata_block(path, markdown)?;
        let mut crate_name = None;
        let mut kind = None;
        let mut doc_id = None;
        let mut doc_style = None;
        let mut neighbor_crates = BTreeSet::new();
        let mut categories = BTreeSet::new();
        let mut query_integration_required = false;
        let mut query_proof_required = false;
        let mut touches_query = false;
        let mut seen_keys = BTreeSet::new();

        for raw_line in metadata_block.lines() {
            let line = raw_line.trim();
            if line.is_empty() {
                continue;
            }
            let (key, value) = line.split_once(':').ok_or_else(|| {
                WorthDocsCloseoutError::new(
                    WorthDocsCloseoutErrorKind::InvalidMetadata,
                    Some(path.to_path_buf()),
                    format!("invalid metadata line `{line}`"),
                )
            })?;
            let key = key.trim();
            if !seen_keys.insert(key) {
                return Err(WorthDocsCloseoutError::new(
                    WorthDocsCloseoutErrorKind::InvalidMetadata,
                    Some(path.to_path_buf()),
                    format!("duplicate metadata key `{key}`"),
                ));
            }
            let value = value.trim();
            match key {
                "crate" => crate_name = Some(value.to_string()),
                "kind" => kind = Some(parse_kind(path, value)?),
                "id" => doc_id = Some(value.to_string()),
                "doc_style" => doc_style = Some(value.to_string()),
                "neighbor_crates" => neighbor_crates = parse_csv_set(value),
                "categories" => categories = parse_csv_set(value),
                "query_integration_required" => {
                    query_integration_required = parse_bool(path, key, value)?
                }
                "query_proof_required" => query_proof_required = parse_bool(path, key, value)?,
                "touches_query" => touches_query = parse_bool(path, key, value)?,
                unknown => {
                    return Err(WorthDocsCloseoutError::new(
                        WorthDocsCloseoutErrorKind::InvalidMetadata,
                        Some(path.to_path_buf()),
                        format!("unknown metadata key `{unknown}`"),
                    ));
                }
            }
        }

        Ok(Self {
            crate_name: required_value(path, "crate", crate_name)?,
            kind: required_value(path, "kind", kind)?,
            doc_id: required_value(path, "id", doc_id)?,
            doc_style,
            neighbor_crates,
            categories,
            query_integration_required,
            query_proof_required,
            touches_query,
        })
    }
}

fn extract_metadata_block(path: &Path, markdown: &str) -> Result<String, WorthDocsCloseoutError> {
    let start = markdown.find("<!-- worth-doc").ok_or_else(|| {
        WorthDocsCloseoutError::new(
            WorthDocsCloseoutErrorKind::MissingMetadata,
            Some(path.to_path_buf()),
            "missing `<!-- worth-doc` block",
        )
    })?;
    let rest = &markdown[start + "<!-- worth-doc".len()..];
    let end = rest.find("-->").ok_or_else(|| {
        WorthDocsCloseoutError::new(
            WorthDocsCloseoutErrorKind::InvalidMetadata,
            Some(path.to_path_buf()),
            "unterminated metadata block",
        )
    })?;
    Ok(rest[..end].trim().to_string())
}

fn parse_kind(path: &Path, value: &str) -> Result<WorthDocKind, WorthDocsCloseoutError> {
    match value {
        "crate_readme" => Ok(WorthDocKind::CrateReadme),
        "foundation" => Ok(WorthDocKind::Foundation),
        "feature" => Ok(WorthDocKind::Feature),
        "boundary" => Ok(WorthDocKind::Boundary),
        _ => Err(WorthDocsCloseoutError::new(
            WorthDocsCloseoutErrorKind::InvalidMetadata,
            Some(path.to_path_buf()),
            format!("unknown doc kind `{value}`"),
        )),
    }
}

fn parse_bool(path: &Path, key: &str, value: &str) -> Result<bool, WorthDocsCloseoutError> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(WorthDocsCloseoutError::new(
            WorthDocsCloseoutErrorKind::InvalidMetadata,
            Some(path.to_path_buf()),
            format!("`{key}` must be `true` or `false`, found `{value}`"),
        )),
    }
}

fn parse_csv_set(value: &str) -> BTreeSet<String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|entry| !entry.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn required_value<T>(
    path: &Path,
    key: &str,
    value: Option<T>,
) -> Result<T, WorthDocsCloseoutError> {
    value.ok_or_else(|| {
        WorthDocsCloseoutError::new(
            WorthDocsCloseoutErrorKind::MissingMetadata,
            Some(path.to_path_buf()),
            format!("missing required metadata key `{key}`"),
        )
    })
}
