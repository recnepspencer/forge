use std::path::Path;

use worth_store::physical_runtime::{
    AdmittedPhysicalRecordFormat, AdmittedRecordAccessPolicy, AdmittedRecordPlacementPolicy,
    ManifestEntryCapacity, PhysicalRecordAccessPolicy, PhysicalRecordFormatDeclaration,
    PhysicalRecordPlacementPolicy, RecordByteLimit,
};

const CONFIGURATION_SCHEMA: &str = "worth.store.c5_1.physical-work-courtroom.configuration.v1";
pub(super) const BOUNDED_RESIDENCY_CONFIGURATION_SCHEMA: &str =
    "worth.store.physical-work-courtroom.bounded-residency.configuration.v1";

pub(super) struct CourtroomConfiguration {
    payload_bytes: usize,
}

impl CourtroomConfiguration {
    pub(super) fn read(path: &Path) -> Result<Self, String> {
        let encoded = std::fs::read_to_string(path)
            .map_err(|error| format!("cannot read courtroom configuration: {error}"))?;
        let mut lines = encoded.lines();
        if lines.next() != Some(CONFIGURATION_SCHEMA) {
            return Err("unsupported courtroom configuration schema".to_owned());
        }
        let payload_bytes = lines
            .next()
            .and_then(|line| line.strip_prefix("payload-bytes="))
            .ok_or_else(|| "configuration omitted payload byte count".to_owned())?
            .parse::<usize>()
            .map_err(|_| "configuration payload byte count is invalid".to_owned())?;
        if payload_bytes == 0 || payload_bytes > 1024 * 1024 {
            return Err("configuration payload byte count is outside 1..=1048576".to_owned());
        }
        if lines.next().is_some() {
            return Err("configuration contains undeclared fields".to_owned());
        }
        Ok(Self { payload_bytes })
    }

    pub(super) const fn payload_bytes(&self) -> usize {
        self.payload_bytes
    }
}

pub(super) fn record_configuration() -> (
    AdmittedPhysicalRecordFormat,
    AdmittedRecordPlacementPolicy,
    AdmittedRecordAccessPolicy,
) {
    let format = AdmittedPhysicalRecordFormat::admit(
        PhysicalRecordFormatDeclaration::builder()
            .admit()
            .expect("canonical v1 format declaration"),
    );
    let placement = PhysicalRecordPlacementPolicy::builder()
        .manifest_capacity(ManifestEntryCapacity::new(64).expect("nonzero capacity"))
        .extent_threshold(RecordByteLimit::new(8_192).expect("nonzero threshold"))
        .admit(format)
        .expect("courtroom placement is compatible");
    let access = PhysicalRecordAccessPolicy::builder()
        .admit(format)
        .expect("courtroom access is compatible");
    (format, placement, access)
}

pub(super) fn validate_supported(path: &Path) -> Result<(), String> {
    let encoded = std::fs::read_to_string(path)
        .map_err(|error| format!("cannot read courtroom configuration: {error}"))?;
    match encoded.lines().next() {
        Some(CONFIGURATION_SCHEMA) => CourtroomConfiguration::read(path).map(|_| ()),
        Some(BOUNDED_RESIDENCY_CONFIGURATION_SCHEMA) => {
            super::bounded_residency::validate_configuration(path)
        }
        _ => Err("unsupported courtroom configuration schema".to_owned()),
    }
}
