use crate::{
    backend::records::StoreState,
    wal::{DurableMutationId, WalRecordPayload},
};
use std::collections::BTreeSet;

use super::DurableRecoveryPlan;

pub(crate) fn build_recovery_plan(state: &StoreState) -> DurableRecoveryPlan {
    let closed_durable_mutation_ids = state
        .wal_records
        .values()
        .filter_map(|record| match &record.payload {
            WalRecordPayload::RecoveryDecision(_) => Some(record.durable_mutation_id),
            _ => None,
        })
        .collect::<BTreeSet<DurableMutationId>>();
    let pending_durable_mutation_ids = state
        .wal_records
        .values()
        .map(|record| record.durable_mutation_id)
        .filter(|durable_mutation_id| !closed_durable_mutation_ids.contains(durable_mutation_id))
        .collect::<BTreeSet<DurableMutationId>>()
        .into_iter()
        .collect();
    DurableRecoveryPlan {
        pending_durable_mutation_ids,
    }
}
