use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiFilesystemWatcherBackend {
    Fsevent,
    Inotify,
    Kqueue,
    ReadDirectoryChanges,
    OtherNative,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiFilesystemWatcherReadiness {
    root: PathBuf,
    backend: WorthUiFilesystemWatcherBackend,
}

impl WorthUiFilesystemWatcherReadiness {
    pub(super) fn new(root: PathBuf, backend: WorthUiFilesystemWatcherBackend) -> Self {
        Self { root, backend }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn backend(&self) -> WorthUiFilesystemWatcherBackend {
        self.backend
    }
}
