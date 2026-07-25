use std::collections::BTreeSet;

use super::super::{
    offline_protocol::OfflineObservation,
    protocol::{C6CancellationObservation, C6SiegeObservation},
    world::{C6SiegeWorld, DIRTY_FRAMES, PINNED_FRAMES, PIN_LEASES, RECORD_BYTES, RESIDENT_BYTES},
};

pub(super) fn verify_residency(
    world: &C6SiegeWorld,
    child: C6SiegeObservation,
) -> Result<(), String> {
    let reads = child.reads;
    if child.resident_budget() != RESIDENT_BYTES
        || reads.peak_resident_bytes > RESIDENT_BYTES
        || reads.peak_admitted_bytes > world.admitted_byte_limit()
        || reads.cold_effects == 0
        || reads.hot_effects != 0
        || reads.refault_effects == 0
        || reads.physical_work == 0
        || reads.faults == 0
        || reads.hits == 0
        || reads.evictions == 0
    {
        return Err("Courtroom C read pressure escaped or bypassed canonical residency".into());
    }
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

pub(super) fn verify_artifacts(
    world: &C6SiegeWorld,
    child: C6SiegeObservation,
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

fn verify_cancellation(cancellation: C6CancellationObservation) -> Result<(), String> {
    let operation_span = cancellation
        .last_operation
        .checked_sub(cancellation.first_operation)
        .and_then(|difference| difference.checked_add(1));
    if cancellation.physical_work == 0
        || cancellation.first_operation == 0
        || operation_span != Some(cancellation.physical_work)
        || !cancellation.handoff_bound
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

fn verify_dirty_and_close(child: C6SiegeObservation) -> Result<(), String> {
    let dirty = child.dirty;
    if dirty.work_operation == 0
        || dirty.source_work_count != 1
        || dirty.first_source_operation == 0
        || dirty.first_source_operation != dirty.last_source_operation
        || dirty.work_operation == dirty.first_source_operation
        || dirty.backend_operation == 0
        || dirty.dirty_at_pause != 1
        || dirty.dirty_after_receipt != 0
        || dirty.positioned_writes != 1
        || dirty.candidate_publications != 1
        || dirty.writebacks != 1
    {
        return Err("Courtroom C dirty work did not remain dirty through exact receipt".into());
    }
    let close = child.close;
    if close.inspection_required
        || close.resident_bytes != 0
        || close.pinned_frames != 0
        || close.pin_leases != 0
        || close.dirty_frames != 0
    {
        return Err("Courtroom C close retained residency or inspection posture".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::verify_cancellation;
    use crate::courtroom_campaign::c6_inheritance_siege::{
        protocol::C6CancellationObservation, world::RECORD_BYTES,
    };

    #[test]
    fn cancellation_oracle_accepts_cold_open_and_effect_free_cancellation() {
        assert!(verify_cancellation(C6CancellationObservation {
            physical_work: 1,
            first_operation: 7,
            last_operation: 7,
            handoff_bound: true,
            unread_payload_bytes: RECORD_BYTES as u64,
            open_media_effects: 1,
            cancellation_media_effects: 0,
        })
        .is_ok());
    }

    #[test]
    fn cancellation_oracle_rejects_each_causal_bypass() {
        let accepted = C6CancellationObservation {
            physical_work: 1,
            first_operation: 7,
            last_operation: 7,
            handoff_bound: true,
            unread_payload_bytes: RECORD_BYTES as u64,
            open_media_effects: 1,
            cancellation_media_effects: 0,
        };
        for hostile in [
            C6CancellationObservation {
                physical_work: 0,
                ..accepted
            },
            C6CancellationObservation {
                first_operation: 0,
                ..accepted
            },
            C6CancellationObservation {
                last_operation: 6,
                ..accepted
            },
            C6CancellationObservation {
                physical_work: 2,
                ..accepted
            },
            C6CancellationObservation {
                handoff_bound: false,
                ..accepted
            },
            C6CancellationObservation {
                unread_payload_bytes: 0,
                ..accepted
            },
            C6CancellationObservation {
                open_media_effects: 0,
                ..accepted
            },
            C6CancellationObservation {
                cancellation_media_effects: 1,
                ..accepted
            },
        ] {
            assert!(verify_cancellation(hostile).is_err(), "{hostile:?}");
        }
    }
}
