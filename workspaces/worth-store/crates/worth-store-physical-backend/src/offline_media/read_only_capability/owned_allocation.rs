use std::path::Path;

use super::OfflineMediaReadDenial;

pub(super) fn allocation_for<T>(capacity: usize) -> Result<u64, OfflineMediaReadDenial> {
    u64::try_from(capacity)
        .ok()
        .and_then(|count| count.checked_mul(std::mem::size_of::<T>() as u64))
        .ok_or(OfflineMediaReadDenial::CounterOverflow)
}

pub(super) fn enforce_owned_allocation(
    admitted: u64,
    limit: u64,
) -> Result<(), OfflineMediaReadDenial> {
    if admitted > limit {
        Err(OfflineMediaReadDenial::OwnedAllocationBudgetExceeded { admitted, limit })
    } else {
        Ok(())
    }
}

#[cfg(windows)]
pub(super) fn path_owned_bytes(path: &Path) -> Option<u64> {
    use std::os::windows::ffi::OsStrExt;
    u64::try_from(path.as_os_str().encode_wide().count())
        .ok()?
        .checked_mul(2)
}

#[cfg(unix)]
pub(super) fn path_owned_bytes(path: &Path) -> Option<u64> {
    use std::os::unix::ffi::OsStrExt;
    u64::try_from(path.as_os_str().as_bytes().len()).ok()
}

#[cfg(not(any(windows, unix)))]
pub(super) fn path_owned_bytes(path: &Path) -> Option<u64> {
    u64::try_from(path.to_string_lossy().len()).ok()
}
