use worth_query_installation::facade::WorthQueryPortablePackageRecordFamily as Family;

use crate::binary_input::BinaryInput;
use crate::compatibility::{
    WorthQueryPackageArchiveCompatibilityProfile, WorthQueryPackageArchiveProtocolLayer,
};
use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};
use crate::limits::WorthQueryPackageArchiveLimits;

use super::super::protocol::{family_from_tag, RECORD_FRAME_HEADER_BYTES};

pub(super) struct DecodedRecordFrame<'a> {
    family: Family,
    canonical_index: u32,
    payload: &'a [u8],
}

impl<'a> DecodedRecordFrame<'a> {
    pub(super) const fn family(&self) -> Family {
        self.family
    }

    pub(super) const fn canonical_index(&self) -> u32 {
        self.canonical_index
    }

    pub(super) const fn payload(&self) -> &'a [u8] {
        self.payload
    }
}

pub(super) fn decode_exact_record_frame(
    bytes: &[u8],
    limits: WorthQueryPackageArchiveLimits,
) -> Result<DecodedRecordFrame<'_>, Denial> {
    validate_record_frame_length(u64::try_from(bytes.len()).unwrap_or(u64::MAX), limits)?;
    let mut input = BinaryInput::new(bytes);
    let limits = limits.narrowed();
    require_supported_record_version(input.u16()?)?;
    let family =
        family_from_tag(input.u16()?).ok_or_else(|| Denial::new(Kind::UnsupportedRecordFamily))?;
    let canonical_index = input.u32()?;
    validate_canonical_index(canonical_index, limits)?;
    let payload_length =
        usize::try_from(input.u32()?).map_err(|_| Denial::new(Kind::InvalidRecordLength))?;
    if payload_length != input.remaining_len() {
        return Err(Denial::new(Kind::InvalidRecordLength));
    }
    let payload = input.take(payload_length)?;
    Ok(DecodedRecordFrame {
        family,
        canonical_index,
        payload,
    })
}

pub(super) fn decode_next_record_frame<'a>(
    input: &mut BinaryInput<'a>,
    limits: WorthQueryPackageArchiveLimits,
) -> Result<DecodedRecordFrame<'a>, Denial> {
    let limits = limits.narrowed();
    require_supported_record_version(input.u16()?)?;
    let family =
        family_from_tag(input.u16()?).ok_or_else(|| Denial::new(Kind::UnsupportedRecordFamily))?;
    let canonical_index = input.u32()?;
    validate_canonical_index(canonical_index, limits)?;
    let payload_length = u64::from(input.u32()?);
    let frame_length = RECORD_FRAME_HEADER_BYTES
        .checked_add(payload_length)
        .ok_or_else(|| Denial::new(Kind::InvalidRecordLength))?;
    validate_record_frame_length(frame_length, limits)?;
    let payload_length =
        usize::try_from(payload_length).map_err(|_| Denial::new(Kind::InvalidRecordLength))?;
    let payload = input.take(payload_length)?;
    Ok(DecodedRecordFrame {
        family,
        canonical_index,
        payload,
    })
}

fn require_supported_record_version(observed_version: u16) -> Result<(), Denial> {
    WorthQueryPackageArchiveCompatibilityProfile::CURRENT
        .admit(
            WorthQueryPackageArchiveProtocolLayer::RecordFrame,
            observed_version,
        )
        .map_err(|compatibility| {
            Denial::incompatible(Kind::UnsupportedRecordVersion, compatibility)
        })
}

pub(super) fn validate_canonical_index(
    canonical_index: u32,
    limits: WorthQueryPackageArchiveLimits,
) -> Result<(), Denial> {
    if canonical_index >= limits.narrowed().maximum_records() {
        return Err(Denial::new(Kind::RecordIndexBudgetExceeded));
    }
    Ok(())
}

pub(super) fn validate_record_frame_length(
    observed: u64,
    limits: WorthQueryPackageArchiveLimits,
) -> Result<(), Denial> {
    let maximum = RECORD_FRAME_HEADER_BYTES
        .checked_add(limits.narrowed().maximum_logical_bytes())
        .ok_or_else(|| Denial::new(Kind::RecordFrameByteBudgetExceeded))?;
    if observed > maximum {
        return Err(Denial::new(Kind::RecordFrameByteBudgetExceeded));
    }
    Ok(())
}
