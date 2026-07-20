#[cfg(unix)]
pub(super) fn positioned_read(
    file: &std::fs::File,
    buffer: &mut [u8],
    offset: u64,
) -> std::io::Result<usize> {
    std::os::unix::fs::FileExt::read_at(file, buffer, offset)
}

#[cfg(windows)]
pub(super) fn positioned_read(
    file: &std::fs::File,
    buffer: &mut [u8],
    offset: u64,
) -> std::io::Result<usize> {
    std::os::windows::fs::FileExt::seek_read(file, buffer, offset)
}

#[cfg(unix)]
pub(super) fn positioned_write(
    file: &std::fs::File,
    buffer: &[u8],
    offset: u64,
) -> std::io::Result<usize> {
    std::os::unix::fs::FileExt::write_at(file, buffer, offset)
}

#[cfg(windows)]
pub(super) fn positioned_write(
    file: &std::fs::File,
    buffer: &[u8],
    offset: u64,
) -> std::io::Result<usize> {
    std::os::windows::fs::FileExt::seek_write(file, buffer, offset)
}

#[cfg(not(any(unix, windows)))]
compile_error!("C.4 filesystem media requires an explicitly supported positioned-I/O platform");
