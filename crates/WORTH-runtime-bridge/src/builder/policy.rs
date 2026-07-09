use crate::policy::{
    BridgeArtifactPolicyBaseline, BridgeDiagnosticsPolicyBaseline, BridgeExecutionPolicyBaseline,
    BridgeRuntimePolicy,
};

pub(crate) fn replace_runtime_policy(
    _existing: BridgeRuntimePolicy,
    replacement: BridgeRuntimePolicy,
) -> BridgeRuntimePolicy {
    replacement
}

pub(crate) fn replace_execution_policy_baseline(
    policy: BridgeRuntimePolicy,
    execution: BridgeExecutionPolicyBaseline,
) -> BridgeRuntimePolicy {
    policy.with_execution(execution)
}

pub(crate) fn replace_diagnostics_policy_baseline(
    policy: BridgeRuntimePolicy,
    diagnostics: BridgeDiagnosticsPolicyBaseline,
) -> BridgeRuntimePolicy {
    policy.with_diagnostics(diagnostics)
}

pub(crate) fn replace_artifact_policy_baseline(
    policy: BridgeRuntimePolicy,
    artifacts: BridgeArtifactPolicyBaseline,
) -> BridgeRuntimePolicy {
    policy.with_artifacts(artifacts)
}

pub(crate) fn finalize_runtime_policy(policy: BridgeRuntimePolicy) -> BridgeRuntimePolicy {
    policy
}
