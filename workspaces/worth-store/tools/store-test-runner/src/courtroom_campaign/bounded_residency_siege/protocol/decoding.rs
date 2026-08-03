mod allocation;
mod cancellation;
mod generation_fencing;
mod work_reconciliation;

use std::num::NonZeroU32;

use super::{
    BoundedCancellationCaseObservation, BoundedCancellationDispatch, BoundedCancellationObligation,
    BoundedCancellationObservation, BoundedCancellationRecovery, BoundedCancellationSeam,
    BoundedCancellationSignal, BoundedCancellationTerminal, BoundedResidencyCloseObservation,
    BoundedResidencyDirtyObservation, BoundedResidencyDuplicateFaultObservation,
    BoundedResidencyPinObservation, BoundedResidencyPinnedEvictionObservation,
    BoundedResidencyProcessAllocationObservation, BoundedResidencyReadObservation,
    BoundedResidencySiegeObservation, BoundedResidencySpeculationObservation,
    BoundedResidencySpeculativeKindObservation,
};
use crate::courtroom_campaign::bounded_residency_siege::process_execution::CapturedProcess;

const FIXED_PROTOCOL_MARKERS: usize = 40;

pub(super) use allocation::parse as parse_allocation;
#[cfg(test)]
pub(super) use work_reconciliation::parse as parse_work_reconciliation;

pub(in crate::courtroom_campaign::bounded_residency_siege) fn parse(
    process: &CapturedProcess,
) -> Result<BoundedResidencySiegeObservation, String> {
    let allocation = parse_allocation(process.stdout())?;
    let work_reconciliation = work_reconciliation::parse(process.stdout())?;
    let expected_lines = usize::try_from(allocation.trace.event_count)
        .ok()
        .and_then(|events| {
            work_reconciliation
                .records
                .len()
                .checked_mul(2)
                .and_then(|record_lines| {
                    record_lines.checked_add(work_reconciliation.signal_bindings.len())
                })
                .and_then(|work_lines| events.checked_add(work_lines))
        })
        .and_then(|events| events.checked_add(FIXED_PROTOCOL_MARKERS))
        .ok_or_else(|| "Courtroom C allocation event count exceeded process output".to_owned())?;
    if process.stdout().len() != expected_lines {
        return Err(format!(
            "Courtroom C child emitted {} lines; expected exactly {expected_lines}",
            process.stdout().len(),
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
        schedule: parse_schedule(process.stdout())?,
        process_allocation: parse_process_allocation(process.stdout(), reported)?,
        reads: parse_reads(process.stdout())?,
        pins: parse_pins(process.stdout())?,
        pinned_eviction: parse_pinned_eviction(process.stdout())?,
        duplicate: parse_duplicate(process.stdout())?,
        cancellation: cancellation::parse(process.stdout())?,
        generation_fencing: generation_fencing::parse(process.stdout())?,
        dirty: parse_dirty(process.stdout())?,
        speculation: parse_speculation(process.stdout())?,
        work_reconciliation,
        allocation,
        close: parse_close(process.stdout())?,
    })
}

fn parse_schedule(
    lines: &[String],
) -> Result<[super::super::schedule::ScheduleDecision; 4], String> {
    let schedule = fields(lines, "BOUNDED_RESIDENCY_SCHEDULE ", 2)?;
    super::super::schedule::parse_executed_trace(schedule[1])
}

pub(super) fn parse_process_allocation(
    lines: &[String],
    expected_process: NonZeroU32,
) -> Result<BoundedResidencyProcessAllocationObservation, String> {
    let value = fields(lines, "BOUNDED_RESIDENCY_PROCESS_ALLOCATION ", 3)?;
    let process = nonzero(value[1], "process-allocation process")?;
    if process != expected_process {
        return Err("Courtroom C process-allocation evidence named a foreign process".into());
    }
    Ok(BoundedResidencyProcessAllocationObservation {
        process,
        largest_successful_request_bytes: number(
            value[2],
            "largest successful process-allocation request",
        )?,
    })
}

pub(super) fn parse_reads(lines: &[String]) -> Result<BoundedResidencyReadObservation, String> {
    let value = fields(lines, "BOUNDED_RESIDENCY_READS ", 36)?;
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
        caller_copy_operations: number(value[28], "caller copy operations")?,
        caller_copied_bytes: number(value[29], "caller copied bytes")?,
        store_copy_operations: number(value[30], "Store copy operations")?,
        store_copied_bytes: number(value[31], "Store copied bytes")?,
        peak_copy_width: number(value[32], "peak copy width")?,
        store_maximum_copy_width: number(value[33], "Store maximum copy width")?,
        streaming_scratch_bytes: number(value[34], "streaming scratch bytes")?,
        largest_record_bytes: number(value[35], "largest record bytes")?,
    })
}

fn parse_duplicate(lines: &[String]) -> Result<BoundedResidencyDuplicateFaultObservation, String> {
    let value = fields(lines, "BOUNDED_RESIDENCY_DUPLICATE ", 12)?;
    Ok(BoundedResidencyDuplicateFaultObservation {
        faults: number(value[1], "duplicate faults")?,
        source_loads: number(value[2], "duplicate source loads")?,
        coalesced_waiters: number(value[3], "duplicate coalesced waiters")?,
        pinned_frames: number(value[4], "duplicate pinned frames")?,
        pin_leases: number(value[5], "duplicate pin leases")?,
        positioned_reads: number(value[6], "duplicate positioned reads")?,
        owner_work: number(value[7], "duplicate owner work")?,
        waiter_work: number(value[8], "duplicate waiter work")?,
        same_frame: boolean(value[9], "duplicate frame identity")?,
        same_prefix: boolean(value[10], "duplicate payload prefix")?,
        waiter_created_work: boolean(value[11], "duplicate waiter-created work")?,
    })
}

fn parse_pins(lines: &[String]) -> Result<BoundedResidencyPinObservation, String> {
    let value = fields(lines, "BOUNDED_RESIDENCY_PINS ", 14)?;
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
    if value[11] != "AfterLeaseRelease" {
        return Err(format!(
            "Courtroom C over-pin retry posture was `{}`",
            value[11]
        ));
    }
    if boolean(value[12], "over-pin effect posture")? {
        return Err("Courtroom C pre-effect over-pin reported a possible media effect".into());
    }
    Ok(BoundedResidencyPinObservation {
        views: number(value[1], "live public views")?,
        unique_frame_identities: number(value[2], "unique pinned frame identities")?,
        zero_copy_events: number(value[3], "public-view copy events")?,
        peak_pinned_frames: number(value[4], "peak pinned frames")?,
        peak_pin_leases: number(value[5], "peak pin leases")?,
        basis_matched: boolean(value[13], "over-pin basis match")?,
    })
}

pub(super) fn parse_pinned_eviction(
    lines: &[String],
) -> Result<BoundedResidencyPinnedEvictionObservation, String> {
    let value = fields(lines, "BOUNDED_RESIDENCY_PINNED_EVICTION ", 7)?;
    Ok(BoundedResidencyPinnedEvictionObservation {
        forced_evictions: number(value[1], "forced evictions under pinned authority")?,
        pinned_frames_before: number(value[2], "pinned frames before forced eviction")?,
        pinned_frames_after: number(value[3], "pinned frames after forced eviction")?,
        pin_leases_before: number(value[4], "pin leases before forced eviction")?,
        pin_leases_after: number(value[5], "pin leases after forced eviction")?,
        bases_preserved: boolean(value[6], "pinned bases preserved")?,
    })
}

pub(in crate::courtroom_campaign::bounded_residency_siege) fn parse_dirty(
    lines: &[String],
) -> Result<BoundedResidencyDirtyObservation, String> {
    let value = fields(lines, "BOUNDED_RESIDENCY_DIRTY ", 41)?;
    Ok(BoundedResidencyDirtyObservation {
        primary_publication: number(value[1], "primary publication")?,
        retry_publication: number(value[2], "retry publication")?,
        primary_candidate_writebacks: number(value[3], "primary candidate writebacks")?,
        retry_candidate_writebacks: number(value[4], "retry candidate writebacks")?,
        primary_candidate_publications: number(value[5], "primary candidate publications")?,
        retry_candidate_publications: number(value[6], "retry candidate publications")?,
        denied_candidate_publications: number(value[7], "denied candidate publications")?,
        primary_last_candidate_operation: number(value[8], "primary candidate operation")?,
        retry_last_candidate_operation: number(value[9], "retry candidate operation")?,
        primary_records: number(value[10], "primary publication records")?,
        retry_records: number(value[11], "retry publication records")?,
        dirty_at_dispatch: number(value[12], "dirty frames at dispatch")?,
        dirty_peak: number(value[13], "peak dirty frames")?,
        dirty_after_denial: number(value[14], "dirty frames after denial")?,
        dirty_after_primary: number(value[15], "dirty frames after primary settlement")?,
        dirty_terminal: number(value[16], "terminal dirty frames")?,
        active_claims_at_dispatch: number(value[17], "active claims at dispatch")?,
        active_writebehind_at_dispatch: number(value[18], "write-behind at dispatch")?,
        peak_writebehind: number(value[19], "peak write-behind")?,
        terminal_writebehind: number(value[20], "terminal write-behind")?,
        pressure_requested: number(value[21], "pressure requested")?,
        pressure_admitted: number(value[22], "pressure admitted")?,
        pressure_limit: number(value[23], "pressure limit")?,
        pressure_basis_exact: boolean(value[24], "pressure basis")?,
        pressure_retry_after_settlement: boolean(value[25], "pressure retry posture")?,
        pressure_effect_free: boolean(value[26], "pressure effect posture")?,
        cleanup_deletions: number(value[27], "cleanup deletions")?,
        cleanup_complete: boolean(value[28], "cleanup completion")?,
        writebehind_attempts: number(value[29], "write-behind attempts")?,
        writebehind_admissions: number(value[30], "write-behind admissions")?,
        writebehind_denials: number(value[31], "write-behind denials")?,
        writebehind_completions: number(value[32], "write-behind completions")?,
        writeback_attempts: number(value[33], "writeback attempts")?,
        exact_receipts: number(value[34], "exact writeback receipts")?,
        retryable_writebacks: number(value[35], "retryable writebacks")?,
        indeterminate_writebacks: number(value[36], "indeterminate writebacks")?,
        inspection_required_writebacks: number(value[37], "inspection-required writebacks")?,
        candidate_publications: number(value[38], "candidate publications")?,
        writebacks: number(value[39], "writebacks")?,
        positioned_writes: number(value[40], "positioned writes")?,
    })
}

pub(super) fn parse_speculation(
    lines: &[String],
) -> Result<BoundedResidencySpeculationObservation, String> {
    Ok(BoundedResidencySpeculationObservation {
        prefetch: parse_speculative_kind(lines, "BOUNDED_RESIDENCY_PREFETCH ", "prefetch")?,
        read_ahead: parse_speculative_kind(lines, "BOUNDED_RESIDENCY_READ_AHEAD ", "read-ahead")?,
        write_behind: parse_speculative_kind(
            lines,
            "BOUNDED_RESIDENCY_WRITE_BEHIND ",
            "write-behind",
        )?,
    })
}

fn parse_speculative_kind(
    lines: &[String],
    marker: &str,
    label: &str,
) -> Result<BoundedResidencySpeculativeKindObservation, String> {
    let value = fields(lines, marker, 14)?;
    Ok(BoundedResidencySpeculativeKindObservation {
        attempts: number(value[1], &format!("{label} attempts"))?,
        admissions: number(value[2], &format!("{label} admissions"))?,
        denials: number(value[3], &format!("{label} denials"))?,
        completions: number(value[4], &format!("{label} completions"))?,
        peak_frames: number(value[5], &format!("{label} peak frames"))?,
        terminal_frames: number(value[6], &format!("{label} terminal frames"))?,
        hits: number(value[7], &format!("{label} hits"))?,
        effectful_misses: number(value[8], &format!("{label} effectful misses"))?,
        hit_signal_requests: number(value[9], &format!("{label} hit Signal requests"))?,
        denial_signal_requests: number(value[10], &format!("{label} denial Signal requests"))?,
        effectful_signal_requests: number(
            value[11],
            &format!("{label} effectful Signal requests"),
        )?,
        signal_family_exact: boolean(value[12], &format!("{label} Signal family"))?,
        foundational_basis_exact: boolean(value[13], &format!("{label} Foundational basis"))?,
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
