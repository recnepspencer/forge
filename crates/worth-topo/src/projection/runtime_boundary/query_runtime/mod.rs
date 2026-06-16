mod adapters;
mod contracts;
pub(crate) mod identity_authority;
pub use identity_authority::{
    topology_query_runtime_phase_eight_compile_fail_targets,
    topology_query_runtime_phase_eight_golden_paths,
    topology_query_runtime_phase_nine_compile_fail_targets,
    topology_query_runtime_phase_nine_golden_paths,
    PHASE_EIGHT_EXCLUDED_FOLKLORE_PATHS, PHASE_EIGHT_FORBIDDEN_SUBSTITUTION_PATTERNS,
    PHASE_EIGHT_QUERY_RUNTIME_SCAN_PATHS,
    PHASE_NINE_FORBIDDEN_SUBSTITUTION_PATTERNS, PHASE_NINE_QUERY_RUNTIME_SCAN_PATHS,
    TOPOLOGY_QUERY_RUNTIME_PHASE_EIGHT_COMPILE_FAIL_TARGET_COUNT,
    TOPOLOGY_QUERY_RUNTIME_PHASE_EIGHT_GOLDEN_PATH_COUNT,
    TOPOLOGY_QUERY_RUNTIME_PHASE_NINE_COMPILE_FAIL_TARGET_COUNT,
    TOPOLOGY_QUERY_RUNTIME_PHASE_NINE_GOLDEN_PATH_COUNT,
};
mod mutation_support;
mod mutation_support_rows;
mod operator_bindings;
mod operator_post_write;
mod read_support;
mod runtime_closeout;
mod runtime_posture;

#[cfg(test)]
mod tests;

use forge_query::facade::{ForgeQueryRuntime, ForgeQueryWorkspace};

pub use contracts::{TopologyRuntimeAdapters, TopologyRuntimeFailure, TopologyRuntimeSupport};
pub use mutation_support::{
    TopologyQueryMutationFamilySupportStatus, TopologyQueryMutationLane,
    TopologyQueryMutationLaneExecutionShape, TopologyQueryMutationLaneSupportStatus,
    TopologyRuntimeMutationFamilySupportRow, TopologyRuntimeMutationLaneSupportRow,
};
pub(crate) use operator_bindings::TopologyQueryBindingIndex;
pub(crate) use operator_post_write::TopologyPostWriteQueryArtifact;
pub use read_support::{TopologyQueryReadFamilySupportStatus, TopologyRuntimeReadFamilySupportRow};
pub use runtime_closeout::{
    TopologyRuntimeCloseout, TopologyRuntimeCloseoutFamily, TopologyRuntimeCloseoutRow,
    TopologyRuntimeCloseoutStatus,
};
pub use runtime_posture::{
    TopologyRuntimePostureCapability, TopologyRuntimePostureRow, TopologyRuntimePostureStatus,
};

pub use self::adapters::write_authority::TopologyRuntimeWriteAuthority;
pub use self::adapters::TopologyRuntimeSchemaAdapter;
use self::adapters::{
    TopologyExistingTruthVerificationAdapter, TopologyInspectorEvidence,
    TopologyRuntimeDeclarationInitializationAdapter, TopologyRuntimeSnapshotIdentityAdapter,
    TopologyRuntimeSourceAdapter, TopologyStaticSignalSink, TopologySubscriptionActivation,
};

pub fn topology_runtime(
    adapters: TopologyRuntimeAdapters,
    name: impl Into<String>,
) -> Result<ForgeQueryWorkspace, TopologyRuntimeFailure> {
    let support_profile = adapters.support().support_profile();
    let binding = adapters.binding.clone();
    let write_binding = binding.clone();
    let mut builder = ForgeQueryRuntime::builder()
        .runtime_bridge(self::adapters::build_runtime_bridge(binding.clone())?)
        .schema_adapter(self::adapters::TopologyRuntimeSchemaAdapter)
        .source_adapter(TopologyRuntimeSourceAdapter::new(binding.clone()))
        .snapshot_identity(TopologyRuntimeSnapshotIdentityAdapter::new(binding.clone()))
        .write_authority(
            self::adapters::write_authority::TopologyRuntimeWriteAuthority::new(write_binding),
        )
        .signal_sink(TopologyStaticSignalSink)
        .subscription_activation(TopologySubscriptionActivation::new(
            adapters.support().subscription_activation_evidence(),
        ))
        .preview_basis(adapters.support().preview_basis_adapter())
        .inspector_evidence(TopologyInspectorEvidence::new(
            adapters.support().write_receipt_evidence_label(),
            adapters.support().inspector_evidence_label(),
        ))
        .declaration_initialization(TopologyRuntimeDeclarationInitializationAdapter::new(
            adapters.declaration_initialization.clone(),
        ))
        .support_profile(support_profile);
    if adapters
        .support()
        .supports_posture(runtime_posture::TopologyRuntimePostureCapability::AuthoritativeWrites)
    {
        builder = builder.existing_truth_verification(
            TopologyExistingTruthVerificationAdapter::new(binding.clone()),
        );
    }
    let runtime = builder.build_backend_from_parts().build()?;
    runtime.workspace(name).map_err(Into::into)
}
