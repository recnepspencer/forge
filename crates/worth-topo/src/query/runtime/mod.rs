mod adapters;
mod contracts;
mod edit_support;
mod read_support;
mod runtime_closeout;
mod runtime_posture;

#[cfg(test)]
mod tests;

use forge_query::facade::{ForgeQueryRuntime, ForgeQueryWorkspace};

pub(crate) use contracts::TOPOLOGY_SNAPSHOT_HISTORICAL_BASIS_EVIDENCE;
pub use contracts::{TopologyRuntimeAdapters, TopologyRuntimeFailure, TopologyRuntimeSupport};
pub use edit_support::{
    TopologyQueryEditFamilySupportStatus, TopologyQueryEditLane,
    TopologyQueryEditLaneExecutionShape, TopologyQueryEditLaneSupportStatus,
    TopologyRuntimeEditFamilySupportRow, TopologyRuntimeEditLaneSupportRow,
};
pub use read_support::{TopologyQueryReadFamilySupportStatus, TopologyRuntimeReadFamilySupportRow};
pub use runtime_closeout::{
    TopologyRuntimeCloseout, TopologyRuntimeCloseoutFamily, TopologyRuntimeCloseoutRow,
    TopologyRuntimeCloseoutStatus,
};
pub use runtime_posture::{
    TopologyRuntimePostureCapability, TopologyRuntimePostureRow, TopologyRuntimePostureStatus,
};

use self::adapters::write_authority::TopologyRuntimeWriteAuthority;
use self::adapters::{
    build_runtime_bridge, TopologyExistingTruthVerificationAdapter, TopologyInspectorEvidence,
    TopologyPreviewBasis, TopologyRuntimeSchemaAdapter, TopologyRuntimeSourceAdapter,
    TopologyStaticSignalSink, TopologySubscriptionActivation,
};

pub fn topology_runtime(
    adapters: TopologyRuntimeAdapters,
    name: impl Into<String>,
) -> Result<ForgeQueryWorkspace, TopologyRuntimeFailure> {
    let support_profile = adapters.support().support_profile();
    let binding = adapters.binding.clone();
    let write_binding = binding.clone();
    let mut builder = ForgeQueryRuntime::builder()
        .runtime_bridge(build_runtime_bridge(binding.clone())?)
        .schema_adapter(TopologyRuntimeSchemaAdapter)
        .source_adapter(TopologyRuntimeSourceAdapter::new(binding.clone()))
        .write_authority(TopologyRuntimeWriteAuthority::new(write_binding))
        .signal_sink(TopologyStaticSignalSink)
        .subscription_activation(TopologySubscriptionActivation::new(
            adapters.support().subscription_activation_evidence(),
        ))
        .preview_basis(TopologyPreviewBasis::new(
            adapters.support().preview_basis_denial_reason(),
        ))
        .inspector_evidence(TopologyInspectorEvidence::new(
            adapters.support().write_receipt_evidence_label(),
            adapters.support().inspector_evidence_label(),
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
