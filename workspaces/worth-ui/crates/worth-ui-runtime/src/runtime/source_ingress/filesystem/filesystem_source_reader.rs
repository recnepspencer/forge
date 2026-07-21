use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::thread;

use crate::runtime::source_ingress::provider::WorthUiSourceProvider;
use crate::runtime::source_ingress::{
    WorthUiReloadDebounce, WorthUiSettledSourceSnapshot, WorthUiWatcherEvent,
};

use super::filesystem_source_acquisition_denial::WorthUiFilesystemSourceAcquisitionDenial;
use super::filesystem_source_provider::WorthUiFilesystemSourceProvider;

pub(super) fn read_filesystem_source(
    provider: &WorthUiFilesystemSourceProvider,
) -> Result<WorthUiSettledSourceSnapshot, WorthUiFilesystemSourceAcquisitionDenial> {
    let debounce = WorthUiReloadDebounce::default();
    freeze_filesystem_source(
        provider,
        &debounce,
        &[WorthUiWatcherEvent::provider_revision(
            provider.root().to_string_lossy(),
        )],
        1,
    )
}

pub(super) fn freeze_filesystem_source(
    provider: &WorthUiFilesystemSourceProvider,
    debounce: &WorthUiReloadDebounce,
    events: &[WorthUiWatcherEvent],
    sequence: u64,
) -> Result<WorthUiSettledSourceSnapshot, WorthUiFilesystemSourceAcquisitionDenial> {
    let root = canonical_source_root(provider.root())?;
    let first = read_module_tree(&root)?;
    thread::sleep(debounce.settlement_window());
    let second = read_module_tree(&root)?;
    freeze_matching_module_trees(debounce, events, sequence, root, first, second)
}

fn freeze_matching_module_trees(
    debounce: &WorthUiReloadDebounce,
    events: &[WorthUiWatcherEvent],
    sequence: u64,
    root: PathBuf,
    first: Vec<(String, String)>,
    second: Vec<(String, String)>,
) -> Result<WorthUiSettledSourceSnapshot, WorthUiFilesystemSourceAcquisitionDenial> {
    if first != second {
        return Err(WorthUiFilesystemSourceAcquisitionDenial::UnstableSourceTree(root));
    }
    if second.is_empty() {
        return Err(WorthUiFilesystemSourceAcquisitionDenial::EmptySourceRoot(
            root,
        ));
    }
    let denial_path = root.clone();
    let provider = WorthUiSourceProvider::filesystem_snapshot(root, second);
    debounce
        .debounce(provider, events, sequence)
        .map_err(|_| WorthUiFilesystemSourceAcquisitionDenial::SnapshotAdmissionFailed(denial_path))
}

pub(super) fn canonical_source_root(
    root: &Path,
) -> Result<PathBuf, WorthUiFilesystemSourceAcquisitionDenial> {
    let metadata = fs::symlink_metadata(root).map_err(|_| {
        WorthUiFilesystemSourceAcquisitionDenial::RootMetadataUnavailable(root.to_path_buf())
    })?;
    if metadata.file_type().is_symlink() {
        return Err(
            WorthUiFilesystemSourceAcquisitionDenial::SymbolicLinkUnsupported(root.to_path_buf()),
        );
    }
    if !metadata.is_dir() {
        return Err(WorthUiFilesystemSourceAcquisitionDenial::RootNotDirectory(
            root.to_path_buf(),
        ));
    }
    fs::canonicalize(root).map_err(|_| {
        WorthUiFilesystemSourceAcquisitionDenial::RootMetadataUnavailable(root.to_path_buf())
    })
}

fn read_module_tree(
    root: &Path,
) -> Result<Vec<(String, String)>, WorthUiFilesystemSourceAcquisitionDenial> {
    let mut modules = Vec::new();
    read_directory(root, root, &mut modules)?;
    modules.sort_by(|left, right| left.0.cmp(&right.0));
    Ok(modules)
}

fn read_directory(
    root: &Path,
    directory: &Path,
    modules: &mut Vec<(String, String)>,
) -> Result<(), WorthUiFilesystemSourceAcquisitionDenial> {
    let entries = fs::read_dir(directory).map_err(|_| {
        WorthUiFilesystemSourceAcquisitionDenial::DirectoryReadFailed(directory.to_path_buf())
    })?;
    let mut paths = entries
        .map(|entry| {
            entry.map(|entry| entry.path()).map_err(|_| {
                WorthUiFilesystemSourceAcquisitionDenial::DirectoryReadFailed(
                    directory.to_path_buf(),
                )
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    paths.sort();
    for path in paths {
        read_path(root, &path, modules)?;
    }
    Ok(())
}

fn read_path(
    root: &Path,
    path: &Path,
    modules: &mut Vec<(String, String)>,
) -> Result<(), WorthUiFilesystemSourceAcquisitionDenial> {
    let metadata = fs::symlink_metadata(path).map_err(|_| {
        WorthUiFilesystemSourceAcquisitionDenial::RootMetadataUnavailable(path.to_path_buf())
    })?;
    if metadata.file_type().is_symlink() {
        return Err(
            WorthUiFilesystemSourceAcquisitionDenial::SymbolicLinkUnsupported(path.to_path_buf()),
        );
    }
    if metadata.is_dir() {
        return read_directory(root, path, modules);
    }
    if path.extension() != Some(OsStr::new("wui")) {
        return Ok(());
    }
    let relative = path
        .strip_prefix(root)
        .expect("walked path must remain below root");
    let relative = relative
        .to_str()
        .ok_or_else(|| {
            WorthUiFilesystemSourceAcquisitionDenial::NonUtf8ModulePath(path.to_path_buf())
        })?
        .replace('\\', "/");
    let source = fs::read_to_string(path).map_err(|_| {
        WorthUiFilesystemSourceAcquisitionDenial::SourceReadFailed(path.to_path_buf())
    })?;
    modules.push((relative, source));
    Ok(())
}
