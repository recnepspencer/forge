use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use super::prefix_scan::{scan_segment_reader, WAL_SCAN_BUFFER_BYTES};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalExactFrontierPrefixRequest {
    path: PathBuf,
    segment_id: u64,
    generation: u64,
    target_lsn_end: u64,
    maximum_scan_bytes: u64,
}

impl WalExactFrontierPrefixRequest {
    pub fn new(
        path: impl Into<PathBuf>,
        segment_id: u64,
        generation: u64,
        target_lsn_end: u64,
        maximum_scan_bytes: u64,
    ) -> Self {
        Self {
            path: path.into(),
            segment_id,
            generation,
            target_lsn_end,
            maximum_scan_bytes,
        }
    }
    pub fn path(&self) -> &Path {
        &self.path
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalExactFrontierPrefixDenial {
    InvalidTarget,
    TargetNotFrameBoundary,
    ScanBudgetExceeded,
    InvalidWal,
    Io,
    AllocationFailed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WalExactFrontierPrefix {
    bytes: u64,
    digest: [u8; 32],
    target_lsn_end: u64,
    frames_scanned: u64,
    source_bytes_scanned: u64,
}

impl WalExactFrontierPrefix {
    pub const fn bytes(self) -> u64 {
        self.bytes
    }
    pub const fn digest(self) -> [u8; 32] {
        self.digest
    }
    pub const fn target_lsn_end(self) -> u64 {
        self.target_lsn_end
    }
    pub const fn frames_scanned(self) -> u64 {
        self.frames_scanned
    }
    pub const fn source_bytes_scanned(self) -> u64 {
        self.source_bytes_scanned
    }
}

pub fn inspect_wal_exact_frontier_prefix(
    request: WalExactFrontierPrefixRequest,
) -> Result<WalExactFrontierPrefix, WalExactFrontierPrefixDenial> {
    if request.target_lsn_end == 0 || request.maximum_scan_bytes == 0 {
        return Err(WalExactFrontierPrefixDenial::InvalidTarget);
    }
    let mut file =
        std::fs::File::open(&request.path).map_err(|_| WalExactFrontierPrefixDenial::Io)?;
    let source_bytes = file
        .metadata()
        .map_err(|_| WalExactFrontierPrefixDenial::Io)?
        .len();
    if source_bytes > request.maximum_scan_bytes {
        return Err(WalExactFrontierPrefixDenial::ScanBudgetExceeded);
    }
    let mut buffer = Vec::new();
    buffer
        .try_reserve_exact(WAL_SCAN_BUFFER_BYTES)
        .map_err(|_| WalExactFrontierPrefixDenial::AllocationFailed)?;
    buffer.resize(WAL_SCAN_BUFFER_BYTES, 0);
    let mut prefix_bytes = None;
    let mut frames_scanned = 0_u64;
    let scan = scan_segment_reader(
        &mut file,
        0,
        None,
        request.segment_id,
        request.generation,
        &mut buffer,
        |frame| {
            frames_scanned = frames_scanned.saturating_add(1);
            if frame.lsn_end == request.target_lsn_end {
                prefix_bytes = Some(frame.encoded_end);
            }
        },
    )
    .map_err(|_| WalExactFrontierPrefixDenial::InvalidWal)?;
    let prefix_bytes = prefix_bytes.ok_or(WalExactFrontierPrefixDenial::TargetNotFrameBoundary)?;
    let allocation = usize::try_from(prefix_bytes)
        .map_err(|_| WalExactFrontierPrefixDenial::ScanBudgetExceeded)?;
    let mut prefix = Vec::new();
    prefix
        .try_reserve_exact(allocation)
        .map_err(|_| WalExactFrontierPrefixDenial::AllocationFailed)?;
    prefix.resize(allocation, 0);
    file.seek(SeekFrom::Start(0))
        .map_err(|_| WalExactFrontierPrefixDenial::Io)?;
    file.read_exact(&mut prefix)
        .map_err(|_| WalExactFrontierPrefixDenial::Io)?;
    Ok(WalExactFrontierPrefix {
        bytes: prefix_bytes,
        digest: Sha256::digest(&prefix).into(),
        target_lsn_end: request.target_lsn_end,
        frames_scanned,
        source_bytes_scanned: scan.bytes_scanned,
    })
}
