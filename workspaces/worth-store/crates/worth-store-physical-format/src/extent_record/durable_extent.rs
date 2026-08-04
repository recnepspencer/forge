use crate::record_framing::{
    decode_durable_frame, initialize_durable_frame_reusing, reseal_durable_frame,
    DURABLE_FRAME_HEADER_BYTES,
};
use crate::{
    DurableFrameDenial, DurableFrameKind, PersistedRecordIdentity, PhysicalRecordFormatDeclaration,
    RecordExtentGenerationCell,
};

pub const EXTENT_CHUNK_METADATA_BYTES: usize = 64;
pub const DURABLE_EXTENT_FRAME_HEADER_BYTES: usize = DURABLE_FRAME_HEADER_BYTES;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExtentChunkCoordinate {
    record: PersistedRecordIdentity,
    extent: RecordExtentGenerationCell,
    logical_bytes: u64,
    logical_offset: u64,
    ordinal: u32,
}

impl ExtentChunkCoordinate {
    pub const fn new(
        record: PersistedRecordIdentity,
        extent: RecordExtentGenerationCell,
        logical_bytes: u64,
        logical_offset: u64,
        ordinal: u32,
    ) -> Option<Self> {
        if logical_bytes == 0 || logical_offset >= logical_bytes || ordinal == 0 {
            return None;
        }
        Some(Self {
            record,
            extent,
            logical_bytes,
            logical_offset,
            ordinal,
        })
    }

    pub const fn record(self) -> PersistedRecordIdentity {
        self.record
    }

    pub const fn extent_cell(self) -> RecordExtentGenerationCell {
        self.extent
    }

    pub const fn logical_bytes(self) -> u64 {
        self.logical_bytes
    }

    pub const fn logical_offset(self) -> u64 {
        self.logical_offset
    }

    pub const fn ordinal(self) -> u32 {
        self.ordinal
    }
}

pub struct UnsealedExtentChunk {
    frame: Vec<u8>,
    format: PhysicalRecordFormatDeclaration,
    ordinal: u32,
}

impl UnsealedExtentChunk {
    pub fn payload_mut(&mut self) -> &mut [u8] {
        &mut self.frame[DURABLE_FRAME_HEADER_BYTES + EXTENT_CHUNK_METADATA_BYTES..]
    }

    pub fn seal(mut self) -> Vec<u8> {
        reseal_durable_frame(
            &mut self.frame,
            DurableFrameKind::Extent,
            self.format,
            u64::from(self.ordinal),
        );
        self.frame
    }
}

pub fn prepare_extent_chunk(
    format: PhysicalRecordFormatDeclaration,
    coordinate: ExtentChunkCoordinate,
    chunk_len: usize,
) -> Result<UnsealedExtentChunk, ExtentFrameDenial> {
    prepare_extent_chunk_reusing(format, coordinate, chunk_len, Vec::new())
}

pub fn prepare_extent_chunk_reusing(
    format: PhysicalRecordFormatDeclaration,
    coordinate: ExtentChunkCoordinate,
    chunk_len: usize,
    scratch: Vec<u8>,
) -> Result<UnsealedExtentChunk, ExtentFrameDenial> {
    if chunk_len == 0 {
        return Err(ExtentFrameDenial::MalformedLength);
    }
    let chunk_len = u32::try_from(chunk_len).map_err(|_| ExtentFrameDenial::PayloadTooLarge)?;
    if coordinate
        .logical_offset
        .checked_add(u64::from(chunk_len))
        .filter(|end| *end <= coordinate.logical_bytes)
        .is_none()
    {
        return Err(ExtentFrameDenial::MalformedLength);
    }
    let mut frame = initialize_durable_frame_reusing(
        scratch,
        DurableFrameKind::Extent,
        format,
        u64::from(coordinate.ordinal),
        EXTENT_CHUNK_METADATA_BYTES + chunk_len as usize,
    );
    let payload = &mut frame[DURABLE_FRAME_HEADER_BYTES..];
    payload[..16].copy_from_slice(&coordinate.record.allocation_epoch());
    payload[16..24].copy_from_slice(&coordinate.record.ordinal().to_le_bytes());
    payload[24..32].copy_from_slice(&coordinate.extent.extent_id().get().to_le_bytes());
    payload[32..40].copy_from_slice(&coordinate.extent.generation().get().to_le_bytes());
    payload[40..48].copy_from_slice(&coordinate.logical_bytes.to_le_bytes());
    payload[48..56].copy_from_slice(&coordinate.logical_offset.to_le_bytes());
    payload[56..60].copy_from_slice(&chunk_len.to_le_bytes());
    Ok(UnsealedExtentChunk {
        frame,
        format,
        ordinal: coordinate.ordinal,
    })
}

pub fn encode_extent_chunk(
    format: PhysicalRecordFormatDeclaration,
    coordinate: ExtentChunkCoordinate,
    chunk: &[u8],
) -> Result<Vec<u8>, ExtentFrameDenial> {
    let mut frame = prepare_extent_chunk(format, coordinate, chunk.len())?;
    frame.payload_mut().copy_from_slice(chunk);
    Ok(frame.seal())
}

pub fn decode_extent_chunk(
    bytes: &[u8],
    expected: ExtentChunkCoordinate,
) -> Result<(&[u8], PhysicalRecordFormatDeclaration), ExtentFrameDenial> {
    let (format, frame) =
        decode_durable_frame(bytes, DurableFrameKind::Extent).map_err(ExtentFrameDenial::Frame)?;
    if frame.identity != u64::from(expected.ordinal)
        || frame.payload.len() < EXTENT_CHUNK_METADATA_BYTES
        || frame.payload[60..64] != [0; 4]
    {
        return Err(ExtentFrameDenial::MalformedLength);
    }
    let record = PersistedRecordIdentity::new(
        frame.payload[..16].try_into().unwrap(),
        u64::from_le_bytes(frame.payload[16..24].try_into().unwrap()),
    )
    .ok_or(ExtentFrameDenial::InvalidRecordIdentity)?;
    if record != expected.record {
        return Err(ExtentFrameDenial::RecordIdentityMismatch);
    }
    if u64::from_le_bytes(frame.payload[24..32].try_into().unwrap())
        != expected.extent.extent_id().get()
        || u64::from_le_bytes(frame.payload[40..48].try_into().unwrap()) != expected.logical_bytes
        || u64::from_le_bytes(frame.payload[48..56].try_into().unwrap()) != expected.logical_offset
    {
        return Err(ExtentFrameDenial::MalformedLength);
    }
    if u64::from_le_bytes(frame.payload[32..40].try_into().unwrap())
        != expected.extent.generation().get()
    {
        return Err(ExtentFrameDenial::GenerationMismatch);
    }
    let length = u32::from_le_bytes(frame.payload[56..60].try_into().unwrap()) as usize;
    if length == 0 || frame.payload.len() != EXTENT_CHUNK_METADATA_BYTES + length {
        return Err(ExtentFrameDenial::MalformedLength);
    }
    Ok((&frame.payload[EXTENT_CHUNK_METADATA_BYTES..], format))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtentFrameDenial {
    Frame(DurableFrameDenial),
    MalformedLength,
    InvalidRecordIdentity,
    RecordIdentityMismatch,
    GenerationMismatch,
    PayloadTooLarge,
}
