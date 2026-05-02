mod adapters;
mod contracts;

#[cfg(test)]
mod tests;

use forge_query::facade::{ForgeQueryRuntime, ForgeQueryWorkspace};

pub use contracts::{
    WorthTopologyQueryEditFamilySupportStatus, WorthTopologyRuntimeAdapters,
    WorthTopologyRuntimeFailure, WorthTopologyRuntimeSupport,
};

use self::adapters::write_authority::WorthTopologyRuntimeWriteAuthority;
use self::adapters::{
    build_runtime_bridge, WorthTopologyExistingTruthVerificationAdapter,
    WorthTopologyInspectorEvidence, WorthTopologyPreviewBasis, WorthTopologyRuntimeSchemaAdapter,
    WorthTopologyRuntimeSourceAdapter, WorthTopologyStaticSignalSink,
    WorthTopologySubscriptionActivation,
};

pub fn worth_topology_runtime(
    adapters: WorthTopologyRuntimeAdapters,
    name: impl Into<String>,
) -> Result<ForgeQueryWorkspace, WorthTopologyRuntimeFailure> {
    let support_profile = adapters.support().support_profile();
    let binding = adapters.binding.clone();
    let write_binding = binding.clone();
    let mut builder = ForgeQueryRuntime::builder()
        .runtime_bridge(build_runtime_bridge(binding.clone())?)
        .schema_adapter(WorthTopologyRuntimeSchemaAdapter)
        .source_adapter(WorthTopologyRuntimeSourceAdapter::new(binding.clone()))
        .write_authority(WorthTopologyRuntimeWriteAuthority::new(write_binding))
        .signal_sink(WorthTopologyStaticSignalSink)
        .subscription_activation(WorthTopologySubscriptionActivation)
        .preview_basis(WorthTopologyPreviewBasis)
        .inspector_evidence(WorthTopologyInspectorEvidence)
        .support_profile(support_profile);
    if adapters.support().authoritative_writes_supported() {
        builder = builder.existing_truth_verification(
            WorthTopologyExistingTruthVerificationAdapter::new(binding.clone()),
        );
    }
    let runtime = builder.build_backend_from_parts().build()?;
    runtime.workspace(name).map_err(Into::into)
}
