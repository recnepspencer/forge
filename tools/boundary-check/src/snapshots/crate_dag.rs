use super::document::{CrateDagDocument, DependencyRow, SCHEMA_VERSION};
use crate::manifest_types::Road1Package;

pub(crate) fn crate_dag_document(packages: &[Road1Package]) -> CrateDagDocument {
    let mut rows = packages
        .iter()
        .map(|package| {
            let mut dependencies = package.dependencies.clone();
            dependencies.sort();
            dependencies.dedup();
            DependencyRow {
                package: package.name.clone(),
                dependencies,
            }
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.package.cmp(&right.package));
    CrateDagDocument {
        schema_version: SCHEMA_VERSION,
        packages: rows,
    }
}
