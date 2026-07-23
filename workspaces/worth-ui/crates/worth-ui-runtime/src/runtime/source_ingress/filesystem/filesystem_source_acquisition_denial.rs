use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiFilesystemSourceAcquisitionDenial {
    RootNotDirectory(PathBuf),
    RootMetadataUnavailable(PathBuf),
    SymbolicLinkUnsupported(PathBuf),
    DirectoryReadFailed(PathBuf),
    NonUtf8ModulePath(PathBuf),
    SourceReadFailed(PathBuf),
    EmptySourceRoot(PathBuf),
    UnstableSourceTree(PathBuf),
    SnapshotAdmissionFailed(PathBuf),
}

impl WorthUiFilesystemSourceAcquisitionDenial {
    pub fn path(&self) -> &Path {
        match self {
            Self::RootNotDirectory(path)
            | Self::RootMetadataUnavailable(path)
            | Self::SymbolicLinkUnsupported(path)
            | Self::DirectoryReadFailed(path)
            | Self::NonUtf8ModulePath(path)
            | Self::SourceReadFailed(path)
            | Self::EmptySourceRoot(path)
            | Self::UnstableSourceTree(path)
            | Self::SnapshotAdmissionFailed(path) => path,
        }
    }
}
