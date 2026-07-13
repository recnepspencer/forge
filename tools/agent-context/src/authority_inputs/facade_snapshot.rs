use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
struct Document {
    schema_version: u32,
    facades: Vec<Row>,
}
#[derive(Deserialize)]
struct Row {
    package: String,
    exports: Vec<String>,
}

pub(crate) struct CommittedFacadeSnapshot {
    path: PathBuf,
    exports: BTreeMap<String, Vec<String>>,
}

impl CommittedFacadeSnapshot {
    pub(crate) fn load(root: &Path) -> Result<Self, String> {
        let path = root.join("tools/boundary-check/snapshots/facades.toml");
        let text = fs::read_to_string(&path)
            .map_err(|e| format!("read facade snapshot {}: {e}", path.display()))?;
        let document: Document = toml::from_str(&text)
            .map_err(|e| format!("parse facade snapshot {}: {e}", path.display()))?;
        if document.schema_version != 1 {
            return Err(format!(
                "unsupported facade snapshot schema {} in {}",
                document.schema_version,
                path.display()
            ));
        }
        let mut exports = BTreeMap::new();
        for row in document.facades {
            let mut seen = BTreeSet::new();
            if row.exports.iter().any(|item| !seen.insert(item)) {
                return Err(format!(
                    "duplicate facade export for {} in {}",
                    row.package,
                    path.display()
                ));
            }
            if exports.insert(row.package.clone(), row.exports).is_some() {
                return Err(format!(
                    "duplicate facade row {} in {}",
                    row.package,
                    path.display()
                ));
            }
        }
        Ok(Self { path, exports })
    }

    pub(crate) fn exports_for(&self, package: &str) -> Result<Vec<String>, String> {
        self.exports.get(package).cloned().ok_or_else(|| {
            format!(
                "facade snapshot {} has no row for governed package {package}",
                self.path.display()
            )
        })
    }
}
