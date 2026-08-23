mod bridge_registrations;
mod compute;
mod graph_participation;
mod installed_operation;
mod instance;

use worth_query::facade::{domain, runtime};

pub(crate) use bridge_registrations::presentation_bridge_registrations;
pub(crate) use installed_operation::WorthUiPresentationAsyncDomainEntry;
pub(super) use instance::{
    install_presentation_semantic_instance, publish_presentation_semantic_change,
    retire_presentation_semantic_instance,
};

use compute::WorthUiPresentationConditionalCompute;
use graph_participation::{
    presentation_graph_definition, WorthUiPresentationGraphProvider,
    WorthUiPresentationSemanticGraph,
};
use installed_operation::{
    presentation_aspect_contracts, presentation_async_definition,
    WorthUiPresentationAsyncOperationExecutor,
};
pub(super) use installed_operation::{
    WorthUiPresentationAsyncOperation, WorthUiPresentationAsyncOperationFamily,
};

pub(crate) fn worth_ui_presentation_async_domain_package(
) -> domain::WorthQueryDomainPackage<WorthUiPresentationAsyncDomainEntry> {
    domain::WorthQueryDomainPackage::declare(
        WorthUiPresentationAsyncDomainEntry,
        domain::WorthQueryDomainIdentityDeclaration::new(
            domain::WorthQueryDomainIdentityNamespace::new("WORTH.ui")
                .expect("static WUI namespace must admit"),
            domain::WorthQueryDomainIdentityName::new("presentation-async")
                .expect("static WUI presentation domain must admit"),
            domain::WorthQueryDomainSemanticVersion::new(1, 0),
        ),
    )
    .requires_capability(domain::WorthQueryCapabilityFamily::QueryRead)
    .requires_capability(domain::WorthQueryCapabilityFamily::QueryComposition)
    .requires_configuration(domain::WorthQueryConfigSectionFamily::Query)
    .requires_configuration(domain::WorthQueryConfigSectionFamily::Relational)
    .operation(presentation_async_definition())
    .operation_graph_participation::<
        WorthUiPresentationAsyncOperation,
        WorthUiPresentationAsyncOperationFamily,
        WorthUiPresentationSemanticGraph,
    >("presentation")
}

pub(crate) fn install_worth_ui_presentation_async_runtime(
    builder: runtime::WorthQueryRuntimeBuilder,
) -> Result<runtime::WorthQueryRuntimeBuilder, runtime::WorthQueryAspectContractRegistrationDenial>
{
    let builder = builder.aspect_contracts(presentation_aspect_contracts())?;
    Ok(builder
        .graph_participation(presentation_graph_definition())
        .graph_participation_provider(
            WorthUiPresentationSemanticGraph,
            WorthUiPresentationGraphProvider,
        )
        .owned_topology_conditional_instances(
            WorthUiPresentationAsyncDomainEntry,
            WorthUiPresentationAsyncOperation,
            WorthUiPresentationAsyncOperationFamily,
            WorthUiPresentationSemanticGraph,
            domain::WorthQueryConditionalNodeLocation::operation("presentation-currentness")
                .expect("static WUI conditional location must admit"),
            WorthUiPresentationConditionalCompute::new(0),
        )
        .domain_operation_executor(
            WorthUiPresentationAsyncDomainEntry,
            WorthUiPresentationAsyncOperation,
            WorthUiPresentationAsyncOperationFamily,
            WorthUiPresentationAsyncOperationExecutor,
        ))
}
