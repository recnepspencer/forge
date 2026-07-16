mod ordinary_owner_execution;

#[cfg(test)]
pub(in crate::courtroom::protocol_models::replication_admission) use ordinary_owner_execution::publication_pending_observation;
pub(in crate::courtroom::protocol_models) use ordinary_owner_execution::{
    ordinary_replication_admission_actions, ordinary_replication_admission_traces,
    replay_replication_divergence_guard,
};
