use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::topology::WorkspaceSourceInventory;

use super::ledger;

const REQUIRED_FIELDS: &[&str] = &[
    "current_owner",
    "responsibility",
    "authority_in",
    "authority_out",
    "lifecycle",
    "failure_owner",
    "cost",
    "destination",
    "disposition",
    "forbidden_shortcut",
];

pub(super) fn audit(
    inventory: &WorkspaceSourceInventory,
    document: &toml::Value,
) -> Result<(), String> {
    audit_paths(inventory, document, None)
}

#[cfg(test)]
pub(super) fn audit_with_extra_path(
    inventory: &WorkspaceSourceInventory,
    document: &toml::Value,
    extra_path: &str,
) -> Result<(), String> {
    audit_paths(inventory, document, Some(extra_path))
}

fn audit_paths(
    inventory: &WorkspaceSourceInventory,
    document: &toml::Value,
    extra_path: Option<&str>,
) -> Result<(), String> {
    validate_header(document)?;
    let scopes = ledger::tables(document, "scope")?;
    let rows = ledger::tables(document, "classification")?;
    validate_rows(rows)?;
    let mut paths_by_scope = capture_scope_paths(inventory, scopes)?;
    if let Some(path) = extra_path {
        paths_by_scope
            .get_mut("runtime-source")
            .ok_or_else(|| "runtime-source scope should exist".to_owned())?
            .insert(path.to_owned());
    }

    for scope in scopes {
        audit_scope(inventory, scope, rows, &paths_by_scope)?;
    }
    Ok(())
}

fn validate_header(document: &toml::Value) -> Result<(), String> {
    if ledger::text(document, "schema")? != "worth-ui.milestone-3.10.1.source-semantics.v1" {
        return Err("source-semantics ledger schema should be v1".to_owned());
    }
    let progression = ledger::strings(document, "canonical_progression")?;
    if progression.len() < 10
        || progression.first() != Some(&"authored source")
        || progression.last() != Some(&"inspection projection")
    {
        return Err(
            "canonical progression should cover authored source through inspection".to_owned(),
        );
    }
    if ledger::strings(document, "forbidden_parallel_progressions")?.len() < 4 {
        return Err("parallel progression denials should be explicit".to_owned());
    }
    Ok(())
}

fn validate_rows(rows: &[toml::Value]) -> Result<(), String> {
    let mut ids = BTreeSet::new();
    for row in rows {
        let id = ledger::text(row, "id")?;
        if !ids.insert(id) {
            return Err(format!("duplicate source classification `{id}`"));
        }
        for field in REQUIRED_FIELDS {
            if row.get(*field).is_none() {
                return Err(format!("classification `{id}` is missing `{field}`"));
            }
        }
        let disposition = ledger::text(row, "disposition")?;
        if ["move", "split", "remove"].contains(&disposition) {
            let phase = ledger::integer(row, "exit_phase")?;
            if !(2..=6).contains(&phase) {
                return Err(format!(
                    "classification `{id}` has invalid exit phase {phase}"
                ));
            }
        }
        validate_capability_ownership(row)?;
    }
    Ok(())
}

pub(super) fn validate_capability_ownership(row: &toml::Value) -> Result<(), String> {
    let capabilities = ledger::strings(row, "capabilities")?;
    if capabilities.contains(&"filesystem-transport")
        && capabilities
            .iter()
            .any(|value| ["parser", "authored-legality"].contains(value))
    {
        return Err("filesystem transport cannot own parser or authored legality".to_owned());
    }
    let language_owned = capabilities.iter().any(|capability| {
        [
            "tokenization",
            "parser",
            "syntax-tree",
            "authored-normalization",
            "authored-legality",
            "semantic-assembly",
            "semantic-equivalence",
        ]
        .contains(capability)
    });
    let runtime_owned = ledger::text(row, "current_owner")? == "worth-ui-runtime";
    if language_owned && runtime_owned {
        let disposition = ledger::text(row, "disposition")?;
        let exit_phase = ledger::integer(row, "exit_phase")?;
        if !["move", "split"].contains(&disposition) || exit_phase > 3 {
            return Err(format!(
                "language capability `{}` should exit runtime by Phase 3",
                ledger::text(row, "id")?
            ));
        }
    }
    Ok(())
}

fn capture_scope_paths(
    inventory: &WorkspaceSourceInventory,
    scopes: &[toml::Value],
) -> Result<BTreeMap<String, BTreeSet<String>>, String> {
    let mut result = BTreeMap::new();
    for scope in scopes {
        let id = ledger::text(scope, "id")?;
        let root = ledger::text(scope, "root")?;
        let excluded = scope
            .get("exclude")
            .map(|_| ledger::strings(scope, "exclude"))
            .transpose()?
            .unwrap_or_default();
        let paths = inventory
            .rust_files_under(root)
            .filter_map(|source| {
                let relative = source.relative_path().strip_prefix(root).ok()?;
                let normalized = normalize(relative);
                (!excluded
                    .iter()
                    .any(|component| normalized.split('/').any(|part| part == *component)))
                .then_some(normalized)
            })
            .collect();
        if result.insert(id.to_owned(), paths).is_some() {
            return Err(format!("duplicate source scope `{id}`"));
        }
    }
    Ok(result)
}

fn audit_scope(
    inventory: &WorkspaceSourceInventory,
    scope: &toml::Value,
    rows: &[toml::Value],
    paths_by_scope: &BTreeMap<String, BTreeSet<String>>,
) -> Result<(), String> {
    let id = ledger::text(scope, "id")?;
    let paths = paths_by_scope
        .get(id)
        .ok_or_else(|| format!("scope `{id}` was not captured"))?;
    let expected = ledger::integer(scope, "expected_files")? as usize;
    if paths.len() != expected {
        return Err(format!(
            "scope `{id}` has {} files; expected {expected}",
            paths.len()
        ));
    }
    let prefixed = paths
        .iter()
        .map(|path| {
            format!(
                "workspaces/worth-ui/{}/{}",
                ledger::text(scope, "root").expect("validated scope root"),
                path
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let observed = ledger::fingerprint(prefixed);
    let expected_fingerprint = ledger::text(scope, "path_set_fingerprint")?;
    if observed != expected_fingerprint {
        return Err(format!(
            "scope `{id}` file-set fingerprint changed: {observed} != {expected_fingerprint}"
        ));
    }
    let observed_content = content_fingerprint(inventory, scope, paths)?;
    let expected_content = ledger::text(scope, "content_fingerprint")?;
    if observed_content != expected_content {
        return Err(format!(
            "scope `{id}` content fingerprint changed: {observed_content} != {expected_content}"
        ));
    }
    for path in paths {
        let matches = rows
            .iter()
            .filter(|row| ledger::text(row, "scope").ok() == Some(id))
            .filter(|row| row_matches(row, path))
            .count();
        if matches != 1 {
            return Err(format!(
                "`{id}/{path}` should match exactly one classification; matched {matches}"
            ));
        }
    }
    Ok(())
}

fn content_fingerprint(
    inventory: &WorkspaceSourceInventory,
    scope: &toml::Value,
    paths: &BTreeSet<String>,
) -> Result<String, String> {
    let root = ledger::text(scope, "root")?;
    let mut bytes = Vec::new();
    for path in paths {
        bytes.extend_from_slice(path.as_bytes());
        bytes.push(0);
        bytes.extend_from_slice(inventory.text(Path::new(root).join(path)).as_bytes());
        bytes.push(0);
    }
    Ok(ledger::fingerprint(bytes))
}

fn row_matches(row: &toml::Value, path: &str) -> bool {
    let kind = ledger::text(row, "match").unwrap_or("");
    let single = ledger::optional_text(row, "path");
    let many = row
        .get("paths")
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(toml::Value::as_str);
    let exact_many = row
        .get("exact_paths")
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(toml::Value::as_str);
    let directory_many = row
        .get("directory_paths")
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(toml::Value::as_str);
    match kind {
        "exact" => single == Some(path),
        "directory" => single.is_some_and(|prefix| in_directory(path, prefix)),
        "exact-set" => many.into_iter().any(|candidate| candidate == path),
        "directory-set" => many.into_iter().any(|prefix| in_directory(path, prefix)),
        "mixed-set" => {
            exact_many.into_iter().any(|candidate| candidate == path)
                || directory_many
                    .into_iter()
                    .any(|prefix| in_directory(path, prefix))
        }
        _ => false,
    }
}

fn in_directory(path: &str, directory: &str) -> bool {
    path == format!("{directory}/mod.rs") || path.starts_with(&format!("{directory}/"))
}

fn normalize(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
