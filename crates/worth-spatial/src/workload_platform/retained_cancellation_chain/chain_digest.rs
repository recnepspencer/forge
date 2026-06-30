use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::{
    chain_checkpoint::RetainedCancellationCheckpoint,
    chain_counters::RetainedCancellationChainCounters,
};

pub(crate) fn retained_cancellation_chain_digest(
    compiled_product_identity_digest: &str,
    equivalence_policy_identity_digest: &str,
    workload_identity: &str,
    retained_basis_identity: &str,
    projection_consumed_identity: &str,
    checkpoints: &[RetainedCancellationCheckpoint],
    counters: RetainedCancellationChainCounters,
) -> String {
    let checkpoint_identities = checkpoints
        .iter()
        .map(|checkpoint| checkpoint.checkpoint_identity().to_string())
        .collect::<Vec<_>>()
        .join("|");
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "retained-cancellation-chain-workload".to_string(),
            format!("compiled-product:{compiled_product_identity_digest}"),
            format!("equivalence-policy:{equivalence_policy_identity_digest}"),
            format!("workload:{workload_identity}"),
            format!("retained_basis:{retained_basis_identity}"),
            format!("projection_consumed:{projection_consumed_identity}"),
            format!("checkpoints:{checkpoint_identities}"),
            format!("checkpoint_count:{}", counters.checkpoint_count()),
            format!("replayed_count:{}", counters.replayed_checkpoint_count()),
            format!(
                "trigger_replay_count:{}",
                counters.trigger_local_replay_count()
            ),
            format!("retained_artifacts:{}", counters.retained_artifact_count()),
            format!(
                "projection_consumed_facts:{}",
                counters.projection_consumed_fact_count()
            ),
        ],
    )
}
