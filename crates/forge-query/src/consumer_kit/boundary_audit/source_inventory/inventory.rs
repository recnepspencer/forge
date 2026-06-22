use crate::{ForgeQueryBoundaryAuditSourceSet, ForgeQueryEvidenceIdentity};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBoundaryAuditSourceInventory {
    crate_name: String,
    required_roots: Vec<String>,
    files: Vec<ForgeQueryBoundaryAuditSourceInventoryFile>,
    inventory_identity: ForgeQueryEvidenceIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBoundaryAuditSourceInventoryFile {
    label: String,
    source_path: String,
    source: String,
}

impl ForgeQueryBoundaryAuditSourceInventory {
    pub(super) fn sealed(
        crate_name: String,
        required_roots: Vec<String>,
        files: Vec<ForgeQueryBoundaryAuditSourceInventoryFile>,
        inventory_identity: ForgeQueryEvidenceIdentity,
    ) -> Self {
        Self {
            crate_name,
            required_roots,
            files,
            inventory_identity,
        }
    }

    pub fn crate_name(&self) -> &str {
        &self.crate_name
    }

    pub fn required_roots(&self) -> &[String] {
        &self.required_roots
    }

    pub fn files(&self) -> &[ForgeQueryBoundaryAuditSourceInventoryFile] {
        &self.files
    }

    pub fn source_count(&self) -> usize {
        self.files.len()
    }

    pub fn source_paths(&self) -> Vec<&str> {
        self.files
            .iter()
            .map(ForgeQueryBoundaryAuditSourceInventoryFile::source_path)
            .collect()
    }

    pub fn inventory_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.inventory_identity
    }

    pub fn boundary_sources(&self) -> ForgeQueryBoundaryAuditSourceSet {
        self.files.iter().fold(
            ForgeQueryBoundaryAuditSourceSet::new(self.crate_name()),
            |sources, file| sources.source_file(file.label(), file.source_path(), file.source()),
        )
    }
}

impl ForgeQueryBoundaryAuditSourceInventoryFile {
    pub(super) fn discovered(label: String, source_path: String, source: String) -> Self {
        Self {
            label,
            source_path,
            source,
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub fn source(&self) -> &str {
        &self.source
    }
}
