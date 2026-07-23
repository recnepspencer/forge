use super::{
    AdmittedPhysicalRecordFormat, PhysicalRecordFormatDeclaration, RecordByteLimit,
    RecordCountLimit,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalRecordAccessPolicy {
    transfer: RecordByteLimit,
    scratch: RecordByteLimit,
    scan: RecordCountLimit,
    append_records: RecordCountLimit,
    append_bytes: RecordByteLimit,
}

#[derive(Debug, Default)]
pub struct PhysicalRecordAccessPolicyBuilder {
    transfer: Option<RecordByteLimit>,
    scratch: Option<RecordByteLimit>,
    scan: Option<RecordCountLimit>,
    append_records: Option<RecordCountLimit>,
    append_bytes: Option<RecordByteLimit>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalRecordAccessPolicyDenial {
    TransferSmallerThanPage,
    ScratchSmallerThanPage,
    ScanMetadataExceedsScratch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedRecordAccessPolicy {
    policy: PhysicalRecordAccessPolicy,
    format: PhysicalRecordFormatDeclaration,
}

impl PhysicalRecordAccessPolicy {
    pub fn builder() -> PhysicalRecordAccessPolicyBuilder {
        PhysicalRecordAccessPolicyBuilder::default()
    }
}

impl PhysicalRecordAccessPolicyBuilder {
    pub fn transfer_limit(mut self, bytes: RecordByteLimit) -> Self {
        self.transfer = Some(bytes);
        self
    }

    pub fn scratch_limit(mut self, bytes: RecordByteLimit) -> Self {
        self.scratch = Some(bytes);
        self
    }

    pub fn scan_record_limit(mut self, records: RecordCountLimit) -> Self {
        self.scan = Some(records);
        self
    }

    pub fn append_record_limit(mut self, records: RecordCountLimit) -> Self {
        self.append_records = Some(records);
        self
    }

    pub fn append_byte_limit(mut self, bytes: RecordByteLimit) -> Self {
        self.append_bytes = Some(bytes);
        self
    }

    pub fn admit(
        self,
        format: AdmittedPhysicalRecordFormat,
    ) -> Result<AdmittedRecordAccessPolicy, PhysicalRecordAccessPolicyDenial> {
        let page = format.declaration().page_size().bytes();
        let policy = PhysicalRecordAccessPolicy {
            transfer: self.transfer.unwrap_or(RecordByteLimit(page)),
            scratch: self.scratch.unwrap_or(RecordByteLimit(page)),
            scan: self.scan.unwrap_or(RecordCountLimit(256)),
            append_records: self.append_records.unwrap_or(RecordCountLimit(4_096)),
            append_bytes: self
                .append_bytes
                .unwrap_or(RecordByteLimit(page.saturating_mul(1_024))),
        };
        if policy.transfer.get() < page {
            return Err(PhysicalRecordAccessPolicyDenial::TransferSmallerThanPage);
        }
        if policy.scratch.get() < page {
            return Err(PhysicalRecordAccessPolicyDenial::ScratchSmallerThanPage);
        }
        let metadata_bytes = u64::from(policy.scan.get())
            .saturating_mul(std::mem::size_of::<super::ScannedPhysicalRecord>() as u64);
        if metadata_bytes > u64::from(policy.scratch.get()) {
            return Err(PhysicalRecordAccessPolicyDenial::ScanMetadataExceedsScratch);
        }
        Ok(AdmittedRecordAccessPolicy {
            policy,
            format: format.declaration(),
        })
    }
}

impl AdmittedRecordAccessPolicy {
    pub const fn transfer_limit(self) -> RecordByteLimit {
        self.policy.transfer
    }

    pub const fn scratch_limit(self) -> RecordByteLimit {
        self.policy.scratch
    }

    pub const fn scan_limit(self) -> RecordCountLimit {
        self.policy.scan
    }

    pub const fn append_record_limit(self) -> RecordCountLimit {
        self.policy.append_records
    }

    pub const fn append_byte_limit(self) -> RecordByteLimit {
        self.policy.append_bytes
    }

    pub(super) fn admits(self, format: AdmittedPhysicalRecordFormat) -> bool {
        self.format == format.declaration()
    }
}
