use cap_fs_ext::{FollowSymlinks, OpenOptionsFollowExt};

use super::NamespaceDirectoryHandle;

/// Revalidates the opened object immediately before a coordinated namespace
/// mutation. The admitted deployment contract excludes unmanaged mutation by
/// actors sharing the store service account; this check detects drift already
/// visible when the serialized owner operation begins.
pub(super) fn validates_coordinated_name(
    directory: &NamespaceDirectoryHandle,
    name: &str,
    expected: &same_file::Handle,
) -> std::io::Result<bool> {
    let mut options = cap_std::fs::OpenOptions::new();
    options.read(true).follow(FollowSymlinks::No);
    let current = directory
        .directory()
        .open_with(name, &options)
        .map(cap_std::fs::File::into_std)?;
    same_file::Handle::from_file(current).map(|identity| identity == *expected)
}
