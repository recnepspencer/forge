use crate::{
    ChunkDamageLocality, ChunkIntegrityCounters, ChunkIntegrityDenial, ChunkIntegrityDenialKind,
    ChunkIntegrityInspectionRequest, ChunkIntegrityReport,
};
use worth_store_physical_format::PhysicalHeaderKind;

const CHUNK_MAGIC: &[u8] = b"CHNK|";
const CHUNK_STATUS_OK: &str = "ok";
const CHUNK_STATUS_HEADER_DAMAGE: &str = "header-damage";
const CHUNK_STATUS_PAYLOAD_DAMAGE: &str = "payload-damage";
const CHUNK_STATUS_CHUNK_BOUNDARY_DAMAGE: &str = "chunk-boundary-damage";
const CHUNK_STATUS_EXTENT_BOUNDARY_DAMAGE: &str = "extent-boundary-damage";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkIntegrityAuthority;

impl ChunkIntegrityAuthority {
    pub const fn new() -> Self {
        Self
    }

    pub fn inspect(
        self,
        request: ChunkIntegrityInspectionRequest<'_>,
    ) -> Result<ChunkIntegrityReport, ChunkIntegrityDenial> {
        let input = request.input();
        let basis = input.admission().basis().clone();
        let window = request.streaming_window();
        let Some(frame) = input.admission().checked_frame() else {
            let counters =
                protected_window_counters(window.object_bytes(), window.window_bytes(), 0);
            return Err(chunk_denial(
                ChunkIntegrityDenialKind::MissingCheckedChunkWindow,
                counters,
                basis,
                None,
            ));
        };
        let counters = protected_window_counters(
            window.object_bytes(),
            window.window_bytes(),
            frame.checked_bytes().len_bytes() as u64,
        );
        reject_protected_window_overread(
            frame.checked_bytes().len_bytes() as u64,
            window,
            counters,
            basis.clone(),
        )?;
        if !matches!(
            frame.physical_witness().kind(),
            PhysicalHeaderKind::Frame(_)
        ) {
            return Err(chunk_denial(
                ChunkIntegrityDenialKind::ChunkHeaderDamage,
                counters.with_chunk_header_check(),
                basis.clone(),
                Some(ChunkDamageLocality::ChunkHeader(basis.scope())),
            ));
        }
        inspect_chunk_window_bytes(frame.checked_bytes().as_bytes(), counters, basis, window)
    }
}

impl Default for ChunkIntegrityAuthority {
    fn default() -> Self {
        Self::new()
    }
}

fn reject_protected_window_overread(
    protected_bytes: u64,
    window: crate::ChunkIntegrityStreamingWindow,
    counters: ChunkIntegrityCounters,
    basis: crate::PhysicalScopeBasis,
) -> Result<(), ChunkIntegrityDenial> {
    if protected_bytes <= window.window_bytes() {
        return Ok(());
    }
    Err(chunk_denial(
        ChunkIntegrityDenialKind::ProtectedWindowExceedsStreamingWindow,
        counters,
        basis.clone(),
        Some(ChunkDamageLocality::Unknown(basis.scope())),
    ))
}

fn inspect_chunk_window_bytes(
    bytes: &[u8],
    counters: ChunkIntegrityCounters,
    basis: crate::PhysicalScopeBasis,
    window: crate::ChunkIntegrityStreamingWindow,
) -> Result<ChunkIntegrityReport, ChunkIntegrityDenial> {
    let Some(status) = parse_chunk_window_status(bytes) else {
        return Err(chunk_denial(
            ChunkIntegrityDenialKind::ChunkHeaderDamage,
            counters.with_chunk_header_check(),
            basis.clone(),
            Some(ChunkDamageLocality::ChunkHeader(basis.scope())),
        ));
    };
    match status {
        CHUNK_STATUS_OK => Ok(ChunkIntegrityReport::new(
            basis,
            window.object_bytes(),
            window.window_bytes(),
            completed_chunk_counters(counters),
        )),
        CHUNK_STATUS_HEADER_DAMAGE => Err(chunk_denial(
            ChunkIntegrityDenialKind::ChunkHeaderDamage,
            counters.with_chunk_header_check(),
            basis.clone(),
            Some(ChunkDamageLocality::ChunkHeader(basis.scope())),
        )),
        CHUNK_STATUS_PAYLOAD_DAMAGE => Err(chunk_denial(
            ChunkIntegrityDenialKind::ChunkPayloadDamage,
            counters
                .with_chunk_header_check()
                .with_chunk_payload_check(),
            basis.clone(),
            Some(ChunkDamageLocality::ChunkPayload(basis.scope())),
        )),
        CHUNK_STATUS_CHUNK_BOUNDARY_DAMAGE => Err(chunk_denial(
            ChunkIntegrityDenialKind::ChunkBoundaryDamage,
            counters
                .with_chunk_header_check()
                .with_chunk_payload_check()
                .with_chunk_boundary_check(),
            basis.clone(),
            Some(ChunkDamageLocality::ChunkBoundary(basis.scope())),
        )),
        CHUNK_STATUS_EXTENT_BOUNDARY_DAMAGE => Err(chunk_denial(
            ChunkIntegrityDenialKind::ExtentBoundaryDamage,
            completed_chunk_counters(counters),
            basis.clone(),
            Some(ChunkDamageLocality::ExtentBoundary(basis.scope())),
        )),
        _ => Err(chunk_denial(
            ChunkIntegrityDenialKind::UnknownChunkIntegrity,
            counters.with_chunk_header_check(),
            basis.clone(),
            Some(ChunkDamageLocality::Unknown(basis.scope())),
        )),
    }
}

fn protected_window_counters(
    object_bytes: u64,
    window_bytes: u64,
    inspected_bytes: u64,
) -> ChunkIntegrityCounters {
    ChunkIntegrityCounters::start(object_bytes, window_bytes, inspected_bytes)
        .with_skipped_whole_object_read()
}

fn completed_chunk_counters(counters: ChunkIntegrityCounters) -> ChunkIntegrityCounters {
    counters
        .with_chunk_header_check()
        .with_chunk_payload_check()
        .with_chunk_boundary_check()
        .with_extent_boundary_check()
}

fn parse_chunk_window_status(bytes: &[u8]) -> Option<&str> {
    let tail = bytes.strip_prefix(CHUNK_MAGIC)?;
    let (status, _) = split_once(tail, b'|')?;
    std::str::from_utf8(status).ok()
}

fn split_once(bytes: &[u8], delimiter: u8) -> Option<(&[u8], &[u8])> {
    let index = bytes.iter().position(|byte| *byte == delimiter)?;
    Some((&bytes[..index], &bytes[index + 1..]))
}

fn chunk_denial(
    kind: ChunkIntegrityDenialKind,
    counters: ChunkIntegrityCounters,
    basis: crate::PhysicalScopeBasis,
    locality: Option<ChunkDamageLocality>,
) -> ChunkIntegrityDenial {
    let denial = ChunkIntegrityDenial::new(kind, counters).with_basis(basis);
    match locality {
        Some(locality) => denial.with_damage_locality(locality),
        None => denial,
    }
}
