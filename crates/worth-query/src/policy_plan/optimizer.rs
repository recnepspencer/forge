use crate::policy_narrowing::{
    optimizer_input_from_narrowed_policy_query, NarrowedPolicyQueryArtifact,
    PolicyAwareOptimizerInput,
};

pub(crate) fn lower_policy_aware_optimizer_input(
    artifact: &NarrowedPolicyQueryArtifact,
) -> PolicyAwareOptimizerInput {
    optimizer_input_from_narrowed_policy_query(artifact)
}
