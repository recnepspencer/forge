use super::BoundedResidencyConfiguration;

const EXPECTED_INLINE_RECORD_BYTES: usize = 3_000;
const EXPECTED_INLINE_RECORDS: usize = 64;
const EXPECTED_EXTENT_RECORD_BYTES: usize = 1_048_576;
const EXPECTED_EXTENT_RECORDS: usize = 109;
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
const EXPECTED_CHECKPOINT_MEMORY_BYTES: u64 = 1_048_576;
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

impl BoundedResidencyConfiguration {
    pub(super) fn validate(self) -> Result<(), String> {
        self.validate_exact_world()?;
        self.validate_scope_and_speculative_ceilings()?;
        self.validate_adversarial_ratios()?;
        self.validate_checkpoint_residency()?;
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
            (
                self.checkpoint_memory_bytes,
                EXPECTED_CHECKPOINT_MEMORY_BYTES,
            ),
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

    fn validate_checkpoint_residency(self) -> Result<(), String> {
        let fixed_residency = self
            .resident_bytes
            .checked_add(self.metadata_bytes)
            .and_then(|bytes| bytes.checked_add(self.dirty_replacement_bytes))
            .ok_or_else(|| "bounded-residency fixed residency overflowed".to_owned())?;
        let checkpoint_total = fixed_residency
            .checked_add(self.checkpoint_memory_bytes)
            .ok_or_else(|| "bounded-residency checkpoint residency overflowed".to_owned())?;
        if self.checkpoint_memory_bytes > self.operation_bytes
            || self.checkpoint_memory_bytes > self.scope_bytes[4]
            || checkpoint_total > self.total_bytes
        {
            return Err(
                "bounded-residency checkpoint memory exceeds its admitted residency envelope"
                    .to_owned(),
            );
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
}
