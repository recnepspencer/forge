use std::path::Path;

#[path = "record_configuration.rs"]
mod record_configuration_impl;

pub(super) use record_configuration_impl::record_configuration;

const CONFIGURATION_SCHEMA: &str = "worth.store.c5_1.physical-work-courtroom.configuration.v1";
pub(super) const BOUNDED_RESIDENCY_CONFIGURATION_SCHEMA: &str =
    "worth.store.physical-work-courtroom.bounded-residency.configuration.v3";

pub(super) struct CourtroomConfiguration {
    payload_bytes: usize,
}

pub(super) enum ReopenConfiguration {
    Standard,
    BoundedResidency(super::bounded_residency::configuration::BoundedResidencyConfiguration),
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

impl ReopenConfiguration {
    pub(super) fn read(path: &Path) -> Result<Self, String> {
        let encoded = std::fs::read_to_string(path)
            .map_err(|error| format!("cannot read courtroom configuration: {error}"))?;
        match encoded.lines().next() {
            Some(CONFIGURATION_SCHEMA) => {
                CourtroomConfiguration::read(path)?;
                Ok(Self::Standard)
            }
            Some(BOUNDED_RESIDENCY_CONFIGURATION_SCHEMA) => {
                super::bounded_residency::configuration::BoundedResidencyConfiguration::read(path)
                    .map(Self::BoundedResidency)
            }
            _ => Err("unsupported courtroom configuration schema".to_owned()),
        }
    }
}
