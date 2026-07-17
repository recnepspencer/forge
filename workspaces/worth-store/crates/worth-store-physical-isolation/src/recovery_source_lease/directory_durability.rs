use std::path::Path;

use super::RecoverySourceLeaseDenial;

#[cfg(windows)]
pub(super) fn sync_directory(path: &Path) -> Result<(), RecoverySourceLeaseDenial> {
    use std::os::windows::fs::OpenOptionsExt;

    std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(0x0200_0000)
        .open(path)?
        .sync_all()?;
    Ok(())
}

#[cfg(not(windows))]
pub(super) fn sync_directory(path: &Path) -> Result<(), RecoverySourceLeaseDenial> {
    std::fs::File::open(path)?.sync_all()?;
    Ok(())
}
