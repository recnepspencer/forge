use super::baseline::{baseline_diagnostic, facade_path, load_facades};
use super::document::FacadeDocument;
use crate::diagnostics::Diagnostic;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

#[derive(Clone)]
pub(crate) struct CommittedFacadeExports {
    by_package: BTreeMap<String, BTreeSet<String>>,
}

impl CommittedFacadeExports {
    fn from_validated_document(document: &FacadeDocument) -> Self {
        Self {
            by_package: document
                .facades
                .iter()
                .map(|row| (row.package.clone(), row.exports.iter().cloned().collect()))
                .collect(),
        }
    }

    pub(crate) fn names_for<'a>(
        &'a self,
        packages: impl Iterator<Item = &'a str>,
    ) -> BTreeSet<String> {
        packages
            .filter_map(|package| self.by_package.get(package))
            .flatten()
            .cloned()
            .collect()
    }
}

pub(crate) fn load_committed_facade_exports(
    root: &Path,
) -> Result<CommittedFacadeExports, Diagnostic> {
    let path = facade_path(root);
    let document = load_facades(&path).map_err(|error| baseline_diagnostic(&path, error))?;
    Ok(CommittedFacadeExports::from_validated_document(&document))
}
