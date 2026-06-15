use std::path::{Path, PathBuf};

use crate::source::{WorthUiSourceImport, WorthUiSourceModuleId};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiSourceModuleRecord {
    module_id: WorthUiSourceModuleId,
    relative_path: PathBuf,
    source_text: String,
    imports: Vec<WorthUiSourceImport>,
}

impl WorthUiSourceModuleRecord {
    pub(crate) fn new(
        module_id: WorthUiSourceModuleId,
        relative_path: PathBuf,
        source_text: String,
        imports: Vec<WorthUiSourceImport>,
    ) -> Self {
        Self {
            module_id,
            relative_path,
            source_text,
            imports,
        }
    }

    pub(crate) fn module_id(&self) -> &WorthUiSourceModuleId {
        &self.module_id
    }

    pub(crate) fn relative_path(&self) -> &Path {
        &self.relative_path
    }

    pub(crate) fn source_text(&self) -> &str {
        &self.source_text
    }

    pub(crate) fn imports(&self) -> &[WorthUiSourceImport] {
        &self.imports
    }
}
