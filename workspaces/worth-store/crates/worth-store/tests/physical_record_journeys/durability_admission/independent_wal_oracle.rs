use std::ops::Range;

use worth_store::physical_runtime::{
    PhysicalDataFrameIdentity, PhysicalDataFrameSubject, PhysicalRedoTargetClaim,
};
use worth_store_physical_format::RecordArtifactFile;

const FRAME_HEADER_BYTES: usize = 116;
const FRAME_FOOTER_BYTES: usize = 32;
const BINDING_DOMAIN: &[u8] = b"store.physical.mutation-attempt-binding.v1";
const REDO_DOMAIN: &[u8] = b"store.physical.wal.canonical-redo.v1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ExpectedAttemptBinding {
    pub(super) key: [u8; 32],
    pub(super) issuance: u64,
    pub(super) expiry: u64,
    pub(super) fingerprint: [u8; 32],
    pub(super) store: [u8; 16],
    pub(super) runtime: u64,
    pub(super) operation: u64,
    pub(super) member: [u8; 32],
    pub(super) lsn_start: u64,
    pub(super) lsn_end: u64,
    pub(super) redo_digest: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct IndependentRedoTargetClaim {
    target: Vec<u8>,
    digest: [u8; 32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum BindingField {
    Domain,
    Key,
    Issuance,
    Expiry,
    Fingerprint,
    Store,
    Runtime,
    Operation,
    Member,
    LsnStart,
    LsnEnd,
    RedoDigest,
    RedoPayload,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum BindingInspectionDenial {
    InvalidFrame,
    Truncated,
    InvalidFieldLength(BindingField),
    DomainMismatch,
    FieldMismatch(BindingField),
    TrailingBytes,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct InspectedAttemptBinding {
    pub(super) value: ExpectedAttemptBinding,
    pub(super) spans: Vec<(BindingField, Range<usize>)>,
}

impl InspectedAttemptBinding {
    pub(super) fn span(&self, field: BindingField) -> Range<usize> {
        self.spans
            .iter()
            .find(|(candidate, _)| *candidate == field)
            .map(|(_, span)| span.clone())
            .expect("every binding field has one span")
    }
}

pub(super) fn independent_frame_payload(bytes: &[u8]) -> Result<&[u8], BindingInspectionDenial> {
    if bytes.len() < FRAME_HEADER_BYTES + FRAME_FOOTER_BYTES
        || bytes.get(..8) != Some(b"WORTHWAL".as_slice())
        || read_u16_at(bytes, 8)? != 1
        || usize::from(read_u16_at(bytes, 10)?) != FRAME_HEADER_BYTES
    {
        return Err(BindingInspectionDenial::InvalidFrame);
    }
    let payload_bytes = usize::try_from(read_u64_at(bytes, 44)?)
        .map_err(|_| BindingInspectionDenial::InvalidFrame)?;
    let payload_end = FRAME_HEADER_BYTES
        .checked_add(payload_bytes)
        .ok_or(BindingInspectionDenial::InvalidFrame)?;
    if payload_end
        .checked_add(FRAME_FOOTER_BYTES)
        .filter(|end| *end == bytes.len())
        .is_none()
    {
        return Err(BindingInspectionDenial::InvalidFrame);
    }
    Ok(&bytes[FRAME_HEADER_BYTES..payload_end])
}

pub(super) fn independent_target_claim(
    claim: PhysicalRedoTargetClaim,
) -> IndependentRedoTargetClaim {
    IndependentRedoTargetClaim {
        target: independent_target_identity(claim.target()),
        digest: claim.resulting_payload_digest(),
    }
}

pub(super) fn independent_canonical_redo(
    records: &[&[u8]],
    lsn_start: u64,
    targets: &[Vec<IndependentRedoTargetClaim>],
) -> Vec<u8> {
    assert!(
        !records.is_empty(),
        "the independent redo oracle requires a nonempty fixture"
    );
    assert_eq!(
        records.len(),
        targets.len(),
        "every redo record requires its exact target claims"
    );
    let mut encoded = Vec::new();
    write_field(&mut encoded, REDO_DOMAIN);
    encoded.extend_from_slice(&(records.len() as u64).to_le_bytes());
    for (ordinal, record) in records.iter().enumerate() {
        encoded.extend_from_slice(&(ordinal as u32).to_le_bytes());
        encoded.extend_from_slice(&(lsn_start + ordinal as u64).to_le_bytes());
        encoded.extend_from_slice(&(targets[ordinal].len() as u64).to_le_bytes());
        for claim in &targets[ordinal] {
            write_field(&mut encoded, &claim.target);
            encoded.extend_from_slice(&claim.digest);
        }
        write_field(&mut encoded, record);
    }
    encoded
}

fn independent_target_identity(target: PhysicalDataFrameIdentity) -> Vec<u8> {
    let coordinate = target.coordinate();
    let mut bytes = Vec::with_capacity(96);
    match target.subject() {
        PhysicalDataFrameSubject::InlinePage(page) => {
            bytes.push(target.kind() as u8);
            bytes.extend_from_slice(&page.segment_id().get().to_le_bytes());
            bytes.extend_from_slice(&page.page_id().get().to_le_bytes());
            bytes.extend_from_slice(&page.generation().get().to_le_bytes());
        }
        PhysicalDataFrameSubject::ExtentChunk(chunk) => {
            bytes.push(target.kind() as u8);
            let record = chunk.record();
            bytes.extend_from_slice(&record.allocation_epoch());
            bytes.extend_from_slice(&record.ordinal().to_le_bytes());
            bytes.extend_from_slice(&chunk.extent_cell().extent_id().get().to_le_bytes());
            bytes.extend_from_slice(&chunk.extent_cell().generation().get().to_le_bytes());
            bytes.extend_from_slice(&chunk.logical_bytes().to_le_bytes());
            bytes.extend_from_slice(&chunk.logical_offset().to_le_bytes());
            bytes.extend_from_slice(&chunk.ordinal().to_le_bytes());
        }
    }
    match coordinate.artifact() {
        RecordArtifactFile::Segment {
            segment,
            generation,
        } => {
            bytes.push(5);
            bytes.extend_from_slice(&segment.to_le_bytes());
            bytes.extend_from_slice(&generation.to_le_bytes());
        }
        RecordArtifactFile::Extent { extent, generation } => {
            bytes.push(8);
            bytes.extend_from_slice(&extent.to_le_bytes());
            bytes.extend_from_slice(&generation.to_le_bytes());
        }
        _ => panic!("redo targets are data artifacts only"),
    }
    bytes.extend_from_slice(&coordinate.offset().to_le_bytes());
    bytes.extend_from_slice(&coordinate.length().to_le_bytes());
    bytes
}

pub(super) fn split_member_payload(
    payload: &[u8],
) -> Result<(&[u8], &[u8]), BindingInspectionDenial> {
    let mut cursor = ByteCursor::new(payload);
    let binding = cursor.field(BindingField::Domain)?;
    let redo = cursor.field(BindingField::RedoDigest)?;
    cursor.finish()?;
    Ok((binding, redo))
}

pub(super) fn inspect_member_payload(
    payload: &[u8],
    expected: &ExpectedAttemptBinding,
    expected_redo: &[u8],
) -> Result<InspectedAttemptBinding, BindingInspectionDenial> {
    let (binding, redo) = split_member_payload(payload)?;
    if redo != expected_redo {
        return Err(BindingInspectionDenial::FieldMismatch(
            BindingField::RedoPayload,
        ));
    }
    inspect_attempt_binding(binding, expected)
}

pub(super) fn inspect_attempt_binding(
    bytes: &[u8],
    expected: &ExpectedAttemptBinding,
) -> Result<InspectedAttemptBinding, BindingInspectionDenial> {
    let mut cursor = ByteCursor::new(bytes);
    let domain = cursor.fixed_field(BindingField::Domain, BINDING_DOMAIN.len())?;
    if domain != BINDING_DOMAIN {
        return Err(BindingInspectionDenial::DomainMismatch);
    }
    let key = array(cursor.fixed_field(BindingField::Key, 32)?);
    let issuance = cursor.u64(BindingField::Issuance)?;
    let expiry = cursor.u64(BindingField::Expiry)?;
    let fingerprint = array(cursor.fixed_field(BindingField::Fingerprint, 32)?);
    let store = array(cursor.fixed_field(BindingField::Store, 16)?);
    let runtime = cursor.u64(BindingField::Runtime)?;
    let operation = cursor.u64(BindingField::Operation)?;
    let member = array(cursor.fixed_field(BindingField::Member, 32)?);
    let lsn_start = cursor.u64(BindingField::LsnStart)?;
    let lsn_end = cursor.u64(BindingField::LsnEnd)?;
    let redo_digest = array(cursor.fixed_field(BindingField::RedoDigest, 32)?);
    cursor.finish()?;
    let value = ExpectedAttemptBinding {
        key,
        issuance,
        expiry,
        fingerprint,
        store,
        runtime,
        operation,
        member,
        lsn_start,
        lsn_end,
        redo_digest,
    };
    require_expected_fields(&value, expected)?;
    Ok(InspectedAttemptBinding {
        value,
        spans: cursor.spans,
    })
}

fn require_expected_fields(
    value: &ExpectedAttemptBinding,
    expected: &ExpectedAttemptBinding,
) -> Result<(), BindingInspectionDenial> {
    for (field, equal) in [
        (BindingField::Key, value.key == expected.key),
        (BindingField::Issuance, value.issuance == expected.issuance),
        (BindingField::Expiry, value.expiry == expected.expiry),
        (
            BindingField::Fingerprint,
            value.fingerprint == expected.fingerprint,
        ),
        (BindingField::Store, value.store == expected.store),
        (BindingField::Runtime, value.runtime == expected.runtime),
        (
            BindingField::Operation,
            value.operation == expected.operation,
        ),
        (BindingField::Member, value.member == expected.member),
        (
            BindingField::LsnStart,
            value.lsn_start == expected.lsn_start,
        ),
        (BindingField::LsnEnd, value.lsn_end == expected.lsn_end),
        (
            BindingField::RedoDigest,
            value.redo_digest == expected.redo_digest,
        ),
    ] {
        if !equal {
            return Err(BindingInspectionDenial::FieldMismatch(field));
        }
    }
    Ok(())
}

struct ByteCursor<'a> {
    bytes: &'a [u8],
    offset: usize,
    spans: Vec<(BindingField, Range<usize>)>,
}

impl<'a> ByteCursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            offset: 0,
            spans: Vec::new(),
        }
    }

    fn field(&mut self, field: BindingField) -> Result<&'a [u8], BindingInspectionDenial> {
        let length = usize::try_from(self.read_u64()?)
            .map_err(|_| BindingInspectionDenial::InvalidFieldLength(field))?;
        self.take(field, length)
    }

    fn fixed_field(
        &mut self,
        field: BindingField,
        expected: usize,
    ) -> Result<&'a [u8], BindingInspectionDenial> {
        let length = usize::try_from(self.read_u64()?)
            .map_err(|_| BindingInspectionDenial::InvalidFieldLength(field))?;
        if length != expected {
            return Err(BindingInspectionDenial::InvalidFieldLength(field));
        }
        self.take(field, length)
    }

    fn u64(&mut self, field: BindingField) -> Result<u64, BindingInspectionDenial> {
        let bytes = self.take(field, 8)?;
        Ok(u64::from_le_bytes(bytes.try_into().expect("fixed u64")))
    }

    fn read_u64(&mut self) -> Result<u64, BindingInspectionDenial> {
        let bytes = self
            .bytes
            .get(self.offset..self.offset + 8)
            .ok_or(BindingInspectionDenial::Truncated)?;
        self.offset += 8;
        Ok(u64::from_le_bytes(bytes.try_into().expect("fixed u64")))
    }

    fn take(
        &mut self,
        field: BindingField,
        length: usize,
    ) -> Result<&'a [u8], BindingInspectionDenial> {
        let end = self
            .offset
            .checked_add(length)
            .ok_or(BindingInspectionDenial::Truncated)?;
        let value = self
            .bytes
            .get(self.offset..end)
            .ok_or(BindingInspectionDenial::Truncated)?;
        self.spans.push((field, self.offset..end));
        self.offset = end;
        Ok(value)
    }

    fn finish(&self) -> Result<(), BindingInspectionDenial> {
        (self.offset == self.bytes.len())
            .then_some(())
            .ok_or(BindingInspectionDenial::TrailingBytes)
    }
}

fn read_u16_at(bytes: &[u8], offset: usize) -> Result<u16, BindingInspectionDenial> {
    let value = bytes
        .get(offset..offset + 2)
        .ok_or(BindingInspectionDenial::Truncated)?;
    Ok(u16::from_le_bytes(value.try_into().expect("fixed u16")))
}

fn read_u64_at(bytes: &[u8], offset: usize) -> Result<u64, BindingInspectionDenial> {
    let value = bytes
        .get(offset..offset + 8)
        .ok_or(BindingInspectionDenial::Truncated)?;
    Ok(u64::from_le_bytes(value.try_into().expect("fixed u64")))
}

fn array<const N: usize>(bytes: &[u8]) -> [u8; N] {
    bytes.try_into().expect("fixed field length was checked")
}

fn write_field(target: &mut Vec<u8>, field: &[u8]) {
    target.extend_from_slice(&(field.len() as u64).to_le_bytes());
    target.extend_from_slice(field);
}
