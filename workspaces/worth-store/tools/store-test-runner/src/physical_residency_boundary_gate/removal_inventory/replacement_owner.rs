use std::{collections::BTreeMap, path::Path};

use super::{RemovalRow, RemovalStatus};
use crate::workspace_root;

pub(super) fn assert_completed_rows_have_present_replacement(
    phase: &str,
    ledger: &BTreeMap<String, RemovalRow>,
) -> Result<(), String> {
    let completed = ledger
        .values()
        .filter(|row| {
            row.deletion_phase == phase
                && matches!(row.status, RemovalStatus::Deleted(ref deleted_phase) if deleted_phase == phase)
        })
        .collect::<Vec<_>>();
    if completed.is_empty() {
        return Err(format!("{phase} cleanup has no completed removal rows"));
    }
    for row in completed {
        let replacement = replacement_owner_path(row)
            .ok_or_else(|| format!("{} has no path-bound replacement owner", row.path))?;
        if !replacement.exists() {
            return Err(format!(
                "{} replacement owner does not exist: {}",
                row.path,
                replacement.display()
            ));
        }
    }
    Ok(())
}

fn replacement_owner_path(row: &RemovalRow) -> Option<std::path::PathBuf> {
    if let Some(relative) = row.replacement_owner.strip_prefix("workspace:") {
        if invalid_relative_path(relative) {
            return None;
        }
        return Some(workspace_root().join(relative.trim_end_matches('/')));
    }
    let (crate_root, _) = row.path.split_once("/src/")?;
    if invalid_relative_path(&row.replacement_owner) {
        return None;
    }
    Some(
        workspace_root()
            .join(crate_root)
            .join("src")
            .join(row.replacement_owner.trim_end_matches('/')),
    )
}

fn invalid_relative_path(path: &str) -> bool {
    path.is_empty()
        || Path::new(path).is_absolute()
        || path.split('/').any(|component| component == "..")
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use super::{assert_completed_rows_have_present_replacement, invalid_relative_path};
    use crate::physical_residency_boundary_gate::removal_inventory::{RemovalRow, RemovalStatus};

    #[test]
    fn replacement_owner_rejects_empty_absolute_and_escaping_paths() {
        assert!(invalid_relative_path(""));
        assert!(invalid_relative_path("../foreign.rs"));
        assert!(invalid_relative_path("owner/../foreign.rs"));
        assert!(invalid_relative_path("C:/foreign.rs"));
        assert!(!invalid_relative_path("physical_residency/owner.rs"));
    }

    #[test]
    fn completed_replacement_rejects_prose_empty_escaping_and_absent_owners() {
        for replacement in [
            "",
            "../foreign.rs",
            "prose-only-owner",
            "workspace:missing-replacement.rs",
        ] {
            let ledger = BTreeMap::from([("deleted.rs".to_owned(), row(replacement))]);
            assert!(
                assert_completed_rows_have_present_replacement("phase-5", &ledger).is_err(),
                "{replacement}"
            );
        }
    }

    #[test]
    fn completed_replacement_accepts_a_present_path_bound_owner() {
        let ledger = BTreeMap::from([("deleted.rs".to_owned(), row("workspace:Cargo.toml"))]);
        assert!(assert_completed_rows_have_present_replacement("phase-5", &ledger).is_ok());
    }

    fn row(replacement_owner: &str) -> RemovalRow {
        RemovalRow {
            path: "crates/worth-store/src/deleted.rs".to_owned(),
            families: BTreeSet::from(["c6-identifier".to_owned()]),
            deletion_phase: "phase-5".to_owned(),
            replacement_owner: replacement_owner.to_owned(),
            absence_gate: "source-and-metadata-absence".to_owned(),
            status: RemovalStatus::Deleted("phase-5".to_owned()),
        }
    }
}
