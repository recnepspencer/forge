use std::path::Path;

use worth_store::physical_runtime::{
    AdmittedPhysicalRecordFormat, PhysicalOperationAllocationScope,
    PhysicalRecordResidencyPolicyOutcome, PhysicalSpeculativeWorkKind,
};

use super::super::configuration::BOUNDED_RESIDENCY_CONFIGURATION_SCHEMA;

#[path = "configuration/policy.rs"]
mod policy;

const EXPECTED_INLINE_RECORD_BYTES: usize = 3_000;
const EXPECTED_INLINE_RECORDS: usize = 64;
const EXPECTED_EXTENT_RECORD_BYTES: usize = 1_048_576;
const EXPECTED_EXTENT_RECORDS: usize = 109;
const SERVING_APPEND_RECORDS: usize = 2;
const EXPECTED_TOTAL_BYTES: u64 = 6_979_584;
const EXPECTED_RESIDENT_BYTES: u64 = 65_536;
const EXPECTED_METADATA_BYTES: u64 = 32_768;
const EXPECTED_FRAME_ENTRIES: u32 = 12;
const EXPECTED_RESIDENT_FRAMES: u32 = 8;
const EXPECTED_PINNED_FRAMES: u32 = 4;
const EXPECTED_PIN_LEASES: u32 = 6;
const EXPECTED_DIRTY_FRAMES: u32 = 2;
const EXPECTED_DIRTY_REPLACEMENT_BYTES: u64 = 65_536;
const EXPECTED_OPERATION_BYTES: u64 = 6_815_744;
const EXPECTED_FOREGROUND_READ_SCOPE_BYTES: u64 = 2_097_152;
const EXPECTED_FOREGROUND_WRITE_SCOPE_BYTES: u64 = 6_815_744;
const EXPECTED_RECOVERY_SCOPE_BYTES: u64 = 2_359_296;
const EXPECTED_SCRUB_SCOPE_BYTES: u64 = 1_835_008;
const EXPECTED_MAINTENANCE_SCOPE_BYTES: u64 = 1_572_864;
const EXPECTED_VERIFICATION_SCOPE_BYTES: u64 = 1_048_576;
const EXPECTED_BLOB_SCOPE_BYTES: u64 = 1_310_720;
const EXPECTED_PREFETCH_FRAMES: u32 = 2;
const EXPECTED_READ_AHEAD_FRAMES: u32 = 2;
const EXPECTED_WRITE_BEHIND_FRAMES: u32 = 1;
const FIXED_CONTROL_TERMINAL_EVENTS: usize = 12;
pub(in crate::bounded_residency) const STREAMING_SCRATCH_BYTES: usize = 8 * 1024;

#[derive(Debug, Clone, Copy)]
pub(super) struct BoundedResidencyConfiguration {
    seed: u64,
    inline_record_bytes: usize,
    inline_records: usize,
    extent_record_bytes: usize,
    extent_records: usize,
    total_bytes: u64,
    resident_bytes: u64,
    metadata_bytes: u64,
    frame_entries: u32,
    resident_frames: u32,
    pinned_frames: u32,
    pin_leases: u32,
    dirty_frames: u32,
    dirty_replacement_bytes: u64,
    operation_bytes: u64,
    scope_bytes: [u64; 7],
    speculative_frames: [u32; 3],
}

impl BoundedResidencyConfiguration {
    pub(super) fn read(path: &Path) -> Result<Self, String> {
        let encoded = std::fs::read_to_string(path)
            .map_err(|error| format!("cannot read bounded-residency configuration: {error}"))?;
        let mut lines = encoded.lines();
        if lines.next() != Some(BOUNDED_RESIDENCY_CONFIGURATION_SCHEMA) {
            return Err("unsupported bounded-residency configuration schema".to_owned());
        }
        let configuration = Self {
            seed: field(&mut lines, "seed")?,
            inline_record_bytes: field(&mut lines, "inline-record-bytes")?,
            inline_records: field(&mut lines, "inline-records")?,
            extent_record_bytes: field(&mut lines, "extent-record-bytes")?,
            extent_records: field(&mut lines, "extent-records")?,
            total_bytes: field(&mut lines, "total-bytes")?,
            resident_bytes: field(&mut lines, "resident-bytes")?,
            metadata_bytes: field(&mut lines, "metadata-bytes")?,
            frame_entries: field(&mut lines, "frame-entries")?,
            resident_frames: field(&mut lines, "resident-frames")?,
            pinned_frames: field(&mut lines, "pinned-frames")?,
            pin_leases: field(&mut lines, "pin-leases")?,
            dirty_frames: field(&mut lines, "dirty-frames")?,
            dirty_replacement_bytes: field(&mut lines, "dirty-replacement-bytes")?,
            operation_bytes: field(&mut lines, "operation-bytes")?,
            scope_bytes: [
                field(&mut lines, "scope-foreground-read-bytes")?,
                field(&mut lines, "scope-foreground-write-bytes")?,
                field(&mut lines, "scope-recovery-bytes")?,
                field(&mut lines, "scope-scrub-bytes")?,
                field(&mut lines, "scope-maintenance-bytes")?,
                field(&mut lines, "scope-verification-bytes")?,
                field(&mut lines, "scope-blob-bytes")?,
            ],
            speculative_frames: [
                field(&mut lines, "speculative-prefetch-frames")?,
                field(&mut lines, "speculative-read-ahead-frames")?,
                field(&mut lines, "speculative-write-behind-frames")?,
            ],
        };
        if lines.next().is_some() {
            return Err("bounded-residency configuration contains undeclared fields".to_owned());
        }
        configuration.validate()?;
        Ok(configuration)
    }

    fn validate(self) -> Result<(), String> {
        self.validate_exact_world()?;
        self.validate_scope_and_speculative_ceilings()?;
        self.validate_adversarial_ratios()?;
        self.validate_combined_scope_pressure()
    }

    fn validate_exact_world(self) -> Result<(), String> {
        let expected = [
            (
                self.inline_record_bytes as u64,
                EXPECTED_INLINE_RECORD_BYTES as u64,
            ),
            (self.inline_records as u64, EXPECTED_INLINE_RECORDS as u64),
            (
                self.extent_record_bytes as u64,
                EXPECTED_EXTENT_RECORD_BYTES as u64,
            ),
            (self.extent_records as u64, EXPECTED_EXTENT_RECORDS as u64),
            (self.total_bytes, EXPECTED_TOTAL_BYTES),
            (self.resident_bytes, EXPECTED_RESIDENT_BYTES),
            (self.metadata_bytes, EXPECTED_METADATA_BYTES),
            (
                u64::from(self.frame_entries),
                u64::from(EXPECTED_FRAME_ENTRIES),
            ),
            (
                u64::from(self.resident_frames),
                u64::from(EXPECTED_RESIDENT_FRAMES),
            ),
            (
                u64::from(self.pinned_frames),
                u64::from(EXPECTED_PINNED_FRAMES),
            ),
            (u64::from(self.pin_leases), u64::from(EXPECTED_PIN_LEASES)),
            (
                u64::from(self.dirty_frames),
                u64::from(EXPECTED_DIRTY_FRAMES),
            ),
            (
                self.dirty_replacement_bytes,
                EXPECTED_DIRTY_REPLACEMENT_BYTES,
            ),
            (self.operation_bytes, EXPECTED_OPERATION_BYTES),
        ];
        if self.seed == 0 || expected.iter().any(|(actual, required)| actual != required) {
            return Err(
                "bounded-residency configuration does not declare the exact hostile world"
                    .to_owned(),
            );
        }
        Ok(())
    }

    fn validate_scope_and_speculative_ceilings(self) -> Result<(), String> {
        if self.scope_bytes
            != [
                EXPECTED_FOREGROUND_READ_SCOPE_BYTES,
                EXPECTED_FOREGROUND_WRITE_SCOPE_BYTES,
                EXPECTED_RECOVERY_SCOPE_BYTES,
                EXPECTED_SCRUB_SCOPE_BYTES,
                EXPECTED_MAINTENANCE_SCOPE_BYTES,
                EXPECTED_VERIFICATION_SCOPE_BYTES,
                EXPECTED_BLOB_SCOPE_BYTES,
            ]
            || self.speculative_frames
                != [
                    EXPECTED_PREFETCH_FRAMES,
                    EXPECTED_READ_AHEAD_FRAMES,
                    EXPECTED_WRITE_BEHIND_FRAMES,
                ]
        {
            return Err("bounded-residency scope or speculative ceilings drifted".to_owned());
        }
        Ok(())
    }

    fn validate_adversarial_ratios(self) -> Result<(), String> {
        let payload = self.payload_bytes()?;
        if payload < self.resident_bytes.saturating_mul(32)
            || payload < self.total_bytes.saturating_mul(16)
        {
            return Err("bounded-residency payload is below its adversarial ratios".to_owned());
        }
        Ok(())
    }

    fn validate_combined_scope_pressure(self) -> Result<(), String> {
        let scope_sum = self
            .scope_bytes
            .iter()
            .try_fold(0_u64, |sum, bytes| sum.checked_add(*bytes))
            .ok_or_else(|| "bounded-residency scope sum overflowed".to_owned())?;
        if scope_sum <= self.operation_bytes {
            return Err(
                "bounded-residency scope sum must exceed the global operation envelope".to_owned(),
            );
        }
        Ok(())
    }

    pub(super) fn serving_policy(
        self,
        format: AdmittedPhysicalRecordFormat,
    ) -> PhysicalRecordResidencyPolicyOutcome {
        policy::admit_serving(self, format)
    }

    pub(super) fn producer_policy(
        self,
        format: AdmittedPhysicalRecordFormat,
    ) -> PhysicalRecordResidencyPolicyOutcome {
        policy::admit_producer(self, format)
    }

    pub(super) const fn seed(self) -> u64 {
        self.seed
    }

    pub(super) const fn record_count(self) -> usize {
        self.inline_records + self.extent_records
    }

    pub(super) const fn producer_record_count(self) -> usize {
        self.record_count() - SERVING_APPEND_RECORDS
    }

    pub(super) const fn serving_append_ordinals(self) -> [usize; SERVING_APPEND_RECORDS] {
        [self.record_count() - 2, self.record_count() - 1]
    }

    pub(super) const fn first_extent_ordinal(self) -> usize {
        self.inline_records
    }

    pub(super) const fn record_bytes(self, ordinal: usize) -> Option<usize> {
        if ordinal < self.inline_records {
            Some(self.inline_record_bytes)
        } else if ordinal < self.record_count() {
            Some(self.extent_record_bytes)
        } else {
            None
        }
    }

    pub(super) fn payload_bytes(self) -> Result<u64, String> {
        let inline = (self.inline_record_bytes as u64)
            .checked_mul(self.inline_records as u64)
            .ok_or_else(|| "bounded-residency inline payload overflowed".to_owned())?;
        let extent = (self.extent_record_bytes as u64)
            .checked_mul(self.extent_records as u64)
            .ok_or_else(|| "bounded-residency extent payload overflowed".to_owned())?;
        inline
            .checked_add(extent)
            .ok_or_else(|| "bounded-residency payload overflowed".to_owned())
    }

    pub(super) fn producer_payload_bytes(self) -> Result<u64, String> {
        let reserved = (self.extent_record_bytes as u64)
            .checked_mul(SERVING_APPEND_RECORDS as u64)
            .ok_or_else(|| "bounded-residency serving append payload overflowed".to_owned())?;
        self.payload_bytes()?
            .checked_sub(reserved)
            .ok_or_else(|| "bounded-residency producer payload underflowed".to_owned())
    }

    pub(super) fn causal_evidence_capacity(self) -> Result<usize, String> {
        let transfer_work = (0..self.record_count()).try_fold(0_usize, |total, ordinal| {
            let bytes = self
                .record_bytes(ordinal)
                .ok_or_else(|| "bounded-residency record inventory drifted".to_owned())?;
            let transfers = bytes
                .checked_add(STREAMING_SCRATCH_BYTES - 1)
                .and_then(|rounded| rounded.checked_div(STREAMING_SCRATCH_BYTES))
                .ok_or_else(|| "bounded-residency transfer count overflowed".to_owned())?;
            total
                .checked_add(transfers)
                .ok_or_else(|| "bounded-residency transfer inventory overflowed".to_owned())
        })?;
        let discovery_work = self
            .record_count()
            .checked_mul(2)
            .ok_or_else(|| "bounded-residency discovery work overflowed".to_owned())?;
        let streaming_work = transfer_work
            .checked_mul(2)
            .ok_or_else(|| "bounded-residency streaming work overflowed".to_owned())?;
        let pin_work = (self.pin_leases as usize)
            .checked_mul(2)
            .ok_or_else(|| "bounded-residency pin work overflowed".to_owned())?;
        discovery_work
            .checked_add(streaming_work)
            .and_then(|capacity| capacity.checked_add(pin_work))
            .and_then(|capacity| capacity.checked_add(FIXED_CONTROL_TERMINAL_EVENTS))
            .ok_or_else(|| "bounded-residency causal evidence capacity overflowed".to_owned())
    }

    pub(super) const fn total_bytes(self) -> u64 {
        self.total_bytes
    }

    pub(super) const fn resident_bytes(self) -> u64 {
        self.resident_bytes
    }

    pub(super) const fn metadata_bytes(self) -> u64 {
        self.metadata_bytes
    }

    pub(super) const fn frame_entries(self) -> u32 {
        self.frame_entries
    }

    pub(super) const fn resident_frames(self) -> u32 {
        self.resident_frames
    }

    pub(super) const fn pinned_frames(self) -> u32 {
        self.pinned_frames
    }

    pub(super) const fn pin_leases(self) -> u32 {
        self.pin_leases
    }

    pub(super) const fn dirty_frames(self) -> u32 {
        self.dirty_frames
    }

    pub(super) const fn dirty_replacement_bytes(self) -> u64 {
        self.dirty_replacement_bytes
    }

    pub(super) const fn operation_bytes(self) -> u64 {
        self.operation_bytes
    }

    pub(super) const fn scope_bytes(self, scope: PhysicalOperationAllocationScope) -> u64 {
        match scope {
            PhysicalOperationAllocationScope::ForegroundRead => self.scope_bytes[0],
            PhysicalOperationAllocationScope::ForegroundWrite => self.scope_bytes[1],
            PhysicalOperationAllocationScope::Recovery => self.scope_bytes[2],
            PhysicalOperationAllocationScope::Scrub => self.scope_bytes[3],
            PhysicalOperationAllocationScope::Maintenance => self.scope_bytes[4],
            PhysicalOperationAllocationScope::Verification => self.scope_bytes[5],
            PhysicalOperationAllocationScope::Blob => self.scope_bytes[6],
        }
    }

    pub(super) const fn speculative_frames(self, kind: PhysicalSpeculativeWorkKind) -> u32 {
        match kind {
            PhysicalSpeculativeWorkKind::Prefetch => self.speculative_frames[0],
            PhysicalSpeculativeWorkKind::ReadAhead => self.speculative_frames[1],
            PhysicalSpeculativeWorkKind::WriteBehind => self.speculative_frames[2],
        }
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
        .ok_or_else(|| format!("bounded-residency configuration omitted `{name}`"))?
        .parse()
        .map_err(|_| format!("bounded-residency configuration field `{name}` is invalid"))
}

#[cfg(test)]
#[path = "configuration/tests.rs"]
mod tests;
