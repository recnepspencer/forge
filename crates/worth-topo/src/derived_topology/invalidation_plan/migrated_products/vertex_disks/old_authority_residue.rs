use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct VertexDiskOldAuthorityResidueRow {
    caller: String,
    owner: String,
    blocker: String,
    removal_trigger: String,
    row_digest: String,
}

impl VertexDiskOldAuthorityResidueRow {
    fn new(
        caller: impl Into<String>,
        owner: impl Into<String>,
        blocker: impl Into<String>,
        removal_trigger: impl Into<String>,
    ) -> Self {
        let caller = caller.into();
        let owner = owner.into();
        let blocker = blocker.into();
        let removal_trigger = removal_trigger.into();
        let row_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:vertex-disk-old-authority-residue-row:v1".to_string(),
            format!("caller:{caller}"),
            format!("owner:{owner}"),
            format!("blocker:{blocker}"),
            format!("removal-trigger:{removal_trigger}"),
        ]);
        Self {
            caller,
            owner,
            blocker,
            removal_trigger,
            row_digest,
        }
    }

    pub fn caller(&self) -> &str {
        &self.caller
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn blocker(&self) -> &str {
        &self.blocker
    }

    pub fn removal_trigger(&self) -> &str {
        &self.removal_trigger
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct VertexDiskOldAuthorityResidue {
    capped_direct_interpreter_count: usize,
    capped_rows: Vec<VertexDiskOldAuthorityResidueRow>,
    residue_digest: String,
}

impl VertexDiskOldAuthorityResidue {
    pub fn current_source_scan() -> Self {
        Self::new(current_forbidden_source_rows())
    }

    pub fn required_capped_callers() -> &'static [&'static str] {
        &[]
    }

    #[cfg(test)]
    pub(crate) fn unknown_old_authority_for_tests() -> Self {
        Self::new(vec![VertexDiskOldAuthorityResidueRow::new(
            "derived_topology::vertex_disks::unknown_old_entry_point",
            "",
            "",
            "",
        )])
    }

    fn new(capped_rows: Vec<VertexDiskOldAuthorityResidueRow>) -> Self {
        let capped_direct_interpreter_count = capped_rows.len();
        let mut parts = vec![
            "worth-topo:vertex-disk-old-authority-residue:v1".to_string(),
            format!("capped-count:{capped_direct_interpreter_count}"),
        ];
        parts.extend(
            capped_rows
                .iter()
                .map(|row| format!("row:{}", row.row_digest())),
        );
        let residue_digest = super::super::super::catalog::catalog_digest(parts);
        Self {
            capped_direct_interpreter_count,
            capped_rows,
            residue_digest,
        }
    }

    pub const fn capped_direct_interpreter_count(&self) -> usize {
        self.capped_direct_interpreter_count
    }

    pub fn capped_rows(&self) -> &[VertexDiskOldAuthorityResidueRow] {
        &self.capped_rows
    }

    pub fn contains_required_caps(&self) -> bool {
        Self::required_capped_callers()
            .iter()
            .all(|required| self.capped_rows.iter().any(|row| row.caller() == *required))
    }

    pub fn residue_digest(&self) -> &str {
        &self.residue_digest
    }
}

fn current_forbidden_source_rows() -> Vec<VertexDiskOldAuthorityResidueRow> {
    source_roots()
        .into_iter()
        .flat_map(scan_source_root_for_forbidden_vertex_disk_authority)
        .collect()
}

fn scan_source_root_for_forbidden_vertex_disk_authority(
    root: PathBuf,
) -> Vec<VertexDiskOldAuthorityResidueRow> {
    let mut rows = Vec::new();
    scan_directory(&root, &mut rows);
    rows
}

fn scan_directory(directory: &Path, rows: &mut Vec<VertexDiskOldAuthorityResidueRow>) {
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if !is_excluded_source_path(&path) {
                scan_directory(&path, rows);
            }
            continue;
        }
        if path.extension().and_then(|ext| ext.to_str()) != Some("rs")
            || is_excluded_source_path(&path)
        {
            continue;
        }
        let Ok(source) = fs::read_to_string(&path) else {
            continue;
        };
        if let Some(token) = forbidden_vertex_disk_authority_token(&source) {
            rows.push(VertexDiskOldAuthorityResidueRow::new(
                path.to_string_lossy().into_owned(),
                "vertex-disk migrated product source firewall",
                token,
                "delete direct vertex-disk authority and route through migrated read-stage",
            ));
        }
    }
}

fn source_roots() -> Vec<PathBuf> {
    [PathBuf::from("crates/worth-topo/src")]
        .into_iter()
        .filter(|path| path.exists())
        .collect()
}

pub(super) fn forbidden_vertex_disk_authority_token(source: &str) -> Option<&'static str> {
    [
        "derived_topology::vertex_disks",
        "derived_topology/vertex_disks",
        "VertexDiskBoundaryInterpretation",
    ]
    .into_iter()
    .find(|token| source.contains(token))
}

fn is_excluded_source_path(path: &Path) -> bool {
    let path = path.to_string_lossy().replace('\\', "/");
    path.contains("/derived_topology/invalidation_plan/migrated_products/vertex_disks/")
        || path.contains("/derived_topology/invalidation_plan/inventory/")
}
