use std::collections::BTreeSet;

use super::super::{
    offline_protocol::OfflineObservation,
    protocol::{
        BoundedResidencyCancellationObservation, BoundedResidencyReadObservation,
        BoundedResidencySiegeObservation,
    },
    world::{
        BoundedResidencySiegeWorld, DIRTY_FRAMES, PINNED_FRAMES, PIN_LEASES, RECORD_BYTES,
        RESIDENT_BYTES,
    },
};

mod dirty_close;

use dirty_close::verify_dirty_and_close;

pub(super) fn verify_residency(
    world: &BoundedResidencySiegeWorld,
    child: BoundedResidencySiegeObservation,
) -> Result<(), String> {
    let reads = child.reads;
    if child.resident_budget() != RESIDENT_BYTES {
        return Err("Courtroom C child used a foreign resident-byte budget".into());
    }
    verify_read_pressure(reads, world.admitted_byte_limit())?;
    let pins = child.pins;
    if pins.cold_work != 1
        || pins.hot_work != 0
        || pins.refault_work != 1
        || pins.peak_pinned_frames == 0
        || pins.peak_pinned_frames > PINNED_FRAMES
        || pins.peak_pin_leases != PIN_LEASES
    {
        return Err("Courtroom C pin pressure did not prove its bounded handoff".into());
    }
    verify_cancellation(child.cancellation)?;
    let close = child.close;
    if close.peak_resident_bytes > RESIDENT_BYTES
        || close.peak_admitted_bytes > world.admitted_byte_limit()
        || close.peak_dirty_frames > DIRTY_FRAMES
        || close.peak_resident_bytes < reads.peak_resident_bytes
        || close.peak_admitted_bytes < reads.peak_admitted_bytes
    {
        return Err("Courtroom C final memory peaks escaped or omitted the joined siege".into());
    }
    verify_dirty_and_close(child)
}

fn verify_read_pressure(
    reads: BoundedResidencyReadObservation,
    admitted_byte_limit: u64,
) -> Result<(), String> {
    let active_declared_work = reads
        .metadata_read_work_declared
        .checked_add(reads.range_read_work_declared);
    let active_dispatched_work = reads
        .metadata_read_work_dispatched
        .checked_add(reads.range_read_work_dispatched);
    let terminal_work = reads
        .metadata_read_work_terminal
        .checked_add(reads.range_read_work_terminal);
    let expected_metadata_effects = reads
        .metadata_read_work_terminal
        .checked_add(reads.range_read_work_terminal);
    let operation_span = reads
        .last_operation
        .checked_sub(reads.first_operation)
        .and_then(|difference| difference.checked_add(1));
    if reads.peak_resident_bytes > RESIDENT_BYTES
        || reads.peak_admitted_bytes > admitted_byte_limit
        || reads.cold_effects == 0
        || reads.cold_work != reads.cold_metadata_effects
        || reads.cold_effects >= reads.cold_work
        || reads.hot_effects != 0
        || reads.hot_metadata_effects != 0
        || reads.hot_work != 0
        || reads.refault_effects == 0
        || reads.refault_work != reads.refault_metadata_effects
        || reads.refault_effects >= reads.refault_work
        || reads.physical_work == 0
        || reads.first_operation == 0
        || operation_span != Some(reads.physical_work)
        || !reads.runtime_bound
        || active_declared_work != Some(0)
        || active_dispatched_work != Some(0)
        || terminal_work != Some(reads.physical_work)
        || reads.positioned_read_effects != reads.range_read_work_terminal
        || Some(reads.metadata_read_effects) != expected_metadata_effects
        || reads.range_read_work_declared != 0
        || reads.range_read_work_dispatched != 0
        || reads.range_read_work_terminal != reads.faults
        || reads.source_loads != reads.faults
        || reads.faults == 0
        || reads.hits == 0
        || reads.evictions == 0
    {
        return Err(
            "Courtroom C read pressure did not reconcile residency faults, canonical work, \
             runtime identities, and media effects"
                .into(),
        );
    }
    Ok(())
}

pub(super) fn verify_artifacts(
    world: &BoundedResidencySiegeWorld,
    child: BoundedResidencySiegeObservation,
    offline: &OfflineObservation,
) -> Result<(), String> {
    if offline.artifacts().is_empty() || offline.recovery_obligations() != 0 {
        return Err("Courtroom C offline truth omitted artifacts or retained recovery work".into());
    }
    let mut paths = BTreeSet::new();
    let total = offline
        .artifacts()
        .iter()
        .try_fold(0_u64, |total, artifact| {
            if !paths.insert(artifact.path()) {
                return Err("Courtroom C offline manifest duplicated an artifact path".to_owned());
            }
            Ok(total.saturating_add(artifact.byte_length()))
        })?;
    if total != child.directory_bytes()
        || total < RESIDENT_BYTES.saturating_mul(8)
        || world.expected_payload_bytes() < RESIDENT_BYTES.saturating_mul(8)
    {
        return Err("Courtroom C durable world was not materially larger than residency".into());
    }
    Ok(())
}

fn verify_cancellation(
    cancellation: BoundedResidencyCancellationObservation,
) -> Result<(), String> {
    let operation_span = cancellation
        .last_operation
        .checked_sub(cancellation.first_operation)
        .and_then(|difference| difference.checked_add(1));
    if cancellation.physical_work == 0
        || cancellation.first_operation == 0
        || operation_span != Some(cancellation.physical_work)
        || !cancellation.runtime_bound
        || cancellation.unread_payload_bytes != RECORD_BYTES as u64
        || cancellation.open_media_effects == 0
        || cancellation.cancellation_media_effects != 0
    {
        return Err(
            "Courtroom C cancellation open bypassed C.5.1, lost range, or cancellation caused media"
                .into(),
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{verify_cancellation, verify_read_pressure};
    use crate::courtroom_campaign::bounded_residency_siege::{
        protocol::{BoundedResidencyCancellationObservation, BoundedResidencyReadObservation},
        world::{RECORD_BYTES, RESIDENT_BYTES},
    };

    #[test]
    fn cancellation_oracle_accepts_cold_open_and_effect_free_cancellation() {
        assert!(
            verify_cancellation(BoundedResidencyCancellationObservation {
                physical_work: 1,
                first_operation: 7,
                last_operation: 7,
                runtime_bound: true,
                unread_payload_bytes: RECORD_BYTES as u64,
                open_media_effects: 1,
                cancellation_media_effects: 0,
            })
            .is_ok()
        );
    }

    #[test]
    fn cancellation_oracle_rejects_each_causal_bypass() {
        let accepted = BoundedResidencyCancellationObservation {
            physical_work: 1,
            first_operation: 7,
            last_operation: 7,
            runtime_bound: true,
            unread_payload_bytes: RECORD_BYTES as u64,
            open_media_effects: 1,
            cancellation_media_effects: 0,
        };
        for hostile in [
            BoundedResidencyCancellationObservation {
                physical_work: 0,
                ..accepted
            },
            BoundedResidencyCancellationObservation {
                first_operation: 0,
                ..accepted
            },
            BoundedResidencyCancellationObservation {
                last_operation: 6,
                ..accepted
            },
            BoundedResidencyCancellationObservation {
                physical_work: 2,
                ..accepted
            },
            BoundedResidencyCancellationObservation {
                runtime_bound: false,
                ..accepted
            },
            BoundedResidencyCancellationObservation {
                unread_payload_bytes: 0,
                ..accepted
            },
            BoundedResidencyCancellationObservation {
                open_media_effects: 0,
                ..accepted
            },
            BoundedResidencyCancellationObservation {
                cancellation_media_effects: 1,
                ..accepted
            },
        ] {
            assert!(verify_cancellation(hostile).is_err(), "{hostile:?}");
        }
    }

    #[test]
    fn read_pressure_oracle_accepts_exact_causal_reconciliation() {
        assert!(verify_read_pressure(accepted_reads(), 128_000).is_ok());
    }

    #[test]
    fn read_pressure_oracle_rejects_every_one_field_bypass() {
        let accepted = accepted_reads();
        for hostile in [
            BoundedResidencyReadObservation {
                cold_effects: 0,
                ..accepted
            },
            BoundedResidencyReadObservation {
                cold_work: 5,
                ..accepted
            },
            BoundedResidencyReadObservation {
                cold_metadata_effects: 2,
                ..accepted
            },
            BoundedResidencyReadObservation {
                hot_effects: 1,
                ..accepted
            },
            BoundedResidencyReadObservation {
                hot_metadata_effects: 1,
                ..accepted
            },
            BoundedResidencyReadObservation {
                hot_work: 1,
                ..accepted
            },
            BoundedResidencyReadObservation {
                refault_effects: 0,
                ..accepted
            },
            BoundedResidencyReadObservation {
                refault_work: 5,
                ..accepted
            },
            BoundedResidencyReadObservation {
                refault_metadata_effects: 2,
                ..accepted
            },
            BoundedResidencyReadObservation {
                physical_work: 9,
                ..accepted
            },
            BoundedResidencyReadObservation {
                positioned_read_effects: 3,
                ..accepted
            },
            BoundedResidencyReadObservation {
                metadata_read_effects: 5,
                ..accepted
            },
            BoundedResidencyReadObservation {
                metadata_read_work_declared: 1,
                ..accepted
            },
            BoundedResidencyReadObservation {
                metadata_read_work_dispatched: 1,
                ..accepted
            },
            BoundedResidencyReadObservation {
                metadata_read_work_terminal: 5,
                ..accepted
            },
            BoundedResidencyReadObservation {
                range_read_work_declared: 1,
                ..accepted
            },
            BoundedResidencyReadObservation {
                range_read_work_dispatched: 1,
                ..accepted
            },
            BoundedResidencyReadObservation {
                range_read_work_terminal: 3,
                ..accepted
            },
            BoundedResidencyReadObservation {
                first_operation: 0,
                ..accepted
            },
            BoundedResidencyReadObservation {
                last_operation: 19,
                ..accepted
            },
            BoundedResidencyReadObservation {
                runtime_bound: false,
                ..accepted
            },
            BoundedResidencyReadObservation {
                peak_resident_bytes: RESIDENT_BYTES + 1,
                ..accepted
            },
            BoundedResidencyReadObservation {
                peak_admitted_bytes: 128_001,
                ..accepted
            },
            BoundedResidencyReadObservation {
                faults: 3,
                ..accepted
            },
            BoundedResidencyReadObservation {
                source_loads: 3,
                ..accepted
            },
            BoundedResidencyReadObservation {
                hits: 0,
                ..accepted
            },
            BoundedResidencyReadObservation {
                evictions: 0,
                ..accepted
            },
        ] {
            assert!(
                verify_read_pressure(hostile, 128_000).is_err(),
                "{hostile:?}"
            );
        }
    }

    fn accepted_reads() -> BoundedResidencyReadObservation {
        BoundedResidencyReadObservation {
            cold_effects: 2,
            hot_effects: 0,
            refault_effects: 2,
            cold_metadata_effects: 3,
            hot_metadata_effects: 0,
            refault_metadata_effects: 3,
            cold_work: 3,
            hot_work: 0,
            refault_work: 3,
            physical_work: 10,
            positioned_read_effects: 4,
            metadata_read_effects: 10,
            metadata_read_work_declared: 0,
            metadata_read_work_dispatched: 0,
            metadata_read_work_terminal: 6,
            range_read_work_declared: 0,
            range_read_work_dispatched: 0,
            range_read_work_terminal: 4,
            first_operation: 11,
            last_operation: 20,
            runtime_bound: true,
            peak_resident_bytes: RESIDENT_BYTES,
            peak_admitted_bytes: 128_000,
            faults: 4,
            source_loads: 4,
            hits: 1,
            evictions: 1,
        }
    }
}
