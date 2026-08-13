//! Construction phases for one published application runtime.

use worth_query_installation::facade::{
    ApplicationSchema, WorthQueryInstalledApplicationSchema,
    WorthQueryInstalledGraphParticipationAuthority,
};

use super::{
    WorthQueryPrimaryGraphApplicationRuntime, WorthQueryPrimaryGraphBootstrap,
    WorthQueryPrimaryGraphInstallationDenial, WorthQueryPrimaryGraphInstallationDenialKind,
    WorthQueryPrimaryGraphProvider,
};
use crate::domain_computation::authorization::{
    WorthQueryInstalledAuthorizationRegistry, WorthQueryRuntimeClock,
};
use crate::domain_computation::execution_runtime::{
    WorthQueryExecutionInstallationAuthority, WorthQueryExecutionRuntime,
};
use crate::domain_computation::primary_graph::authentication_clock::WorthQueryAuthenticationClock;

pub(in crate::domain_computation::primary_graph) struct ApplicationRuntimePublication<Schema> {
    pub(in crate::domain_computation::primary_graph) bootstrap:
        WorthQueryPrimaryGraphBootstrap<Schema>,
    pub(in crate::domain_computation::primary_graph) runtime: WorthQueryExecutionRuntime,
    pub(in crate::domain_computation::primary_graph) authority:
        WorthQueryExecutionInstallationAuthority,
    pub(in crate::domain_computation::primary_graph) installed_schema:
        WorthQueryInstalledApplicationSchema<Schema>,
    pub(in crate::domain_computation::primary_graph) authorization_clock: WorthQueryRuntimeClock,
    pub(in crate::domain_computation::primary_graph) fault_port:
        std::sync::Arc<dyn super::super::provider::fault_port::WorthQueryPrimaryGraphFaultPort>,
}

pub(super) fn require_no_conditional_bindings<Schema>(
    runtime: &WorthQueryExecutionRuntime,
    installed_schema: &WorthQueryInstalledApplicationSchema<Schema>,
) -> Result<(), WorthQueryPrimaryGraphInstallationDenial>
where
    Schema: ApplicationSchema,
{
    let count = runtime
        .installed_packages()
        .installed_conditional_node_count_for_schema(
            installed_schema.owner(),
            installed_schema.schema_name(),
        );
    if count == 0 {
        Ok(())
    } else {
        Err(WorthQueryPrimaryGraphInstallationDenial::new(
            WorthQueryPrimaryGraphInstallationDenialKind::ConditionalBindingsRequired,
            format!(
                "{} declared conditional nodes require the conditional publication progression",
                count
            ),
        ))
    }
}

pub(super) fn publish_application_runtime_with_clock<Schema>(
    input: ApplicationRuntimePublication<Schema>,
) -> Result<
    WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    WorthQueryPrimaryGraphInstallationDenial,
>
where
    Schema: ApplicationSchema,
{
    let ApplicationRuntimePublication {
        bootstrap,
        runtime,
        authority,
        installed_schema,
        authorization_clock,
        fault_port,
    } = input;
    validate_application_schema(&runtime, &installed_schema)?;
    let authorization = compile_authorization(&bootstrap, &installed_schema)?;
    let graph =
        publish_application_graph(bootstrap, runtime, authority, &installed_schema, fault_port)?;
    Ok(assemble_application_runtime(
        graph,
        installed_schema,
        authorization,
        authorization_clock,
        Default::default(),
    ))
}

pub(in crate::domain_computation::primary_graph) fn publish_application_runtime_with_conditionals<
    Schema,
>(
    input: ApplicationRuntimePublication<Schema>,
    bindings: Vec<
        Box<dyn super::super::conditional_operation::WorthQueryPendingConditionalOperation<Schema>>,
    >,
) -> Result<
    WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    super::super::conditional_operation::WorthQueryConditionalRuntimeInstallationDenial,
>
where
    Schema: ApplicationSchema + 'static,
{
    let ApplicationRuntimePublication {
        bootstrap,
        runtime,
        authority,
        installed_schema,
        authorization_clock,
        fault_port,
    } = input;
    validate_application_schema(&runtime, &installed_schema)
        .map_err(super::super::conditional_operation::publication_denial)?;
    let expected = runtime
        .installed_packages()
        .installed_conditional_node_count_for_schema(
            installed_schema.owner(),
            installed_schema.schema_name(),
        );
    super::super::conditional_operation::require_complete_binding_inventory(expected, &bindings)?;
    let authorization = compile_authorization(&bootstrap, &installed_schema)
        .map_err(super::super::conditional_operation::publication_denial)?;
    let mut graph =
        publish_application_graph(bootstrap, runtime, authority, &installed_schema, fault_port)
            .map_err(super::super::conditional_operation::publication_denial)?;
    let conditional_operations = super::super::conditional_operation::install_pending_bindings(
        bindings,
        &mut graph.bridge,
        graph.runtime.authority_identity().as_u64(),
        graph.runtime.installed_packages().runtime_ordinal(),
        graph.runtime.installed_packages().generation().ordinal(),
        graph.primary_graph_authority.provider_identity(),
        super::super::application_branch::PRIMARY_APPLICATION_BRANCH,
    )?;
    Ok(assemble_application_runtime(
        graph,
        installed_schema,
        authorization,
        authorization_clock,
        conditional_operations,
    ))
}

struct PublishedApplicationGraph {
    runtime: WorthQueryExecutionRuntime,
    publication: super::WorthQueryPrimaryGraphPublication,
    relational_source: worth_relational::facade::bridge::RuntimeBridgeRelationalSource,
    execution_basis_source:
        worth_relational::facade::runtime::RelationalApplicationCommitBasisSource,
    bridge: super::super::managed_bridge::WorthQueryInstalledApplicationBridge,
    primary_provider: std::sync::Arc<WorthQueryPrimaryGraphProvider>,
    primary_graph_authority: WorthQueryInstalledGraphParticipationAuthority,
}

fn validate_application_schema<Schema>(
    runtime: &WorthQueryExecutionRuntime,
    installed_schema: &WorthQueryInstalledApplicationSchema<Schema>,
) -> Result<(), WorthQueryPrimaryGraphInstallationDenial>
where
    Schema: ApplicationSchema,
{
    runtime
        .installed_packages()
        .validate_application_schema(installed_schema)
        .map_err(|denial| {
            WorthQueryPrimaryGraphInstallationDenial::new(
                WorthQueryPrimaryGraphInstallationDenialKind::StaleInstalledSchema,
                denial.subject(),
            )
        })
}

fn compile_authorization<Schema>(
    bootstrap: &WorthQueryPrimaryGraphBootstrap<Schema>,
    installed_schema: &WorthQueryInstalledApplicationSchema<Schema>,
) -> Result<WorthQueryInstalledAuthorizationRegistry, WorthQueryPrimaryGraphInstallationDenial>
where
    Schema: ApplicationSchema,
{
    WorthQueryInstalledAuthorizationRegistry::compile(installed_schema, &bootstrap.graph.layout)
        .map_err(|denial| {
            WorthQueryPrimaryGraphInstallationDenial::new(
                WorthQueryPrimaryGraphInstallationDenialKind::AuthorizationPolicyRejected,
                denial.subject(),
            )
        })
}

fn publish_application_graph<Schema>(
    bootstrap: WorthQueryPrimaryGraphBootstrap<Schema>,
    mut runtime: WorthQueryExecutionRuntime,
    authority: WorthQueryExecutionInstallationAuthority,
    installed_schema: &WorthQueryInstalledApplicationSchema<Schema>,
    fault_port: std::sync::Arc<
        dyn super::super::provider::fault_port::WorthQueryPrimaryGraphFaultPort,
    >,
) -> Result<PublishedApplicationGraph, WorthQueryPrimaryGraphInstallationDenial>
where
    Schema: ApplicationSchema,
{
    let publication = bootstrap.publish(&mut runtime, &authority)?;
    let graph = runtime
        .retain_primary_graph_integration_handle()
        .expect("publishing the primary graph installs its integration authority");
    let relational_source = graph.relational_bridge_source();
    let execution_basis_source = graph.relational_execution_basis_source();
    let bridge = super::super::managed_bridge::install_application_bridge(
        installed_schema,
        relational_source.clone(),
    )?;
    let (provider_anchor, primary_provider) =
        WorthQueryPrimaryGraphProvider::install(graph, fault_port);
    let primary_graph_authority =
        install_graph_participation_authority(&authority, provider_anchor)?;
    Ok(PublishedApplicationGraph {
        runtime,
        publication,
        relational_source,
        execution_basis_source,
        bridge,
        primary_provider,
        primary_graph_authority,
    })
}

fn install_graph_participation_authority(
    authority: &WorthQueryExecutionInstallationAuthority,
    provider_anchor: std::sync::Arc<
        crate::domain_computation::provider_session::graph_provider::bounded_step::provider_anchor::WorthQueryGraphProviderAnchor,
    >,
) -> Result<WorthQueryInstalledGraphParticipationAuthority, WorthQueryPrimaryGraphInstallationDenial>
{
    WorthQueryInstalledGraphParticipationAuthority::install(
        authority.installation_runtime(),
        "primary",
        provider_anchor.provider_identity(),
        true,
        Some("primary"),
        provider_anchor,
    )
    .map_err(|detail| {
        WorthQueryPrimaryGraphInstallationDenial::new(
            WorthQueryPrimaryGraphInstallationDenialKind::RelationalSchemaRejected,
            detail,
        )
    })
}

fn assemble_application_runtime<Schema>(
    graph: PublishedApplicationGraph,
    installed_schema: WorthQueryInstalledApplicationSchema<Schema>,
    authorization: WorthQueryInstalledAuthorizationRegistry,
    authorization_clock: WorthQueryRuntimeClock,
    conditional_operations:
        super::super::conditional_operation::WorthQueryConditionalOperationRegistry<Schema>,
) -> WorthQueryPrimaryGraphApplicationRuntime<Schema> {
    let runtime_authority = graph.runtime.authority_identity();
    // One clock, shared. The registry hands it back to any handle that needs to
    // re-check its own deadline, which is why no recovery transition takes a
    // clock argument (R8.31).
    let authorization_clock = std::sync::Arc::new(authorization_clock);
    let recovery_handles = std::sync::Arc::new(
        crate::domain_computation::managed_run::WorthQueryRecoveryHandleRegistry::for_runtime(
            runtime_authority,
            std::sync::Arc::clone(&authorization_clock),
        ),
    );
    WorthQueryPrimaryGraphApplicationRuntime {
        runtime: graph.runtime,
        installed_schema,
        publication: graph.publication,
        authorization,
        authorization_clock,
        authentication_clock: WorthQueryAuthenticationClock::system(),
        relational_source: graph.relational_source,
        execution_basis_source: graph.execution_basis_source,
        bridge: graph.bridge,
        conditional_operations,
        primary_provider: graph.primary_provider,
        primary_graph_authority: graph.primary_graph_authority,
        result_buffers: Default::default(),
        basis_leases: Default::default(),
        next_preview_session: std::sync::atomic::AtomicU64::new(1),
        next_external_dispatch_attempt: std::sync::atomic::AtomicU64::new(1),
        external_effect_transport: std::sync::OnceLock::new(),
        recovery_handles,
    }
}
