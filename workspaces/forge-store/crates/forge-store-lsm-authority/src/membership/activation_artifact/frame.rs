use super::super::artifact::checksum;
use super::super::model::LsmMembershipReadmissionAuthority;
use super::super::session::LsmMembershipDenial;
use super::event::PersistedMembershipActivation;

const MAGIC: [u8; 8] = *b"FSLACTV\0";
const VERSION: u16 = 1;
pub(super) const HEADER_BYTES: usize = 32;

pub(crate) fn decode_activation(
    bytes: &[u8],
    authority: LsmMembershipReadmissionAuthority,
) -> Result<PersistedMembershipActivation, LsmMembershipDenial> {
    if bytes.len() < HEADER_BYTES {
        return Err(LsmMembershipDenial::PersistedMembershipArtifactInvalid);
    }
    let header = &bytes[..HEADER_BYTES];
    if header[..8] != MAGIC {
        return Err(LsmMembershipDenial::PersistedMembershipArtifactInvalid);
    }
    let version = u16::from_le_bytes([header[8], header[9]]);
    let payload_len = u32::from_le_bytes(header[12..16].try_into().unwrap());
    if version != VERSION || header[10..12] != [0, 0] {
        return Err(LsmMembershipDenial::PersistedMembershipArtifactInvalid);
    }
    let expected_header_checksum = u64::from_le_bytes(header[16..24].try_into().unwrap());
    if header_checksum(version, payload_len) != expected_header_checksum {
        return Err(LsmMembershipDenial::PersistedMembershipArtifactInvalid);
    }
    let frame_len = HEADER_BYTES
        .checked_add(payload_len as usize)
        .ok_or(LsmMembershipDenial::PersistedMembershipArtifactInvalid)?;
    if bytes.len() != frame_len {
        return Err(LsmMembershipDenial::PersistedMembershipArtifactInvalid);
    }
    let payload = &bytes[HEADER_BYTES..];
    let expected_payload_checksum = u64::from_le_bytes(header[24..32].try_into().unwrap());
    if payload_checksum(version, payload) != expected_payload_checksum {
        return Err(LsmMembershipDenial::PersistedMembershipArtifactInvalid);
    }
    PersistedMembershipActivation::decode(payload, authority)
}

pub(crate) fn has_activation_magic(bytes: &[u8]) -> bool {
    bytes.starts_with(&MAGIC)
}

pub(crate) fn encode_activation(
    activation: &PersistedMembershipActivation,
) -> Result<Vec<u8>, LsmMembershipDenial> {
    let payload = activation.encode_payload()?;
    let payload_len = u32::try_from(payload.len())
        .map_err(|_| LsmMembershipDenial::PersistedMembershipArtifactInvalid)?;
    let mut frame = Vec::with_capacity(HEADER_BYTES + payload.len());
    frame.extend_from_slice(&MAGIC);
    frame.extend_from_slice(&VERSION.to_le_bytes());
    frame.extend_from_slice(&[0, 0]);
    frame.extend_from_slice(&payload_len.to_le_bytes());
    frame.extend_from_slice(&header_checksum(VERSION, payload_len).to_le_bytes());
    frame.extend_from_slice(&payload_checksum(VERSION, &payload).to_le_bytes());
    frame.extend_from_slice(&payload);
    Ok(frame)
}

fn header_checksum(version: u16, payload_len: u32) -> u64 {
    let mut protected = Vec::with_capacity(6);
    protected.extend_from_slice(&version.to_le_bytes());
    protected.extend_from_slice(&payload_len.to_le_bytes());
    checksum(&protected)
}

fn payload_checksum(version: u16, payload: &[u8]) -> u64 {
    let mut protected = Vec::with_capacity(6 + payload.len());
    protected.extend_from_slice(&version.to_le_bytes());
    protected.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    protected.extend_from_slice(payload);
    checksum(&protected)
}
