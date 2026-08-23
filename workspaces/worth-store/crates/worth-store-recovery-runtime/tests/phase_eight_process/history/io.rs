use std::path::Path;
use std::time::{Duration, Instant};

use super::super::child_lifecycle::ProcessChildGuard;

pub(super) fn read_u32(bytes: &[u8], cursor: &mut usize) -> Result<u32, String> {
    let end = cursor
        .checked_add(4)
        .ok_or_else(|| "C8 identity receipt cursor overflow".to_owned())?;
    let value = bytes
        .get(*cursor..end)
        .ok_or_else(|| "C8 identity receipt ended before its count".to_owned())?;
    *cursor = end;
    Ok(u32::from_le_bytes(
        value.try_into().expect("four-byte receipt field"),
    ))
}

pub(super) fn read_u64(bytes: &[u8], cursor: &mut usize) -> Result<u64, String> {
    let end = cursor
        .checked_add(8)
        .ok_or_else(|| "C8 identity receipt cursor overflow".to_owned())?;
    let value = bytes
        .get(*cursor..end)
        .ok_or_else(|| "C8 identity receipt ended before a record ordinal".to_owned())?;
    *cursor = end;
    Ok(u64::from_le_bytes(
        value.try_into().expect("eight-byte receipt field"),
    ))
}

pub(super) fn read_array<const N: usize>(
    bytes: &[u8],
    cursor: &mut usize,
) -> Result<[u8; N], String> {
    let end = cursor
        .checked_add(N)
        .ok_or_else(|| "C8 identity receipt cursor overflow".to_owned())?;
    let value = bytes
        .get(*cursor..end)
        .ok_or_else(|| "C8 identity receipt ended before a record field".to_owned())?;
    *cursor = end;
    value
        .try_into()
        .map_err(|_| "C8 identity receipt record field has the wrong width".to_owned())
}

pub(super) fn read_byte(bytes: &[u8], cursor: &mut usize) -> Result<u8, String> {
    let value = *bytes
        .get(*cursor)
        .ok_or_else(|| "C8 identity receipt ended before a record fate".to_owned())?;
    *cursor += 1;
    Ok(value)
}

pub(crate) fn c8_writer_binary_path() -> std::path::PathBuf {
    super::super::support_binaries::phase_eight_process_binaries()
        .writer()
        .path()
        .to_path_buf()
}

pub(super) fn wait_for_marker(
    child: &mut ProcessChildGuard,
    marker: &Path,
    label: &str,
) -> Result<(), String> {
    let deadline = Instant::now() + Duration::from_secs(120);
    while !marker.exists() {
        if Instant::now() >= deadline {
            return Err(format!("{label} marker timeout: {}", marker.display()));
        }
        if child
            .child_mut()
            .try_wait()
            .map_err(|error| format!("poll {label} child: {error}"))?
            .is_some()
        {
            return Err(format!("production writer exited before {label}"));
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    Ok(())
}
