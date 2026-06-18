use std::path::PathBuf;

use crate::consumer_kit::boundary_audit::error::{
    ForgeQueryBoundaryAuditError, ForgeQueryBoundaryAuditErrorKind,
};

use super::evidence::derive_source_inventory_identity;
use super::filesystem::{collect_rs_files, normalize_path};
use super::inventory::ForgeQueryBoundaryAuditSourceInventory;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBoundaryAuditSourceInventoryBuilder {
    crate_name: String,
    required_roots: Vec<PathBuf>,
    include_rs_files: bool,
}

impl ForgeQueryBoundaryAuditSourceInventoryBuilder {
    pub(crate) fn new(crate_name: impl Into<String>) -> Self {
        Self {
            crate_name: crate_name.into(),
            required_roots: Vec::new(),
            include_rs_files: false,
        }
    }

    pub fn required_root(mut self, root: impl Into<PathBuf>) -> Self {
        self.required_roots.push(root.into());
        self
    }

    pub fn include_rs_files(mut self) -> Self {
        self.include_rs_files = true;
        self
    }

    pub fn seal(
        self,
    ) -> Result<ForgeQueryBoundaryAuditSourceInventory, ForgeQueryBoundaryAuditError> {
        if self.crate_name.trim().is_empty() {
            return Err(ForgeQueryBoundaryAuditError::new(
                ForgeQueryBoundaryAuditErrorKind::EmptyCrateName,
                "boundary audit source inventory crate name must not be empty",
            ));
        }

        let mut files = Vec::new();
        let mut required_roots = Vec::new();
        for root in self.required_roots {
            if !root.exists() {
                return Err(ForgeQueryBoundaryAuditError::new(
                    ForgeQueryBoundaryAuditErrorKind::MissingRequiredRoot,
                    format!(
                        "required boundary audit source root `{}` does not exist",
                        root.display()
                    ),
                ));
            }
            required_roots.push(normalize_path(&root));
            if self.include_rs_files {
                collect_rs_files(&self.crate_name, &root, &mut files)?;
            }
        }
        files.sort_by(|left, right| left.source_path().cmp(right.source_path()));
        let inventory_identity =
            derive_source_inventory_identity(&self.crate_name, &required_roots, &files);

        Ok(ForgeQueryBoundaryAuditSourceInventory::sealed(
            self.crate_name,
            required_roots,
            files,
            inventory_identity,
        ))
    }
}
