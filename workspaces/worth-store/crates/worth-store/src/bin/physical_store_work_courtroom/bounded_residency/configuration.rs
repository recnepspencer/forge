use std::path::Path;

use worth_store::physical_runtime::{
    AdmittedPhysicalRecordFormat, PhysicalOperationAllocationScope, PhysicalRecordResidencyPolicy,
    PhysicalRecordResidencyPolicyOutcome, PhysicalSpeculativeWorkKind,
};

use super::super::configuration::BOUNDED_RESIDENCY_CONFIGURATION_SCHEMA;

#[derive(Debug, Clone, Copy)]
pub(super) struct BoundedResidencyConfiguration {
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

impl BoundedResidencyConfiguration {
    pub(super) fn read(path: &Path) -> Result<Self, String> {
        let encoded = std::fs::read_to_string(path)
            .map_err(|error| format!("cannot read C.6 pressure configuration: {error}"))?;
        let mut lines = encoded.lines();
        if lines.next() != Some(BOUNDED_RESIDENCY_CONFIGURATION_SCHEMA) {
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
        if [
            self.resident_bytes,
            self.metadata_bytes,
            self.operation_bytes,
        ]
        .contains(&0)
            || self.pinned_frames == 0
            || self.pin_leases == 0
            || self.dirty_frames == 0
            || self.frame_entries == 0
        {
            return Err("C.6 residency policy dimensions must be nonzero".to_owned());
        }
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

    pub(super) fn policy(
        self,
        format: AdmittedPhysicalRecordFormat,
    ) -> PhysicalRecordResidencyPolicyOutcome {
        use PhysicalOperationAllocationScope as Scope;
        use PhysicalSpeculativeWorkKind as Speculation;

        let total_bytes = self
            .resident_bytes
            .checked_mul(2)
            .and_then(|bytes| bytes.checked_add(self.metadata_bytes))
            .and_then(|bytes| bytes.checked_add(self.operation_bytes))
            .expect("validated C.6 policy total fits u64");
        PhysicalRecordResidencyPolicy::builder()
            .total_bytes(nonzero_bytes(total_bytes))
            .resident_bytes(nonzero_bytes(self.resident_bytes))
            .metadata_bytes(nonzero_bytes(self.metadata_bytes))
            .frame_entries(nonzero_count(self.frame_entries))
            .pinned_frames(nonzero_count(self.pinned_frames))
            .pin_leases(nonzero_count(self.pin_leases))
            .dirty_frames(nonzero_count(self.dirty_frames))
            .dirty_replacement_bytes(nonzero_bytes(self.resident_bytes))
            .operation_bytes(nonzero_bytes(self.operation_bytes))
            .scope_bytes(Scope::ForegroundRead, nonzero_bytes(self.operation_bytes))
            .scope_bytes(Scope::ForegroundWrite, nonzero_bytes(self.operation_bytes))
            .scope_bytes(Scope::Recovery, nonzero_bytes(self.operation_bytes))
            .scope_bytes(Scope::Scrub, nonzero_bytes(self.operation_bytes))
            .scope_bytes(Scope::Maintenance, nonzero_bytes(self.operation_bytes))
            .scope_bytes(Scope::Verification, nonzero_bytes(self.operation_bytes))
            .scope_bytes(Scope::Blob, nonzero_bytes(self.operation_bytes))
            .speculative_frames(Speculation::Prefetch, nonzero_count(self.pinned_frames))
            .speculative_frames(Speculation::ReadAhead, nonzero_count(self.pinned_frames))
            .speculative_frames(Speculation::WriteBehind, nonzero_count(self.dirty_frames))
            .admit(format)
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

fn nonzero_bytes(value: u64) -> std::num::NonZeroU64 {
    std::num::NonZeroU64::new(value).expect("validated C.6 byte dimensions are nonzero")
}

fn nonzero_count(value: u32) -> std::num::NonZeroU32 {
    std::num::NonZeroU32::new(value).expect("validated C.6 count dimensions are nonzero")
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
    use super::BoundedResidencyConfiguration;

    #[test]
    fn configuration_requires_a_materially_oversized_store() {
        let temporary = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(
            temporary.path(),
            concat!(
                "worth.store.physical-work-courtroom.bounded-residency.configuration.v1\n",
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
        assert!(BoundedResidencyConfiguration::read(temporary.path()).is_err());
    }
}
