use std::path::Path;

use worth_store::physical_runtime::PhysicalRecordResidencyPolicy;

use super::super::configuration::C6_PRESSURE_CONFIGURATION_SCHEMA;

#[derive(Debug, Clone, Copy)]
pub(super) struct C6PressureConfiguration {
    record_bytes: usize,
    record_count: usize,
    resident_bytes: u64,
    metadata_bytes: u64,
    pinned_frames: u32,
    pin_leases: u32,
    dirty_frames: u32,
    operation_bytes: u64,
    frame_entries: u32,
}

impl C6PressureConfiguration {
    pub(super) fn read(path: &Path) -> Result<Self, String> {
        let encoded = std::fs::read_to_string(path)
            .map_err(|error| format!("cannot read C.6 pressure configuration: {error}"))?;
        let mut lines = encoded.lines();
        if lines.next() != Some(C6_PRESSURE_CONFIGURATION_SCHEMA) {
            return Err("unsupported C.6 pressure configuration schema".to_owned());
        }
        let configuration = Self {
            record_bytes: field(&mut lines, "record-bytes")?,
            record_count: field(&mut lines, "record-count")?,
            resident_bytes: field(&mut lines, "resident-bytes")?,
            metadata_bytes: field(&mut lines, "metadata-bytes")?,
            pinned_frames: field(&mut lines, "pinned-frames")?,
            pin_leases: field(&mut lines, "pin-leases")?,
            dirty_frames: field(&mut lines, "dirty-frames")?,
            operation_bytes: field(&mut lines, "operation-bytes")?,
            frame_entries: field(&mut lines, "frame-entries")?,
        };
        if lines.next().is_some() {
            return Err("C.6 pressure configuration contains undeclared fields".to_owned());
        }
        configuration.validate()?;
        Ok(configuration)
    }

    fn validate(self) -> Result<(), String> {
        if !(1_024..=64 * 1024).contains(&self.record_bytes) {
            return Err("C.6 record bytes are outside 1024..=65536".to_owned());
        }
        if !(16..=4_096).contains(&self.record_count) {
            return Err("C.6 record count is outside 16..=4096".to_owned());
        }
        let oracle_bytes = self.oracle_bytes()?;
        if oracle_bytes < self.resident_bytes.saturating_mul(8) {
            return Err("C.6 Store is not at least eight residency budgets".to_owned());
        }
        if self.pin_leases < 2 || self.pin_leases >= self.pinned_frames {
            return Err("C.6 pin leases must expose a bounded over-pin edge".to_owned());
        }
        self.policy()
            .ok_or_else(|| "C.6 residency policy is internally inconsistent".to_owned())?;
        Ok(())
    }

    pub(super) fn read_oracle(self, path: &Path) -> Result<Box<[u8]>, String> {
        let bytes =
            std::fs::read(path).map_err(|error| format!("cannot read C.6 oracle: {error}"))?;
        if bytes.len() as u64 != self.oracle_bytes()? {
            return Err("C.6 oracle byte length does not match configuration".to_owned());
        }
        Ok(bytes.into_boxed_slice())
    }

    pub(super) fn policy(self) -> Option<PhysicalRecordResidencyPolicy> {
        PhysicalRecordResidencyPolicy::new_with_metadata_budget(
            self.resident_bytes,
            self.metadata_bytes,
            self.pinned_frames,
            self.dirty_frames,
            self.operation_bytes,
            self.frame_entries,
        )?
        .with_pin_lease_limit(self.pin_leases)
    }

    pub(super) const fn record_bytes(self) -> usize {
        self.record_bytes
    }

    pub(super) const fn record_count(self) -> usize {
        self.record_count
    }

    pub(super) const fn resident_bytes(self) -> u64 {
        self.resident_bytes
    }

    pub(super) const fn pin_leases(self) -> u32 {
        self.pin_leases
    }

    pub(super) const fn frame_entries(self) -> u32 {
        self.frame_entries
    }

    fn oracle_bytes(self) -> Result<u64, String> {
        u64::try_from(self.record_bytes)
            .ok()
            .and_then(|bytes| bytes.checked_mul(self.record_count as u64))
            .ok_or_else(|| "C.6 oracle byte length overflowed".to_owned())
    }
}

fn field<Value: std::str::FromStr>(
    lines: &mut std::str::Lines<'_>,
    name: &str,
) -> Result<Value, String> {
    let prefix = format!("{name}=");
    lines
        .next()
        .and_then(|line| line.strip_prefix(&prefix))
        .ok_or_else(|| format!("C.6 configuration omitted `{name}`"))?
        .parse()
        .map_err(|_| format!("C.6 configuration field `{name}` is invalid"))
}

#[cfg(test)]
mod tests {
    use super::C6PressureConfiguration;

    #[test]
    fn configuration_requires_a_materially_oversized_store() {
        let temporary = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(
            temporary.path(),
            concat!(
                "worth.store.c5_1.c6-inheritance-siege.configuration.v1\n",
                "record-bytes=1024\n",
                "record-count=16\n",
                "resident-bytes=65536\n",
                "metadata-bytes=16384\n",
                "pinned-frames=8\n",
                "pin-leases=2\n",
                "dirty-frames=2\n",
                "operation-bytes=1048576\n",
                "frame-entries=8\n",
            ),
        )
        .unwrap();
        assert!(C6PressureConfiguration::read(temporary.path()).is_err());
    }
}
