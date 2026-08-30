use worth_query_installation::facade::{
    WorthQueryPortablePackageRecord, WorthQueryPortablePackageRecordFamily as Family,
    WorthQueryPortablePackageRecordView,
};

use crate::binary_input::BinaryInput;
use crate::binary_output::BinaryOutput;
use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};
use crate::limits::WorthQueryPackageArchiveLimits;

use super::decode_budget::RecordDecodeAttempt;
pub use super::decode_budget::WorthQueryPackageArchiveDecodeWork;
use super::encoding_budget::{RecordEncodingWork, RecordPayloadEncodingWork};
use super::protocol::{
    family_tag, RECORD_FRAME_HEADER_BYTES, WORTH_QUERY_PACKAGE_ARCHIVE_RECORD_PROTOCOL_VERSION,
};

mod header;

use header::{
    decode_exact_record_frame, decode_next_record_frame, validate_canonical_index,
    validate_record_frame_length, DecodedRecordFrame,
};

/// One structurally decoded portable package record carrying no Query authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryUntrustedPortablePackageRecordFrame {
    canonical_index: u32,
    record: WorthQueryPortablePackageRecord,
}

impl WorthQueryUntrustedPortablePackageRecordFrame {
    pub const fn canonical_index(&self) -> u32 {
        self.canonical_index
    }

    pub const fn family(&self) -> Family {
        self.record.family()
    }

    pub const fn record(&self) -> &WorthQueryPortablePackageRecord {
        &self.record
    }

    pub fn into_record(self) -> WorthQueryPortablePackageRecord {
        self.record
    }

    pub fn into_parts(self) -> (u32, WorthQueryPortablePackageRecord) {
        (self.canonical_index, self.record)
    }
}

/// Stateful hostile-input decoder carrying aggregate record-stream budgets.
pub struct WorthQueryPackageArchiveRecordDecoder {
    limits: WorthQueryPackageArchiveLimits,
    work: WorthQueryPackageArchiveDecodeWork,
}

impl WorthQueryPackageArchiveRecordDecoder {
    pub fn new(limits: WorthQueryPackageArchiveLimits) -> Self {
        Self {
            limits: limits.narrowed(),
            work: WorthQueryPackageArchiveDecodeWork::default(),
        }
    }

    pub const fn work(&self) -> WorthQueryPackageArchiveDecodeWork {
        self.work
    }

    pub fn decode_frame(
        &mut self,
        bytes: &[u8],
    ) -> Result<WorthQueryUntrustedPortablePackageRecordFrame, Denial> {
        let frame = decode_exact_record_frame(bytes, self.limits)?;
        self.decode_parsed_frame(frame)
    }

    pub(crate) fn decode_next_frame<'a>(
        &mut self,
        input: &mut BinaryInput<'a>,
    ) -> Result<WorthQueryUntrustedPortablePackageRecordFrame, Denial> {
        let frame = decode_next_record_frame(input, self.limits)?;
        self.decode_parsed_frame(frame)
    }

    fn decode_parsed_frame(
        &mut self,
        frame: DecodedRecordFrame<'_>,
    ) -> Result<WorthQueryUntrustedPortablePackageRecordFrame, Denial> {
        let payload_bytes = u64::try_from(frame.payload().len())
            .map_err(|_| Denial::new(Kind::InvalidRecordLength))?;
        let mut attempt = RecordDecodeAttempt::begin(self.work, payload_bytes, self.limits)?;
        let record = if super::package_root::is_package_root_family(frame.family()) {
            super::package_root::decode_package_root_payload(frame.family(), frame.payload())?
        } else if frame.family() == Family::DomainOperation {
            let mut input = BinaryInput::new(frame.payload());
            let record = super::domain_operation::decode_payload(&mut input, &mut attempt)?;
            if !input.is_finished() {
                return Err(Denial::new(Kind::TrailingBytes));
            }
            record
        } else if frame.family() == Family::ArtifactContract {
            let mut input = BinaryInput::new(frame.payload());
            let record = super::artifact_contract::decode_payload(&mut input, &mut attempt)?;
            if !input.is_finished() {
                return Err(Denial::new(Kind::TrailingBytes));
            }
            record
        } else if frame.family() == Family::ApplicationSchema {
            let mut input = BinaryInput::new(frame.payload());
            let record = super::application_schema::decode_payload(&mut input, &mut attempt)?;
            if !input.is_finished() {
                return Err(Denial::new(Kind::TrailingBytes));
            }
            record
        } else if frame.family() == Family::ConditionalApplicationOperation {
            let mut input = BinaryInput::new(frame.payload());
            let record = super::conditional_application_operation::decode_payload(&mut input)?;
            if !input.is_finished() {
                return Err(Denial::new(Kind::TrailingBytes));
            }
            record
        } else if frame.family() == Family::NativeAspectContract {
            let mut input = BinaryInput::new(frame.payload());
            let record = super::native_aspect_contract::decode_payload(&mut input, &mut attempt)?;
            if !input.is_finished() {
                return Err(Denial::new(Kind::TrailingBytes));
            }
            record
        } else if frame.family() == Family::ApplicationOperationContract {
            let mut input = BinaryInput::new(frame.payload());
            let record =
                super::application_operation_contract::decode_payload(&mut input, &mut attempt)?;
            if !input.is_finished() {
                return Err(Denial::new(Kind::TrailingBytes));
            }
            record
        } else {
            return Err(Denial::new(Kind::UnsupportedRecordFamily));
        };
        self.work = attempt.finish();
        Ok(WorthQueryUntrustedPortablePackageRecordFrame {
            canonical_index: frame.canonical_index(),
            record,
        })
    }
}

/// Encodes one validated-export record through the stable record-family dispatch.
pub fn encode_record_frame(
    view: WorthQueryPortablePackageRecordView<'_>,
    limits: WorthQueryPackageArchiveLimits,
) -> Result<Vec<u8>, Denial> {
    Ok(encode_record_frame_after(view, limits, RecordEncodingWork::default(), u64::MAX)?.0)
}

pub(crate) fn encode_record_frame_after(
    view: WorthQueryPortablePackageRecordView<'_>,
    limits: WorthQueryPackageArchiveLimits,
    prior_work: RecordEncodingWork,
    remaining_archive_bytes: u64,
) -> Result<(Vec<u8>, RecordEncodingWork), Denial> {
    let payload_work = payload_encoding_work(view, limits)?;
    let frame_bytes = RECORD_FRAME_HEADER_BYTES
        .checked_add(payload_work.payload_bytes())
        .ok_or_else(|| Denial::new(Kind::InvalidRecordLength))?;
    if frame_bytes > remaining_archive_bytes {
        return Err(Denial::new(Kind::ArchiveByteBudgetExceeded));
    }
    let mut frame = RecordFrameEncoding::begin(
        view.family(),
        view.canonical_index(),
        payload_work.payload_bytes(),
        limits,
    )?;
    let next_work = prior_work.admit(payload_work, limits)?;
    write_record_payload(view.record(), &mut frame, limits)?;
    Ok((frame.finish()?, next_work))
}

fn payload_encoding_work(
    view: WorthQueryPortablePackageRecordView<'_>,
    limits: WorthQueryPackageArchiveLimits,
) -> Result<RecordPayloadEncodingWork, Denial> {
    match view.record() {
        WorthQueryPortablePackageRecord::DomainOperation(record) => {
            super::domain_operation::payload_encoding_work(record, limits)
        }
        WorthQueryPortablePackageRecord::ArtifactContract(record) => {
            super::artifact_contract::payload_encoding_work(record, limits)
        }
        WorthQueryPortablePackageRecord::ApplicationSchema(record) => {
            super::application_schema::payload_encoding_work(record, limits)
        }
        WorthQueryPortablePackageRecord::ConditionalApplicationOperation(record) => {
            super::conditional_application_operation::payload_encoding_work(record, limits)
        }
        WorthQueryPortablePackageRecord::NativeAspectContract(record) => {
            super::native_aspect_contract::payload_encoding_work(record, limits)
        }
        WorthQueryPortablePackageRecord::ApplicationOperationContract(record) => {
            super::application_operation_contract::payload_encoding_work(record, limits)
        }
        record if super::package_root::is_package_root_family(view.family()) => {
            Ok(RecordPayloadEncodingWork::without_nested_entries(
                super::package_root::package_root_payload_byte_length(record)?,
            ))
        }
        _ => Err(Denial::new(Kind::UnsupportedRecordFamily)),
    }
}

fn write_record_payload(
    record: &WorthQueryPortablePackageRecord,
    frame: &mut RecordFrameEncoding,
    limits: WorthQueryPackageArchiveLimits,
) -> Result<(), Denial> {
    match record {
        WorthQueryPortablePackageRecord::DomainOperation(record) => {
            super::domain_operation::write_payload(record, frame.payload_output())
        }
        WorthQueryPortablePackageRecord::ArtifactContract(record) => {
            super::artifact_contract::write_payload(record, frame.payload_output())
        }
        WorthQueryPortablePackageRecord::ApplicationSchema(record) => {
            super::application_schema::write_payload(record, frame.payload_output(), limits)
        }
        WorthQueryPortablePackageRecord::ConditionalApplicationOperation(record) => {
            super::conditional_application_operation::write_payload(record, frame.payload_output())
        }
        WorthQueryPortablePackageRecord::NativeAspectContract(record) => {
            super::native_aspect_contract::write_payload(record, frame.payload_output())
        }
        WorthQueryPortablePackageRecord::ApplicationOperationContract(record) => {
            super::application_operation_contract::write_payload(record, frame.payload_output())
        }
        record if super::package_root::is_package_root_family(record.family()) => {
            super::package_root::write_package_root_payload(record, frame.payload_output())
        }
        _ => Err(Denial::new(Kind::UnsupportedRecordFamily)),
    }
}

pub(super) struct RecordFrameEncoding {
    output: BinaryOutput,
    expected_frame_bytes: usize,
}

impl RecordFrameEncoding {
    pub(super) fn begin(
        family: Family,
        canonical_index: u32,
        payload_bytes: u64,
        limits: WorthQueryPackageArchiveLimits,
    ) -> Result<Self, Denial> {
        let limits = limits.narrowed();
        validate_canonical_index(canonical_index, limits)?;
        if payload_bytes > limits.maximum_logical_bytes() {
            return Err(Denial::new(Kind::RecordFrameByteBudgetExceeded));
        }
        let payload_length =
            u32::try_from(payload_bytes).map_err(|_| Denial::new(Kind::InvalidRecordLength))?;
        let frame_bytes = RECORD_FRAME_HEADER_BYTES
            .checked_add(payload_bytes)
            .ok_or_else(|| Denial::new(Kind::InvalidRecordLength))?;
        validate_record_frame_length(frame_bytes, limits)?;
        let expected_frame_bytes =
            usize::try_from(frame_bytes).map_err(|_| Denial::new(Kind::InvalidRecordLength))?;
        let mut output = BinaryOutput::with_capacity(expected_frame_bytes);
        output.u16(WORTH_QUERY_PACKAGE_ARCHIVE_RECORD_PROTOCOL_VERSION);
        output.u16(family_tag(family));
        output.u32(canonical_index);
        output.u32(payload_length);
        Ok(Self {
            output,
            expected_frame_bytes,
        })
    }

    pub(super) fn payload_output(&mut self) -> &mut BinaryOutput {
        &mut self.output
    }

    pub(super) fn finish(self) -> Result<Vec<u8>, Denial> {
        let bytes = self.output.into_bytes();
        if bytes.len() != self.expected_frame_bytes {
            return Err(Denial::new(Kind::InvalidRecordLength));
        }
        Ok(bytes)
    }
}
