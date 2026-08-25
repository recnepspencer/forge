use crate::durability::data::{DurabilityError, RecoveryFailureClass};
use crate::runtime::RelationalRuntime;

pub(super) fn prepare_recovery_lineage_sequence(
    restored: &mut RelationalRuntime,
    envelope: &crate::history::data::CanonicalCommitEnvelope,
) {
    if let Some(first_event_id) = envelope
        .lineage_events()
        .first()
        .map(|event| event.event_id())
    {
        restored.lineage.next_event_id = first_event_id;
    }
    if let Some(first_created_lineage_id) = envelope
        .lineage_events()
        .iter()
        .find(|event| event.kind() == crate::lineage::data::LineageEventKind::Create)
        .and_then(|event| event.targets().first())
    {
        restored.lineage.next_lineage_id = first_created_lineage_id.0;
    }
}

pub(super) fn validate_tail_lineage_allocator_capacity<'a>(
    tail_log: impl IntoIterator<Item = &'a crate::history::data::CanonicalCommitEnvelope>,
) -> Result<(), DurabilityError> {
    let envelopes = tail_log.into_iter().collect::<Vec<_>>();
    if envelopes
        .iter()
        .flat_map(|commit| commit.lineage_events())
        .any(|event| event.event_id() >= u64::MAX - 1)
    {
        return Err(DurabilityError::new(
            RecoveryFailureClass::CorruptSegment,
            "recovery tail lineage event id exhausted the allocator",
        ));
    }
    if envelopes
        .iter()
        .flat_map(|commit| commit.lineage_events())
        .filter(|event| event.kind() == crate::lineage::data::LineageEventKind::Create)
        .flat_map(|event| event.targets())
        .any(|lineage_id| lineage_id.0 >= u64::MAX - 1)
    {
        return Err(DurabilityError::new(
            RecoveryFailureClass::CorruptSegment,
            "recovery tail lineage id exhausted the allocator",
        ));
    }
    Ok(())
}

pub(super) fn refresh_checkpoint_counters(
    restored: &mut RelationalRuntime,
) -> Result<(), DurabilityError> {
    let next_commit_id = checked_checkpoint_successor(
        restored
            .history
            .commit_envelopes
            .keys()
            .map(|id| id.0)
            .max(),
        "commit id",
    )?;
    let next_version_id = checked_checkpoint_successor(
        restored
            .history
            .commit_envelopes
            .values()
            .map(|envelope| envelope.commit.version_id.0)
            .max(),
        "version id",
    )?;
    let next_lineage_id = checked_checkpoint_successor(
        restored
            .lineage
            .nodes
            .keys()
            .map(|lineage_id| lineage_id.0)
            .max(),
        "lineage id",
    )?;
    let next_event_id = checked_checkpoint_successor(
        restored
            .lineage
            .events()
            .map(|event| event.event_id())
            .max(),
        "lineage event id",
    )?;
    restored
        .history
        .install_recovered_sequence_floor(next_commit_id, next_version_id);
    restored.lineage.next_lineage_id = next_lineage_id;
    restored.lineage.next_event_id = next_event_id;
    Ok(())
}

fn checked_checkpoint_successor(
    maximum: Option<u64>,
    counter_name: &str,
) -> Result<u64, DurabilityError> {
    match maximum {
        Some(maximum) if maximum >= u64::MAX - 1 => Err(DurabilityError::new(
            RecoveryFailureClass::CorruptCheckpoint,
            format!("recovered checkpoint {counter_name} has no safe next transition"),
        )),
        Some(maximum) => Ok(maximum + 1),
        None => Ok(1),
    }
}
