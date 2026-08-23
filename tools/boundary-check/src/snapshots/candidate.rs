use super::baseline::compare_exact_sets;
use super::commit::commit_snapshot_pair;
use super::crate_dag::crate_dag_document;
use super::document::{CrateDagDocument, FacadeDocument};
use super::facade_surface_observation::{
    observe_facade_document, ConfiguredFacadeSurface, ObservedFacadeExports,
};
use crate::diagnostics::Diagnostic;
use crate::manifest_types::Road1Package;
use std::path::{Path, PathBuf};

pub(crate) struct ConstitutionSnapshots {
    dag: CrateDagDocument,
    facades: FacadeDocument,
}

impl ConstitutionSnapshots {
    pub(crate) fn observe(
        packages: &[Road1Package],
        configured_surfaces: &[ConfiguredFacadeSurface],
    ) -> Result<Self, String> {
        Ok(Self {
            dag: crate_dag_document(packages),
            facades: observe_facade_document(packages, configured_surfaces)?,
        })
    }

    pub(crate) fn check(&self, root: &Path) -> Vec<Diagnostic> {
        compare_exact_sets(root, &self.dag, &self.facades)
    }

    pub(crate) fn write(&self, root: &Path) -> Result<Vec<PathBuf>, String> {
        commit_snapshot_pair(root, &self.dag, &self.facades)
    }

    pub(crate) fn observed_facade_exports(&self) -> ObservedFacadeExports {
        ObservedFacadeExports::from_document(&self.facades)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::DiagnosticCode;
    use crate::snapshots::document::{DependencyRow, FacadeRow, SCHEMA_VERSION};
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn root() -> PathBuf {
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("boundary-snapshot-{id}"))
    }

    fn candidate() -> ConstitutionSnapshots {
        ConstitutionSnapshots {
            dag: CrateDagDocument {
                schema_version: SCHEMA_VERSION,
                packages: vec![DependencyRow {
                    package: "worth-schema-core".into(),
                    dependencies: vec![],
                }],
            },
            facades: FacadeDocument {
                schema_version: SCHEMA_VERSION,
                facades: vec![FacadeRow {
                    package: "worth-schema-core".into(),
                    exports: vec!["Identity".into()],
                }],
            },
        }
    }

    #[test]
    fn regeneration_is_byte_identical_and_check_is_exact() {
        let root = root();
        let baseline = candidate();
        baseline.write(&root).unwrap();
        let first_dag =
            fs::read(root.join("tools/boundary-check/snapshots/crate-dag.toml")).unwrap();
        let first_facades =
            fs::read(root.join("tools/boundary-check/snapshots/facades.toml")).unwrap();
        baseline.write(&root).unwrap();
        assert_eq!(
            first_dag,
            fs::read(root.join("tools/boundary-check/snapshots/crate-dag.toml")).unwrap()
        );
        assert_eq!(
            first_facades,
            fs::read(root.join("tools/boundary-check/snapshots/facades.toml")).unwrap()
        );
        assert!(baseline.check(&root).is_empty());

        let mut widened = candidate();
        widened.dag.packages[0]
            .dependencies
            .push("worth-query-decl".into());
        widened.facades.facades[0].exports.push("Name".into());
        let codes = widened
            .check(&root)
            .into_iter()
            .map(|diagnostic| diagnostic.code())
            .collect::<Vec<_>>();
        assert!(codes.contains(&DiagnosticCode::Bc8003CrateDagSnapshotDrift));
        assert!(codes.contains(&DiagnosticCode::Bc8002FacadeSnapshotDrift));

        baseline.write(&root).unwrap();
        let empty = ConstitutionSnapshots {
            dag: CrateDagDocument {
                schema_version: SCHEMA_VERSION,
                packages: vec![],
            },
            facades: FacadeDocument {
                schema_version: SCHEMA_VERSION,
                facades: vec![],
            },
        };
        assert!(!empty.check(&root).is_empty());
    }
}
