use crate::{
    backend::records::StoreState,
    bulk::BulkPlanKind,
    publication::observe_durable_recovery_publication,
    wal::{DurableMutationId, WalRecord, WalRecordPayload},
};

use super::{super::DurableMutationIdentity, model::RecoverySourceSet};

pub(crate) fn build_recovery_source_set(
    state: &StoreState,
    durable_mutation_id: DurableMutationId,
    wal_records: &[&WalRecord],
    backend_report: crate::media::DurableMediaReport,
) -> Result<RecoverySourceSet, crate::failure::StoreError> {
    Ok(RecoverySourceSet {
        durable_mutation_id,
        mutation_identity: mutation_identity_for_wal_records(durable_mutation_id, wal_records),
        observation: observe_durable_recovery_publication(
            state,
            durable_mutation_id,
            wal_records,
            backend_report,
        )?,
    })
}

fn mutation_identity_for_wal_records(
    durable_mutation_id: DurableMutationId,
    wal_records: &[&WalRecord],
) -> DurableMutationIdentity {
    let intent = wal_records.iter().find_map(|record| match &record.payload {
        WalRecordPayload::DurableMutationIntent(intent) => Some(intent),
        _ => None,
    });
    let Some(intent) = intent else {
        return DurableMutationIdentity::GenericOperation {
            operation_name: format!("durable-mutation-{}", durable_mutation_id.0),
        };
    };

    if let Some(identity) =
        parse_bulk_chunk_identity(&intent.runtime_session_id, &intent.operation_name)
    {
        return identity;
    }

    DurableMutationIdentity::GenericOperation {
        operation_name: intent.operation_name.clone(),
    }
}

fn parse_bulk_chunk_identity(
    runtime_session_id: &str,
    operation_name: &str,
) -> Option<DurableMutationIdentity> {
    let (plan_kind, chunk_ordinal) = if let Some(value) = operation_name
        .strip_prefix("bulk-ingest-chunk-")
        .and_then(|value| value.parse::<u64>().ok())
    {
        (BulkPlanKind::Ingest, value)
    } else if let Some(value) = operation_name
        .strip_prefix("bulk-transform-chunk-")
        .and_then(|value| value.parse::<u64>().ok())
    {
        (BulkPlanKind::Transform, value)
    } else {
        return None;
    };

    let mut parts = runtime_session_id.splitn(3, ':');
    let prefix = parts.next()?;
    let program_id = parts.next()?;
    let plan_id = parts.next()?;
    if prefix != "bulk" || program_id.is_empty() || plan_id.is_empty() {
        return None;
    }

    Some(DurableMutationIdentity::BulkChunk {
        plan_kind,
        program_id: program_id.to_string(),
        plan_id: plan_id.to_string(),
        chunk_ordinal,
    })
}
