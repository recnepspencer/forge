use std::num::NonZeroU32;

use super::process_execution::CapturedProcess;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct BoundedResidencyReadObservation {
    pub(super) cold_effects: u64,
    pub(super) hot_effects: u64,
    pub(super) refault_effects: u64,
    pub(super) cold_metadata_effects: u64,
    pub(super) hot_metadata_effects: u64,
    pub(super) refault_metadata_effects: u64,
    pub(super) cold_work: u64,
    pub(super) hot_work: u64,
    pub(super) refault_work: u64,
    pub(super) physical_work: u64,
    pub(super) positioned_read_effects: u64,
    pub(super) metadata_read_effects: u64,
    pub(super) metadata_read_work_declared: u64,
    pub(super) metadata_read_work_dispatched: u64,
    pub(super) metadata_read_work_terminal: u64,
    pub(super) range_read_work_declared: u64,
    pub(super) range_read_work_dispatched: u64,
    pub(super) range_read_work_terminal: u64,
    pub(super) first_operation: u64,
    pub(super) last_operation: u64,
    pub(super) runtime_bound: bool,
    pub(super) peak_resident_bytes: u64,
    pub(super) peak_admitted_bytes: u64,
    pub(super) faults: u64,
    pub(super) source_loads: u64,
    pub(super) hits: u64,
    pub(super) evictions: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct BoundedResidencyPinObservation {
    pub(super) cold_work: u64,
    pub(super) hot_work: u64,
    pub(super) refault_work: u64,
    pub(super) peak_pinned_frames: u32,
    pub(super) peak_pin_leases: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct BoundedResidencyCancellationObservation {
    pub(super) physical_work: u64,
    pub(super) first_operation: u64,
    pub(super) last_operation: u64,
    pub(super) runtime_bound: bool,
    pub(super) unread_payload_bytes: u64,
    pub(super) open_media_effects: u64,
    pub(super) cancellation_media_effects: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct CancelledWritebackTerminalSettlement;

impl CancelledWritebackTerminalSettlement {
    fn parse(effect_fate: &str, recovery: &str, signal: &str) -> Result<Self, String> {
        if effect_fate != "WriteCompleted"
            || recovery != "ContinueSettlement"
            || signal != "ReconciledFromPhysicalTruth"
        {
            return Err(format!(
                "Courtroom C cancelled writeback settlement was fate `{effect_fate}`, \
                 recovery `{recovery}`, signal `{signal}`; expected physical completion, \
                 continued settlement, and reconciliation from physical truth"
            ));
        }
        Ok(Self)
    }

    pub(super) const fn effect_fate_evidence(self) -> &'static str {
        "write-completed"
    }

    pub(super) const fn recovery_evidence(self) -> &'static str {
        "continue-settlement"
    }

    pub(super) const fn signal_evidence(self) -> &'static str {
        "reconciled-from-physical-truth"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct BoundedResidencyDirtyObservation {
    pub(super) work_operation: u64,
    pub(super) source_work_count: u64,
    pub(super) first_source_operation: u64,
    pub(super) last_source_operation: u64,
    pub(super) backend_operation: u64,
    pub(super) settlement: CancelledWritebackTerminalSettlement,
    pub(super) dirty_at_pause: u32,
    pub(super) dirty_after_receipt: u32,
    pub(super) positioned_writes: u64,
    pub(super) candidate_publications: u64,
    pub(super) writebacks: u64,
    pub(super) active_claims_at_pause: u32,
    pub(super) eviction_releases_at_pause: u64,
    pub(super) competing_claim_denied: bool,
    pub(super) cancellation_settlement_continues: bool,
    pub(super) writeback_attempts: u64,
    pub(super) exact_receipts: u64,
    pub(super) retryable_writebacks: u64,
    pub(super) indeterminate_writebacks: u64,
    pub(super) inspection_required_writebacks: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct BoundedResidencyCloseObservation {
    pub(super) inspection_required: bool,
    pub(super) resident_bytes: u64,
    pub(super) pinned_frames: u32,
    pub(super) pin_leases: u32,
    pub(super) dirty_frames: u32,
    pub(super) peak_resident_bytes: u64,
    pub(super) peak_admitted_bytes: u64,
    pub(super) peak_dirty_frames: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct BoundedResidencySiegeObservation {
    process: NonZeroU32,
    store: [u8; 16],
    runtime: u64,
    generation: u64,
    records: u64,
    payload_bytes: u64,
    directory_bytes: u64,
    resident_budget: u64,
    pub(super) reads: BoundedResidencyReadObservation,
    pub(super) pins: BoundedResidencyPinObservation,
    pub(super) cancellation: BoundedResidencyCancellationObservation,
    pub(super) dirty: BoundedResidencyDirtyObservation,
    pub(super) close: BoundedResidencyCloseObservation,
}

impl BoundedResidencySiegeObservation {
    pub(super) const fn process(self) -> NonZeroU32 {
        self.process
    }

    pub(super) const fn store(self) -> [u8; 16] {
        self.store
    }

    pub(super) const fn runtime(self) -> u64 {
        self.runtime
    }

    pub(super) const fn generation(self) -> u64 {
        self.generation
    }

    pub(super) const fn records(self) -> u64 {
        self.records
    }

    pub(super) const fn payload_bytes(self) -> u64 {
        self.payload_bytes
    }

    pub(super) const fn directory_bytes(self) -> u64 {
        self.directory_bytes
    }

    pub(super) const fn resident_budget(self) -> u64 {
        self.resident_budget
    }
}

pub(super) fn parse(process: &CapturedProcess) -> Result<BoundedResidencySiegeObservation, String> {
    if process.stdout().len() != 7 {
        return Err(format!(
            "Courtroom C child emitted {} lines; expected exactly seven",
            process.stdout().len()
        ));
    }
    let world = fields(process.stdout(), "BOUNDED_RESIDENCY_WORLD ", 9)?;
    let reported = nonzero(world[1], "C.6 process")?;
    if reported != process.process() {
        return Err("Courtroom C child reported a foreign process identity".into());
    }
    let runtime = number(world[3], "C.6 runtime")?;
    let generation = number(world[4], "C.6 generation")?;
    if runtime == 0 || generation == 0 {
        return Err("Courtroom C runtime and generation must be nonzero".into());
    }
    Ok(BoundedResidencySiegeObservation {
        process: reported,
        store: fixed_hex(world[2], "C.6 Store identity")?,
        runtime,
        generation,
        records: number(world[5], "C.6 records")?,
        payload_bytes: number(world[6], "C.6 payload bytes")?,
        directory_bytes: number(world[7], "C.6 directory bytes")?,
        resident_budget: number(world[8], "C.6 resident budget")?,
        reads: parse_reads(process.stdout())?,
        pins: parse_pins(process.stdout())?,
        cancellation: parse_cancellation(process.stdout())?,
        dirty: parse_dirty(process.stdout())?,
        close: parse_close(process.stdout())?,
    })
}

fn parse_reads(lines: &[String]) -> Result<BoundedResidencyReadObservation, String> {
    let value = fields(lines, "BOUNDED_RESIDENCY_READS ", 28)?;
    Ok(BoundedResidencyReadObservation {
        cold_effects: number(value[1], "cold read effects")?,
        hot_effects: number(value[2], "hot read effects")?,
        refault_effects: number(value[3], "refault effects")?,
        cold_metadata_effects: number(value[4], "cold metadata effects")?,
        hot_metadata_effects: number(value[5], "hot metadata effects")?,
        refault_metadata_effects: number(value[6], "refault metadata effects")?,
        cold_work: number(value[7], "cold read work")?,
        hot_work: number(value[8], "hot read work")?,
        refault_work: number(value[9], "refault read work")?,
        physical_work: number(value[10], "read physical work")?,
        positioned_read_effects: number(value[11], "positioned read effects")?,
        metadata_read_effects: number(value[12], "metadata read effects")?,
        metadata_read_work_declared: number(value[13], "metadata read work declared")?,
        metadata_read_work_dispatched: number(value[14], "metadata read work dispatched")?,
        metadata_read_work_terminal: number(value[15], "metadata read work terminal")?,
        range_read_work_declared: number(value[16], "range read work declared")?,
        range_read_work_dispatched: number(value[17], "range read work dispatched")?,
        range_read_work_terminal: number(value[18], "range read work terminal")?,
        first_operation: number(value[19], "first read operation")?,
        last_operation: number(value[20], "last read operation")?,
        runtime_bound: boolean(value[21], "read work runtime binding")?,
        peak_resident_bytes: number(value[22], "peak resident bytes")?,
        peak_admitted_bytes: number(value[23], "peak admitted bytes")?,
        faults: number(value[24], "faults")?,
        source_loads: number(value[25], "source loads")?,
        hits: number(value[26], "hits")?,
        evictions: number(value[27], "evictions")?,
    })
}

fn parse_pins(lines: &[String]) -> Result<BoundedResidencyPinObservation, String> {
    let value = fields(lines, "BOUNDED_RESIDENCY_PINS ", 11)?;
    if value[6] != "PinLeases" || value[7] != "ForegroundRead" {
        return Err(format!(
            "Courtroom C over-pin pressure was dimension `{}` scope `{}`",
            value[6], value[7],
        ));
    }
    let requested: u64 = number(value[8], "over-pin requested leases")?;
    let admitted: u64 = number(value[9], "over-pin admitted leases")?;
    let limit: u64 = number(value[10], "over-pin lease limit")?;
    if requested != 1 || admitted != limit {
        return Err(format!(
            "Courtroom C over-pin pressure was requested={requested} admitted={admitted} limit={limit}",
        ));
    }
    Ok(BoundedResidencyPinObservation {
        cold_work: number(value[1], "cold pin work")?,
        hot_work: number(value[2], "hot pin work")?,
        refault_work: number(value[3], "refault pin work")?,
        peak_pinned_frames: number(value[4], "peak pinned frames")?,
        peak_pin_leases: number(value[5], "peak pin leases")?,
    })
}

fn parse_cancellation(lines: &[String]) -> Result<BoundedResidencyCancellationObservation, String> {
    let value = fields(lines, "BOUNDED_RESIDENCY_CANCEL ", 8)?;
    Ok(BoundedResidencyCancellationObservation {
        physical_work: number(value[1], "pre-cancellation physical work")?,
        first_operation: number(value[2], "first pre-cancellation operation")?,
        last_operation: number(value[3], "last pre-cancellation operation")?,
        runtime_bound: boolean(value[4], "pre-cancellation runtime binding")?,
        unread_payload_bytes: number(value[5], "cancelled unread bytes")?,
        open_media_effects: number(value[6], "open media effects")?,
        cancellation_media_effects: number(value[7], "cancellation media effects")?,
    })
}

fn parse_dirty(lines: &[String]) -> Result<BoundedResidencyDirtyObservation, String> {
    let value = fields(lines, "BOUNDED_RESIDENCY_DIRTY ", 23)?;
    Ok(BoundedResidencyDirtyObservation {
        work_operation: number(value[1], "writeback work operation")?,
        source_work_count: number(value[2], "source work count")?,
        first_source_operation: number(value[3], "first source operation")?,
        last_source_operation: number(value[4], "last source operation")?,
        backend_operation: number(value[5], "writeback backend operation")?,
        settlement: CancelledWritebackTerminalSettlement::parse(value[6], value[7], value[8])?,
        dirty_at_pause: number(value[9], "dirty frames at pause")?,
        dirty_after_receipt: number(value[10], "dirty frames after receipt")?,
        positioned_writes: number(value[11], "positioned writes")?,
        candidate_publications: number(value[12], "candidate publications")?,
        writebacks: number(value[13], "writebacks")?,
        active_claims_at_pause: number(value[14], "active writeback claims at pause")?,
        eviction_releases_at_pause: number(value[15], "eviction releases at pause")?,
        competing_claim_denied: boolean(value[16], "competing claim denial")?,
        cancellation_settlement_continues: boolean(
            value[17],
            "writeback cancellation settlement continuation",
        )?,
        writeback_attempts: number(value[18], "writeback attempts")?,
        exact_receipts: number(value[19], "exact writeback receipts")?,
        retryable_writebacks: number(value[20], "retryable writebacks")?,
        indeterminate_writebacks: number(value[21], "indeterminate writebacks")?,
        inspection_required_writebacks: number(value[22], "inspection-required writebacks")?,
    })
}

fn parse_close(lines: &[String]) -> Result<BoundedResidencyCloseObservation, String> {
    let value = fields(lines, "BOUNDED_RESIDENCY_CLOSE ", 9)?;
    Ok(BoundedResidencyCloseObservation {
        inspection_required: boolean(value[1], "close inspection")?,
        resident_bytes: number(value[2], "close resident bytes")?,
        pinned_frames: number(value[3], "close pinned frames")?,
        pin_leases: number(value[4], "close pin leases")?,
        dirty_frames: number(value[5], "close dirty frames")?,
        peak_resident_bytes: number(value[6], "final peak resident bytes")?,
        peak_admitted_bytes: number(value[7], "final peak admitted bytes")?,
        peak_dirty_frames: number(value[8], "final peak dirty frames")?,
    })
}

fn fields<'lines>(
    lines: &'lines [String],
    prefix: &str,
    count: usize,
) -> Result<Vec<&'lines str>, String> {
    let matching = lines
        .iter()
        .filter(|line| line.starts_with(prefix))
        .collect::<Vec<_>>();
    let [line] = matching.as_slice() else {
        return Err(format!(
            "expected one `{prefix}` marker, found {}",
            matching.len()
        ));
    };
    let fields = line.split_whitespace().collect::<Vec<_>>();
    if fields.len() != count {
        return Err(format!("malformed Courtroom C marker `{line}`"));
    }
    Ok(fields)
}

fn nonzero(encoded: &str, label: &str) -> Result<NonZeroU32, String> {
    NonZeroU32::new(number(encoded, label)?).ok_or_else(|| format!("{label} cannot be zero"))
}

fn fixed_hex<const N: usize>(encoded: &str, label: &str) -> Result<[u8; N], String> {
    if encoded.len() != N * 2 || !encoded.is_ascii() {
        return Err(format!(
            "{label} must contain exactly {N} hexadecimal bytes"
        ));
    }
    let mut bytes = [0_u8; N];
    for (index, byte) in bytes.iter_mut().enumerate() {
        let offset = index * 2;
        *byte = u8::from_str_radix(&encoded[offset..offset + 2], 16)
            .map_err(|_| format!("{label} contains non-hexadecimal data"))?;
    }
    Ok(bytes)
}

fn boolean(encoded: &str, label: &str) -> Result<bool, String> {
    match encoded {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(format!("{label} must be `true` or `false`")),
    }
}

fn number<T: std::str::FromStr>(encoded: &str, label: &str) -> Result<T, String> {
    encoded
        .parse()
        .map_err(|_| format!("{label} is not a valid number"))
}

#[cfg(test)]
mod tests;
