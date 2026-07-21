use std::path::{Path, PathBuf};

use crate::runtime::source_ingress::WorthUiSettledSourceSnapshot;

use super::filesystem_source_acquisition_denial::WorthUiFilesystemSourceAcquisitionDenial;
use super::filesystem_source_reader::read_filesystem_source;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiFilesystemSourceProvider {
    root: PathBuf,
}

impl WorthUiFilesystemSourceProvider {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn read(
        &self,
    ) -> Result<WorthUiSettledSourceSnapshot, WorthUiFilesystemSourceAcquisitionDenial> {
        read_filesystem_source(self)
    }
}
