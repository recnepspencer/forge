use worth_query::facade::{domain, runtime};

use super::{
    WorthUiPresentationAsyncDomainEntry, WorthUiPresentationAsyncOperation,
    WorthUiPresentationAsyncOperationFamily, WorthUiPresentationConditionalCompute,
    WorthUiPresentationSemanticGraph,
};

pub(crate) fn install_presentation_semantic_instance(
    workspace: &mut runtime::WorthQueryWorkspace,
    records: [worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts;
        super::installed_operation::DEPENDENCY_COUNT],
    output_version: u64,
) -> Result<
    runtime::WorthQueryInstalledOwnedConditionalInstance,
    runtime::WorthQueryOwnedConditionalInstanceDenial,
> {
    workspace.install_owned_conditional_instance(
        WorthUiPresentationAsyncDomainEntry,
        WorthUiPresentationAsyncOperation,
        WorthUiPresentationAsyncOperationFamily,
        WorthUiPresentationSemanticGraph,
        domain::WorthQueryConditionalNodeLocation::operation("presentation-currentness")
            .expect("static WUI conditional location must admit"),
        records
            .into_iter()
            .map(|record| {
                domain::WorthQueryOwnedConditionalDependencyInstallation::new(Some(record))
            })
            .collect(),
        worth_runtime_bridge::facade::BridgeConditionalProviderSet::new(),
        WorthUiPresentationConditionalCompute::new(output_version),
    )
}

pub(crate) fn publish_presentation_semantic_change(
    workspace: &mut runtime::WorthQueryWorkspace,
    instance: &runtime::WorthQueryInstalledOwnedConditionalInstance,
    dependency_ordinal: usize,
) -> Result<
    worth_runtime_bridge::facade::CorrespondenceDeliveryOutcome,
    runtime::WorthQueryOwnedConditionalInstanceDenial,
> {
    workspace.publish_owned_conditional_instance_change(
        WorthUiPresentationAsyncDomainEntry,
        WorthUiPresentationAsyncOperation,
        WorthUiPresentationAsyncOperationFamily,
        instance,
        dependency_ordinal,
    )
}

pub(crate) fn retire_presentation_semantic_instance(
    workspace: &mut runtime::WorthQueryWorkspace,
    instance: &runtime::WorthQueryInstalledOwnedConditionalInstance,
) -> Result<(), runtime::WorthQueryOwnedConditionalInstanceDenial> {
    workspace.retire_owned_conditional_instance(
        WorthUiPresentationAsyncDomainEntry,
        WorthUiPresentationAsyncOperation,
        WorthUiPresentationAsyncOperationFamily,
        instance,
    )
}
