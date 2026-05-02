use crate::runtime::{
    ForgeQueryAspectMutationOperation, ForgeQueryContinuityMutationIntent,
    ForgeQueryExistingTruthTargetBinding, ForgeQueryMutationFamily, ForgeQueryMutationMetadata,
    ForgeQueryNamingMutationIntent, ForgeQueryRuntimeBackendPosture,
    ForgeQueryRuntimeSupportProfile, ForgeQuerySymbolicAspectResolutionEvidence,
    ForgeQuerySymbolicTargetReference, ForgeQueryVerifiedExistingTruthAssertion,
    ForgeQueryWriteCommand,
};

pub(crate) type BatchCommandSummary = (
    ForgeQueryMutationFamily,
    Option<String>,
    Option<String>,
    Option<ForgeQueryExistingTruthTargetBinding>,
    Option<ForgeQueryVerifiedExistingTruthAssertion>,
    Option<ForgeQuerySymbolicTargetReference>,
    Option<ForgeQueryNamingMutationIntent>,
    Option<ForgeQueryContinuityMutationIntent>,
    Vec<ForgeQueryAspectMutationOperation>,
    Option<String>,
    Vec<ForgeQuerySymbolicAspectResolutionEvidence>,
    ForgeQueryMutationMetadata,
);

pub(crate) fn should_use_backend_atomic_batch(
    support_profile: &ForgeQueryRuntimeSupportProfile,
    commands: &[ForgeQueryWriteCommand],
) -> bool {
    support_profile.posture() == ForgeQueryRuntimeBackendPosture::Primary
        && commands.len() > 1
        && !commands.iter().any(|command| {
            command.symbolic_target_reference().is_some()
                || !command.symbolic_aspect_references().is_empty()
        })
}
