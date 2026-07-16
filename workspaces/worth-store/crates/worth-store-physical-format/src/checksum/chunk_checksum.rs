use worth_store_contracts::StableDigest;

use crate::{
    ExtentBackedRecordView, FramedRecordView, PhysicalChunkChecksumDenial, RecordPlacementClass,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalChunkChecksumAlgorithm {
    S7CanonicalFnv64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalChunkChecksum {
    algorithm: PhysicalChunkChecksumAlgorithm,
    digest: StableDigest,
}

impl PhysicalChunkChecksum {
    pub const fn algorithm(&self) -> PhysicalChunkChecksumAlgorithm {
        self.algorithm
    }

    pub const fn digest(&self) -> &StableDigest {
        &self.digest
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorePhysicalChunkWriteSource {
    PageLocalSlot,
    ExtentBackedReference,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorePhysicalChunkWriteReceipt {
    payload_bytes: Vec<u8>,
    bytes_written: u64,
    source: StorePhysicalChunkWriteSource,
    _seal: private::StorePhysicalChunkWriteReceiptSeal,
}

impl StorePhysicalChunkWriteReceipt {
    pub fn from_page_record_view(
        view: FramedRecordView<'_>,
    ) -> Result<Self, PhysicalChunkChecksumDenial> {
        Self::from_admitted_payload(
            view.payload().as_bytes(),
            StorePhysicalChunkWriteSource::from_record_placement(
                view.placement().placement_class(),
            ),
        )
    }

    pub fn from_extent_record_view(
        view: ExtentBackedRecordView<'_>,
    ) -> Result<Self, PhysicalChunkChecksumDenial> {
        Self::from_admitted_payload(
            view.payload().as_bytes(),
            StorePhysicalChunkWriteSource::ExtentBackedReference,
        )
    }

    pub fn payload_bytes(&self) -> &[u8] {
        &self.payload_bytes
    }

    pub const fn bytes_written(&self) -> u64 {
        self.bytes_written
    }

    pub const fn source(&self) -> StorePhysicalChunkWriteSource {
        self.source
    }

    fn from_admitted_payload(
        payload_bytes: &[u8],
        source: StorePhysicalChunkWriteSource,
    ) -> Result<Self, PhysicalChunkChecksumDenial> {
        if payload_bytes.is_empty() {
            return Err(PhysicalChunkChecksumDenial::EmptyChunkPayload);
        }

        Ok(Self {
            payload_bytes: payload_bytes.to_vec(),
            bytes_written: payload_bytes.len() as u64,
            source,
            _seal: private::StorePhysicalChunkWriteReceiptSeal,
        })
    }

    pub(crate) fn admit_bootstrap_payload(
        payload_bytes: &[u8],
    ) -> Result<Self, PhysicalChunkChecksumDenial> {
        Self::from_admitted_payload(
            payload_bytes,
            StorePhysicalChunkWriteSource::ExtentBackedReference,
        )
    }
}

impl StorePhysicalChunkWriteSource {
    const fn from_record_placement(placement: RecordPlacementClass) -> Self {
        match placement {
            RecordPlacementClass::PageLocalSlot => Self::PageLocalSlot,
            RecordPlacementClass::ExtentBackedReference => Self::ExtentBackedReference,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalChunkChecksumWitness {
    checksum: PhysicalChunkChecksum,
    bytes_checked: u64,
}

impl PhysicalChunkChecksumWitness {
    pub const fn checksum(&self) -> &PhysicalChunkChecksum {
        &self.checksum
    }

    pub const fn bytes_checked(&self) -> u64 {
        self.bytes_checked
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalChunkPayloadIntegrityWitness {
    receipt: StorePhysicalChunkWriteReceipt,
    checksum: PhysicalChunkChecksumWitness,
}

impl PhysicalChunkPayloadIntegrityWitness {
    pub fn payload_bytes(&self) -> &[u8] {
        self.receipt.payload_bytes()
    }

    pub const fn physical_receipt(&self) -> &StorePhysicalChunkWriteReceipt {
        &self.receipt
    }

    pub const fn checksum(&self) -> &PhysicalChunkChecksumWitness {
        &self.checksum
    }

    pub const fn bytes_checked(&self) -> u64 {
        self.checksum.bytes_checked()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalChunkChecksumAuthority {
    algorithm: PhysicalChunkChecksumAlgorithm,
}

impl PhysicalChunkChecksumAuthority {
    pub const fn canonical_blob_checksum() -> Self {
        Self {
            algorithm: PhysicalChunkChecksumAlgorithm::S7CanonicalFnv64,
        }
    }

    pub fn admit_store_payload(
        self,
        receipt: StorePhysicalChunkWriteReceipt,
    ) -> Result<PhysicalChunkPayloadIntegrityWitness, PhysicalChunkChecksumDenial> {
        let checksum = self.verify(receipt.payload_bytes())?;
        Ok(PhysicalChunkPayloadIntegrityWitness { receipt, checksum })
    }

    pub(crate) fn admit_bootstrap_payload(
        self,
        payload_bytes: &[u8],
    ) -> Result<PhysicalChunkPayloadIntegrityWitness, PhysicalChunkChecksumDenial> {
        self.admit_store_payload(StorePhysicalChunkWriteReceipt::admit_bootstrap_payload(
            payload_bytes,
        )?)
    }

    fn verify(
        self,
        bytes: &[u8],
    ) -> Result<PhysicalChunkChecksumWitness, PhysicalChunkChecksumDenial> {
        if bytes.is_empty() {
            return Err(PhysicalChunkChecksumDenial::EmptyChunkPayload);
        }

        Ok(PhysicalChunkChecksumWitness {
            checksum: PhysicalChunkChecksum {
                algorithm: self.algorithm,
                digest: StableDigest::new(format!("fnv64:{:016x}", stable_fnv64(bytes)))
                    .expect("checksum digest is nonempty"),
            },
            bytes_checked: bytes.len() as u64,
        })
    }
}

fn stable_fnv64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

mod private {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(super) struct StorePhysicalChunkWriteReceiptSeal;
}

#[cfg(test)]
mod tests {
    use super::{
        PhysicalChunkChecksumAuthority, PhysicalChunkChecksumDenial, StorePhysicalChunkWriteReceipt,
    };
    use crate::{
        PhysicalBinaryEncodingWitness, PhysicalGeneration, PhysicalGenerationAuthority,
        PhysicalHeaderAuthority, PhysicalPageId, PhysicalPageKind, PhysicalPageRecordAuthority,
        PhysicalRecordSlot, PhysicalReferenceAuthority, PhysicalSegmentId, SlotAppendRequest,
        PHYSICAL_HEADER_LENGTH,
    };

    #[test]
    fn store_payload_admission_requires_executed_physical_record_view() {
        let receipt = record_receipt(b"chunk").unwrap();
        let payload = PhysicalChunkChecksumAuthority::canonical_blob_checksum()
            .admit_store_payload(receipt)
            .unwrap();

        assert_eq!(payload.bytes_checked(), 5);
        assert_eq!(
            record_receipt(b""),
            Err(PhysicalChunkChecksumDenial::EmptyChunkPayload)
        );
    }

    fn record_receipt(
        bytes: &[u8],
    ) -> Result<StorePhysicalChunkWriteReceipt, PhysicalChunkChecksumDenial> {
        let records = record_authority();
        let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
        let references = PhysicalReferenceAuthority::for_canonical_physical_format();
        let page_cell = generations
            .page_cell(segment(7), page(11))
            .with_page_generation(generation(5));
        let slot_cell = generations
            .slot_cell(segment(7), page(11), slot(1))
            .with_slot_generation(generation(9));
        let empty_page = page_bytes(page_cell, &[]);
        let append = records
            .append_record(
                admitted_page(&records, page_cell, &empty_page),
                SlotAppendRequest::ordinary(slot_cell, bytes),
            )
            .unwrap();
        let reopened_page = page_bytes(page_cell, append.page_payload());
        let validation = references
            .validate_page_slot(append.reference_admission(), slot_cell)
            .unwrap();
        let located = records
            .locate_record(
                admitted_page(&records, page_cell, &reopened_page),
                validation,
            )
            .unwrap();
        StorePhysicalChunkWriteReceipt::from_page_record_view(located.record_view())
    }

    fn admitted_page<'a>(
        records: &PhysicalPageRecordAuthority,
        page_cell: crate::PageGenerationCell,
        bytes: &'a [u8],
    ) -> crate::RecordPagePayload<'a> {
        let header = records
            .decode_record_page_header(page_cell, bytes, PhysicalPageKind::DataPage)
            .unwrap();
        records
            .admit_record_page_payload(bytes, header.witness())
            .unwrap()
    }

    fn record_authority() -> PhysicalPageRecordAuthority {
        PhysicalPageRecordAuthority::for_canonical_physical_format(
            PhysicalHeaderAuthority::for_canonical_physical_format(
                PhysicalBinaryEncodingWitness::physical_format_canonical().unwrap(),
            ),
        )
    }

    fn page_bytes(page_cell: crate::PageGenerationCell, payload: &[u8]) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(PHYSICAL_HEADER_LENGTH as usize + payload.len());
        bytes.extend_from_slice(&crate::header::encode_page_header(
            crate::PhysicalByteOrder::LittleEndian,
            PhysicalPageKind::DataPage,
            page_cell,
            payload.len() as u32,
        ));
        bytes.extend_from_slice(payload);
        bytes
    }

    fn segment(value: u64) -> PhysicalSegmentId {
        PhysicalSegmentId::from_raw(value).unwrap()
    }

    fn page(value: u64) -> PhysicalPageId {
        PhysicalPageId::from_raw(value).unwrap()
    }

    fn slot(value: u16) -> PhysicalRecordSlot {
        PhysicalRecordSlot::from_raw(value).unwrap()
    }

    fn generation(value: u64) -> PhysicalGeneration {
        PhysicalGeneration::from_raw(value).unwrap()
    }
}
