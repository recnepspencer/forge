use super::resident_frame_source::ResidentFrameSourceKey;
use crate::{ResidentFrameDenial, ResidentFrameDenialKind};
use worth_store_physical_format::{
    PhysicalHeaderDecodeWitness, PhysicalHeaderKind, PhysicalReferenceKind,
    PhysicalReferenceValidationWitness,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResidentFrameSize {
    bytes: u64,
}

impl ResidentFrameSize {
    pub(crate) fn from_header(
        header: PhysicalHeaderDecodeWitness,
    ) -> Result<Self, ResidentFrameDenial> {
        let Some(bytes) =
            (header.payload_offset() as u64).checked_add(header.payload_length() as u64)
        else {
            return Err(ResidentFrameDenial::new(
                ResidentFrameDenialKind::FrameSizeOverflow,
            ));
        };
        Ok(Self { bytes })
    }

    pub const fn as_bytes(self) -> u64 {
        self.bytes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResidentFrameLoadRequest {
    reference: PhysicalReferenceValidationWitness,
    header: PhysicalHeaderDecodeWitness,
    frame_size: ResidentFrameSize,
    source_key: ResidentFrameSourceKey,
}

impl ResidentFrameLoadRequest {
    pub fn from_physical_format_physical_frame(
        reference: PhysicalReferenceValidationWitness,
        header: PhysicalHeaderDecodeWitness,
    ) -> Result<Self, ResidentFrameDenial> {
        reject_non_page_slot_reference(reference)?;
        reject_non_frame_header(header)?;
        reject_header_reference_mismatch(reference, header)?;
        let frame_size = ResidentFrameSize::from_header(header)?;
        let source_key =
            ResidentFrameSourceKey::from_physical_format_frame_witnesses(reference, header);
        Ok(Self {
            reference,
            header,
            frame_size,
            source_key,
        })
    }

    pub const fn reference(self) -> PhysicalReferenceValidationWitness {
        self.reference
    }

    pub const fn header(self) -> PhysicalHeaderDecodeWitness {
        self.header
    }

    pub const fn frame_size(self) -> ResidentFrameSize {
        self.frame_size
    }

    pub(crate) const fn source_key(self) -> ResidentFrameSourceKey {
        self.source_key
    }
}

fn reject_non_page_slot_reference(
    reference: PhysicalReferenceValidationWitness,
) -> Result<(), ResidentFrameDenial> {
    if !matches!(
        reference.reference().kind(),
        PhysicalReferenceKind::PageSlot
    ) {
        return Err(ResidentFrameDenial::new(
            ResidentFrameDenialKind::PhysicalReferenceKindRejected,
        ));
    }
    Ok(())
}

fn reject_non_frame_header(header: PhysicalHeaderDecodeWitness) -> Result<(), ResidentFrameDenial> {
    if !matches!(header.kind(), PhysicalHeaderKind::Frame(_)) {
        return Err(ResidentFrameDenial::new(
            ResidentFrameDenialKind::PhysicalHeaderKindRejected,
        ));
    }
    Ok(())
}

fn reject_header_reference_mismatch(
    reference: PhysicalReferenceValidationWitness,
    header: PhysicalHeaderDecodeWitness,
) -> Result<(), ResidentFrameDenial> {
    if reference.owner() != header.owner() {
        return Err(ResidentFrameDenial::new(
            ResidentFrameDenialKind::HeaderOwnerMismatch,
        ));
    }
    Ok(())
}
