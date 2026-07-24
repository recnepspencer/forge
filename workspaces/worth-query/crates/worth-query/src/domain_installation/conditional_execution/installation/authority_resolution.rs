use super::WorthQueryConditionalNodeInstallationDenial;

pub(super) fn installed_conditional_operation<D: 'static, O: 'static, F: 'static>(
    domains: &crate::domain_installation::WorthQueryDomainInstallationRegistry,
) -> Result<
    crate::domain_installation::WorthQueryInstalledDomainOperation<D, O, F>,
    WorthQueryConditionalNodeInstallationDenial,
> {
    let domain = domains
        .domain::<D>()
        .map_err(|_| WorthQueryConditionalNodeInstallationDenial::DomainNotInstalled)?;
    let resolved = domains
        .execution_index()
        .domain_operation_authority(
            std::any::TypeId::of::<D>(),
            std::any::TypeId::of::<O>(),
            std::any::TypeId::of::<F>(),
        )
        .ok_or(WorthQueryConditionalNodeInstallationDenial::OperationNotInstalled)?;
    let bindings = domains
        .execution_index()
        .domain_operation_graph_bindings(
            std::any::TypeId::of::<D>(),
            std::any::TypeId::of::<O>(),
            std::any::TypeId::of::<F>(),
        )
        .to_vec();
    let binding_count = bindings.len();
    Ok(
        crate::domain_installation::WorthQueryInstalledDomainOperation::mint(
            domain.authority_arc(),
            resolved.authority,
            resolved.workflow_graph,
            resolved.evidence_contract,
            bindings,
            crate::domain_installation::WorthQueryInstalledDomainOperationLookupCounters {
                authority_checks: 1,
                indexed_operation_lookups: 1,
                graph_binding_lookups: 1,
                graph_bindings_retained: binding_count,
                ..Default::default()
            },
        ),
    )
}

pub(super) fn installed_conditional_graph<G: 'static>(
    graphs: &crate::domain_installation::WorthQueryInstalledGraphParticipationRegistry,
) -> Result<
    crate::domain_installation::WorthQueryInstalledGraphParticipation<G>,
    WorthQueryConditionalNodeInstallationDenial,
> {
    graphs
        .get::<G>()
        .map(crate::domain_installation::WorthQueryInstalledGraphParticipation::new)
        .map_err(|_| WorthQueryConditionalNodeInstallationDenial::GraphNotInstalled)
}
